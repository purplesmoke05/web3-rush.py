use pyo3::{
    create_exception,
    exceptions::{PyException},
    prelude::*,
};

create_exception!(exceptions, BaseWeb3RushError, PyException);

pub fn wrap_web3_error(error: web3::Error) -> pyo3::PyErr {
    return BaseWeb3RushError::new_err(format!("Web3 error: {:?}", error));
}

pub fn wrap_hex_error(error: hex::FromHexError) -> pyo3::PyErr {
    return BaseWeb3RushError::new_err(format!("Hex error: {:?}", error));
}

pub fn init_module(py: Python, parent_module: &PyModule, library: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "exceptions")?;
    submod.add(stringify!(BaseWeb3RushError), py.get_type::<BaseWeb3RushError>())?;
    library.add(stringify!(BaseWeb3RushError), py.get_type::<BaseWeb3RushError>())?;

    parent_module.add_submodule(submod)?;
    Ok(())
}