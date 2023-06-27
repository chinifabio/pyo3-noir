use std::marker::PhantomData;

use indexmap::IndexMap;
use noir::prelude::IteratorSource;
use noir::{StreamEnvironment};
use pyo3::ffi::PyCFunction;
use pyo3::types::{PyFunction, PyModule};
use pyo3::{pyclass, pymethods, PyAny, PyResult};

use crate::stream::stream::PyStream;
use crate::{PyNoirHandle, REGISTRY};
use crate::source::source::PySource;

#[pyclass]
pub struct PyStreamEnvironment(PyNoirHandle<StreamEnvironment>);

#[pymethods]
impl PyStreamEnvironment {
    #[new]
    fn new() -> Self{
        let mut binding = REGISTRY.lock().unwrap();
        if !binding.contains_key::<PyNoirHandle<StreamEnvironment>>() {
            binding.insert::<PyNoirHandle<StreamEnvironment>>(IndexMap::with_capacity(1));
        }
        let map: &mut IndexMap<usize, StreamEnvironment> = binding.get_mut::<PyNoirHandle<StreamEnvironment>>().unwrap();
        map.insert(0, StreamEnvironment::default());
        Self(PyNoirHandle{idx: 0, _marker: PhantomData})
    }

    pub fn stream(&mut self, source: &PySource) -> PyStream{
        let id = self.0.idx;
        let mut binding = REGISTRY.lock().unwrap();
        let map_env: &mut IndexMap<usize, StreamEnvironment> = binding.get_mut::<PyNoirHandle<StreamEnvironment>>().unwrap();
        let env = map_env.get_mut(&id).unwrap();
        let source = IteratorSource::new(source.iter.clone());

        return PyStream::new(env.stream(source));
    }

    pub fn description(&mut self) -> String {
        let id = self.0.idx;
        let mut binding = REGISTRY.lock().unwrap();
        let map: &mut IndexMap<usize, StreamEnvironment> = binding.get_mut::<PyNoirHandle<StreamEnvironment>>().unwrap();
        let env = map.get(&id).unwrap();
        return "Stream Environment with Parallelism: ".to_string() + env.parallelism().to_string().as_str();
    }
}