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
    let mut opts = op::Options::default();
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
        "backup" => opts.backup = value.downcast::<PyBool>()?.is_true(),
        "fix_errors" => opts.fix_errors = value.downcast::<PyBool>()?.is_true(),
        "pretend" => opts.pretend = value.downcast::<PyBool>()?.is_true(),
        "force" => opts.force = value.downcast::<PyBool>()?.is_true(),
        "preserve_attrs" => opts.preserve_attrs = value.downcast::<PyBool>()?.is_true(),
        "filter" => opts.filter = py_to_index_set(value)?,
        "interlace" => opts.interlace = py_option(value)?,
        _ => return Err(PyTypeError::new_err("Unknown option"))
    }
    Ok(())
}

fn py_to_index_set<'a, T>(val: &'a PyAny) -> PyResult<op::IndexSet<T>>
where T: Clone + Eq + std::hash::Hash + FromPyObject<'a> {
    let mut index_set: op::IndexSet<T> = op::IndexSet::default();
    if let Ok(list) = val.downcast::<PyList>() {
        index_set.reserve(list.len());
        for item in list.iter() {
            index_set.insert(item.extract()?);
        }
    } else if let Ok(set) = val.downcast::<PySet>() {
        index_set.reserve(set.len());
        for item in set.iter() {
            index_set.insert(item.extract()?);
        }
    } else if let Ok(tuple) = val.downcast::<PyTuple>() {
        index_set.reserve(tuple.len());
        for item in tuple.iter() {
            index_set.insert(item.extract()?);
        }
    } else {
        return Err(PyTypeError::new_err("Given value is not a list, set or tuple"))
    }
    Ok(index_set)
}

fn py_option<'a, T>(val: &'a PyAny) -> PyResult<Option<T>>
where T: FromPyObject<'a> {
    if val.is_none() {
        Ok(None)
    } else {
        Ok(Some(val.extract()?))
    }
}
