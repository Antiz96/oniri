// Import external modules
use log::{debug, info, warn};
use niri_ipc::{Output, state::EventStreamState};
use std::{collections::HashMap, env};

// Hack to determine if a window is supposedly already maximized or not
// based on a comparison between the window size and the output size.
// This is to workaround the lack of a window "maximized" state to gather from the IPC,
// and/or the lack of a way to set/unset the maximize state (rather than just toggling it).
// This can be dropped once https://github.com/Antiz96/oniri/issues/3 is resolved.

// Fetch height and width tolerances from CLI, with defaults
pub fn set_tolerances() -> (i32, i32) {
    let mut tol_h = 150;
    let mut tol_w = 150;

    let mut args = env::args();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-H" | "--height-tolerance" => {
                if let Some(value) = args.next() {
                    if let Ok(val) = value.parse::<i32>() {
                        tol_h = val;
                    } else {
                        warn!("Invalid value for {}: {}", arg, value);
                    }
                }
            }
            "-W" | "--width-tolerance" => {
                if let Some(value) = args.next() {
                    if let Ok(val) = value.parse::<i32>() {
                        tol_w = val;
                    } else {
                        warn!("Invalid value for {}: {}", arg, value);
                    }
                }
            }
            _ => {}
        }
    }

    (tol_h, tol_w)
}

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
            warn!("Window {} not found in state", window_id);
            return false;
        }
    };

    if window.is_floating {
        info!("Window {} is floating, skipping", window_id);
        return false;
    }

    let workspace = match window
        .workspace_id
        .and_then(|ws_id| state.workspaces.workspaces.get(&ws_id))
    {
        Some(ws) => ws,
        None => {
            warn!("Workspace for window {} not found", window_id);
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
        "Window {}: out_w={}, out_h={}, tile_w={}, tile_h={}, tol_w={}, tol_h={}",
        window_id, out_w, out_h, tile_w, tile_h, tol_w, tol_h
    );

    let width_ok = (out_w - tile_w).abs() <= tol_w;
    let height_ok = (out_h - tile_h).abs() <= tol_h;

    debug!(
        "Window {}: width_ok={}, height_ok={}, maximized={}",
        window_id,
        width_ok,
        height_ok,
        width_ok && height_ok
    );

    width_ok && height_ok
}
