from py_noir import noir

env = noir.StreamEnvironment()
src = noir.IteratorSource(noir.NoirIter([1.0, 2.0, 3.0, 4.0, 5.0]))
print(src.description()) 
