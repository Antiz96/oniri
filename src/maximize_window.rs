//! Helper for maximizing a window, since before maximizing the window must be focused.

// Import external modules
use log::info;
use niri_ipc::state::EventStreamState;
use niri_ipc::{Request, socket::Socket};

pub fn maximize_window(
    socket: &mut Socket,
    state: &EventStreamState,
    window_id: u64,
    edges_maximizing: bool,
) -> anyhow::Result<()> {
    if edges_maximizing {
        let _ = socket.send(Request::Action(niri_ipc::Action::MaximizeWindowToEdges {
            id: Some(window_id),
        }));
        info!("Maximized window to edges {}", window_id);
    } else {
        // We need this information to restore focus state after maximizing @window_id
        let Some(focused_id) = state
            .windows
            .windows
            .values()
            .find_map(|window| window.is_focused.then_some(window.id))
        else {
            return Ok(());
        };

        let _ = socket.send(Request::Action(niri_ipc::Action::FocusWindow {
            id: window_id,
        }));
        let _ = socket.send(Request::Action(niri_ipc::Action::MaximizeColumn {}));
        let _ = socket.send(Request::Action(niri_ipc::Action::FocusWindow {
            id: focused_id,
        }));
        info!("Maximized window {}", window_id);
    }
    Ok(())
}
