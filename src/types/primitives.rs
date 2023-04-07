use derive_more::Display;

use num_bigint::BigInt;
use pyo3::FromPyObject;
use serde::Deserialize;

use ethers::types::{BlockId as BlockIdOriginal, BlockNumber as BlockNumberOriginal};

use super::str::H256;

#[derive(FromPyObject, Debug)]
pub enum Number {
    #[pyo3(transparent, annotation = "int, decimal.Decimal")]
    Int(BigInt),
    #[pyo3(transparent, annotation = "float")]
    Float(f64),
    #[pyo3(transparent, annotation = "str")]
    String(String),
}

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
