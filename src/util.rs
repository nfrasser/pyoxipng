use core::time::Duration;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyList, PySet, PyTuple};

pub fn py_iter_extract_map<'a, T, U, C>(val: &'a PyAny) -> PyResult<C>
where
    T: FromPyObject<'a>,
    U: From<T>,
    C: Default + Extend<U>,
{
    py_iter_to_collection(val, |val| {
        val.extract::<T>()
            .and_then(|x| Ok(x.into()))
            .or_else(|err| Err(err))
    })
}

/// Convert a collection of strings into a Iterable of `[u8; 4]` byte strings
/// used for the StripChunks option
pub fn py_iter_extract_chunks<'a, C>(val: &'a PyAny) -> PyResult<C>
where
    C: Default + Extend<[u8; 4]>,
{
    py_iter_to_collection(val, |x| {
        let chunk: [u8; 4] = x
            .extract::<String>()
            .or_else(|_| {
                Err(PyTypeError::new_err(format!(
                    "Invalid chunk {} with type {} (str expected)",
                    x.to_string(),
                    x.get_type().to_string()
                )))
            })?
            .as_bytes()
            .try_into()
            .or_else(|_| {
                Err(PyValueError::new_err(format!(
                    "Invalid chunk {} (must have 4 letters)",
                    x.to_string()
                )))
            })?;
        Ok(chunk)
    })
}

pub fn py_iter_to_collection<'a, T, C>(
    val: &'a PyAny,
    extract: impl Fn(&'a PyAny) -> PyResult<T>,
) -> PyResult<C>
where
    C: Default + Extend<T>,
{
    let mut collection = C::default();
    if let Ok(list) = val.downcast::<PyList>() {
        for item in list.iter() {
            collection.extend([extract(item)?]);
        }
    } else if let Ok(set) = val.downcast::<PySet>() {
        for item in set.iter() {
            collection.extend([extract(item)?]);
        }
    } else if let Ok(tuple) = val.downcast::<PyTuple>() {
        for item in tuple.iter() {
            collection.extend([extract(item)?]);
        }
    } else {
        return Err(PyTypeError::new_err(
            "Given value is not a list, set or tuple",
        ));
    }
    Ok(collection)
}

pub fn py_option<'a, T>(val: &'a PyAny) -> PyResult<Option<T>>
where
    T: FromPyObject<'a>,
{
    if val.is_none() {
        Ok(None)
    } else {
        Ok(Some(val.extract()?))
    }
}

pub fn py_duration(val: &PyAny) -> PyResult<Option<Duration>> {
    if let Some(seconds) = py_option::<f64>(val)? {
        Ok(Some(Duration::from_millis((seconds * 1000.) as u64)))
    } else {
        Ok(None)
    }
}
