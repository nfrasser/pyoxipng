use std::path::PathBuf;
use pyo3::prelude::*;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::types::{PyString, PyDict, PyBytes};
use oxipng as op;

mod util;

create_exception!(oxipng, PngError, PyException);

#[pyfunction(kwds="**")]
#[pyo3(text_signature = "(input, output, **kwargs)")]
fn optimize(input: &PyString, output: Option<&PyString>, kwds: Option<&PyDict>) -> PyResult<()> {
    let inpath = PathBuf::from(input.to_str()?);
    let outpath = if let Some(out) = output {
        Some(PathBuf::from(out.to_str()?))
    } else {
        None
    };

    let inpath = op::InFile::Path(inpath);
    let outpath = op::OutFile::Path(outpath);

    op::optimize(&inpath, &outpath, &util::parse_kw_opts(kwds)?)
        .or_else(|err| Err(PngError::new_err(util::png_error_to_string(&err))))?;
    Ok(())
}

#[pyfunction(kwds="**")]
#[pyo3(text_signature = "(data, **kwargs)")]
fn optimize_from_memory(data: &PyBytes, kwds: Option<&PyDict>) -> PyResult<Py<PyBytes>> {
    let output = op::optimize_from_memory(data.as_bytes(), &util::parse_kw_opts(kwds)?)
        .or_else(|err| Err(PngError::new_err(util::png_error_to_string(&err))))?;
    Python::with_gil(|py| {
        let bytes: Py<PyBytes> = PyBytes::new(py, &*output).into();
        Ok(bytes)
    })
}


/// A Python module implemented in Rust.
#[pymodule]
fn oxipng(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("PngError", py.get_type::<PngError>())?;
    m.add_function(wrap_pyfunction!(optimize, m)?)?;
    m.add_function(wrap_pyfunction!(optimize_from_memory, m)?)?;
    Ok(())
}
