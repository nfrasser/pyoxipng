use ::oxipng as oxi;
use core::time::Duration;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    fmt::Debug,
    hash::{Hash, Hasher},
    iter::Iterator,
    num::NonZeroU8,
};

// NOTE: Deprecated as of v9.1
// Should use sequence or set where appropriate
#[derive(FromPyObject)]
pub enum Collection<T: Eq + Hash> {
    #[pyo3(transparent, annotation = "list | tuple")]
    Seq(Vec<T>),
    #[pyo3(transparent, annotation = "set | frozenset")]
    Set(HashSet<T>),
}

pub enum CollectionIterator<T: Eq + Hash> {
    SeqIter(std::vec::IntoIter<T>),
    SetIter(std::collections::hash_set::IntoIter<T>),
}

impl<T: Eq + Hash> Iterator for CollectionIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            CollectionIterator::SeqIter(iter) => iter.next(),
            CollectionIterator::SetIter(iter) => iter.next(),
        }
    }
}

impl<T: Eq + Hash> IntoIterator for Collection<T> {
    type Item = T;
    type IntoIter = CollectionIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Collection::Seq(vec) => CollectionIterator::SeqIter(vec.into_iter()),
            Collection::Set(set) => {
                eprintln!(
                    "(pyoxipng) Deprecation Warning: Python sets will not be accepted arguments in a future release. Please use a list or tuple instead."
                );
                CollectionIterator::SetIter(set.into_iter())
            }
        }
    }
}

impl<T: Eq + Hash> Collection<T> {
    pub fn remap<U, C>(self) -> C
    where
        U: From<T>,
        C: FromIterator<U>,
    {
        self.into_iter().map(|i| i.into()).collect()
    }
}

// Filter
#[pyclass(eq, eq_int)]
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
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
#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone, Debug, Hash)]
pub enum Interlacing {
    Off = oxi::Interlacing::None as isize,
    Adam7 = oxi::Interlacing::Adam7 as isize,
}

#[pymethods]
impl Interlacing {
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
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
    fn strip(val: Collection<[u8; 4]>) -> PyResult<Self> {
        Ok(Self(oxi::StripChunks::Strip(val.remap())))
    }

    #[staticmethod]
    fn safe() -> Self {
        Self(oxi::StripChunks::Safe)
    }

    #[staticmethod]
    fn keep(val: Collection<[u8; 4]>) -> PyResult<Self> {
        Ok(Self(oxi::StripChunks::Keep(val.remap())))
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

/// Extract a python value that may be None
pub fn py_option<'a, T: FromPyObject<'a>>(val: &Bound<'a, PyAny>) -> PyResult<Option<T>> {
    if val.is_none() {
        Ok(None)
    } else {
        Ok(Some(val.extract()?))
    }
}

/// Extract a python value that may be None and convert to another type
pub fn py_option_extract<'a, T, U>(val: &Bound<'a, PyAny>) -> PyResult<Option<U>>
where
    T: FromPyObject<'a>,
    U: From<T>,
{
    Ok(py_option::<T>(val)?.and_then(|v| Some(v.into())))
}

/// Extract a python float as a rust Duration type
pub fn py_duration(val: &Bound<'_, PyAny>) -> PyResult<Option<Duration>> {
    Ok(py_option::<f64>(val)?.and_then(|v| Some(Duration::from_millis((v * 1000.) as u64))))
}
