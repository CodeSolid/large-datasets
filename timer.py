import time

class Timer(object):
    """Allows setting up a timer as a context manager, for example:
       with Timer("My process"):
           # Code you want to time here
    """
    def __init__(self, name=None):
        self.name = name

    def __enter__(self):
        self.tstart = time.perf_counter()

    def __exit__(self, type, value, traceback):
        if self.name:
            print('[%s]' % self.name,)
        elapsed = time.perf_counter() - self.tstart
        print(f"Elapsed time: {elapsed:.2f} seconds")