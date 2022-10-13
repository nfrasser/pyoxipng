use ::oxipng as op;
use op::IndexSet;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use std::num::NonZeroU8;

/// Zlib deflate strategy
#[pyclass]
#[derive(Clone, Debug)]
pub struct Zlib {
    compression: IndexSet<u8>, // 1-9
    strategies: IndexSet<u8>,  // 0-3
    window: u8,                // 8-15
}

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
pub struct Libdeflater();

#[pymethods]
impl Zlib {
    #[new]
    #[args(compression = "None", strategies = "None", window = "None")]
    fn new(
        compression: Option<Vec<u8>>,
        strategies: Option<Vec<u8>>,
        window: Option<u8>,
    ) -> PyResult<Self> {
        Ok(Self {
            compression: if let Some(compression) = compression {
                let mut set = IndexSet::default();
                for i in compression {
                    set.insert(i);
                }
                set
            } else {
                IndexSet::from([9])
            },
            strategies: if let Some(strategies) = strategies {
                let mut set = IndexSet::default();
                for i in strategies {
                    set.insert(i);
                }
                set
            } else {
                IndexSet::from([0, 1, 2, 3])
            },
            window: window.unwrap_or(15),
        })
    }
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
        Self()
    }
}

impl From<Zlib> for op::Deflaters {
    fn from(deflater: Zlib) -> Self {
        op::Deflaters::Zlib {
            compression: deflater.compression,
            strategies: deflater.strategies,
            window: deflater.window,
        }
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
    fn from(_: Libdeflater) -> Self {
        op::Deflaters::Libdeflater
    }
}
