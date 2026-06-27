//!
//! RPC message serialization module (header serialization and deserialization for `Borsh` and `JSON` data structures)
//!

pub mod serde_json {
    //! RPC message serialization for JSON encoding
    use serde::{Deserialize, Serialize};
    use serde_json::{self, Value};

    #[derive(Debug, Serialize, Deserialize)]
    /// JSON-encoded request message sent from the client to the server.
    pub struct JsonClientMessage<Ops, Id> {
        // pub jsonrpc: String,
        /// Optional request id used to correlate the server response.
        pub id: Option<Id>,
        /// Operation (method) being requested.
        pub method: Ops,
        /// Request params payload.
        pub params: Value,
    }

    impl<Ops, Id> JsonClientMessage<Ops, Id> {
        /// Create a new JSON client request from an optional request id, the
        /// requested method and its params payload.
        pub fn new(id: Option<Id>, method: Ops, payload: Value) -> Self {
            JsonClientMessage {
                // jsonrpc: "2.0".to_owned(),
                id,
                method,
                params: payload,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    /// JSON-encoded message sent from the server to the client, used for
    /// responses, errors and notifications.
    pub struct JSONServerMessage<Ops, Id> {
        // pub jsonrpc: String,
        /// Id of the request this message responds to, if any.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<Id>,
        /// Operation (method) associated with the message, if any.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub method: Option<Ops>,
        /// Result or notification payload, if any.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub params: Option<Value>,
        // #[serde(skip_serializing_if = "Option::is_none")]
        // pub result: Option<Value>,
        /// Error detail, present when the message reports a failure.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error: Option<JsonServerError>,
    }

    impl<Ops, Id> JSONServerMessage<Ops, Id> {
        /// Create a new JSON server message from an optional request id, method,
        /// params and error.
        pub fn new(
            id: Option<Id>,
            method: Option<Ops>,
            params: Option<Value>,
            // result: Option<Value>,
            error: Option<JsonServerError>,
        ) -> Self {
            JSONServerMessage {
                // jsonrpc: "2.0".to_owned(),
                method,
                params,
                // result,
                error,
                id,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    /// JSON-RPC style server error carrying a numeric code, a human-readable
    /// message and an optional structured data payload.
    pub struct JsonServerError {
        code: u64,
        message: String,
        data: Option<Value>,
    }

    impl std::fmt::Display for JsonServerError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
            write!(
                f,
                "code:{}  message:`{}` data:{:?}",
                self.code, self.message, self.data
            )
        }
    }

    impl From<crate::error::ServerError> for JsonServerError {
        fn from(err: crate::error::ServerError) -> Self {
            JsonServerError {
                code: 0, //err.code,
                message: err.to_string(),
                data: None, //err.data,
            }
        }
    }
}

pub mod borsh {
    //! RPC message serialization for Borsh encoding

    use crate::error::Error;
    use borsh::{BorshDeserialize, BorshSerialize};
    use workflow_websocket::client::message::Message as WebSocketMessage;
    // use borsh::de::*;

    /// Serialize a request header and payload into a single binary WebSocket
    /// message (the serialized header followed by the raw payload bytes).
    pub fn to_ws_msg<Ops, Id>(header: BorshReqHeader<Ops, Id>, payload: &[u8]) -> WebSocketMessage
    where
        Id: BorshSerialize + BorshDeserialize,
        Ops: BorshSerialize + BorshDeserialize,
    {
        let header = borsh::to_vec(&header).expect("to_ws_msg header serialize error");
        let header_len = header.len();
        let len = payload.len() + header_len;
        let mut buffer = Vec::with_capacity(len);
        #[allow(clippy::uninit_vec)]
        unsafe {
            buffer.set_len(len);
        }
        buffer[0..header_len].copy_from_slice(&header);
        buffer[header_len..].copy_from_slice(payload);
        buffer.into()
    }

    #[derive(Debug, BorshSerialize, BorshDeserialize)]
    /// Header of a Borsh-encoded client request, carrying the optional request
    /// id (used to correlate the response) and the requested operation.
    pub struct BorshReqHeader<Ops, Id>
    where
        Id: BorshSerialize + BorshDeserialize,
        Ops: BorshSerialize + BorshDeserialize,
    {
        /// Optional request id used to correlate the server response.
        pub id: Option<Id>, //u64,
        /// Operation (method) being requested.
        pub op: Ops,
    }

    impl<Ops, Id> BorshReqHeader<Ops, Id>
    where
        Id: BorshSerialize + BorshDeserialize,
        Ops: BorshSerialize + BorshDeserialize,
    {
        /// Create a new request header from an optional request id and an operation.
        pub fn new(id: Option<Id>, op: Ops) -> Self {
            BorshReqHeader { id, op }
        }
    }

    #[derive(Debug, BorshSerialize, BorshDeserialize)]
    /// Header of a Borsh-encoded server message, identifying the request it
    /// responds to (if any), its kind and the associated operation.
    pub struct BorshServerMessageHeader<Ops, Id> {
        /// Id of the request this message responds to, if any.
        pub id: Option<Id>, //u64,
        /// Whether the message is a success, error or notification.
        pub kind: ServerMessageKind,
        /// Operation associated with the message, if any.
        pub op: Option<Ops>,
    }

    impl<Ops, Id> BorshServerMessageHeader<Ops, Id>
    // where
    //     Id: Default,
    {
        /// Create a new server message header from a request id, message kind
        /// and an optional operation.
        pub fn new(id: Option<Id>, kind: ServerMessageKind, op: Option<Ops>) -> Self {
            Self { id, kind, op }
        }
    }

    #[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
    #[borsh(use_discriminant = true)]
    /// Discriminant identifying the nature of a server message.
    pub enum ServerMessageKind {
        /// A successful response to a client request.
        Success = 0,
        /// An error response to a client request.
        Error = 1,
        /// A server-initiated notification (not tied to a request).
        Notification = 0xff,
    }

    impl From<ServerMessageKind> for u32 {
        fn from(kind: ServerMessageKind) -> u32 {
            kind as u32
        }
    }

    #[derive(Debug)]
    /// Error outcome of a server response, distinguishing between a missing
    /// payload, a typed error payload, and a transport/RPC-level error.
    pub enum RespError<T>
    where
        T: BorshDeserialize,
    {
        /// The error response carried no associated data.
        NoData,
        /// The error response carried a deserialized typed payload.
        Data(T),
        /// A transport or RPC-level error occurred.
        Rpc(Error),
    }

    #[derive(Debug)]
    /// A Borsh-encoded client request message combining a header with a
    /// borrowed, already-serialized payload slice.
    pub struct BorshClientMessage<'data, Ops, Id>
    where
        Id: BorshSerialize + BorshDeserialize + 'data,
        Ops: BorshSerialize + BorshDeserialize + 'data,
    {
        /// Request header carrying the optional request id and operation.
        pub header: BorshReqHeader<Ops, Id>,
        /// Raw (already serialized) request payload bytes.
        pub payload: &'data [u8],
    }

    impl<'data, Ops, Id> TryFrom<&'data Vec<u8>> for BorshClientMessage<'data, Ops, Id>
    where
        Id: BorshSerialize + BorshDeserialize + 'data,
        Ops: BorshSerialize + BorshDeserialize + 'data,
    {
        type Error = Error;

        fn try_from(src: &'data Vec<u8>) -> Result<Self, Self::Error> {
            let v: BorshClientMessage<Ops, Id> = src[..].try_into()?;
            Ok(v)
        }
    }

    impl<'data, Ops, Id> TryFrom<&'data [u8]> for BorshClientMessage<'data, Ops, Id>
    where
        Id: BorshSerialize + BorshDeserialize + 'data,
        Ops: BorshSerialize + BorshDeserialize + 'data,
    {
        type Error = Error;

        fn try_from(src: &'data [u8]) -> Result<Self, Self::Error> {
            let mut payload = src;
            let header = BorshReqHeader::<Ops, Id>::deserialize(&mut payload)?;
            let message = BorshClientMessage { header, payload };
            Ok(message)
        }
    }

    #[derive(Debug)]
    /// A Borsh-encoded server message combining a header with a borrowed,
    /// already-serialized payload slice.
    pub struct BorshServerMessage<'data, Ops, Id>
    where
        Id: BorshSerialize + BorshDeserialize + 'data,
        Ops: BorshSerialize + BorshDeserialize + 'data,
    {
        /// Message header carrying the request id, message kind and operation.
        pub header: BorshServerMessageHeader<Ops, Id>,
        /// Raw (already serialized) message payload bytes.
        pub payload: &'data [u8],
    }

    impl<'data, Ops, Id> BorshServerMessage<'data, Ops, Id>
    where
        Id: BorshSerialize + BorshDeserialize + 'data,
        Ops: BorshSerialize + BorshDeserialize + 'data,
    {
        /// Create a new server message from a header and a borrowed payload slice.
        pub fn new(
            header: BorshServerMessageHeader<Ops, Id>,
            payload: &'data [u8],
        ) -> BorshServerMessage<'data, Ops, Id> {
            BorshServerMessage { header, payload }
        }

        /// Serialize the header and payload into a single contiguous
        /// byte buffer (header followed by the raw payload bytes).
        pub fn try_to_vec(&self) -> Result<Vec<u8>, Error> {
            let header = borsh::to_vec(&self.header)?;
            let header_len = header.len();

            let len = header_len + self.payload.len();
            let mut buffer = Vec::with_capacity(len);
            #[allow(clippy::uninit_vec)]
            unsafe {
                buffer.set_len(len);
            }

            buffer[0..header_len].copy_from_slice(&header);
            if !self.payload.is_empty() {
                buffer[header_len..].copy_from_slice(self.payload);
            }
            Ok(buffer)
        }
    }

    impl<'data, Ops, Id> TryFrom<&'data [u8]> for BorshServerMessage<'data, Ops, Id>
    where
        Id: BorshSerialize + BorshDeserialize + 'data,
        Ops: BorshSerialize + BorshDeserialize + 'data,
    {
        type Error = Error;

        fn try_from(src: &'data [u8]) -> Result<Self, Self::Error> {
            let mut payload = src;
            let header = <BorshServerMessageHeader<Ops, Id>>::deserialize(&mut payload)?;
            let message = BorshServerMessage { header, payload };
            Ok(message)
        }
    }
}
