//! Nudge focus after a window is closed (if needed), forcing niri to
//! rescroll the viewport and eliminate any leftover gap on the edge of the screen.

use anyhow::Context;
use log::info;
use niri_ipc::state::EventStreamState;
use niri_ipc::{Action, Request, socket::Socket};

// Check if the newly focused window is the last one in the viewport (used to determine
// if we should nudge the focus)
pub fn is_last(state: &EventStreamState, focused_id: u64) -> bool {
    // Get the focused window from its ID
    let Some(window) = state.windows.windows.get(&focused_id) else {
        return false;
    };

    // Get column position of the window
    let Some((column, _)) = window.layout.pos_in_scrolling_layout else {
        return false;
    };

    // Check if the window's column is the last one
    let is_last_column = state
        .windows
        .windows
        .values()
        .filter(|other| other.workspace_id == window.workspace_id)
        .filter_map(|other| other.layout.pos_in_scrolling_layout)
        .all(|(other_column, _)| other_column <= column);

    // Check if the window is the only / last one in the column
    let is_only_window_in_column = state
        .windows
        .windows
        .values()
        .filter(|other| other.id != focused_id)
        .filter(|other| other.workspace_id == window.workspace_id)
        .filter_map(|other| other.layout.pos_in_scrolling_layout)
        .all(|(other_column, _)| other_column != column);

    is_last_column && is_only_window_in_column
}

// Nudge the focus to re-center the viewport
pub fn nudge_focus(socket: &mut Socket) -> anyhow::Result<()> {
    socket
        .send(Request::Action(Action::FocusColumnLeft {}))
        .context("Failed to send focus-left action")?
        .map_err(anyhow::Error::msg)
        .context("Failed to nudge focus to the left")?;

    socket
        .send(Request::Action(Action::FocusColumnRight {}))
        .context("Failed to send focus-right action")?
        .map_err(anyhow::Error::msg)
        .context("Failed to nudge focus to the right")?;

    info!("Nudged focus to fill gap left by closed window");

    Ok(())
}
