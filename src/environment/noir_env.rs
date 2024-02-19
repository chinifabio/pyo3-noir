use noir_compute::prelude::{IteratorSource, RowCsvSource};
use noir_compute::StreamEnvironment;

use pyo3::types::PyString;
use pyo3::{pyclass, pymethods, Python};

use crate::environment::config::PyEnvironmentConfig;
use crate::source::noir_source::{PyCsvSource, PyIteratorSource};
use crate::stream::noir_stream::PyStream;
use crate::stream::opt_stream::PyOptStream;
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

    pub fn iterator_stream(&mut self, source: PyIteratorSource) -> PyStream {
        let source = IteratorSource::new(source.iter.into_iter());

        let mut env = ENV_REGISTRY.lock().unwrap().remove(&0).unwrap();
        let stream = env.stream(source).into_box();
        ENV_REGISTRY.lock().unwrap().insert(0, env);

        PyStream::new(stream)
    }

    pub fn csv_stream(&mut self, path: &PyCsvSource) -> PyStream {
        let source = RowCsvSource::new(path.path.clone());

        let mut env = ENV_REGISTRY.lock().unwrap().remove(&0).unwrap();
        let stream = env.stream(source).into_box();
        ENV_REGISTRY.lock().unwrap().insert(0, env);

        PyStream::new(stream)
    }

    pub fn opt_stream(&mut self, path: &PyString) -> PyOptStream {
        let mut env = ENV_REGISTRY.lock().unwrap().remove(&0).unwrap();
        let stream = env.stream_csv_optimized(path.to_string());
        ENV_REGISTRY.lock().unwrap().insert(0, env);

        PyOptStream::new(stream)
    }

    pub fn execute(&mut self, py: Python) {
        let env = ENV_REGISTRY.lock().unwrap().remove(&0).unwrap();

        // Needs allow_threads because otherwise this method doesn't release the GIL when it's needed by the user's closures in reduce etc. creating a deadlock.
        py.allow_threads(|| env.execute_blocking());
    }
}
