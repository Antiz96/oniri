//! Hack to determine if a window is supposedly already maximized or not
//! based on a comparison between the window size and the output size.
//! This is to workaround the lack of a window "maximized" state to gather from the IPC,
//! and/or the lack of a way to set/unset the maximize state (rather than just toggling it).
//! This can be dropped once https://github.com/Antiz96/oniri/issues/3 is resolved.

use log::{debug, info, warn};
use niri_ipc::{Output, state::EventStreamState};
use std::collections::HashMap;

pub fn is_maximized(
    state: &EventStreamState,
    outputs: &HashMap<String, Output>,
    window_id: u64,
    tol_w: i32,
    tol_h: i32,
) -> bool {
    let window = match state.windows.windows.get(&window_id) {
        Some(w) => w,
        None => {
            warn!("Window {window_id} not found in state");
            return false;
        }
    };

    if window.is_floating {
        info!("Window {window_id} is floating, skipping");
        return false;
    }

    let workspace = match window
        .workspace_id
        .and_then(|ws_id| state.workspaces.workspaces.get(&ws_id))
    {
        Some(ws) => ws,
        None => {
            warn!("Workspace for window {window_id} not found");
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
            warn!(
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

    debug!(
        "Window {window_id}: out_w={out_w}, out_h={out_h}, tile_w={tile_w}, tile_h={tile_h}, tol_w={tol_w}, tol_h={tol_h}"
    );

    let width_ok = (out_w - tile_w).abs() <= tol_w;
    let height_ok = (out_h - tile_h).abs() <= tol_h;

    debug!(
        "Window {window_id}: width_ok={width_ok}, height_ok={height_ok}, maximized={}",
        width_ok && height_ok
    );

    width_ok && height_ok
}
