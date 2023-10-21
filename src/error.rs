use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use ::oxipng as oxi;

create_exception!(oxipng, PngError, PyException);

pub fn handle_png_error<T>(err: oxi::PngError) -> PyResult<T> {
    Err(PngError::new_err(format!("{}", err)))
}
