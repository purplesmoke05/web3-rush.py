use pyo3::pyclass;

use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

use std::fmt::Debug;

use web3_rush_macros::struct_original_mapping;

use ethers::types::TxpoolContent as TxpoolContentOriginal;
use ethers::types::TxpoolInspect as TxpoolInspectOriginal;
use ethers::types::TxpoolInspectSummary as TxpoolInspectSummaryOriginal;
use ethers::types::TxpoolStatus as TxpoolStatusOriginal;

use super::address::Address;
use super::bigint::U256;
use super::bigint::U64;
use super::transaction::Transaction;

#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
#[pyclass(module = "web3_rush", get_all)]
pub struct TxpoolContent {
    /// pending tx
    pub pending: BTreeMap<Address, BTreeMap<String, Transaction>>,
    /// queued tx
    pub queued: BTreeMap<Address, BTreeMap<String, Transaction>>,
}

impl From<TxpoolContentOriginal> for TxpoolContent {
    fn from(value: TxpoolContentOriginal) -> Self {
        TxpoolContent {
            pending: value
                .pending
                .into_iter()
                .map(|(k, v)| {
                    (
                        k.into(),
                        v.into_iter().map(|(k, v)| ((k, v.into()))).collect(),
                    )
                })
                .collect(),
            queued: value
                .queued
                .into_iter()
                .map(|(k, v)| {
                    (
                        k.into(),
                        v.into_iter().map(|(k, v)| ((k, v.into()))).collect(),
                    )
                })
                .collect(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[struct_original_mapping(TxpoolStatusOriginal)]
#[pyclass(module = "web3_rush", get_all)]
pub struct TxpoolStatus {
    /// number of pending tx
    pub pending: U64,
    /// number of queued tx
    pub queued: U64,
}

#[derive(Debug, Clone, Deserialize)]
#[struct_original_mapping(TxpoolInspectSummaryOriginal)]
#[pyclass(module = "web3_rush", get_all)]
pub struct TxpoolInspectSummary {
    /// Recipient (None when contract creation)
    pub to: Option<Address>,
    /// Transferred value
    pub value: U256,
    /// Gas amount
    pub gas: U256,
    /// Gas Price
    pub gas_price: U256,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[pyclass(module = "web3_rush", get_all)]
pub struct TxpoolInspect {
    /// pending tx
    pub pending: BTreeMap<Address, BTreeMap<String, TxpoolInspectSummary>>,
    /// queued tx
    pub queued: BTreeMap<Address, BTreeMap<String, TxpoolInspectSummary>>,
}

impl From<TxpoolInspectOriginal> for TxpoolInspect {
    fn from(value: TxpoolInspectOriginal) -> Self {
        TxpoolInspect {
            pending: value
                .pending
                .into_iter()
                .map(|(k, v)| {
                    (
                        k.into(),
                        v.into_iter().map(|(k, v)| ((k, v.into()))).collect(),
                    )
                })
                .collect(),
            queued: value
                .queued
                .into_iter()
                .map(|(k, v)| {
                    (
                        k.into(),
                        v.into_iter().map(|(k, v)| ((k, v.into()))).collect(),
                    )
                })
                .collect(),
        }
    }
}
