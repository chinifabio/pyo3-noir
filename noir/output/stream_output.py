from noir.noir import PyStreamOutput
from noir.datatype.noir_data import NoirData

class StreamOutput:
    """
    Class that contains the result of a stream.
    """

    inner = None
    
    def __init__(self, output: PyStreamOutput):
        self.inner = output
        
    def get_result(self):
        """
        Get the result of the stream.
        """
        
        result = self.inner.get()
        if len(result) == 1 and not isinstance(result[0], list):
            return [NoirData(result)]
        else:
            return [NoirData(x) for x in result]