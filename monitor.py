import time
import pickle
import threading
class Tiker():
    def __init__(self, name):
        self.times = []
        self.last_time = 0
        self.sum = 0
        self.name = name
    def start(self):
        self.last_time = time.time()

    def avg(self, l = -1):
        if len(self.times) == 0:
            return 0
        if l == -1:
            return self.sum/len(self.times)
        else:
            return sum(self.times[-l:])/l
    def end(self):
        self.times.append(time.time()-self.last_time)
        self.last_time = time.time()
        self.sum += self.times[-1]
        return self.times[-1]
    
    def print_avg(self, l=-1, freq=-1):
        if len(self.times) % freq ==0:
            print(self.name,self.avg(l))
    
class Monitor():
    def __init__(self):
        self.lock = threading.Lock()
        self.tiker_dict = {}
    def tiker(self, name):
        with self.lock:
            if name in self.tiker_dict.keys():
                return self.tiker_dict[name]
            t = Tiker(name)
            self.tiker_dict[name] = t
            return t
    def del_tiker(self, name):
        with self.lock:
            self.tiker_dict.pop(name)
    def save(self, path):
        with self.lock:
            with open(path, 'wb+') as f:
                pickle.dump(self.tiker_dict, f)

class TimeLine():
    def __init__(self, freq = 1000):
        self.timeline = {}
        self.freq = freq
    def add(self, data_id):
        
        if data_id not in self.timeline.keys():
            # if (len(self.timeline)+1)%self.freq == 0:
            #     self.show()
            self.timeline[data_id] = []
        self.timeline[data_id].append(time.time())
    def show(self):
        for key in self.timeline.keys():
            for i in range(len(self.timeline[key])):
                self.timeline[key][i] -= self.timeline[key][0]
        times = self.timeline.values()

        import matplotlib.pyplot as plt
        for arr in times:
            plt.scatter(range(len(arr)), arr)
        plt.savefig('./test/test2.jpg')
        time.sleep(1000)
m = Monitor()
t = TimeLine()