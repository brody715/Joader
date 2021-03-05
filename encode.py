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

if __name__ == '__main__':
    data = list(range(100000))
    size, data_byte = encode(data)
    print(len(data_byte))
    d = decode_data(data_byte)
    print(len(d))