// Import modules
// niri_ipc::* to connect & gather events from niri's IPC socket and act on those
// std::env to access environment variables & arguments and std::HashMap to create maps
use niri_ipc::{Event, Request, Response, socket::Socket};
use std::{collections::HashMap, env};

// Define NAME & VERSION constants, fetched from Cargo metadata
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> anyhow::Result<()> {
    // Print name and version if the -V / --version arg is passed
    if env::args().any(|arg| arg == "-V" || arg == "--version") {
        println!("{} {}", NAME, VERSION);
        return Ok(());
    }

    // Connect to niri IPC socket and start the event stream to gather events
    let mut event_socket = Socket::connect()?;
    let reply = event_socket.send(Request::EventStream)?;

    // Create a separate socket connection to send actions
    let mut action_socket = Socket::connect()?;

    // Capture events
    if matches!(reply, Ok(Response::Handled)) {
        let mut read_event = event_socket.read_events();

        // Create a window(s)/workspace map
        let mut workspace_windows: HashMap<u64, Vec<u64>> = HashMap::new();

        // Loop over events and filter the ones about windows being opened or closed
        // Update the window(s)/workspace map when events are matched
        while let Ok(event) = read_event() {
            match event {
                Event::WindowOpenedOrChanged { window } => {
                    // Skip floating windows (they cannot/should not be maximized)
                    if window.is_floating {
                        continue;
                    }
                    println!("Trigger Event: Window Opened");
                    let id = window.id;
                    if let Some(ws) = window.workspace_id {
                        workspace_windows.entry(ws).or_default().push(id);
                    }
                    // Maximize the window if it's the only one
                    for windows in workspace_windows.values() {
                        if windows.len() == 1 {
                            let id = windows[0];
                            let _ = action_socket
                                .send(Request::Action(niri_ipc::Action::MaximizeColumn {}));
                            println!("Maximized window {}", id);
                        }
                    }
                }
                Event::WindowClosed { id } => {
                    println!("Trigger Event: Window Closed");
                    for windows in workspace_windows.values_mut() {
                        windows.retain(|&wid| wid != id);
                    }
                    // Maximize the window if it's the only one
                    for windows in workspace_windows.values() {
                        if windows.len() == 1 {
                            let id = windows[0];
                            let _ = action_socket
                                .send(Request::Action(niri_ipc::Action::MaximizeColumn {}));
                            println!("Maximized window {}", id);
                        }
                    }
                }
                // Ignore other events
                _ => {}
            }
        }
    }
    Ok(())
}
