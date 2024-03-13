use noir_compute::{
    data_type::{noir_data::NoirData, stream_item::StreamItem},
    prelude::StreamOutput,
};
use pyo3::{pyclass, pymethods, PyResult};

use crate::{
    datatype::{noir_data::PyNoirData, stream_item::PyStreamItem},
    PyNoirHandle, OPT_OUT_REGISTRY, OUT_REGISTRY,
};

#[pyclass]
pub struct PyStreamOutput(PyNoirHandle<StreamOutput<Vec<NoirData>>>);

#[pyclass]
pub struct PyOptStreamOutput(PyNoirHandle<StreamOutput<Vec<StreamItem>>>);

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

impl PyOptStreamOutput {
    pub fn new(output: StreamOutput<Vec<StreamItem>>) -> Self {
        let mut map = OPT_OUT_REGISTRY.lock().unwrap();
        map.insert(0, output);
        Self(PyNoirHandle {
            idx: 0,
            _marker: std::marker::PhantomData,
        })
    }
}

#[pymethods]
impl PyOptStreamOutput {
    #[getter]
    pub fn get(&mut self) -> PyResult<Vec<PyStreamItem>> {
        let id = self.0.idx;
        let mut map = OPT_OUT_REGISTRY.lock().unwrap();
        let output = map.remove(&id).unwrap();

        Ok(output
            .get()
            .unwrap_or_default()
            .into_iter()
            .map(PyStreamItem)
            .collect())
    }
}
