from noir import *
import sys

config = EnvironmentConfig.from_args()
env = StreamEnvironment(config)
csv_src = CsvSource(sys.argv[3])
res = env.csv_stream(csv_src).drop_none().mean(True).collect_vec()
env.execute()

print(res.get_result())

