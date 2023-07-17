
import sys
import pyo3_noir as noir
from pyo3_noir import PyStreamEnvironment, PyNoirIter, PyStream, PyStreamOutput, PyEnvironmentConfig

class EnvironmentConfig:
    config = None
    
    def __init__(self, config: PyEnvironmentConfig):
        self.config = config
    
    def from_args():
        option = sys.argv[1]
        argument = sys.argv[2]
        
        if option == "-l":
            try:
                num = int(argument)
                if num <= 0:
                    raise ValueError("Number of threads must be greater than 0.")
                else:
                    return EnvironmentConfig(PyEnvironmentConfig.local(num))
            except ValueError:
                raise ValueError("Number of threads must be an integer.")
        
        elif option == "-r":
            try:
                with open(argument, 'r'):
                    return EnvironmentConfig(PyEnvironmentConfig.remote(argument))
            except IOError:
                raise IOError("File does not exist.")
        
        else:
            raise ValueError("Invalid arguments.")

class StreamEnvironment:
    
    env = None
    
    def __init__(self, env_conf: EnvironmentConfig):
        self.env = PyStreamEnvironment(env_conf.config)
        
    def stream(self, src: 'IteratorSource'):
        return Stream(self.env.stream(src.inner))
    
    def execute(self):
        self.env.execute()

class Stream:
    inner = None
    
    def __init__(self, stream: PyStream):
            self.inner = stream
    
    def max(self):
        return self.inner.max()
    
    def collect_vec(self):
        return StreamOutput(self.inner.collect_vec())

class StreamOutput:
    inner = None
    
    def __init__(self, output: PyStreamOutput):
        self.inner = output
        
    def get(self):
        self.inner.get()

class NoirIter:

    inner = None

    def __init__(self, data: list):
        self.inner = PyNoirIter()
        for i in data:
            self.inner.push(i)
        
class IteratorSource:
    inner = None
    
    def __init__(self, iter: NoirIter):
        self.inner = noir.PySource(iter.inner)
    