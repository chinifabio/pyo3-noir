from pyo3_noir import PyStreamEnvironment
from py_noir.environment.config_environment import EnvironmentConfig
from py_noir.source.csv_source import CsvSource
from py_noir.source.iterator_source import IteratorSource
from py_noir.stream.noir_stream import Stream

class StreamEnvironment:
    
    env = None
    
    def __init__(self, env_conf: EnvironmentConfig):
        self.env = PyStreamEnvironment(env_conf.config)
        
    def iterator_stream(self, src: IteratorSource):
        return Stream(self.env.iterator_stream(src.inner))
    
    def csv_stream(self, src: CsvSource):
        return Stream(self.env.csv_stream(src.inner))
    
    def execute(self):
        self.env.execute()

