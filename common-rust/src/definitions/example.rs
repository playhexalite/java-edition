// This file is generated by rust-protobuf 3.0.0-alpha.2. Do not edit
// .proto file is parsed by protoc 3.19.4
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `example.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_0_0_ALPHA_2;

#[derive(PartialEq,Clone,Default)]
pub struct Example {
    // message fields
    name: ::std::option::Option<::std::string::String>,
    id: ::std::option::Option<i32>,
    email: ::std::option::Option<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::rt::CachedSize,
}

impl<'a> ::std::default::Default for &'a Example {
    fn default() -> &'a Example {
        <Example as ::protobuf::Message>::default_instance()
    }
}

impl Example {
    pub fn new() -> Example {
        ::std::default::Default::default()
    }

    // required string name = 1;

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_name(&mut self) {
        self.name = ::std::option::Option::None;
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name = ::std::option::Option::Some(::std::string::String::new());
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // required int32 id = 2;

    pub fn get_id(&self) -> i32 {
        self.id.unwrap_or(0)
    }

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = ::std::option::Option::Some(v);
    }

    // required string email = 3;

    pub fn get_email(&self) -> &str {
        match self.email.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_email(&mut self) {
        self.email = ::std::option::Option::None;
    }

    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        if self.email.is_none() {
            self.email = ::std::option::Option::Some(::std::string::String::new());
        }
        self.email.as_mut().unwrap()
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        self.email.take().unwrap_or_else(|| ::std::string::String::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::new();
        fields.push(::protobuf::reflect::rt::v2::make_option_get_ref_simpler_accessor::<_, _>(
            "name",
            |m: &Example| { &m.name },
            |m: &mut Example| { &mut m.name },
            Example::get_name,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_get_copy_simpler_accessor::<_, _>(
            "id",
            |m: &Example| { &m.id },
            |m: &mut Example| { &mut m.id },
            Example::get_id,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_get_ref_simpler_accessor::<_, _>(
            "email",
            |m: &Example| { &m.email },
            |m: &mut Example| { &mut m.email },
            Example::get_email,
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Example>(
            "Example",
            0,
            fields,
        )
    }
}

impl ::protobuf::Message for Example {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        if self.id.is_none() {
            return false;
        }
        if self.email.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.name = ::std::option::Option::Some(is.read_string()?);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.id = ::std::option::Option::Some(is.read_int32()?);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.email = ::std::option::Option::Some(is.read_string()?);
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
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.email.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(v) = self.id {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.email.as_ref() {
            os.write_string(3, v)?;
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

    fn new() -> Example {
        Example::new()
    }

    fn descriptor_static() -> ::protobuf::reflect::MessageDescriptor {
        ::protobuf::reflect::MessageDescriptor::new_generated_2(file_descriptor(), 0)
    }

    fn default_instance() -> &'static Example {
        static instance: Example = Example {
            name: ::std::option::Option::None,
            id: ::std::option::Option::None,
            email: ::std::option::Option::None,
            unknown_fields: ::protobuf::UnknownFields::new(),
            cached_size: ::protobuf::rt::CachedSize::new(),
        };
        &instance
    }
}

impl ::protobuf::Clear for Example {
    fn clear(&mut self) {
        self.name = ::std::option::Option::None;
        self.id = ::std::option::Option::None;
        self.email = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Example {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Example {
    type RuntimeType = ::protobuf::reflect::runtime_types::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rexample.proto\"C\n\x07Example\x12\x12\n\x04name\x18\x01\x20\x02(\tR\
    \x04name\x12\x0e\n\x02id\x18\x02\x20\x02(\x05R\x02id\x12\x14\n\x05email\
    \x18\x03\x20\x02(\tR\x05email\
";

/// `FileDescriptorProto` object which was a source for this generated file
pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> ::protobuf::reflect::FileDescriptor {
    static file_descriptor_lazy: ::protobuf::rt::LazyV2<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::LazyV2::INIT;
    let file_descriptor = file_descriptor_lazy.get(|| {
        let mut deps = ::std::vec::Vec::new();
        let mut messages = ::std::vec::Vec::new();
        messages.push(Example::generated_message_descriptor_data());
        let mut enums = ::std::vec::Vec::new();
        ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
            file_descriptor_proto(),
            deps,
            messages,
            enums,
        )
    });
    ::protobuf::reflect::FileDescriptor::new_generated_2(file_descriptor)
}
