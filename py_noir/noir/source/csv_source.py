from pyo3_noir import PyCsvSource


class CsvSource:
    inner: None

    def __init__(self, path: str):
        self.inner = PyCsvSource(path)