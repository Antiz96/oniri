//! Nudge focus left/right after a window closes, forcing niri to rescroll the viewport
//! eliminate any leftover gap on the edge of the screen to fill available screen space.

use log::info;
use niri_ipc::state::EventStreamState;
use niri_ipc::{Action, Request, socket::Socket};

pub fn is_leftmost(state: &EventStreamState, window_id: u64) -> bool {
    let window = state
        .windows
        .windows
        .get(&window_id)
        .expect("Window ID not found in state");
    let (column, _) = window
        .layout
        .pos_in_scrolling_layout
        .expect("Window has no position in scrolling layout");

    column == 1
}

pub fn nudge_focus(socket: &mut Socket) -> anyhow::Result<()> {
    let _ = socket.send(Request::Action(Action::FocusColumnLeft {}));
    let _ = socket.send(Request::Action(Action::FocusColumnRight {}));

    info!("Nudged focus to fill gap left by closed window");

    Ok(())
}
