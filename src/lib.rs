use noir_compute::data_type::stream_item::StreamItem;
use noir_compute::prelude::StreamOutput;
use noir_compute::OptStream;
use noir_compute::Stream;
use noir_compute::StreamEnvironment;
use once_cell::sync::Lazy;
use pyo3::prelude::*;

mod datatype;
mod environment;
mod source;
mod stream;

use std::{collections::HashMap, sync::Mutex};

use environment::noir_env::PyStreamEnvironment;

use noir_compute::box_op::BoxedOperator;
use noir_compute::data_type::noir_data::NoirData;

type MyStream = Stream<BoxedOperator<NoirData>>;

static STREAM_REGISTRY: Lazy<Mutex<HashMap<usize, MyStream>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static OPT_STREAM_REGISTRY: Lazy<Mutex<HashMap<usize, OptStream>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static ENV_REGISTRY: Lazy<Mutex<HashMap<usize, StreamEnvironment>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static OUT_REGISTRY: Lazy<Mutex<HashMap<usize, StreamOutput<Vec<NoirData>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static OPT_OUT_REGISTRY: Lazy<Mutex<HashMap<usize, StreamOutput<Vec<StreamItem>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

///
/// This is a handle to a Python object that is stored in the registry.
/// It is used to keep track of the object's index in the registry.
///
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
    m.add_class::<datatype::stream_item::PyStreamItem>()?;
    m.add_class::<stream::opt_stream::PyOptStream>()?;
    m.add_class::<stream::output::PyOptStreamOutput>()?;
    m.add_class::<stream::expressions::PyExpr>()?;
    m.add_function(wrap_pyfunction!(stream::expressions::py_col, m)?)?;
    m.add_function(wrap_pyfunction!(stream::expressions::py_float_lit, m)?)?;
    m.add_function(wrap_pyfunction!(stream::expressions::py_int_lit, m)?)?;
    m.add_function(wrap_pyfunction!(stream::expressions::py_bool_lit, m)?)?;
    m.add_function(wrap_pyfunction!(stream::expressions::py_sum, m)?)?;
    m.add_function(wrap_pyfunction!(stream::expressions::py_count, m)?)?;
    m.add_function(wrap_pyfunction!(stream::expressions::py_max, m)?)?;
    m.add_function(wrap_pyfunction!(stream::expressions::py_min, m)?)?;
    m.add_function(wrap_pyfunction!(stream::expressions::py_avg, m)?)?;
    Ok(())
}
