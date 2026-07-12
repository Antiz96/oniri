//! Initialize niri IPC socket connections and event stream

use log::error;
use niri_ipc::{Request, Response, socket::Socket};
use std::process;

pub fn init_socket_connections() -> anyhow::Result<(Socket, Socket)> {
    // Connect to niri IPC socket
    let mut event_socket = Socket::connect().unwrap_or_else(|error| {
        error!("Failed to connect to niri IPC socket: {error}");
        process::exit(2);
    });

    // Start the event stream
    let reply = event_socket.send(Request::EventStream)?;
    if !matches!(reply, Ok(Response::Handled)) {
        error!("Failed to start event stream: {:?}", reply);
        return Err(anyhow::anyhow!(""));
    }

    // Create a separate socket connection to send actions
    let action_socket = Socket::connect().unwrap_or_else(|error| {
        error!("Failed to connect to niri IPC socket: {error}");
        process::exit(2);
    });

    // Return both sockets connections so they can be called elsewhere
    Ok((event_socket, action_socket))
}
