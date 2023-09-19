from noir.noir import PyStreamOutput
from noir.datatype.noir_data import NoirData

class StreamOutput:
    inner = None
    
    def __init__(self, output: PyStreamOutput):
        self.inner = output
        
    def get_result(self):
        result = self.inner.get()
        print(result)
        if len(result) == 1 and not isinstance(result[0], list):
            return [NoirData(result)]
        else:
            return [NoirData(x) for x in result]