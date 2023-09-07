use noir::prelude::{IteratorSource, CsvSource};
use noir::StreamEnvironment;

use pyo3::{pyclass, pymethods, Python};

use crate::environment::config::PyEnvironmentConfig;
use crate::source::noir_source::{PyIteratorSource, PyCsvSource};
use crate::stream::noir_stream::PyStream;
use crate::ENV_REGISTRY;

#[pyclass]
pub struct PyStreamEnvironment;

impl Default for PyStreamEnvironment {
    fn default() -> Self {
        Self::default()
    }
}

#[pymethods]
impl PyStreamEnvironment {
    #[new]
    pub fn new(config: PyEnvironmentConfig) -> Self {
        let mut map = ENV_REGISTRY.lock().unwrap();
        map.insert(0, StreamEnvironment::new(config.0));
        PyStreamEnvironment
    }

    #[staticmethod]
    pub fn default() -> Self {
        let mut map = ENV_REGISTRY.lock().unwrap();
        map.insert(0, StreamEnvironment::default());
        PyStreamEnvironment
    }

    pub fn iterator_stream(&mut self, source: &PyIteratorSource) -> PyStream {
        let source: IteratorSource<
            noir::data_type::NoirData,
            crate::datatype::noir_type::PyNoirIter,
        > = IteratorSource::new(source.iter.clone());
        let mut env = ENV_REGISTRY.lock().unwrap().remove(&0).unwrap();
        let stream = env.stream(source).into_box();
        ENV_REGISTRY.lock().unwrap().insert(0, env);
        PyStream::new(stream)
    }

    pub fn csv_stream(&mut self, path: &PyCsvSource) -> PyStream{
        let source = CsvSource::new(path.path.clone());
        let mut env = ENV_REGISTRY.lock().unwrap().remove(&0).unwrap();
        let stream = env.stream(source).into_box();
        ENV_REGISTRY.lock().unwrap().insert(0, env);
        PyStream::new(stream)
    }

    pub fn execute(&mut self, py: Python) {
        let env = ENV_REGISTRY.lock().unwrap().remove(&0).unwrap();
        py.allow_threads(|| env.execute_blocking());
    }
}
