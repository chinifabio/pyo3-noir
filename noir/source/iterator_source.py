from noir.noir import PyIteratorSource

class IteratorSource:
    inner = None
    
    def __init__(self, iter: list):
        print(iter)
        self.inner = PyIteratorSource(iter)