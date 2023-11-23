
from noir.noir import PyNoirData

class NoirData:
    """
    Class that represent a single data inside the stream.
    """

    def __init__(self, columns: list):
        self.inner = PyNoirData(columns)

    def __str__(self) -> str:
        return self.inner.__str__()
    
    def __repr__(self) -> str:
        return self.inner.__repr__()

    def len(self) -> int:
        """
        Return the number of columns of the data.
        """
        return self.inner.len()
    
    def is_empty(self) -> bool:
        """
        Return is there are no colums in the data.
        """
        return self.inner.is_empty()