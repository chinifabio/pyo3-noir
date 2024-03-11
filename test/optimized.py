from noir import EnvironmentConfig, StreamEnvironment, col
import sys

config = EnvironmentConfig.default()
env = StreamEnvironment(config)
path_a = sys.argv[1]
path_b = sys.argv[2]
other = env.opt_stream(path_b)
res = env.opt_stream(path_a)\
    .join(other, left_on=col(0), right_on=col(0))\
    .filter(col(1) >= col(11))\
    .collect()
env.execute()
print(res)
