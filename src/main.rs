//! Oniri - A tool that automatically maximizes the only window of a niri workspace.
//!
//! It relies on the niri IPC to verify if a window is the only one of a workspace,
//! whether it's the first one opened or the last remaining ones after all the other windows got closed,
//! and maximize it if so.

// Import external modules
use log::{debug, info};
use niri_ipc::{Event, state::EventStreamState, state::EventStreamStatePart};
use std::env;

// Import internal modules
mod help;
mod maximize_window;
mod outputs_map; // https://github.com/Antiz96/oniri/issues/3
mod size_compare; // https://github.com/Antiz96/oniri/issues/3
mod socket_connections;
mod windows_map;

fn main() -> anyhow::Result<()> {
    // Initialize logger
    env_logger::init();

    // Parse arguments
    let args: Vec<String> = env::args().collect();
    let has_arg = |flag: &str| args.iter().any(|arg| arg == flag);

    // Show help message if the -h / --help arg is passed
    if has_arg("-h") || has_arg("--help") {
        help::show_help();
        return Ok(());
    }

    // Show name and version if the -V / --version arg is passed
    if has_arg("-V") || has_arg("--version") {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // Run in "first-only" mode if the -F / --first-only arg is passed
    let first_only = has_arg("-F") || has_arg("--first-only");
    if first_only {
        info!("Running in first-only mode: only acting on the first window");
    }

    // Set pixel tolerances for window/output size comparison
    // This can be dropped once https://github.com/Antiz96/oniri/issues/3 is resolved
    let (tol_h, tol_w) = size_compare::set_tolerances();
    info!("Using tolerances: height={}, width={}", tol_h, tol_w);

    // Initialize connections to niri IPC socket, start the event stream and gather events
    let (event_socket, mut action_socket) = socket_connections::init_socket_connections()?;

    // Gather state and create an outputs map
    // This can be dropped once https://github.com/Antiz96/oniri/issues/3 is resolved
    let mut state = EventStreamState::default();
    let outputs = outputs_map::init_outputs_map(&mut action_socket)?;

    // Create a workspace/window(s) map and initialize it
    let mut workspace_windows = windows_map::init_windows_map(&mut action_socket)?;

    // Read events gathered from the IPC socket
    let mut read_event = event_socket.read_events();

    // Loop over events
    while let Ok(event) = read_event() {
        // Update state
        // This can be dropped once https://github.com/Antiz96/oniri/issues/3 is resolved
        state.apply(event.clone());

        // Filter events
        match event {
            // Window being opened
            Event::WindowOpenedOrChanged { window } => {
                // Skip floating windows (they cannot/should not be maximized)
                if window.is_floating {
                    continue;
                }

                debug!("Trigger Event: Window Opened");

                // Update the workspace/window(s) map
                let id = window.id;
                if let Some(ws) = window.workspace_id {
                    let entry = workspace_windows.entry(ws).or_default();
                    if !entry.contains(&id) {
                        entry.push(id);
                    }
                }

                // Check if there's only one window in the workspace/window(s) map & maximize it if so
                maximize_window::maximize_window_if_alone(
                    &workspace_windows,
                    &state,   // https://github.com/Antiz96/oniri/issues/3
                    &outputs, // https://github.com/Antiz96/oniri/issues/3
                    tol_h,    // https://github.com/Antiz96/oniri/issues/3
                    tol_w,    // https://github.com/Antiz96/oniri/issues/3
                    &mut action_socket,
                )?;
            }
            // Window being closed
            Event::WindowClosed { id } => {
                debug!("Trigger Event: Window Closed");

                // Update the workspace/window(s) map
                for windows in workspace_windows.values_mut() {
                    windows.retain(|&wid| wid != id);
                }

                // Skip if the -F / --first-only arg is passed
                if first_only {
                    continue;
                }

                // Check if there's only one window in the workspace/window(s) map & maximize it if so
                maximize_window::maximize_window_if_alone(
                    &workspace_windows,
                    &state,   // https://github.com/Antiz96/oniri/issues/3
                    &outputs, // https://github.com/Antiz96/oniri/issues/3
                    tol_h,    // https://github.com/Antiz96/oniri/issues/3
                    tol_w,    // https://github.com/Antiz96/oniri/issues/3
                    &mut action_socket,
                )?;
            }
            // Ignore other events
            _ => {}
        }
    }
    Ok(())
}
