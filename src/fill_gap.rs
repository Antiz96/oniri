//! Nudge focus left/right after a window closes, forcing niri to rescroll
//! the viewport and eliminate any leftover gap on the edge of the screen.

use log::info;
use niri_ipc::{Action, Request, socket::Socket};

pub fn fill_gap(
    socket: &mut Socket,
    move_on_close: bool,
    remaining_windows: usize,
) -> anyhow::Result<()> {
    if !move_on_close || remaining_windows <= 1 {
        return Ok(());
    }

    let _ = socket.send(Request::Action(Action::FocusColumnLeft {}));
    let _ = socket.send(Request::Action(Action::FocusColumnRight {}));
    info!("Nudged focus to fill gap left by closed window");

    Ok(())
}
