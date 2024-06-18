//!
//! RPC message serialization module (header serialization and deserialization for `Borsh` and `JSON` data structures)
//!

pub mod serde_json {
    //! RPC message serialization for JSON encoding
    use serde::{Deserialize, Serialize};
    use serde_json::{self, Value};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct JsonClientMessage<Ops, Id> {
        // pub jsonrpc: String,
        pub id: Option<Id>,
        pub method: Ops,
        pub params: Value,
    }

    impl<Ops, Id> JsonClientMessage<Ops, Id> {
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
    pub struct JSONServerMessage<Ops, Id> {
        // pub jsonrpc: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<Id>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub method: Option<Ops>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub params: Option<Value>,
        // #[serde(skip_serializing_if = "Option::is_none")]
        // pub result: Option<Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error: Option<JsonServerError>,
    }

    impl<Ops, Id> JSONServerMessage<Ops, Id> {
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
    pub struct BorshReqHeader<Ops, Id>
    where
        Id: BorshSerialize + BorshDeserialize,
        Ops: BorshSerialize + BorshDeserialize,
    {
        pub id: Option<Id>, //u64,
        pub op: Ops,
    }

    impl<Ops, Id> BorshReqHeader<Ops, Id>
    where
        Id: BorshSerialize + BorshDeserialize,
        Ops: BorshSerialize + BorshDeserialize,
    {
        pub fn new(id: Option<Id>, op: Ops) -> Self {
            BorshReqHeader { id, op }
        }
    }

    #[derive(Debug, BorshSerialize, BorshDeserialize)]
    pub struct BorshServerMessageHeader<Ops, Id> {
        pub id: Option<Id>, //u64,
        pub kind: ServerMessageKind,
        pub op: Option<Ops>,
    }

    impl<Ops, Id> BorshServerMessageHeader<Ops, Id>
    // where
    //     Id: Default,
    {
        pub fn new(id: Option<Id>, kind: ServerMessageKind, op: Option<Ops>) -> Self {
            Self { id, kind, op }
        }
    }

    #[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
    #[borsh(use_discriminant = true)]
    pub enum ServerMessageKind {
        Success = 0,
        Error = 1,
        Notification = 0xff,
    }

    impl From<ServerMessageKind> for u32 {
        fn from(kind: ServerMessageKind) -> u32 {
            kind as u32
        }
    }

    #[derive(Debug)]
    pub enum RespError<T>
    where
        T: BorshDeserialize,
    {
        NoData,
        Data(T),
        Rpc(Error),
    }

    #[derive(Debug)]
    pub struct BorshClientMessage<'data, Ops, Id>
    where
        Id: BorshSerialize + BorshDeserialize + 'data,
        Ops: BorshSerialize + BorshDeserialize + 'data,
    {
        pub header: BorshReqHeader<Ops, Id>,
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
    pub struct BorshServerMessage<'data, Ops, Id>
    where
        Id: BorshSerialize + BorshDeserialize + 'data,
        Ops: BorshSerialize + BorshDeserialize + 'data,
    {
        pub header: BorshServerMessageHeader<Ops, Id>,
        pub payload: &'data [u8],
    }

    impl<'data, Ops, Id> BorshServerMessage<'data, Ops, Id>
    where
        Id: BorshSerialize + BorshDeserialize + 'data,
        Ops: BorshSerialize + BorshDeserialize + 'data,
    {
        pub fn new(
            header: BorshServerMessageHeader<Ops, Id>,
            payload: &'data [u8],
        ) -> BorshServerMessage<'data, Ops, Id> {
            BorshServerMessage { header, payload }
        }

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
