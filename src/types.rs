use derive_more::{Display, From, Into};

use ethers::types::Bloom as BloomOriginal;
use pyo3::{FromPyObject, pyclass};
use serde::{Serialize, Deserialize};
use solders_macros::{EnumIntoPy, enum_original_mapping};
use core::ops::{Deref, DerefMut};
use std::collections::BTreeMap;
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

#[pyclass(module = "web3_rush")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[tuple_struct_original_mapping(AddressOriginal)]
pub struct Address(pub AddressOriginal);

#[derive(Clone, Debug)]
#[tuple_enum_original_mapping(NameOrAddressOriginal)]
pub enum NameOrAddress {
    /// An ENS Name (format does not get checked)
    Name(String),
    /// An Ethereum Address
    Address(Address),
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[tuple_struct_original_mapping(FeeHistoryOriginal)]
#[pyclass(module = "web3_rush")]
pub struct FeeHistory(pub FeeHistoryOriginal);

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

#[pyclass(module = "web3_rush")]
#[derive(Clone)]
#[tuple_struct_original_mapping(H256Original)]
pub struct H256(pub H256Original);

#[derive(Clone)]
#[tuple_struct_original_mapping(H160Original)]
#[pyclass(module = "web3_rush")]
pub struct H160(pub H160Original);

#[derive(Clone)]
#[tuple_struct_original_mapping(U64Original)]
#[pyclass(module = "web3_rush")]
pub struct U64(pub U64Original);

impl From<i32> for U64 {
    fn from(value: i32) -> U64 {
        U64(U64Original::from(value as u64))
    }
}

#[derive(Clone)]
#[tuple_struct_original_mapping(U256Original)]
#[pyclass(module = "web3_rush")]
pub struct U256(pub U256Original);

#[derive(Clone)]
#[tuple_struct_original_mapping(BytesOriginal)]
#[pyclass(module = "web3_rush")]
pub struct Bytes(pub BytesOriginal);

#[derive(FromPyObject, Clone)]
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

use pyo3::types::PyDict;

#[pyclass(module = "web3_rush")]
#[derive(Clone)]
#[struct_original_mapping(TransactionRequestOriginal)]
pub struct TransactionRequest {
    /// Sender address
    pub from: Option<Address>,
    /// Recipient address (None for contract creation)
    pub to: Option<NameOrAddress>,
    /// Supplied gas (None for sensible default)
    pub gas: Option<U256>,
    /// Gas price (None for sensible default)
    pub gas_price: Option<U256>,
    /// Transfered value (None for no transfer)
    pub value: Option<U256>,
    /// Transaction data (None for empty bytes)
    pub data: Option<Bytes>,
    /// Transaction nonce (None for next available nonce)
    pub nonce: Option<U256>,
    /// Chain ID (None for mainnet)
    pub chain_id: Option<U64>,
}

use ethers::types::transaction::eip1559::Eip1559TransactionRequest as Eip1559TransactionRequestOriginal;
use ethers::types::transaction::eip2930::Eip2930TransactionRequest as Eip2930TransactionRequestOriginal;
use ethers::types::transaction::eip2930::AccessList as AccessListOriginal;

#[pyclass(module = "web3_rush")]
#[derive(Clone)]
#[struct_original_mapping(Eip2930TransactionRequestOriginal)]
pub struct Eip2930TransactionRequest {
    pub tx: TransactionRequest,
    pub access_list: AccessList,
}

#[pyclass(module = "web3_rush")]
#[derive(Clone, Debug, PartialEq)]
#[tuple_struct_original_mapping(AccessListOriginal)]
pub struct AccessList(pub AccessListOriginal);


#[pyclass(module = "web3_rush")]
#[derive(From, Into, Clone)]
#[struct_original_mapping(Eip1559TransactionRequestOriginal)]
pub struct Eip1559TransactionRequest {
    /// Sender address or ENS name
    pub from: Option<Address>,

    /// Recipient address (None for contract creation)
    pub to: Option<NameOrAddress>,

    /// Supplied gas (None for sensible default)
    pub gas: Option<U256>,

    /// Transferred value (None for no transfer)
    pub value: Option<U256>,

    /// The compiled code of a contract OR the first 4 bytes of the hash of the
    /// invoked method signature and encoded parameters. For details see Ethereum Contract ABI
    pub data: Option<Bytes>,

    /// Transaction nonce (None for next available nonce)
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
    pub max_priority_fee_per_gas: Option<U256>,

    /// Represents the maximum amount that a user is willing to pay for their tx (inclusive of
    /// baseFeePerGas and maxPriorityFeePerGas). The difference between maxFeePerGas and
    /// baseFeePerGas + maxPriorityFeePerGas is “refunded” to the user.
    pub max_fee_per_gas: Option<U256>,

    /// Chain ID (None for mainnet)
    pub chain_id: Option<U64>,
}


#[derive(FromPyObject, Clone)]
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
