use noir::data_type::NoirType;
use pyo3::{pyclass, pymethods};


#[pyclass]
#[derive(Clone)]
pub struct NoirIter {
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
