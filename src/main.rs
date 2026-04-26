//! Oniri - A tool that automatically maximizes the only window of a niri workspace.
//!
//! It relies on the niri IPC to verify if a window is the only one of a workspace,
//! whether it's the first one opened or the last remaining ones after all the other windows got closed,
//! and maximize it if so.

// Import external modules
use log::{debug, info};
use niri_ipc::{Event, state::EventStreamState, state::EventStreamStatePart};
use std::env;

use crate::{maximize_window::maximize_window, size_compare::is_maximized};

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

    // Run in "tiling-layout" mode if the -T / --tiling-layout arg is passed
    let tiling_layout = has_arg("-T") || has_arg("--tiling-layout");
    if tiling_layout {
        info!(
            "Running in tiling-layout mode: Opening a second window will collapse the first window"
        );
    }

    // Run in "maximizing-to-edges" mode if the -E / --edges-maximizing arg is passed
    let edges_maximizing = has_arg("-E") || has_arg("--edges-maximizing");
    if edges_maximizing {
        info!("Running in maximize-to-edges mode: Maximize windows to edges");
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

                debug!("Trigger Event: Window Opened Or Changed");

                let id = window.id;
                let Some(ws) = window.workspace_id else {
                    continue;
                };

                // Workaround IPC limitation by checking if the window that triggered the event is
                // in the same workspace.
                //
                // This will differentiate between WindowOpened and WindowChanged.
                if workspace_windows
                    .get(&ws)
                    .is_some_and(|windows| windows.contains(&id))
                {
                    continue;
                }

                // Update the workspace/window(s) map
                for windows in workspace_windows.values_mut() {
                    windows.retain(|&wid| wid != id);
                }

                let windows = workspace_windows.entry(ws).or_default();
                windows.push(id);

                // Check if there's only one window in the workspace/window(s) map & maximize it if so
                match windows.len() {
                    1 => {
                        let first_window = windows[0];
                        if !is_maximized(&state, &outputs, first_window, tol_h, tol_w) {
                            maximize_window(
                                &mut action_socket,
                                &state,
                                first_window,
                                edges_maximizing,
                            )?;
                        }
                    }

                    // If running in tiling layout mode, un-maximize the first window when a second one is opened
                    2 if tiling_layout => {
                        let first_window = windows[0];
                        if is_maximized(&state, &outputs, first_window, tol_h, tol_w) {
                            maximize_window(
                                &mut action_socket,
                                &state,
                                first_window,
                                edges_maximizing,
                            )?;
                        }
                    }
                    _ => {}
                }
            }
            // Window being closed
            Event::WindowClosed { id } => {
                debug!("Trigger Event: Window Closed");

                let Some((_, windows)) = workspace_windows
                    .iter_mut()
                    .find(|(_, windows)| windows.contains(&id))
                else {
                    continue;
                };

                // Update the workspace vector
                windows.retain(|&wid| wid != id);

                // Skip if the -F / --first-only arg is passed
                if first_only {
                    continue;
                }

                // Check if there's only one window in the workspace/window(s) map & maximize it if so
                if windows.len() != 1 {
                    continue;
                }

                let id = windows[0];
                if !is_maximized(&state, &outputs, id, tol_h, tol_w) {
                    maximize_window(&mut action_socket, &state, id, edges_maximizing)?;
                }
            }
            // Ignore other events
            _ => {}
        }
    }
    Ok(())
}
