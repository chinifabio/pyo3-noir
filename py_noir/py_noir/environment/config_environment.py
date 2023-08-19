import sys
from pyo3_noir import PyEnvironmentConfig

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
