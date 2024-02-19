from noir.noir import PyOptStream
from noir.output.opt_output import OptStreamOutput
from noir.stream.expressions import Expr
from typing import List

class OptStream:
    inner = None
    def __init__(self, opt: PyOptStream):
        self.inner = opt

    def filter(self, expr: Expr):
        return OptStream(self.inner.filter(expr.inner))
    
    def group_by(self, expr: Expr | List[Expr]):
        if isinstance(expr, list):
            return OptStream(self.inner.group_by([x.inner for x in expr]))
        elif isinstance(expr, Expr):
            return OptStream(self.inner.group_by([expr.inner]))
        else:
            raise Exception("Invalid type for group_by")
    
    def select(self, expr: Expr | List[Expr]):
        if isinstance(expr, list):
            return OptStream(self.inner.select([x.inner for x in expr]))
        elif isinstance(expr, Expr):
            return OptStream(self.inner.select([expr.inner]))
        else:
            raise Exception("Invalid type for select")

    def collect(self):
        return OptStreamOutput(self.inner.collect())