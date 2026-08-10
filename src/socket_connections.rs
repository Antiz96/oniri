//! Initialize niri IPC socket connections and event stream

use anyhow::Context;
use niri_ipc::{Request, Response, socket::Socket};

pub fn init_socket_connections() -> anyhow::Result<(Socket, Socket)> {
    // Connect to niri IPC socket
    let mut event_socket =
        Socket::connect().context("Failed to connect to niri IPC socket (event socket)")?;

    // Start the event stream
    let reply = event_socket
        .send(Request::EventStream)
        .context("Failed to send event stream request to niri IPC")?;

    if !matches!(reply, Ok(Response::Handled)) {
        return Err(anyhow::anyhow!("Failed to start niri IPC event stream"));
    }

    // Create a separate socket connection to send actions
    let action_socket =
        Socket::connect().context("Failed to connect to niri IPC socket (action socket)")?;

    // Return both sockets connections so they can be called elsewhere
    Ok((event_socket, action_socket))
}
