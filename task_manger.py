import socket
import time
import loader
import threading
from encode import decode_data, encode, decode_size
from mylog import logging


class TaskManager():
    def __init__(self, ip, port, byteorder, message_head_size, out_queue, in_queue, heartbeat_inter, command, listen=5):
        '''
        args:
        '''
        self.soc = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.soc.bind((ip, port))
        self.soc.listen(listen)

        self.message_head_size = message_head_size
        self.byteorder = byteorder

        self.in_queue = in_queue
        self.out_queue = out_queue

        self.lock = threading.Lock()
        self.ticker = {}
        self.heartbeat_inter = heartbeat_inter

        self.command = command

    def start(self, ):
        logging.info("start task manager")
        threading.Thread(target=self.heartbeat).start()
        self.listen()

    def listen(self,):
        while True:
            try:
                client, addr = self.soc.accept()
                logging.info("task manager accept a request: %s", addr)
                thread = threading.Thread(target=self.message_handle, args=(
                    client,))
                thread.start()
            except:
                break

    def message_handle(self, client):
        while True:
            size_byte = client.recv(self.message_head_size)
            if(len(size_byte) == 0):
                logging.error("invalid message")
                break

            size = decode_size(size_byte)
            task_bytes = self.recv_all(client, size)
            task = decode_data(task_bytes)
            self.update_tiker(task)
            if task[1] == self.command.HEARTBEAT.value:
                break

            resp = self.transfer(task)
            size_byte, resp_byte = encode(resp)
            client.send(size_byte)
            client.send(resp_byte)

    def recv_all(self, client, size):
        packet_size = 1024
        task_bytes = b''
        while size > packet_size:
            data = client.recv(packet_size)
            task_bytes += data
            size -= packet_size
        task_bytes += client.recv(size)
        return task_bytes

    def transfer(self, task):
        self.out_queue.put(task)
        while True:
            name, resp = self.in_queue.get()
            if name == task[0]:
                break
            self.in_queue.put((name, resp))
        return resp

    def update_tiker(self, task):
        name = task[0]
        with self.lock:
            self.ticker[name] = time.time()

    def heartbeat(self, ):
        time.sleep(self.heartbeat_inter)
        with self.lock:
            for k in self.ticker.keys():
                if time.time()-self.ticker[k] > self.heartbeat_inter:
                    self.out_queue.put((k, self.command.DELETE))