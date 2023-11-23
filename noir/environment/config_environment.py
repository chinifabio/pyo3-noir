import sys
from noir.noir import PyEnvironmentConfig

class EnvironmentConfig:
    """
    The runtime configuration of the environment,

    This configuration selects which runtime to use for this execution. The runtime is either local (i.e. parallelism is achieved only using threads), or remote (i.e. using both threads locally and remote workers).

    In a remote execution the current binary is copied using scp to a remote host and then executed using ssh. The configuration of the remote environment should be specified via a YAML configuration file.
    """

    config = None
    
    def __init__(self, config: PyEnvironmentConfig):
        self.config = config
    
    def default():
        """
        Create a local environment configuration with 8 threads.
        """
        return EnvironmentConfig(PyEnvironmentConfig.local(8))
    

    def from_args():
        """
        Create a local or remote environment configuration from command line arguments.
        -l n : Create a local environment with n threads.
        -r file : Create a remote environment with the given YAML file.
        """

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
