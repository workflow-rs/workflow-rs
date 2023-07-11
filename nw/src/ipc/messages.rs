use crate::ipc::error::Error;
use crate::ipc::imports::*;
use borsh::{BorshDeserialize, BorshSerialize};
use js_sys::{ArrayBuffer, Uint8Array};
use std::fmt::Debug;

pub fn to_msg<Ops, Id>(header: BorshHeader<Ops, Id>, payload: &[u8]) -> Result<ArrayBuffer>
where
    Id: IdT,
    Ops: BorshSerialize + BorshDeserialize,
{
    let header = header.try_to_vec().expect("to_msg header serialize error");
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
#[repr(u32)]
pub enum MessageKind {
    Notification = 0,
    Request = 1,
    Response = 2,
}

impl From<MessageKind> for u32 {
    fn from(kind: MessageKind) -> u32 {
        kind as u32
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct BorshHeader<Ops, Id = Id64>
where
    Id: BorshSerialize + BorshDeserialize,
    Ops: BorshSerialize + BorshDeserialize,
{
    pub kind: MessageKind,
    pub id: Option<Id>,
    pub op: Ops,
}

impl<Ops, Id> BorshHeader<Ops, Id>
where
    Id: BorshSerialize + BorshDeserialize,
    Ops: BorshSerialize + BorshDeserialize,
{
    pub fn request(id: Option<Id>, op: Ops) -> Self {
        BorshHeader {
            id,
            op,
            kind: MessageKind::Request,
        }
    }

    pub fn response(id: Option<Id>, op: Ops) -> Self {
        BorshHeader {
            id,
            op,
            kind: MessageKind::Response,
        }
    }

    pub fn notification(op: Ops) -> Self {
        BorshHeader {
            id: None,
            op,
            kind: MessageKind::Notification,
        }
    }
}

#[derive(Debug)]
pub struct BorshMessage<'data, Ops, Id = Id64>
where
    Id: BorshSerialize + BorshDeserialize + 'data,
    Ops: BorshSerialize + BorshDeserialize + 'data,
{
    pub header: BorshHeader<Ops, Id>,
    pub payload: &'data [u8],
}

impl<'data, Ops, Id> TryFrom<&'data Vec<u8>> for BorshMessage<'data, Ops, Id>
where
    Id: Debug + BorshSerialize + BorshDeserialize + 'data,
    Ops: Debug + BorshSerialize + BorshDeserialize + 'data,
{
    type Error = Error;

    fn try_from(src: &'data Vec<u8>) -> std::result::Result<Self, Self::Error> {
        let v: BorshMessage<Ops, Id> = src[..].try_into()?;
        Ok(v)
    }
}

impl<'data, Ops, Id> TryFrom<&'data [u8]> for BorshMessage<'data, Ops, Id>
where
    Id: Debug + BorshSerialize + BorshDeserialize + 'data,
    Ops: Debug + BorshSerialize + BorshDeserialize + 'data,
{
    type Error = Error;

    fn try_from(src: &'data [u8]) -> std::result::Result<Self, Self::Error> {
        let mut payload = src;
        let header = BorshHeader::<Ops, Id>::deserialize(&mut payload)?;
        let message = BorshMessage { header, payload };
        Ok(message)
    }
}
