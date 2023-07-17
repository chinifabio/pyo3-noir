use noir::prelude::StreamOutput;
use noir::StreamEnvironment;
use noir::{data_type::NoirType, Stream};
use once_cell::sync::Lazy;
use pyo3::{prelude::*, types::PyTuple};

mod datatype;
mod environment;
mod source;
mod stream;

use std::{collections::HashMap, sync::Mutex};

use environment::noir_env::PyStreamEnvironment;

use crate::datatype::noir_type::PyNoirType;
use noir::box_op::BoxedOperator;
use noir::data_type::NoirData;

type MyStream = Stream<NoirData, BoxedOperator<NoirData>>;

static STREAM_REGISTRY: Lazy<Mutex<HashMap<usize, MyStream>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static ENV_REGISTRY: Lazy<Mutex<HashMap<usize, StreamEnvironment>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static OUT_REGISTRY: Lazy<Mutex<HashMap<usize, StreamOutput<Vec<NoirData>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
#[derive(Clone)]
pub struct PyNoirHandle<T> {
    idx: usize,
    _marker: std::marker::PhantomData<T>,
}

#[pyfunction]
pub fn reduce(py: Python, func: &PyAny) {
    let a = PyNoirType(NoirType::Float32(4.0f32));
    let b = PyNoirType(NoirType::Float32(3.0f32));

    let arg = PyTuple::new(py, &[a, b]);
    println!("callable? {}", func.is_callable());
    println!("result: {}", func.call1(arg).unwrap());
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_noir(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyStreamEnvironment>()?;
    m.add_class::<datatype::noir_type::PyNoirIter>()?;
    m.add_class::<source::noir_source::PySource>()?;
    m.add_class::<stream::noir_stream::PyStream>()?;
    m.add_class::<stream::output::PyStreamOutput>()?;
    m.add_class::<environment::config::PyEnvironmentConfig>()?;
    m.add_function(wrap_pyfunction!(reduce, m)?)?;
    Ok(())
}
