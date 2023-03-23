use derive_more::{Display, From, Into};
use pyo3::types::{PyString, PyInt};
use ethers::types::Bloom as BloomOriginal;
use ethers::utils::to_checksum;
use pyo3::exceptions::PyOverflowError;
use pyo3::{ToPyObject, PyObject, Python, ffi, IntoPy, PyAny, PyResult, PyErr, AsPyPointer};
use pyo3::{FromPyObject, pyclass};
use serde::{Serialize, Deserialize};
use solders_macros::{EnumIntoPy, enum_original_mapping};
use core::ops::{Deref, DerefMut};
use std::collections::BTreeMap;
use std::ffi::c_uchar;
use std::str::FromStr;
use ethers::types::Address as AddressOriginal;
use ethers::types::U64 as U64Original;
use ethers::types::{U256 as U256Original, Bytes as BytesOriginal, H256 as H256Original, TxHash as TxHashOriginal, H160 as H160Original, H64 as H64Original};
use ethers::types::BlockNumber as BlockNumberOriginal;
use ethers::types::FeeHistory as FeeHistoryOriginal;
use ethers::types::BlockId as BlockIdOriginal;
use ethers::types::TransactionRequest as TransactionRequestOriginal;
use ethers::types::OtherFields as OtherFieldsOriginal;
use ethers::types::transaction::eip2718::TypedTransaction as TypedTransactionOriginal;
use ethers::types::NameOrAddress as NameOrAddressOriginal;
use web3_rush_macros::{struct_original_mapping, tuple_struct_original_mapping, tuple_enum_original_mapping};


#[derive(FromPyObject, Debug)]
pub enum Primitives {
    #[pyo3(transparent, annotation = "str")]
    String(String),
    #[pyo3(transparent, annotation = "bytes")]
    Bytes(Vec<u8>),
    #[pyo3(transparent, annotation = "int")]
    Int(isize),
    #[pyo3(transparent, annotation = "bool")]
    Bool(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[tuple_struct_original_mapping(AddressOriginal)]
pub struct Address(pub AddressOriginal);

impl ToPyObject for Address {
    #[inline]
    fn to_object(&self, py: Python<'_>) -> PyObject {
        PyString::new(py, &to_checksum(&self.0, None)).into()
    }
}

impl IntoPy<PyObject> for Address {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

impl FromPyObject<'_> for Address {
    #[inline]
    fn extract(obj: &PyAny) -> PyResult<Self> {
        match AddressOriginal::from_str(obj.downcast::<PyString>()?.to_str()?) {
            Ok(v) => Ok(Address(v)),
            Err(err) => {
                
                Err(wrap_from_hex_error(err))
            },
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[tuple_enum_original_mapping(NameOrAddressOriginal)]
pub enum NameOrAddress {
    /// An ENS Name (format does not get checked)
    Name(String),
    /// An Ethereum Address
    Address(Address),
}

impl ToPyObject for NameOrAddress {
    #[inline]
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match self {
            NameOrAddress::Name(name) => PyString::new(py, name).into(),
            NameOrAddress::Address(addr) => PyString::new(py, &to_checksum(&addr.0, None)).into(),
        }
    }
}

impl IntoPy<PyObject> for NameOrAddress {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

impl FromPyObject<'_> for NameOrAddress {
    #[inline]
    fn extract(obj: &PyAny) -> PyResult<Self> {
        match AddressOriginal::from_str(obj.downcast::<PyString>()?.to_str()?) {
            Ok(v) => Ok(NameOrAddress::Address(Address(v))),
            Err(_) => {
                Ok(NameOrAddress::Name(obj.downcast::<PyString>()?.to_str()?.to_owned()))
            },
        }
    }
}


#[derive(FromPyObject, Clone)]
pub enum BlockNumberParser {
    #[pyo3(transparent, annotation = "str")]
    String(String),
    #[pyo3(transparent, annotation = "int")]
    BigUint(u64),
}

impl Into<BlockNumberOriginal> for BlockNumberParser {
    fn into(self) -> BlockNumberOriginal {
        match self {
            BlockNumberParser::String(s) => {
                match &*s {
                    "latest" => BlockNumberOriginal::Latest,
                    "earliest" => BlockNumberOriginal::Earliest,
                    "pending" => BlockNumberOriginal::Pending,
                    _ => {
                        BlockNumberOriginal::Latest
                    },
                }
            },
            BlockNumberParser::BigUint(n) => {
                BlockNumberOriginal::Number(n.into())
            },
        }
    }
}

#[derive(Clone)]
#[pyclass(module = "web3_rush", get_all)]
pub struct FeeHistory {
    pub base_fee_per_gas: Vec<U256>,
    pub gas_used_ratio: Vec<f64>,
    /// oldestBlock is returned as an unsigned integer up to geth v1.10.6. From
    /// geth v1.10.7, this has been updated to return in the hex encoded form.
    /// The custom deserializer allows backward compatibility for those clients
    /// not running v1.10.7 yet.
    pub oldest_block: U256,
    /// An (optional) array of effective priority fee per gas data points from a single block. All
    /// zeroes are returned if the block is empty.
    pub reward: Vec<Vec<U256>>,
}

impl From<FeeHistoryOriginal> for FeeHistory {
    fn from(value: FeeHistoryOriginal) -> Self {
        FeeHistory { 
            base_fee_per_gas: value.base_fee_per_gas.into_iter().map(|v|{v.into()}).collect(), 
            gas_used_ratio: value.gas_used_ratio, 
            oldest_block: U256(U256Original::from_str(&value.oldest_block.to_string()).unwrap()), 
            reward: value.reward.into_iter().map(|v|{v.into_iter().map(|v|{v.into()}).collect()}).collect()
        }
    }
}

impl Into<FeeHistoryOriginal> for FeeHistory {
    fn into(self) -> FeeHistoryOriginal {
        FeeHistoryOriginal{ 
            base_fee_per_gas: self.base_fee_per_gas.into_iter().map(|v|{U256Original::from_str(&v.0.to_string()).unwrap()}).collect(), 
            gas_used_ratio: self.gas_used_ratio, 
            oldest_block: U256Original::from_str(&self.oldest_block.0.to_string()).unwrap(), 
            reward: self.reward.into_iter().map(|v|{v.into_iter().map(|v|{U256Original::from_str(&v.0.to_string()).unwrap()}).collect()}).collect()
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[tuple_struct_original_mapping(OtherFieldsOriginal)]
#[pyclass(module = "web3_rush")]
pub struct OtherFields(pub OtherFieldsOriginal);

#[derive(FromPyObject, Display)]
#[tuple_struct_original_mapping(String)]
#[pyclass(module = "web3_rush")]
pub struct HexStr(String);

pub enum AnyStr {
    Str(String),
    Bytes(Vec<u8>)
}

#[derive(Clone)]
#[tuple_struct_original_mapping(H64Original)]
#[pyclass(module = "web3_rush")]
pub struct H64(pub H64Original);

#[derive(Clone)]
#[tuple_struct_original_mapping(H256Original)]
pub struct H256(pub H256Original);

impl ToPyObject for H256 {
    #[inline]
    fn to_object(&self, py: Python<'_>) -> PyObject {
        PyString::new(py, &self.0.to_string()).into()
    }
}

impl IntoPy<PyObject> for H256 {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

impl FromPyObject<'_> for H256 {
    #[inline]
    fn extract(obj: &PyAny) -> PyResult<Self> {
        Ok(H256(H256Original::from_str(&obj.downcast::<PyString>()?.to_string()).unwrap()))
    }
}


#[derive(Clone)]
#[tuple_struct_original_mapping(H160Original)]
#[pyclass(module = "web3_rush")]
pub struct H160(pub H160Original);

#[derive(Clone, Serialize, Deserialize)]
#[tuple_struct_original_mapping(U64Original)]
#[pyclass(module = "web3_rush")]
pub struct U64(pub U64Original);


#[derive(Clone, Deserialize)]
pub struct U256(pub U256Original);

impl From<ruint::aliases::U256> for U256 {
    fn from(value: ruint::aliases::U256) -> U256 {
        U256(U256Original::from_big_endian(&value.to_be_bytes_vec()))
    }
}

impl Into<ruint::aliases::U256> for U256 {
    fn into(self) -> ruint::aliases::U256 {
        ruint::aliases::U256::from_str(&self.0.to_string()).unwrap()
    }
}

impl From<num_bigint::BigUint> for U256 {
    fn from(value: num_bigint::BigUint) -> U256 {
        value.into()
    }
}

impl Into<num_bigint::BigUint> for U256 {
    fn into(self) -> num_bigint::BigUint {
        num_bigint::BigUint::from_str(&self.0.to_string()).unwrap()
    }
}

impl From<U256Original> for U256 {
    fn from(value: U256Original) -> U256 {
        U256(value)
    }
}

impl Into<U256Original> for U256 {
    fn into(self) -> U256Original {
        self.0
    }
}

impl ToPyObject for U256 {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let binding = Into::<ruint::aliases::U256>::into(self.clone());
        let bytes = binding.as_le_bytes();
        unsafe {
            let obj =
                ffi::_PyLong_FromByteArray(bytes.as_ptr().cast::<c_uchar>(), bytes.len(), 1, 0);
            PyObject::from_owned_ptr(py, obj)
        }
    }
}

impl IntoPy<PyObject> for U256 {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

impl<'source> FromPyObject<'source> for U256 {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        let mut result = ruint::aliases::U256::from(0);

        #[cfg(target_endian = "little")]
        let py_result = unsafe {
            let raw = result.as_le_slice_mut();
            ffi::_PyLong_AsByteArray(
                ob.as_ptr().cast::<ffi::PyLongObject>(),
                raw.as_mut_ptr(),
                raw.len(),
                1,
                0,
            )
        };

        #[cfg(not(target_endian = "little"))]
        let py_result = {
            let mut raw = vec![0_u8; Self::LIMBS * 8];
            let py_result = unsafe {
                ffi::_PyLong_AsByteArray(
                    ob.as_ptr().cast::<ffi::PyLongObject>(),
                    raw.as_mut_ptr(),
                    raw.len(),
                    1,
                    0,
                )
            };
            result = Self::try_from_le_slice(raw.as_slice()).ok_or_else(|| {
                PyOverflowError::new_err(format!("Number to large to fit Uint<{}>", Self::BITS))
            })?;
            py_result
        };

        if py_result != 0 {
            return Err(PyErr::fetch(ob.py()));
        }

        #[cfg(target_endian = "little")]
        if let Some(last) = Into::<ruint::aliases::U256>::into(result).as_limbs().last() {
            if *last > mask(ruint::aliases::U256::BITS) {
                return Err(PyOverflowError::new_err(format!(
                    "Number to large to fit Uint<{}>",
                    ruint::aliases::U256::BITS
                )));
            }
        }

        Ok(result.into())
    }
}


const fn mask(bits: usize) -> u64 {
    if bits == 0 {
        return 0;
    }
    let bits = bits % 64;
    if bits == 0 {
        u64::MAX
    } else {
        (1 << bits) - 1
    }
}


#[derive(Clone, Serialize, Deserialize)]
#[tuple_struct_original_mapping(BytesOriginal)]
#[pyclass(module = "web3_rush")]
pub struct Bytes(pub BytesOriginal);

#[derive(FromPyObject, Clone)]
pub enum BlockId {
    /// By Hash
    #[pyo3(transparent)]
    Hash(H256),
    /// By Number
    #[pyo3(transparent)]
    Number(BlockNumberParser),
}

impl Into<BlockIdOriginal> for BlockId {
    fn into(self) -> BlockIdOriginal {
        match self {
            BlockId::Hash(h) => {
                BlockIdOriginal::Hash(h.into())
            },
            BlockId::Number(n) => {
                BlockIdOriginal::Number(n.into())
            },
        }
    }
}

#[pyclass(module = "web3_rush")]
#[derive(Clone, Deserialize)]
#[struct_original_mapping(TransactionRequestOriginal)]
pub struct TransactionRequest {
    /// Sender address
    pub from: Option<Address>,
    /// Recipient address (None for contract creation)
    #[serde(flatten)]
    pub to: Option<NameOrAddress>,
    /// Supplied gas (None for sensible default)
    #[serde(flatten)]
    pub gas: Option<U256>,
    /// Gas price (None for sensible default)
    #[serde(flatten)]
    pub gas_price: Option<U256>,
    /// Transfered value (None for no transfer)
    #[serde(flatten)]
    pub value: Option<U256>,
    /// Transaction data (None for empty bytes)
    #[serde(flatten)]
    pub data: Option<Bytes>,
    /// Transaction nonce (None for next available nonce)
    #[serde(flatten)]
    pub nonce: Option<U256>,
    /// Chain ID (None for mainnet)
    #[serde(flatten)]
    pub chain_id: Option<U64>,
}

use ethers::types::transaction::eip1559::Eip1559TransactionRequest as Eip1559TransactionRequestOriginal;
use ethers::types::transaction::eip2930::Eip2930TransactionRequest as Eip2930TransactionRequestOriginal;
use ethers::types::transaction::eip2930::AccessList as AccessListOriginal;

#[pyclass(module = "web3_rush")]
#[derive(Clone, Deserialize)]
#[struct_original_mapping(Eip2930TransactionRequestOriginal)]
pub struct Eip2930TransactionRequest {
    pub tx: TransactionRequest,
    pub access_list: AccessList,
}

#[pyclass(module = "web3_rush")]
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[tuple_struct_original_mapping(AccessListOriginal)]
pub struct AccessList(pub AccessListOriginal);


#[pyclass(module = "web3_rush")]
#[derive(From, Into, Clone, Deserialize)]
#[struct_original_mapping(Eip1559TransactionRequestOriginal)]
pub struct Eip1559TransactionRequest {
    /// Sender address or ENS name
    pub from: Option<Address>,

    /// Recipient address (None for contract creation)
    pub to: Option<NameOrAddress>,

    /// Supplied gas (None for sensible default)
    #[serde(flatten)]
    pub gas: Option<U256>,

    /// Transferred value (None for no transfer)
    #[serde(flatten)]
    pub value: Option<U256>,

    /// The compiled code of a contract OR the first 4 bytes of the hash of the
    /// invoked method signature and encoded parameters. For details see Ethereum Contract ABI
    #[serde(flatten)]
    pub data: Option<Bytes>,

    /// Transaction nonce (None for next available nonce)
    #[serde(flatten)]
    pub nonce: Option<U256>,

    pub access_list: AccessList,

    /// Represents the maximum tx fee that will go to the miner as part of the user's
    /// fee payment. It serves 3 purposes:
    /// 1. Compensates miners for the uncle/ommer risk + fixed costs of including transaction in a
    /// block; 2. Allows users with high opportunity costs to pay a premium to miners;
    /// 3. In times where demand exceeds the available block space (i.e. 100% full, 30mm gas),
    /// this component allows first price auctions (i.e. the pre-1559 fee model) to happen on the
    /// priority fee.
    ///
    /// More context [here](https://hackmd.io/@q8X_WM2nTfu6nuvAzqXiTQ/1559-wallets)
    #[serde(flatten)]
    pub max_priority_fee_per_gas: Option<U256>,

    /// Represents the maximum amount that a user is willing to pay for their tx (inclusive of
    /// baseFeePerGas and maxPriorityFeePerGas). The difference between maxFeePerGas and
    /// baseFeePerGas + maxPriorityFeePerGas is “refunded” to the user.
    #[serde(flatten)]
    pub max_fee_per_gas: Option<U256>,

    /// Chain ID (None for mainnet)
    #[serde(flatten)]
    pub chain_id: Option<U64>,
}


#[derive(FromPyObject, Clone, EnumIntoPy, Deserialize)]
#[tuple_enum_original_mapping(TypedTransactionOriginal)]
pub enum TypedTransaction {
    #[pyo3(transparent)]
    Legacy(TransactionRequest),
    #[pyo3(transparent)]
    Eip2930(Eip2930TransactionRequest),
    #[pyo3(transparent)]
    Eip1559(Eip1559TransactionRequest),
}


use ethers::types::Log as LogOriginal;

#[derive(From, Into, Clone)]
#[struct_original_mapping(LogOriginal)]
#[pyclass(module = "web3_rush")]
pub struct Log {
    /// H160
    pub address: Address,
    /// Topics
    pub topics: Vec<H256>,
    /// Data
    pub data: Bytes,
    /// Block Hash
    pub block_hash: Option<H256>,
    /// Block Number
    pub block_number: Option<U64>,
    /// Transaction Hash
    pub transaction_hash: Option<H256>,
    /// Transaction Index
    pub transaction_index: Option<U64>,
    /// Log Index in Block
    pub log_index: Option<U256>,
    /// Log Index in Transaction
    pub transaction_log_index: Option<U256>,
    /// Log Type
    pub log_type: Option<String>,
    /// Removed
    pub removed: Option<bool>,
}


use ethers::types::TransactionReceipt as TransactionReceiptOriginal;

use crate::exceptions::wrap_from_hex_error;

#[derive(Clone)]
#[tuple_struct_original_mapping(BloomOriginal)]
#[pyclass(module = "web3_rush")]
pub struct Bloom(pub BloomOriginal);

#[derive(From, Into, Clone)]
#[struct_original_mapping(TransactionReceiptOriginal)]
#[pyclass(module = "web3_rush")]
pub struct TransactionReceipt {
    /// Transaction hash.
    pub transaction_hash: H256,
    /// Index within the block.
    pub transaction_index: U64,
    /// Hash of the block this transaction was included within.
    pub block_hash: Option<H256>,
    /// Number of the block this transaction was included within.
    pub block_number: Option<U64>,
    /// Sender
    /// Note: default address if the client did not return this value
    /// (maintains backwards compatibility for <= 0.7.0 when this field was missing)
    pub from: Address,
    /// Recipient (None when contract creation)
    /// Note: Also `None` if the client did not return this value
    /// (maintains backwards compatibility for <= 0.7.0 when this field was missing)
    pub to: Option<Address>,
    /// Cumulative gas used within the block after this was executed.
    pub cumulative_gas_used: U256,
    /// Gas used by this transaction alone.
    ///
    /// Gas used is `None` if the the client is running in light client mode.
    pub gas_used: Option<U256>,
    /// Contract address created, or `None` if not a deployment.
    pub contract_address: Option<Address>,
    /// Logs generated within this transaction.
    pub logs: Vec<Log>,
    /// Status: either 1 (success) or 0 (failure).
    pub status: Option<U64>,
    /// State root.
    pub root: Option<H256>,
    /// Logs bloom
    pub logs_bloom: Bloom,
    /// Transaction type, Some(1) for AccessList transaction, None for Legacy
    pub transaction_type: Option<U64>,
    /// Effective gas price
    pub effective_gas_price: Option<U256>,
    // Captures unknown fields such as additional fields used by L2s
    pub other: OtherFields,
}
