from noir.noir import PyOptStreamOutput

class OptStreamOutput:
    inner = None
    def __init__(self, output: PyOptStreamOutput):
        self.inner = output

    def get_result(self):
        result = self.inner.get()
        return result
    
    def __str__(self) -> str:
        items = self.inner.get
        return "\n".join([str(x) for x in items]) if items else "Empty"