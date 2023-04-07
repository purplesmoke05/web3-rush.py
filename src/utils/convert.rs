use aes::{
    cipher::{self, InnerIvInit, KeyInit, StreamCipherCore},
    Aes128,
};
use bigdecimal::{BigDecimal, ToPrimitive};
use eth_keystore::{CipherparamsJson, CryptoJson, KdfType, KdfparamsType, KeystoreError};
use ethers_core::rand::{CryptoRng, Rng};
use hmac::digest::Update;
use hmac::Hmac;
use once_cell::sync::Lazy;
use sha2::Digest;
use sha2::Sha256;
use sha3::Keccak256;
use std::{str::FromStr, string::FromUtf8Error};
use uuid::Uuid;

use hex::ToHex;
use num_bigint::BigInt;
use pyo3::{exceptions::PyTypeError, PyErr};

use crate::{
    exceptions::BaseWeb3RushError,
    types::primitives::{Number, Primitives},
    types::str::{AnyStr, HexStr},
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

use eth_keystore::EthKeystore as EthKeystoreOriginal;

pub fn decrypt_key<S>(keystore: EthKeystoreOriginal, password: S) -> Result<Vec<u8>, KeystoreError>
where
    S: AsRef<[u8]>,
{
    // Derive the key.
    let key = match keystore.crypto.kdfparams {
        KdfparamsType::Pbkdf2 {
            c,
            dklen,
            prf: _,
            salt,
        } => {
            let mut key = vec![0u8; dklen as usize];
            let _ = pbkdf2::pbkdf2::<Hmac<Sha256>>(password.as_ref(), &salt, c, key.as_mut_slice());
            key
        }
        KdfparamsType::Scrypt {
            dklen,
            n,
            p,
            r,
            salt,
        } => {
            let mut key = vec![0u8; dklen as usize];
            let log_n = (n as f32).log2() as u8;
            let scrypt_params = scrypt::Params::new(log_n, r, p, 32).unwrap();
            let _ = scrypt::scrypt(password.as_ref(), &salt, &scrypt_params, key.as_mut_slice());
            key
        }
    };

    // Derive the MAC from the derived key and ciphertext.
    let derived_mac = Keccak256::new()
        .chain(&key[16..32])
        .chain(&keystore.crypto.ciphertext)
        .finalize();

    if derived_mac.as_slice() != keystore.crypto.mac.as_slice() {
        return Err(KeystoreError::MacMismatch);
    }

    // Decrypt the private key bytes using AES-128-CTR
    let decryptor =
        Aes128Ctr::new(&key[..16], &keystore.crypto.cipherparams.iv[..16]).expect("invalid length");

    let mut pk = keystore.crypto.ciphertext;
    decryptor.apply_keystream(&mut pk);

    Ok(pk)
}

struct Aes128Ctr {
    inner: ctr::CtrCore<Aes128, ctr::flavors::Ctr128BE>,
}

impl Aes128Ctr {
    fn new(key: &[u8], iv: &[u8]) -> Result<Self, cipher::InvalidLength> {
        let cipher = aes::Aes128::new_from_slice(key).unwrap();
        let inner = ctr::CtrCore::inner_iv_slice_init(cipher, iv).unwrap();
        Ok(Self { inner })
    }

    fn apply_keystream(self, buf: &mut [u8]) {
        self.inner.apply_keystream_partial(buf.into());
    }
}
const DEFAULT_CIPHER: &str = "aes-128-ctr";
const DEFAULT_KEY_SIZE: usize = 32usize;
const DEFAULT_IV_SIZE: usize = 16usize;
const DEFAULT_KDF_PARAMS_DKLEN: u8 = 32u8;
const DEFAULT_KDF_PARAMS_LOG_N: u8 = 13u8;
const DEFAULT_KDF_PARAMS_R: u32 = 8u32;
const DEFAULT_KDF_PARAMS_P: u32 = 1u32;
pub fn encrypt_key<R, B, S>(
    rng: &mut R,
    pk: B,
    password: S,
) -> Result<EthKeystoreOriginal, KeystoreError>
where
    R: Rng + CryptoRng,
    B: AsRef<[u8]>,
    S: AsRef<[u8]>,
{
    // Generate a random salt.
    let mut salt = vec![0u8; DEFAULT_KEY_SIZE];
    rng.fill_bytes(salt.as_mut_slice());

    // Derive the key.
    let mut key = vec![0u8; DEFAULT_KDF_PARAMS_DKLEN as usize];
    let scrypt_params = scrypt::Params::new(
        DEFAULT_KDF_PARAMS_LOG_N,
        DEFAULT_KDF_PARAMS_R,
        DEFAULT_KDF_PARAMS_P,
        32,
    )
    .unwrap();
    scrypt::scrypt(password.as_ref(), &salt, &scrypt_params, key.as_mut_slice())
        .expect("scrypt failed");

    // Encrypt the private key using AES-128-CTR.
    let mut iv = vec![0u8; DEFAULT_IV_SIZE];
    rng.fill_bytes(iv.as_mut_slice());

    let encryptor = Aes128Ctr::new(&key[..16], &iv[..16]).expect("invalid length");

    let mut ciphertext = pk.as_ref().to_vec();
    encryptor.apply_keystream(&mut ciphertext);

    // Calculate the MAC.
    let mac = Keccak256::new()
        .chain(&key[16..32])
        .chain(&ciphertext)
        .finalize();

    // If a file name is not specified for the keystore, simply use the strigified uuid.
    let id = Uuid::new_v4();
    let _name = id.to_string();

    // Construct and serialize the encrypted JSON keystore.
    let keystore = EthKeystoreOriginal {
        id,
        version: 3,
        crypto: CryptoJson {
            cipher: String::from(DEFAULT_CIPHER),
            cipherparams: CipherparamsJson { iv },
            ciphertext: ciphertext.to_vec(),
            kdf: KdfType::Scrypt,
            kdfparams: KdfparamsType::Scrypt {
                dklen: DEFAULT_KDF_PARAMS_DKLEN,
                n: 2u32.pow(DEFAULT_KDF_PARAMS_LOG_N as u32),
                p: DEFAULT_KDF_PARAMS_P,
                r: DEFAULT_KDF_PARAMS_R,
                salt,
            },
            mac: mac.to_vec(),
        },
    };

    Ok(keystore.into())
}
