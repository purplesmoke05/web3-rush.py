
use derive_more::{Display, From, Into};
use num_bigint::BigUint;
use pyo3::exceptions::PyTypeError;
use pyo3::ffi::PyObject;
use pyo3::types::PyString;
use pyo3::{ToPyObject, Python, PyAny, PyResult, IntoPy, PyErr};
use pyo3::{FromPyObject, pyclass};
use serde::{Serialize, Deserialize};
use solders_macros::{EnumIntoPy, enum_original_mapping};
use web3::types::{Address as AddressOriginal, Block, U64};
use web3::types::BlockNumber as BlockNumberOriginal;
use web3::types::FeeHistory as FeeHistoryOriginal;
use web3::types::BlockId as BlockIdOriginal;


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
#[derive(
    Serialize,
    Deserialize,
    PartialEq,
    Debug,
    Clone,
    From,
    Into,
)]
pub struct Address(pub AddressOriginal);

impl Address {
    pub fn new(v: web3::types::H160) -> Self {
        Address(v)
    }
}

#[derive(FromPyObject)]
pub enum BlockNumberParser {
    #[pyo3(transparent, annotation = "str")]
    String(String),
    #[pyo3(transparent, annotation = "int")]
    BigUint(u64),
}

impl BlockNumberParser {
    pub fn into_org(self) -> Result<BlockNumberOriginal, PyErr> {
        match self {
            BlockNumberParser::String(s) => {
                match &*s {
                    "latest" => Ok(BlockNumberOriginal::Latest),
                    "earliest" => Ok(BlockNumberOriginal::Earliest),
                    "pending" => Ok(BlockNumberOriginal::Pending),
                    _ => {
                        Err(PyTypeError::new_err(""))
                    },
                }
            },
            BlockNumberParser::BigUint(n) => {
                Ok(BlockNumberOriginal::Number(n.into()))
            },
        }
    }
}

#[derive(From, Into, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[pyclass]
pub struct FeeHistory(pub FeeHistoryOriginal);

#[pyclass(module = "web3_rush", subclass)]
#[derive(
    FromPyObject,
    Debug,
    Display,
)]
pub struct HexStr(String);

impl HexStr {
    pub fn new(v: String) -> Self {
        HexStr(v)
    }

    pub fn value(self) -> String {
        self.0
    }
}

pub enum AnyStr {
    Str(String),
    Bytes(Vec<u8>)
}
