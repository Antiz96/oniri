//! Oniri - A tool that automatically maximizes the only window of a niri workspace.
//!
//! It relies on the niri IPC to verify if a window is the only one of a workspace,
//! whether it's the first one opened or the last remaining ones after all the other windows got closed,
//! and maximize it if so.

use clap::Parser;
use log::{debug, error, info};
use niri_ipc::{Event, state::EventStreamState, state::EventStreamStatePart};
use std::collections::HashMap;
use std::io::ErrorKind;
use std::process;

use crate::fill_gap::fill_gap;
use crate::maximize_window::maximize_window;
use crate::size_compare::is_maximized;

mod fill_gap;
mod help;
mod lockfile;
mod maximize_window;
mod outputs_map; // https://github.com/Antiz96/oniri/issues/3
mod size_compare; // https://github.com/Antiz96/oniri/issues/3
mod socket_connections;
mod version;
mod windows_map;

// Argument parser
#[derive(Parser)]
#[command(disable_help_flag = true, disable_version_flag = true)]
struct Args {
    // Options / flags
    #[arg(short = 'F', long)]
    first_only: bool,

    #[arg(short = 'T', long)]
    tiling_layout: bool,

    #[arg(short = 'E', long)]
    edges_maximizing: bool,

    #[arg(short = 'M', long)]
    move_on_close: bool,

    #[arg(short = 'H', long, default_value_t = 150)]
    height_tolerance: i32,

    #[arg(short = 'W', long, default_value_t = 150)]
    width_tolerance: i32,

    #[arg(short = 'h', long)]
    help: bool,

    #[arg(short = 'V', long)]
    version: bool,
}

fn main() -> anyhow::Result<()> {
    // Initialize logger
    env_logger::init();

    // Parse arguments
    let args = Args::parse();

    // Show help message if the -h / --help arg is passed
    if args.help {
        help::show_help();
        return Ok(());
    }

    // Show name and version if the -V / --version arg is passed
    if args.version {
        version::show_version();
        return Ok(());
    }

    // Create (if needed) and acquire lockfile
    // Exit if there's already an instance running
    // or if there was an issue creating or acquiring the lockfile (e.g. permission issue)
    let _lock = lockfile::acquire_lockfile().unwrap_or_else(|error| {
        if error.kind() == ErrorKind::AlreadyExists {
            error!("Another instance of oniri is already running");
        } else {
            error!("Failed to acquire lockfile: {error}");
        }

        process::exit(1);
    });

    // Run in "first-only" mode if the -F / --first-only arg is passed
    let first_only = args.first_only;
    if first_only {
        info!("Running in first-only mode: only acting on the first window");
    }

    // Run in "tiling-layout" mode if the -T / --tiling-layout arg is passed
    let tiling_layout = args.tiling_layout;
    if tiling_layout {
        info!(
            "Running in tiling-layout mode: Opening a second window will collapse the first window"
        );
    }

    // Run in "edges-maximizing" mode if the -E / --edges-maximizing arg is passed
    let edges_maximizing = args.edges_maximizing;
    if edges_maximizing {
        info!("Running in edges-maximizing mode: Maximize windows to edges");
    }

    // Run in "move-on-close" mode if the -M / --move-on-close arg is passed
    let move_on_close = args.move_on_close;
    if move_on_close {
        info!("Running in move-on-close mode: Moving the viewport on close to fill remaining gap");
    }

    // Set pixel tolerances for window/output size comparison
    // This can be dropped once https://github.com/Antiz96/oniri/issues/3 is resolved
    let tol_h = args.height_tolerance;
    let tol_w = args.width_tolerance;
    info!("Using tolerances: height={tol_h}, width={tol_w}");

    // Initialize connections to niri IPC socket, start the event stream and gather events
    let (event_socket, mut action_socket) = socket_connections::init_socket_connections()?;

    // Gather state and create an outputs map
    // This can be dropped once https://github.com/Antiz96/oniri/issues/3 is resolved
    let mut state = EventStreamState::default();
    let outputs = outputs_map::init_outputs_map(&mut action_socket)?;

    // Create a workspace/window(s) map and initialize it
    let mut workspace_windows = windows_map::init_windows_map(&mut action_socket)?;

    // Track windows that have moved workspaces until their tile size is properly recalculated
    // in the "WindowLayoutsChanged" event
    // This avoids comparing a window's old layout size against its new workspace output's resolution
    // See https://github.com/Antiz96/oniri/issues/79 for details
    // This can be dropped once https://github.com/Antiz96/oniri/issues/3 is resolved
    let mut moved_windows_map = HashMap::<u64, u64>::new();

    // Read events gathered from the IPC socket
    let mut read_event = event_socket.read_events();

    // Loop over events
    while let Ok(event) = read_event() {
        // Check if the closing window sits in the leftmost column, before state.apply()
        // below removes it from the state and this information becomes unreachable (used for the
        // move-on-close mode).
        let closed_window_was_leftmost =
            matches!(&event, Event::WindowClosed { id } if fill_gap::is_leftmost(&state, *id));

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
                // If not, track the previous workspace to act on it (if needed), for instance if the current
                // window is moved to another workspace and there's only one window remaining on the
                // previous one (which should therefore be maximized).
                //
                // This will differentiate between WindowOpened and WindowChanged.
                let previous_ws = workspace_windows
                    .iter()
                    .find_map(|(&tracked_ws, windows)| windows.contains(&id).then_some(tracked_ws));

                if previous_ws == Some(ws) {
                    continue;
                }

                // Update the workspace/window(s) map
                for windows in workspace_windows.values_mut() {
                    windows.retain(|&wid| wid != id);
                }

                let windows = workspace_windows.entry(ws).or_default();
                windows.push(id);

                // Check if there's only one window in the current workspace & maximize it if so
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

                // If the window that triggered the event has been moved to another workspace:
                if let Some(old_ws) = previous_ws
                    && old_ws != ws
                {
                    // Track windows that have moved workspaces until their tile size is properly recalculated
                    // in the "WindowLayoutsChanged" event
                    // This avoids comparing a window's old layout size against its new workspace output's resolution
                    // See https://github.com/Antiz96/oniri/issues/79 for details
                    // This can be dropped once https://github.com/Antiz96/oniri/issues/3 is resolved
                    moved_windows_map.insert(id, old_ws);
                }
            }

            Event::WindowLayoutsChanged { changes } => {
                for (id, _) in changes {
                    if let Some(old_ws) = moved_windows_map.remove(&id) {
                        // Retrieve updated window and workspace state
                        let Some(window) = state.windows.windows.get(&id) else {
                            continue;
                        };

                        let Some(ws) = window.workspace_id else {
                            continue;
                        };

                        let Some(windows) = workspace_windows.get(&ws) else {
                            continue;
                        };

                        // In tiling layout mode, un-maximize the current window
                        // if it was previously the only window of its workspace but now isn't
                        if tiling_layout
                            && windows.len() > 1
                            && let Some(&last_window) = windows.last()
                            && workspace_windows
                                .get(&old_ws)
                                .is_none_or(|old_windows| old_windows.is_empty())
                        {
                            if is_maximized(&state, &outputs, last_window, tol_h, tol_w) {
                                maximize_window(
                                    &mut action_socket,
                                    &state,
                                    last_window,
                                    edges_maximizing,
                                )?;
                            }
                        }
                        // If there's one window left in the previous workspace, maximize it
                        // (unless we're running in "first-only" mode).
                        else if !first_only
                            && let Some(old_windows) = workspace_windows.get(&old_ws)
                            && old_windows.len() == 1
                        {
                            let remaining = old_windows[0];

                            if !is_maximized(&state, &outputs, remaining, tol_h, tol_w) {
                                maximize_window(
                                    &mut action_socket,
                                    &state,
                                    remaining,
                                    edges_maximizing,
                                )?;
                            }
                        }
                    }
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

                // If running in "move-on-close" mode, nudge the focus to close any leftover gap (if
                // needed)
                if move_on_close && !closed_window_was_leftmost {
                    fill_gap(&mut action_socket, windows.len())?;
                }

                // Skip if running in "first only" mode
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
