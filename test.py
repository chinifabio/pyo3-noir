from py_noir import noir

env = noir.StreamEnvironment()
print(env.stream(noir.Source("test.txt")).description()) 
