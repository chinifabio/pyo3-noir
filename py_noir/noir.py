
import pyo3_noir as noir
from pyo3_noir import PyStreamEnvironment, PyNoirIter, PyStream, PyStreamOutput

class StreamEnvironment:
    
    env = None
    
    def __init__(self):
        self.env = PyStreamEnvironment()
        
    def stream(self, src: 'IteratorSource'):
        return Stream(self.env.stream(src.inner))
    
    def execute(self):
        self.env.execute()

class Stream:
    inner = None
    
    def __init__(self, stream: PyStream):
            self.inner = stream
    
    def max(self):
        return self.inner.max()
    
    def collect_vec(self):
        return StreamOutput(self.inner.collect_vec())

class StreamOutput:
    inner = None
    
    def __init__(self, output: PyStreamOutput):
        self.inner = output
        
    def get(self):
        self.inner.get()

class NoirIter:

    inner = None

    def __init__(self, data: list):
        self.inner = PyNoirIter()
        for i in data:
            self.inner.push(i)
        
class IteratorSource:
    inner = None
    
    def __init__(self, iter: NoirIter):
        self.inner = noir.PySource(iter.inner)
    