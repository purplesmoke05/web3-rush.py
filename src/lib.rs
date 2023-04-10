pub mod utils;

use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time;

use account::EthKeystore;
use account::LocalAccount;
use account::SignedTransaction;
use async_std::task::block_on;
use derive_more::From;
use derive_more::Into;
use eth_keystore::EthKeystore as EthKeystoreOriginal;
use ethers::prelude::rand::thread_rng;
use ethers::providers::Middleware;
use ethers::signers::LocalWallet;
use ethers::signers::Signer;
use ethers::types::Address as AddressOriginal;
use ethers::types::Bytes as BytesOriginal;
use ethers::types::Signature;
use ethers::types::U256 as U256Original;
use ethers::utils::rlp;
use ethers::utils::rlp::Decodable;
use ethers::utils::to_checksum;
use ethers_core::types::transaction::eip2718::TypedTransaction as TypedTransactionOriginal;
use ethers_core::types::Transaction as TransactionOriginal;
use exceptions::wrap_from_hex_error;
use exceptions::wrap_from_signature_error;
use exceptions::wrap_from_wallet_error;

use exceptions::wrap_parse_error;
use exceptions::wrap_provider_error;

use pyo3::exceptions::PyTypeError;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use pythonize::depythonize;
use types::address::Address;
use types::address::NameOrAddress;
use types::block::Block;
use types::fee::FeeHistory;
use types::log::Filter;
use types::log::Log;
use types::node::NodeInfo;
use types::node::PeerInfo;
use types::primitives::BlockId;
use types::primitives::BlockNumberParser;
use types::str::AnyStr;
use types::str::Bytes;
use types::str::HexStr;
use types::str::H256;
use types::syncing::SyncingStatus;
use types::transaction::Eip1559TransactionRequest;
use types::transaction::Eip2930TransactionRequest;
use types::transaction::GotTransaction;
use types::transaction::Transaction;
use types::transaction::TransactionReceipt;
use types::transaction::TransactionRequest;
use types::transaction::TypedTransaction;
use types::txpool::TxpoolContent;
use types::txpool::TxpoolInspect;
use types::txpool::TxpoolStatus;

use utils::decrypt_key;
use utils::encode_hex;
use utils::encrypt_key;

pub mod account;
pub mod exceptions;
pub mod types;
use num_bigint::BigUint;
use ruint::aliases::U256;

#[derive(From, Into)]
#[pyclass(module = "web3_rush")]
pub struct EthHttp(Arc<ethers::providers::Provider<ethers::providers::Http>>);

#[derive(From, Into)]
#[pyclass(module = "web3_rush.web3")]
pub struct Account {}

#[pymethods]
impl Account {
    #[new]
    pub fn new() -> Self {
        Account {}
    }

    #[staticmethod]
    pub fn create() -> PyResult<LocalAccount> {
        let wallet = LocalWallet::new(&mut thread_rng());
        Ok(LocalAccount(wallet))
    }

    #[staticmethod]
    pub fn from_key(private_key: &str) -> PyResult<LocalAccount> {
        match private_key.parse::<LocalWallet>() {
            Ok(wallet) => Ok(LocalAccount(wallet)),
            Err(err) => Err(wrap_from_wallet_error(err)),
        }
    }

    #[staticmethod]
    pub fn sign_message(message: &str, private_key: &str) -> PyResult<String> {
        let message = message.as_bytes();

        match block_on(
            Account::from_key(private_key)
                .expect("Invalid private key")
                .0
                .sign_message(message),
        ) {
            Ok(signature) => Ok(encode_hex(AnyStr::Str(signature.to_string()))?.into()),
            Err(err) => Err(wrap_from_wallet_error(err)),
        }
    }

    #[staticmethod]
    pub fn recover_transaction(tx: HexStr) -> PyResult<String> {
        let typed_tx_hex = hex::decode(tx.0.trim_start_matches("0x")).unwrap();
        let tx_rlp = rlp::Rlp::new(typed_tx_hex.as_slice());
        let actual_tx = TransactionOriginal::decode(&tx_rlp).unwrap();
        let signature = Signature {
            r: actual_tx.r,
            s: actual_tx.s,
            v: actual_tx.v.as_u64(),
        };
        let typed_tx: TypedTransactionOriginal = (&actual_tx).into();
        match signature.recover(typed_tx.sighash()) {
            Ok(result) => Ok(result.to_string()),
            Err(err) => Err(wrap_from_signature_error(err)),
        }
    }

    #[staticmethod]
    pub fn sign_transaction(tx: PyObject, private_key: &str) -> PyResult<SignedTransaction> {
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
        let org_tx: TypedTransactionOriginal = tx.into();

        match Account::from_key(private_key)
            .expect("Invalid private key")
            .0
            .sign_transaction_sync(&org_tx)
        {
            Ok(signature) => Ok(SignedTransaction {
                raw_transaction: BytesOriginal::from_iter(org_tx.data().unwrap()).into(),
                hash: BytesOriginal::from_iter(org_tx.hash(&signature).0).into(),
                r: signature.r.into(),
                s: signature.s.into(),
                v: signature.v,
            }),
            Err(err) => Err(wrap_from_wallet_error(err)),
        }
    }

    #[staticmethod]
    pub fn decrypt(keyfile_json: PyObject, password: &str) -> PyResult<String> {
        let keyfile_json =
            Python::with_gil(
                |py| match depythonize::<EthKeystore>(keyfile_json.as_ref(py)) {
                    Ok(res) => Ok(res),
                    Err(err) => return Err(err),
                },
            )?;

        let keystore: EthKeystoreOriginal = keyfile_json.into();
        match decrypt_key(keystore, password) {
            Ok(res) => Ok(String::from_utf8(res).unwrap()),
            Err(err) => Err(PyValueError::new_err(err.to_string())),
        }
    }

    #[staticmethod]
    pub fn encrypt(private_key: &str, password: &str) -> PyResult<Py<PyAny>> {
        match encrypt_key(&mut rand::thread_rng(), private_key, password) {
            Ok(res) => {
                let res = Python::with_gil(|py| match pythonize::pythonize(py, &res) {
                    Ok(res) => Ok(res),
                    Err(err) => Err(err),
                });
                match res {
                    Ok(res) => Ok(res),
                    Err(err) => Err(PyValueError::new_err(err.to_string())),
                }
            }
            Err(err) => Err(PyValueError::new_err(err.to_string())),
        }
    }
}

#[pymethods]
impl EthHttp {
    #[getter]
    pub fn account(&self) -> Account {
        Account {}
    }

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

    pub fn get_transaction(&self, tx_hash: H256) -> PyResult<Option<GotTransaction>> {
        match block_on(self.0.get_transaction::<H256>(tx_hash.into())) {
            Ok(result) => match result {
                Some(result) => {
                    let tx: Transaction = result.into();
                    return Ok(Some(tx.into()));
                }
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
            Ok(result) => Ok(H256(result.tx_hash())),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    pub fn send_raw_transaction(&self, tx: HexStr) -> PyResult<H256> {
        match block_on(
            self.0
                .send_raw_transaction(Bytes::from(tx.to_string()).into()),
        ) {
            Ok(result) => Ok(H256(result.tx_hash())),
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

#[pymethods]
impl GethHttp {
    pub fn datadir(&self) -> PyResult<String> {
        self.admin.datadir()
    }
    pub fn add_peer(&self, peer: String) -> PyResult<bool> {
        self.admin.add_peer(peer)
    }
    pub fn peers(&self) -> PyResult<Vec<PeerInfo>> {
        self.admin.peers()
    }
    pub fn node_info(&self) -> PyResult<NodeInfo> {
        self.admin.node_info()
    }
    pub fn start_http(&self) -> PyResult<bool> {
        self.admin.start_http()
    }
    pub fn start_ws(&self) -> PyResult<bool> {
        self.admin.start_ws()
    }
    pub fn stop_http(&self) -> PyResult<bool> {
        self.admin.start_http()
    }
    pub fn stop_ws(&self) -> PyResult<bool> {
        self.admin.stop_ws()
    }
    pub fn make_dag(&self, block_number: U256) -> PyResult<bool> {
        self.miner.make_dag(block_number)
    }
    pub fn set_extra(&self, extra_data: H256) -> PyResult<()> {
        self.miner.set_extra(extra_data)
    }
    pub fn set_gas_price(&self, gas_price: U256) -> PyResult<()> {
        self.miner.set_gas_price(gas_price)
    }
    pub fn start(&self, num_of_threads: usize) -> PyResult<()> {
        self.miner.start(num_of_threads)
    }
    pub fn stop(&self) -> PyResult<()> {
        self.miner.stop()
    }
    pub fn start_auto_dag(&self) -> PyResult<()> {
        self.miner.start_auto_dag()
    }
    pub fn stop_auto_dag(&self) -> PyResult<()> {
        self.miner.stop_auto_dag()
    }
    pub fn inspect(&self) -> PyResult<TxpoolInspect> {
        self.txpool.inspect()
    }
    pub fn status(&self) -> PyResult<TxpoolStatus> {
        self.txpool.status()
    }
    pub fn content(&self) -> PyResult<TxpoolContent> {
        self.txpool.content()
    }
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
    pub fn eth(&self) -> PyResult<EthHttp> {
        Ok(EthHttp(self.client.clone()))
    }

    #[getter]
    pub fn geth(&self) -> PyResult<GethHttp> {
        Ok(GethHttp {
            miner: GethMinerHttp(self.client.clone()),
            admin: GethAdminHttp(self.client.clone()),
            txpool: GethTxPoolHttp(self.client.clone()),
        })
    }

    #[getter]
    pub fn net(&self) -> PyResult<NetHttp> {
        Ok(NetHttp(self.client.clone()))
    }

    #[getter]
    pub fn client_version(&self) -> PyResult<String> {
        match block_on(self.client.client_version()) {
            Ok(result) => Ok(result),
            Err(err) => Err(wrap_provider_error(err)),
        }
    }

    #[getter]
    #[inline]
    pub fn api(&self) -> PyResult<String> {
        Ok("0.1.0".to_string())
    }

    #[staticmethod]
    pub fn is_address(address: String) -> PyResult<bool> {
        match ethers::types::Address::from_str(&address) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    #[staticmethod]
    pub fn is_checksum_address(address: String) -> PyResult<bool> {
        match ethers::types::Address::from_str(&address) {
            Ok(addr) => Ok(to_checksum(&addr, None) == address),
            Err(_) => Ok(false),
        }
    }

    #[staticmethod]
    pub fn to_checksum_address(address: String) -> PyResult<String> {
        match ethers::types::Address::from_str(&address) {
            Ok(addr) => Ok(to_checksum(&addr, None)),
            Err(err) => Err(wrap_from_hex_error(err)),
        }
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn web3_rush(_py: Python, m: &PyModule) -> PyResult<()> {
    let module = PyModule::new(_py, "module")?;
    module.add_class::<Web3>()?;
    module.add_class::<Account>()?;
    m.add_submodule(module)?;
    exceptions::init_module(_py, m, m)?;
    Ok(())
}
