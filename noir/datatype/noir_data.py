
from noir.noir import PyNoirData

class NoirData:

    def __init__(self, columns: list):
        self.inner = PyNoirData(columns)

    def __str__(self) -> str:
        return self.inner.__str__()
    
    def __repr__(self) -> str:
        return self.inner.__repr__()

    def len(self) -> int:
        return self.inner.len()
    
    def is_empty(self) -> bool:
        return self.inner.is_empty()