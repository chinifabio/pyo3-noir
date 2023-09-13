from pyo3_noir import PyStream
from noir.output.stream_output import StreamOutput
from collections.abc import Callable

def vectorize_fn(fn: Callable):
  def func(a, vec):
    for x in vec:
        a = fn(a, x)
    return a
  return func

class Stream:
    inner = None
    
    def __init__(self, stream: PyStream):
            self.inner = stream
    
    def max(self) -> 'Stream':
        return self.inner.max()
    
    def reduce(self, func: Callable) -> 'Stream':
        return self.inner.reduce(func)
    
    def reduce_assoc(self, func: Callable) -> 'Stream':
        return self.inner.reduce_assoc(func)
    
    def reduce_batch(self, fn: Callable, batch_size: int) -> 'Stream':
        reduce = vectorize_fn(fn)
        return self.inner.reduce_batch(reduce, batch_size)
    
    def reduce_batch_assoc(self, func: Callable, local_batch_size: int, global_batch_size: int) -> 'Stream':
        return self.inner.reduce_batch_assoc(func, local_batch_size, global_batch_size)
    
    def collect_vec(self) -> StreamOutput:
        return StreamOutput(self.inner.collect_vec())