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

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum HPrimOp {
    PNOP = 0,
    INTADD = 8,
    INTSUB = 9,
    INTMUL = 10,
    INTNEG = 19,
    INTGT = 22,
    INTGE = 23,
    INTEQ = 24,
    INTNE = 25,
    INTLT = 26,
    INTLE = 27,
    DATATOTAG = 100,
    TAGTOENUM = 101,
}

impl ::protobuf::ProtobufEnum for HPrimOp {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<HPrimOp> {
        match value {
            0 => ::std::option::Option::Some(HPrimOp::PNOP),
            8 => ::std::option::Option::Some(HPrimOp::INTADD),
            9 => ::std::option::Option::Some(HPrimOp::INTSUB),
            10 => ::std::option::Option::Some(HPrimOp::INTMUL),
            19 => ::std::option::Option::Some(HPrimOp::INTNEG),
            22 => ::std::option::Option::Some(HPrimOp::INTGT),
            23 => ::std::option::Option::Some(HPrimOp::INTGE),
            24 => ::std::option::Option::Some(HPrimOp::INTEQ),
            25 => ::std::option::Option::Some(HPrimOp::INTNE),
            26 => ::std::option::Option::Some(HPrimOp::INTLT),
            27 => ::std::option::Option::Some(HPrimOp::INTLE),
            100 => ::std::option::Option::Some(HPrimOp::DATATOTAG),
            101 => ::std::option::Option::Some(HPrimOp::TAGTOENUM),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [HPrimOp] = &[
            HPrimOp::PNOP,
            HPrimOp::INTADD,
            HPrimOp::INTSUB,
            HPrimOp::INTMUL,
            HPrimOp::INTNEG,
            HPrimOp::INTGT,
            HPrimOp::INTGE,
            HPrimOp::INTEQ,
            HPrimOp::INTNE,
            HPrimOp::INTLT,
            HPrimOp::INTLE,
            HPrimOp::DATATOTAG,
            HPrimOp::TAGTOENUM,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<HPrimOp>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("HPrimOp", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for HPrimOp {
}

impl ::std::default::Default for HPrimOp {
    fn default() -> Self {
        HPrimOp::PNOP
    }
}

impl ::protobuf::reflect::ProtobufValue for HPrimOp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nprim.proto*\xa3\x01\n\x07HPrimOp\x12\x08\n\x04PNOP\x10\0\x12\n\n\x06\
    INTADD\x10\x08\x12\n\n\x06INTSUB\x10\t\x12\n\n\x06INTMUL\x10\n\x12\n\n\
    \x06INTNEG\x10\x13\x12\t\n\x05INTGT\x10\x16\x12\t\n\x05INTGE\x10\x17\x12\
    \t\n\x05INTEQ\x10\x18\x12\t\n\x05INTNE\x10\x19\x12\t\n\x05INTLT\x10\x1a\
    \x12\t\n\x05INTLE\x10\x1b\x12\r\n\tDATATOTAG\x10d\x12\r\n\tTAGTOENUM\x10\
    eJ\xc1\x07\n\x06\x12\x04\0\0\x15\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\
    \n\x02\x05\0\x12\x04\x03\0\x15\x01\n\n\n\x03\x05\0\x01\x12\x03\x03\x05\
    \x0c\n\x0b\n\x04\x05\0\x02\0\x12\x03\x04\x04\r\n\x0c\n\x05\x05\0\x02\0\
    \x01\x12\x03\x04\x04\x08\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x04\x0b\x0c\
    \n3\n\x04\x05\0\x02\x01\x12\x03\x08\x04\x0f\"\x1e\x20(+#)\x20::\x20Int#\
    \x20->\x20Int#\x20->\x20Int#\n2\x06\x20Int#\n\n\x0c\n\x05\x05\0\x02\x01\
    \x01\x12\x03\x08\x04\n\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x08\r\x0e\n\
    +\n\x04\x05\0\x02\x02\x12\x03\t\x04\x0f\"\x1e\x20(-#)\x20::\x20Int#\x20-\
    >\x20Int#\x20->\x20Int#\n\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\t\x04\n\
    \n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\t\r\x0e\n+\n\x04\x05\0\x02\x03\
    \x12\x03\n\x04\x10\"\x1e\x20(*#)\x20::\x20Int#\x20->\x20Int#\x20->\x20In\
    t#\n\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03\n\x04\n\n\x0c\n\x05\x05\0\x02\
    \x03\x02\x12\x03\n\r\x0f\n)\n\x04\x05\0\x02\x04\x12\x03\x0b\x04\x10\"\
    \x1c\x20negateInt#\x20::\x20Int#\x20->\x20Int#\n\n\x0c\n\x05\x05\0\x02\
    \x04\x01\x12\x03\x0b\x04\n\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x0b\r\
    \x0f\n,\n\x04\x05\0\x02\x05\x12\x03\x0c\x04\x10\"\x1f\x20(>#)\x20\x20::\
    \x20Int#\x20->\x20Int#\x20->\x20Int#\n\n\x0c\n\x05\x05\0\x02\x05\x01\x12\
    \x03\x0c\x04\t\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03\x0c\r\x0f\n,\n\x04\
    \x05\0\x02\x06\x12\x03\r\x04\x10\"\x1f\x20(>=#)\x20::\x20Int#\x20->\x20I\
    nt#\x20->\x20Int#\n\n\x0c\n\x05\x05\0\x02\x06\x01\x12\x03\r\x04\t\n\x0c\
    \n\x05\x05\0\x02\x06\x02\x12\x03\r\r\x0f\n,\n\x04\x05\0\x02\x07\x12\x03\
    \x0e\x04\x10\"\x1f\x20(==#)\x20::\x20Int#\x20->\x20Int#\x20->\x20Int#\n\
    \n\x0c\n\x05\x05\0\x02\x07\x01\x12\x03\x0e\x04\t\n\x0c\n\x05\x05\0\x02\
    \x07\x02\x12\x03\x0e\r\x0f\n,\n\x04\x05\0\x02\x08\x12\x03\x0f\x04\x10\"\
    \x1f\x20(/=#)\x20::\x20Int#\x20->\x20Int#\x20->\x20Int#\n\n\x0c\n\x05\
    \x05\0\x02\x08\x01\x12\x03\x0f\x04\t\n\x0c\n\x05\x05\0\x02\x08\x02\x12\
    \x03\x0f\r\x0f\n,\n\x04\x05\0\x02\t\x12\x03\x10\x04\x10\"\x1f\x20(<#)\
    \x20\x20::\x20Int#\x20->\x20Int#\x20->\x20Int#\n\n\x0c\n\x05\x05\0\x02\t\
    \x01\x12\x03\x10\x04\t\n\x0c\n\x05\x05\0\x02\t\x02\x12\x03\x10\r\x0f\n,\
    \n\x04\x05\0\x02\n\x12\x03\x11\x04\x10\"\x1f\x20(<=#)\x20::\x20Int#\x20-\
    >\x20Int#\x20->\x20Int#\n\n\x0c\n\x05\x05\0\x02\n\x01\x12\x03\x11\x04\t\
    \n\x0c\n\x05\x05\0\x02\n\x02\x12\x03\x11\r\x0f\n&\n\x04\x05\0\x02\x0b\
    \x12\x03\x13\x04\x14\"\x19\x20dataToTag#\x20::\x20a\x20->\x20Int#\n\n\
    \x0c\n\x05\x05\0\x02\x0b\x01\x12\x03\x13\x04\r\n\x0c\n\x05\x05\0\x02\x0b\
    \x02\x12\x03\x13\x10\x13\n&\n\x04\x05\0\x02\x0c\x12\x03\x14\x04\x14\"\
    \x19\x20tagToEnum#\x20::\x20Int#\x20->\x20a\n\n\x0c\n\x05\x05\0\x02\x0c\
    \x01\x12\x03\x14\x04\r\n\x0c\n\x05\x05\0\x02\x0c\x02\x12\x03\x14\x10\x13\
    b\x06proto3\
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
