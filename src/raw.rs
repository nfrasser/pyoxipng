use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::borrow::Cow;

use crate::{error, parse, util};
use ::oxipng as oxi;

#[pyclass]
#[derive(Debug, Clone)]
pub struct ColorType(pub oxi::ColorType);

#[pymethods]
impl ColorType {
    #[staticmethod]
    #[pyo3(signature = (transparent_shade=None))]
    fn grayscale(transparent_shade: Option<u16>) -> Self {
        Self(oxi::ColorType::Grayscale { transparent_shade })
    }

    #[staticmethod]
    #[pyo3(signature = (transparent_color=None))]
    fn rgb(transparent_color: Option<&PyAny>) -> PyResult<Self> {
        let transparent_color = if let Some(col) = transparent_color {
            let col = util::py_iter_extract::<u16, Vec<u16>>(col)?;
            if col.len() != 3 {
                return Err(PyValueError::new_err(
                    "Expected collection of three 16-bit ints",
                ));
            }
            Some(oxi::RGB16::new(col[0], col[1], col[2]))
        } else {
            None
        };
        Ok(Self(oxi::ColorType::RGB { transparent_color }))
    }

    #[staticmethod]
    #[pyo3(signature = (palette))]
    fn indexed(palette: &PyList) -> PyResult<Self> {
        let capacity = palette.len();
        if capacity == 0 || capacity > 256 {
            return Err(PyValueError::new_err(
                "palette len must be greater than 0 and less than or equal to 256",
            ));
        }
        let mut pal = Vec::with_capacity(capacity);
        for col in palette {
            let col = util::py_iter_extract::<u8, Vec<u8>>(col)?;
            if col.len() != 4 {
                return Err(PyValueError::new_err(
                    "Expected each item in palette to be a collection of four 8-bit ints",
                ));
            }
            pal.push(oxi::RGBA8::new(col[0], col[1], col[2], col[3]))
        }
        Ok(Self(oxi::ColorType::Indexed { palette: pal }))
    }

    #[staticmethod]
    fn grayscale_alpha() -> Self {
        Self(oxi::ColorType::GrayscaleAlpha)
    }

    #[staticmethod]
    fn rgba() -> Self {
        Self(oxi::ColorType::RGBA)
    }
}
#[pyclass]
#[derive(Debug)]
pub struct RawImage(pub oxi::RawImage);

#[pymethods]
impl RawImage {
    #[new]
    #[pyo3(signature = (data, width, height, *, color_type=None, bit_depth=8))]
    fn py_new(
        data: Vec<u8>,
        width: u32,
        height: u32,
        color_type: Option<&ColorType>, // default RGBA
        bit_depth: Option<u8>,          // default 8
    ) -> PyResult<Self> {
        let color_type = if let Some(t) = color_type {
            t.0.clone()
        } else {
            oxi::ColorType::RGBA
        };
        let bit_depth = if let Some(d) = bit_depth {
            match d {
                1 => oxi::BitDepth::One,
                2 => oxi::BitDepth::Two,
                4 => oxi::BitDepth::Four,
                8 => oxi::BitDepth::Eight,
                16 => oxi::BitDepth::Sixteen,
                _ => {
                    return Err(PyValueError::new_err(format!(
                        "Invalid bit_depth {}; must be 1, 2, 4, 8 or 16",
                        d
                    )))
                }
            }
        } else {
            oxi::BitDepth::Eight
        };

        Ok(RawImage(
            oxi::RawImage::new(width, height, color_type, bit_depth, data)
                .or_else(error::handle_png_error)?,
        ))
    }

    #[pyo3(signature = (name, data))]
    fn add_png_chunk(&mut self, name: &PyAny, data: Vec<u8>) -> PyResult<()> {
        self.0.add_png_chunk(util::py_str_to_chunk(name)?, data);
        Ok(())
    }

    #[pyo3(signature = (data))]
    fn add_icc_profile(&mut self, data: &[u8]) {
        self.0.add_icc_profile(data)
    }

    #[pyo3(signature = (**kwargs))]
    fn create_optimized_png(&self, kwargs: Option<&PyDict>) -> PyResult<Cow<[u8]>> {
        self.0
            .create_optimized_png(&parse::parse_kw_opts(kwargs)?)
            .and_then(|data| Ok(data.into()))
            .or_else(error::handle_png_error)
    }
}
