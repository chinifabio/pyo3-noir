use std::vec::Vec;
use noir::{StreamEnvironment, Stream, prelude::IteratorSource, data_type::NoirType};
use pyo3::prelude::*;

#[pyclass]
#[repr(transparent)]
struct PyStreamEnvironment {
    pub env: StreamEnvironment,
}

#[pyclass]
#[derive(Clone)]
struct NoirIter {
    inner: Vec<NoirType>,
}

#[pymethods]
impl NoirIter {

    #[new]
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
        }
    }

    pub fn push(&mut self, item: f32) {
        self.inner.push(
            NoirType::Float32(item)
        );
    }
    
}

impl Iterator for NoirIter {
    type Item = NoirType;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop()
    }
}

#[pyclass]
#[repr(transparent)]
struct PySource{
    pub source: IteratorSource<NoirType, NoirIter>,
}

#[pyclass]
#[repr(transparent)]
struct PyStream{
    pub stream: Stream<NoirType, IteratorSource<NoirType, NoirIter>>,
}

#[pymethods]
impl PyStream {
    pub fn description(&self) -> String {
        "Stream of NoirType".to_string()
    }    
}

#[pymethods]
impl PySource{
    #[new]
    pub fn new(iter: NoirIter) -> Self {
        Self {
            source: IteratorSource::new(iter),
        }
    }
}

#[pymethods]
impl PyStreamEnvironment {
    #[new]
    pub fn new_default() -> Self {
        Self {
            env: StreamEnvironment::default(),
        }
    }

    /*
    pub fn stream(&mut self, s: PySource) -> PyStream {
        PyStream {
            stream: self.env.stream(s.source),
        }
    }
    */

    pub fn description(&self) -> String { 
        "Stream Environment with ".to_owned() + self.env.parallelism().to_string().as_str() + " cores"
    }
}



/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_noir(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyStreamEnvironment>()?;
    m.add_class::<PyStream>()?;
    m.add_class::<PySource>()?;
    m.add_class::<NoirIter>()?;
    Ok(())
}