use pyo3::types::PyBool;
use pyo3::{pyclass, IntoPy, PyCell, PyObject, Python, ToPyObject};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[pyclass(module = "web3_rush", get_all)]
pub struct SyncProgress {
    pub current_block: U64,
    pub highest_block: U64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub known_states: Option<U64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pulled_states: Option<U64>,
    pub starting_block: U64,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum SyncingStatus {
    IsFalse,
    IsSyncing(SyncProgress),
}

use ethers::types::SyncProgress as SyncProgressOriginal;
use ethers::types::SyncingStatus as SyncingStatusOriginal;

use super::bigint::U64;

impl Into<SyncProgressOriginal> for SyncProgress {
    fn into(self) -> SyncProgressOriginal {
        SyncProgressOriginal {
            current_block: self.current_block.into(),
            highest_block: self.highest_block.into(),
            starting_block: self.starting_block.into(),
            pulled_states: match self.pulled_states {
                Some(v) => Some(v.into()),
                None => None,
            },
            known_states: match self.known_states {
                Some(v) => Some(v.into()),
                None => None,
            },
            healed_bytecode_bytes: None,
            healed_bytecodes: None,
            healed_trienode_bytes: None,
            healed_trienodes: None,
            healing_bytecode: None,
            healing_trienodes: None,
            synced_account_bytes: None,
            synced_accounts: None,
            synced_bytecode_bytes: None,
            synced_bytecodes: None,
            synced_storage: None,
            synced_storage_bytes: None,
        }
    }
}

impl From<SyncProgressOriginal> for SyncProgress {
    fn from(value: SyncProgressOriginal) -> Self {
        SyncProgress {
            current_block: value.current_block.into(),
            highest_block: value.highest_block.into(),
            known_states: match value.known_states {
                Some(v) => Some(v.into()),
                None => None,
            },
            pulled_states: match value.pulled_states {
                Some(v) => Some(v.into()),
                None => None,
            },
            starting_block: value.starting_block.into(),
        }
    }
}

impl Into<SyncingStatusOriginal> for SyncingStatus {
    fn into(self) -> SyncingStatusOriginal {
        match self {
            SyncingStatus::IsFalse => SyncingStatusOriginal::IsFalse,
            SyncingStatus::IsSyncing(v) => SyncingStatusOriginal::IsSyncing(Box::new(v.into())),
        }
    }
}

impl From<SyncingStatusOriginal> for SyncingStatus {
    fn from(value: SyncingStatusOriginal) -> Self {
        match value {
            SyncingStatusOriginal::IsFalse => SyncingStatus::IsFalse,
            SyncingStatusOriginal::IsSyncing(v) => SyncingStatus::IsSyncing((*v).into()),
        }
    }
}

impl ToPyObject for SyncingStatus {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match self {
            SyncingStatus::IsFalse => PyBool::new(py, false).into(),
            SyncingStatus::IsSyncing(v) => PyCell::new(py, v.to_owned()).unwrap().into(),
        }
    }
}

impl IntoPy<PyObject> for SyncingStatus {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}
