use ethers::types::{U256 as U256Original, U64 as U64Original};

use pyo3::exceptions::PyOverflowError;

use pyo3::{
    ffi, AsPyPointer, FromPyObject, IntoPy, PyAny, PyErr, PyObject, PyResult, Python, ToPyObject,
};

use serde::de;
use serde::{Deserialize, Serialize};

use std::ffi::c_uchar;
use std::fmt::Debug;
use std::str::FromStr;
use web3_rush_macros::tuple_struct_original_mapping;

#[derive(Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
#[tuple_struct_original_mapping(U64Original)]
#[repr(transparent)]
#[serde(transparent)]
pub struct U64(pub U64Original);

impl<'de> Deserialize<'de> for U64 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let num: serde_json::Number = serde_json::Number::deserialize(deserializer)?;
        let bigint = U64Original::from_str(&num.to_string()).unwrap();
        Ok(U64(bigint))
    }
}

impl ToPyObject for U64 {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let binding = Into::<ruint::aliases::U64>::into(self.clone());
        let bytes = binding.as_le_bytes();
        unsafe {
            let obj =
                ffi::_PyLong_FromByteArray(bytes.as_ptr().cast::<c_uchar>(), bytes.len(), 1, 0);
            PyObject::from_owned_ptr(py, obj)
        }
    }
}

impl IntoPy<PyObject> for U64 {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

impl<'source> FromPyObject<'source> for U64 {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        let mut result = ruint::aliases::U64::from(0);

        #[cfg(target_endian = "little")]
        let py_result = unsafe {
            let raw = result.as_le_slice_mut();
            ffi::_PyLong_AsByteArray(
                ob.as_ptr().cast::<ffi::PyLongObject>(),
                raw.as_mut_ptr(),
                raw.len(),
                1,
                0,
            )
        };

        #[cfg(not(target_endian = "little"))]
        let py_result = {
            let mut raw = vec![0_u8; Self::LIMBS * 8];
            let py_result = unsafe {
                ffi::_PyLong_AsByteArray(
                    ob.as_ptr().cast::<ffi::PyLongObject>(),
                    raw.as_mut_ptr(),
                    raw.len(),
                    1,
                    0,
                )
            };
            result = Self::try_from_le_slice(raw.as_slice()).ok_or_else(|| {
                PyOverflowError::new_err(format!("Number to large to fit Uint<{}>", Self::BITS))
            })?;
            py_result
        };

        if py_result != 0 {
            return Err(PyErr::fetch(ob.py()));
        }

        #[cfg(target_endian = "little")]
        if let Some(last) = Into::<ruint::aliases::U64>::into(result).as_limbs().last() {
            if *last > mask(ruint::aliases::U64::BITS) {
                return Err(PyOverflowError::new_err(format!(
                    "Number to large to fit Uint<{}>",
                    ruint::aliases::U64::BITS
                )));
            }
        }

        Ok(result.into())
    }
}

impl From<ruint::aliases::U64> for U64 {
    fn from(value: ruint::aliases::U64) -> U64 {
        U64(U64Original::from_big_endian(&value.to_be_bytes_vec()))
    }
}

impl Into<ruint::aliases::U64> for U64 {
    fn into(self) -> ruint::aliases::U64 {
        ruint::aliases::U64::from_str(&self.0.to_string()).unwrap()
    }
}

#[derive(Clone, Serialize, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct U256(pub num_bigint::BigUint);

impl<'de> Deserialize<'de> for U256 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let num: serde_json::Number = serde_json::Number::deserialize(deserializer)?;
        let bigint = num_bigint::BigUint::from_str(&num.to_string()).unwrap();
        Ok(U256(bigint))
    }
}

impl From<U256Original> for U256 {
    fn from(value: U256Original) -> U256 {
        U256(num_bigint::BigUint::from_str(&value.to_string()).unwrap())
    }
}

impl Into<U256Original> for U256 {
    fn into(self) -> U256Original {
        U256Original::from_dec_str(&self.0.to_string()).unwrap()
    }
}

impl From<num_bigint::BigUint> for U256 {
    fn from(value: num_bigint::BigUint) -> U256 {
        value.into()
    }
}

impl Into<num_bigint::BigUint> for U256 {
    fn into(self) -> num_bigint::BigUint {
        self.0
    }
}

impl ToPyObject for U256 {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        self.0.to_object(py)
    }
}

impl IntoPy<PyObject> for U256 {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

impl<'source> FromPyObject<'source> for U256 {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        match num_bigint::BigUint::extract(ob) {
            Ok(res) => Ok(U256(res)),
            Err(err) => Err(err),
        }
    }
}

const fn mask(bits: usize) -> u64 {
    if bits == 0 {
        return 0;
    }
    let bits = bits % 64;
    if bits == 0 {
        u64::MAX
    } else {
        (1 << bits) - 1
    }
}
