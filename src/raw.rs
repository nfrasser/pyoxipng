use crate::{error, options};
use ::oxipng as oxi;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::borrow::Cow;

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
    fn rgb(transparent_color: Option<[u16; 3]>) -> PyResult<Self> {
        let transparent_color = if let Some(col) = transparent_color {
            Some(oxi::RGB16::new(col[0], col[1], col[2]))
        } else {
            None
        };
        Ok(Self(oxi::ColorType::RGB { transparent_color }))
    }

    #[staticmethod]
    #[pyo3(signature = (palette))]
    fn indexed(palette: Vec<[u8; 4]>) -> PyResult<Self> {
        let len = palette.len();
        if len == 0 || len > 256 {
            return Err(PyValueError::new_err(
                "palette len must be greater than 0 and less than or equal to 256",
            ));
        }
        Ok(Self(oxi::ColorType::Indexed {
            palette: palette
                .iter()
                .map(|col| oxi::RGBA8::new(col[0], col[1], col[2], col[3]))
                .collect(),
        }))
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
    fn add_png_chunk(&mut self, name: &[u8], data: Vec<u8>) -> PyResult<()> {
        self.0.add_png_chunk(
            name.try_into().or(Err(PyValueError::new_err(
                "Invalid chunk (must be 4 bytes long)",
            )))?,
            data,
        );
        Ok(())
    }

    #[pyo3(signature = (data))]
    fn add_icc_profile(&mut self, data: &[u8]) {
        self.0.add_icc_profile(data)
    }

    #[pyo3(signature = (**kwargs))]
    fn create_optimized_png<'a>(&self, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Cow<[u8]>> {
        self.0
            .create_optimized_png(&options::parse_kw_opts(kwargs)?)
            .and_then(|data| Ok(data.into()))
            .or_else(error::handle_png_error)
    }
}
