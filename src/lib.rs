pub mod utils;

use std::ops::Add;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time;

use async_std::task::block_on;
use derive_more::From;
use derive_more::Into;
use ethers::providers::Middleware;
use exceptions::wrap_parse_error;
use exceptions::wrap_provider_error;
use exceptions::wrap_web3_error;
use pyo3::exceptions::PyTypeError;
use pyo3::{prelude::*};
use pythonize::depythonize;
use tokio::runtime::Runtime;
use types::Address;
use types::AnyStr;
use types::BlockId;
use types::BlockNumberParser;
use types::Bytes;
use types::Eip1559TransactionRequest;
use types::Eip2930TransactionRequest;
use types::FeeHistory;
use types::H256;
use types::NameOrAddress;
use types::TransactionReceipt;
use utils::add_0x_prefix;
use utils::encode_hex;
use utils::to_hex_i32;
use utils::to_int;
use ethers::types::Address as AddressOriginal;
use ethers::types::Transaction as TransactionOriginal;
use ethers::types::H256 as H256Original;
use ethers::types::U256 as U256Original;
use serde::{Deserialize, Serialize};
pub mod exceptions;
pub mod types;
use types::{HexStr, Primitives, TransactionRequest, TypedTransaction};
use ruint::aliases::U256;

#[derive(
    Serialize,
    Deserialize,
    PartialEq,
    Debug,
    Clone,
)]
#[tuple_struct_original_mapping(TransactionOriginal)]
#[pyclass(module = "web3_rush")]
pub struct Transaction(pub TransactionOriginal);

#[derive(
    From,
    Into,
)]
#[pyclass(module = "web3_rush")]
pub struct EthHttp(Arc<ethers::providers::Provider<ethers::providers::Http>>);
use num_bigint::{BigInt, BigUint};
use web3_rush_macros::tuple_struct_original_mapping;

#[pymethods]
impl EthHttp {
    #[getter]
    pub fn accounts(&self) -> PyResult<Vec<Address>> {
        match block_on(self.0.get_accounts()){
            Ok(result) => {
                Ok(result.into_iter().map(|r| {Address(r) }).collect())
            },
            Err(err) => {
                Err(wrap_provider_error(err))
            },
        }
    }

    #[getter]
    pub fn block_number(&self) -> PyResult<u64> {
        match block_on(self.0.get_block_number()){
            Ok(result) => {
                Ok(result.as_u64())
            },
            Err(err) => {
                Err(wrap_provider_error(err))
            },
        }
    }

    #[getter]
    pub fn chain_id(&self) -> PyResult<BigUint> {
        match block_on(self.0.get_chainid()){
            Ok(result) => {
                Ok(BigUint::from_str(&result.to_string()).unwrap())
            },
            Err(err) => {
                Err(wrap_provider_error(err))
            },
        }
    }

    #[getter]
    pub fn coinbase(&self) -> PyResult<Address> {
        match block_on(self.0.request("eth_coinbase", ())){
            Ok(result) => {
                Ok(result)
            },
            Err(err) => {
                Err(wrap_provider_error(err))
            },
        }
    }

    #[getter]
    pub fn gas_price(&self) -> PyResult<BigUint> {
        match block_on(self.0.get_gas_price()){
            Ok(result) => {
                Ok(BigUint::from_str(&result.to_string()).unwrap())
            },
            Err(err) => {
                Err(wrap_provider_error(err))
            },
        }
    }

    #[getter]
    pub fn mining(&self) -> PyResult<bool> {
        match block_on(self.0.mining()){
            Ok(result) => {
                Ok(result)
            },
            Err(err) => {
                Err(wrap_provider_error(err))
            },
        }
    }

    pub fn fee_history(&self, block_count: U256, newest_block: BlockNumberParser, reward_percentiles: Vec<f64>) -> PyResult<FeeHistory> {
        match block_on(self.0.fee_history::<U256Original>(block_count.into(), newest_block.into(), &reward_percentiles)){
            Ok(result) => {
                Ok(result.into())
            },
            Err(err) => {
                Err(wrap_provider_error(err))
            },
        }
    }
    
    pub fn call(&self, req: TypedTransaction, block: Option<BlockId>) -> PyResult<Bytes> {
        let block = match block {
            Some(b) => Some(b.into()),
            None => None,
        };
        match block_on(self.0.call(&req.into(), block)){
            Ok(result) => {
                Ok(result.into())
            },
            Err(err) => {
                Err(wrap_provider_error(err))
            },
        }
    }

    pub fn estimate_gas(&self, req: TypedTransaction, block: Option<BlockId>) -> PyResult<U256> {
        let block = match block {
            Some(b) => Some(b.into()),
            None => None,
        };
        match block_on(self.0.estimate_gas(&req.into(), block)){
            Ok(result) => {
                Ok(result.into())
            },
            Err(err) => {
                Err(wrap_provider_error(err))
            },
        }
    }

    pub fn get_transaction(&self, tx_hash: H256) -> PyResult<Option<Transaction>> {
        match block_on(self.0.get_transaction::<H256>(tx_hash.into())) {
            Ok(result) => {
                match result {
                    Some(result) => {Ok(Some(result.into()))},
                    None => {Ok(None)}
                }
            },
            Err(err) => {
                Err(wrap_provider_error(err))
            },
        }
    }

    pub fn send_transaction(&self, tx: PyObject) -> PyResult<H256> {
        let tx = Python::with_gil(|py| {
            match depythonize::<TransactionRequest>(tx.as_ref(py)) {
                Ok(res) => Ok(TypedTransaction::Legacy(res)),
                Err(err) => match depythonize::<Eip1559TransactionRequest>(tx.as_ref(py)) {
                    Ok(res) => Ok(TypedTransaction::Eip1559(res)),
                    Err(err) => match depythonize::<Eip2930TransactionRequest>(tx.as_ref(py)) {
                        Ok(res) => Ok(TypedTransaction::Eip2930(res)),
                        Err(err) => {
                            return Err(err);
                        }
                    }
                },
            }
        })?;
        match block_on(self.0.send_transaction::<TypedTransaction>(tx, None)) {
            Ok(result) => {
                Ok(types::H256(H256Original::from_slice(result.as_ref())))
            },
            Err(err) => {
                Err(wrap_provider_error(err))
            },
        }
    }

    pub fn get_transaction_receipt(&self, tx_hash: H256) -> PyResult<Option<TransactionReceipt>> {
        match block_on(self.0.get_transaction_receipt::<H256>(tx_hash.into())) {
            Ok(result) => {
                match result {
                    Some(result) => Ok(Some(result.into())),
                    None => Ok(None)
                }
            },
            Err(err) => {
                Err(wrap_provider_error(err))
            },
        }
    }

    pub fn get_transaction_count(&self, address: Address, block: Option<BlockId>) -> PyResult<U256> {
        let block = match block {
            Some(b) => Some(b.into()),
            None => None,
        };
        match block_on(self.0.get_transaction_count::<AddressOriginal>(address.into(), block.into())) {
            Ok(result) => {
                Ok(result.into())
            },
            Err(err) => {
                Err(wrap_provider_error(err))
            },
        }
    }

    pub fn wait_for_transaction_receipt(&self, tx_hash: H256, timeout: f64, poll_latency: f64) -> PyResult<TransactionReceipt> {
        let wait_start = time::Instant::now();

        while wait_start.elapsed() <= time::Duration::from_secs_f64(timeout) {
            match block_on(self.0.get_transaction_receipt::<H256>(tx_hash.clone().into())) {
                Ok(result) => {
                    match result {
                        Some(result) => {
                            return Ok(result.into());
                        },
                        None =>{
                            thread::sleep(time::Duration::from_secs_f64(poll_latency));
                        }
                    }
                }
                Err(err) => {
                    return Err(wrap_provider_error(err));
                }
            };
        }
        Err(PyTypeError::new_err(format!("Transaction {} is not in the chain after {} seconds", tx_hash.0.to_string(), timeout)))
    }
}

#[derive(
    From,
    Into,
    Clone
)]
#[pyclass(module = "web3_rush")]
pub struct Web3ApiHttp(Arc<ethers::providers::Provider<ethers::providers::Http>>);

#[pymethods]
impl Web3ApiHttp {
    #[getter]
    pub fn client_version(&self) -> PyResult<String> {
        match block_on(self.0.client_version()){
            Ok(result) => {
                Ok(result)
            },
            Err(err) => {
                Err(wrap_provider_error(err))
            },
        }
    }

    #[getter]
    pub fn api(&self) -> PyResult<String> {
        Ok("0.1.0".to_owned())
    }

    #[staticmethod]
    pub fn toHex(primitive: Option<Primitives>, hexstr: Option<HexStr>, text: Option<String>) -> PyResult<String> {
        Web3ApiHttp::to_hex(primitive, hexstr, text)
    }

    #[staticmethod]
    pub fn to_hex(primitive: Option<Primitives>, hexstr: Option<HexStr>, text: Option<String>) -> PyResult<String> {
        if let Some(hexstr) = hexstr {
            Ok(add_0x_prefix(hexstr).into())
        } else if let Some(text) = text {
            match encode_hex(AnyStr::Str(text)) {
                Ok(text) => {
                    Ok(text.into())
                },
                Err(err) => {
                    Err(PyTypeError::new_err(err))
                },
            }
        } else {
            match primitive {
                Some(primitive) => {
                    match primitive {
                        Primitives::Bool(b) => {
                            match b {
                                true => Ok(format!("0x1")),
                                false => Ok(format!("0x0")),
                            }
                        },
                        Primitives::String(str) => {
                            match encode_hex(AnyStr::Str(str)) {
                                Ok(text) => {
                                    Ok(text.into())
                                },
                                Err(err) => {
                                    Err(PyTypeError::new_err(err))
                                },
                            }
                        }
                        Primitives::Bytes(bytes) => {
                            match encode_hex(AnyStr::Bytes(bytes)) {
                                Ok(text) => {
                                    Ok(text.into())
                                },
                                Err(err) => {
                                    Err(PyTypeError::new_err(err))
                                },
                            }
                        },
                        Primitives::Int(i) => {
                            Ok(to_hex_i32(i as i32))
                        },
                    }
                },
                None => Err(PyTypeError::new_err("")),
            }
        }
    }

    #[staticmethod]
    pub fn to_int(primitive: Option<Primitives>, hexstr: Option<HexStr>, text: Option<String>) -> PyResult<isize> {
        to_int(primitive, hexstr, text)
    }

    #[staticmethod]
    pub fn toInt(primitive: Option<Primitives>, hexstr: Option<HexStr>, text: Option<String>) -> PyResult<isize> {
        to_int(primitive, hexstr, text)
    }
}

#[pyclass(module = "web3_rush")]
pub struct Web3 {
    client: Arc<ethers::providers::Provider<ethers::providers::Http>>
}

#[pymethods]
impl Web3 {
    #[new]
    pub fn new(url: String) -> PyResult<Self> {
        let client = ethers::providers::Provider::<ethers::providers::Http>::try_from(url);
        match client {
            Ok(client) => {
                Ok(Web3 { client:Arc::new(client) })
            },
            Err(err) => {
                Err(wrap_parse_error(err))
            },
        }
    }

    #[getter]
    pub fn web3(&self) -> PyResult<Web3ApiHttp> {
        Ok(Web3ApiHttp(self.client.clone()))
    }
    #[getter]
    pub fn eth(&self) -> PyResult<EthHttp> {
        Ok(EthHttp(self.client.clone()))
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn web3_rush(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Web3>()?;
    exceptions::init_module(_py, m, m)?;
    Ok(())
}