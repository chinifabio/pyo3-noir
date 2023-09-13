from pyo3_noir import PyStream
from noir.output.stream_output import StreamOutput
from collections.abc import Callable
from dask.distributed import Client, LocalCluster

def vectorize_fn(fn: Callable, client: Client):
  def func(a, vec):
    for x in vec:
        a = fn(a, x)
    return a
  
  def func_dask(a, vec):
      return client.submit(func, a,vec ).result()
  
  return func_dask

class Stream:
    inner = None
    client = None
    
    def __init__(self, stream: PyStream):    
        self.inner = stream
    
    def max(self) -> 'Stream':
        return self.inner.max()
    
    def reduce(self, func: Callable) -> 'Stream':
        return self.inner.reduce(func)
    
    def reduce_assoc(self, func: Callable) -> 'Stream':
        return self.inner.reduce_assoc(func)
    
    def reduce_batch(self, fn: Callable, batch_size: int, client: Client) -> 'Stream':
        reduce = vectorize_fn(fn, client)
        return self.inner.reduce_batch(reduce, batch_size)
    
    def reduce_batch_assoc(self, func: Callable, local_batch_size: int, global_batch_size: int) -> 'Stream':
        return self.inner.reduce_batch_assoc(func, local_batch_size, global_batch_size)
    
    def collect_vec(self) -> StreamOutput:
        return StreamOutput(self.inner.collect_vec())