//! Create a workspace/window(s) map and initialize it

// Import external modules
use niri_ipc::{Request, Response, socket::Socket};
use std::collections::HashMap;

pub fn init_windows_map(action_socket: &mut Socket) -> anyhow::Result<HashMap<u64, Vec<u64>>> {
    let response = action_socket.send(Request::Windows)?;

    let Ok(Response::Windows(windows)) = response else {
        return Ok(HashMap::new());
    };

    let mut workspace_windows: HashMap<u64, Vec<u64>> = HashMap::new();

    for window in windows {
        // Skip floating windows (they cannot/should not be maximized)
        if window.is_floating {
            continue;
        }

        if let Some(ws) = window.workspace_id {
            workspace_windows.entry(ws).or_default().push(window.id);
        }
    }

    Ok(workspace_windows)
}
