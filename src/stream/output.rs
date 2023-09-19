use noir::{data_type::NoirData, prelude::StreamOutput};
use pyo3::{pyclass, pymethods};

use crate::{datatype::noir_data::PyNoirData, PyNoirHandle, OUT_REGISTRY};

#[pyclass]
pub struct PyStreamOutput(PyNoirHandle<StreamOutput<Vec<NoirData>>>);

impl PyStreamOutput {
    pub fn new(output: StreamOutput<Vec<NoirData>>) -> Self {
        let mut map = OUT_REGISTRY.lock().unwrap();
        map.insert(0, output);
        Self(PyNoirHandle {
            idx: 0,
            _marker: std::marker::PhantomData,
        })
    }
}

#[pymethods]
impl PyStreamOutput {
    pub fn get(&mut self) -> Vec<PyNoirData> {
        let id = self.0.idx;
        let mut map = OUT_REGISTRY.lock().unwrap();
        let output = map.remove(&id).unwrap();
        output.get().unwrap().into_iter().map(PyNoirData).collect()
    }
}
