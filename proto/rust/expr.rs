// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Expr {
    // message fields
    pub field_type: ExprType,
    pub p_num: u32,
    pub tag: u32,
    pub payloads: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Expr {}

impl Expr {
    pub fn new() -> Expr {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Expr {
        static mut instance: ::protobuf::lazy::Lazy<Expr> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Expr,
        };
        unsafe {
            instance.get(Expr::new)
        }
    }

    // .ExprType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ExprType::INVALID;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ExprType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> ExprType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &ExprType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ExprType {
        &mut self.field_type
    }

    // uint32 p_num = 2;

    pub fn clear_p_num(&mut self) {
        self.p_num = 0;
    }

    // Param is passed by value, moved
    pub fn set_p_num(&mut self, v: u32) {
        self.p_num = v;
    }

    pub fn get_p_num(&self) -> u32 {
        self.p_num
    }

    fn get_p_num_for_reflect(&self) -> &u32 {
        &self.p_num
    }

    fn mut_p_num_for_reflect(&mut self) -> &mut u32 {
        &mut self.p_num
    }

    // uint32 tag = 3;

    pub fn clear_tag(&mut self) {
        self.tag = 0;
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: u32) {
        self.tag = v;
    }

    pub fn get_tag(&self) -> u32 {
        self.tag
    }

    fn get_tag_for_reflect(&self) -> &u32 {
        &self.tag
    }

    fn mut_tag_for_reflect(&mut self) -> &mut u32 {
        &mut self.tag
    }

    // repeated fixed64 payloads = 4;

    pub fn clear_payloads(&mut self) {
        self.payloads.clear();
    }

    // Param is passed by value, moved
    pub fn set_payloads(&mut self, v: ::std::vec::Vec<u64>) {
        self.payloads = v;
    }

    // Mutable pointer to the field.
    pub fn mut_payloads(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.payloads
    }

    // Take field
    pub fn take_payloads(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.payloads, ::std::vec::Vec::new())
    }

    pub fn get_payloads(&self) -> &[u64] {
        &self.payloads
    }

    fn get_payloads_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.payloads
    }

    fn mut_payloads_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.payloads
    }
}

impl ::protobuf::Message for Expr {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.field_type, 1, &mut self.unknown_fields)?
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.p_num = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.tag = tmp;
                },
                4 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.payloads)?;
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
        if self.field_type != ExprType::INVALID {
            my_size += ::protobuf::rt::enum_size(1, self.field_type);
        }
        if self.p_num != 0 {
            my_size += ::protobuf::rt::value_size(2, self.p_num, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.tag != 0 {
            my_size += ::protobuf::rt::value_size(3, self.tag, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += 9 * self.payloads.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.field_type != ExprType::INVALID {
            os.write_enum(1, self.field_type.value())?;
        }
        if self.p_num != 0 {
            os.write_uint32(2, self.p_num)?;
        }
        if self.tag != 0 {
            os.write_uint32(3, self.tag)?;
        }
        for v in &self.payloads {
            os.write_fixed64(4, *v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Expr {
    fn new() -> Expr {
        Expr::new()
    }

    fn descriptor_static(_: ::std::option::Option<Expr>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ExprType>>(
                    "type",
                    Expr::get_field_type_for_reflect,
                    Expr::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "p_num",
                    Expr::get_p_num_for_reflect,
                    Expr::mut_p_num_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tag",
                    Expr::get_tag_for_reflect,
                    Expr::mut_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "payloads",
                    Expr::get_payloads_for_reflect,
                    Expr::mut_payloads_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Expr>(
                    "Expr",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Expr {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_p_num();
        self.clear_tag();
        self.clear_payloads();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Expr {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ExprType {
    INVALID = 0,
    INDIR = 1,
    CONSTR = 2,
    FUNC = 4,
    THUNK_F = 6,
    THUNK_C = 7,
    PAPPLY = 8,
    BOTTOM = 9,
}

impl ::protobuf::ProtobufEnum for ExprType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ExprType> {
        match value {
            0 => ::std::option::Option::Some(ExprType::INVALID),
            1 => ::std::option::Option::Some(ExprType::INDIR),
            2 => ::std::option::Option::Some(ExprType::CONSTR),
            4 => ::std::option::Option::Some(ExprType::FUNC),
            6 => ::std::option::Option::Some(ExprType::THUNK_F),
            7 => ::std::option::Option::Some(ExprType::THUNK_C),
            8 => ::std::option::Option::Some(ExprType::PAPPLY),
            9 => ::std::option::Option::Some(ExprType::BOTTOM),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ExprType] = &[
            ExprType::INVALID,
            ExprType::INDIR,
            ExprType::CONSTR,
            ExprType::FUNC,
            ExprType::THUNK_F,
            ExprType::THUNK_C,
            ExprType::PAPPLY,
            ExprType::BOTTOM,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ExprType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ExprType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ExprType {
}

impl ::std::default::Default for ExprType {
    fn default() -> Self {
        ExprType::INVALID
    }
}

impl ::protobuf::reflect::ProtobufValue for ExprType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nexpr.proto\"h\n\x04Expr\x12\x1d\n\x04type\x18\x01\x20\x01(\x0e2\t.Ex\
    prTypeR\x04type\x12\x13\n\x05p_num\x18\x02\x20\x01(\rR\x04pNum\x12\x10\n\
    \x03tag\x18\x03\x20\x01(\rR\x03tag\x12\x1a\n\x08payloads\x18\x04\x20\x03\
    (\x06R\x08payloads*j\n\x08ExprType\x12\x0b\n\x07INVALID\x10\0\x12\t\n\
    \x05INDIR\x10\x01\x12\n\n\x06CONSTR\x10\x02\x12\x08\n\x04FUNC\x10\x04\
    \x12\x0b\n\x07THUNK_F\x10\x06\x12\x0b\n\x07THUNK_C\x10\x07\x12\n\n\x06PA\
    PPLY\x10\x08\x12\n\n\x06BOTTOM\x10\tJ\xea\x06\n\x06\x12\x04\0\0\x12\x01\
    \n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x07\x01\n\
    \n\n\x03\x04\0\x01\x12\x03\x02\x08\x0c\n\x0b\n\x04\x04\0\x02\0\x12\x03\
    \x03\x04\"\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x03\x04\x02\x0e\n\x0c\n\x05\
    \x04\0\x02\0\x06\x12\x03\x03\x04\x0c\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x03\x15\x19\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x20!\n\x1d\n\x04\
    \x04\0\x02\x01\x12\x03\x04\x04\"\"\x10\x20pointer\x20number\n\n\r\n\x05\
    \x04\0\x02\x01\x04\x12\x04\x04\x04\x03\"\n\x0c\n\x05\x04\0\x02\x01\x05\
    \x12\x03\x04\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\x15\x1a\n\
    \x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04\x20!\n.\n\x04\x04\0\x02\x02\x12\
    \x03\x05\x04\"\"!\x20Data\x20constructor\x20tag\x20(optional)\n\n\r\n\
    \x05\x04\0\x02\x02\x04\x12\x04\x05\x04\x04\"\n\x0c\n\x05\x04\0\x02\x02\
    \x05\x12\x03\x05\x04\n\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x05\x15\x18\
    \n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x05\x20!\n\x0b\n\x04\x04\0\x02\
    \x03\x12\x03\x06\x04\"\n\x0c\n\x05\x04\0\x02\x03\x04\x12\x03\x06\x04\x0c\
    \n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x06\r\x14\n\x0c\n\x05\x04\0\x02\
    \x03\x01\x12\x03\x06\x15\x1d\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x06\
    \x20!\n\n\n\x02\x05\0\x12\x04\t\0\x12\x01\n\n\n\x03\x05\0\x01\x12\x03\t\
    \x05\r\n\x0b\n\x04\x05\0\x02\0\x12\x03\n\x04\x10\n\x0c\n\x05\x05\0\x02\0\
    \x01\x12\x03\n\x04\x0b\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\n\x0e\x0f\n\
    \x1a\n\x04\x05\0\x02\x01\x12\x03\x0b\x04\x10\"\r\x20Indirection\n\n\x0c\
    \n\x05\x05\0\x02\x01\x01\x12\x03\x0b\x04\t\n\x0c\n\x05\x05\0\x02\x01\x02\
    \x12\x03\x0b\x0e\x0f\n\x1f\n\x04\x05\0\x02\x02\x12\x03\x0c\x04\x10\"\x12\
    \x20Data\x20constructor\n\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\x0c\x04\
    \n\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x0c\x0e\x0f\n\x1f\n\x04\x05\0\
    \x02\x03\x12\x03\r\x04\x10\"\x12\x20Function\x20\x08object\n\n\x0c\n\x05\
    \x05\0\x02\x03\x01\x12\x03\r\x04\x08\n\x0c\n\x05\x05\0\x02\x03\x02\x12\
    \x03\r\x0e\x0f\n#\n\x04\x05\0\x02\x04\x12\x03\x0e\x04\x10\"\x16\x20Thunk\
    \x20has\x20a\x20function\n\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03\x0e\x04\
    \x0b\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x0e\x0e\x0f\n\"\n\x04\x05\0\
    \x02\x05\x12\x03\x0f\x04\x10\"\x15\x20Thunk\x20has\x20a\x20closure\n\n\
    \x0c\n\x05\x05\0\x02\x05\x01\x12\x03\x0f\x04\x0b\n\x0c\n\x05\x05\0\x02\
    \x05\x02\x12\x03\x0f\x0e\x0f\n*\n\x04\x05\0\x02\x06\x12\x03\x10\x04\x10\
    \"\x1d\x20Partial\x20apply\x20to\x20a\x20function\n\n\x0c\n\x05\x05\0\
    \x02\x06\x01\x12\x03\x10\x04\n\n\x0c\n\x05\x05\0\x02\x06\x02\x12\x03\x10\
    \x0e\x0f\n\x1a\n\x04\x05\0\x02\x07\x12\x03\x11\x04\x10\"\r\x20\xe2\x8a\
    \xa5\x20(error)\n\n\x0c\n\x05\x05\0\x02\x07\x01\x12\x03\x11\x04\n\n\x0c\
    \n\x05\x05\0\x02\x07\x02\x12\x03\x11\x0e\x0fb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
