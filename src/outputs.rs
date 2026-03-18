// This can be dropped once https://github.com/Antiz96/oniri/issues/3 is solved

// Import modules
use niri_ipc::{Output, Request, Response, socket::Socket};
use std::collections::HashMap;

pub fn outputs_maps ( action_socket: &mut Socket, ) -> anyhow::Result<HashMap<String, Output>> {
    // Gather state and create an outputs map
    // This is used later to workaround some limitations of the niri IPC
    // See https://github.com/Antiz96/oniri/issues/3
    let response = action_socket.send(Request::Outputs)?;

    let outputs = match response {
        Ok(Response::Outputs(outputs)) => outputs,
        _ => HashMap::new(),
    };

    for name in outputs.keys() {
        println!("Registered output: {}", name);
    }

    Ok(outputs)
}
