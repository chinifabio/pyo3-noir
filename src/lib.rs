use noir::prelude::StreamOutput;
use noir::StreamEnvironment;
use noir::Stream;
use once_cell::sync::Lazy;
use pyo3::prelude::*;

mod datatype;
mod environment;
mod source;
mod stream;

use std::{collections::HashMap, sync::Mutex};

use environment::noir_env::PyStreamEnvironment;


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

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "noir")]
fn pyo3_noir(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyStreamEnvironment>()?;
    m.add_class::<source::noir_source::PyIteratorSource>()?;
    m.add_class::<source::noir_source::PyCsvSource>()?;
    m.add_class::<stream::noir_stream::PyStream>()?;
    m.add_class::<stream::output::PyStreamOutput>()?;
    m.add_class::<environment::config::PyEnvironmentConfig>()?;
    m.add_class::<datatype::noir_data::PyNoirData>()?;
    Ok(())
}
