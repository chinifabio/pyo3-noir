use noir::{Stream, data_type::NoirType, prelude::IteratorSource};
use pyo3::{pyclass, pymethods};

use crate::datatype::noir_type::PyNoirIter;

#[pyclass]
#[repr(transparent)]
struct PyStreamSelf{
    stream: Stream<NoirType, IteratorSource<NoirType, PyNoirIter>>
}

impl PyStreamSelf {
    pub fn new(stream: Stream<NoirType, IteratorSource<NoirType, PyNoirIter>>) -> Self {
        Self{
            stream: stream
        }
    }
}

#[pymethods]
impl PyStreamSelf{
    /*
    pub fn max(self) {
        self.stream.max_noir(true);
    }
    */
}