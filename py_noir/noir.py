
import pyo3_noir as noir
from pyo3_noir import PyStreamEnvironment, PyNoirIter, PySource

class StreamEnvironment:
    
    env = None
    
    def __init__(self):
        self.env = PyStreamEnvironment()
    
    def description(self):
        """ Return the description of the stream environment"""
        return self.env.description()
    
class NoirIter:

    iter = None

    def __init__(self, data: list):
        self.iter = PyNoirIter()
        for i in data:
            self.iter.push(i)
        
class IteratorSource:
    src = None
    
    def __init__(self, iter: NoirIter):
        self.src = noir.PySource(iter.iter)
        
    def description(self):
        return self.src.description()

def mul(a, b):
    return a*b

if __name__ == "__main__":
    noir.reduce(mul)
