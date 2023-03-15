
use derivative::Derivative;
use derive_more::{Display, From, Into};
use inter_struct::StructInto;
use num_bigint::BigUint;
use pyo3::exceptions::PyTypeError;
use pyo3::ffi::PyObject;
use pyo3::types::PyString;

use pyo3::{ToPyObject, Python, PyAny, PyResult, IntoPy, PyErr};
use pyo3::{FromPyObject, pyclass};
use serde::{Serialize, Deserialize};
use solders_macros::{EnumIntoPy, enum_original_mapping};
use thinwrap::thin_wrap;
use thinwrap::auto_deref;
use core::ops::{Deref, DerefMut};
use web3::types::{Address as AddressOriginal, Block, U64, U256, Bytes, AccessList, H256 as H256Original, H160};
use web3::types::BlockNumber as BlockNumberOriginal;
use web3::types::FeeHistory as FeeHistoryOriginal;
use web3::types::BlockId as BlockIdOriginal;
use web3_rush_macros::{struct_original_mapping, tuple_struct_original_mapping};
use crate::web3::types::CallRequest as CallRequestOriginal;


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

#[pyclass(module = "web3_rush", subclass)]
#[derive(Debug, Clone)]
#[tuple_struct_original_mapping(H160)]
pub struct Address(pub H160);

#[derive(FromPyObject)]
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

#[pyclass(module = "web3_rush")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[tuple_struct_original_mapping(FeeHistoryOriginal)]
pub struct FeeHistory(pub FeeHistoryOriginal);

#[pyclass(module = "web3_rush", subclass)]
#[derive(FromPyObject, Display)]
#[tuple_struct_original_mapping(String)]
pub struct HexStr(String);

pub enum AnyStr {
    Str(String),
    Bytes(Vec<u8>)
}


#[pyclass(module = "web3_rush")]
#[derive(From, Into, Clone, Debug, PartialEq)]
#[struct_original_mapping(CallRequestOriginal)]
pub struct CallRequest {
    /// Sender address (None for arbitrary address)
    pub from: Option<H160>,
    /// To address (None allowed for eth_estimateGas)
    pub to: Option<H160>,
    /// Supplied gas (None for sensible default)
    pub gas: Option<U256>,
    /// Gas price (None for sensible default)
    pub gas_price: Option<U256>,
    /// Transfered value (None for no transfer)
    pub value: Option<U256>,
    /// Data (None for empty data)
    pub data: Option<Bytes>,
    /// Transaction type, Some(1) for AccessList transaction, None for Legacy
    pub transaction_type: Option<U64>,
    /// Access list
    pub access_list: Option<AccessList>,
    /// Max fee per gas
    pub max_fee_per_gas: Option<U256>,
    /// miner bribe
    pub max_priority_fee_per_gas: Option<U256>,
}

#[pyclass(module = "web3_rush")]
#[derive(Clone)]
#[tuple_struct_original_mapping(H256Original)]
pub struct H256(pub H256Original);


#[derive(FromPyObject)]
pub enum BlockId {
    /// By Hash
    Hash(H256),
    /// By Number
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
