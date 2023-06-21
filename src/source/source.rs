use noir::{prelude::{FileSource}};
use pyo3::{pyclass, pymethods};



#[pyclass]
#[derive(Clone)]
pub struct PySource{
    pub source: FileSource
}


#[pymethods]
impl PySource{
    #[new]
    pub fn new(path: String) -> Self {
        Self {
            source: FileSource::new(path),
        }
    }
}