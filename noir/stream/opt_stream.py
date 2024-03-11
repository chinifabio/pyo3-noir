from noir.noir import PyOptStream
from noir.output.opt_output import OptStreamOutput
from noir.stream.expressions import Expr
from typing import List

def handle_expr(expr: Expr | List[Expr]):
    if isinstance(expr, list):
        return [x.inner for x in expr]
    elif isinstance(expr, Expr):
        return [expr.inner]
    else:
        raise Exception("Invalid type for expression")

class OptStream:
    """
    The Optimizable Stream is a chain of operators that work on a flow of StreamItem.

    Internally a stream is composed by a chain of blocks, each of which can be seen as a simpler stream with input and output types.

    A block is internally composed of a chain of operators, nested like the Iterator from std.

    The difference between the Stream and the OptStream is that the OptStream is optimized, you can customize those optimizations using specific methods.
    """
    inner = None
    def __init__(self, opt: PyOptStream):
        self.inner = opt

    def collect(self):
        return OptStreamOutput(self.inner.collect())

    def filter(self, expr: Expr):
        return OptStream(self.inner.filter(expr.inner))
    
    def group_by(self, expr: Expr | List[Expr]):
        return OptStream(self.inner.group_by(handle_expr(expr)))
    
    def select(self, expr: Expr | List[Expr]):
        return OptStream(self.inner.select(handle_expr(expr)))
    
    def mean(self, skip_na: bool = False):
        return OptStream(self.inner.mean(skip_na))
    
    def join(self, other: "OptStream", left_on: Expr | List[Expr], right_on: Expr | List[Expr]):
        return OptStream(self.inner.join(other.inner, handle_expr(left_on), handle_expr(right_on)))
    
    def with_compiled_expression(self, compile: bool):
        return OptStream(self.inner.with_compiled_expression(compile))
    
    def with_predicate_pushdown(self, pushdown: bool):
        return OptStream(self.inner.with_predicate_pushdown(pushdown))
    
    def with_projection_pushdown(self, pushdown: bool):
        return OptStream(self.inner.with_projection_pushdown(pushdown))
    
    def infer_schema(self):
        return OptStream(self.inner.infer_schema())