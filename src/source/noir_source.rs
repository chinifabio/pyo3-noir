use pyo3::{pyclass, pymethods};

use crate::datatype::noir_type::PyNoirIter;

#[pyclass]
#[derive(Clone)]
pub struct PySource {
    pub iter: PyNoirIter,
}

#[pymethods]
impl PySource {
    #[new]
    pub fn new(iter: PyNoirIter) -> Self {
        Self { iter }
    }
}
