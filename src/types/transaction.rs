use derive_more::{From, Into};

use ethers::types::transaction::eip1559::Eip1559TransactionRequest as Eip1559TransactionRequestOriginal;
use ethers::types::transaction::eip2718::TypedTransaction as TypedTransactionOriginal;
use ethers::types::transaction::eip2930::AccessList as AccessListOriginal;
use ethers::types::transaction::eip2930::Eip2930TransactionRequest as Eip2930TransactionRequestOriginal;
use ethers::types::{
    OtherFields as OtherFieldsOriginal, Transaction as TransactionOriginal,
    TransactionRequest as TransactionRequestOriginal,
};

use pyo3::{pyclass, FromPyObject, IntoPy, PyObject, Python};

use serde::{Deserialize, Serialize};
use solders_macros::EnumIntoPy;

use std::fmt::Debug;

use web3_rush_macros::{
    struct_original_mapping, tuple_enum_original_mapping, tuple_struct_original_mapping,
};

#[pyclass(module = "web3_rush")]
#[derive(Clone, Deserialize, Debug)]
#[struct_original_mapping(TransactionRequestOriginal)]
pub struct TransactionRequest {
    /// Sender address
    pub from: Option<Address>,
    /// Recipient address (None for contract creation)
    pub to: Option<NameOrAddress>,
    /// Supplied gas (None for sensible default)
    pub gas: Option<U256>,
    /// Gas price (None for sensible default)
    #[serde(alias = "gasPrice")]
    pub gas_price: Option<U256>,
    /// Transfered value (None for no transfer)
    #[serde(default)]
    pub value: Option<U256>,
    /// Transaction data (None for empty bytes)
    pub data: Option<Bytes>,
    /// Transaction nonce (None for next available nonce)
    pub nonce: Option<U256>,
    /// Chain ID (None for mainnet)
    #[serde(default, alias = "chainId")]
    pub chain_id: Option<U64>,
}

#[pyclass(module = "web3_rush")]
#[derive(Clone, Deserialize, Debug)]
#[struct_original_mapping(Eip2930TransactionRequestOriginal)]
pub struct Eip2930TransactionRequest {
    #[serde(flatten)]
    pub tx: TransactionRequest,
    pub access_list: AccessList,
}

#[pyclass(module = "web3_rush")]
#[derive(Clone, Debug, PartialEq, Deserialize, Eq, Default)]
#[tuple_struct_original_mapping(AccessListOriginal)]
pub struct AccessList(pub AccessListOriginal);

#[pyclass(module = "web3_rush")]
#[derive(From, Into, Clone, Deserialize, Debug)]
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

#[derive(FromPyObject, Clone, EnumIntoPy, Deserialize, Debug)]
#[tuple_enum_original_mapping(TypedTransactionOriginal)]
#[serde(untagged)]
pub enum TypedTransaction {
    #[pyo3(transparent)]
    Legacy(TransactionRequest),
    #[pyo3(transparent)]
    Eip2930(Eip2930TransactionRequest),
    #[pyo3(transparent)]
    Eip1559(Eip1559TransactionRequest),
}

use ethers::types::TransactionReceipt as TransactionReceiptOriginal;

use super::address::Address;
use super::address::NameOrAddress;
use super::bigint::U256;
use super::bigint::U64;
use super::log::Bloom;
use super::log::Log;
use super::str::Bytes;
use super::str::H256;

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

#[derive(Deserialize, Clone, Debug, PartialEq)]
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
    pub other: OtherFields,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[pyclass(module = "web3_rush", get_all)]
pub struct GotTransaction {
    pub hash: H256,
    pub nonce: U256,
    #[serde(default, flatten)]
    pub block_hash: Option<H256>,
    #[serde(default, flatten)]
    pub block_number: Option<U64>,
    #[serde(default, flatten)]
    pub transaction_index: Option<U64>,
    #[serde(default = "Address::zero", flatten)]
    #[pyo3(name = "from_")]
    pub from: Address,
    #[serde(default, flatten)]
    pub to: Option<Address>,
    pub value: U256,
    #[serde(flatten)]
    pub gas_price: Option<U256>,
    pub gas: U256,
    pub input: Bytes,
    pub v: U64,
    pub r: String,
    pub s: String,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "Option::is_none",
        flatten
    )]
    pub transaction_type: Option<U64>,
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
    pub max_priority_fee_per_gas: Option<U256>,
    #[serde(
        rename = "maxFeePerGas",
        default,
        skip_serializing_if = "Option::is_none",
        flatten
    )]
    pub max_fee_per_gas: Option<U256>,
    #[serde(
        rename = "chainId",
        default,
        skip_serializing_if = "Option::is_none",
        flatten
    )]
    pub chain_id: Option<U256>,
    #[serde(flatten)]
    pub other: OtherFields,
}

impl From<Transaction> for GotTransaction {
    fn from(value: Transaction) -> Self {
        Self {
            hash: value.hash,
            nonce: value.nonce,
            block_hash: value.block_hash,
            block_number: value.block_number,
            transaction_index: value.transaction_index,
            from: value.from,
            to: value.to,
            value: value.value,
            gas_price: value.gas_price,
            gas: value.gas,
            input: value.input,
            v: value.v,
            r: format!(
                "0x{}",
                hex_string::HexString::from_bytes(&value.r.0.to_bytes_be()).as_string()
            ),
            s: format!(
                "0x{}",
                hex_string::HexString::from_bytes(&value.s.0.to_bytes_be()).as_string()
            ),
            transaction_type: value.transaction_type,
            access_list: value.access_list,
            max_priority_fee_per_gas: value.max_priority_fee_per_gas,
            max_fee_per_gas: value.max_fee_per_gas,
            chain_id: value.chain_id,
            other: value.other,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[tuple_struct_original_mapping(OtherFieldsOriginal)]
#[pyclass(module = "web3_rush")]
pub struct OtherFields(pub OtherFieldsOriginal);
