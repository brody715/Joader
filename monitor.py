import time
import pickle

class Tiker():
    def __init__(self):
        self.times = []
        self.last_time = 0
    def start(self):
        self.last_time = time.time()
    def end(self):
        self.times.append(time.time()-self.last_time)
        self.last_time = time.time()
        return self.times[-1]

class Monitor():
    def __init__(self):
        self.tiker_dict = {}
    def tiker(self, name):
        if name in self.tiker_dict.keys():
            return self.tiker_dict[name]
        t = Tiker()
        self.tiker_dict[name] = t
        return t
    def del_tiker(self, name):
        self.tiker_dict.pop(name)
    def save(self, path):
        with open(path, 'wb+') as f:
            pickle.dump(self.tiker_dict, f)

m = Monitor()