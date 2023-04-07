use std::str::FromStr;

use pyo3::{types::PyString, FromPyObject, IntoPy, PyAny, PyObject, PyResult, Python, ToPyObject};
use serde::{Deserialize, Serialize};

use ethers::{
    types::{
        Address as AddressOriginal, NameOrAddress as NameOrAddressOriginal, H160 as H160Original,
    },
    utils::to_checksum,
};
use web3_rush_macros::{tuple_enum_original_mapping, tuple_struct_original_mapping};

use crate::exceptions::wrap_from_hex_error;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[tuple_struct_original_mapping(H160Original)]
pub struct Address(pub AddressOriginal);

impl ToPyObject for Address {
    #[inline]
    fn to_object(&self, py: Python<'_>) -> PyObject {
        PyString::new(py, &to_checksum(&self.0, None)).into()
    }
}

impl IntoPy<PyObject> for Address {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

impl FromPyObject<'_> for Address {
    #[inline]
    fn extract(obj: &PyAny) -> PyResult<Self> {
        match AddressOriginal::from_str(obj.downcast::<PyString>()?.to_str()?) {
            Ok(v) => Ok(Address(v)),
            Err(err) => Err(wrap_from_hex_error(err)),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[tuple_enum_original_mapping(NameOrAddressOriginal)]
#[serde(untagged)]
pub enum NameOrAddress {
    /// An Ethereum Address
    Address(Address),
    /// An ENS Name (format does not get checked)
    Name(String),
}

impl ToPyObject for NameOrAddress {
    #[inline]
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match self {
            NameOrAddress::Name(name) => PyString::new(py, name).into(),
            NameOrAddress::Address(addr) => PyString::new(py, &to_checksum(&addr.0, None)).into(),
        }
    }
}

impl IntoPy<PyObject> for NameOrAddress {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

impl FromPyObject<'_> for NameOrAddress {
    #[inline]
    fn extract(obj: &PyAny) -> PyResult<Self> {
        match AddressOriginal::from_str(obj.downcast::<PyString>()?.to_str()?) {
            Ok(v) => Ok(NameOrAddress::Address(Address(v))),
            Err(_) => Ok(NameOrAddress::Name(
                obj.downcast::<PyString>()?.to_str()?.to_owned(),
            )),
        }
    }
}
