//! Create an outputs map, used later for window/output size comparison,
//! used as a workaround for some limitations of the niri IPC
//! This can be dropped once https://github.com/Antiz96/oniri/issues/3 is solved

use anyhow::{Context, anyhow};
use log::info;
use niri_ipc::{Output, Request, Response, socket::Socket};
use std::collections::HashMap;

pub fn init_outputs_map(action_socket: &mut Socket) -> anyhow::Result<HashMap<String, Output>> {
    let response = action_socket
        .send(Request::Outputs)
        .context("Failed to get outputs list")?;

    let Ok(Response::Outputs(outputs)) = response else {
        return Err(anyhow!(
            "Unexpected response to outputs request by the niri IPC"
        ));
    };

    for name in outputs.keys() {
        info!("Registered output: {name}");
    }

    Ok(outputs)
}
