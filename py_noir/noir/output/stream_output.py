from pyo3_noir import PyStreamOutput


class StreamOutput:
    inner = None
    
    def __init__(self, output: PyStreamOutput):
        self.inner = output
        
    def get(self):
        self.inner.get()