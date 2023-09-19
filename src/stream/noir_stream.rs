use std::marker::PhantomData;

use noir::{box_op::BoxedOperator, data_type::NoirData, Stream};
use pyo3::{pyclass, pymethods, PyObject};

use crate::{PyNoirHandle, STREAM_REGISTRY};

use super::{
    output::PyStreamOutput,
    utils::{binary_batch_lamda, binary_lamda},
};

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
    pub fn median_exact(&mut self, skip_nan: bool) -> Self {
        let id = self.0.idx;
        let mut map = STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();
        map.insert(id, stream.median_noir_data(skip_nan).into_box());
        PyStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    pub fn mean(&mut self, skip_nan: bool) -> Self {
        let id = self.0.idx;
        let mut map = STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();
        map.insert(id, stream.mean_noir_data(skip_nan).into_box());
        PyStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    pub fn min(&mut self, skip_nan: bool) -> Self{
        let id = self.0.idx;
        let mut map = STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();
        map.insert(id, stream.min_noir_data(skip_nan).into_box());
        PyStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    pub fn max(&mut self, skip_nan: bool) -> Self {
        let id = self.0.idx;
        let mut map = STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();
        map.insert(id, stream.max_noir_data(skip_nan).into_box());
        PyStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    pub fn reduce(&mut self, closure: PyObject) -> Self {
        let id = self.0.idx;
        let mut map = STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();

        let prova = move |a, b| binary_lamda(&closure, a, b);

        map.insert(id, stream.reduce(prova).into_box());

        PyStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    pub fn reduce_assoc(&mut self, closure: PyObject) -> Self {
        let id = self.0.idx;
        let mut map = STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();

        let prova = move |a, b| binary_lamda(&closure, a, b);

        map.insert(id, stream.reduce_assoc(prova).into_box());

        PyStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    pub fn reduce_batch(&mut self, closure: PyObject, batch_size: usize) -> Self {
        let id = self.0.idx;
        let mut map = STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();

        let prova = move |a, b| binary_batch_lamda(&closure, a, b);

        map.insert(id, stream.reduce_batch(prova, batch_size).into_box());

        PyStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    pub fn reduce_batch_assoc(
        &mut self,
        closure: PyObject,
        local_batch_size: usize,
        global_batch_size: usize,
    ) -> Self {
        let id = self.0.idx;
        let mut map = STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();

        let prova = move |a, b| binary_batch_lamda(&closure, a, b);

        map.insert(
            id,
            stream
                .reduce_batch_assoc(prova, local_batch_size, global_batch_size)
                .into_box(),
        );

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
