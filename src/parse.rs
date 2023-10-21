use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyString};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::num::NonZeroU8;

use ::oxipng as oxi;
use oxi::IndexSet;

use crate::util::*;

// Filter
#[pyclass]
#[derive(Clone, Debug)]
pub enum RowFilter {
    NoOp = oxi::RowFilter::None as isize,
    Sub = oxi::RowFilter::Sub as isize,
    Up = oxi::RowFilter::Up as isize,
    Average = oxi::RowFilter::Average as isize,
    Paeth = oxi::RowFilter::Paeth as isize,
    MinSum = oxi::RowFilter::MinSum as isize,
    Entropy = oxi::RowFilter::Entropy as isize,
    Bigrams = oxi::RowFilter::Bigrams as isize,
    BigEnt = oxi::RowFilter::BigEnt as isize,
    Brute = oxi::RowFilter::Brute as isize,
}

#[pymethods]
impl RowFilter {
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        (self.clone() as u64).hash(&mut hasher);
        hasher.finish()
    }
}

impl From<RowFilter> for oxi::RowFilter {
    fn from(val: RowFilter) -> Self {
        match val {
            RowFilter::NoOp => Self::None,
            RowFilter::Sub => Self::Sub,
            RowFilter::Up => Self::Up,
            RowFilter::Average => Self::Average,
            RowFilter::Paeth => Self::Paeth,
            RowFilter::MinSum => Self::MinSum,
            RowFilter::Entropy => Self::Entropy,
            RowFilter::Bigrams => Self::Bigrams,
            RowFilter::BigEnt => Self::BigEnt,
            RowFilter::Brute => Self::Brute,
        }
    }
}

// Interlacing
#[pyclass]
#[derive(Clone, Debug)]
pub enum Interlacing {
    Off = oxi::Interlacing::None as isize,
    Adam7 = oxi::Interlacing::Adam7 as isize,
}

#[pymethods]
impl Interlacing {
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        (self.clone() as u64).hash(&mut hasher);
        hasher.finish()
    }
}

impl From<Interlacing> for oxi::Interlacing {
    fn from(val: Interlacing) -> Self {
        match val {
            Interlacing::Off => Self::None,
            Interlacing::Adam7 => Self::Adam7,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct StripChunks(pub oxi::StripChunks);

#[pymethods]
impl StripChunks {
    #[staticmethod]
    fn none() -> Self {
        Self(oxi::StripChunks::None)
    }

    #[staticmethod]
    fn strip(val: &PyAny) -> PyResult<Self> {
        let chunks: IndexSet<[u8; 4]> = py_iter_to_collection(val, py_str_to_chunk)?;
        Ok(Self(oxi::StripChunks::Strip(chunks)))
    }

    #[staticmethod]
    fn safe() -> Self {
        Self(oxi::StripChunks::Safe)
    }

    #[staticmethod]
    fn keep(val: &PyAny) -> PyResult<Self> {
        let chunks: IndexSet<[u8; 4]> = py_iter_to_collection(val, py_str_to_chunk)?;
        Ok(Self(oxi::StripChunks::Keep(chunks)))
    }
    #[staticmethod]
    fn all() -> Self {
        Self(oxi::StripChunks::All)
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Deflaters(pub oxi::Deflaters);

#[pymethods]
impl Deflaters {
    #[staticmethod]
    fn libdeflater(compression: u8) -> Self {
        Self(oxi::Deflaters::Libdeflater { compression })
    }

    #[staticmethod]
    fn zopfli(iterations: u8) -> PyResult<Self> {
        if let Some(iterations) = NonZeroU8::new(iterations) {
            Ok(Self(oxi::Deflaters::Zopfli { iterations }))
        } else {
            Err(PyTypeError::new_err(format!(
                "Invalid zopfli iterations {}; must be in range [1, 255]",
                iterations
            )))
        }
    }
}

pub fn parse_kw_opts(kwds: Option<&PyDict>) -> PyResult<oxi::Options> {
    if let Some(kwopts) = kwds {
        parse_kw_opts_dict(kwopts)
    } else {
        Ok(oxi::Options::default())
    }
}

pub fn parse_kw_opts_dict(kwops: &PyDict) -> PyResult<oxi::Options> {
    let mut opts = if let Some(level) = kwops.get_item("level") {
        let level: u8 = level.extract().or_else(|err| {
            Err(PyValueError::new_err(format!(
                "Invalid optimization level; countered {}",
                err
            )))
        })?;
        if level > 6 {
            return Err(PyValueError::new_err(
                "Invalid optimization level; must be between 0 and 6 inclusive",
            ));
        }
        oxi::Options::from_preset(level)
    } else {
        oxi::Options::default()
    };

    for (k, v) in kwops.iter() {
        let key: &PyString = k.downcast()?;
        let key = key.to_str()?;
        parse_kw_opt(key, v, &mut opts).or_else(|err| {
            Err(PyTypeError::new_err(format!(
                "Invalid option '{}'; encountered {}",
                key, err
            )))
        })?;
    }
    Ok(opts)
}

fn parse_kw_opt(key: &str, value: &PyAny, opts: &mut oxi::Options) -> PyResult<()> {
    match key {
        "level" => {} // Handled elsewhere, ignore
        "fix_errors" => opts.fix_errors = value.downcast::<PyBool>()?.is_true(),
        "force" => opts.force = value.downcast::<PyBool>()?.is_true(),
        "filter" => {
            opts.filter =
                py_iter_extract_map::<RowFilter, oxi::RowFilter, IndexSet<oxi::RowFilter>>(value)?
        }
        "interlace" => {
            opts.interlace = if let Some(i) = py_option::<Interlacing>(value)? {
                Some(i.into())
            } else {
                None
            }
        }
        "optimize_alpha" => opts.optimize_alpha = value.downcast::<PyBool>()?.is_true(),
        "bit_depth_reduction" => opts.bit_depth_reduction = value.downcast::<PyBool>()?.is_true(),
        "color_type_reduction" => opts.color_type_reduction = value.downcast::<PyBool>()?.is_true(),
        "palette_reduction" => opts.palette_reduction = value.downcast::<PyBool>()?.is_true(),
        "grayscale_reduction" => opts.grayscale_reduction = value.downcast::<PyBool>()?.is_true(),
        "idat_recoding" => opts.idat_recoding = value.downcast::<PyBool>()?.is_true(),
        "scale_16" => opts.scale_16 = value.downcast::<PyBool>()?.is_true(),
        "strip" => opts.strip = value.extract::<StripChunks>()?.0,
        "deflate" => opts.deflate = value.extract::<Deflaters>()?.0,
        "fast_evaluation" => opts.fast_evaluation = value.downcast::<PyBool>()?.is_true(),
        "timeout" => opts.timeout = py_duration(value)?,
        _ => return Err(PyTypeError::new_err("Unsupported option")),
    }
    Ok(())
}
