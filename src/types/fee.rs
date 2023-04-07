use ethers::types::{FeeHistory as FeeHistoryOriginal, U256 as U256Original};

use pyo3::pyclass;

use std::str::FromStr;

use super::bigint::U256;

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
            oldest_block: value.oldest_block.into(),
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
