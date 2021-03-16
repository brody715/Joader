import random
import queue


class SamplingNode(object):
    def __init__(self, idx_list, name_dict={}, leaf_name=None, is_leaf=False):
        self.idx_list = idx_list
        self.dict = {}
        for i in range(len(idx_list)):
            self.dict[idx_list[i]] = i

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
                del self.dict[idx]
        
        self.idx_list = list(self.dict.keys())
        diff = (self.length-len(self.idx_list))
        for name in self.name_dict:
            self.name_dict[name] -= diff
        self.length -= diff

    def has(self, idx):
        if idx in self.dict.keys():
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
            node = node.add_name(name, length)
            self.differ(node)
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
    
    def parent_sampling(self, preidx_dict):
        del_idx = []
        myname_set = set(self.name_dict.keys())
        for item in preidx_dict.items():
            idx = item[0]

            if myname_set.issubset(item[1]):
                self.idx_list.append(idx)
                item[1].difference_update(myname_set)

            if len(item[1]) == 0:
                del_idx.append(idx)
        
        for idx in del_idx:
            del preidx_dict[idx]
            
    def sampling(self, preidx_dict, name_set):
        # print("call", preidx_dict, name_set, "self ", self.name_dict, self.idx_list)
        if len(preidx_dict) == 0 and len(name_set) == 0:
            return self.name_dict, {}

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
        
        preidx = -1
        if len(parent) != 0:
            choice = random.choice(range(self.length))
            preidx = self.idx_list[choice]
            del self.idx_list[choice]
        
        self.parent_sampling(preidx_dict)
        self.length = len(self.idx_list)

        # 所有的集合都在此采样
        sampling_res = {}
        for name in parent:
            sampling_res[name] = preidx
        if preidx != -1:
            allset = set(self.name_dict.keys())
            allset.difference_update(parent)
            if len(allset) != 0:
                preidx_dict[preidx] = allset
    
        for name in self.name_dict:
            self.name_dict[name] = self.length

        if self.left is not None:
            left_namedict, left_res = self.left.sampling(preidx_dict, child)
            sampling_res.update(left_res)
            # print("left:", left_namedict)
            self.update_namedict(left_namedict)
        if self.right is not None:
            right_namedict, right_res = self.right.sampling(preidx_dict, child)
            sampling_res.update(right_res)
            # print("right:", right_namedict)
            self.update_namedict(right_namedict)
        
        # 如果是叶子节点，需要手段更新
        
                
        return self.name_dict, sampling_res
    
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
    def __init__(self, root=None):
        self.root = root

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
        _,res = self.root.sampling({}, set(self.root.name_dict.keys()))
        # print("------------------------")
        return res
    def rebalance(self, root):
        pass
    def _rebalance(self, root):
        pass


    def __str__(self, ):
        q=queue.Queue()
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
    l=range(1000000,1000010)
    # l = [3,2,1]
    name = []
    for i in l:
        name.append(str(i))
    
    for i in range(len(l)):
        t.insert(list(range(l[i])), name[i])
    # print(t)
    now = time.time()
    res = {}
    for i in range(len(l)):
        ans = t.sampling()
        for item in ans.items():
            if item[0] not in res.keys():
                res[item[0]] = []
            res[item[0]].append(item[1])
    t = time.time()-now
    for it in res.items():
        print(it)
        # assert(int(it[0]) == len(it[1]))
        print(it[0], len(it[1]))
    print(t)
if __name__ == '__main__':
    import time
    
    test()
    
