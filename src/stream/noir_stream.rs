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

/**
 * Methods that will be exposed to Python.
 * All the methods should have the following structure:
 * 
 * 1. Remove the struct from the registry using the index in the handle.
 * 2. Perform preliminar operations if needed before calling the method on the stream. (e.g. create the closure from PyObject)
 * 3. Call the method on the stream and insert the new stream in the registry.
 * 4. Return a new handle.
 */
#[pymethods]
impl PyStream {
    pub fn median_exact(&mut self, skip_nan: bool) -> Self {
        let id = self.0.idx;
        let mut map = STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();
        
        map.insert(id, stream.quantile_exact(0.5, skip_nan).into_box());
        
        PyStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    pub fn drop_none(&mut self) -> Self {
        let id = self.0.idx;
        let mut map = STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();
        
        map.insert(id, stream.drop_none().into_box());
        
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

    pub fn min(&mut self, skip_nan: bool) -> Self {
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

        let func = move |a, b| binary_lamda(&closure, a, b);

        map.insert(id, stream.reduce(func).into_box());

        PyStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    /**
     * Used only for testing purposes.
     * Untill the Python closure needs the GIL it's not possible to perform any parallelism.
     */
    pub fn reduce_assoc(&mut self, closure: PyObject) -> Self {
        let id = self.0.idx;
        let mut map = STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();

        let func = move |a, b| binary_lamda(&closure, a, b);

        map.insert(id, stream.reduce_assoc(func).into_box());

        PyStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    pub fn reduce_batch(&mut self, closure: PyObject, batch_size: usize) -> Self {
        let id = self.0.idx;
        let mut map = STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();

        let func = move |a, b| binary_batch_lamda(&closure, a, b);

        map.insert(id, stream.reduce_batch(func, batch_size).into_box());

        PyStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    /**
     * Used only for testing purposes.
     * Untill the Python closure needs the GIL it's not possible to perform any parallelism.
     */
    pub fn reduce_batch_assoc(
        &mut self,
        closure: PyObject,
        local_batch_size: usize,
        global_batch_size: usize,
    ) -> Self {
        let id = self.0.idx;
        let mut map = STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();

        let func = move |a, b| binary_batch_lamda(&closure, a, b);

        map.insert(
            id,
            stream
                .reduce_batch_assoc(func, local_batch_size, global_batch_size)
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
