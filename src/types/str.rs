use derive_more::Display;

use ethers::types::{
    Bytes as BytesOriginal, H160 as H160Original, H256 as H256Original, H64 as H64Original,
};

use hex::ToHex;

use pyo3::types::PyString;
use pyo3::{pyclass, FromPyObject, IntoPy, PyAny, PyObject, PyResult, Python, ToPyObject};

use serde::{Deserialize, Serialize};

use std::fmt::Debug;
use std::str::FromStr;
use web3_rush_macros::tuple_struct_original_mapping;

#[derive(Clone, Display, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[tuple_struct_original_mapping(String)]
#[pyclass(module = "web3_rush")]
pub struct HexStr(pub String);

pub enum AnyStr {
    Str(String),
    Bytes(Vec<u8>),
}

#[derive(Clone, Default, Deserialize)]
#[tuple_struct_original_mapping(H64Original)]
pub struct H64(pub H64Original);

impl ToPyObject for H64 {
    #[inline]
    fn to_object(&self, py: Python<'_>) -> PyObject {
        PyString::new(py, &format!("0x{}", self.0.encode_hex::<String>())).into()
    }
}

impl IntoPy<PyObject> for H64 {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

#[derive(
    Clone, Serialize, Deserialize, Default, Display, PartialEq, Eq, PartialOrd, Ord, Debug,
)]
#[tuple_struct_original_mapping(H256Original)]
pub struct H256(pub H256Original);

impl ToPyObject for H256 {
    #[inline]
    fn to_object(&self, py: Python<'_>) -> PyObject {
        PyString::new(py, &format!("0x{}", self.0.encode_hex::<String>())).into()
    }
}

impl IntoPy<PyObject> for H256 {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

impl FromPyObject<'_> for H256 {
    #[inline]
    fn extract(obj: &PyAny) -> PyResult<Self> {
        let binding = obj.downcast::<pyo3::types::PyString>()?.to_string();
        let s = binding.trim_start_matches("0x");
        Ok(H256(H256Original::from_str(s).unwrap()))
    }
}

#[derive(Clone)]
#[tuple_struct_original_mapping(H160Original)]
#[pyclass(module = "web3_rush")]
pub struct H160(pub H160Original);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[tuple_struct_original_mapping(BytesOriginal)]
pub struct Bytes(pub BytesOriginal);

impl From<String> for Bytes {
    fn from(value: String) -> Self {
        Bytes(BytesOriginal::from(value.as_bytes().to_vec()))
    }
}

impl ToPyObject for Bytes {
    #[inline]
    fn to_object(&self, py: Python<'_>) -> PyObject {
        PyString::new(py, &format!("0x{}", self.0.encode_hex::<String>())).into()
    }
}

impl IntoPy<PyObject> for Bytes {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}
