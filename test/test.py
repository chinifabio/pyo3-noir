from noir import *

def closure(a, b):
    n_0 = a[0] - b[0]
    n_1 = a[1] + b[1]
    return [n_0, n_1]

config1 = EnvironmentConfig.default()
env1 = StreamEnvironment(config1)
csv_src1 = CsvSource("data.csv")
res1 = env1.csv_stream(csv_src1).reduce(closure).collect_vec()
env1.execute()
print("reduce {}".format(res1.get()) )

config2 = EnvironmentConfig.default()
env2 = StreamEnvironment(config2)
csv_src2 = CsvSource("data.csv")
res2 = env2.csv_stream(csv_src2).reduce_assoc(closure).collect_vec()
env2.execute()
print("reduce_assoc {}".format(res2.get()))

config3 = EnvironmentConfig.default()
env3 = StreamEnvironment(config3)
csv_src3 = CsvSource("data.csv")
res3 = env3.csv_stream(csv_src3).reduce_batch(closure, 4).collect_vec()
env3.execute()
print("reduce_batch {}".format(res3.get()))

config4 = EnvironmentConfig.default()
env4 = StreamEnvironment(config4)
csv_src4 = CsvSource("data.csv")
res4 = env4.csv_stream(csv_src4).reduce_batch_assoc(closure, 4, 4).collect_vec()
env4.execute()
print("reduce_batch_assoc {}".format(res4.get()))