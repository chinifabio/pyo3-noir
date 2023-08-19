use pyo3::{pyclass, pymethods};

use crate::datatype::noir_type::PyNoirIter;

#[pyclass]
#[derive(Clone)]
pub struct PyIteratorSource {
    pub iter: PyNoirIter,
}

#[pymethods]
impl PyIteratorSource {
    #[new]
    pub fn new(iter: PyNoirIter) -> Self {
        Self { iter }
    }
}

#[pyclass]
pub struct PyCsvSource{
    pub path: String,
}

#[pymethods]
impl PyCsvSource{
    #[new]
    pub fn new(path: String) -> Self{
        Self{ path } 
    }
}

