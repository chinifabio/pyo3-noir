use indexmap::IndexMap;
use noir::data_type::NoirType;
use once_cell::sync::Lazy;
use pyo3::{prelude::*, types::PyTuple};
use typemap_rev::{TypeMap, TypeMapKey};

mod stream;
mod datatype;
mod environment;
mod source;

use std::sync::Mutex;

use environment::environment::PyStreamEnvironment;

use crate::datatype::noir_type::PyNoirType;

static REGISTRY: Lazy<Mutex<TypeMap>> = Lazy::new(|| Mutex::new(TypeMap::new()));

#[derive(Clone)]
pub struct PyNoirHandle<T> {
    idx: usize,
    _marker: std::marker::PhantomData<T>,
}
  

impl<T: 'static + Sync + Send> TypeMapKey for PyNoirHandle<T> {
    type Value = IndexMap<usize, T>;
}

#[pyfunction]
pub fn reduce(py: Python, func: &PyAny) {
    let a = PyNoirType(NoirType::Float32(4.0f32));
    let b = PyNoirType(NoirType::Float32(3.0f32));

    let arg = PyTuple::new(py, &[a,b]);
    println!("callable? {}", func.is_callable());
    println!("result: {}", func.call1(arg).unwrap());
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_noir(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyStreamEnvironment>()?;
    m.add_class::<datatype::noir_type::PyNoirIter>()?;
    m.add_class::<source::source::PySource>()?;
    m.add_function(wrap_pyfunction!(reduce, m)?)?;
    Ok(())
}