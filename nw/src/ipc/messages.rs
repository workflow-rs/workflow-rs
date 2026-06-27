use crate::ipc::error::Error;
use crate::ipc::imports::*;
use borsh::{BorshDeserialize, BorshSerialize};
use js_sys::{ArrayBuffer, Uint8Array};
use std::fmt::Debug;

/// Serializes the given header and appends the payload bytes, returning a
/// JavaScript [`ArrayBuffer`] suitable for transmission over the IPC channel.
pub fn to_msg<Ops, Id>(header: BorshHeader<Id>, payload: &[u8]) -> Result<ArrayBuffer>
where
    Id: IdT,
    Ops: BorshSerialize + BorshDeserialize,
{
    let header = borsh::to_vec(&header).expect("to_msg header serialize error");
    // log_info!("header: {:?}", header);
    // log_info!("payload: {:?}", payload);
    let header_len = header.len();
    let len = payload.len() + header_len;
    let mut buffer = Vec::with_capacity(len);
    #[allow(clippy::uninit_vec)]
    unsafe {
        buffer.set_len(len);
    }
    buffer[0..header_len].copy_from_slice(&header);
    buffer[header_len..].copy_from_slice(payload);
    // log_info!("to_msg buffer: {:?}", buffer);

    let array = Uint8Array::from(&buffer[..]);
    Ok(array.buffer())
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
#[repr(u8)]
#[borsh(use_discriminant = true)]
/// Discriminates the three kinds of IPC messages exchanged between peers.
pub enum MessageKind {
    /// A one-way message that expects no response.
    Notification = 0,
    /// A message expecting a matching [`Response`](Self::Response).
    Request = 1,
    /// A reply to a previously received [`Request`](Self::Request).
    Response = 2,
}

impl From<MessageKind> for u32 {
    fn from(kind: MessageKind) -> u32 {
        kind as u32
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
/// Header prefixed to every IPC message, encoding the message kind, an
/// optional correlation id, and the Borsh-serialized operation discriminator.
pub struct BorshHeader<Id = Id64>
where
    Id: BorshSerialize + BorshDeserialize,
{
    /// Whether this message is a notification, request or response.
    pub kind: MessageKind,
    /// Correlation id linking a request to its response (absent for notifications).
    pub id: Option<Id>,
    /// Borsh-serialized bytes of the operation identifier.
    pub op: Vec<u8>,
}

impl<Id> BorshHeader<Id>
where
    Id: BorshSerialize + BorshDeserialize,
{
    /// Constructs a header for a request message carrying the given operation
    /// and an optional id used to correlate the eventual response.
    pub fn request<Ops>(id: Option<Id>, op: Ops) -> Self
    where
        Ops: OpsT,
    {
        BorshHeader {
            id,
            op: borsh::to_vec(&op).expect("request op serialize error"),
            kind: MessageKind::Request,
        }
    }

    /// Constructs a header for a response message carrying the given operation
    /// and the id of the request it answers.
    pub fn response<Ops>(id: Option<Id>, op: Ops) -> Self
    where
        Ops: OpsT,
    {
        BorshHeader {
            id,
            op: borsh::to_vec(&op).expect("response op serialize error"),
            kind: MessageKind::Response,
        }
    }

    /// Constructs a header for a notification message carrying the given
    /// operation (notifications have no id since no response is expected).
    pub fn notification<Ops>(op: Ops) -> Self
    where
        Ops: OpsT,
    {
        BorshHeader {
            id: None,
            op: borsh::to_vec(&op).expect("notification op serialize error"),
            kind: MessageKind::Notification,
        }
    }
}

#[derive(Debug)]
/// A decoded IPC message, pairing its [`BorshHeader`] with a borrowed slice of
/// the raw payload bytes that follow the header.
pub struct BorshMessage<'data, Id = Id64>
where
    Id: BorshSerialize + BorshDeserialize + 'data,
{
    /// Decoded message header describing the kind, id and operation.
    pub header: BorshHeader<Id>,
    /// Raw payload bytes that follow the header in the encoded message.
    pub payload: &'data [u8],
}

impl<'data, Id> TryFrom<&'data Vec<u8>> for BorshMessage<'data, Id>
where
    Id: Debug + BorshSerialize + BorshDeserialize + 'data,
{
    type Error = Error;

    fn try_from(src: &'data Vec<u8>) -> std::result::Result<Self, Self::Error> {
        let v: BorshMessage<Id> = src[..].try_into()?;
        Ok(v)
    }
}

impl<'data, Id> TryFrom<&'data [u8]> for BorshMessage<'data, Id>
where
    Id: Debug + BorshSerialize + BorshDeserialize + 'data,
{
    type Error = Error;

    fn try_from(src: &'data [u8]) -> std::result::Result<Self, Self::Error> {
        let mut payload = src;
        let header = BorshHeader::<Id>::deserialize(&mut payload)?;
        let message = BorshMessage { header, payload };
        Ok(message)
    }
}
