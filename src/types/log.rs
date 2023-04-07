use derive_more::{From, Into};

use ethers::types::{Bloom as BloomOriginal, Log as LogOriginal};

use hex::ToHex;

use pyo3::types::PyString;
use pyo3::{pyclass, FromPyObject, IntoPy, PyAny, PyObject, PyResult, Python, ToPyObject};

use serde::Deserialize;

use std::fmt::Debug;
use std::str::FromStr;
use web3_rush_macros::{struct_original_mapping, tuple_struct_original_mapping};

use super::address::Address;
use super::bigint::U256;
use super::bigint::U64;
use super::primitives::BlockNumberParser;
use super::str::Bytes;
use super::str::H256;

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
