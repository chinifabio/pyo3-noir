from noir import EnvironmentConfig, StreamEnvironment, col
import sys

config = EnvironmentConfig.from_args()
env = StreamEnvironment(config)
env.spown_remote_workers()

path_a = sys.argv[3]
path_b = sys.argv[4]

other = env.opt_stream(path_b)
res = env.opt_stream(path_a)\
    .join(other, left_on=col(0), right_on=col(0))\
    .filter(col(1) >= col(11))\
    .collect()
env.execute()
