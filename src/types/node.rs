use ethers::types::U256 as U256Original;

use pyo3::types::PyString;
use pyo3::{pyclass, IntoPy, PyCell, PyObject, Python, ToPyObject};

use serde::{Deserialize, Deserializer, Serialize};

use std::fmt::Debug;
use std::str::FromStr;
use web3_rush_macros::struct_original_mapping;

use ethers::core::utils::ChainConfig as ChainConfigOriginal;
use ethers::core::utils::CliqueConfig as CliqueConfigOriginal;
use ethers::core::utils::EthashConfig as EthashConfigOriginal;
use ethers::providers::admin::EthProtocolInfo as EthProtocolInfoOriginal;
use ethers::providers::admin::NodeInfo as NodeInfoOriginal;
use ethers::providers::admin::PeerInfo as PeerInfoOriginal;
use ethers::providers::admin::PeerNetworkInfo as PeerNetworkInfoOriginal;
use ethers::providers::admin::PeerProtocolInfo as PeerProtocolInfoOriginal;
use ethers::providers::admin::Ports as PortsOriginal;
use ethers::providers::admin::ProtocolInfo as ProtocolInfoOriginal;
use ethers::providers::admin::SnapProtocolInfo as SnapProtocolInfoOriginal;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[struct_original_mapping(PortsOriginal)]
#[pyclass(module = "web3_rush", get_all)]
pub struct Ports {
    /// The node's discovery port.
    pub discovery: u16,

    /// The node's listener port.
    pub listener: u16,
}

/// Empty consensus configuration for proof-of-work networks.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[struct_original_mapping(EthashConfigOriginal)]
#[pyclass(module = "web3_rush", get_all)]
pub struct EthashConfig {}

/// Consensus configuration for Clique.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[struct_original_mapping(CliqueConfigOriginal)]
#[pyclass(module = "web3_rush", get_all)]
pub struct CliqueConfig {
    /// Number of seconds between blocks to enforce.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<u64>,

    /// Epoch length to reset votes and checkpoints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epoch: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Default, PartialEq, Eq)]
#[serde(default)]
#[struct_original_mapping(ChainConfigOriginal)]
#[pyclass(module = "web3_rush", get_all)]
pub struct ChainConfig {
    /// The network's chain ID.
    pub chain_id: u64,

    /// The homestead switch block (None = no fork, 0 = already homestead).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homestead_block: Option<u64>,

    /// The DAO fork switch block (None = no fork).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dao_fork_block: Option<u64>,

    /// Whether or not the node supports the DAO hard-fork.
    pub dao_fork_support: bool,

    /// The EIP-150 hard fork block (None = no fork).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip150_block: Option<u64>,

    /// The EIP-150 hard fork hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip150_hash: Option<H256>,

    /// The EIP-155 hard fork block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip155_block: Option<u64>,

    /// The EIP-158 hard fork block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip158_block: Option<u64>,

    /// The Byzantium hard fork block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byzantium_block: Option<u64>,

    /// The Constantinople hard fork block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constantinople_block: Option<u64>,

    /// The Petersburg hard fork block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub petersburg_block: Option<u64>,

    /// The Istanbul hard fork block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub istanbul_block: Option<u64>,

    /// The Muir Glacier hard fork block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muir_glacier_block: Option<u64>,

    /// The Berlin hard fork block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub berlin_block: Option<u64>,

    /// The London hard fork block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub london_block: Option<u64>,

    /// The Arrow Glacier hard fork block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrow_glacier_block: Option<u64>,

    /// The Gray Glacier hard fork block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gray_glacier_block: Option<u64>,

    /// Virtual fork after the merge to use as a network splitter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_netsplit_block: Option<u64>,

    /// Shanghai switch time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shanghai_time: Option<u64>,

    /// Cancun switch time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancun_time: Option<u64>,

    /// Total difficulty reached that triggers the merge consensus upgrade.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_total_difficulty: Option<U256>,

    /// A flag specifying that the network already passed the terminal total difficulty. Its
    /// purpose is to disable legacy sync without having seen the TTD locally.
    pub terminal_total_difficulty_passed: bool,

    /// Ethash parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethash: Option<EthashConfig>,

    /// Clique parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clique: Option<CliqueConfig>,
}

/// Represents a short summary of the `eth` sub-protocol metadata known about the host peer.
///
/// See [geth's `NodeInfo`
/// struct](https://github.com/ethereum/go-ethereum/blob/c2e0abce2eedc1ba2a1b32c46fd07ef18a25354a/eth/protocols/eth/handler.go#L129)
/// for how these fields are determined.
#[derive(Clone, Debug, Deserialize)]
#[struct_original_mapping(EthProtocolInfoOriginal)]
#[pyclass(module = "web3_rush", get_all)]
pub struct EthProtocolInfo {
    /// The eth network version.
    pub network: u64,

    /// The total difficulty of the host's blockchain.
    pub difficulty: U256,

    /// The Keccak hash of the host's genesis block.
    pub genesis: H256,

    /// The chain configuration for the host's fork rules.
    pub config: ChainConfig,

    /// The hash of the host's best known block.
    pub head: H256,
}

/// Represents a short summary of the host's `snap` sub-protocol metadata.
///
/// This is just an empty struct, because [geth's internal representation is
/// empty](https://github.com/ethereum/go-ethereum/blob/c2e0abce2eedc1ba2a1b32c46fd07ef18a25354a/eth/protocols/snap/handler.go#L571-L576).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[struct_original_mapping(SnapProtocolInfoOriginal)]
#[pyclass(module = "web3_rush", get_all)]
pub struct SnapProtocolInfo {}

/// Represents protocols that the connected RPC node supports.
///
/// This contains protocol information reported by the connected RPC node.
#[derive(Clone, Debug, Deserialize)]
#[struct_original_mapping(ProtocolInfoOriginal)]
#[pyclass(module = "web3_rush", get_all)]
pub struct ProtocolInfo {
    /// Details about the node's supported eth protocol. `None` if unsupported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eth: Option<EthProtocolInfo>,

    /// Details about the node's supported snap protocol. `None` if unsupported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snap: Option<SnapProtocolInfo>,
}

/// This includes general information about a running node, spanning networking and protocol
/// details.
#[derive(Clone, Debug, Deserialize)]
#[pyclass(module = "web3_rush", get_all)]
pub struct NodeInfo {
    /// The node's private key.
    pub id: H256,

    /// The node's user agent, containing a client name, version, OS, and other metadata.
    pub name: String,

    /// The enode URL of the connected node.
    pub enode: String,

    /// The [ENR](https://eips.ethereum.org/EIPS/eip-778) of the running client.
    pub enr: String,

    /// The IP address of the connected node.
    pub ip: String,

    /// The node's listening ports.
    pub ports: Ports,

    /// The node's listening address.
    #[serde(rename = "listenAddr")]
    pub listen_addr: String,

    /// The protocols that the node supports, with protocol metadata.
    pub protocols: ProtocolInfo,
}

impl From<NodeInfoOriginal> for NodeInfo {
    fn from(value: NodeInfoOriginal) -> Self {
        NodeInfo {
            id: value.id.into(),
            name: value.name.into(),
            enode: value.enode.into(),
            enr: value.enr.to_string(),
            ip: value.ip.to_string(),
            ports: value.ports.into(),
            listen_addr: value.listen_addr.into(),
            protocols: value.protocols.into(),
        }
    }
}

/// Represents networking related information about the peer, including details about whether or
/// not it is inbound, trusted, or static.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[pyclass(module = "web3_rush", get_all)]
pub struct PeerNetworkInfo {
    /// The local endpoint of the TCP connection.
    pub local_address: String,

    /// The remote endpoint of the TCP connection.
    pub remote_address: String,

    /// Whether or not the peer is inbound.
    pub inbound: bool,

    /// Whether or not the peer is trusted.
    pub trusted: bool,

    /// Whether or not the peer is a static peer.
    #[serde(rename = "static")]
    pub static_node: bool,
}

impl From<PeerNetworkInfoOriginal> for PeerNetworkInfo {
    fn from(value: PeerNetworkInfoOriginal) -> Self {
        PeerNetworkInfo {
            local_address: value.local_address.to_string(),
            remote_address: value.remote_address.to_string(),
            inbound: value.inbound,
            trusted: value.trusted,
            static_node: value.static_node,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[pyclass(module = "web3_rush", get_all)]
pub struct PeerProtocolInfo {
    /// Details about the peer's supported eth protocol. `None` if unsupported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eth: Option<EthPeerInfo>,

    /// Details about the peer's supported snap protocol. `None` if unsupported
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snap: Option<SnapPeerInfo>,
}

impl From<PeerProtocolInfoOriginal> for PeerProtocolInfo {
    fn from(value: PeerProtocolInfoOriginal) -> Self {
        PeerProtocolInfo {
            eth: match value.eth {
                Some(v) => Some(v.into()),
                None => None,
            },
            snap: match value.snap {
                Some(v) => Some(v.into()),
                None => None,
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "web3_rush", get_all)]
pub struct PeerInfo {
    /// The peer's ENR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enr: Option<String>,

    /// The peer's enode URL.
    pub enode: String,

    /// The peer's enode ID.
    pub id: String,

    /// The peer's name.
    pub name: String,

    /// The peer's capabilities.
    pub caps: Vec<String>,

    /// Networking information about the peer.
    pub network: PeerNetworkInfo,

    /// The protocols that the peer supports, with protocol metadata.
    pub protocols: PeerProtocolInfo,
}

impl From<PeerInfoOriginal> for PeerInfo {
    fn from(value: PeerInfoOriginal) -> Self {
        PeerInfo {
            enr: match value.enr {
                Some(v) => Some(v.to_string()),
                None => None,
            },
            enode: value.enode,
            id: value.id,
            name: value.name,
            caps: value.caps,
            network: value.network.into(),
            protocols: value.protocols.into(),
        }
    }
}

fn deser_handshake<'de, D>(deserializer: D) -> Result<(), D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s == "handshake" {
        Ok(())
    } else {
        Err(serde::de::Error::custom(
            "expected \"handshake\" if protocol info did not appear in the response",
        ))
    }
}

fn ser_handshake<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str("handshake")
}

use ethers::providers::admin::EthInfo as EthInfoOriginal;
use ethers::providers::admin::EthPeerInfo as EthPeerInfoOriginal;
use ethers::providers::admin::SnapInfo as SnapInfoOriginal;
use ethers::providers::admin::SnapPeerInfo as SnapPeerInfoOriginal;

use super::bigint::U256;
use super::str::H256;

/// Can contain either eth protocol info or a string "handshake", which geth uses if the peer is
/// still completing the handshake for the protocol.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum EthPeerInfo {
    /// The `eth` sub-protocol metadata known about the host peer.
    Info(Box<EthInfo>),

    /// The string "handshake" if the peer is still completing the handshake for the protocol.
    #[serde(deserialize_with = "deser_handshake", serialize_with = "ser_handshake")]
    Handshake,
}

impl From<EthPeerInfoOriginal> for EthPeerInfo {
    fn from(value: EthPeerInfoOriginal) -> Self {
        match value {
            EthPeerInfoOriginal::Info(i) => EthPeerInfo::Info(Box::new((*i).into())),
            EthPeerInfoOriginal::Handshake => EthPeerInfo::Handshake,
        }
    }
}

impl ToPyObject for EthPeerInfo {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match self {
            EthPeerInfo::Info(i) => PyCell::new(py, *i.to_owned()).unwrap().into(),
            EthPeerInfo::Handshake => PyString::new(py, "handshake").into(),
        }
    }
}

impl IntoPy<PyObject> for EthPeerInfo {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

pub fn from_int_or_hex<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum IntOrHex {
        Int(serde_json::Number),
        Hex(String),
    }

    match IntOrHex::deserialize(deserializer)? {
        IntOrHex::Hex(s) => Ok(U256Original::from_str(s.as_str()).unwrap().into()),
        IntOrHex::Int(n) => Ok(U256Original::from_dec_str(&n.to_string()).unwrap().into()),
    }
}

/// Represents a short summary of the `eth` sub-protocol metadata known about a connected peer
///
/// See [geth's `ethPeerInfo`
/// struct](https://github.com/ethereum/go-ethereum/blob/53d1ae096ac0515173e17f0f81a553e5f39027f7/eth/peer.go#L28)
/// for how these fields are determined.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[pyclass(module = "web3_rush", get_all)]
#[struct_original_mapping(EthInfoOriginal)]
pub struct EthInfo {
    /// The negotiated eth version.
    pub version: u64,

    /// The total difficulty of the peer's blockchain.
    #[serde(deserialize_with = "from_int_or_hex")]
    pub difficulty: U256,

    /// The hash of the peer's best known block.
    pub head: H256,
}

/// Can contain either snap protocol info or a string "handshake", which geth uses if the peer is
/// still completing the handshake for the protocol.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum SnapPeerInfo {
    /// The `snap` sub-protocol metadata known about the host peer.
    Info(SnapInfo),

    /// The string "handshake" if the peer is still completing the handshake for the protocol.
    #[serde(deserialize_with = "deser_handshake", serialize_with = "ser_handshake")]
    Handshake,
}

impl ToPyObject for SnapPeerInfo {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match self {
            SnapPeerInfo::Info(i) => PyCell::new(py, i.to_owned()).unwrap().into(),
            SnapPeerInfo::Handshake => PyString::new(py, "handshake").into(),
        }
    }
}

impl IntoPy<PyObject> for SnapPeerInfo {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

impl From<SnapPeerInfoOriginal> for SnapPeerInfo {
    fn from(value: SnapPeerInfoOriginal) -> Self {
        match value {
            SnapPeerInfoOriginal::Info(i) => SnapPeerInfo::Info(i.into()),
            SnapPeerInfoOriginal::Handshake => SnapPeerInfo::Handshake,
        }
    }
}

/// Represents a short summary of the `snap` sub-protocol metadata known about a connected peer.
///
/// See [geth's `snapPeerInfo`
/// struct](https://github.com/ethereum/go-ethereum/blob/53d1ae096ac0515173e17f0f81a553e5f39027f7/eth/peer.go#L53)
/// for how these fields are determined.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[pyclass(module = "web3_rush", get_all)]
#[struct_original_mapping(SnapInfoOriginal)]
pub struct SnapInfo {
    /// The negotiated snap version.
    pub version: u64,
}
