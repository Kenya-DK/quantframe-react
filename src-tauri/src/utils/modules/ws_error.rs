#[derive(Debug, Eq, PartialEq)]
pub enum WsError {
    ReservedPath(String),
    InvalidPath(String),
    AlreadyRegistered(String),
    InvalidMessageReceived(String),
    ConnectionError,
    InvalidMessage,
    SendError(String),
    NotConnected,
}
