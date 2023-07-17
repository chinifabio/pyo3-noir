from py_noir import noir

env = noir.StreamEnvironment()
src = noir.IteratorSource(noir.NoirIter([1.0, 2.0, 8.0, 4.0, 5.0]))
res = env.stream(src).max().collect_vec()
env.execute()
print(res.get())
