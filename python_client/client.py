# import grpc
# import task_pb2
# import task_pb2_grpc

# channel = grpc.insecure_channel('0.0.0.0:5688')
# client = task_pb2_grpc.TaskStub(channel)
# request = task_pb2.CreateTaskRequest(keys = ["1", "2", "3"], weights = [1,2,3])
# resp = client.Create(request)
# print(resp)