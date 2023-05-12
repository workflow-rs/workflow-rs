use crate::error::Error;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

pub trait JsValueTrait {
    fn try_as_u8(&self) -> Result<u8, Error>;
    fn try_as_u16(&self) -> Result<u16, Error>;
    fn try_as_u32(&self) -> Result<u32, Error>;
    fn try_as_u64(&self) -> Result<u64, Error>;
    fn try_as_vec_u8(&self) -> Result<Vec<u8>, Error>;
}

impl JsValueTrait for JsValue {
    fn try_as_u8(&self) -> Result<u8, Error> {
        let f = self
            .as_f64()
            .ok_or_else(|| Error::WrongType(format!("value is not a number: `{self:?}`")))?;
        if f < 0.0 || f > u8::MAX as f64 {
            Err(Error::Bounds(format!(
                "value `{f}` is out of bounds (0..{})",
                u8::MAX
            )))
        } else {
            Ok(f as u8)
        }
    }

    fn try_as_u16(&self) -> Result<u16, Error> {
        let f = self
            .as_f64()
            .ok_or_else(|| Error::WrongType(format!("value is not a number: `{self:?}`")))?;
        if f < 0.0 || f > u16::MAX as f64 {
            Err(Error::Bounds(format!(
                "value `{f}` is ount of bounds (0..{})",
                u16::MAX
            )))
        } else {
            Ok(f as u16)
        }
    }

    fn try_as_u32(&self) -> Result<u32, Error> {
        let f = self
            .as_f64()
            .ok_or_else(|| Error::WrongType(format!("value is not a number: `{self:?}`")))?;
        if f < 0.0 || f > u32::MAX as f64 {
            Err(Error::Bounds(format!(
                "value `{f}` is ount of bounds (0..{})",
                u32::MAX
            )))
        } else {
            Ok(f as u32)
        }
    }

    fn try_as_u64(&self) -> Result<u64, Error> {
        if self.is_string() {
            let hex_str = self.as_string().unwrap();
            if hex_str.len() > 16 {
                Err(Error::WrongSize("try_as_u64(): supplied string must be < 16 chars".to_string()))
            } else {
                let mut out = [0u8; 8];
                let mut input = [b'0'; 16];
                let start = input.len() - hex_str.len();
                input[start..].copy_from_slice(hex_str.as_bytes());
                faster_hex::hex_decode(&input, &mut out)?;
                Ok(u64::from_be_bytes(out))
            }
        } else if self.is_bigint() {
            Ok(self.clone().try_into().map_err(|err| {
                Error::Convert(format!(
                    "try_as_u64(): unable to convert BigInt value to u64: `{self:?}`: {err:?}"
                ))
            })?)
        } else {
            Ok(self.as_f64()
                .ok_or_else(|| Error::WrongType(format!("value is not a number ({self:?})")))?
                as u64)
        }
    
    }

    fn try_as_vec_u8(&self) -> Result<Vec<u8>, Error> {
        if self.is_string() {
            let hex_string = self.as_string().unwrap();
            let len = hex_string.len();
            if len == 0 {
                Ok(vec![])
            } else if len & 0x1 == 1 {
                Err(Error::HexStringNotEven(hex_string))
            } else {
                let mut vec = vec![0u8; hex_string.len() / 2];
                faster_hex::hex_decode(hex_string.as_bytes(), &mut vec)?;
                Ok(vec)
            }
        } else if self.is_object() {
            let array = Uint8Array::new(self);
            let vec: Vec<u8> = array.to_vec();
            Ok(vec)
        } else {
            Err(Error::WrongType(
                "value is not a hex string or an array".to_string(),
            ))
        }
    }
}
