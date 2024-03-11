from noir.noir import PyStreamEnvironment
from noir.environment.config_environment import EnvironmentConfig
from noir.source.csv_source import CsvSource
from noir.source.iterator_source import IteratorSource
from noir.stream.noir_stream import Stream
from noir.stream.opt_stream import OptStream

class StreamEnvironment:
    """
    Streaming environment from which it's possible to register new streams and start the computation.

    This is the entrypoint for the library: construct an environment providing an EnvironmentConfig, then you can ask new streams providing the source from where to read from.

    Parameters
    ----------
    env_conf: EnvironmentConfig
        The configuration of the environment.
    """

    env = None
    

    def __init__(self, env_conf: EnvironmentConfig):
        self.env = PyStreamEnvironment(env_conf.config)
        
    def iterator_stream(self, src: IteratorSource):
        """
        Create a Stream starting from an iterator source.

        Parameters
        ----------
        src: IteratorSource
            The source from which to read the elements.
        """
        return Stream(self.env.iterator_stream(src.inner))
    
    def csv_stream(self, src: CsvSource):
        """
        Create a Stream starting from a CSV source.

        Parameters
        ----------
        src: CsvSource
            The source from which to read the elements.
        """
        return Stream(self.env.csv_stream(src.inner))
    
    def opt_stream(self, path: str):
        """
        Create an optimizalbe Stream starting from a file.

        Parameters
        ----------
        path: str
            The path to the file from which to read the elements.
        """
        return OptStream(self.env.opt_stream(path))
    
    def spown_remote_workers(self):
        """
        If a remote configuration was given, spowns the remote workers.
        """
        self.env.spown_remote_workers()
    
    def execute(self):
        """
        Execute the pipeline.
        """
        self.env.execute()

