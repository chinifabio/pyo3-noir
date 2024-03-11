use noir_compute::data_type::stream_item::StreamItem;
use pyo3::{pyclass, pymethods, PyResult};

use super::noir_type::PyNoirType;

#[repr(transparent)]
#[pyclass]
#[derive(Debug)]
pub struct PyStreamItem(pub StreamItem);

#[pymethods]
impl PyStreamItem {
    #[new]
    pub fn new(data: Vec<PyNoirType>) -> Self {
        Self(StreamItem::new(data.into_iter().map(|x| x.0).collect()))
    }

    fn __repr__(&self) -> String {
        format!("{}", self.0)
    }

    #[getter]
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self.0))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }
}
