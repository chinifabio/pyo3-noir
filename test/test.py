from py_noir import *

def closure(a, b):
    n_0 = a[0] - b[0]
    n_1 = a[1] + b[1]
    n_2 = a[2] + b[2]
    n_3 = a[3] - b[3]
    n_4 = a[4] + b[4]
    return [n_0, n_1, n_2, n_3, n_4]

config = EnvironmentConfig.from_args()
env = StreamEnvironment(config)
csv_src = CsvSource("test/test.csv")
res = env.csv_stream(csv_src).reduce(closure).collect_vec()
env.execute()
print(res.get())
