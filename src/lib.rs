use pyo3::prelude::*;

mod stream;
mod datatype;
mod environment;
mod source;

use source::source::PySource;
use stream::stream::PyStream;
use datatype::noir_type::NoirIter;
use environment::environment::PyStreamEnvironment;



/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_noir(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyStreamEnvironment>()?;
    m.add_class::<PyStream>()?;
    m.add_class::<PySource>()?;
    m.add_class::<NoirIter>()?;
    Ok(())
}