from .stream import *
from .environment import *
from .stream.expressions import *
from .output import *
from .source import *
from .datatype import *

def stream_csv(path: str) -> Stream:
    return StreamEnvironment(EnvironmentConfig.default()).opt_stream(path)
