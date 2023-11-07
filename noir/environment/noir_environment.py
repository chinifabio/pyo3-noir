from noir.noir import PyStreamEnvironment
from noir.environment.config_environment import EnvironmentConfig
from noir.source.csv_source import CsvSource
from noir.source.iterator_source import IteratorSource
from noir.stream.noir_stream import Stream

class StreamEnvironment:
    
    env = None
    
    """
    Streaming environment from which it's possible to register new streams and start the computation.

    This is the entrypoint for the library: construct an environment providing an EnvironmentConfig, then you can ask new streams providing the source from where to read from.
    """
    def __init__(self, env_conf: EnvironmentConfig):
        self.env = PyStreamEnvironment(env_conf.config)
        
    def iterator_stream(self, src: IteratorSource):
        return Stream(self.env.iterator_stream(src.inner))
    
    def csv_stream(self, src: CsvSource):
        return Stream(self.env.csv_stream(src.inner))
    
    def execute(self):
        self.env.execute()

