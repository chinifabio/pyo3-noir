from noir import EnvironmentConfig, StreamEnvironment, col
from noir import max as noir_max
import sys
import time

config = EnvironmentConfig.default()
env = StreamEnvironment(config)
res = env.opt_stream(sys.argv[1])\
    .group_by(col(1) % 10)\
    .select(noir_max(col(0) + col(2)))\
    .collect()
env.execute()

# print(res.get_result())
for x in res.get_result():
    print(x)

