use ::oxipng as oxi;
use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyTuple};
use std::path::PathBuf;

mod parse;
mod util;

create_exception!(oxipng, PngError, PyException);

/// Optimize the png file at the given input location. Optionally send it to the
/// given output location.
#[pyfunction]
#[pyo3(signature = (input, *args, **kwargs))]
fn optimize(input: &PyAny, args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<()> {
    if args.len() > 1 {
        return Err(PyTypeError::new_err(format!(
            "optimize() takes 1 or 2 positional arguments but {} were given",
            args.len() + 1
        )));
    }

    let inpath = oxi::InFile::Path(PathBuf::from(input.str()?.to_str()?));
    let outpath = if args.len() == 1 {
        let output = args.get_item(0)?;
        if output.is_none() {
            // Explicit None, don't write output, just calculate
            oxi::OutFile::None
        } else {
            // Write to path
            oxi::OutFile::from_path(PathBuf::from(output.str()?.to_str()?))
        }
    } else {
        // in-place optimize
        oxi::OutFile::Path {
            path: None,
            preserve_attrs: false,
        }
    };

    oxi::optimize(&inpath, &outpath, &parse::parse_kw_opts(kwargs)?)
        .or_else(|err| Err(PngError::new_err(parse::png_error_to_string(&err))))?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (data, **kwargs))]
fn optimize_from_memory(data: &PyBytes, kwargs: Option<&PyDict>) -> PyResult<Py<PyBytes>> {
    let output = oxi::optimize_from_memory(data.as_bytes(), &parse::parse_kw_opts(kwargs)?)
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
    m.add_class::<parse::StripChunks>()?;
    m.add_class::<parse::Deflaters>()?;
    m.add_function(wrap_pyfunction!(optimize, m)?)?;
    m.add_function(wrap_pyfunction!(optimize_from_memory, m)?)?;
    Ok(())
}
