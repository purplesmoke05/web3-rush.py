use ethers::types::transaction::eip1559::Eip1559TransactionRequest as Eip1559TransactionRequestOriginal;
use ethers::types::transaction::eip2930::AccessList as AccessListOriginal;
use ethers::types::transaction::eip2930::Eip2930TransactionRequest as Eip2930TransactionRequestOriginal;

use core::ops::{Deref, DerefMut};
use core::panic;
use derive_more::{Display, From, Into};
use ethers::types::transaction::eip2718::TypedTransaction as TypedTransactionOriginal;
use ethers::types::Address as AddressOriginal;
use ethers::types::BlockId as BlockIdOriginal;
use ethers::types::BlockNumber as BlockNumberOriginal;
use ethers::types::Bloom as BloomOriginal;
use ethers::types::FeeHistory as FeeHistoryOriginal;
use ethers::types::NameOrAddress as NameOrAddressOriginal;
use ethers::types::OtherFields as OtherFieldsOriginal;
use ethers::types::Transaction as TransactionOriginal;
use ethers::types::TransactionRequest as TransactionRequestOriginal;
use ethers::types::U64 as U64Original;
use ethers::types::{
    Bytes as BytesOriginal, TxHash as TxHashOriginal, H160 as H160Original, H256 as H256Original,
    H64 as H64Original, U256 as U256Original,
};
use ethers::utils::to_checksum;
use hex::ToHex;
use pyo3::exceptions::PyOverflowError;
use pyo3::types::PyBool;
use pyo3::types::{PyBytes, PyInt, PyString};
use pyo3::PyCell;
use pyo3::{ffi, AsPyPointer, IntoPy, PyAny, PyErr, PyObject, PyResult, Python, ToPyObject};
use pyo3::{pyclass, FromPyObject};
use serde::{Deserialize, Serialize};
use solders_macros::{enum_original_mapping, EnumIntoPy};
use std::collections::BTreeMap;
use std::ffi::c_uchar;
use std::str::FromStr;
use web3_rush_macros::{
    struct_original_mapping, tuple_enum_original_mapping, tuple_struct_original_mapping,
};

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
#[tuple_struct_original_mapping(H160Original)]
pub struct Address(pub AddressOriginal);

impl Address {
    fn zero() -> Address {
        Address(AddressOriginal::zero())
    }
}

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
            Err(err) => Err(wrap_from_hex_error(err)),
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
            Err(_) => Ok(NameOrAddress::Name(
                obj.downcast::<PyString>()?.to_str()?.to_owned(),
            )),
        }
    }
}

#[derive(FromPyObject, Clone, Display, Deserialize)]
#[serde(untagged)]
pub enum BlockNumberParser {
    #[pyo3(transparent, annotation = "str")]
    String(String),
    #[pyo3(transparent, annotation = "int")]
    BigUint(u64),
}

impl Into<BlockNumberOriginal> for BlockNumberParser {
    fn into(self) -> BlockNumberOriginal {
        match self {
            BlockNumberParser::String(s) => match &*s {
                "latest" => BlockNumberOriginal::Latest,
                "earliest" => BlockNumberOriginal::Earliest,
                "pending" => BlockNumberOriginal::Pending,
                _ => BlockNumberOriginal::Latest,
            },
            BlockNumberParser::BigUint(n) => BlockNumberOriginal::Number(n.into()),
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
            base_fee_per_gas: value
                .base_fee_per_gas
                .into_iter()
                .map(|v| v.into())
                .collect(),
            gas_used_ratio: value.gas_used_ratio,
            oldest_block: U256(U256Original::from_str(&value.oldest_block.to_string()).unwrap()),
            reward: value
                .reward
                .into_iter()
                .map(|v| v.into_iter().map(|v| v.into()).collect())
                .collect(),
        }
    }
}

impl Into<FeeHistoryOriginal> for FeeHistory {
    fn into(self) -> FeeHistoryOriginal {
        FeeHistoryOriginal {
            base_fee_per_gas: self
                .base_fee_per_gas
                .into_iter()
                .map(|v| U256Original::from_str(&v.0.to_string()).unwrap())
                .collect(),
            gas_used_ratio: self.gas_used_ratio,
            oldest_block: U256Original::from_str(&self.oldest_block.0.to_string()).unwrap(),
            reward: self
                .reward
                .into_iter()
                .map(|v| {
                    v.into_iter()
                        .map(|v| U256Original::from_str(&v.0.to_string()).unwrap())
                        .collect()
                })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[tuple_struct_original_mapping(OtherFieldsOriginal)]
#[pyclass(module = "web3_rush")]
pub struct OtherFields(pub OtherFieldsOriginal);

#[derive(FromPyObject, Display)]
#[tuple_struct_original_mapping(String)]
#[pyclass(module = "web3_rush")]
pub struct HexStr(String);

pub enum AnyStr {
    Str(String),
    Bytes(Vec<u8>),
}

#[derive(Clone, Default, Deserialize)]
#[tuple_struct_original_mapping(H64Original)]
pub struct H64(pub H64Original);

impl ToPyObject for H64 {
    #[inline]
    fn to_object(&self, py: Python<'_>) -> PyObject {
        PyString::new(py, &format!("0x{}", self.0.encode_hex::<String>())).into()
    }
}

impl IntoPy<PyObject> for H64 {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

#[derive(Clone, Deserialize, Default, Serialize, Display)]
#[tuple_struct_original_mapping(H256Original)]
pub struct H256(pub H256Original);

impl ToPyObject for H256 {
    #[inline]
    fn to_object(&self, py: Python<'_>) -> PyObject {
        PyString::new(py, &format!("0x{}", self.0.encode_hex::<String>())).into()
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
        let binding = obj.downcast::<pyo3::types::PyString>()?.to_string();
        let s = binding.trim_start_matches("0x");
        Ok(H256(H256Original::from_str(s).unwrap()))
    }
}

#[derive(Clone)]
#[tuple_struct_original_mapping(H160Original)]
#[pyclass(module = "web3_rush")]
pub struct H160(pub H160Original);

#[derive(Clone, Serialize, Deserialize)]
#[tuple_struct_original_mapping(U64Original)]
pub struct U64(pub U64Original);

impl ToPyObject for U64 {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let binding = Into::<ruint::aliases::U64>::into(self.clone());
        let bytes = binding.as_le_bytes();
        unsafe {
            let obj =
                ffi::_PyLong_FromByteArray(bytes.as_ptr().cast::<c_uchar>(), bytes.len(), 1, 0);
            PyObject::from_owned_ptr(py, obj)
        }
    }
}

impl IntoPy<PyObject> for U64 {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

impl<'source> FromPyObject<'source> for U64 {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        let mut result = ruint::aliases::U64::from(0);

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
        if let Some(last) = Into::<ruint::aliases::U64>::into(result).as_limbs().last() {
            if *last > mask(ruint::aliases::U64::BITS) {
                return Err(PyOverflowError::new_err(format!(
                    "Number to large to fit Uint<{}>",
                    ruint::aliases::U64::BITS
                )));
            }
        }

        Ok(result.into())
    }
}

impl From<ruint::aliases::U64> for U64 {
    fn from(value: ruint::aliases::U64) -> U64 {
        U64(U64Original::from_big_endian(&value.to_be_bytes_vec()))
    }
}

impl Into<ruint::aliases::U64> for U64 {
    fn into(self) -> ruint::aliases::U64 {
        ruint::aliases::U64::from_str(&self.0.to_string()).unwrap()
    }
}

#[derive(Clone, Deserialize, Default)]
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

#[derive(Clone, Serialize, Deserialize, Default)]
#[tuple_struct_original_mapping(BytesOriginal)]
pub struct Bytes(pub BytesOriginal);

impl From<String> for Bytes {
    fn from(value: String) -> Self {
        Bytes(BytesOriginal::from(value.as_bytes().to_vec()))
    }
}

impl ToPyObject for Bytes {
    #[inline]
    fn to_object(&self, py: Python<'_>) -> PyObject {
        PyString::new(py, &format!("0x{}", self.0.encode_hex::<String>())).into()
    }
}

impl IntoPy<PyObject> for Bytes {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[pyclass(module = "web3_rush", get_all)]
pub struct SyncProgress {
    pub current_block: U64,
    pub highest_block: U64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub known_states: Option<U64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pulled_states: Option<U64>,
    pub starting_block: U64,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum SyncingStatus {
    IsFalse,
    IsSyncing(SyncProgress),
}

use ethers::types::SyncProgress as SyncProgressOriginal;
use ethers::types::SyncingStatus as SyncingStatusOriginal;

impl Into<SyncProgressOriginal> for SyncProgress {
    fn into(self) -> SyncProgressOriginal {
        SyncProgressOriginal {
            current_block: self.current_block.into(),
            highest_block: self.highest_block.into(),
            starting_block: self.starting_block.into(),
            pulled_states: match self.pulled_states {
                Some(v) => Some(v.into()),
                None => None,
            },
            known_states: match self.known_states {
                Some(v) => Some(v.into()),
                None => None,
            },
            healed_bytecode_bytes: None,
            healed_bytecodes: None,
            healed_trienode_bytes: None,
            healed_trienodes: None,
            healing_bytecode: None,
            healing_trienodes: None,
            synced_account_bytes: None,
            synced_accounts: None,
            synced_bytecode_bytes: None,
            synced_bytecodes: None,
            synced_storage: None,
            synced_storage_bytes: None,
        }
    }
}

impl From<SyncProgressOriginal> for SyncProgress {
    fn from(value: SyncProgressOriginal) -> Self {
        SyncProgress {
            current_block: value.current_block.into(),
            highest_block: value.highest_block.into(),
            known_states: match value.known_states {
                Some(v) => Some(v.into()),
                None => None,
            },
            pulled_states: match value.pulled_states {
                Some(v) => Some(v.into()),
                None => None,
            },
            starting_block: value.starting_block.into(),
        }
    }
}

impl Into<SyncingStatusOriginal> for SyncingStatus {
    fn into(self) -> SyncingStatusOriginal {
        match self {
            SyncingStatus::IsFalse => SyncingStatusOriginal::IsFalse,
            SyncingStatus::IsSyncing(v) => SyncingStatusOriginal::IsSyncing(Box::new(v.into())),
        }
    }
}

impl From<SyncingStatusOriginal> for SyncingStatus {
    fn from(value: SyncingStatusOriginal) -> Self {
        match value {
            SyncingStatusOriginal::IsFalse => SyncingStatus::IsFalse,
            SyncingStatusOriginal::IsSyncing(v) => SyncingStatus::IsSyncing((*v).into()),
        }
    }
}

impl ToPyObject for SyncingStatus {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match self {
            SyncingStatus::IsFalse => PyBool::new(py, false).into(),
            SyncingStatus::IsSyncing(v) => PyCell::new(py, v.to_owned()).unwrap().into(),
        }
    }
}

impl IntoPy<PyObject> for SyncingStatus {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

#[derive(FromPyObject, Clone, Display)]
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
            BlockId::Hash(h) => BlockIdOriginal::Hash(h.into()),
            BlockId::Number(n) => BlockIdOriginal::Number(n.into()),
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

#[derive(From, Into, Clone, Deserialize)]
#[struct_original_mapping(LogOriginal)]
#[pyclass(module = "web3_rush", get_all)]
pub struct Log {
    /// H160
    pub address: Address,
    /// Topics
    pub topics: Vec<H256>,
    /// Data
    pub data: Bytes,
    /// Block Hash
    #[serde(default, flatten)]
    pub block_hash: Option<H256>,
    /// Block Number
    #[serde(default, flatten)]
    pub block_number: Option<U64>,
    /// Transaction Hash
    #[serde(default, flatten)]
    pub transaction_hash: Option<H256>,
    /// Transaction Index
    #[serde(default, flatten)]
    pub transaction_index: Option<U64>,
    /// Log Index in Block
    #[serde(default, flatten)]
    pub log_index: Option<U256>,
    /// Log Index in Transaction
    #[serde(default, flatten)]
    pub transaction_log_index: Option<U256>,
    /// Log Type
    #[serde(default, flatten)]
    pub log_type: Option<String>,
    /// Removed
    #[serde(default, flatten)]
    pub removed: Option<bool>,
}

use ethers::types::TransactionReceipt as TransactionReceiptOriginal;

use crate::exceptions::wrap_from_hex_error;

#[derive(Clone, Deserialize)]
#[tuple_struct_original_mapping(BloomOriginal)]
pub struct Bloom(pub BloomOriginal);

impl ToPyObject for Bloom {
    #[inline]
    fn to_object(&self, py: Python<'_>) -> PyObject {
        PyString::new(py, &format!("0x{}", self.0.encode_hex::<String>())).into()
    }
}

impl IntoPy<PyObject> for Bloom {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

impl FromPyObject<'_> for Bloom {
    #[inline]
    fn extract(obj: &PyAny) -> PyResult<Self> {
        let binding = obj.downcast::<pyo3::types::PyString>()?.to_string();
        let s = binding.trim_start_matches("0x");
        Ok(Bloom(BloomOriginal::from_str(s).unwrap()))
    }
}

#[derive(From, Into, Clone, Deserialize)]
#[struct_original_mapping(TransactionReceiptOriginal)]
#[pyclass(module = "web3_rush", get_all)]
pub struct TransactionReceipt {
    /// Transaction hash.
    pub transaction_hash: H256,
    /// Index within the block.
    pub transaction_index: U64,
    /// Hash of the block this transaction was included within.
    #[serde(default, flatten)]
    pub block_hash: Option<H256>,
    /// Number of the block this transaction was included within.
    #[serde(default, flatten)]
    pub block_number: Option<U64>,
    /// Sender
    /// Note: default address if the client did not return this value
    /// (maintains backwards compatibility for <= 0.7.0 when this field was missing)
    #[serde(default = "Address::zero", flatten)]
    #[pyo3(name = "from_")]
    pub from: Address,
    /// Recipient (None when contract creation)
    /// Note: Also `None` if the client did not return this value
    /// (maintains backwards compatibility for <= 0.7.0 when this field was missing)
    #[serde(default, flatten)]
    pub to: Option<Address>,
    /// Cumulative gas used within the block after this was executed.
    pub cumulative_gas_used: U256,
    /// Gas used by this transaction alone.
    ///
    /// Gas used is `None` if the the client is running in light client mode.
    #[serde(default, flatten)]
    pub gas_used: Option<U256>,
    /// Contract address created, or `None` if not a deployment.
    #[serde(default, flatten)]
    pub contract_address: Option<Address>,
    /// Logs generated within this transaction.
    pub logs: Vec<Log>,
    /// Status: either 1 (success) or 0 (failure).
    #[serde(default, flatten)]
    pub status: Option<U64>,
    /// State root.
    #[serde(default, flatten)]
    pub root: Option<H256>,
    /// Logs bloom
    pub logs_bloom: Bloom,
    /// Transaction type, Some(1) for AccessList transaction, None for Legacy
    #[serde(default, flatten)]
    pub transaction_type: Option<U64>,
    /// Effective gas price
    #[serde(default, flatten)]
    pub effective_gas_price: Option<U256>,
    // Captures unknown fields such as additional fields used by L2s
    pub other: OtherFields,
}

#[derive(Deserialize, Clone)]
#[struct_original_mapping(TransactionOriginal)]
#[pyclass(module = "web3_rush", get_all)]
pub struct Transaction {
    /// The transaction's hash
    pub hash: H256,

    /// The transaction's nonce
    pub nonce: U256,

    /// Block hash. None when pending.
    #[serde(default, flatten)]
    pub block_hash: Option<H256>,

    /// Block number. None when pending.
    #[serde(default, flatten)]
    pub block_number: Option<U64>,

    /// Transaction Index. None when pending.
    #[serde(default, flatten)]
    pub transaction_index: Option<U64>,

    /// Sender
    #[serde(default = "Address::zero", flatten)]
    #[pyo3(name = "from_")]
    pub from: Address,

    /// Recipient (None when contract creation)
    #[serde(default, flatten)]
    pub to: Option<Address>,

    /// Transferred value
    pub value: U256,

    /// Gas Price, null for Type 2 transactions
    #[serde(flatten)]
    pub gas_price: Option<U256>,

    /// Gas amount
    pub gas: U256,

    /// Input data
    pub input: Bytes,

    /// ECDSA recovery id
    pub v: U64,

    /// ECDSA signature r
    pub r: U256,

    /// ECDSA signature s
    pub s: U256,
    // EIP2718
    /// Transaction type, Some(2) for EIP-1559 transaction,
    /// Some(1) for AccessList transaction, None for Legacy
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "Option::is_none",
        flatten
    )]
    pub transaction_type: Option<U64>,

    // EIP2930
    #[serde(
        rename = "accessList",
        default,
        skip_serializing_if = "Option::is_none",
        flatten
    )]
    pub access_list: Option<AccessList>,

    #[serde(
        rename = "maxPriorityFeePerGas",
        default,
        skip_serializing_if = "Option::is_none",
        flatten
    )]
    /// Represents the maximum tx fee that will go to the miner as part of the user's
    /// fee payment. It serves 3 purposes:
    /// 1. Compensates miners for the uncle/ommer risk + fixed costs of including transaction in a
    /// block; 2. Allows users with high opportunity costs to pay a premium to miners;
    /// 3. In times where demand exceeds the available block space (i.e. 100% full, 30mm gas),
    /// this component allows first price auctions (i.e. the pre-1559 fee model) to happen on the
    /// priority fee.
    ///
    /// More context [here](https://hackmd.io/@q8X_WM2nTfu6nuvAzqXiTQ/1559-wallets)
    pub max_priority_fee_per_gas: Option<U256>,

    #[serde(
        rename = "maxFeePerGas",
        default,
        skip_serializing_if = "Option::is_none",
        flatten
    )]
    /// Represents the maximum amount that a user is willing to pay for their tx (inclusive of
    /// baseFeePerGas and maxPriorityFeePerGas). The difference between maxFeePerGas and
    /// baseFeePerGas + maxPriorityFeePerGas is “refunded” to the user.
    pub max_fee_per_gas: Option<U256>,

    #[serde(
        rename = "chainId",
        default,
        skip_serializing_if = "Option::is_none",
        flatten
    )]
    pub chain_id: Option<U256>,

    /// Captures unknown fields such as additional fields used by L2s
    #[serde(flatten)]
    pub other: crate::types::OtherFields,
}

#[derive(Default, Clone, Deserialize)]
#[pyclass(module = "web3_rush", get_all)]
pub struct Block {
    /// Hash of the block
    pub hash: Option<H256>,
    /// Hash of the parent
    #[serde(default, rename = "parentHash")]
    pub parent_hash: H256,
    /// Hash of the uncles
    #[cfg(not(feature = "celo"))]
    #[serde(default, rename = "sha3Uncles")]
    pub uncles_hash: H256,
    /// Miner/author's address. None if pending.
    #[serde(default, rename = "miner")]
    pub author: Option<Address>,
    /// State root hash
    #[serde(default, rename = "stateRoot")]
    pub state_root: H256,
    /// Transactions root hash
    #[serde(default, rename = "transactionsRoot")]
    pub transactions_root: H256,
    /// Transactions receipts root hash
    #[serde(default, rename = "receiptsRoot")]
    pub receipts_root: H256,
    /// Block number. None if pending.
    pub number: Option<U64>,
    /// Gas Used
    #[serde(default, rename = "gasUsed")]
    pub gas_used: U256,
    /// Gas Limit
    #[cfg(not(feature = "celo"))]
    #[serde(default, rename = "gasLimit")]
    pub gas_limit: U256,
    /// Extra data
    #[serde(default, rename = "extraData")]
    pub extra_data: Bytes,
    /// Logs bloom
    #[serde(rename = "logsBloom")]
    pub logs_bloom: Option<Bloom>,
    /// Timestamp
    #[serde(default)]
    pub timestamp: U256,
    /// Difficulty
    #[cfg(not(feature = "celo"))]
    #[serde(default)]
    pub difficulty: U256,
    /// Total difficulty
    #[serde(rename = "totalDifficulty")]
    pub total_difficulty: Option<U256>,
    /// Seal fields
    #[serde(default, rename = "sealFields")]
    pub seal_fields: Vec<Bytes>,
    /// Uncles' hashes
    #[cfg(not(feature = "celo"))]
    #[serde(default)]
    pub uncles: Vec<H256>,
    /// Transactions
    #[serde(bound = "H256: Serialize + serde::de::DeserializeOwned", default)]
    pub transactions: Vec<H256>,
    /// Size in bytes
    pub size: Option<U256>,
    /// Mix Hash
    #[serde(rename = "mixHash")]
    #[cfg(not(feature = "celo"))]
    pub mix_hash: Option<H256>,
    /// Nonce
    #[cfg(not(feature = "celo"))]
    pub nonce: Option<H64>,
    /// Base fee per unit of gas (if past London)
    #[serde(rename = "baseFeePerGas")]
    pub base_fee_per_gas: Option<U256>,

    #[cfg(feature = "celo")]
    #[cfg_attr(docsrs, doc(cfg(feature = "celo")))]
    /// The block's randomness
    pub randomness: Randomness,

    /// BLS signatures with a SNARK-friendly hash function
    #[cfg(feature = "celo")]
    #[cfg_attr(docsrs, doc(cfg(feature = "celo")))]
    #[serde(rename = "epochSnarkData", default)]
    pub epoch_snark_data: Option<EpochSnarkData>,

    /// Captures unknown fields such as additional fields used by L2s
    #[cfg(not(feature = "celo"))]
    #[serde(flatten)]
    pub other: OtherFields,
}

use ethers::types::Block as BlockOriginal;

impl From<BlockOriginal<H256Original>> for Block {
    fn from(value: BlockOriginal<H256Original>) -> Self {
        Block {
            hash: match value.hash {
                Some(v) => Some(v.into()),
                None => None,
            },
            parent_hash: value.parent_hash.into(),
            uncles_hash: value.uncles_hash.into(),
            author: match value.author {
                Some(v) => Some(v.into()),
                None => None,
            },
            state_root: value.state_root.into(),
            transactions_root: value.transactions_root.into(),
            receipts_root: value.receipts_root.into(),
            number: match value.number {
                Some(v) => Some(v.into()),
                None => None,
            },
            gas_used: value.gas_used.into(),
            gas_limit: value.gas_limit.into(),
            extra_data: value.extra_data.into(),
            logs_bloom: match value.logs_bloom {
                Some(v) => Some(v.into()),
                None => None,
            },
            timestamp: value.timestamp.into(),
            difficulty: value.difficulty.into(),
            total_difficulty: match value.total_difficulty {
                Some(v) => Some(v.into()),
                None => None,
            },
            seal_fields: value.seal_fields.into_iter().map(|v| v.into()).collect(),
            uncles: value.uncles.into_iter().map(|v| v.into()).collect(),
            transactions: value.transactions.into_iter().map(|v| v.into()).collect(),
            size: match value.size {
                Some(v) => Some(v.into()),
                None => None,
            },
            mix_hash: match value.mix_hash {
                Some(v) => Some(v.into()),
                None => None,
            },
            nonce: match value.nonce {
                Some(v) => Some(v.into()),
                None => None,
            },
            base_fee_per_gas: match value.base_fee_per_gas {
                Some(v) => Some(v.into()),
                None => None,
            },
            other: value.other.into(),
        }
    }
}

use ethers::types::Filter as FilterOriginal;
use ethers::types::FilterBlockOption as FilterBlockOptionOriginal;
use ethers::types::ValueOrArray as ValueOrArrayOriginal;

#[derive(FromPyObject, Clone, Deserialize)]
pub struct Range {
    #[serde(rename = "from_block")]
    from_block: Option<BlockNumberParser>,
    #[serde(rename = "to_block")]
    to_block: Option<BlockNumberParser>,
}

/// Represents the target range of blocks for the filter
#[derive(FromPyObject, Clone, Deserialize)]
#[serde(untagged)]
pub enum FilterBlockOption {
    #[pyo3(transparent)]
    Range(Range),
    #[pyo3(transparent)]
    AtBlockHash(H256),
}

impl Into<FilterBlockOptionOriginal> for FilterBlockOption {
    fn into(self) -> FilterBlockOptionOriginal {
        match self {
            FilterBlockOption::Range(Range {
                from_block,
                to_block,
            }) => FilterBlockOptionOriginal::Range {
                from_block: match from_block {
                    Some(v) => Some(v.into()),
                    None => None,
                },
                to_block: match to_block {
                    Some(v) => Some(v.into()),
                    None => None,
                },
            },
            FilterBlockOption::AtBlockHash(v) => FilterBlockOptionOriginal::AtBlockHash(v.into()),
        }
    }
}

/// Union type for representing a single value or a vector of values inside a filter
#[derive(FromPyObject, Debug, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
pub enum ValueOrArray<T> {
    /// A single value
    #[pyo3(transparent)]
    Value(T),
    /// A vector of values
    #[pyo3(transparent)]
    Array(Vec<T>),
}

impl<T> Into<ValueOrArrayOriginal<Option<T>>> for ValueOrArray<Option<T>> {
    fn into(self) -> ValueOrArrayOriginal<Option<T>> {
        match self {
            ValueOrArray::Value(v) => ValueOrArrayOriginal::Value(match v {
                Some(v) => Some(v.into()),
                None => None,
            }),
            ValueOrArray::Array(a) => ValueOrArrayOriginal::Array(
                a.into_iter()
                    .map(|a| match a {
                        Some(v) => Some(v.into()),
                        None => None,
                    })
                    .collect(),
            ),
        }
    }
}

pub type Topic = ValueOrArray<Option<H256>>;

/// Filter for
#[derive(FromPyObject, Clone, Deserialize)]
pub struct Filter {
    /// Filter block options, specifying on which blocks the filter should
    /// match.
    // https://eips.ethereum.org/EIPS/eip-234
    #[serde(flatten)]
    pub block_option: FilterBlockOption,

    /// Address
    #[serde(rename = "address", default)]
    pub address: Option<ValueOrArray<Address>>,

    /// Topics
    // and/or TopicFilter
    #[serde(rename = "topics", default = "default_topics")]
    pub topics: [Option<Topic>; 4],
}

fn default_topics() -> [Option<Topic>; 4] {
    [None, None, None, None]
}

impl Into<FilterOriginal> for Filter {
    fn into(self) -> FilterOriginal {
        let [topics_0, topics_1, topics_2, topics_3] = self.topics;
        FilterOriginal {
            block_option: self.block_option.into(),
            address: match self.address {
                Some(v) => match v {
                    ValueOrArray::Value(v) => Some(ValueOrArrayOriginal::Value(v.into())),
                    ValueOrArray::Array(a) => Some(ValueOrArrayOriginal::Array(
                        a.into_iter().map(|a| a.into()).collect(),
                    )),
                },
                None => None,
            },
            topics: [
                match topics_0 {
                    Some(v) => match v {
                        ValueOrArray::Value(v) => Some(ValueOrArrayOriginal::Value(match v {
                            Some(v) => Some(v.into()),
                            None => None,
                        })),
                        ValueOrArray::Array(a) => Some(ValueOrArrayOriginal::Array(
                            a.into_iter()
                                .map(|a| match a {
                                    Some(v) => Some(v.into()),
                                    None => None,
                                })
                                .collect(),
                        )),
                    },
                    None => None,
                },
                match topics_1 {
                    Some(v) => match v {
                        ValueOrArray::Value(v) => Some(ValueOrArrayOriginal::Value(match v {
                            Some(v) => Some(v.into()),
                            None => None,
                        })),
                        ValueOrArray::Array(a) => Some(ValueOrArrayOriginal::Array(
                            a.into_iter()
                                .map(|a| match a {
                                    Some(v) => Some(v.into()),
                                    None => None,
                                })
                                .collect(),
                        )),
                    },
                    None => None,
                },
                match topics_2 {
                    Some(v) => match v {
                        ValueOrArray::Value(v) => Some(ValueOrArrayOriginal::Value(match v {
                            Some(v) => Some(v.into()),
                            None => None,
                        })),
                        ValueOrArray::Array(a) => Some(ValueOrArrayOriginal::Array(
                            a.into_iter()
                                .map(|a| match a {
                                    Some(v) => Some(v.into()),
                                    None => None,
                                })
                                .collect(),
                        )),
                    },
                    None => None,
                },
                match topics_3 {
                    Some(v) => match v {
                        ValueOrArray::Value(v) => Some(ValueOrArrayOriginal::Value(match v {
                            Some(v) => Some(v.into()),
                            None => None,
                        })),
                        ValueOrArray::Array(a) => Some(ValueOrArrayOriginal::Array(
                            a.into_iter()
                                .map(|a| match a {
                                    Some(v) => Some(v.into()),
                                    None => None,
                                })
                                .collect(),
                        )),
                    },
                    None => None,
                },
            ],
        }
    }
}
