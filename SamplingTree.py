import random
import queue


class SamplingNode(object):
    def __init__(self, idx_list, is_leaf, name=""):
        self.idx_list = idx_list

        self.length = len(idx_list)
        self.max_length = len(idx_list)
        self.min_length = len(idx_list)

        self.dict = {}
        for i in range(len(idx_list)):
            self.dict[idx_list[i]] = i

        self.is_leaf = is_leaf
        self.name_list = []
        if is_leaf:
            self.name_list.append(name)
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

        for idx in node.idx_list:
            if self.has(idx):
                del self.dict[idx]
        self.idx_list = list(self.dict.keys())
        diff = (self.length-len(self.idx_list))
        self.max_length -= diff
        self.min_length -= diff
        self.length -= diff

    def union(self, node):
        if node is None:
            return
        for idx in range(node.idx_list):
            if not self.has(idx):
                self.idx_list.append(idx)
                self.dict[idx] = len(self.idx_list)-1

        diff = (len(self.idx_list)-self.length)
        self.max_length += diff
        self.min_length += diff
        self.length += diff

    def has(self, idx):
        if idx in self.dict.keys():
            return True
        return False

    def update_left(self, left):
        if len(left) == 0:
            self.add_leaf(left.name_list)
            return
        self.left = left
        self.min_length = min(self.min_length, left.min_length+self.length)
        self.max_length = max(self.max_length, left.max_length+self.length)

    def update_right(self, right):
        if len(right) == 0:
            self.add_leaf(right.name_list)
            return
        self.right = right
        self.min_length = min(self.min_length, right.min_length+self.length)
        self.max_length = max(self.max_length, right.max_length+self.length)

    def random_choice(self, pre_idx):
        pass

    def add_leaf(self, name_list):
        self.is_leaf = True
        self.name_list.extend(name_list)

    def __str__(self, ):
        if self.is_leaf:
            res = "("*len(self.name_list)
        else:
            res = "["

        if len(self.idx_list) != 0:
            res += str(self.idx_list[0])+","
            res += str(self.idx_list[-1])

        if self.is_leaf:
            res += ")"*len(self.name_list)
        else:
            res += "]"
        return res
    


class SamplingTree(object):
    def __init__(self, ):
        self.root = None

    def insert(self, idx_list, name):
        node = SamplingNode(idx_list, True, name=name)
        self.root = self._insert(node, self.root)

    def _insert(self, node, root):
        if root != None and node.contain(root):
            node.differ(root)
            if root.left is None and root.right is None:
                root.update_left(self._insert(node, root.left))
            elif root.left is None:
                if len(root.right.intersect(node)) > 0:
                    root.update_right(self._insert(node, root.right))
                else:
                    root.update_left(self._insert(node, root.left))
            elif root.right is None:
                if len(root.left.intersect(node)) > 0:
                    root.update_left(self._insert(node, root.left))
                else:
                    root.update_right(self._insert(node, root.right))
            else:
                if len(node) < root.left.max_length or len(root.left.intersect(node)) >= len(root.right.intersect(node)):
                    root.update_left(self._insert(node, root.left))
                else:
                    root.update_right(self._insert(node, root.right))
            return root
        else:
            if root is None:
                return node

            ins=node.intersect(root)
            new_root=SamplingNode(ins, is_leaf=False)

            if node.max_length > root.max_length:
                root.differ(new_root)
                node.differ(new_root)
                new_root.update_left(root)
                new_root.update_right(node)
            elif node.min_length <= root.min_length:
                root.differ(new_root)
                node.differ(new_root)
                new_root.update_left(node)
                new_root.update_right(root)
            else:
                root.differ(new_root)
                if root.left is None and root.right is None:
                    root.left=root
                elif root.left is not None:
                    root.left.union(root)
                elif root.right is not None:
                    root.right.union(root)

                new_root.left=root.left
                new_root.right=root.right
                new_root=self._insert(node, new_root)

            return new_root

    def sampling(self,):
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
    l=[99, 1000001, 100, 101, 100000]
    for i in l:
        t.insert(list(range(i)), str(i))
    print(t)


if __name__ == '__main__':
    import time
    now = time.time()
    test()
    print(time.time()-now)
