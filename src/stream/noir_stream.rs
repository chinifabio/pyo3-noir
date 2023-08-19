use std::marker::PhantomData;

use noir::{box_op::BoxedOperator, data_type::NoirData, Stream};
use pyo3::{pyclass, pymethods, Python, PyObject};

use crate::{PyNoirHandle, STREAM_REGISTRY, datatype::noir_type::PyNoirData};

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

pub fn bynary_lamda(    
    lambda: &PyObject,
    a: NoirData,
    b: NoirData,
) -> NoirData {
    Python::with_gil(|py| {
        let args = (PyNoirData(a), PyNoirData(b));
        lambda.call1(py, args).unwrap().extract::<PyNoirData>(py).unwrap().0
    })
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

    pub fn reduce(&mut self, closure: PyObject) -> Self {
        let id = self.0.idx;
        let mut map = STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();
        
        let prova = move |a,b| bynary_lamda(&closure, a, b);

        map.insert(id, stream.reduce(prova).into_box());
        
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
