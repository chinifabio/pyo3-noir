from pyo3_noir import PyStreamEnvironment
from noir.environment.config_environment import EnvironmentConfig
from noir.source.csv_source import CsvSource
from noir.source.iterator_source import IteratorSource
from noir.stream.noir_stream import Stream

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

