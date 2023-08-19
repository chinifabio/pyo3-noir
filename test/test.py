from py_noir import *

def closure(a, b):
    return a + b

config = EnvironmentConfig.from_args()
env = StreamEnvironment(config)
src = IteratorSource(NoirIter([1.0, 2.0, 3.0, 4.0, 5.0]))
res = env.iterator_stream(src).reduce(closure).collect_vec()
env.execute()
print(res.get())
