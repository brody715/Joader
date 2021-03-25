import random
import queue


class SamplingNode(object):
    def __init__(self, idx_list, name_dict={}, leaf_name=None, is_leaf=False):
        self.idx_list = []
        self.idx_list.extend(idx_list)
        self.idx_set = set(self.idx_list)

        self.length = len(idx_list)
        self.delta = 0
        self.is_leaf = is_leaf
        self.leaf_name = leaf_name

        self.name_dict = {}
        for name in name_dict:
            self.name_dict[name] = name_dict[name]
        if leaf_name is not None:
            self.name_dict[leaf_name] = self.length
            self.is_leaf = True

        self.left = None
        self.right = None

    def __len__(self, ):
        return self.length

    def contain(self, node):
        return len(self.intersect(node)) == len(node)

    def intersect(self, node):
        ins = []
        if node is None:
            return ins
        if len(node) > self.length:
            for idx in self.idx_list:
                if node.has(idx):
                    ins.append(idx)
        else:
            for idx in node.idx_list:
                if self.has(idx):
                    ins.append(idx)
        return ins

    def differ(self, node):
        if node is None:
            return
        for idx in self.idx_list:
            if node.has(idx):
                self.idx_set.remove(idx)
        
        self.idx_list = list(self.idx_set)
        diff = (self.length-len(self.idx_list))
        for name in self.name_dict:
            self.name_dict[name] -= diff
        self.length -= diff

    def has(self, idx):
        if idx in self.idx_set:
            return True
        return False
    
    def insert(self, node):
        if(node.contain(self)):
            new_root = self.add_name(node.leaf_name, node.length)
            node.differ(new_root)
            if len(node.intersect(new_root.left)) <= len(node.intersect(new_root.right)):
                if new_root.right is None:
                    new_root.right = node
                else:
                    new_root.right = new_root.right.insert(node)
            else:
                if new_root.left is None:
                    new_root.left = node
                else:
                    new_root.left = new_root.left.insert(node)
            
            return new_root
        else:
            ins = self.intersect(node)
            new_root = SamplingNode(ins, name_dict=self.name_dict)
            for name, length in node.name_dict.items():
                new_root.add_name(name, length)
            
            self.differ(new_root)
            node.differ(new_root)
            
            new_root.left = self
            new_root.right = node
            return new_root
    
    def add_name(self, name, length):
        if self.is_leaf:
            node = SamplingNode(self.idx_list, self.name_dict, is_leaf=False)
            self.differ(node)
            node = node.add_name(name, length)
            node.left = self
            return node
        else:
            self.name_dict[name] = length
        return self

    def remove(self, name):
        if name in self.name_dict.keys():
            del self.name_dict[name]
        else:
            return self
        
        if len(self.name_dict) == 0:
            return None
        
        if self.left is not None:
            self.left = self.left.remove(name)
        if self.right is not None:
            self.right = self.right.remove(name)

        return self
    
    def _process_predix(self, preidx_dict):
        del_idx = []
        myname_set = set(self.name_dict.keys())
        for item in preidx_dict.items():
            idx = item[0]

            if myname_set.issubset(item[1]):
                self.idx_list.append(idx)
                self.idx_set.add(idx)
                item[1].difference_update(myname_set)

            if len(item[1]) == 0:
                del_idx.append(idx)
        
        for idx in del_idx:
            del preidx_dict[idx]
        
    def _split_child(self, name_set):
        parent = set()
        child = set()
        common = self.length
        sorted_names = sorted(self.name_dict.items(), key=lambda item:item[1])
        for item in sorted_names:
            if item[1] == 0:
                continue
            name = item[0]
            if name in name_set:
                # 如果len(child) != 0, 说明之后的都需要加入child
                if len(child) == 0 and random.uniform(0.00001, 1) < common/item[1]:
                    common = item[1]
                    parent.add(name)
                else:
                    child.add(name)
        return parent, child

    def sample(self, preidx_dict, name_set):
        if len(preidx_dict) == 0 and len(name_set) == 0:
            print(preidx_dict)
            return self.name_dict, {}, {}

        parent, child = self._split_child(name_set)
        
        preidx = -1
        if len(parent) != 0:
            choice = random.choice(range(self.length))
            preidx = self.idx_list[choice]
            self.idx_set.remove(preidx)
            del self.idx_list[choice]
            
        self._process_predix(preidx_dict)
        self.length = len(self.idx_list)

        # 所有的集合都在此采样
        sampling_res = {}
        expectation = {}
        for name in parent:
            sampling_res[name] = preidx
            expectation[preidx] = len(self.name_dict)
        if preidx != -1:
            name_set = set(self.name_dict.keys())
            name_set.difference_update(parent)
            if len(name_set) != 0:
                preidx_dict[preidx] = name_set

        # 为了接下来的update_namedict, 所以一开始需要重置长度
        for name in self.name_dict:
            self.name_dict[name] = self.length
        # push to child
        if self.left is not None:
            left_namedict, left_res, left_expect = self.left.sample(preidx_dict, child)
            sampling_res.update(left_res)
            expectation = self._merge_expectation(expectation, left_expect)
            self.update_namedict(left_namedict)
        if self.right is not None:
            right_namedict, right_res, right_expect = self.right.sample(preidx_dict, child)
            expectation = self._merge_expectation(expectation, right_expect)
            sampling_res.update(right_res)
            self.update_namedict(right_namedict)
        return self.name_dict, sampling_res, expectation
    
    def _merge_expectation(self, exp1, exp2):
        for key in exp2:
            if key not in exp1.keys():
                exp1[key] = 0
            exp1[key] += exp2[key]
        return exp1
    
    def update_namedict(self, name_dict):
        for name in name_dict:
            self.name_dict[name] += name_dict[name]

    def __str__(self, ):
        res = "("
        
        res += str(self.name_dict)
        res += "**"+str(self.idx_list)
        res += ")"
        return res


class SamplingTree(object):
    def __init__(self):
        self.root = None

    def insert(self, idx_list, name):
        node = SamplingNode(idx_list, leaf_name=name)
        if self.root is None:
            self.root = node
        else:
            self.root = self.root.insert(node)

    def remove(self, name):
        if self.root is not None:
            self.root.remove(name)

    def sampling(self,):
        idx_dict = {}
        if self.root is None:
            return idx_dict, {}
        
        _, res, expectation = self.root.sample({}, set(self.root.name_dict.keys()))
        
        for name, idx in res.items():
            if idx not in idx_dict.keys():
                idx_dict[idx] = []
            idx_dict[idx].append(name)
        # print("------------------------")
        expect_diff = {}
        for idx in idx_dict.keys():
            expect_diff[idx] = expectation[idx]-len(idx_dict[idx])
        return idx_dict, expect_diff
    def rebalance(self, root):
        pass
    def _rebalance(self, root):
        pass


    def __str__(self, ):
        q=queue.Queue()
        if self.root == None:
            return ""
        q.put(self.root)
        q.put("\n")

        res=""
        while(q.qsize() > 1):
            s=q.get()
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
    t=SamplingTree()
    l=[4,2,3,1]
    # l = [3,2,1]
    name = []
    res = {}
    
    for i in range(len(l)):
        name.append(str(i))
    for i in range(len(l)):
        t.insert(list(range(l[i])), name[i])
    
    for i in range(max(l)):
        res.append(t.sampling())

if __name__ == '__main__':
    import time
    test()
    
