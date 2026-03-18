// Import modules
use niri_ipc::{Output, state::EventStreamState};
use std::collections::HashMap;

// Hack to determine if a window is supposedly already maximized or not
// based on a comparison between the window size and the output size.
// This is to workaround the lack of a window "maximized" state to gather from the IPC,
// and/or the lack of a way to set/unset the maximize state (rather than just toggling it).
// This can be dropped once https://github.com/Antiz96/oniri/issues/3 is resolved.
pub fn is_maximized(
    state: &EventStreamState,
    outputs: &HashMap<String, Output>,
    window_id: u64,
) -> bool {
    let tol_w = 150;
    let tol_h = 150;

    let window = match state.windows.windows.get(&window_id) {
        Some(w) => w,
        None => {
            println!("Window {} not found in state", window_id);
            return false;
        }
    };

    if window.is_floating {
        println!("Window {} is floating, skipping", window_id);
        return false;
    }

    let workspace = match window
        .workspace_id
        .and_then(|ws_id| state.workspaces.workspaces.get(&ws_id))
    {
        Some(ws) => ws,
        None => {
            println!("Workspace for window {} not found", window_id);
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
            println!(
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

    println!(
        "Window {}: out_w={}, out_h={}, tile_w={}, tile_h={}, tol_w={}, tol_h={}",
        window_id, out_w, out_h, tile_w, tile_h, tol_w, tol_h
    );

    let width_ok = (out_w - tile_w).abs() <= tol_w;
    let height_ok = (out_h - tile_h).abs() <= tol_h;

    println!(
        "Window {}: width_ok={}, height_ok={}, maximized={}",
        window_id,
        width_ok,
        height_ok,
        width_ok && height_ok
    );

    width_ok && height_ok
}
