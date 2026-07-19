//! Helper for maximizing a window

use log::info;
use niri_ipc::state::EventStreamState;
use niri_ipc::{Action, Request, socket::Socket};

pub fn maximize_window(
    socket: &mut Socket,
    state: &EventStreamState,
    window_id: u64,
    edges_maximizing: bool,
) -> anyhow::Result<()> {
    if edges_maximizing {
        socket
            .send(Request::Action(Action::MaximizeWindowToEdges {
                id: Some(window_id),
            }))?
            .map_err(anyhow::Error::msg)?;
        info!("Maximized window to edges {window_id}");
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

        socket
            .send(Request::Action(Action::FocusWindow { id: window_id }))?
            .map_err(anyhow::Error::msg)?;
        socket
            .send(Request::Action(Action::MaximizeColumn {}))?
            .map_err(anyhow::Error::msg)?;
        socket
            .send(Request::Action(Action::FocusWindow { id: focused_id }))?
            .map_err(anyhow::Error::msg)?;
        info!("Maximized window {window_id}");
    }
    Ok(())
}
