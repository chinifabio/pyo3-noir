use noir::{Stream, prelude::{FileSource}};
use pyo3::{pyclass, pymethods};



#[pyclass]
pub struct PyStream{
    pub stream: Stream<String, FileSource>,
}

#[pymethods]
impl PyStream {
    pub fn description(&self) -> String {
        "Stream of String frm file".to_string()
    }    

    pub fn reduce(&mut self, func: (String, String) -> String) {
        self.stream.reduce();
    }
}