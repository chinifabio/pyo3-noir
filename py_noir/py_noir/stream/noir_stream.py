from pyo3_noir import PyStream
from py_noir.output.stream_output import StreamOutput
from collections.abc import Callable

class Stream:
    inner = None
    
    def __init__(self, stream: PyStream):
            self.inner = stream
    
    def max(self) -> 'Stream':
        return self.inner.max()
    
    def reduce(self, func: Callable) -> 'Stream':
        return self.inner.reduce(func)
    
    def collect_vec(self) -> StreamOutput:
        return StreamOutput(self.inner.collect_vec())