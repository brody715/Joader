# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: job.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='job.proto',
  package='job',
  syntax='proto3',
  serialized_options=None,
  create_key=_descriptor._internal_create_key,
  serialized_pb=b'\n\tjob.proto\x12\x03job\"^\n\x04\x44\x61ta\x12\n\n\x02\x62s\x18\x01 \x01(\x0c\x12\x1f\n\x02ty\x18\x02 \x01(\x0e\x32\x13.job.Data.data_type\")\n\tdata_type\x12\x08\n\x04UINT\x10\x00\x12\x07\n\x03INT\x10\x01\x12\t\n\x05IMAGE\x10\x02\"6\n\x10\x43reateJobRequest\x12\x0c\n\x04name\x18\x01 \x01(\t\x12\x14\n\x0c\x64\x61taset_name\x18\x02 \x01(\t\"3\n\x11\x43reateJobResponse\x12\x0e\n\x06length\x18\x01 \x01(\x04\x12\x0e\n\x06job_id\x18\x03 \x01(\x04\"\x1d\n\x0bNextRequest\x12\x0e\n\x06job_id\x18\x01 \x01(\x04\"\'\n\x0cNextResponse\x12\x17\n\x04\x64\x61ta\x18\x01 \x03(\x0b\x32\t.job.Data\"6\n\x10\x44\x65leteJobRequest\x12\x0c\n\x04name\x18\x01 \x01(\t\x12\x14\n\x0c\x64\x61taset_name\x18\x02 \x01(\t\"\x13\n\x11\x44\x65leteJobResponse2\xad\x01\n\x06JobSvc\x12:\n\tCreateJob\x12\x15.job.CreateJobRequest\x1a\x16.job.CreateJobResponse\x12+\n\x04Next\x12\x10.job.NextRequest\x1a\x11.job.NextResponse\x12:\n\tDeleteJob\x12\x15.job.DeleteJobRequest\x1a\x16.job.DeleteJobResponseb\x06proto3'
)



_DATA_DATA_TYPE = _descriptor.EnumDescriptor(
  name='data_type',
  full_name='job.Data.data_type',
  filename=None,
  file=DESCRIPTOR,
  create_key=_descriptor._internal_create_key,
  values=[
    _descriptor.EnumValueDescriptor(
      name='UINT', index=0, number=0,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='INT', index=1, number=1,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='IMAGE', index=2, number=2,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=71,
  serialized_end=112,
)
_sym_db.RegisterEnumDescriptor(_DATA_DATA_TYPE)


_DATA = _descriptor.Descriptor(
  name='Data',
  full_name='job.Data',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='bs', full_name='job.Data.bs', index=0,
      number=1, type=12, cpp_type=9, label=1,
      has_default_value=False, default_value=b"",
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='ty', full_name='job.Data.ty', index=1,
      number=2, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
    _DATA_DATA_TYPE,
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=18,
  serialized_end=112,
)


_CREATEJOBREQUEST = _descriptor.Descriptor(
  name='CreateJobRequest',
  full_name='job.CreateJobRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='name', full_name='job.CreateJobRequest.name', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='dataset_name', full_name='job.CreateJobRequest.dataset_name', index=1,
      number=2, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=114,
  serialized_end=168,
)


_CREATEJOBRESPONSE = _descriptor.Descriptor(
  name='CreateJobResponse',
  full_name='job.CreateJobResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='length', full_name='job.CreateJobResponse.length', index=0,
      number=1, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='job_id', full_name='job.CreateJobResponse.job_id', index=1,
      number=3, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=170,
  serialized_end=221,
)


_NEXTREQUEST = _descriptor.Descriptor(
  name='NextRequest',
  full_name='job.NextRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='job_id', full_name='job.NextRequest.job_id', index=0,
      number=1, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=223,
  serialized_end=252,
)


_NEXTRESPONSE = _descriptor.Descriptor(
  name='NextResponse',
  full_name='job.NextResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='data', full_name='job.NextResponse.data', index=0,
      number=1, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=254,
  serialized_end=293,
)


_DELETEJOBREQUEST = _descriptor.Descriptor(
  name='DeleteJobRequest',
  full_name='job.DeleteJobRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='name', full_name='job.DeleteJobRequest.name', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='dataset_name', full_name='job.DeleteJobRequest.dataset_name', index=1,
      number=2, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=295,
  serialized_end=349,
)


_DELETEJOBRESPONSE = _descriptor.Descriptor(
  name='DeleteJobResponse',
  full_name='job.DeleteJobResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=351,
  serialized_end=370,
)

_DATA.fields_by_name['ty'].enum_type = _DATA_DATA_TYPE
_DATA_DATA_TYPE.containing_type = _DATA
_NEXTRESPONSE.fields_by_name['data'].message_type = _DATA
DESCRIPTOR.message_types_by_name['Data'] = _DATA
DESCRIPTOR.message_types_by_name['CreateJobRequest'] = _CREATEJOBREQUEST
DESCRIPTOR.message_types_by_name['CreateJobResponse'] = _CREATEJOBRESPONSE
DESCRIPTOR.message_types_by_name['NextRequest'] = _NEXTREQUEST
DESCRIPTOR.message_types_by_name['NextResponse'] = _NEXTRESPONSE
DESCRIPTOR.message_types_by_name['DeleteJobRequest'] = _DELETEJOBREQUEST
DESCRIPTOR.message_types_by_name['DeleteJobResponse'] = _DELETEJOBRESPONSE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

Data = _reflection.GeneratedProtocolMessageType('Data', (_message.Message,), {
  'DESCRIPTOR' : _DATA,
  '__module__' : 'job_pb2'
  # @@protoc_insertion_point(class_scope:job.Data)
  })
_sym_db.RegisterMessage(Data)

CreateJobRequest = _reflection.GeneratedProtocolMessageType('CreateJobRequest', (_message.Message,), {
  'DESCRIPTOR' : _CREATEJOBREQUEST,
  '__module__' : 'job_pb2'
  # @@protoc_insertion_point(class_scope:job.CreateJobRequest)
  })
_sym_db.RegisterMessage(CreateJobRequest)

CreateJobResponse = _reflection.GeneratedProtocolMessageType('CreateJobResponse', (_message.Message,), {
  'DESCRIPTOR' : _CREATEJOBRESPONSE,
  '__module__' : 'job_pb2'
  # @@protoc_insertion_point(class_scope:job.CreateJobResponse)
  })
_sym_db.RegisterMessage(CreateJobResponse)

NextRequest = _reflection.GeneratedProtocolMessageType('NextRequest', (_message.Message,), {
  'DESCRIPTOR' : _NEXTREQUEST,
  '__module__' : 'job_pb2'
  # @@protoc_insertion_point(class_scope:job.NextRequest)
  })
_sym_db.RegisterMessage(NextRequest)

NextResponse = _reflection.GeneratedProtocolMessageType('NextResponse', (_message.Message,), {
  'DESCRIPTOR' : _NEXTRESPONSE,
  '__module__' : 'job_pb2'
  # @@protoc_insertion_point(class_scope:job.NextResponse)
  })
_sym_db.RegisterMessage(NextResponse)

DeleteJobRequest = _reflection.GeneratedProtocolMessageType('DeleteJobRequest', (_message.Message,), {
  'DESCRIPTOR' : _DELETEJOBREQUEST,
  '__module__' : 'job_pb2'
  # @@protoc_insertion_point(class_scope:job.DeleteJobRequest)
  })
_sym_db.RegisterMessage(DeleteJobRequest)

DeleteJobResponse = _reflection.GeneratedProtocolMessageType('DeleteJobResponse', (_message.Message,), {
  'DESCRIPTOR' : _DELETEJOBRESPONSE,
  '__module__' : 'job_pb2'
  # @@protoc_insertion_point(class_scope:job.DeleteJobResponse)
  })
_sym_db.RegisterMessage(DeleteJobResponse)



_JOBSVC = _descriptor.ServiceDescriptor(
  name='JobSvc',
  full_name='job.JobSvc',
  file=DESCRIPTOR,
  index=0,
  serialized_options=None,
  create_key=_descriptor._internal_create_key,
  serialized_start=373,
  serialized_end=546,
  methods=[
  _descriptor.MethodDescriptor(
    name='CreateJob',
    full_name='job.JobSvc.CreateJob',
    index=0,
    containing_service=None,
    input_type=_CREATEJOBREQUEST,
    output_type=_CREATEJOBRESPONSE,
    serialized_options=None,
    create_key=_descriptor._internal_create_key,
  ),
  _descriptor.MethodDescriptor(
    name='Next',
    full_name='job.JobSvc.Next',
    index=1,
    containing_service=None,
    input_type=_NEXTREQUEST,
    output_type=_NEXTRESPONSE,
    serialized_options=None,
    create_key=_descriptor._internal_create_key,
  ),
  _descriptor.MethodDescriptor(
    name='DeleteJob',
    full_name='job.JobSvc.DeleteJob',
    index=2,
    containing_service=None,
    input_type=_DELETEJOBREQUEST,
    output_type=_DELETEJOBRESPONSE,
    serialized_options=None,
    create_key=_descriptor._internal_create_key,
  ),
])
_sym_db.RegisterServiceDescriptor(_JOBSVC)

DESCRIPTOR.services_by_name['JobSvc'] = _JOBSVC

# @@protoc_insertion_point(module_scope)
