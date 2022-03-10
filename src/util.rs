use core::time::Duration;
use std::string::String;
use pyo3::prelude::*;
use pyo3::exceptions::PyTypeError;
use pyo3::types::{PyString, PyDict, PyBool, PyList, PySet, PyTuple};
use oxipng as op;

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
        _ => String::from("An unknown error occurred!")
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
        let level: u8 = level.extract()
            .or_else(|err| Err(PyTypeError::new_err(format!("Invalid optimization level; countered {}", err))))?;
        if level > 6 {
            return Err(PyTypeError::new_err(format!("Invalid optimization level; must be between 0 and 6 inclusive")))
        }
        op::Options::from_preset(level)
    } else {
        op::Options::default()
    };

    for (k, v) in kwops.iter() {
        let key: &PyString = k.downcast()?;
        let key = key.to_str()?;
        parse_kw_opt(key, v, &mut opts)
            .or_else(|err| Err(PyTypeError::new_err(format!("Invalid option '{}'; encountered {}", key, err))))?;
    }
    Ok(opts)
}

fn parse_kw_opt(key: &str, value: &PyAny, opts: &mut op::Options) -> PyResult<()> {
    match key {
        "level" => {}, // Handled elsewhere, ignore
        "backup" => opts.backup = value.downcast::<PyBool>()?.is_true(),
        "fix_errors" => opts.fix_errors = value.downcast::<PyBool>()?.is_true(),
        "pretend" => opts.pretend = value.downcast::<PyBool>()?.is_true(),
        "force" => opts.force = value.downcast::<PyBool>()?.is_true(),
        "preserve_attrs" => opts.preserve_attrs = value.downcast::<PyBool>()?.is_true(),
        "filter" => opts.filter = py_iter_to_collection::<u8, op::IndexSet<u8>>(value)?,
        "interlace" => opts.interlace = py_option(value)?,
        // "alphas" => panic!("Not supported yet!"),
        "bit_depth_reduction" => opts.bit_depth_reduction = value.downcast::<PyBool>()?.is_true(),
        "color_type_reduction" => opts.color_type_reduction = value.downcast::<PyBool>()?.is_true(),
        "palette_reduction" => opts.palette_reduction = value.downcast::<PyBool>()?.is_true(),
        "grayscale_reduction" => opts.grayscale_reduction = value.downcast::<PyBool>()?.is_true(),
        "idat_recoding" => opts.idat_recoding = value.downcast::<PyBool>()?.is_true(),
        // "strip" => panic!("Not supported yet!"),
        // "deflate" => panic!("Not supported yet!"),
        "use_heuristics" => opts.use_heuristics = value.downcast::<PyBool>()?.is_true(),
        "timeout" => opts.timeout = py_duration(value)?,
        _ => return Err(PyTypeError::new_err("Unsupported option"))
    }
    Ok(())
}

fn py_iter_to_collection<'a, T, C>(val: &'a PyAny) -> PyResult<C>
where T: FromPyObject<'a>, C: Default + Extend<T> {
    let mut collection = C::default();
    if let Ok(list) = val.downcast::<PyList>() {
        for item in list.iter() {
            collection.extend([item.extract()?]);
        }
    } else if let Ok(set) = val.downcast::<PySet>() {
        for item in set.iter() {
            collection.extend([item.extract()?]);
        }
    } else if let Ok(tuple) = val.downcast::<PyTuple>() {
        for item in tuple.iter() {
            collection.extend([item.extract()?]);
        }
    } else {
        return Err(PyTypeError::new_err("Given value is not a list, set or tuple"))
    }
    Ok(collection)
}

fn py_option<'a, T>(val: &'a PyAny) -> PyResult<Option<T>>
where T: FromPyObject<'a> {
    if val.is_none() {
        Ok(None)
    } else {
        Ok(Some(val.extract()?))
    }
}

fn py_duration(val: &PyAny) -> PyResult<Option<Duration>> {
    if let Some(seconds) = py_option::<f64>(val)? {
        Ok(Some(Duration::from_millis((seconds * 1000.) as u64)))
    } else {
        Ok(None)
    }
}
