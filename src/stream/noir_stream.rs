use std::marker::PhantomData;

use noir::{box_op::BoxedOperator, data_type::NoirData, Stream};
use pyo3::{pyclass, pymethods};

use crate::{PyNoirHandle, STREAM_REGISTRY};

use super::output::PyStreamOutput;

type MyStream = Stream<NoirData, BoxedOperator<NoirData>>;
#[pyclass]
pub struct PyStream(PyNoirHandle<MyStream>);

impl PyStream {
    pub fn new(stream: MyStream) -> Self {
        let mut map = STREAM_REGISTRY.lock().unwrap();
        map.insert(0, stream);
        Self(PyNoirHandle {
            idx: 0,
            _marker: PhantomData,
        })
    }
}

#[pymethods]
impl PyStream {
    pub fn max(&mut self) -> Self {
        let id = self.0.idx;
        let mut map = STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();
        map.insert(id, stream.max_noir_data(true).into_box());
        PyStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    pub fn collect_vec(&mut self) -> PyStreamOutput {
        let id = self.0.idx;
        let mut map = STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();
        PyStreamOutput::new(stream.collect_vec())
    }
}
