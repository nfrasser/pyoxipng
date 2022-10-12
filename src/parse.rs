use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyString};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::string::String;

use ::oxipng as op;
use op::IndexSet;

use crate::deflaters::{Libdeflater, Zlib, Zopfli};
use crate::util::*;

// Alpha optimization
#[pyclass]
#[derive(Clone, Debug)]
pub enum AlphaOptim {
    NoOp = op::AlphaOptim::NoOp as isize,
    Black = op::AlphaOptim::Black as isize,
    White = op::AlphaOptim::White as isize,
    Up = op::AlphaOptim::Up as isize,
    Right = op::AlphaOptim::Right as isize,
    Down = op::AlphaOptim::Down as isize,
    Left = op::AlphaOptim::Left as isize,
}

#[pymethods]
impl AlphaOptim {
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        (*self as u64).hash(&mut hasher);
        hasher.finish()
    }
}

impl From<AlphaOptim> for op::AlphaOptim {
    fn from(val: AlphaOptim) -> Self {
        match val {
            AlphaOptim::NoOp => Self::NoOp,
            AlphaOptim::Black => Self::Black,
            AlphaOptim::White => Self::White,
            AlphaOptim::Up => Self::Up,
            AlphaOptim::Right => Self::Right,
            AlphaOptim::Down => Self::Down,
            AlphaOptim::Left => Self::Left,
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
        "pretend" => opts.pretend = value.downcast::<PyBool>()?.is_true(),
        "force" => opts.force = value.downcast::<PyBool>()?.is_true(),
        "preserve_attrs" => opts.preserve_attrs = value.downcast::<PyBool>()?.is_true(),
        "filter" => opts.filter = py_iter_extract::<u8, IndexSet<u8>>(value)?,
        "interlace" => opts.interlace = py_option(value)?,
        "alphas" => {
            opts.alphas =
                py_iter_extract_map::<AlphaOptim, op::AlphaOptim, IndexSet<op::AlphaOptim>>(value)?
        }
        "bit_depth_reduction" => opts.bit_depth_reduction = value.downcast::<PyBool>()?.is_true(),
        "color_type_reduction" => opts.color_type_reduction = value.downcast::<PyBool>()?.is_true(),
        "palette_reduction" => opts.palette_reduction = value.downcast::<PyBool>()?.is_true(),
        "grayscale_reduction" => opts.grayscale_reduction = value.downcast::<PyBool>()?.is_true(),
        "idat_recoding" => opts.idat_recoding = value.downcast::<PyBool>()?.is_true(),
        "strip" => opts.strip = value.extract::<Headers>()?.0,
        "deflate" => match value {
            value if value.is_instance_of::<Zlib>()? => {
                opts.deflate = value.extract::<Zlib>()?.into()
            }
            value if value.is_instance_of::<Zopfli>()? => {
                opts.deflate = value.extract::<Zopfli>()?.into()
            }
            value if value.is_instance_of::<Libdeflater>()? => {
                opts.deflate = value.extract::<Libdeflater>()?.into()
            }
            _ => return Err(PyTypeError::new_err("Unsupported option")),
        },
        "use_heuristics" => opts.use_heuristics = value.downcast::<PyBool>()?.is_true(),
        "timeout" => opts.timeout = py_duration(value)?,
        _ => return Err(PyTypeError::new_err("Unsupported option")),
    }
    Ok(())
}
