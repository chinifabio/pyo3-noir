from noir.noir import py_col, py_float_lit, py_int_lit, py_bool_lit, PyExpr, py_sum, py_count, py_max, py_min, py_avg
from typing import Union, TypeAlias

PythonLiteral = Union[int, float, bool]
IntoExpr: TypeAlias = Union[PythonLiteral, "Expr"]

def into_pyexpr(input: IntoExpr) -> PyExpr:
    if isinstance(input, int):
        return py_int_lit(input)
    elif isinstance(input, float):
        return py_float_lit(input)
    elif isinstance(input, bool):
        return py_bool_lit(input)
    elif isinstance(input, Expr):
        return input.inner
    else:
        raise ValueError(f"Cannot convert {input} to expression")

class Expr:
    inner = None
    def __init__(self, inner: IntoExpr):
        self.inner = inner

    def __add__(self, rhs: IntoExpr):
        other = into_pyexpr(rhs)
        return Expr(self.inner + other)
    
    def __sub__(self, rhs: IntoExpr):
        other = into_pyexpr(rhs)
        return Expr(self.inner - other)
    
    def __mul__(self, rhs: IntoExpr):
        other = into_pyexpr(rhs)
        return Expr(self.inner * other)
    
    def __truediv__(self, rhs: IntoExpr):
        other = into_pyexpr(rhs)
        return Expr(self.inner / other)
    
    def __mod__(self, rhs: int):
        other = into_pyexpr(rhs)
        return Expr(self.inner % other)
    
    def __eq__(self, other: IntoExpr):
        other = into_pyexpr(other)
        return Expr(self.inner == other)
    
    def __gt__(self, other):
        other = into_pyexpr(other)
        return Expr(self.inner > other)
    
    def __lt__(self, other):
        other = into_pyexpr(other)
        return Expr(self.inner < other)
    
    def __ge__(self, other):
        other = into_pyexpr(other)
        return Expr(self.inner >= other)
    
    def __le__(self, other):
        other = into_pyexpr(other)
        return Expr(self.inner <= other)
    
    def floor(self):
        return Expr(self.inner.floor())
    
    def ceil(self):
        return Expr(self.inner.ceil())
    
    def abs(self):
        return Expr(self.inner.abs())
    
    def sqrt(self):
        return Expr(self.inner.sqrt())
    
    def round(self):
        return Expr(self.inner.round())

def col(n: int) -> Expr:
    return Expr(py_col(n))

def lit(n: PythonLiteral) -> Expr:
    return Expr(into_pyexpr(n))

def sum(expr: Expr) -> Expr:
    expr = into_pyexpr(expr)
    return Expr(py_sum(expr))

def count(expr: Expr) -> Expr:
    expr = into_pyexpr(expr)
    return Expr(py_count(expr))

def max(expr: Expr) -> Expr:
    expr = into_pyexpr(expr)
    return Expr(py_max(expr))

def min(expr: Expr) -> Expr:
    expr = into_pyexpr(expr)
    return Expr(py_min(expr))

def avg(expr: Expr) -> Expr:
    expr = into_pyexpr(expr)
    return Expr(py_avg(expr))
