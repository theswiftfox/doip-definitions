
use pyo3::prelude::*;

use crate::header::DoipHeader;

#[pymodule]
fn doip_definitions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DoipHeader>()?;
    Ok(())
}
