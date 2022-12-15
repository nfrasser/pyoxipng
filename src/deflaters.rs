use ::oxipng as op;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use std::num::NonZeroU8;

/// Zopfli deflate strategy
#[pyclass]
#[derive(Clone, Debug)]
pub struct Zopfli {
    /// The number of compression iterations to do.
    iterations: NonZeroU8,
}

/// libdeflater deflate strategy
#[pyclass]
#[derive(Clone, Debug)]
pub struct Libdeflater
{
    compression: u8, // 1-12
}

#[pymethods]
impl Zopfli {
    #[new]
    fn new(iterations: u8) -> PyResult<Self> {
        if let Some(iters) = NonZeroU8::new(iterations) {
            Ok(Self { iterations: iters })
        } else {
            Err(PyTypeError::new_err(format!(
                "Invalid zopfli iterations {}; must be in range [1, 255]",
                iterations
            )))
        }
    }
}

#[pymethods]
impl Libdeflater {
    #[new]
    fn new() -> Self {
        Self{compression: 2}
    }
}

impl From<Zopfli> for op::Deflaters {
    fn from(deflater: Zopfli) -> Self {
        op::Deflaters::Zopfli {
            iterations: deflater.iterations,
        }
    }
}

impl From<Libdeflater> for op::Deflaters {
    fn from(deflater: Libdeflater) -> Self {
        op::Deflaters::Libdeflater{
            compression: deflater.compression,
        }
    }
}
