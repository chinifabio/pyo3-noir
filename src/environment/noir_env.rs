use noir::prelude::IteratorSource;
use noir::StreamEnvironment;

use pyo3::{pyclass, pymethods};

use crate::source::noir_source::PySource;
use crate::stream::noir_stream::PyStream;
use crate::ENV_REGISTRY;

#[pyclass]
pub struct PyStreamEnvironment;

impl Default for PyStreamEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

#[pymethods]
impl PyStreamEnvironment {
    #[new]
    pub fn new() -> Self {
        let mut map = ENV_REGISTRY.lock().unwrap();
        map.insert(0, StreamEnvironment::default());
        PyStreamEnvironment
    }

    pub fn stream(&mut self, source: &PySource) -> PyStream {
        let source: IteratorSource<
            noir::data_type::NoirData,
            crate::datatype::noir_type::PyNoirIter,
        > = IteratorSource::new(source.iter.clone());
        let mut env = ENV_REGISTRY.lock().unwrap().remove(&0).unwrap();
        let stream = env.stream(source).into_box();
        ENV_REGISTRY.lock().unwrap().insert(0, env);
        PyStream::new(stream)
    }

    pub fn execute(&mut self) {
        let env = ENV_REGISTRY.lock().unwrap().remove(&0).unwrap();
        env.execute_blocking();
    }
}
