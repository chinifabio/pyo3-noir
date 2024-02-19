from noir.noir import PyStreamItem

class StreamItem:
    def __init__(self, columns):
        self.inner = PyStreamItem(columns)

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

    