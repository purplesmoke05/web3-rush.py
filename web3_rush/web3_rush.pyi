from typing import Dict, Optional, Union, TypedDict

class Web3:
    web3: Web3Module
    eth: EthModule
    net: NetModule

# ***************************************
# Web3 Module
# ***************************************

class Web3Module:
    client_version: str
    api: str
    @staticmethod
    def to_hex(
        primitive: Union[bytes, int, bool], hexstr: Optional[str], text: Optional[str]
    ) -> str: ...
    @staticmethod
    def toHex(
        primitive: Union[bytes, int, bool], hexstr: Optional[str], text: Optional[str]
    ) -> str: ...
    @staticmethod
    def to_int(
        primitive: Union[bytes, int, bool], hexstr: Optional[str], text: Optional[str]
    ) -> int: ...
    @staticmethod
    def toInt(
        primitive: Union[bytes, int, bool], hexstr: Optional[str], text: Optional[str]
    ) -> int: ...
    @staticmethod
    def to_wei(number: Union[int, float, str], unit: str) -> int: ...
    @staticmethod
    def toWei(number: Union[int, float, str], unit: str) -> int: ...
    @staticmethod
    def from_wei(number: Union[int, float, str], unit: str) -> float: ...
    @staticmethod
    def fromWei(number: Union[int, float, str], unit: str) -> float: ...

# ***************************************
# Net Module
# ***************************************

class NetModule:
    version: str
    peer_count: int
    listening: bool

# ***************************************
# Eth Module
# ***************************************

class SyncingStatus:
    current_block: int
    highest_block: int
    known_states: Optional[int]
    pulled_states: Optional[int]
    starting_block: int

TransactionRequest = TypedDict(
    "TransactionRequest",
    {
        "from": Optional[str],
        "to": Optional[str],
        "gas": Optional[int],
        "gas_price": Optional[int],
        "value": Optional[int],
        "data": Optional[str],
        "nonce": Optional[int],
        "chain_id": Optional[int],
    },
    total=False,
)

class AccessListItem(TypedDict):
    address: str
    storage_keys: list[str]

class Filter(TypedDict, total=False):
    block_from: int
    block_to: int
    address: Union[list[str], str]
    topics: list[
        Optional[list[str, str]],
        Optional[list[str, str]],
        Optional[list[str, str]],
        Optional[list[str, str]],
    ]

Eip1559TransactionRequest = TypedDict(
    "Eip1559TransactionRequest",
    {
        "from": Optional[str],
        "to": Optional[str],
        "gas": Optional[int],
        "value": Optional[int],
        "data": Optional[str],
        "nonce": Optional[int],
        "access_list": list[AccessListItem],
        "max_fee_per_gas": Optional[int],
        "chain_id": Optional[int],
    },
    total=False,
)

Eip2930TransactionRequest = TypedDict(
    "Eip2930TransactionRequest",
    {
        "from": Optional[str],
        "to": Optional[str],
        "gas": Optional[int],
        "gas_price": Optional[int],
        "value": Optional[int],
        "data": Optional[str],
        "nonce": Optional[int],
        "chain_id": Optional[int],
        "AccessList": list[AccessListItem],
    },
    total=False,
)

class Transaction:
    hash: str
    nonce: int
    block_hash: str
    block_number: int
    transaction_index: int
    from_: str
    to: Optional[str]
    value: int
    gas_price: Optional[int]
    gas: int
    input: str
    v: int
    r: int
    s: int
    transaction_type: Optional[int]
    access_list: Optional[list[AccessListItem]]
    max_priority_fee_per_gas: Optional[int]
    max_fee_per_gas: Optional[int]
    chain_id: Optional[int]

class Log:
    address: str
    topics: list[str]
    data: str
    block_hash: Optional[str]
    block_number: Optional[int]
    transaction_hash: Optional[str]
    transaction_index: Optional[int]
    log_index: Optional[int]
    transaction_log_index: Optional[str]
    log_type: Optional[str]
    removed: Optional[bool]

Bloom = str

class TransactionReceipt:
    transaction_hash: str
    transaction_index: int
    block_hash: Optional[str]
    block_number: Optional[int]
    from_: str
    to: Optional[str]
    cumulative_gas_used: int
    gas_used: Optional[int]
    contract_address: Optional[str]
    logs: list[Log]
    status: Optional[int]
    root: Optional[str]
    logs_bloom: Bloom
    transaction_type: Optional[int]
    effective_gas_price: Optional[int]
    other: dict

class Block:
    hash: Optional[str]
    parent_hash: Optional[str]
    uncles_hash: str
    author: Optional[str]
    state_root: str
    transactions_root: str
    receipts_root: str
    number: Optional[int]
    gas_used: int
    gas_limit: int
    extra_data: str
    logs_bloom: Optional[Bloom]
    timestamp: int
    difficulty: int
    total_difficulty: Optional[int]
    seal_fields: list[str]
    uncles: list[str]
    transactions: list[str]
    size: Optional[int]
    mix_hash: Optional[str]
    nonce: Optional[str]
    base_fee_per_gas: Optional[int]
    other: dict

class FeeHistory:
    base_fee_per_gas: list[int]
    gas_used_ratio: list[float]
    oldest_block: int
    reward: list[list[int]]

class SignedTransaction:
    raw_transaction: str
    hash: str
    r: int
    s: int
    v: int

class LocalAccount:
    address: str
    chain_id: int
    def sign_transaction(self, tx: str) -> SignedTransaction: ...
    def sign_message(self, message: str) -> str: ...

class Account:
    @staticmethod
    def create() -> LocalAccount: ...
    @staticmethod
    def from_key(private_key: str) -> LocalAccount: ...
    @staticmethod
    def sign_message(message: str, private_key: str) -> str: ...
    @staticmethod
    def recover_transaction(tx: str) -> str: ...
    @staticmethod
    def sign_transaction(tx: str, private_key: str) -> SignedTransaction: ...
    @staticmethod
    def decrypt(keyfile_json: dict, password: str) -> str: ...
    @staticmethod
    def encrypt(private_key: str, password: str) -> str: ...

class EthModule:
    account: Account
    accounts: list[str]
    block_number: int
    chain_id: int
    coinbase: str
    gas_price: int
    mining: bool
    syncing: SyncingStatus | bool

    def fee_history(
        self, block_count: int, newest_block: Union[int, str], reward_percentiles: list[float]
    ) -> FeeHistory: ...
    def call(
        self,
        tx: Union[TransactionRequest, Eip1559TransactionRequest, Eip2930TransactionRequest],
        block: Union[int, str],
    ) -> str: ...
    def estimate_gas(
        self,
        tx: Union[TransactionRequest, Eip1559TransactionRequest, Eip2930TransactionRequest],
        block: Union[int, str],
    ) -> int: ...
    def get_transaction(self, tx_hash: str) -> Optional[Transaction | dict]: ...
    def get_raw_transaction(self, tx_hash: str) -> Optional[str]: ...
    def send_transaction(
        self, tx: Union[TransactionRequest, Eip1559TransactionRequest, Eip2930TransactionRequest]
    ) -> str: ...
    def send_raw_transaction(self, tx: str) -> str: ...
    def get_transaction_receipt(self, tx: str) -> Optional[TransactionReceipt]: ...
    def get_transaction_count(self, address: str, block: Optional[Union[str, int]]) -> int: ...
    def wait_for_transaction_receipt(
        self, tx_hash: str, timeout: float, poll_latency: float
    ) -> TransactionReceipt: ...
    def get_block(self, block_identifier: Union[str, int]) -> Block: ...
    def get_balance(self, address: str, block_identifier: Optional[Union[str, int]]) -> int: ...
    def get_code(self, address: str, block_identifier: Optional[Union[str, int]]) -> str: ...
    def get_logs(self, filter: Filter) -> list[Log]: ...

# ***************************************
# Geth Module
# ***************************************

class PeerNetworkInfo:
    local_address: str
    remote_address: str
    inbound: bool
    trusted: bool
    static: bool

class EthInfo:
    version: int
    difficulty: int
    head: str

class SnapInfo:
    version: int

class PeerProtocolInfo:
    eth: Optional[EthInfo]
    snap: Optional[SnapInfo]

class PeerInfo:
    enr: Optional[str]
    enode: str
    id: str
    name: str
    caps: list[str]
    network: PeerNetworkInfo
    protocols: PeerProtocolInfo

class Ports:
    discovery: int
    listener: int

class ProtocolInfo:
    eth: Optional[EthProtocolInfo]
    snap: Optional[SnapProtocolInfo]

class ChainConfig:
    chain_id: str
    homestead_block: Optional[int]
    dao_fork_block: Optional[int]
    dao_fork_support: bool
    eip150_block: Optional[int]
    eip150_hash: Optional[str]
    eip155_block: Optional[int]
    eip158_block: Optional[int]
    byzantium_block: Optional[int]
    constantinople_block: Optional[int]
    petersburg_block: Optional[int]
    istanbul_block: Optional[int]
    muir_glacier_block: Optional[int]
    berlin_block: Optional[int]
    london_block: Optional[int]
    arrow_glacier_block: Optional[int]
    gray_glacier_block: Optional[int]
    merge_netsplit_block: Optional[int]
    shanghai_time: Optional[int]
    cancun_time: Optional[int]
    terminal_total_difficulty: Optional[int]
    terminal_total_difficulty_passed: bool

class EthProtocolInfo:
    network: int
    difficulty: int
    genesis: str
    config: ChainConfig
    head: str

class NodeInfo:
    id: str
    name: str
    enode: str
    enr: str
    ip: str
    ports: Ports
    listen_addr: str
    protocols: ProtocolInfo

class TxpoolInspectSummary:
    to: Optional[str]
    value: int
    gas: int
    gas_price: int

class TxpoolInspect:
    pending: Dict[str, Dict[str, TxpoolInspectSummary]]
    queued: Dict[str, Dict[str, TxpoolInspectSummary]]

class TxpoolContent:
    pending: Dict[str, Dict[str, Transaction]]
    queued: Dict[str, Dict[str, Transaction]]

class TxpoolStatus:
    pending: int
    queued: int

class GethModule:
    # Miner Sub Module
    def make_dag(self, block_number: int) -> bool: ...
    def set_extra(self, extra_data: str): ...
    def set_gas_price(self, gas_price: int): ...
    def start(self, num_of_threads: int): ...
    def stop(self): ...
    def start_auto_dag(self): ...
    def stop_auto_dag(self): ...
    # Admin Sub Module
    def datadir(self) -> str: ...
    def add_peer(self, peer: str) -> bool: ...
    def peers(self) -> list[PeerInfo]: ...
    def node_info(self) -> list[NodeInfo]: ...
    def start_http(self) -> bool: ...
    def start_ws(self) -> bool: ...
    def stop_http(self) -> bool: ...
    def stop_ws(self) -> bool: ...
    # Miner Sub Module
    def make_dag(self, block_number: int) -> bool: ...
    def set_extra(self, extra_data: str): ...
    def set_gas_price(self, gas_price: int): ...
    def start(self, num_of_threads: int): ...
    def stop(self): ...
    def start_auto_dag(self): ...
    def stop_auto_dag(self): ...
    def inspect(self) -> TxpoolInspect: ...
    def status(self) -> TxpoolStatus: ...
    def content(self) -> TxpoolContent: ...
