use ethers::types::H256 as H256Original;

use pyo3::pyclass;

use serde::{Deserialize, Serialize};

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

use super::address::Address;
use super::bigint::U256;
use super::bigint::U64;
use super::log::Bloom;
use super::str::Bytes;
use super::str::H256;
use super::str::H64;
use super::transaction::OtherFields;

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
