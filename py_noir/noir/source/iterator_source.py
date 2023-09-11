from pyo3_noir import PyNoirIter, PyIteratorSource

class NoirIter:

    inner = None

    def __init__(self, data: list):
        self.inner = PyNoirIter()
        for i in data:
            self.inner.push(i)
        
class IteratorSource:
    inner = None
    
    def __init__(self, iter: NoirIter):
        self.inner = PyIteratorSource(iter.inner)