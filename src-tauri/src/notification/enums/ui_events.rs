#[derive(PartialEq, Debug, Clone)]
pub enum UIEvent {
    OnStartingUp,
    UpdateUser,
    // WebSocket use if the WebSocket is connected or not
    OnError,
}
impl UIEvent {
    pub fn as_str(&self) -> &str {
        match *self {
            UIEvent::OnStartingUp => "App:StartingUp",
            UIEvent::UpdateUser => "User:Update",
            UIEvent::OnError => "App:Error",
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum UIOperationEvent {
    CreateOrUpdate,
    Delete,
    Set,
}
impl UIOperationEvent {
    pub fn as_str(&self) -> &str {
        match *self {
            UIOperationEvent::CreateOrUpdate => "CREATE_OR_UPDATE",
            UIOperationEvent::Delete => "DELETE",
            UIOperationEvent::Set => "SET",
        }
    }
}
