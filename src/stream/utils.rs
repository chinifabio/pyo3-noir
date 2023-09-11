use noir::data_type::NoirData;
use pyo3::{PyObject, Python};
use crate::datatype::noir_type::PyNoirData;

pub fn binary_lamda(    
    lambda: &PyObject,
    a: NoirData,
    b: NoirData,
) -> NoirData {
    Python::with_gil(|py| {
        let args = (PyNoirData(a), PyNoirData(b));
        lambda.call1(py, args).unwrap().extract::<PyNoirData>(py).unwrap().0
    })
}

pub fn binary_batch_lamda(
    lambda: &PyObject,
    mut a: NoirData,
    b: Vec<NoirData>,
) -> NoirData {
    Python::with_gil(|py| {
        for i in b {
            let args = (PyNoirData(a), PyNoirData(i));
            a = lambda.call1(py, args).unwrap().extract::<PyNoirData>(py).unwrap().0;
        }
        return a;
    })
}