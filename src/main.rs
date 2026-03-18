// Import modules
use niri_ipc::{Event, state::EventStreamState, state::EventStreamStatePart};
use std::collections::HashMap;

// Import internal libraries
mod maximize;
mod outputs; // https://github.com/Antiz96/oniri/issues/3
mod sizecompare; // https://github.com/Antiz96/oniri/issues/3
mod socket;
mod version;

fn main() -> anyhow::Result<()> {
    // Show name and version if the -V / --version arg is passed
    if version::show_version() {
        return Ok(());
    }

    // Initialize socket connections to niri IPC
    let (event_socket, mut action_socket) = socket::initialize_socket_connections()?;

    // Gather state and create an outputs map
    // This can be dropped once https://github.com/Antiz96/oniri/issues/3 is resolved
    let mut state = EventStreamState::default();
    let outputs = outputs::outputs_maps(&mut action_socket)?;

    // Read events gathered from the IPC socket
    let mut read_event = event_socket.read_events();

    // Create a workspace/window(s) map
    let mut workspace_windows: HashMap<u64, Vec<u64>> = HashMap::new();

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

                println!("Trigger Event: Window Opened");

                // Update the workspace/window(s) map
                let id = window.id;
                if let Some(ws) = window.workspace_id {
                    workspace_windows.entry(ws).or_default().push(id);
                }

                // Check if there's only one window in the workspace/window(s) map & maximize it if so
                maximize::maximize_window_if_alone(
                    &workspace_windows,
                    &state,   // https://github.com/Antiz96/oniri/issues/3
                    &outputs, // https://github.com/Antiz96/oniri/issues/3
                    &mut action_socket,
                )?;
            }
            // Window being closed
            Event::WindowClosed { id } => {
                println!("Trigger Event: Window Closed");

                // Update the workspace/window(s) map
                for windows in workspace_windows.values_mut() {
                    windows.retain(|&wid| wid != id);
                }

                // Check if there's only one window in the workspace/window(s) map & maximize it if so
                maximize::maximize_window_if_alone(
                    &workspace_windows,
                    &state,   // https://github.com/Antiz96/oniri/issues/3
                    &outputs, // https://github.com/Antiz96/oniri/issues/3
                    &mut action_socket,
                )?;
            }
            // Ignore other events
            _ => {}
        }
    }
    Ok(())
}
