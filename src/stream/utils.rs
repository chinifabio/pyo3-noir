use noir::data_type::NoirData;
use pyo3::{PyObject, Python, types::PyList};
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
        let arg_b = PyList::new(py,b.into_iter().map(PyNoirData));
        let arg_a = PyNoirData(a);
        a = lambda.call1(py, (arg_a, arg_b)).unwrap().extract::<PyNoirData>(py).unwrap().0;
        return a;
    })
}