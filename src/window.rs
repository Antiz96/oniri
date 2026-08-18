//! Helper for maximizing a window

use anyhow::Context;
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
            }))
            .context("Failed to send maximize-to-edges action")?
            .map_err(anyhow::Error::msg)
            .context("Failed to maximize window to edges")?;

        info!("Maximized window to edges {window_id}");
    } else {
        // We need this information to restore focus state after maximizing @window_id
        let Some(focused_id) = state
            .windows
            .windows
            .values()
            .find_map(|window| window.is_focused.then_some(window.id))
            .or_else(|| {
                state
                    .workspaces
                    .workspaces
                    .values()
                    .find(|workspace| workspace.is_focused)
                    .and_then(|workspace| workspace.active_window_id)
            })
        else {
            return Ok(());
        };

        socket
            .send(Request::Action(Action::FocusWindow { id: window_id }))
            .context("Failed to send focus-window action")?
            .map_err(anyhow::Error::msg)
            .context("Failed to focus window")?;

        socket
            .send(Request::Action(Action::MaximizeColumn {}))
            .context("Failed to send maximize-column action")?
            .map_err(anyhow::Error::msg)
            .context("Failed to maximize column")?;

        socket
            .send(Request::Action(Action::FocusWindow { id: focused_id }))
            .context("Failed to send focus-window restoration action")?
            .map_err(anyhow::Error::msg)
            .context("Niri failed to restore focused window")?;

        info!("Maximized window {window_id}");
    }
    Ok(())
}
