use std::marker::PhantomData;

use noir_compute::{optimization::dsl::expressions::Expr, OptStream};
use pyo3::{pyclass, pymethods};

use crate::{PyNoirHandle, OPT_STREAM_REGISTRY};

use super::{expressions::PyExpr, output::PyOptStreamOutput};

#[pyclass]
pub struct PyOptStream(PyNoirHandle<OptStream>);

impl PyOptStream {
    pub fn new(stream: OptStream) -> Self {
        let mut map = OPT_STREAM_REGISTRY.lock().unwrap();
        map.insert(0, stream);
        Self(PyNoirHandle {
            idx: 0,
            _marker: PhantomData,
        })
    }
}

#[pymethods]
impl PyOptStream {
    pub fn filter(&mut self, expr: PyExpr) -> Self {
        let id = self.0.idx;
        let mut map = OPT_STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();

        map.insert(id, stream.filter(expr.0));

        PyOptStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    pub fn group_by(&mut self, expr: Vec<PyExpr>) -> Self {
        let id = self.0.idx;
        let mut map = OPT_STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();

        let expr: Vec<Expr> = expr.into_iter().map(|x| x.0).collect();
        map.insert(id, stream.group_by(expr.as_slice()));

        PyOptStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    pub fn select(&mut self, expr: Vec<PyExpr>) -> Self {
        let id = self.0.idx;
        let mut map = OPT_STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();

        let expr: Vec<Expr> = expr.into_iter().map(|x| x.0).collect();
        map.insert(id, stream.select(expr.as_slice()));

        PyOptStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    pub fn collect(&mut self) -> PyOptStreamOutput {
        let id = self.0.idx;
        let mut map = OPT_STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();

        PyOptStreamOutput::new(stream.collect_vec())
    }
}
