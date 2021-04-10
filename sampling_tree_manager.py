import time
import threading
from mylog import logging
from loader import Loader
from buffer_manager import BufferManger
from sampling_tree import SamplingTree


class SamplerTreeManager(object):
    def __init__(self, in_queue, out_queue, mmap_file_path, command, buffer_manager):
        self.mmap_file_path = mmap_file_path
        self.sampling_tree = SamplingTree()
        self.tree_lock = threading.Lock()
        self.command = command
        self.buffer_manager = buffer_manager
        self.in_queue = in_queue
        self.out_queue = out_queue
        # 当所有进程都已经sampling完毕，就block住，防止空转
        self.blocking_sampling = threading.Condition()

    def add_task(self, name, idx_list):
        head = self.buffer_manager.add_task(name)
        if head == -1:
            return head
        
        with self.tree_lock:
            self.sampling_tree.insert(idx_list, name)

        with self.blocking_sampling:
            self.blocking_sampling.notify()
        logging.info("add subsampler name %s", name)
        return head

    def delete_task(self, name):
        logging.info("sampler delete subs %s", name)
        self.buffer_manager.delete_task(name)
        with self.tree_lock:
            self.sampling_tree.remove(name)
        return 1
    
    def sampling_idx(self, ):
        while True:
            with self.tree_lock:
                idx_dict, expect_diff = self.sampling_tree.sampling()
            if len(idx_dict) == 0:
                with self.blocking_sampling:
                    logging.info("sampling idx blocking")
                    self.blocking_sampling.wait()
                logging.info("sampling idx resuming")
            
            for i in idx_dict.keys():
                logging.critical("sampler put idx %d", i)
                self.buffer_manager.write(i, idx_dict[i], expect_diff[i])
    
    def message_handle(self, task):
        name = task[0]
        cmd = task[1]

        logging.info("sampler receive a task : %s. %d", name, cmd)
        if (cmd == self.command.ADD.value):
            data = task[2]
            succ = self.add_task(name, data)
        elif(cmd == self.command.DELETE.value):
            succ = self.delete_task(name)
        else:
            logging.error("unknown command")
            exit(0)
        self.out_queue.put((name, (succ, self.buffer_manager.buffer_path)))

    def start(self):
        logging.info("start sampler")

        # start a thread to put index
        idx_sampler = threading.Thread(target=self.sampling_idx)
        idx_sampler.start()

        while True:
            try:
                task = self.in_queue.get(True)
                self.message_handle(task)
            except:
                print("sampler is exiting ......")
                return

# def test():
#     sa = Sampler()
#     sa.add_subsampler("aaa",list(range(100)))
#     sa.add_subsampler("bbb",list(range(100)))
#     idx_sampler = threading.Thread(target=sa.sampling_idx, args=())
#     idx_sampler.start()
#     idx_sampler.join()
# if __name__ == '__main__':
#     test()
    