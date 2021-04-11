import threading
import time


class Replacer(object):
    def __init__(self):
        self.diff_dict = {}
        self.id_dict = {}
        self.min_diff = 10000000
        self.cursor = -1

        self.lock = threading.Lock()

    def update(self, data_id, expect_diff):
        with self.lock:
            self.id_dict[data_id] = expect_diff
            if expect_diff not in self.diff_dict.keys():
                self.diff_dict[expect_diff] = []
                self.min_diff = min(self.min_diff, expect_diff)
            self.diff_dict[expect_diff].append(data_id)

    def has(self, data_id):
        with self.lock:
            return data_id in self.id_dict.keys()

    def delete(self):
        if self.min_diff in self.diff_dict.keys():
            del self.diff_dict[self.min_diff][self.cursor]
        if len(self.diff_dict[self.min_diff]) == 0:
            del self.diff_dict[self.min_diff]
        self.min_diff = min(self.diff_dict.keys())

    def _next(self):
        if len(self.id_dict) == 0 or len(self.diff_dict) == 0:
            return -1

        if self.cursor+1 >= len(self.diff_dict[self.min_diff]):
            self.cursor = -1

        self.cursor += 1
        data_id = self.diff_dict[self.min_diff][self.cursor]

        if data_id not in self.id_dict.keys() or self.id_dict[data_id] != self.min_diff:
            self.delete()
            return self._next()
        return data_id

    def next(self):
        with self.lock:
            return self._next()

    def pin(self, data_id):
        with self.lock:
            if data_id in self.id_dict.keys():
                del self.id_dict[data_id]


class RReplacer(object):
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
            self.id_list = list(self.id_dict.keys())

    def next(self):
        with self.lock:
            if (len(self.id_list) == 0):
                return -1
            res = self.id_list.pop()
            while res not in self.id_dict.keys():
                res = self.next()
        return res
