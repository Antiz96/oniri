// Import modules
use niri_ipc::{Request, Response, socket::Socket};

pub fn initialize_socket_connections() -> anyhow::Result<(Socket, Socket)> {
    // Connect to niri IPC socket, start the event stream and gather events
    let mut event_socket = Socket::connect()?;
    let reply = event_socket.send(Request::EventStream)?;
    if !matches!(reply, Ok(Response::Handled)) {
        anyhow::bail!("Failed to start event stream");
    }

    // Create a separate socket connection to send actions
    let action_socket = Socket::connect()?;

    // Return both sockets connections so they can be called elsewhere
    Ok((event_socket, action_socket))
}
