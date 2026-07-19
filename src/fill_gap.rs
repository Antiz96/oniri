//! Nudge focus left/right after a window closes, forcing niri to rescroll
//! the viewport and eliminate any leftover gap on the edge of the screen.

use log::info;
use niri_ipc::state::EventStreamState;
use niri_ipc::{Action, Request, socket::Socket};

pub fn is_leftmost_column(state: &EventStreamState, window_id: u64) -> bool {
    let Some(window) = state.windows.windows.get(&window_id) else {
        return false;
    };
    let Some((column, _)) = window.layout.pos_in_scrolling_layout else {
        return false;
    };

    state
        .windows
        .windows
        .values()
        .filter(|other| other.workspace_id == window.workspace_id)
        .filter_map(|other| other.layout.pos_in_scrolling_layout)
        .all(|(other_column, _)| other_column >= column)
}

pub fn fill_gap(
    socket: &mut Socket,
    move_on_close: bool,
    was_leftmost: bool,
    remaining_windows: usize,
) -> anyhow::Result<()> {
    if !move_on_close || was_leftmost || remaining_windows <= 1 {
        return Ok(());
    }

    let _ = socket.send(Request::Action(Action::FocusColumnLeft {}));
    let _ = socket.send(Request::Action(Action::FocusColumnRight {}));
    info!("Nudged focus to fill gap left by closed window");

    Ok(())
}
