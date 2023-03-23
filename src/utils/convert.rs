use std::{string::FromUtf8Error};

use hex::ToHex;
use pyo3::{PyErr, exceptions::PyTypeError};

use crate::{types::{HexStr, AnyStr, Primitives}, exceptions::BaseWeb3RushError};

pub fn add_0x_prefix(value: HexStr) -> HexStr{
    return HexStr::from(format!("0x{}", value))
}

pub fn encode_hex(value: AnyStr) -> Result<HexStr, FromUtf8Error> {
    let ascii_bytes = match value {
        AnyStr::Str(str) => {
            str.to_ascii_lowercase()
        },
        AnyStr::Bytes(bytes) => {
            String::from_utf8(bytes)?
        },
    };

    Ok(add_0x_prefix(HexStr::from(ascii_bytes.as_bytes().encode_hex::<String>())))
}

pub fn decode_hex(value: String) -> Result<Vec<u8>, PyErr> {
    let non_prefixed = value.replace("0x", "");
    let ascii_hex = non_prefixed.to_ascii_lowercase();
    (0..ascii_hex.len()).step_by(2).map(|i|u8::from_str_radix(&ascii_hex[i..i + 2], 16).map_err(|e| e.into())).collect()
}

pub fn to_hex_i32(value: i32) -> String {
    let value = if value > 255 { 255 } else if value < 0 { 0 } else { value };
    format!("0x{:x}", value)
}

pub fn to_bytes(primitive: Option<Primitives>, hexstr: Option<HexStr>, text: Option<String>) -> Result<Vec<u8>, PyErr> {
    if let Some(primitive) = primitive {
        match primitive {
            Primitives::Bool(b) => match b {
                true => {
                    Ok("\x01".as_bytes().to_vec())
                },
                false => Ok("\x00".as_bytes().to_vec()),
            },
            Primitives::String(_) => Err(PyTypeError::new_err("expected a bool, int, byte or bytearray in first arg, or keyword of hexstr or text")),
            Primitives::Bytes(b) => Ok(b),
            Primitives::Int(i) => Ok(i.to_be_bytes().to_vec()),
        }
    } else if let Some(hexstr) = hexstr {
        let v: String = hexstr.into();
        if v.len() % 2 > 0 {
            return Ok(decode_hex("0x0".to_owned()+&v.replace("0x", ""))?)
        };
        Ok(decode_hex(v)?)
    } else if let Some(text) = text {
        Ok(text.to_ascii_lowercase().into())
    } else {
        Err(PyTypeError::new_err("expected a bool, int, byte or bytearray in first arg, or keyword of hexstr or text"))
    }
}

pub fn to_int(primitive: Option<Primitives>, hexstr: Option<HexStr>, text: Option<String>) -> Result<isize, PyErr> {
    if let Some(hexstr) = hexstr {
        Ok(isize::from_str_radix(&hexstr.to_string().replace("0x", ""), 16)?)
    } else if let Some(text) = text {
        Ok(text.parse().unwrap_or(0))
    } else if let Some(primitive) = primitive {
        match primitive {
            Primitives::String(_) => {
                Err(PyTypeError::new_err("Pass in strings with keyword hexstr or text"))
            },
            Primitives::Bytes(mut b) => {
                b.reverse();
                Ok(b.into_iter().enumerate().map(|(i, b)| ((b as isize) << (i *8))).sum())
            },
            Primitives::Int(i) => Ok(i),
            Primitives::Bool(b) => Ok(b.into()),
        }
    } else {
        Err(BaseWeb3RushError::new_err(format!("Invalid type.  Expected one of int/bool/str/bytes/bytearray.  Got {:?}", primitive)))
    }
}