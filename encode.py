import pickle

SIZE_CNT = 4
BYTE_ORDER = 'big'

def encode(data):
    data = pickle.dumps(data)
    size = len(data).to_bytes(SIZE_CNT, byteorder=BYTE_ORDER)
    return size, data

def decode_data(data):
    return pickle.loads(data)

def decode_size(size):
    return int.from_bytes(size, byteorder=BYTE_ORDER)