// Import modules
// niri_ipc::* to connect & gather events from niri's IPC socket and act on those
// For niri_ipc::Output and niri_ipc::state::EventStreamState{,,Part}, see https://github.com/Antiz96/oniri/issues/3
// std::collections::HashMap to create maps
use niri_ipc::{
    Event, Output, Request, Response, socket::Socket, state::EventStreamState,
    state::EventStreamStatePart,
};
use std::collections::HashMap;

// Import internal libraries
mod version;

fn main() -> anyhow::Result<()> {
    version::show_version();

    // Connect to niri IPC socket and start the event stream to gather events
    let mut event_socket = Socket::connect()?;
    let reply = event_socket.send(Request::EventStream)?;

    // Create a separate socket connection to send actions
    let mut action_socket = Socket::connect()?;

    // Gather the whole state and create an outputs map
    // This is used later to workaround some limitations of the niri IPC
    // See https://github.com/Antiz96/oniri/issues/3
    let mut state = EventStreamState::default();
    let response = action_socket.send(Request::Outputs)?;
    let output_list: HashMap<String, Output> = match response {
        Ok(Response::Outputs(outputs)) => outputs,
        _ => HashMap::new(),
    };
    let mut outputs: HashMap<String, Output> = HashMap::new();
    for (name, output) in output_list {
        outputs.insert(name.clone(), output);
        println!("Registered output: {}", name);
    }

    // Capture events
    if matches!(reply, Ok(Response::Handled)) {
        let mut read_event = event_socket.read_events();

        // Create a window(s)/workspace map
        let mut workspace_windows: HashMap<u64, Vec<u64>> = HashMap::new();

        // Loop over events and filter the ones about windows being opened or closed
        // Update the window(s)/workspace map when events are matched
        while let Ok(event) = read_event() {
            state.apply(event.clone()); // https://github.com/Antiz96/oniri/issues/3

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
                            // https://github.com/Antiz96/oniri/issues/3
                            if !is_maximized(&state, &outputs, id) {
                                let _ = action_socket
                                    .send(Request::Action(niri_ipc::Action::MaximizeColumn {}));
                                println!("Maximized window {}", id);
                            }
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
                            // https://github.com/Antiz96/oniri/issues/3
                            if !is_maximized(&state, &outputs, id) {
                                let _ = action_socket
                                    .send(Request::Action(niri_ipc::Action::MaximizeColumn {}));
                                println!("Maximized window {}", id);
                            }
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

// Hack to determine if a window is supposedly already maximized or not
// based on a comparison between the window size and the output size.
// This is to workaround the lack of a window "maximized" state to gather from the IPC,
// and/or the lack of a way to set/unset the maximize state (rather than just toggling it).
// See https://github.com/Antiz96/oniri/issues/3 for more details.
fn is_maximized(
    state: &EventStreamState,
    outputs: &HashMap<String, Output>,
    window_id: u64,
) -> bool {
    let tol_w = 150;
    let tol_h = 150;

    let window = match state.windows.windows.get(&window_id) {
        Some(w) => w,
        None => {
            println!("Window {} not found in state", window_id);
            return false;
        }
    };

    if window.is_floating {
        println!("Window {} is floating, skipping", window_id);
        return false;
    }

    let workspace = match window
        .workspace_id
        .and_then(|ws_id| state.workspaces.workspaces.get(&ws_id))
    {
        Some(ws) => ws,
        None => {
            println!("Workspace for window {} not found", window_id);
            return false;
        }
    };

    let logical = match workspace
        .output
        .as_ref()
        .and_then(|name| outputs.get(name))
        .and_then(|o| o.logical.as_ref())
    {
        Some(l) => l,
        None => {
            println!(
                "Output for workspace {} not found or has no logical size",
                workspace.id
            );
            return false;
        }
    };

    let out_w = logical.width as i32;
    let out_h = logical.height as i32;
    let (tile_w, tile_h) = window.layout.tile_size;
    let tile_w = tile_w as i32;
    let tile_h = tile_h as i32;

    println!(
        "Window {}: out_w={}, out_h={}, tile_w={}, tile_h={}, tol_w={}, tol_h={}",
        window_id, out_w, out_h, tile_w, tile_h, tol_w, tol_h
    );

    let width_ok = (out_w - tile_w).abs() <= tol_w;
    let height_ok = (out_h - tile_h).abs() <= tol_h;

    println!(
        "Window {}: width_ok={}, height_ok={}, maximized={}",
        window_id,
        width_ok,
        height_ok,
        width_ok && height_ok
    );

    width_ok && height_ok
}
