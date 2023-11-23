from noir.noir import PyStream
from noir.output.stream_output import StreamOutput
from collections.abc import Callable

# Function used to vectorize the user's function when using batched methods.
def vectorize_fn(fn: Callable):
  def func(a, vec):
    for x in vec:
        a = fn(a, x)
    return a
  return func


class Stream:
    """
    A Stream represents a chain of operators that work on a flow of NoirData.

    Internally a stream is composed by a chain of blocks, each of which can be seen as a simpler stream with input and output types.

    A block is internally composed of a chain of operators, nested like the Iterator from std.
    """

    inner = None
    
    def __init__(self, stream: PyStream):
            self.inner = stream

    def drop_none(self) -> 'Stream':
        """
        Remove from the stream all the rows that containts a None value.
        """
        return Stream(self.inner.drop_none())


    def median(self, skip_nan: bool) -> 'Stream':
        """
        Compute the median of the columns of the stream.
        """
        return Stream(self.inner.median_exact(skip_nan))
    
    def mean(self, skip_nan: bool) -> 'Stream':
        """
        Compute the mean of the columns of the stream.
        """
        return Stream(self.inner.mean(skip_nan))
    
    def min(self, skip_nan: bool) -> 'Stream':
        """
        Find the min of the columns of the stream.
        """
        return Stream(self.inner.min(skip_nan))

    def max(self, skip_nan: bool) -> 'Stream':
        """
        Find the max of the columns of the stream.
        """
        return Stream(self.inner.max(skip_nan))
    
    def reduce(self, func: Callable) -> 'Stream':
        """
        Reduce the stream using the given function.
        """
        return Stream(self.inner.reduce(func))
    
    def reduce_assoc(self, func: Callable) -> 'Stream':
        """
        Reduce the stream using the given function.

        Note
        ----
        This function is for test only and until the user's funtion need GIL it should not be used.
        """
        return Stream(self.inner.reduce_assoc(func))
    
    def reduce_batch(self, fn: Callable, batch_size: int) -> 'Stream':
        """
        Reduce the stream using the given function.

        The data are send from Rust to Python in batches of size `batch_size`.
        """
        reduce = vectorize_fn(fn)
        return Stream(self.inner.reduce_batch(reduce, batch_size))
    
    def reduce_batch_assoc(self, func: Callable, local_batch_size: int, global_batch_size: int) -> 'Stream':
        """
        Reduce the stream using the given function.
        
        The data are send from Rust to Python in batches.

        Note
        ----
        This function is for test only and until the user's funtion need GIL it should not be used.
        """
        return Stream(self.inner.reduce_batch_assoc(func, local_batch_size, global_batch_size))
    
    def collect_vec(self) -> StreamOutput:
        """
        Collect the stream into a Vector.
        """
        return StreamOutput(self.inner.collect_vec())