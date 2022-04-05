import numpy as np
import proto.job_pb2 as job_pb2
import proto.job_pb2_grpc as job_pb2_grpc
import grpc
import sys
sys.path.append("./proto")


class Job(object):
    def __init__(self, ip, length: int, job_name: str, dataset_name: str, job_id):
        self.length = length
        self.job_id = job_id
        self.client = grpc.insecure_channel(
            ip, options=(('grpc.enable_http_proxy', 0),))
        self.job_name = job_name
        self.dataset_name = dataset_name

    @staticmethod
    def new(dataset_name: str, name: str, ip: str, nums: int = 1):
        channel = grpc.insecure_channel(
            ip, options=(('grpc.enable_http_proxy', 0),))
        client = job_pb2_grpc.JobSvcStub(channel)
        request = job_pb2.CreateJobRequest(
            dataset_name=dataset_name, name=name)
        resp = client.CreateJob(request)
        job_id = resp.job_id
        length = resp.length
        return Job(ip, length, name, dataset_name, job_id)

    def transform(self, data: job_pb2.Data):
        if data.ty == job_pb2.Data.UINT:
            return int.from_bytes(data.bs, 'big', signed=False)
        elif data.ty == job_pb2.Data.INT:
            return int.from_bytes(data.bs, 'big', signed=True)
        elif data.ty == job_pb2.Data.IMAGE:
            w = int.from_bytes(data.bs[-4:])
            h = int.from_bytes(data.bs[-8:-4])
            return np.array(data.bs[:-8]).reshape(-1, w, h)
        else:
            assert False

    def next(self):
        request = job_pb2.NextRequest(loader_id=self.loader_id)
        data_list = self.client.Next(request)
        res = []
        for data in data_list:
            res.append(self.transform(data))
        return res

    def len(self):
        return self.length

    def delete(self):
        request = job_pb2.DeleteJobRequest(
            dataset_name=self.dataset_name, name=self.name)
        resp = self.client.DeleteJob(request)
        return resp
