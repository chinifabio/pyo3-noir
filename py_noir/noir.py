
import pyo3_noir as noir
from pyo3_noir import PyStreamEnvironment, PySource, PyStream, NoirIter

class Stream:
    stream = None
    
    def __init__(self, stream):
        self.stream = stream
        
    def description(self):
        """ Return the description of the stream """
        return self.stream.description()

class Source:
    source = None
    
    def __init__(self, path: str):
        self.source = PySource(path)
    

class StreamEnvironment:
    
    env = None
    
    def __init__(self):
        self.env = PyStreamEnvironment()
    
    def description(self):
        """ Return the description of the stream environment"""
        return self.env.description()
    
    def stream(self, s: Source) -> Stream:
        """ Return a stream """
        return Stream(self.env.stream(s.source))
