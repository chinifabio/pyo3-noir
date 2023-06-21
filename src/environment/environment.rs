use noir::{StreamEnvironment};
use pyo3::{pyclass, pymethods};

use crate::{stream::stream::PyStream, source::source::PySource};

#[pyclass]
pub struct PyStreamEnvironment {
    pub env: StreamEnvironment,
}

#[pymethods]
impl PyStreamEnvironment {
    #[new]
    pub fn new_default() -> Self {
        Self {
            env: StreamEnvironment::default(),
        }
    }

    
    pub fn stream(&mut self, source: PySource) -> PyStream {
        PyStream {
            stream: self.env.stream(source.source),
        }
    }
    

    pub fn description(&self) -> String { 
        "Stream Environment with ".to_owned() + self.env.parallelism().to_string().as_str() + " cores"
    }
}