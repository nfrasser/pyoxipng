use ::oxipng as oxi;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::borrow::Cow;
use std::path::PathBuf;

mod error;
mod options;
mod raw;
mod types;

/// Optimize the png file at the given input location. Optionally send it to the
/// given output location.
#[pyfunction]
#[pyo3(signature = (input, output=None, **kwargs))]
fn optimize(
    input: PathBuf,
    output: Option<PathBuf>,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<()> {
    let inpath = oxi::InFile::Path(input);
    let outpath = output
        .and_then(|path| Some(oxi::OutFile::from_path(path)))
        .unwrap_or(oxi::OutFile::Path {
            path: None,
            preserve_attrs: false,
        }); // No arg specified, in-place optimize

    oxi::optimize(&inpath, &outpath, &options::parse_kw_opts(kwargs)?)
        .or_else(error::handle_png_error)?;
    Ok(())
}

/// Perform optimization on the input file using the options provided, where the
/// file is already loaded in-memory
#[pyfunction]
#[pyo3(signature = (data, **kwargs))]
fn optimize_from_memory<'a>(
    data: &'a [u8],
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<Cow<'a, [u8]>> {
    // Note: returned Cow<[u8]> interpreted as Python bytes
    oxi::optimize_from_memory(data, &options::parse_kw_opts(kwargs)?)
        .and_then(|data| Ok(data.into()))
        .or_else(error::handle_png_error)
}

/// A Python module implemented in Rust.
#[pymodule]
fn oxipng(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("PngError", py.get_type::<error::PngError>())?;
    m.add_class::<types::RowFilter>()?;
    m.add_class::<types::Interlacing>()?;
    m.add_class::<types::StripChunks>()?;
    m.add_class::<types::Deflaters>()?;
    m.add_class::<raw::ColorType>()?;
    m.add_class::<raw::RawImage>()?;
    m.add_function(wrap_pyfunction!(optimize, m)?)?;
    m.add_function(wrap_pyfunction!(optimize_from_memory, m)?)?;
    Ok(())
}
