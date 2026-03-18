// Import external modules
use niri_ipc::{Output, Request, socket::Socket};
use std::collections::HashMap;

// Import internal modules
use crate::sizecompare::is_maximized; // https://github.com/Antiz96/oniri/issues/3

// Check if there's only one window in the workspace/window(s) map & maximize it if so,
// unless it's maximized already (https://github.com/Antiz96/oniri/issues/3)
pub fn maximize_window_if_alone(
    workspace_windows: &HashMap<u64, Vec<u64>>,
    state: &niri_ipc::state::EventStreamState, // https://github.com/Antiz96/oniri/issues/3
    outputs: &HashMap<String, Output>,         // https://github.com/Antiz96/oniri/issues/3
    tol_h: i32,                                // https://github.com/Antiz96/oniri/issues/3
    tol_w: i32,                                // https://github.com/Antiz96/oniri/issues/3
    action_socket: &mut Socket,
) -> anyhow::Result<()> {
    for windows in workspace_windows.values() {
        if windows.len() == 1 {
            let id = windows[0];
            // https://github.com/Antiz96/oniri/issues/3
            if !is_maximized(state, outputs, id, tol_h, tol_w) {
                let _ = action_socket.send(Request::Action(niri_ipc::Action::MaximizeColumn {}));
                println!("Maximized window {}", id);
            }
        }
    }
    Ok(())
}
