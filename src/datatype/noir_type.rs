use noir_compute::data_type::noir_type::NoirType;
use pyo3::{FromPyObject, IntoPy, PyAny, PyObject, PyResult, Python, ToPyObject};

#[repr(transparent)]
pub struct PyNoirType(pub NoirType);

impl IntoPy<PyObject> for PyNoirType {
    fn into_py(self, py: Python) -> PyObject {
        match self.0 {
            NoirType::Float32(a) => a.into_py(py),
            NoirType::Int32(a) => a.into_py(py),
            NoirType::None() => py.None(),
            NoirType::NaN() => f32::NAN.into_py(py),
            NoirType::Bool(a) => a.into_py(py),
        }
    }
}

impl ToPyObject for PyNoirType {
    fn to_object(&self, py: Python) -> PyObject {
        match self.0 {
            NoirType::Float32(a) => a.into_py(py),
            NoirType::Int32(a) => a.into_py(py),
            NoirType::None() => py.None(),
            NoirType::NaN() => f32::NAN.into_py(py),
            NoirType::Bool(a) => a.into_py(py),
        }
    }
}

impl FromPyObject<'_> for PyNoirType {
    fn extract(ob: &'_ PyAny) -> PyResult<Self> {
        let data = ob.extract::<f32>();
        if let Ok(data) = data {
            Ok(PyNoirType(NoirType::Float32(data)))
        } else {
            let data = ob.extract::<i32>();
            if let Ok(data) = data {
                Ok(PyNoirType(NoirType::Int32(data)))
            } else {
                Ok(PyNoirType(NoirType::NaN()))
            }
        }
    }
}
