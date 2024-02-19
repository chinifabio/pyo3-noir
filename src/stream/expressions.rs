use noir_compute::optimization::dsl::expressions::{
    binary_expr, lit, max, sum, unary_expr, Expr, UnaryOp,
};
use noir_compute::prelude::BinaryOp;
use pyo3::basic::CompareOp;
use pyo3::prelude::*;

#[pyclass]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PyExpr(pub Expr);

impl From<PyExpr> for Expr {
    fn from(value: PyExpr) -> Self {
        value.0
    }
}

impl From<Expr> for PyExpr {
    fn from(value: Expr) -> Self {
        PyExpr(value)
    }
}

#[pymethods]
impl PyExpr {
    pub fn __add__(&self, rhs: Self) -> PyResult<Self> {
        Ok(binary_expr(self.0.clone(), BinaryOp::Plus, rhs.0).into())
    }

    pub fn __sub__(&self, rhs: Self) -> PyResult<Self> {
        Ok(binary_expr(self.0.clone(), BinaryOp::Minus, rhs.0).into())
    }

    pub fn __mul__(&self, rhs: Self) -> PyResult<Self> {
        Ok(binary_expr(self.0.clone(), BinaryOp::Multiply, rhs.0).into())
    }

    pub fn __truediv__(&self, rhs: Self) -> PyResult<Self> {
        Ok(binary_expr(self.0.clone(), BinaryOp::Divide, rhs.0).into())
    }

    pub fn __mod__(&self, rhs: Self) -> PyResult<Self> {
        Ok(binary_expr(self.0.clone(), BinaryOp::Mod, rhs.0).into())
    }

    pub fn __and__(&self, rhs: Self) -> PyResult<Self> {
        Ok(binary_expr(self.0.clone(), BinaryOp::And, rhs.0).into())
    }

    pub fn __or__(&self, rhs: Self) -> PyResult<Self> {
        Ok(binary_expr(self.0.clone(), BinaryOp::Or, rhs.0).into())
    }

    pub fn __xor__(&self, rhs: Self) -> PyResult<Self> {
        Ok(binary_expr(self.0.clone(), BinaryOp::Xor, rhs.0).into())
    }

    pub fn __richcmp__(&self, other: Self, op: CompareOp) -> PyResult<Self> {
        match op {
            CompareOp::Eq => Ok(binary_expr(self.0.clone(), BinaryOp::Eq, other.0).into()),
            CompareOp::Ne => Ok(binary_expr(self.0.clone(), BinaryOp::NotEq, other.0).into()),
            CompareOp::Lt => Ok(binary_expr(self.0.clone(), BinaryOp::Lt, other.0).into()),
            CompareOp::Le => Ok(binary_expr(self.0.clone(), BinaryOp::LtEq, other.0).into()),
            CompareOp::Gt => Ok(binary_expr(self.0.clone(), BinaryOp::Gt, other.0).into()),
            CompareOp::Ge => Ok(binary_expr(self.0.clone(), BinaryOp::GtEq, other.0).into()),
        }
    }

    pub fn floor(&self) -> PyResult<Self> {
        Ok(unary_expr(UnaryOp::Floor, self.0.clone()).into())
    }

    pub fn ceil(&self) -> PyResult<Self> {
        Ok(unary_expr(UnaryOp::Ceil, self.0.clone()).into())
    }

    pub fn abs(&self) -> PyResult<Self> {
        Ok(unary_expr(UnaryOp::Abs, self.0.clone()).into())
    }

    pub fn sqrt(&self) -> PyResult<Self> {
        Ok(unary_expr(UnaryOp::Sqrt, self.0.clone()).into())
    }

    pub fn round(&self) -> PyResult<Self> {
        Ok(unary_expr(UnaryOp::Round, self.0.clone()).into())
    }
}

#[pyfunction]
pub fn py_col(n: usize) -> PyExpr {
    PyExpr(Expr::NthColumn(n))
}

#[pyfunction]
pub fn py_float_lit(n: f32) -> PyExpr {
    PyExpr(lit(n))
}

#[pyfunction]
pub fn py_int_lit(n: i32) -> PyExpr {
    PyExpr(lit(n))
}

#[pyfunction]
pub fn py_bool_lit(n: bool) -> PyExpr {
    PyExpr(lit(n))
}

#[pyfunction]
pub fn py_sum(expr: PyExpr) -> PyExpr {
    PyExpr(sum(expr.0))
}

#[pyfunction]
pub fn py_max(expr: PyExpr) -> PyExpr {
    PyExpr(max(expr.0))
}
