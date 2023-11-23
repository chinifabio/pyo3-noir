from noir.noir import PyIteratorSource

class IteratorSource:
    """
    Source that consumes an iterator and emits all its elements into the stream.

    The iterator will be consumed only from one replica, therefore this source is not parallel.

    Parameters
    ----------
    iter: list
        The iterator to consume.
    """

    inner = None
    
    def __init__(self, iter: list):
        print(iter)
        self.inner = PyIteratorSource(iter)