use super::error::Error;
use std::sync::Arc;
use workflow_core::channel::*;

// /// Internal control message signaling WebSocket state
// /// change, WebSocket shutdown and other custom events.
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum Ctl {
//     /// Connection is opened
//     Open,
//     /// Connection is closed
//     Closed,
// }

/// The enum containing a client-side WebSocket message.
/// This enum defines the message type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Message {
    /// Text message
    Text(String),
    /// Binary message
    Binary(Vec<u8>),
    /// Connection has Opened
    Open,
    /// Connection has Closed
    Close,
}

impl From<Message> for Vec<u8> {
    fn from(msg: Message) -> Self {
        match msg {
            Message::Text(string) => string.into(),
            Message::Binary(vec) => vec,
            _ => {
                panic!(
                    "WebSocket - From<Message> for Vec<u8>: unsupported message type: {:?}",
                    msg
                );
            }
        }
    }
}

impl From<Vec<u8>> for Message {
    fn from(vec: Vec<u8>) -> Self {
        Message::Binary(vec)
    }
}

impl From<String> for Message {
    fn from(s: String) -> Self {
        Message::Text(s)
    }
}

impl From<&str> for Message {
    fn from(s: &str) -> Self {
        Message::Text(s.to_string())
    }
}

impl AsRef<[u8]> for Message {
    fn as_ref(&self) -> &[u8] {
        match self {
            Message::Text(string) => string.as_ref(),
            Message::Binary(vec) => vec.as_ref(),
            _ => {
                panic!(
                    "WebSocket - AsRef<[u8]> for Message: unsupported message type: {:?}",
                    self
                );
            }
        }
    }
}

// /// Wrapper for a WebSocket message being dispatched to the server.
// /// This enum represents a message `Post` or `WithAck` type that
// /// contains a callback that is invoked on successful message
// /// handoff to the underlying interface (a browser WebSocket interface
// /// of tokio WebSocket interface))
// #[derive(Clone)]
// pub enum DispatchMessage {
//     Post(Message),
//     WithAck(Message, Sender<Result<Arc<()>, Arc<Error>>>),
//     // DispatcherShutdown,
// }

pub type Ack = Option<Sender<Result<Arc<()>, Arc<Error>>>>;

// impl DispatchMessage {
//     pub fn is_ctl(&self) -> bool {
//         matches!(self, DispatchMessage::DispatcherShutdown)
//     }
// }
