import pickle

def encode(data):
    return pickle.dumps(data)
def decode(data):
    return pickle.loads(data)