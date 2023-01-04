use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyString};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::num::NonZeroU8;
use std::string::String;

use ::oxipng as op;
use op::IndexSet;

use crate::util::*;

// Filter
#[pyclass]
#[derive(Clone, Debug)]
pub enum RowFilter {
    NoOp = op::RowFilter::None as isize,
    Sub = op::RowFilter::Sub as isize,
    Up = op::RowFilter::Up as isize,
    Average = op::RowFilter::Average as isize,
    Paeth = op::RowFilter::Paeth as isize,
    MinSum = op::RowFilter::MinSum as isize,
    Entropy = op::RowFilter::Entropy as isize,
    Bigrams = op::RowFilter::Bigrams as isize,
    BigEnt = op::RowFilter::BigEnt as isize,
    Brute = op::RowFilter::Brute as isize,
}

#[pymethods]
impl RowFilter {
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        (self.clone() as u64).hash(&mut hasher);
        hasher.finish()
    }
}

impl From<RowFilter> for op::RowFilter {
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
    Off = op::Interlacing::None as isize,
    Adam7 = op::Interlacing::Adam7 as isize,
}

#[pymethods]
impl Interlacing {
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        (self.clone() as u64).hash(&mut hasher);
        hasher.finish()
    }
}

impl From<Interlacing> for op::Interlacing {
    fn from(val: Interlacing) -> Self {
        match val {
            Interlacing::Off => Self::None,
            Interlacing::Adam7 => Self::Adam7,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Headers(pub op::Headers);

#[pymethods]
impl Headers {
    #[staticmethod]
    fn none() -> Self {
        Self(op::Headers::None)
    }

    #[staticmethod]
    fn strip(val: &PyAny) -> PyResult<Self> {
        let chunks: Vec<String> = py_iter_extract(val)?;
        Ok(Self(op::Headers::Strip(chunks)))
    }

    #[staticmethod]
    fn safe() -> Self {
        Self(op::Headers::Safe)
    }

    #[staticmethod]
    fn keep(val: &PyAny) -> PyResult<Self> {
        let chunks: IndexSet<String> = py_iter_extract(val)?;
        Ok(Self(op::Headers::Keep(chunks)))
    }
    #[staticmethod]
    fn all() -> Self {
        Self(op::Headers::All)
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Deflaters(pub op::Deflaters);

#[pymethods]
impl Deflaters {
    #[staticmethod]
    fn libdeflater(compression: u8) -> Self {
        Self(op::Deflaters::Libdeflater { compression })
    }

    #[staticmethod]
    fn zopfli(iterations: u8) -> PyResult<Self> {
        if let Some(iterations) = NonZeroU8::new(iterations) {
            Ok(Self(op::Deflaters::Zopfli { iterations }))
        } else {
            Err(PyTypeError::new_err(format!(
                "Invalid zopfli iterations {}; must be in range [1, 255]",
                iterations
            )))
        }
    }
}

pub fn png_error_to_string(err: &op::PngError) -> String {
    match err {
        op::PngError::DeflatedDataTooLong(x) => format!("Deflated Data Too Long: {}", x),
        op::PngError::TimedOut => String::from("Timed Out"),
        op::PngError::NotPNG => String::from("Not PNG"),
        op::PngError::APNGNotSupported => String::from("APNG Not Supported"),
        op::PngError::InvalidData => String::from("Invalid Data"),
        op::PngError::TruncatedData => String::from("Truncated Data"),
        op::PngError::ChunkMissing(s) => format!("Chunk Missing: {}", s),
        op::PngError::Other(err) => format!("Other: {}", err),
        _ => String::from("An unknown error occurred!"),
    }
}

pub fn parse_kw_opts(kwds: Option<&PyDict>) -> PyResult<op::Options> {
    if let Some(kwopts) = kwds {
        parse_kw_opts_dict(kwopts)
    } else {
        Ok(op::Options::default())
    }
}

pub fn parse_kw_opts_dict(kwops: &PyDict) -> PyResult<op::Options> {
    let mut opts = if let Some(level) = kwops.get_item("level") {
        let level: u8 = level.extract().or_else(|err| {
            Err(PyTypeError::new_err(format!(
                "Invalid optimization level; countered {}",
                err
            )))
        })?;
        if level > 6 {
            return Err(PyTypeError::new_err(format!(
                "Invalid optimization level; must be between 0 and 6 inclusive"
            )));
        }
        op::Options::from_preset(level)
    } else {
        op::Options::default()
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

fn parse_kw_opt(key: &str, value: &PyAny, opts: &mut op::Options) -> PyResult<()> {
    match key {
        "level" => {} // Handled elsewhere, ignore
        "backup" => opts.backup = value.downcast::<PyBool>()?.is_true(),
        "fix_errors" => opts.fix_errors = value.downcast::<PyBool>()?.is_true(),
        "check" => opts.check = value.downcast::<PyBool>()?.is_true(),
        "pretend" => opts.pretend = value.downcast::<PyBool>()?.is_true(),
        "force" => opts.force = value.downcast::<PyBool>()?.is_true(),
        "preserve_attrs" => opts.preserve_attrs = value.downcast::<PyBool>()?.is_true(),
        "filter" => {
            opts.filter =
                py_iter_extract_map::<RowFilter, op::RowFilter, IndexSet<op::RowFilter>>(value)?
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
        "strip" => opts.strip = value.extract::<Headers>()?.0,
        "deflate" => opts.deflate = value.extract::<Deflaters>()?.0,
        "fast_evaluation" => opts.fast_evaluation = value.downcast::<PyBool>()?.is_true(),
        "timeout" => opts.timeout = py_duration(value)?,
        _ => return Err(PyTypeError::new_err("Unsupported option")),
    }
    Ok(())
}
