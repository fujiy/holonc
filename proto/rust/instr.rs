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
pub struct Func {
    // message fields
    pub arity: u32,
    pub srt: ::std::vec::Vec<u64>,
    pub blocks: ::protobuf::RepeatedField<Block>,
    pub symbol: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Func {}

impl Func {
    pub fn new() -> Func {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Func {
        static mut instance: ::protobuf::lazy::Lazy<Func> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Func,
        };
        unsafe {
            instance.get(Func::new)
        }
    }

    // uint32 arity = 1;

    pub fn clear_arity(&mut self) {
        self.arity = 0;
    }

    // Param is passed by value, moved
    pub fn set_arity(&mut self, v: u32) {
        self.arity = v;
    }

    pub fn get_arity(&self) -> u32 {
        self.arity
    }

    fn get_arity_for_reflect(&self) -> &u32 {
        &self.arity
    }

    fn mut_arity_for_reflect(&mut self) -> &mut u32 {
        &mut self.arity
    }

    // repeated fixed64 srt = 2;

    pub fn clear_srt(&mut self) {
        self.srt.clear();
    }

    // Param is passed by value, moved
    pub fn set_srt(&mut self, v: ::std::vec::Vec<u64>) {
        self.srt = v;
    }

    // Mutable pointer to the field.
    pub fn mut_srt(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.srt
    }

    // Take field
    pub fn take_srt(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.srt, ::std::vec::Vec::new())
    }

    pub fn get_srt(&self) -> &[u64] {
        &self.srt
    }

    fn get_srt_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.srt
    }

    fn mut_srt_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.srt
    }

    // repeated .Block blocks = 3;

    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_blocks(&mut self, v: ::protobuf::RepeatedField<Block>) {
        self.blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blocks(&mut self) -> &mut ::protobuf::RepeatedField<Block> {
        &mut self.blocks
    }

    // Take field
    pub fn take_blocks(&mut self) -> ::protobuf::RepeatedField<Block> {
        ::std::mem::replace(&mut self.blocks, ::protobuf::RepeatedField::new())
    }

    pub fn get_blocks(&self) -> &[Block] {
        &self.blocks
    }

    fn get_blocks_for_reflect(&self) -> &::protobuf::RepeatedField<Block> {
        &self.blocks
    }

    fn mut_blocks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Block> {
        &mut self.blocks
    }

    // string symbol = 4;

    pub fn clear_symbol(&mut self) {
        self.symbol.clear();
    }

    // Param is passed by value, moved
    pub fn set_symbol(&mut self, v: ::std::string::String) {
        self.symbol = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_symbol(&mut self) -> &mut ::std::string::String {
        &mut self.symbol
    }

    // Take field
    pub fn take_symbol(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.symbol, ::std::string::String::new())
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    fn get_symbol_for_reflect(&self) -> &::std::string::String {
        &self.symbol
    }

    fn mut_symbol_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.symbol
    }
}

impl ::protobuf::Message for Func {
    fn is_initialized(&self) -> bool {
        for v in &self.blocks {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.arity = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.srt)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blocks)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.symbol)?;
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
        if self.arity != 0 {
            my_size += ::protobuf::rt::value_size(1, self.arity, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += 9 * self.srt.len() as u32;
        for value in &self.blocks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.symbol.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.symbol);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.arity != 0 {
            os.write_uint32(1, self.arity)?;
        }
        for v in &self.srt {
            os.write_fixed64(2, *v)?;
        };
        for v in &self.blocks {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.symbol.is_empty() {
            os.write_string(4, &self.symbol)?;
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

impl ::protobuf::MessageStatic for Func {
    fn new() -> Func {
        Func::new()
    }

    fn descriptor_static(_: ::std::option::Option<Func>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "arity",
                    Func::get_arity_for_reflect,
                    Func::mut_arity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "srt",
                    Func::get_srt_for_reflect,
                    Func::mut_srt_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Block>>(
                    "blocks",
                    Func::get_blocks_for_reflect,
                    Func::mut_blocks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "symbol",
                    Func::get_symbol_for_reflect,
                    Func::mut_symbol_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Func>(
                    "Func",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Func {
    fn clear(&mut self) {
        self.clear_arity();
        self.clear_srt();
        self.clear_blocks();
        self.clear_symbol();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Func {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Func {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Block {
    // message fields
    pub bitmap: ::std::vec::Vec<u8>,
    pub code: ::protobuf::RepeatedField<Instr>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Block {}

impl Block {
    pub fn new() -> Block {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Block {
        static mut instance: ::protobuf::lazy::Lazy<Block> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Block,
        };
        unsafe {
            instance.get(Block::new)
        }
    }

    // bytes bitmap = 1;

    pub fn clear_bitmap(&mut self) {
        self.bitmap.clear();
    }

    // Param is passed by value, moved
    pub fn set_bitmap(&mut self, v: ::std::vec::Vec<u8>) {
        self.bitmap = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bitmap(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.bitmap
    }

    // Take field
    pub fn take_bitmap(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.bitmap, ::std::vec::Vec::new())
    }

    pub fn get_bitmap(&self) -> &[u8] {
        &self.bitmap
    }

    fn get_bitmap_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.bitmap
    }

    fn mut_bitmap_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.bitmap
    }

    // repeated .Instr code = 2;

    pub fn clear_code(&mut self) {
        self.code.clear();
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: ::protobuf::RepeatedField<Instr>) {
        self.code = v;
    }

    // Mutable pointer to the field.
    pub fn mut_code(&mut self) -> &mut ::protobuf::RepeatedField<Instr> {
        &mut self.code
    }

    // Take field
    pub fn take_code(&mut self) -> ::protobuf::RepeatedField<Instr> {
        ::std::mem::replace(&mut self.code, ::protobuf::RepeatedField::new())
    }

    pub fn get_code(&self) -> &[Instr] {
        &self.code
    }

    fn get_code_for_reflect(&self) -> &::protobuf::RepeatedField<Instr> {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Instr> {
        &mut self.code
    }
}

impl ::protobuf::Message for Block {
    fn is_initialized(&self) -> bool {
        for v in &self.code {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.bitmap)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.code)?;
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
        if !self.bitmap.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.bitmap);
        }
        for value in &self.code {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.bitmap.is_empty() {
            os.write_bytes(1, &self.bitmap)?;
        }
        for v in &self.code {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for Block {
    fn new() -> Block {
        Block::new()
    }

    fn descriptor_static(_: ::std::option::Option<Block>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bitmap",
                    Block::get_bitmap_for_reflect,
                    Block::mut_bitmap_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Instr>>(
                    "code",
                    Block::get_code_for_reflect,
                    Block::mut_code_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Block>(
                    "Block",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Block {
    fn clear(&mut self) {
        self.clear_bitmap();
        self.clear_code();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Block {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Block {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Instr {
    // message fields
    pub opcode: Opcode,
    pub p_num: u32,
    pub operands: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Instr {}

impl Instr {
    pub fn new() -> Instr {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Instr {
        static mut instance: ::protobuf::lazy::Lazy<Instr> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Instr,
        };
        unsafe {
            instance.get(Instr::new)
        }
    }

    // .Opcode opcode = 1;

    pub fn clear_opcode(&mut self) {
        self.opcode = Opcode::NOP;
    }

    // Param is passed by value, moved
    pub fn set_opcode(&mut self, v: Opcode) {
        self.opcode = v;
    }

    pub fn get_opcode(&self) -> Opcode {
        self.opcode
    }

    fn get_opcode_for_reflect(&self) -> &Opcode {
        &self.opcode
    }

    fn mut_opcode_for_reflect(&mut self) -> &mut Opcode {
        &mut self.opcode
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

    // repeated uint32 operands = 3;

    pub fn clear_operands(&mut self) {
        self.operands.clear();
    }

    // Param is passed by value, moved
    pub fn set_operands(&mut self, v: ::std::vec::Vec<u32>) {
        self.operands = v;
    }

    // Mutable pointer to the field.
    pub fn mut_operands(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.operands
    }

    // Take field
    pub fn take_operands(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.operands, ::std::vec::Vec::new())
    }

    pub fn get_operands(&self) -> &[u32] {
        &self.operands
    }

    fn get_operands_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.operands
    }

    fn mut_operands_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.operands
    }
}

impl ::protobuf::Message for Instr {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.opcode, 1, &mut self.unknown_fields)?
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.p_num = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.operands)?;
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
        if self.opcode != Opcode::NOP {
            my_size += ::protobuf::rt::enum_size(1, self.opcode);
        }
        if self.p_num != 0 {
            my_size += ::protobuf::rt::value_size(2, self.p_num, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.operands {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.opcode != Opcode::NOP {
            os.write_enum(1, self.opcode.value())?;
        }
        if self.p_num != 0 {
            os.write_uint32(2, self.p_num)?;
        }
        for v in &self.operands {
            os.write_uint32(3, *v)?;
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

impl ::protobuf::MessageStatic for Instr {
    fn new() -> Instr {
        Instr::new()
    }

    fn descriptor_static(_: ::std::option::Option<Instr>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Opcode>>(
                    "opcode",
                    Instr::get_opcode_for_reflect,
                    Instr::mut_opcode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "p_num",
                    Instr::get_p_num_for_reflect,
                    Instr::mut_p_num_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "operands",
                    Instr::get_operands_for_reflect,
                    Instr::mut_operands_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Instr>(
                    "Instr",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Instr {
    fn clear(&mut self) {
        self.clear_opcode();
        self.clear_p_num();
        self.clear_operands();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Instr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Instr {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Opcode {
    NOP = 0,
    JUMP = 2,
    PUSH = 3,
    RETURN = 4,
    PRIMOP_P = 5,
    PRIMOP_R = 6,
    CONSTR_P = 7,
    CONSTR_R = 8,
    FUNC_P = 9,
    FUNC_R = 10,
    THUNK_F_P = 11,
    THUNK_F_R = 12,
    THUNK_C_P = 13,
    THUNK_C_R = 14,
    BOTTOM_P = 15,
    BOTTOM_R = 16,
    EVAL = 17,
    MATCH_A_E_P = 18,
    MATCH_A_E_R = 19,
}

impl ::protobuf::ProtobufEnum for Opcode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Opcode> {
        match value {
            0 => ::std::option::Option::Some(Opcode::NOP),
            2 => ::std::option::Option::Some(Opcode::JUMP),
            3 => ::std::option::Option::Some(Opcode::PUSH),
            4 => ::std::option::Option::Some(Opcode::RETURN),
            5 => ::std::option::Option::Some(Opcode::PRIMOP_P),
            6 => ::std::option::Option::Some(Opcode::PRIMOP_R),
            7 => ::std::option::Option::Some(Opcode::CONSTR_P),
            8 => ::std::option::Option::Some(Opcode::CONSTR_R),
            9 => ::std::option::Option::Some(Opcode::FUNC_P),
            10 => ::std::option::Option::Some(Opcode::FUNC_R),
            11 => ::std::option::Option::Some(Opcode::THUNK_F_P),
            12 => ::std::option::Option::Some(Opcode::THUNK_F_R),
            13 => ::std::option::Option::Some(Opcode::THUNK_C_P),
            14 => ::std::option::Option::Some(Opcode::THUNK_C_R),
            15 => ::std::option::Option::Some(Opcode::BOTTOM_P),
            16 => ::std::option::Option::Some(Opcode::BOTTOM_R),
            17 => ::std::option::Option::Some(Opcode::EVAL),
            18 => ::std::option::Option::Some(Opcode::MATCH_A_E_P),
            19 => ::std::option::Option::Some(Opcode::MATCH_A_E_R),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Opcode] = &[
            Opcode::NOP,
            Opcode::JUMP,
            Opcode::PUSH,
            Opcode::RETURN,
            Opcode::PRIMOP_P,
            Opcode::PRIMOP_R,
            Opcode::CONSTR_P,
            Opcode::CONSTR_R,
            Opcode::FUNC_P,
            Opcode::FUNC_R,
            Opcode::THUNK_F_P,
            Opcode::THUNK_F_R,
            Opcode::THUNK_C_P,
            Opcode::THUNK_C_R,
            Opcode::BOTTOM_P,
            Opcode::BOTTOM_R,
            Opcode::EVAL,
            Opcode::MATCH_A_E_P,
            Opcode::MATCH_A_E_R,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Opcode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Opcode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Opcode {
}

impl ::std::default::Default for Opcode {
    fn default() -> Self {
        Opcode::NOP
    }
}

impl ::protobuf::reflect::ProtobufValue for Opcode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0binstr.proto\"f\n\x04Func\x12\x14\n\x05arity\x18\x01\x20\x01(\rR\
    \x05arity\x12\x10\n\x03srt\x18\x02\x20\x03(\x06R\x03srt\x12\x1e\n\x06blo\
    cks\x18\x03\x20\x03(\x0b2\x06.BlockR\x06blocks\x12\x16\n\x06symbol\x18\
    \x04\x20\x01(\tR\x06symbol\";\n\x05Block\x12\x16\n\x06bitmap\x18\x01\x20\
    \x01(\x0cR\x06bitmap\x12\x1a\n\x04code\x18\x02\x20\x03(\x0b2\x06.InstrR\
    \x04code\"Y\n\x05Instr\x12\x1f\n\x06opcode\x18\x01\x20\x01(\x0e2\x07.Opc\
    odeR\x06opcode\x12\x13\n\x05p_num\x18\x02\x20\x01(\rR\x04pNum\x12\x1a\n\
    \x08operands\x18\x03\x20\x03(\rR\x08operands*\x85\x02\n\x06Opcode\x12\
    \x07\n\x03NOP\x10\0\x12\x08\n\x04JUMP\x10\x02\x12\x08\n\x04PUSH\x10\x03\
    \x12\n\n\x06RETURN\x10\x04\x12\x0c\n\x08PRIMOP_P\x10\x05\x12\x0c\n\x08PR\
    IMOP_R\x10\x06\x12\x0c\n\x08CONSTR_P\x10\x07\x12\x0c\n\x08CONSTR_R\x10\
    \x08\x12\n\n\x06FUNC_P\x10\t\x12\n\n\x06FUNC_R\x10\n\x12\r\n\tTHUNK_F_P\
    \x10\x0b\x12\r\n\tTHUNK_F_R\x10\x0c\x12\r\n\tTHUNK_C_P\x10\r\x12\r\n\tTH\
    UNK_C_R\x10\x0e\x12\x0c\n\x08BOTTOM_P\x10\x0f\x12\x0c\n\x08BOTTOM_R\x10\
    \x10\x12\x08\n\x04EVAL\x10\x11\x12\x0f\n\x0bMATCH_A_E_P\x10\x12\x12\x0f\
    \n\x0bMATCH_A_E_R\x10\x13J\x9a\x12\n\x06\x12\x04\0\03\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\xd8\x01\n\x02\x04\0\x12\x04\t\0\x0e\x01\x1a\xcb\
    \x01\x20sr:\x202\x20byte.\x20Stack\x20relative\x20address\n\x20st:\x208\
    \x20byte.\x20Static\x20object\x20pointer\n\x20cp:\x208\x20byte.\x20Code\
    \x20pointer\n\x20cr:\x202\x20byte.\x20Code\x20offset\n\x20k:\x20\x201\
    \x20byte.\x20Data\x20constructor\x20kind\n\x20po:\x201\x20byte.\x20PrimO\
    p\n\x20bo:\x202\x20byte.\x20Block\x20offset\n\n\n\n\x03\x04\0\x01\x12\
    \x03\t\x08\x0c\n\x0b\n\x04\x04\0\x02\0\x12\x03\n\x04\x20\n\r\n\x05\x04\0\
    \x02\0\x04\x12\x04\n\x04\t\x0e\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\n\x04\
    \n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\n\x15\x1a\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\n\x1e\x1f\n\x12\n\x04\x04\0\x02\x01\x12\x03\x0b\x04\x20\"\
    \x05\x20SRT\n\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x0b\x04\x0c\n\x0c\n\
    \x05\x04\0\x02\x01\x05\x12\x03\x0b\r\x14\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x03\x0b\x15\x18\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0b\x1e\x1f\n\
    \x0b\n\x04\x04\0\x02\x02\x12\x03\x0c\x04\x20\n\x0c\n\x05\x04\0\x02\x02\
    \x04\x12\x03\x0c\x04\x0c\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03\x0c\r\x12\
    \n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0c\x15\x1b\n\x0c\n\x05\x04\0\x02\
    \x02\x03\x12\x03\x0c\x1e\x1f\n\x0b\n\x04\x04\0\x02\x03\x12\x03\r\x04\x20\
    \n\r\n\x05\x04\0\x02\x03\x04\x12\x04\r\x04\x0c\x20\n\x0c\n\x05\x04\0\x02\
    \x03\x05\x12\x03\r\x04\n\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\r\x15\x1b\
    \n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\r\x1e\x1f\n\n\n\x02\x04\x01\x12\
    \x04\x10\0\x13\x01\n\n\n\x03\x04\x01\x01\x12\x03\x10\x08\r\n*\n\x04\x04\
    \x01\x02\0\x12\x03\x11\x04\x1e\"\x1d\x20Stack\x20payload\x20layout\x20bi\
    tmap\n\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x11\x04\x10\x0f\n\x0c\n\x05\
    \x04\x01\x02\0\x05\x12\x03\x11\x04\t\n\x0c\n\x05\x04\x01\x02\0\x01\x12\
    \x03\x11\x13\x19\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x11\x1c\x1d\n\x0b\
    \n\x04\x04\x01\x02\x01\x12\x03\x12\x04\x1e\n\x0c\n\x05\x04\x01\x02\x01\
    \x04\x12\x03\x12\x04\x0c\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03\x12\r\
    \x12\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x12\x13\x17\n\x0c\n\x05\x04\
    \x01\x02\x01\x03\x12\x03\x12\x1c\x1d\n\n\n\x02\x04\x02\x12\x04\x15\0\x19\
    \x01\n\n\n\x03\x04\x02\x01\x12\x03\x15\x08\r\n\x0b\n\x04\x04\x02\x02\0\
    \x12\x03\x16\x04!\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x16\x04\x15\x0f\n\
    \x0c\n\x05\x04\x02\x02\0\x06\x12\x03\x16\x04\n\n\x0c\n\x05\x04\x02\x02\0\
    \x01\x12\x03\x16\x14\x1a\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x16\x1f\
    \x20\n\x1d\n\x04\x04\x02\x02\x01\x12\x03\x17\x04!\"\x10\x20pointer\x20nu\
    mber\n\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\x17\x04\x16!\n\x0c\n\x05\
    \x04\x02\x02\x01\x05\x12\x03\x17\x04\n\n\x0c\n\x05\x04\x02\x02\x01\x01\
    \x12\x03\x17\x14\x19\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x17\x1f\x20\
    \n\x1e\n\x04\x04\x02\x02\x02\x12\x03\x18\x04!\"\x11\x20Actually\x2016\
    \x20bit\n\n\x0c\n\x05\x04\x02\x02\x02\x04\x12\x03\x18\x04\x0c\n\x0c\n\
    \x05\x04\x02\x02\x02\x05\x12\x03\x18\r\x13\n\x0c\n\x05\x04\x02\x02\x02\
    \x01\x12\x03\x18\x14\x1c\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x18\x1f\
    \x20\n\n\n\x02\x05\0\x12\x04\x1b\03\x01\n\n\n\x03\x05\0\x01\x12\x03\x1b\
    \x05\x0b\n\x0b\n\x04\x05\0\x02\0\x12\x03\x1c\x04\x14\n\x0c\n\x05\x05\0\
    \x02\0\x01\x12\x03\x1c\x04\x07\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x1c\
    \x12\x13\n\xac\x01\n\x04\x05\0\x02\x01\x12\x03!\x04\x14\"\x1b\x20Jump\
    \x20to\x20the\x20function\x20(cp)\n2\x81\x01\x20SREF\x20\x20\x20\x20\x20\
    \x20\x20\x20=\x201;\x20\x20//\x20Push\x20static\x20references\x20(pt)*\n\
    \x20BLOCK\x20\x20\x20\x20\x20\x20\x20=\x201;\x20\x20//\x20Tell\x20stack\
    \x20layout\x20(sl)\x20and\x20push\x20static\x20references\x20(pt)*\n\n\
    \x0c\n\x05\x05\0\x02\x01\x01\x12\x03!\x04\x08\n\x0c\n\x05\x05\0\x02\x01\
    \x02\x12\x03!\x12\x13\n%\n\x04\x05\0\x02\x02\x12\x03\"\x04\x14\"\x18\x20\
    Push\x20(copy)\x20local\x20(sr)\n\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\
    \"\x04\x08\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\"\x12\x13\n\x20\n\x04\
    \x05\0\x02\x03\x12\x03#\x04\x14\"\x13\x20Return\x20local\x20(sr)\n\n\x0c\
    \n\x05\x05\0\x02\x03\x01\x12\x03#\x04\n\n\x0c\n\x05\x05\0\x02\x03\x02\
    \x12\x03#\x12\x13\n+\n\x04\x05\0\x02\x04\x12\x03$\x04\x14\"\x1e\x20Primi\
    tive\x20operator\x20(po)\x20call\n\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03\
    $\x04\x0c\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03$\x12\x13\n\x0b\n\x04\x05\
    \0\x02\x05\x12\x03%\x04\x14\n\x0c\n\x05\x05\0\x02\x05\x01\x12\x03%\x04\
    \x0c\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03%\x12\x13\n8\n\x04\x05\0\x02\
    \x06\x12\x03&\x04\x14\"+\x20Make\x20a\x20data\x20cons\x20(k)\x20with\x20\
    arguments\x20(sr)*\n\n\x0c\n\x05\x05\0\x02\x06\x01\x12\x03&\x04\x0c\n\
    \x0c\n\x05\x05\0\x02\x06\x02\x12\x03&\x12\x13\n\x0b\n\x04\x05\0\x02\x07\
    \x12\x03'\x04\x14\n\x0c\n\x05\x05\0\x02\x07\x01\x12\x03'\x04\x0c\n\x0c\n\
    \x05\x05\0\x02\x07\x02\x12\x03'\x12\x13\n*\n\x04\x05\0\x02\x08\x12\x03(\
    \x04\x14\"\x1d\x20Make\x20a\x20function\x20(cp)\x20object\n\n\x0c\n\x05\
    \x05\0\x02\x08\x01\x12\x03(\x04\n\n\x0c\n\x05\x05\0\x02\x08\x02\x12\x03(\
    \x12\x13\n\x0b\n\x04\x05\0\x02\t\x12\x03)\x04\x15\n\x0c\n\x05\x05\0\x02\
    \t\x01\x12\x03)\x04\n\n\x0c\n\x05\x05\0\x02\t\x02\x12\x03)\x12\x14\nF\n\
    \x04\x05\0\x02\n\x12\x03*\x04\x15\"9\x20Make\x20a\x20thunk\x20with\x20th\
    e\x20function\x20(cp)\x20and\x20arguments\x20(sr)*\n\n\x0c\n\x05\x05\0\
    \x02\n\x01\x12\x03*\x04\r\n\x0c\n\x05\x05\0\x02\n\x02\x12\x03*\x12\x14\n\
    \x0b\n\x04\x05\0\x02\x0b\x12\x03+\x04\x15\n\x0c\n\x05\x05\0\x02\x0b\x01\
    \x12\x03+\x04\r\n\x0c\n\x05\x05\0\x02\x0b\x02\x12\x03+\x12\x14\nE\n\x04\
    \x05\0\x02\x0c\x12\x03,\x04\x15\"8\x20Make\x20a\x20thunk\x20with\x20the\
    \x20closure\x20(sr)\x20and\x20arguments\x20(sr)*\n\n\x0c\n\x05\x05\0\x02\
    \x0c\x01\x12\x03,\x04\r\n\x0c\n\x05\x05\0\x02\x0c\x02\x12\x03,\x12\x14\n\
    \x0b\n\x04\x05\0\x02\r\x12\x03-\x04\x15\n\x0c\n\x05\x05\0\x02\r\x01\x12\
    \x03-\x04\r\n\x0c\n\x05\x05\0\x02\r\x02\x12\x03-\x12\x14\n!\n\x04\x05\0\
    \x02\x0e\x12\x03.\x04\x15\"\x14\x20Make\x20bottom\x20object\n\n\x0c\n\
    \x05\x05\0\x02\x0e\x01\x12\x03.\x04\x0c\n\x0c\n\x05\x05\0\x02\x0e\x02\
    \x12\x03.\x12\x14\n\x0b\n\x04\x05\0\x02\x0f\x12\x03/\x04\x15\n\x0c\n\x05\
    \x05\0\x02\x0f\x01\x12\x03/\x04\x0c\n\x0c\n\x05\x05\0\x02\x0f\x02\x12\
    \x03/\x12\x14\n\"\n\x04\x05\0\x02\x10\x12\x030\x04\x15\"\x15\x20eval\x20\
    closures\x20(sr)*\n\n\x0c\n\x05\x05\0\x02\x10\x01\x12\x030\x04\x08\n\x0c\
    \n\x05\x05\0\x02\x10\x02\x12\x030\x12\x14\n=\n\x04\x05\0\x02\x11\x12\x03\
    1\x04\x15\"0\x20Eval\x20ADT\x20(sr)\x20and\x20pattern\x20match,\x20jump\
    \x20by\x20(bo)*\n\n\x0c\n\x05\x05\0\x02\x11\x01\x12\x031\x04\x0f\n\x0c\n\
    \x05\x05\0\x02\x11\x02\x12\x031\x12\x14\n\x0b\n\x04\x05\0\x02\x12\x12\
    \x032\x04\x15\n\x0c\n\x05\x05\0\x02\x12\x01\x12\x032\x04\x0f\n\x0c\n\x05\
    \x05\0\x02\x12\x02\x12\x032\x12\x14b\x06proto3\
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
