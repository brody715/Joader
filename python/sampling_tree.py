import random
import queue
import time

class SamplingNode(object):
    def __init__(self, id_list, task_dict):
        self.id_list = id_list
        self.id_set = set(id_list)
        # self.cursor = 0

        self.task_dict = task_dict

        self.left = None
        self.right = None

    def intersection(self, id_list):
        return self.id_set.intersection(id_list)

    def difference(self, id_list):
        return self.id_set.difference(id_list)

    def intersection_update(self, id_list, update_taskdict=True):
        old_l = len(self.id_set)
        self.id_set.intersection_update(id_list)
        new_l = len(self.id_set)
        if update_taskdict:
            for key in self.task_dict:
                self.task_dict[key] += new_l - old_l
        self.id_list = list(self.id_set)

    def union_update(self, id_list, update_taskdict=True):
        old_l = len(self.id_set)
        self.id_set.update(id_list)
        new_l = len(self.id_set)
        if update_taskdict:
            for key in self.task_dict:
                self.task_dict[key] += new_l - old_l
        self.id_list = list(self.id_set)

    def difference_update(self, id_list, update_taskdict=True):
        old_l = len(self.id_set)
        self.id_set.difference_update(id_list)
        new_l = len(self.id_set)
        if update_taskdict:
            for key in self.task_dict:
                self.task_dict[key] += new_l - old_l
        self.id_list = list(self.id_set)

    def insert(self, node):
        if len(node) < min(self.task_dict.values()):
            return self.new_node(node)
        if self.left is None and self.right is None:
            return node.new_node(self)

        difference = self.difference(node.id_list)
        self.intersection_update(node.id_list, False)
        self.task_dict.update(node.task_dict)

        node.difference_update(self.id_set, True)
        self.left.union_update(difference, True)
        self.right.union_update(difference, True)
        self.right = self.right.insert(node)
        return self

    def new_node(self, node):
        task_dict = self.task_dict.copy()
        task_dict.update(node.task_dict)

        intersection = self.intersection(node.id_list)
        self.difference_update(intersection, True)
        node.difference_update(intersection, True)

        new_root = SamplingNode(list(intersection), task_dict)
        new_root.left = node
        new_root.right = self
        return new_root

    def random_choice(self):
        idx = random.choice(range(len(self.id_list)))
        e = self.id_list[idx]
        del self.id_list[idx]
        self.id_set.remove(e)
        return e

    def _min_task(self):
        min_value = 100000000
        for t in self.task_dict:
            if min_value > self.task_dict[t]:
                task = t
                min_value = self.task_dict[t]
        return task

    def _split(self):
        common = len(self.id_list)
        sorted_task = sorted(self.task_dict.keys(),
                             key=lambda k: self.task_dict[k])
        zero_end = 0
        for i in range(len(sorted_task)):
            task = sorted_task[i]
            if self.task_dict[task] == 0:
                zero_end += 1
                continue
            if random.uniform(0.0000001, 1) > common/self.task_dict[task]:
                parent = set(sorted_task[zero_end:i])
                child = set(sorted_task[i:])
                return parent, child
            common = self.task_dict[task]
        child = set()
        parent = set(sorted_task[zero_end:])
        return parent, child

    def _update_task(self):
        for task in self.task_dict:
            self.task_dict[task] = len(self.id_list)
        if self.left is not None:
            for task in self.left.task_dict:
                self.task_dict[task] += self.left.task_dict[task]
        if self.right is not None:
            for task in self.right.task_dict:
                self.task_dict[task] += self.right.task_dict[task]

    def sample(self, task_set, add_id_list):
        if len(self.task_dict) != len(task_set):
            res, exp = self.right.sample(task_set, add_id_list)
            self._update_task()
            return res, exp
        res = {}
        exp = {}
        pushdown = []
        parent, child = self._split()
        # print(parent, child, self.task_dict)
        if len(parent) != 0:
            e = self.random_choice()
            res[e] = parent
            pushdown.append(e)
            exp[e] = len(self.task_dict)
        if len(child) == len(self.task_dict):
            task = self._min_task()  # left child
            child.remove(task)
            res, exp = self.left.sample({task}, [])

        if len(child) != 0:
            sub_res, sub_exp = self.right.sample(child, pushdown)
            res.update(sub_res)
            exp.update(sub_exp)
        self.id_set.update(add_id_list)
        self.id_list.extend(add_id_list)
        self._update_task()
        return res, exp

    def remove(self, task):
        if task in self.task_dict:
            del self.task_dict[task]
        if len(self.task_dict) == 0:
            return None
        self.left = self.left.remove(task)
        self.right = self.right.remove(task)

        if self.left is None:
            self.right.union_update(self.id_list, True)
            return self.right
        if self.right is None:
            self.left.union_update(self.id_list, True)
            return self.left

    def __len__(self):
        return len(self.id_set)

    def __str__(self):
        if len(self.id_list) == 0:
            return "["+"]"+str(self.task_dict)
        return "["+str(self.id_list[0]) +"-"+ str(self.id_list[-1])+"]"+str(self.task_dict)


class SamplingTree(object):
    def __init__(self):
        self.root = None

    def insert(self, idx_list, task):
        node = SamplingNode(idx_list, {task: len(idx_list)})
        if self.root is None:
            self.root = node
        else:
            self.root = self.root.insert(node)

    def remove(self, name):
        if self.root is not None:
            self.root.remove(name)

    def sampling(self,):
        if self.root is None:
            return {}, {}

        id_dict, exp = self.root.sample(set(self.root.task_dict.keys()), [])

        exp_diff = {}
        for data_id in id_dict.keys():
            exp_diff[data_id] = exp[data_id]-len(id_dict[data_id])
        return id_dict, exp_diff

    def __str__(self, ):
        q = queue.Queue()
        if self.root == None:
            return ""
        q.put(self.root)
        q.put("\n")

        res = ""
        while(q.qsize() > 1):
            s = q.get()
            if type(s) == str:
                res += s
                if s == '\n':
                    q.put(s)
                continue

            res += str(s)
            q.put("<")
            if s.left is not None:
                q.put(s.left)
            q.put("|")
            if s.right is not None:
                q.put(s.right)
            q.put(">")

        return res


def test():
    t = SamplingTree()
    l = []
    for _ in range(8):
        l.append(random.randint(1000,10000))
    name = []
    res = {}

    for i in range(len(l)):
        name.append(str(l[i]))
        res[str(l[i])] = []

    for i in range(len(l)):
        t.insert(list(range(l[i])), name[i])
    # print(t)
    for i in range(max(l)):
        now = time.time()
        id_dict, _ = t.sampling()
        print(time.time()-now)
        exit(0)
        for data_id in id_dict:
            for name in id_dict[data_id]:
                res[name].append(data_id)

    for name in res:
        assert(len(res[name]) == int(name))


if __name__ == '__main__':

    for _ in range(1):
        test()
