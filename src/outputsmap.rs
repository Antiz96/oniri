// Import external modules
use niri_ipc::{Output, Request, Response, socket::Socket};
use std::collections::HashMap;

// Create an outputs map, used later for window/output size comparison,
// used as a workaround for some limitations of the niri IPC
// This can be dropped once https://github.com/Antiz96/oniri/issues/3 is solved
pub fn outputs_map(action_socket: &mut Socket) -> anyhow::Result<HashMap<String, Output>> {
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
