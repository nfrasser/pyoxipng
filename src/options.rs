use crate::types::*;
use ::oxipng as oxi;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyString};

pub fn parse_kw_opts(kwds: Option<&Bound<'_, PyDict>>) -> PyResult<oxi::Options> {
    if let Some(kwopts) = kwds {
        parse_kw_opts_dict(kwopts)
    } else {
        Ok(oxi::Options::default())
    }
}

pub fn parse_kw_opts_dict(kwops: &Bound<'_, PyDict>) -> PyResult<oxi::Options> {
    let mut opts = if let Some(level) = kwops.get_item("level")? {
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
        let key = k.downcast::<PyString>()?;
        let key = key.to_str()?;
        parse_kw_opt(key, &v, &mut opts).or_else(|err| {
            Err(PyTypeError::new_err(format!(
                "Invalid option '{}'; encountered {}",
                key, err
            )))
        })?;
    }
    Ok(opts)
}

fn parse_kw_opt(key: &str, value: &Bound<'_, PyAny>, opts: &mut oxi::Options) -> PyResult<()> {
    match key {
        "level" => {} // Handled elsewhere, ignore
        "fix_errors" => opts.fix_errors = value.downcast::<PyBool>()?.is_true(),
        "force" => opts.force = value.downcast::<PyBool>()?.is_true(),
        "filter" => opts.filter = value.extract::<Collection<RowFilter>>()?.remap(),
        "interlace" => opts.interlace = py_option_extract::<Interlacing, oxi::Interlacing>(value)?,
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
