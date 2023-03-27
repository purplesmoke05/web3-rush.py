use bigdecimal::{BigDecimal, ToPrimitive};
use once_cell::sync::Lazy;
use std::{str::FromStr, string::FromUtf8Error};

use hex::ToHex;
use num_bigint::BigInt;
use pyo3::{exceptions::PyTypeError, PyErr};

use crate::{
    exceptions::BaseWeb3RushError,
    types::{AnyStr, HexStr, Number, Primitives},
};

pub fn add_0x_prefix(value: HexStr) -> HexStr {
    return HexStr::from(format!("0x{}", value));
}

pub fn encode_hex(value: AnyStr) -> Result<HexStr, FromUtf8Error> {
    let ascii_bytes = match value {
        AnyStr::Str(str) => str.to_ascii_lowercase(),
        AnyStr::Bytes(bytes) => String::from_utf8(bytes)?,
    };

    Ok(add_0x_prefix(HexStr::from(
        ascii_bytes.as_bytes().encode_hex::<String>(),
    )))
}

pub fn decode_hex(value: String) -> Result<Vec<u8>, PyErr> {
    let non_prefixed = value.replace("0x", "");
    let ascii_hex = non_prefixed.to_ascii_lowercase();
    (0..ascii_hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&ascii_hex[i..i + 2], 16).map_err(|e| e.into()))
        .collect()
}

pub fn to_hex_i32(value: i32) -> String {
    let value = if value > 255 {
        255
    } else if value < 0 {
        0
    } else {
        value
    };
    format!("0x{:x}", value)
}

#[allow(unused)]
pub fn to_bytes(
    primitive: Option<Primitives>,
    hexstr: Option<HexStr>,
    text: Option<String>,
) -> Result<Vec<u8>, PyErr> {
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
            return Ok(decode_hex("0x0".to_owned() + &v.replace("0x", ""))?);
        };
        Ok(decode_hex(v)?)
    } else if let Some(text) = text {
        Ok(text.to_ascii_lowercase().into())
    } else {
        Err(PyTypeError::new_err(
            "expected a bool, int, byte or bytearray in first arg, or keyword of hexstr or text",
        ))
    }
}

pub fn to_int(
    primitive: Option<Primitives>,
    hexstr: Option<HexStr>,
    text: Option<String>,
) -> Result<isize, PyErr> {
    if let Some(hexstr) = hexstr {
        Ok(isize::from_str_radix(
            &hexstr.to_string().replace("0x", ""),
            16,
        )?)
    } else if let Some(text) = text {
        Ok(text.parse().unwrap_or(0))
    } else if let Some(primitive) = primitive {
        match primitive {
            Primitives::String(_) => Err(PyTypeError::new_err(
                "Pass in strings with keyword hexstr or text",
            )),
            Primitives::Bytes(mut b) => {
                b.reverse();
                Ok(b.into_iter()
                    .enumerate()
                    .map(|(i, b)| ((b as isize) << (i * 8)))
                    .sum())
            }
            Primitives::Int(i) => Ok(i),
            Primitives::Bool(b) => Ok(b.into()),
        }
    } else {
        Err(BaseWeb3RushError::new_err(format!(
            "Invalid type.  Expected one of int/bool/str/bytes/bytearray.  Got {:?}",
            primitive
        )))
    }
}

static MIN_WEI: Lazy<BigInt> = Lazy::new(|| BigInt::from(0));
static MAX_WEI: Lazy<BigInt> = Lazy::new(|| {
    BigInt::from_str(
        "115792089237316195423570985008687907853269984665640564039457584007913129639935",
    )
    .unwrap()
});

static WEI: Lazy<BigInt> = Lazy::new(|| BigInt::from(1));
static K_WEI: Lazy<BigInt> = Lazy::new(|| BigInt::from(1000));
static M_WEI: Lazy<BigInt> = Lazy::new(|| BigInt::from(1000000));
static G_WEI: Lazy<BigInt> = Lazy::new(|| BigInt::from(1000000000));
static SZABO: Lazy<BigInt> = Lazy::new(|| BigInt::from_str("1000000000000").unwrap());
static FINNEY: Lazy<BigInt> = Lazy::new(|| BigInt::from_str("1000000000000000").unwrap());
static ETHER: Lazy<BigInt> = Lazy::new(|| BigInt::from_str("1000000000000000000").unwrap());
static K_ETHER: Lazy<BigInt> = Lazy::new(|| BigInt::from_str("1000000000000000000000").unwrap());
static M_ETHER: Lazy<BigInt> = Lazy::new(|| BigInt::from_str("1000000000000000000000000").unwrap());
static G_ETHER: Lazy<BigInt> =
    Lazy::new(|| BigInt::from_str("1000000000000000000000000000").unwrap());
static T_ETHER: Lazy<BigInt> =
    Lazy::new(|| BigInt::from_str("1000000000000000000000000000000").unwrap());

pub fn units(unit: &str) -> Result<BigInt, PyErr> {
    match unit {
        "wei" => Ok(WEI.clone()),
        "kwei" => Ok(K_WEI.clone()),
        "baggage" => Ok(K_WEI.clone()),
        "femtoether" => Ok(K_WEI.clone()),
        "mwei" => Ok(M_WEI.clone()),
        "lovelace" => Ok(M_WEI.clone()),
        "picoether" => Ok(M_WEI.clone()),
        "gwei" => Ok(G_WEI.clone()),
        "shannon" => Ok(G_WEI.clone()),
        "nanoether" => Ok(G_WEI.clone()),
        "nano" => Ok(G_WEI.clone()),
        "szabo" => Ok(SZABO.clone()),
        "microether" => Ok(SZABO.clone()),
        "finney" => Ok(FINNEY.clone()),
        "milliether" => Ok(FINNEY.clone()),
        "milli" => Ok(FINNEY.clone()),
        "ether" => Ok(ETHER.clone()),
        "kether" => Ok(K_ETHER.clone()),
        "grand" => Ok(K_ETHER.clone()),
        "mether" => Ok(M_ETHER.clone()),
        "gether" => Ok(G_ETHER.clone()),
        "tether" => Ok(T_ETHER.clone()),
        _ => Err(PyTypeError::new_err(format!(
            "Unknown unit.  Must be one of unit literal"
        ))),
    }
}

fn int_to_wei(i: BigInt, unit_value: BigInt) -> Result<BigInt, PyErr> {
    if i == BigInt::from(0) {
        return Ok(i);
    }
    let result_value = i * unit_value;
    if result_value < *MIN_WEI || result_value > *MAX_WEI {
        return Err(PyTypeError::new_err(
            "Resulting wei value must be between 1 and 2**256 - 1",
        ));
    }
    return Ok(result_value);
}

fn float_to_wei(f: f64, unit_value: BigInt) -> Result<BigInt, PyErr> {
    if f == 0.0 {
        return Ok(BigInt::from(0));
    }
    let f = match BigDecimal::from_str(&f.to_string()) {
        Ok(f) => f,
        Err(_) => {
            return Err(PyTypeError::new_err(
                "Unsupported type.  Must be one of integer, float, or string",
            ))
        }
    };
    let result_value = f * BigDecimal::from(unit_value);
    if result_value < BigDecimal::from(MIN_WEI.clone())
        || result_value > BigDecimal::from(MAX_WEI.clone())
    {
        return Err(PyTypeError::new_err(
            "Resulting wei value must be between 1 and 2**256 - 1",
        ));
    }
    let result_value = match result_value.to_string().find(".") {
        Some(index) => BigInt::from_str(&result_value.to_string()[0..index].to_string()).unwrap(),
        None => BigInt::from_str(&result_value.to_string()).unwrap(),
    };
    return Ok(result_value);
}

pub fn to_wei(number: Number, unit: String) -> Result<BigInt, PyErr> {
    let unit_value = match units(&unit.to_lowercase()) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    match number {
        Number::String(s) => match s.contains(".") {
            true => match f64::from_str(&s) {
                Ok(f) => return float_to_wei(f, unit_value),
                Err(_) => {
                    return Err(PyTypeError::new_err(
                        "Unsupported type.  Must be one of integer, float, or string",
                    ))
                }
            },
            false => match BigInt::from_str(&s) {
                Ok(i) => return int_to_wei(i, unit_value),
                Err(_) => {
                    return Err(PyTypeError::new_err(
                        "Unsupported type.  Must be one of integer, float, or string",
                    ))
                }
            },
        },
        Number::Int(i) => return int_to_wei(i, unit_value),
        Number::Float(f) => return float_to_wei(f, unit_value),
    };
}

fn int_from_wei(i: BigInt, unit_value: BigInt) -> Result<f64, PyErr> {
    if i == BigInt::from(0) {
        return Ok(0.0);
    }
    if i < *MIN_WEI && i > *MIN_WEI {
        return Err(PyTypeError::new_err(
            "value must be between 1 and 2**256 - 1",
        ));
    }
    let result_value = i / unit_value;
    return Ok(result_value.to_f64().unwrap());
}

fn float_from_wei(f: f64, unit_value: BigInt) -> Result<f64, PyErr> {
    let f = match BigDecimal::from_str(&f.to_string()) {
        Ok(f) => f,
        Err(_) => {
            return Err(PyTypeError::new_err(
                "Unsupported type.  Must be one of integer, float, or string",
            ))
        }
    };
    let result_value = f / BigDecimal::from(unit_value);
    return Ok(result_value.to_f64().unwrap());
}

pub fn from_wei(number: Number, unit: String) -> Result<f64, PyErr> {
    let unit_value = match units(&unit.to_lowercase()) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    match number {
        Number::Int(i) => return int_from_wei(i, unit_value),
        Number::Float(f) => return float_from_wei(f, unit_value),
        Number::String(s) => match s.contains(".") {
            true => match f64::from_str(&s) {
                Ok(f) => return float_from_wei(f, unit_value),
                Err(_) => {
                    return Err(PyTypeError::new_err(
                        "Unsupported type.  Must be one of integer, float, or string",
                    ))
                }
            },
            false => match BigInt::from_str(&s) {
                Ok(i) => return int_from_wei(i, unit_value),
                Err(_) => {
                    return Err(PyTypeError::new_err(
                        "Unsupported type.  Must be one of integer, float, or string",
                    ))
                }
            },
        },
    }
}
