// This file is generated by rust-protobuf 2.25.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `dataloader.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_1;

#[derive(PartialEq,Clone,Default)]
pub struct CreateDataloaderRequest {
    // message fields
    pub dataset_id: u32,
    pub keys: ::std::vec::Vec<u32>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CreateDataloaderRequest {
    fn default() -> &'a CreateDataloaderRequest {
        <CreateDataloaderRequest as ::protobuf::Message>::default_instance()
    }
}

impl CreateDataloaderRequest {
    pub fn new() -> CreateDataloaderRequest {
        ::std::default::Default::default()
    }

    // uint32 dataset_id = 1;


    pub fn get_dataset_id(&self) -> u32 {
        self.dataset_id
    }
    pub fn clear_dataset_id(&mut self) {
        self.dataset_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_dataset_id(&mut self, v: u32) {
        self.dataset_id = v;
    }

    // repeated uint32 keys = 2;


    pub fn get_keys(&self) -> &[u32] {
        &self.keys
    }
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::std::vec::Vec<u32>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for CreateDataloaderRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.dataset_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.keys)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.dataset_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.dataset_id, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.keys {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.dataset_id != 0 {
            os.write_uint32(1, self.dataset_id)?;
        }
        for v in &self.keys {
            os.write_uint32(2, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CreateDataloaderRequest {
        CreateDataloaderRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "dataset_id",
                |m: &CreateDataloaderRequest| { &m.dataset_id },
                |m: &mut CreateDataloaderRequest| { &mut m.dataset_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "keys",
                |m: &CreateDataloaderRequest| { &m.keys },
                |m: &mut CreateDataloaderRequest| { &mut m.keys },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CreateDataloaderRequest>(
                "CreateDataloaderRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CreateDataloaderRequest {
        static instance: ::protobuf::rt::LazyV2<CreateDataloaderRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CreateDataloaderRequest::new)
    }
}

impl ::protobuf::Clear for CreateDataloaderRequest {
    fn clear(&mut self) {
        self.dataset_id = 0;
        self.keys.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateDataloaderRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateDataloaderRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreateDataloaderResponse {
    // message fields
    pub shared_mem_file: ::std::string::String,
    pub loader_id: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CreateDataloaderResponse {
    fn default() -> &'a CreateDataloaderResponse {
        <CreateDataloaderResponse as ::protobuf::Message>::default_instance()
    }
}

impl CreateDataloaderResponse {
    pub fn new() -> CreateDataloaderResponse {
        ::std::default::Default::default()
    }

    // string shared_mem_file = 2;


    pub fn get_shared_mem_file(&self) -> &str {
        &self.shared_mem_file
    }
    pub fn clear_shared_mem_file(&mut self) {
        self.shared_mem_file.clear();
    }

    // Param is passed by value, moved
    pub fn set_shared_mem_file(&mut self, v: ::std::string::String) {
        self.shared_mem_file = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shared_mem_file(&mut self) -> &mut ::std::string::String {
        &mut self.shared_mem_file
    }

    // Take field
    pub fn take_shared_mem_file(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.shared_mem_file, ::std::string::String::new())
    }

    // uint64 loader_id = 3;


    pub fn get_loader_id(&self) -> u64 {
        self.loader_id
    }
    pub fn clear_loader_id(&mut self) {
        self.loader_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_loader_id(&mut self, v: u64) {
        self.loader_id = v;
    }
}

impl ::protobuf::Message for CreateDataloaderResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.shared_mem_file)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.loader_id = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.shared_mem_file.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.shared_mem_file);
        }
        if self.loader_id != 0 {
            my_size += ::protobuf::rt::value_size(3, self.loader_id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.shared_mem_file.is_empty() {
            os.write_string(2, &self.shared_mem_file)?;
        }
        if self.loader_id != 0 {
            os.write_uint64(3, self.loader_id)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CreateDataloaderResponse {
        CreateDataloaderResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "shared_mem_file",
                |m: &CreateDataloaderResponse| { &m.shared_mem_file },
                |m: &mut CreateDataloaderResponse| { &mut m.shared_mem_file },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "loader_id",
                |m: &CreateDataloaderResponse| { &m.loader_id },
                |m: &mut CreateDataloaderResponse| { &mut m.loader_id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CreateDataloaderResponse>(
                "CreateDataloaderResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CreateDataloaderResponse {
        static instance: ::protobuf::rt::LazyV2<CreateDataloaderResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CreateDataloaderResponse::new)
    }
}

impl ::protobuf::Clear for CreateDataloaderResponse {
    fn clear(&mut self) {
        self.shared_mem_file.clear();
        self.loader_id = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateDataloaderResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateDataloaderResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NextRequest {
    // message fields
    pub loader_id: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a NextRequest {
    fn default() -> &'a NextRequest {
        <NextRequest as ::protobuf::Message>::default_instance()
    }
}

impl NextRequest {
    pub fn new() -> NextRequest {
        ::std::default::Default::default()
    }

    // uint64 loader_id = 1;


    pub fn get_loader_id(&self) -> u64 {
        self.loader_id
    }
    pub fn clear_loader_id(&mut self) {
        self.loader_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_loader_id(&mut self, v: u64) {
        self.loader_id = v;
    }
}

impl ::protobuf::Message for NextRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.loader_id = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.loader_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.loader_id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.loader_id != 0 {
            os.write_uint64(1, self.loader_id)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> NextRequest {
        NextRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "loader_id",
                |m: &NextRequest| { &m.loader_id },
                |m: &mut NextRequest| { &mut m.loader_id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<NextRequest>(
                "NextRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static NextRequest {
        static instance: ::protobuf::rt::LazyV2<NextRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(NextRequest::new)
    }
}

impl ::protobuf::Clear for NextRequest {
    fn clear(&mut self) {
        self.loader_id = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NextRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NextRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NextResponse {
    // message fields
    pub address: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a NextResponse {
    fn default() -> &'a NextResponse {
        <NextResponse as ::protobuf::Message>::default_instance()
    }
}

impl NextResponse {
    pub fn new() -> NextResponse {
        ::std::default::Default::default()
    }

    // uint64 address = 2;


    pub fn get_address(&self) -> u64 {
        self.address
    }
    pub fn clear_address(&mut self) {
        self.address = 0;
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: u64) {
        self.address = v;
    }
}

impl ::protobuf::Message for NextResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.address = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.address != 0 {
            my_size += ::protobuf::rt::value_size(2, self.address, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.address != 0 {
            os.write_uint64(2, self.address)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> NextResponse {
        NextResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "address",
                |m: &NextResponse| { &m.address },
                |m: &mut NextResponse| { &mut m.address },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<NextResponse>(
                "NextResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static NextResponse {
        static instance: ::protobuf::rt::LazyV2<NextResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(NextResponse::new)
    }
}

impl ::protobuf::Clear for NextResponse {
    fn clear(&mut self) {
        self.address = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NextResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NextResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteDataloaderRequest {
    // message fields
    pub loader_id: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DeleteDataloaderRequest {
    fn default() -> &'a DeleteDataloaderRequest {
        <DeleteDataloaderRequest as ::protobuf::Message>::default_instance()
    }
}

impl DeleteDataloaderRequest {
    pub fn new() -> DeleteDataloaderRequest {
        ::std::default::Default::default()
    }

    // uint64 loader_id = 3;


    pub fn get_loader_id(&self) -> u64 {
        self.loader_id
    }
    pub fn clear_loader_id(&mut self) {
        self.loader_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_loader_id(&mut self, v: u64) {
        self.loader_id = v;
    }
}

impl ::protobuf::Message for DeleteDataloaderRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.loader_id = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.loader_id != 0 {
            my_size += ::protobuf::rt::value_size(3, self.loader_id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.loader_id != 0 {
            os.write_uint64(3, self.loader_id)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> DeleteDataloaderRequest {
        DeleteDataloaderRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "loader_id",
                |m: &DeleteDataloaderRequest| { &m.loader_id },
                |m: &mut DeleteDataloaderRequest| { &mut m.loader_id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<DeleteDataloaderRequest>(
                "DeleteDataloaderRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static DeleteDataloaderRequest {
        static instance: ::protobuf::rt::LazyV2<DeleteDataloaderRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(DeleteDataloaderRequest::new)
    }
}

impl ::protobuf::Clear for DeleteDataloaderRequest {
    fn clear(&mut self) {
        self.loader_id = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteDataloaderRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteDataloaderRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteDataloaderResponse {
    // message fields
    pub resp: LoaderStatus,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DeleteDataloaderResponse {
    fn default() -> &'a DeleteDataloaderResponse {
        <DeleteDataloaderResponse as ::protobuf::Message>::default_instance()
    }
}

impl DeleteDataloaderResponse {
    pub fn new() -> DeleteDataloaderResponse {
        ::std::default::Default::default()
    }

    // .LoaderStatus resp = 3;


    pub fn get_resp(&self) -> LoaderStatus {
        self.resp
    }
    pub fn clear_resp(&mut self) {
        self.resp = LoaderStatus::Ok;
    }

    // Param is passed by value, moved
    pub fn set_resp(&mut self, v: LoaderStatus) {
        self.resp = v;
    }
}

impl ::protobuf::Message for DeleteDataloaderResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                3 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.resp, 3, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.resp != LoaderStatus::Ok {
            my_size += ::protobuf::rt::enum_size(3, self.resp);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.resp != LoaderStatus::Ok {
            os.write_enum(3, ::protobuf::ProtobufEnum::value(&self.resp))?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> DeleteDataloaderResponse {
        DeleteDataloaderResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<LoaderStatus>>(
                "resp",
                |m: &DeleteDataloaderResponse| { &m.resp },
                |m: &mut DeleteDataloaderResponse| { &mut m.resp },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<DeleteDataloaderResponse>(
                "DeleteDataloaderResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static DeleteDataloaderResponse {
        static instance: ::protobuf::rt::LazyV2<DeleteDataloaderResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(DeleteDataloaderResponse::new)
    }
}

impl ::protobuf::Clear for DeleteDataloaderResponse {
    fn clear(&mut self) {
        self.resp = LoaderStatus::Ok;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteDataloaderResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteDataloaderResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum LoaderStatus {
    Ok = 0,
    False = 1,
}

impl ::protobuf::ProtobufEnum for LoaderStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<LoaderStatus> {
        match value {
            0 => ::std::option::Option::Some(LoaderStatus::Ok),
            1 => ::std::option::Option::Some(LoaderStatus::False),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [LoaderStatus] = &[
            LoaderStatus::Ok,
            LoaderStatus::False,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<LoaderStatus>("LoaderStatus", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for LoaderStatus {
}

impl ::std::default::Default for LoaderStatus {
    fn default() -> Self {
        LoaderStatus::Ok
    }
}

impl ::protobuf::reflect::ProtobufValue for LoaderStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10dataloader.proto\"L\n\x17CreateDataloaderRequest\x12\x1d\n\ndatase\
    t_id\x18\x01\x20\x01(\rR\tdatasetId\x12\x12\n\x04keys\x18\x02\x20\x03(\r\
    R\x04keys\"_\n\x18CreateDataloaderResponse\x12&\n\x0fshared_mem_file\x18\
    \x02\x20\x01(\tR\rsharedMemFile\x12\x1b\n\tloader_id\x18\x03\x20\x01(\
    \x04R\x08loaderId\"*\n\x0bNextRequest\x12\x1b\n\tloader_id\x18\x01\x20\
    \x01(\x04R\x08loaderId\"(\n\x0cNextResponse\x12\x18\n\x07address\x18\x02\
    \x20\x01(\x04R\x07address\"6\n\x17DeleteDataloaderRequest\x12\x1b\n\tloa\
    der_id\x18\x03\x20\x01(\x04R\x08loaderId\"=\n\x18DeleteDataloaderRespons\
    e\x12!\n\x04resp\x18\x03\x20\x01(\x0e2\r.LoaderStatusR\x04resp*!\n\x0cLo\
    aderStatus\x12\x06\n\x02Ok\x10\0\x12\t\n\x05False\x10\x012\xc3\x01\n\nDa\
    taLoader\x12G\n\x10CreateDataloader\x12\x18.CreateDataloaderRequest\x1a\
    \x19.CreateDataloaderResponse\x12#\n\x04Next\x12\x0c.NextRequest\x1a\r.N\
    extResponse\x12G\n\x10DeleteDataloader\x12\x18.DeleteDataloaderRequest\
    \x1a\x19.DeleteDataloaderResponseb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
