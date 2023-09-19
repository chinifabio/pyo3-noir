use crate::datatype::noir_data::PyNoirData;
use noir::data_type::NoirData;
use pyo3::{types::PyList, PyObject, Python};

pub fn binary_lamda(lambda: &PyObject, a: NoirData, b: NoirData) -> NoirData {
    Python::with_gil(|py| {
        let args = (PyNoirData(a), PyNoirData(b));
        lambda
            .call1(py, args)
            .unwrap()
            .extract::<PyNoirData>(py)
            .unwrap()
            .0
    })
}

pub fn binary_batch_lamda(lambda: &PyObject, a: NoirData, b: Vec<NoirData>) -> NoirData {
    Python::with_gil(|py| {
        let arg_b = PyList::new(py, b.into_iter().map(PyNoirData));
        let arg_a = PyNoirData(a);
        lambda
            .call1(py, (arg_a, arg_b))
            .unwrap()
            .extract::<PyNoirData>(py)
            .unwrap()
            .0
    })
}
