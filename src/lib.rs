
pub mod utils;

use std::ops::Add;
use std::str::FromStr;

use async_std::task::block_on;
use derive_more::From;
use derive_more::Into;
use exceptions::wrap_web3_error;
use pyo3::exceptions::PyTypeError;
use pyo3::{prelude::*};
use tokio::runtime::Runtime;
use types::Address;
use types::AnyStr;
use types::BlockId;
use types::BlockNumberParser;
use types::FeeHistory;
use utils::add_0x_prefix;
use utils::encode_hex;
use utils::to_hex_i32;
use utils::to_int;
use web3;
use web3::types::Bytes;
use web3::types::CallRequest as CallRequestOriginal;
use web3::types::Transaction as TransactionOriginal;
use web3::types::BlockHeader as BlockHeaderOriginal;
use serde::{Deserialize, Serialize};
pub mod exceptions;
pub mod types;
use types::{HexStr, Primitives, CallRequest};
use ruint::aliases::U256;

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
pub struct Transaction(pub TransactionOriginal);

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
pub struct BlockData(pub BlockHeaderOriginal);

#[pyclass(module = "web3_rush", subclass)]
#[derive(
    From,
    Into,
)]
pub struct EthHttp(web3::api::Eth<web3::transports::Http>);
use num_bigint::{BigInt, BigUint};

#[pymethods]
impl EthHttp {
    #[getter]
    pub fn accounts(&self) -> PyResult<Vec<String>> {
        match block_on(self.0.accounts()){
            Ok(result) => {
                Ok(result.into_iter().map(|r| {r.to_string() }).collect())
            },
            Err(err) => {
                Err(wrap_web3_error(err))
            },
        }
    }

    #[getter]
    pub fn hashrate(&self) -> PyResult<BigUint> {
        match block_on(self.0.hashrate()){
            Ok(result) => {
                Ok(BigUint::from_str(&result.to_string()).unwrap())
            },
            Err(err) => {
                Err(wrap_web3_error(err))
            },
        }
    }

    #[getter]
    pub fn block_number(&self) -> PyResult<u64> {
        match block_on(self.0.block_number()){
            Ok(result) => {
                Ok(result.as_u64())
            },
            Err(err) => {
                Err(wrap_web3_error(err))
            },
        }
    }

    #[getter]
    pub fn chain_id(&self) -> PyResult<BigUint> {
        match block_on(self.0.chain_id()){
            Ok(result) => {
                Ok(BigUint::from_str(&result.to_string()).unwrap())
            },
            Err(err) => {
                Err(wrap_web3_error(err))
            },
        }
    }

    #[getter]
    pub fn coinbase(&self) -> PyResult<String> {
        match block_on(self.0.coinbase()){
            Ok(result) => {
                Ok(result.to_string())
            },
            Err(err) => {
                Err(wrap_web3_error(err))
            },
        }
    }

    #[getter]
    pub fn gas_price(&self) -> PyResult<BigUint> {
        match block_on(self.0.gas_price()){
            Ok(result) => {
                Ok(BigUint::from_str(&result.to_string()).unwrap())
            },
            Err(err) => {
                Err(wrap_web3_error(err))
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
                Err(wrap_web3_error(err))
            },
        }
    }

    pub fn fee_history(&self, block_count: U256, newest_block: BlockNumberParser, reward_percentiles: Option<Vec<f64>>) -> PyResult<FeeHistory> {
        match block_on(self.0.fee_history(block_count.into(), newest_block.into(), reward_percentiles)){
            Ok(result) => {
                Ok(result.into())
            },
            Err(err) => {
                Err(wrap_web3_error(err))
            },
        }
    }
    
    pub fn call(&self, req: CallRequest, block: Option<BlockId>) -> PyResult<Vec<u8>> {
        let block = match block {
            Some(b) => Some(b.into()),
            None => None,
        };
        match block_on(self.0.call(req.into(), block)){
            Ok(result) => {
                Ok(result.0)
            },
            Err(err) => {
                Err(wrap_web3_error(err))
            },
        }
    }
}

#[pyclass(module = "web3_rush", subclass)]
#[derive(
    From,
    Into,
)]
pub struct Web3ApiHttp(web3::api::Web3Api<web3::transports::Http>);

#[pymethods]
impl Web3ApiHttp {
    #[getter]
    pub fn client_version(&self) -> PyResult<String> {
        match block_on(self.0.client_version()){
            Ok(result) => {
                Ok(result)
            },
            Err(err) => {
                Err(wrap_web3_error(err))
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
    client: web3::Web3<web3::transports::Http>
}

#[pymethods]
impl Web3 {
    #[new]
    pub fn new(url: String) -> PyResult<Self> {
        let transport = web3::transports::Http::new(&url);
        match transport {
            Ok(transport) => {
                let client = web3::Web3::new(transport);
                Ok(Web3 { client })
            },
            Err(err) => {
                Err(wrap_web3_error(err))
            },
        }
    }

    #[getter]
    pub fn web3(&self) -> PyResult<Web3ApiHttp> {
        Ok(Web3ApiHttp(self.client.web3()))
    }
    #[getter]
    pub fn eth(&self) -> PyResult<EthHttp> {
        Ok(EthHttp(self.client.eth()))
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