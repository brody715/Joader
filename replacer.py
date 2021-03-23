import threading

class Replacer(object):
    def __init__(self):
        self.id_dict = {}
        self.id_list = []
        
        self.lock = threading.Lock()
    def update(self, data_id, expect_diff):
        with self.lock:
            self.id_dict[data_id] = expect_diff

    def delete(self, data_id):
        with self.lock:
            if data_id in self.id_dict.keys():
                del self.id_dict[data_id]

    def reset(self):
        with self.lock:
            self.id_list.clear()
    

    def next(self):
        with self.lock:
            if (len(self.id_list) == 0):
                self.id_list = sorted(self.id_dict.keys(), key=lambda x:self.id_dict[x], reverse=True)
            
            res = self.id_list.pop()
            while res not in self.id_dict.keys():
                res = self.next()
            return res



