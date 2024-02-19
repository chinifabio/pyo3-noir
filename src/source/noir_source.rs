use crate::datatype::noir_data::PyNoirData;
use noir_compute::data_type::noir_data::NoirData;
use pyo3::{pyclass, pymethods};

#[pyclass]
#[derive(Clone)]
pub struct PyIteratorSource {
    pub iter: Vec<NoirData>,
}

#[pymethods]
impl PyIteratorSource {
    #[new]
    pub fn new(iter: Vec<PyNoirData>) -> Self {
        iter.iter().for_each(|x| println!("{:?}", x));
        let a = iter.into_iter().map(|x| x.0).collect();
        Self { iter: a }
    }
}

#[pyclass]
pub struct PyCsvSource {
    pub path: String,
}

#[pymethods]
impl PyCsvSource {
    #[new]
    pub fn new(path: String) -> Self {
        Self { path }
    }
}
