use noir::data_type::NoirType;
use pyo3::{pyclass, pymethods, IntoPy, Python, PyResult, PyObject, ToPyObject};


pub struct PyNoirType(pub NoirType);

impl IntoPy<PyObject> for PyNoirType {
    fn into_py(self, py: Python) -> PyObject{
        match self.0{
            NoirType::Float32(a) => a.into_py(py),
            _ => panic!("Not implemented yet"),
        }
    }
}

impl ToPyObject for PyNoirType {
    fn to_object(&self, py: Python) -> PyObject{
        match self.0{
            NoirType::Float32(a) => a.into_py(py),
            _ => panic!("Not implemented yet"),
        }
    }
}


#[pyclass]
#[derive(Clone)]
pub struct PyNoirIter {
    inner: Vec<NoirType>,
}

#[pymethods]
impl PyNoirIter {

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

impl Iterator for PyNoirIter {
    type Item = NoirType;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop()
    }
}
