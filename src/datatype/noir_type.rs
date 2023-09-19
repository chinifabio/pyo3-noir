
use noir::data_type::NoirType;
use pyo3::{IntoPy, PyObject, Python, ToPyObject, FromPyObject, PyAny, PyResult};


#[repr(transparent)]
pub struct PyNoirType(pub NoirType);


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

impl FromPyObject<'_> for PyNoirType {
    fn extract(ob: &'_ PyAny) -> PyResult<Self> {
        let data = ob.extract::<f32>();
        if data.is_err() {
            let data = ob.extract::<i32>();
            if data.is_err() {
                return Ok(PyNoirType(NoirType::NaN()));
            }else{
                return Ok(PyNoirType(NoirType::Int32(data.unwrap())))
            }
        }else{
            Ok(PyNoirType(NoirType::Float32(data.unwrap())))
        }
    }
}
