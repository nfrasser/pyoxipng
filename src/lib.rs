use ::oxipng as op;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use std::path::PathBuf;

mod parse;
mod util;

create_exception!(oxipng, PngError, PyException);

/// Optimize the png file at the given input location. Optionally send it to the
/// given output location.
#[pyfunction]
#[pyo3(signature = (input, output=None, **kwargs))]
fn optimize(input: &PyAny, output: Option<&PyAny>, kwargs: Option<&PyDict>) -> PyResult<()> {
    let inpath = PathBuf::from(input.str()?.to_str()?);
    let outpath = if let Some(out) = output {
        Some(PathBuf::from(out.str()?.to_str()?))
    } else {
        None
    };

    let inpath = op::InFile::Path(inpath);
    let outpath = op::OutFile::Path(outpath);

    op::optimize(&inpath, &outpath, &parse::parse_kw_opts(kwargs)?)
        .or_else(|err| Err(PngError::new_err(parse::png_error_to_string(&err))))?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (data, **kwargs))]
fn optimize_from_memory(data: &PyBytes, kwargs: Option<&PyDict>) -> PyResult<Py<PyBytes>> {
    let output = op::optimize_from_memory(data.as_bytes(), &parse::parse_kw_opts(kwargs)?)
        .or_else(|err| Err(PngError::new_err(parse::png_error_to_string(&err))))?;
    Python::with_gil(|py| {
        let bytes: Py<PyBytes> = PyBytes::new(py, &*output).into();
        Ok(bytes)
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn oxipng(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("PngError", py.get_type::<PngError>())?;
    m.add_class::<parse::RowFilter>()?;
    m.add_class::<parse::Interlacing>()?;
    m.add_class::<parse::Headers>()?;
    m.add_class::<parse::Deflaters>()?;
    m.add_function(wrap_pyfunction!(optimize, m)?)?;
    m.add_function(wrap_pyfunction!(optimize_from_memory, m)?)?;
    Ok(())
}
