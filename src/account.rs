use async_std::task::block_on;

use ethers::signers::LocalWallet;

use ethers::types::transaction::eip2718::TypedTransaction as TypedTransactionOriginal;

use ethers::types::{Bytes as BytesOriginal, H160 as H160Original};
use ethers::utils::to_checksum;
use hex::FromHex;
use hex::ToHex;

use pyo3::exceptions::PyValueError;

use pyo3::pymethods;

use pyo3::{pyclass, FromPyObject, PyAny, PyObject, PyResult, Python};
use pythonize::depythonize;

use serde::{Deserialize, Deserializer, Serialize};

use std::fmt::Debug;

use web3_rush_macros::{struct_original_mapping, tuple_struct_original_mapping};

use eth_keystore::CipherparamsJson as CipherparamsJsonOriginal;
use eth_keystore::CryptoJson as CryptoJsonOriginal;
/// Wallet
use eth_keystore::EthKeystore as EthKeystoreOriginal;
use eth_keystore::KdfType as KdfTypeOriginal;
use eth_keystore::KdfparamsType as KdfparamsTypeOriginal;
use serde::ser::Serializer;
use uuid::Uuid as UuidOriginal;

#[derive(FromPyObject, Clone, Debug, Deserialize, Serialize)]
/// This struct represents the deserialized form of an encrypted JSON keystore based on the
/// [Web3 Secret Storage Definition](https://github.com/ethereum/wiki/wiki/Web3-Secret-Storage-Definition).
pub struct EthKeystore {
    pub crypto: CryptoJson,
    pub id: Uuid,
    pub version: u8,
}

impl From<EthKeystoreOriginal> for EthKeystore {
    fn from(value: EthKeystoreOriginal) -> Self {
        EthKeystore {
            crypto: value.crypto.into(),
            id: value.id.into(),
            version: value.version,
        }
    }
}

impl Into<EthKeystoreOriginal> for EthKeystore {
    fn into(self) -> EthKeystoreOriginal {
        EthKeystoreOriginal {
            crypto: self.crypto.into(),
            id: self.id.0,
            version: self.version,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[tuple_struct_original_mapping(UuidOriginal)]
pub struct Uuid(pub UuidOriginal);

impl FromPyObject<'_> for Uuid {
    fn extract(ob: &'_ PyAny) -> PyResult<Self> {
        let uuid = match UuidOriginal::parse_str(ob.extract::<String>()?.as_str()) {
            Ok(res) => res,
            Err(err) => return Err(PyValueError::new_err(err.to_string())),
        };
        Ok(Uuid(uuid))
    }
}

#[derive(FromPyObject, Clone, Debug, Deserialize, Serialize)]
/// Represents the "crypto" part of an encrypted JSON keystore.
#[struct_original_mapping(CryptoJsonOriginal)]
pub struct CryptoJson {
    pub cipher: String,
    pub cipherparams: CipherparamsJson,
    #[serde(serialize_with = "buffer_to_hex", deserialize_with = "hex_to_buffer")]
    pub ciphertext: Vec<u8>,
    pub kdf: KdfType,
    pub kdfparams: KdfparamsType,
    #[serde(serialize_with = "buffer_to_hex", deserialize_with = "hex_to_buffer")]
    pub mac: Vec<u8>,
}

#[derive(FromPyObject, Clone, Debug, Deserialize, Serialize)]
/// Represents the "cipherparams" part of an encrypted JSON keystore.
#[struct_original_mapping(CipherparamsJsonOriginal)]
pub struct CipherparamsJson {
    #[serde(serialize_with = "buffer_to_hex", deserialize_with = "hex_to_buffer")]
    pub iv: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
#[pyclass(module = "web3_rush")]
/// Types of key derivition functions supported by the Web3 Secret Storage.
pub enum KdfType {
    Pbkdf2,
    Scrypt,
}

impl From<KdfTypeOriginal> for KdfType {
    fn from(value: KdfTypeOriginal) -> Self {
        match value {
            KdfTypeOriginal::Pbkdf2 => KdfType::Pbkdf2,
            KdfTypeOriginal::Scrypt => KdfType::Scrypt,
        }
    }
}

impl Into<KdfTypeOriginal> for KdfType {
    fn into(self) -> KdfTypeOriginal {
        match self {
            KdfType::Pbkdf2 => KdfTypeOriginal::Pbkdf2,
            KdfType::Scrypt => KdfTypeOriginal::Scrypt,
        }
    }
}

#[derive(FromPyObject, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
/// Defines the various parameters used in the supported KDFs.
pub enum KdfparamsType {
    Pbkdf2 {
        c: u32,
        dklen: u8,
        prf: String,
        #[serde(serialize_with = "buffer_to_hex", deserialize_with = "hex_to_buffer")]
        salt: Vec<u8>,
    },
    Scrypt {
        dklen: u8,
        n: u32,
        p: u32,
        r: u32,
        #[serde(serialize_with = "buffer_to_hex", deserialize_with = "hex_to_buffer")]
        salt: Vec<u8>,
    },
}

impl From<KdfparamsTypeOriginal> for KdfparamsType {
    fn from(value: KdfparamsTypeOriginal) -> Self {
        match value {
            KdfparamsTypeOriginal::Pbkdf2 {
                c,
                dklen,
                prf,
                salt,
            } => KdfparamsType::Pbkdf2 {
                c,
                dklen,
                prf,
                salt: salt.to_vec(),
            },
            KdfparamsTypeOriginal::Scrypt {
                dklen,
                n,
                p,
                r,
                salt,
            } => KdfparamsType::Scrypt {
                dklen,
                n,
                p,
                r,
                salt: salt.to_vec(),
            },
        }
    }
}

impl Into<KdfparamsTypeOriginal> for KdfparamsType {
    fn into(self) -> KdfparamsTypeOriginal {
        match self {
            KdfparamsType::Pbkdf2 {
                c,
                dklen,
                prf,
                salt,
            } => KdfparamsTypeOriginal::Pbkdf2 {
                c,
                dklen,
                prf,
                salt: salt.into(),
            },
            KdfparamsType::Scrypt {
                dklen,
                n,
                p,
                r,
                salt,
            } => KdfparamsTypeOriginal::Scrypt {
                dklen,
                n,
                p,
                r,
                salt: salt.into(),
            },
        }
    }
}

fn buffer_to_hex<T, S>(buffer: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    serializer.serialize_str(&buffer.encode_hex::<String>())
}

fn hex_to_buffer<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer)
        .and_then(|string| Vec::from_hex(&string).map_err(|err| Error::custom(err.to_string())))
}

use ethers::signers::Signer;

use crate::exceptions::wrap_from_wallet_error;
use crate::types::bigint::U256;
use crate::types::str::AnyStr;
use crate::types::str::Bytes;
use crate::types::transaction::Eip1559TransactionRequest;
use crate::types::transaction::Eip2930TransactionRequest;
use crate::types::transaction::TransactionRequest;
use crate::types::transaction::TypedTransaction;
use crate::utils::encode_hex;

#[pyclass(module = "web3_rush")]
pub struct LocalAccount(pub LocalWallet);

#[pymethods]
impl LocalAccount {
    #[getter]
    pub fn address(&self) -> String {
        to_checksum(&H160Original::from_slice(&self.0.address().0), None)
    }

    #[getter]
    pub fn chain_id(&self) -> u64 {
        self.0.chain_id()
    }

    pub fn sign_transaction(&self, tx: PyObject) -> PyResult<SignedTransaction> {
        let tx = Python::with_gil(
            |py| match depythonize::<TransactionRequest>(tx.as_ref(py)) {
                Ok(res) => Ok(TypedTransaction::Legacy(res)),
                Err(_err) => match depythonize::<Eip1559TransactionRequest>(tx.as_ref(py)) {
                    Ok(res) => Ok(TypedTransaction::Eip1559(res)),
                    Err(_) => match depythonize::<Eip2930TransactionRequest>(tx.as_ref(py)) {
                        Ok(res) => Ok(TypedTransaction::Eip2930(res)),
                        Err(err) => {
                            return Err(err);
                        }
                    },
                },
            },
        )?;
        let org_tx: TypedTransactionOriginal = tx.into();

        match self.0.sign_transaction_sync(&org_tx) {
            Ok(signature) => Ok(SignedTransaction {
                raw_transaction: org_tx.rlp_signed(&signature).into(),
                hash: BytesOriginal::from_iter(org_tx.hash(&signature).0).into(),
                r: signature.r.into(),
                s: signature.s.into(),
                v: signature.v,
            }),
            Err(err) => Err(wrap_from_wallet_error(err)),
        }
    }

    pub fn sign_message(&self, message: &str) -> PyResult<String> {
        let message = message.as_bytes();

        match block_on(self.0.sign_message(message)) {
            Ok(signature) => Ok(encode_hex(AnyStr::Str(signature.to_string()))?.into()),
            Err(err) => Err(wrap_from_wallet_error(err)),
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[pyclass(module = "web3_rush", get_all)]
pub struct SignedTransaction {
    pub raw_transaction: Bytes,
    pub hash: Bytes,
    /// R value
    pub r: U256,
    /// S Value
    pub s: U256,
    /// V value
    pub v: u64,
}
