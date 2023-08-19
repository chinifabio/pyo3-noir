use noir::data_type::{NoirData, NoirType};
use pyo3::{pyclass, pymethods, IntoPy, PyObject, Python, ToPyObject, FromPyObject, PyAny, PyResult};


#[repr(transparent)]
pub struct PyNoirType(pub NoirType);

#[repr(transparent)]
#[derive(Clone)]
pub struct PyNoirData(pub NoirData);

impl IntoPy<PyObject> for PyNoirType {
    fn into_py(self, py: Python) -> PyObject {
        match self.0 {
            NoirType::Float32(a) => a.into_py(py),
            NoirType::Int32(a) => a.into_py(py),
            NoirType::None() => py.None(),
            NoirType::NaN() => f32::NAN.into_py(py)
        }
    }
}

impl ToPyObject for PyNoirType {
    fn to_object(&self, py: Python) -> PyObject {
        match self.0 {
            NoirType::Float32(a) => a.into_py(py),
            NoirType::Int32(a) => a.into_py(py),
            NoirType::None() => py.None(),
            NoirType::NaN() => f32::NAN.into_py(py)
        }
    }
}

impl IntoPy<PyObject> for PyNoirData {
    fn into_py(self, py: Python) -> PyObject {
        match self.0 {
            NoirData::NoirType(a) => PyNoirType(a).into_py(py),
            NoirData::Row(a) => {
                let mut row = Vec::new();
                for i in a {
                    row.push(PyNoirType(i));
                }
                row.into_py(py)
            },
        }
    }
}

impl ToPyObject for PyNoirData {
    fn to_object(&self, py: Python) -> PyObject {
        match &self.0 {
            NoirData::NoirType(a) => PyNoirType(a.clone()).into_py(py),
            NoirData::Row(a) => {
                let mut row = Vec::new();
                for i in a {
                    row.push(PyNoirType(i.clone()));
                }
                row.into_py(py)
            },
        }
    }
}

impl FromPyObject<'_> for PyNoirData {
    fn extract(ob: &'_ PyAny) -> PyResult<Self> {
        let data = ob.extract::<PyNoirType>()?;
        Ok(PyNoirData(noir::data_type::NoirData::NoirType(data.0)))
    }
}

impl FromPyObject<'_> for PyNoirType {
    fn extract(ob: &'_ PyAny) -> PyResult<Self> {
        let data = ob.extract::<f32>()?;
        Ok(PyNoirType(NoirType::Float32(data)))
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyNoirIter {
    inner: Vec<NoirData>,
}

#[pymethods]
impl PyNoirIter {
    #[new]
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, item: f32) {
        self.inner.push(NoirData::NoirType(NoirType::Float32(item)));
    }
}

impl Default for PyNoirIter {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for PyNoirIter {
    type Item = NoirData;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop()
    }
}
