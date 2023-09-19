from noir import *

def closure(a: NoirData, b: NoirData):
    n_0 = a[0] - b[0]
    n_1 = a[1] + b[1]
    return [n_0, n_1]

config3 = EnvironmentConfig.default()
env3 = StreamEnvironment(config3)
csv_src3 = CsvSource("data.csv")
iterator = [[1.0, 3.0], ["", 6.0], [7.0, 3.0]]
iterator2 = [1.0, 2.0, 3.0]
iter_sorce = IteratorSource(iterator2)
str = env3.iterator_stream(iter_sorce)
max = str.min(True)
res3 = max.collect_vec()
env3.execute()
print(res3.get_result()[0])