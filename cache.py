import asyncio
from collections import OrderedDict
from decimal import Decimal
import fnmatch
from functools import wraps
import hashlib
import inspect
import re
from threading import RLock, Lock
import time
import sys
import logging

LOG_FORMAT = "[%(asctime)s]%(levelname)s : %(message)s"
logging.basicConfig(level=logging.DEBUG, format=LOG_FORMAT)

class Cache(object):
    def __init__(self, maxsize=None):
        if maxsize is None:
            self.maxsize = 256*1024*1024 #256MB
        self.data = {}
        self.index = {}
        
        self.rlock = RLock()
        self.wlock = Lock()

    def clear(self):
        """Clear all cache entries."""
        with self.wlock:
            self.data_list.clear()
            self.index.clear()

    def has(self, key):
        """
        Return whether cache key exists and hasn't expired.
        Returns:
            bool
        """
        with self.rlock:
            return key in self.data.keys()

    def size(self):
        with self.rlock:
            size = sys.getsizeof(self.data)
        return size

    def full(self):
        if self.maxsize == 0:
            return False
        return self.size() >= self.maxsize

    def get(self, key):
        with self.wlock:
            try:
                logging.info("cache get key: %d", key )
                value = self.data[key]
                self.index[key] -= 1
                if (self.index[key] <= 0):
                    logging.info("cache delete key: %d", key)
                    self.data.pop(key)
                return value
            except KeyError:
                return None

    def set(self, key, value):
        logging.info("cache set key: %d", key)
        assert(value is not None)
        if self.full():
            return False
        with self.wlock:
            self.data[key] = value


    def delete(self, key):
        try:
            self.data.pop(key)
        except KeyError:
            pass
    
    def merge_index(self, dic):
        for key in dic.keys():
            if key in self.index.keys():
                self.index[key] += dic[key]
            else:
                self.index[key] = dic[key]
