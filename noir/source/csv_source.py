from noir.noir import PyCsvSource


class CsvSource:
    """
    Source that consumes a csv file and emits all its elements into the stream.

    Parameters
    ----------
    path: str
        The path of the csv file to consume.
    """
    
    inner: None

    def __init__(self, path: str):
        self.inner = PyCsvSource(path)