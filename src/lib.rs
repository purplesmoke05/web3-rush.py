pub mod utils;

use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time;

use async_std::task::block_on;
use derive_more::From;
use derive_more::Into;
use ethers::providers::Middleware;
use ethers::signers::Wallet;
use ethers::types::Address as AddressOriginal;
use ethers::types::U256 as U256Original;
use exceptions::wrap_from_wallet_error;
use exceptions::wrap_parse_error;
use exceptions::wrap_web3_error;
use num_bigint::BigInt;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pythonize::depythonize;
use types::Address;
use types::AnyStr;
use types::Block;
use types::BlockId;
use types::BlockNumberParser;
use types::Bytes;
use types::Eip1559TransactionRequest;
use types::Eip2930TransactionRequest;
use types::FeeHistory;
use types::Filter;
use types::Log;
use types::NameOrAddress;
use types::NodeInfo;
use types::PeerInfo;
use types::SyncingStatus;
use types::Transaction;
use types::TransactionReceipt;
use types::TxpoolContent;
use types::TxpoolInspect;
use types::TxpoolStatus;
use types::H256;
use utils::add_0x_prefix;
use utils::encode_hex;
use utils::from_wei;
use utils::to_hex_i32;
use utils::{to_int, to_wei};
pub mod exceptions;
pub mod types;
use num_bigint::BigUint;
use ruint::aliases::U256;
use types::{HexStr, Number, Primitives, TransactionRequest, TypedTransaction};

#[derive(From, Into)]
#[pyclass(module = "web3_rush")]
pub struct EthHttp(Arc<ethers::providers::Provider<ethers::providers::Http>>);

#[pymethods]
impl EthHttp {
    #[getter]
    pub fn accounts(&self) -> PyResult<Vec<Address>> {
        match block_on(self.0.get_accounts()) {
            Ok(result) => Ok(result.into_iter().map(|r| Address(r)).collect()),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    #[getter]
    pub fn block_number(&self) -> PyResult<u64> {
        match block_on(self.0.get_block_number()) {
            Ok(result) => Ok(result.as_u64()),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    #[getter]
    pub fn chain_id(&self) -> PyResult<BigUint> {
        match block_on(self.0.get_chainid()) {
            Ok(result) => Ok(BigUint::from_str(&result.to_string()).unwrap()),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    #[getter]
    pub fn coinbase(&self) -> PyResult<Address> {
        match block_on(self.0.request("eth_coinbase", ())) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    #[getter]
    pub fn gas_price(&self) -> PyResult<BigUint> {
        match block_on(self.0.get_gas_price()) {
            Ok(result) => Ok(BigUint::from_str(&result.to_string()).unwrap()),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    #[getter]
    pub fn mining(&self) -> PyResult<bool> {
        match block_on(self.0.mining()) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    // #[getter]
    // pub fn max_priority_fee(&self) -> PyResult<bool> {
    //      TODO: implement
    // }

    #[getter]
    pub fn syncing(&self) -> PyResult<SyncingStatus> {
        match block_on(self.0.syncing()) {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn fee_history(
        &self,
        block_count: U256,
        newest_block: BlockNumberParser,
        reward_percentiles: Vec<f64>,
    ) -> PyResult<FeeHistory> {
        match block_on(self.0.fee_history::<U256Original>(
            block_count.into(),
            newest_block.into(),
            &reward_percentiles,
        )) {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn call(&self, tx: PyObject, block: Option<BlockId>) -> PyResult<Bytes> {
        let tx = Python::with_gil(
            |py| match depythonize::<TransactionRequest>(tx.as_ref(py)) {
                Ok(res) => Ok(TypedTransaction::Legacy(res)),
                Err(_) => match depythonize::<Eip1559TransactionRequest>(tx.as_ref(py)) {
                    Ok(res) => Ok(TypedTransaction::Eip1559(res)),
                    Err(_) => match depythonize::<Eip2930TransactionRequest>(tx.as_ref(py)) {
                        Ok(res) => Ok(TypedTransaction::Eip2930(res)),
                        Err(err) => {
                            return Err(err);
                        }
                    },
                },
            },
        )?;
        let block = match block {
            Some(b) => Some(b.into()),
            None => None,
        };
        match block_on(self.0.call(&tx.into(), block)) {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn estimate_gas(&self, tx: PyObject, block: Option<BlockId>) -> PyResult<U256> {
        let tx = Python::with_gil(
            |py| match depythonize::<TransactionRequest>(tx.as_ref(py)) {
                Ok(res) => Ok(TypedTransaction::Legacy(res)),
                Err(_) => match depythonize::<Eip1559TransactionRequest>(tx.as_ref(py)) {
                    Ok(res) => Ok(TypedTransaction::Eip1559(res)),
                    Err(_) => match depythonize::<Eip2930TransactionRequest>(tx.as_ref(py)) {
                        Ok(res) => Ok(TypedTransaction::Eip2930(res)),
                        Err(err) => {
                            return Err(err);
                        }
                    },
                },
            },
        )?;
        let block = match block {
            Some(b) => Some(b.into()),
            None => None,
        };
        match block_on(self.0.estimate_gas(&tx.into(), block)) {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn get_transaction(&self, tx_hash: H256) -> PyResult<Option<Transaction>> {
        match block_on(self.0.get_transaction::<H256>(tx_hash.into())) {
            Ok(result) => match result {
                Some(result) => Ok(Some(result.into())),
                None => Ok(None),
            },
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn get_raw_transaction(&self, tx_hash: H256) -> PyResult<Option<String>> {
        match block_on(self.0.get_transaction::<H256>(tx_hash.into())) {
            Ok(result) => match result {
                Some(result) => Ok(Some(result.rlp().to_string())),
                None => Ok(None),
            },
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn send_transaction(&self, tx: PyObject) -> PyResult<H256> {
        let tx = Python::with_gil(
            |py| match depythonize::<TransactionRequest>(tx.as_ref(py)) {
                Ok(res) => Ok(TypedTransaction::Legacy(res)),
                Err(_) => match depythonize::<Eip1559TransactionRequest>(tx.as_ref(py)) {
                    Ok(res) => Ok(TypedTransaction::Eip1559(res)),
                    Err(_) => match depythonize::<Eip2930TransactionRequest>(tx.as_ref(py)) {
                        Ok(res) => Ok(TypedTransaction::Eip2930(res)),
                        Err(err) => {
                            return Err(err);
                        }
                    },
                },
            },
        )?;
        match block_on(self.0.send_transaction::<TypedTransaction>(tx, None)) {
            Ok(result) => Ok(types::H256(result.tx_hash())),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn send_raw_transaction(&self, tx: HexStr) -> PyResult<H256> {
        match block_on(
            self.0
                .send_raw_transaction(Bytes::from(tx.to_string()).into()),
        ) {
            Ok(result) => Ok(types::H256(result.tx_hash())),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn get_transaction_receipt(&self, tx_hash: H256) -> PyResult<Option<TransactionReceipt>> {
        match block_on(self.0.get_transaction_receipt::<H256>(tx_hash.into())) {
            Ok(result) => match result {
                Some(result) => Ok(Some(result.into())),
                None => Ok(None),
            },
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn get_transaction_count(
        &self,
        address: Address,
        block: Option<BlockId>,
    ) -> PyResult<U256> {
        let block = match block {
            Some(b) => Some(b.into()),
            None => None,
        };
        match block_on(
            self.0
                .get_transaction_count::<AddressOriginal>(address.into(), block.into()),
        ) {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn wait_for_transaction_receipt(
        &self,
        tx_hash: H256,
        timeout: f64,
        poll_latency: f64,
    ) -> PyResult<TransactionReceipt> {
        let wait_start = time::Instant::now();

        while wait_start.elapsed() <= time::Duration::from_secs_f64(timeout) {
            match block_on(
                self.0
                    .get_transaction_receipt::<H256>(tx_hash.clone().into()),
            ) {
                Ok(result) => match result {
                    Some(result) => {
                        return Ok(result.into());
                    }
                    None => {
                        thread::sleep(time::Duration::from_secs_f64(poll_latency));
                    }
                },
                Err(err) => {
                    return Err(wrap_provider_error(err));
                }
            };
        }
        Err(PyTypeError::new_err(format!(
            "Transaction {} is not in the chain after {} seconds",
            tx_hash.0.to_string(),
            timeout
        )))
    }

    pub fn get_block(&self, block_identifier: BlockId) -> PyResult<Block> {
        match block_on(self.0.get_block::<BlockId>(block_identifier.into())) {
            Ok(result) => match result {
                Some(v) => Ok(Block::from(v)),
                None => Err(PyTypeError::new_err(format!(
                    "There is no block with given identifier",
                ))),
            },
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn get_balance(
        &self,
        address: NameOrAddress,
        block_identifier: Option<BlockId>,
    ) -> PyResult<U256> {
        match block_on(self.0.get_balance::<NameOrAddress>(
            address,
            match block_identifier {
                Some(v) => Some(v.into()),
                None => None,
            },
        )) {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn get_code(
        &self,
        address: NameOrAddress,
        block_identifier: Option<BlockId>,
    ) -> PyResult<Bytes> {
        match block_on(self.0.get_code::<NameOrAddress>(
            address,
            match block_identifier {
                Some(v) => Some(v.into()),
                None => None,
            },
        )) {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn get_logs(&self, filter: PyObject) -> PyResult<Vec<Log>> {
        let filter = Python::with_gil(|py| match depythonize::<Filter>(filter.as_ref(py)) {
            Ok(res) => Ok(res),
            Err(err) => Err(err),
        })?;
        match block_on(self.0.get_logs(&filter.into())) {
            Ok(result) => Ok(result.into_iter().map(|r| r.into()).collect()),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }
}

#[derive(From, Into)]
#[pyclass(module = "web3_rush")]
pub struct GethHttp {
    pub miner: GethMinerHttp,
    pub admin: GethAdminHttp,
    // pub personal: GethPersonalHttp,
    pub txpool: GethTxPoolHttp,
}

#[derive(From, Into)]
#[pyclass(module = "web3_rush")]
pub struct GethPersonalHttp(Arc<ethers::providers::Provider<ethers::providers::Http>>);

#[derive(From, Into)]
#[pyclass(module = "web3_rush")]
pub struct GethTxPoolHttp(Arc<ethers::providers::Provider<ethers::providers::Http>>);

#[pymethods]
impl GethTxPoolHttp {
    pub fn inspect(&self) -> PyResult<TxpoolInspect> {
        match block_on(self.0.txpool_inspect()) {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn status(&self) -> PyResult<TxpoolStatus> {
        match block_on(self.0.txpool_status()) {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn content(&self) -> PyResult<TxpoolContent> {
        match block_on(self.0.txpool_content()) {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }
}

#[derive(From, Into)]
#[pyclass(module = "web3_rush")]
pub struct GethAdminHttp(Arc<ethers::providers::Provider<ethers::providers::Http>>);

#[pymethods]
impl GethAdminHttp {
    pub fn datadir(&self) -> PyResult<String> {
        match block_on(self.0.request("admin_datadir", ())) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn add_peer(&self, peer: String) -> PyResult<bool> {
        match block_on(self.0.add_peer(peer)) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn peers(&self) -> PyResult<Vec<PeerInfo>> {
        match block_on(self.0.peers()) {
            Ok(result) => Ok(result.into_iter().map(|v| v.into()).collect()),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn node_info(&self) -> PyResult<NodeInfo> {
        match block_on(self.0.node_info()) {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn start_http(&self) -> PyResult<bool> {
        match block_on(self.0.request("admin_startHTTP", ())) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn start_ws(&self) -> PyResult<bool> {
        match block_on(self.0.request("admin_startWS", ())) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn stop_http(&self) -> PyResult<bool> {
        match block_on(self.0.request("admin_stopHTTP", ())) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn stop_ws(&self) -> PyResult<bool> {
        match block_on(self.0.request("admin_stopWS", ())) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }
}

#[derive(From, Into)]
#[pyclass(module = "web3_rush")]
pub struct GethMinerHttp(Arc<ethers::providers::Provider<ethers::providers::Http>>);

#[pymethods]
impl GethMinerHttp {
    pub fn make_dag(&self, block_number: U256) -> PyResult<bool> {
        match block_on(self.0.request("miner_makeDag", block_number)) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn set_extra(&self, extra_data: H256) -> PyResult<()> {
        match block_on(self.0.request("miner_setExtra", extra_data.0)) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn set_gas_price(&self, gas_price: U256) -> PyResult<()> {
        match block_on(self.0.request("miner_setGasPrice", gas_price)) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn start(&self, num_of_threads: usize) -> PyResult<()> {
        match block_on(self.0.start_mining(Some(num_of_threads))) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn stop(&self) -> PyResult<()> {
        match block_on(self.0.stop_mining()) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn start_auto_dag(&self) -> PyResult<()> {
        match block_on(self.0.request("miner_startAutoDag", ())) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn stop_auto_dag(&self) -> PyResult<()> {
        match block_on(self.0.request("miner_stopAutoDag", ())) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }
}

#[derive(From, Into)]
#[pyclass(module = "web3_rush")]
pub struct NetHttp(Arc<ethers::providers::Provider<ethers::providers::Http>>);

#[pymethods]
impl NetHttp {
    #[getter]
    pub fn version(&self) -> PyResult<String> {
        match block_on(self.0.get_net_version()) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    #[getter]
    pub fn peer_count(&self) -> PyResult<U256> {
        match block_on(self.0.peers()) {
            Ok(result) => Ok(U256Original::from(result.len()).into()),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    #[getter]
    pub fn listening(&self) -> PyResult<bool> {
        match block_on(self.0.request("net_listening", ())) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }
}

#[derive(From, Into, Clone)]
#[pyclass(module = "web3_rush")]
pub struct Web3ApiHttp(Arc<ethers::providers::Provider<ethers::providers::Http>>);

#[pymethods]
impl Web3ApiHttp {
    #[getter]
    pub fn client_version(&self) -> PyResult<String> {
        match block_on(self.0.client_version()) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    #[getter]
    pub fn api(&self) -> PyResult<String> {
        Ok("0.1.0".to_owned())
    }

    #[staticmethod]
    #[allow(non_snake_case)]
    pub fn toHex(
        primitive: Option<Primitives>,
        hexstr: Option<HexStr>,
        text: Option<String>,
    ) -> PyResult<String> {
        Web3ApiHttp::to_hex(primitive, hexstr, text)
    }

    #[staticmethod]
    pub fn to_hex(
        primitive: Option<Primitives>,
        hexstr: Option<HexStr>,
        text: Option<String>,
    ) -> PyResult<String> {
        if let Some(hexstr) = hexstr {
            Ok(add_0x_prefix(hexstr).into())
        } else if let Some(text) = text {
            match encode_hex(AnyStr::Str(text)) {
                Ok(text) => Ok(text.into()),
                Err(err) => Err(PyTypeError::new_err(err)),
            }
        } else {
            match primitive {
                Some(primitive) => match primitive {
                    Primitives::Bool(b) => match b {
                        true => Ok(format!("0x1")),
                        false => Ok(format!("0x0")),
                    },
                    Primitives::String(str) => match encode_hex(AnyStr::Str(str)) {
                        Ok(text) => Ok(text.into()),
                        Err(err) => Err(PyTypeError::new_err(err)),
                    },
                    Primitives::Bytes(bytes) => match encode_hex(AnyStr::Bytes(bytes)) {
                        Ok(text) => Ok(text.into()),
                        Err(err) => Err(PyTypeError::new_err(err)),
                    },
                    Primitives::Int(i) => Ok(to_hex_i32(i as i32)),
                },
                None => Err(PyTypeError::new_err("")),
            }
        }
    }

    #[staticmethod]
    pub fn to_int(
        primitive: Option<Primitives>,
        hexstr: Option<HexStr>,
        text: Option<String>,
    ) -> PyResult<isize> {
        to_int(primitive, hexstr, text)
    }

    #[staticmethod]
    #[allow(non_snake_case)]
    pub fn toInt(
        primitive: Option<Primitives>,
        hexstr: Option<HexStr>,
        text: Option<String>,
    ) -> PyResult<isize> {
        to_int(primitive, hexstr, text)
    }

    #[staticmethod]
    pub fn to_wei(number: Number, unit: String) -> PyResult<BigInt> {
        to_wei(number, unit)
    }

    #[staticmethod]
    #[allow(non_snake_case)]
    pub fn toWei(number: Number, unit: String) -> PyResult<BigInt> {
        to_wei(number, unit)
    }

    #[staticmethod]
    pub fn from_wei(number: Number, unit: String) -> PyResult<f64> {
        from_wei(number, unit)
    }

    #[staticmethod]
    #[allow(non_snake_case)]
    pub fn fromWei(number: Number, unit: String) -> PyResult<f64> {
        from_wei(number, unit)
    }
}

#[pyclass(module = "web3_rush")]
pub struct Web3 {
    client: Arc<ethers::providers::Provider<ethers::providers::Http>>,
}

#[pymethods]
impl Web3 {
    #[new]
    pub fn new(url: String) -> PyResult<Self> {
        let client = ethers::providers::Provider::<ethers::providers::Http>::try_from(url);
        match client {
            Ok(client) => Ok(Web3 {
                client: Arc::new(client),
            }),
            Err(err) => Err(wrap_parse_error(err)),
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
    #[getter]
    pub fn net(&self) -> PyResult<NetHttp> {
        Ok(NetHttp(self.client.clone()))
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
