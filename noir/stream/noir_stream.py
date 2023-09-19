from noir.noir import PyStream
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

    def median(self, skip_nan: bool) -> 'Stream':
        return Stream(self.inner.median_exact(skip_nan))
    
    def mean(self, skip_nan: bool) -> 'Stream':
        return Stream(self.inner.mean(skip_nan))
    
    def max(self, skip_nan: bool) -> 'Stream':
        return Stream(self.inner.max(skip_nan))
    
    def reduce(self, func: Callable) -> 'Stream':
        return Stream(self.inner.reduce(func))
    
    def reduce_assoc(self, func: Callable) -> 'Stream':
        return Stream(self.inner.reduce_assoc(func))
    
    def reduce_batch(self, fn: Callable, batch_size: int) -> 'Stream':
        reduce = vectorize_fn(fn)
        return Stream(self.inner.reduce_batch(reduce, batch_size))
    
    def reduce_batch_assoc(self, func: Callable, local_batch_size: int, global_batch_size: int) -> 'Stream':
        return Stream(self.inner.reduce_batch_assoc(func, local_batch_size, global_batch_size))
    
    def collect_vec(self) -> StreamOutput:
        return StreamOutput(self.inner.collect_vec())