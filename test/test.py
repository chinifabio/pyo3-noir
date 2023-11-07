from noir import *

def closure(a: NoirData, b: NoirData):
    n_0 = a[0] - b[0]
    n_1 = a[1] + b[1]
    return [n_0, n_1]

config = EnvironmentConfig.from_args()
env = StreamEnvironment(config)
csv_src = CsvSource("data.csv")
res = env.csv_stream(csv_src).mean(True).collect_vec()
env.execute()
print(res.get_result()[0])