use std::marker::PhantomData;

use noir_compute::{optimization::dsl::expressions::Expr, OptStream};
use pyo3::{pyclass, pymethods};

use crate::{PyNoirHandle, OPT_STREAM_REGISTRY};

use super::{expressions::PyExpr, output::PyOptStreamOutput};

#[pyclass]
pub struct PyOptStream(PyNoirHandle<OptStream>);

impl Clone for PyOptStream {
    fn clone(&self) -> Self {
        let id = self.0.idx;
        Self(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }
}

impl PyOptStream {
    pub fn new(stream: OptStream) -> Self {
        let mut stream_reg = OPT_STREAM_REGISTRY.lock().unwrap();
        let new_id = stream_reg.len();
        stream_reg.insert(new_id, stream);
        Self(PyNoirHandle {
            idx: new_id,
            _marker: PhantomData,
        })
    }
}

#[pymethods]
impl PyOptStream {
    pub fn collect(&mut self) -> PyOptStreamOutput {
        let id = self.0.idx;
        let mut map = OPT_STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();

        PyOptStreamOutput::new(stream.collect_vec())
    }

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

    pub fn mean(&mut self, skip_na: bool) -> Self {
        let id = self.0.idx;
        let mut map = OPT_STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();

        map.insert(id, stream.mean(skip_na));

        PyOptStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    pub fn join(
        &mut self,
        other: PyOptStream,
        left_on: Vec<PyExpr>,
        right_on: Vec<PyExpr>,
    ) -> Self {
        let id = self.0.idx;
        let mut map = OPT_STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();
        let other_id = other.0.idx;
        let other_stream = map.remove(&other_id).unwrap();

        let left_on: Vec<Expr> = left_on.into_iter().map(|x| x.0).collect();
        let right_on: Vec<Expr> = right_on.into_iter().map(|x| x.0).collect();
        map.insert(
            id,
            stream.join(other_stream, left_on.as_slice(), right_on.as_slice()),
        );

        PyOptStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    pub fn with_compiled_expression(&mut self, compile: bool) -> Self {
        let id = self.0.idx;
        let mut map = OPT_STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();

        map.insert(id, stream.with_compiled_expressions(compile));

        PyOptStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    pub fn with_predicate_pushdown(&mut self, pushdown: bool) -> Self {
        let id = self.0.idx;
        let mut map = OPT_STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();

        map.insert(id, stream.with_predicate_pushdown(pushdown));

        PyOptStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    pub fn with_projection_pushdown(&mut self, pushdown: bool) -> Self {
        let id = self.0.idx;
        let mut map = OPT_STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();

        map.insert(id, stream.with_projection_pushdown(pushdown));

        PyOptStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }

    pub fn infer_schema(&mut self) -> Self {
        let id = self.0.idx;
        let mut map = OPT_STREAM_REGISTRY.lock().unwrap();
        let stream = map.remove(&id).unwrap();

        map.insert(id, stream.infer_schema());

        PyOptStream(PyNoirHandle {
            idx: id,
            _marker: PhantomData,
        })
    }
}
