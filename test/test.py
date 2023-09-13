from noir import *

def closure(a, b):
    n_0 = a[0] - b[0]
    n_1 = a[1] + b[1]
    return [n_0, n_1]

config3 = EnvironmentConfig.default()
env3 = StreamEnvironment(config3)
csv_src3 = CsvSource("data.csv")
res3 = env3.csv_stream(csv_src3).reduce_batch(closure, 4).collect_vec()
env3.execute()
print("reduce_batch {}".format(res3.get()))