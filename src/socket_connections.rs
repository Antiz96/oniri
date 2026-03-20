// Import external modules
use log::error;
use niri_ipc::{Request, Response, socket::Socket};

pub fn init_socket_connections() -> anyhow::Result<(Socket, Socket)> {
    // Connect to niri IPC socket
    let mut event_socket = Socket::connect().map_err(|e| {
        error!("Failed to connect to event socket: {}", e);
        e
    })?;

    // Start the event stream
    let reply = event_socket.send(Request::EventStream)?;
    if !matches!(reply, Ok(Response::Handled)) {
        error!("Failed to start event stream: {:?}", reply);
        return Err(anyhow::anyhow!(""));
    }

    // Create a separate socket connection to send actions
    let action_socket = Socket::connect().map_err(|e| {
        error!("Failed to connect to action socket: {}", e);
        e
    })?;

    // Return both sockets connections so they can be called elsewhere
    Ok((event_socket, action_socket))
}
