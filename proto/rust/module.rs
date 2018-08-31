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
pub struct HObject {
    // message fields
    pub name: ::std::string::String,
    pub major_version: i64,
    pub minor_version: i64,
    pub entry_point: u64,
    pub local_offset: u64,
    pub imports: ::std::collections::HashMap<u64, Symbol>,
    pub caf_exports: ::std::vec::Vec<u64>,
    pub code_exports: ::std::vec::Vec<u64>,
    pub cafs: ::protobuf::RepeatedField<super::expr::Expr>,
    pub text: ::protobuf::RepeatedField<super::instr::Func>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HObject {}

impl HObject {
    pub fn new() -> HObject {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HObject {
        static mut instance: ::protobuf::lazy::Lazy<HObject> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HObject,
        };
        unsafe {
            instance.get(HObject::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // int64 major_version = 2;

    pub fn clear_major_version(&mut self) {
        self.major_version = 0;
    }

    // Param is passed by value, moved
    pub fn set_major_version(&mut self, v: i64) {
        self.major_version = v;
    }

    pub fn get_major_version(&self) -> i64 {
        self.major_version
    }

    fn get_major_version_for_reflect(&self) -> &i64 {
        &self.major_version
    }

    fn mut_major_version_for_reflect(&mut self) -> &mut i64 {
        &mut self.major_version
    }

    // int64 minor_version = 3;

    pub fn clear_minor_version(&mut self) {
        self.minor_version = 0;
    }

    // Param is passed by value, moved
    pub fn set_minor_version(&mut self, v: i64) {
        self.minor_version = v;
    }

    pub fn get_minor_version(&self) -> i64 {
        self.minor_version
    }

    fn get_minor_version_for_reflect(&self) -> &i64 {
        &self.minor_version
    }

    fn mut_minor_version_for_reflect(&mut self) -> &mut i64 {
        &mut self.minor_version
    }

    // fixed64 entry_point = 5;

    pub fn clear_entry_point(&mut self) {
        self.entry_point = 0;
    }

    // Param is passed by value, moved
    pub fn set_entry_point(&mut self, v: u64) {
        self.entry_point = v;
    }

    pub fn get_entry_point(&self) -> u64 {
        self.entry_point
    }

    fn get_entry_point_for_reflect(&self) -> &u64 {
        &self.entry_point
    }

    fn mut_entry_point_for_reflect(&mut self) -> &mut u64 {
        &mut self.entry_point
    }

    // fixed64 local_offset = 6;

    pub fn clear_local_offset(&mut self) {
        self.local_offset = 0;
    }

    // Param is passed by value, moved
    pub fn set_local_offset(&mut self, v: u64) {
        self.local_offset = v;
    }

    pub fn get_local_offset(&self) -> u64 {
        self.local_offset
    }

    fn get_local_offset_for_reflect(&self) -> &u64 {
        &self.local_offset
    }

    fn mut_local_offset_for_reflect(&mut self) -> &mut u64 {
        &mut self.local_offset
    }

    // repeated .HObject.ImportsEntry imports = 10;

    pub fn clear_imports(&mut self) {
        self.imports.clear();
    }

    // Param is passed by value, moved
    pub fn set_imports(&mut self, v: ::std::collections::HashMap<u64, Symbol>) {
        self.imports = v;
    }

    // Mutable pointer to the field.
    pub fn mut_imports(&mut self) -> &mut ::std::collections::HashMap<u64, Symbol> {
        &mut self.imports
    }

    // Take field
    pub fn take_imports(&mut self) -> ::std::collections::HashMap<u64, Symbol> {
        ::std::mem::replace(&mut self.imports, ::std::collections::HashMap::new())
    }

    pub fn get_imports(&self) -> &::std::collections::HashMap<u64, Symbol> {
        &self.imports
    }

    fn get_imports_for_reflect(&self) -> &::std::collections::HashMap<u64, Symbol> {
        &self.imports
    }

    fn mut_imports_for_reflect(&mut self) -> &mut ::std::collections::HashMap<u64, Symbol> {
        &mut self.imports
    }

    // repeated fixed64 caf_exports = 11;

    pub fn clear_caf_exports(&mut self) {
        self.caf_exports.clear();
    }

    // Param is passed by value, moved
    pub fn set_caf_exports(&mut self, v: ::std::vec::Vec<u64>) {
        self.caf_exports = v;
    }

    // Mutable pointer to the field.
    pub fn mut_caf_exports(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.caf_exports
    }

    // Take field
    pub fn take_caf_exports(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.caf_exports, ::std::vec::Vec::new())
    }

    pub fn get_caf_exports(&self) -> &[u64] {
        &self.caf_exports
    }

    fn get_caf_exports_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.caf_exports
    }

    fn mut_caf_exports_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.caf_exports
    }

    // repeated fixed64 code_exports = 12;

    pub fn clear_code_exports(&mut self) {
        self.code_exports.clear();
    }

    // Param is passed by value, moved
    pub fn set_code_exports(&mut self, v: ::std::vec::Vec<u64>) {
        self.code_exports = v;
    }

    // Mutable pointer to the field.
    pub fn mut_code_exports(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.code_exports
    }

    // Take field
    pub fn take_code_exports(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.code_exports, ::std::vec::Vec::new())
    }

    pub fn get_code_exports(&self) -> &[u64] {
        &self.code_exports
    }

    fn get_code_exports_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.code_exports
    }

    fn mut_code_exports_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.code_exports
    }

    // repeated .Expr cafs = 13;

    pub fn clear_cafs(&mut self) {
        self.cafs.clear();
    }

    // Param is passed by value, moved
    pub fn set_cafs(&mut self, v: ::protobuf::RepeatedField<super::expr::Expr>) {
        self.cafs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cafs(&mut self) -> &mut ::protobuf::RepeatedField<super::expr::Expr> {
        &mut self.cafs
    }

    // Take field
    pub fn take_cafs(&mut self) -> ::protobuf::RepeatedField<super::expr::Expr> {
        ::std::mem::replace(&mut self.cafs, ::protobuf::RepeatedField::new())
    }

    pub fn get_cafs(&self) -> &[super::expr::Expr] {
        &self.cafs
    }

    fn get_cafs_for_reflect(&self) -> &::protobuf::RepeatedField<super::expr::Expr> {
        &self.cafs
    }

    fn mut_cafs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::expr::Expr> {
        &mut self.cafs
    }

    // repeated .Func text = 14;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::protobuf::RepeatedField<super::instr::Func>) {
        self.text = v;
    }

    // Mutable pointer to the field.
    pub fn mut_text(&mut self) -> &mut ::protobuf::RepeatedField<super::instr::Func> {
        &mut self.text
    }

    // Take field
    pub fn take_text(&mut self) -> ::protobuf::RepeatedField<super::instr::Func> {
        ::std::mem::replace(&mut self.text, ::protobuf::RepeatedField::new())
    }

    pub fn get_text(&self) -> &[super::instr::Func] {
        &self.text
    }

    fn get_text_for_reflect(&self) -> &::protobuf::RepeatedField<super::instr::Func> {
        &self.text
    }

    fn mut_text_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::instr::Func> {
        &mut self.text
    }
}

impl ::protobuf::Message for HObject {
    fn is_initialized(&self) -> bool {
        for v in &self.cafs {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.text {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.major_version = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.minor_version = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.entry_point = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.local_offset = tmp;
                },
                10 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeFixed64, ::protobuf::types::ProtobufTypeMessage<Symbol>>(wire_type, is, &mut self.imports)?;
                },
                11 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.caf_exports)?;
                },
                12 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.code_exports)?;
                },
                13 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cafs)?;
                },
                14 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.text)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if self.major_version != 0 {
            my_size += ::protobuf::rt::value_size(2, self.major_version, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.minor_version != 0 {
            my_size += ::protobuf::rt::value_size(3, self.minor_version, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.entry_point != 0 {
            my_size += 9;
        }
        if self.local_offset != 0 {
            my_size += 9;
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeFixed64, ::protobuf::types::ProtobufTypeMessage<Symbol>>(10, &self.imports);
        my_size += 9 * self.caf_exports.len() as u32;
        my_size += 9 * self.code_exports.len() as u32;
        for value in &self.cafs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.text {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.major_version != 0 {
            os.write_int64(2, self.major_version)?;
        }
        if self.minor_version != 0 {
            os.write_int64(3, self.minor_version)?;
        }
        if self.entry_point != 0 {
            os.write_fixed64(5, self.entry_point)?;
        }
        if self.local_offset != 0 {
            os.write_fixed64(6, self.local_offset)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeFixed64, ::protobuf::types::ProtobufTypeMessage<Symbol>>(10, &self.imports, os)?;
        for v in &self.caf_exports {
            os.write_fixed64(11, *v)?;
        };
        for v in &self.code_exports {
            os.write_fixed64(12, *v)?;
        };
        for v in &self.cafs {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.text {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for HObject {
    fn new() -> HObject {
        HObject::new()
    }

    fn descriptor_static(_: ::std::option::Option<HObject>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    HObject::get_name_for_reflect,
                    HObject::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "major_version",
                    HObject::get_major_version_for_reflect,
                    HObject::mut_major_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "minor_version",
                    HObject::get_minor_version_for_reflect,
                    HObject::mut_minor_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "entry_point",
                    HObject::get_entry_point_for_reflect,
                    HObject::mut_entry_point_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "local_offset",
                    HObject::get_local_offset_for_reflect,
                    HObject::mut_local_offset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeFixed64, ::protobuf::types::ProtobufTypeMessage<Symbol>>(
                    "imports",
                    HObject::get_imports_for_reflect,
                    HObject::mut_imports_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "caf_exports",
                    HObject::get_caf_exports_for_reflect,
                    HObject::mut_caf_exports_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "code_exports",
                    HObject::get_code_exports_for_reflect,
                    HObject::mut_code_exports_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::expr::Expr>>(
                    "cafs",
                    HObject::get_cafs_for_reflect,
                    HObject::mut_cafs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::instr::Func>>(
                    "text",
                    HObject::get_text_for_reflect,
                    HObject::mut_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HObject>(
                    "HObject",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HObject {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_major_version();
        self.clear_minor_version();
        self.clear_entry_point();
        self.clear_local_offset();
        self.clear_imports();
        self.clear_caf_exports();
        self.clear_code_exports();
        self.clear_cafs();
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HObject {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Symbol {
    // message fields
    pub package: i32,
    pub module: i32,
    pub local: u64,
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Symbol {}

impl Symbol {
    pub fn new() -> Symbol {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Symbol {
        static mut instance: ::protobuf::lazy::Lazy<Symbol> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Symbol,
        };
        unsafe {
            instance.get(Symbol::new)
        }
    }

    // int32 package = 1;

    pub fn clear_package(&mut self) {
        self.package = 0;
    }

    // Param is passed by value, moved
    pub fn set_package(&mut self, v: i32) {
        self.package = v;
    }

    pub fn get_package(&self) -> i32 {
        self.package
    }

    fn get_package_for_reflect(&self) -> &i32 {
        &self.package
    }

    fn mut_package_for_reflect(&mut self) -> &mut i32 {
        &mut self.package
    }

    // int32 module = 2;

    pub fn clear_module(&mut self) {
        self.module = 0;
    }

    // Param is passed by value, moved
    pub fn set_module(&mut self, v: i32) {
        self.module = v;
    }

    pub fn get_module(&self) -> i32 {
        self.module
    }

    fn get_module_for_reflect(&self) -> &i32 {
        &self.module
    }

    fn mut_module_for_reflect(&mut self) -> &mut i32 {
        &mut self.module
    }

    // fixed64 local = 3;

    pub fn clear_local(&mut self) {
        self.local = 0;
    }

    // Param is passed by value, moved
    pub fn set_local(&mut self, v: u64) {
        self.local = v;
    }

    pub fn get_local(&self) -> u64 {
        self.local
    }

    fn get_local_for_reflect(&self) -> &u64 {
        &self.local
    }

    fn mut_local_for_reflect(&mut self) -> &mut u64 {
        &mut self.local
    }

    // string name = 4;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
}

impl ::protobuf::Message for Symbol {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_int32()?;
                    self.package = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.module = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.local = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
        if self.package != 0 {
            my_size += ::protobuf::rt::value_size(1, self.package, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.module != 0 {
            my_size += ::protobuf::rt::value_size(2, self.module, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.local != 0 {
            my_size += 9;
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.package != 0 {
            os.write_int32(1, self.package)?;
        }
        if self.module != 0 {
            os.write_int32(2, self.module)?;
        }
        if self.local != 0 {
            os.write_fixed64(3, self.local)?;
        }
        if !self.name.is_empty() {
            os.write_string(4, &self.name)?;
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

impl ::protobuf::MessageStatic for Symbol {
    fn new() -> Symbol {
        Symbol::new()
    }

    fn descriptor_static(_: ::std::option::Option<Symbol>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "package",
                    Symbol::get_package_for_reflect,
                    Symbol::mut_package_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "module",
                    Symbol::get_module_for_reflect,
                    Symbol::mut_module_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "local",
                    Symbol::get_local_for_reflect,
                    Symbol::mut_local_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Symbol::get_name_for_reflect,
                    Symbol::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Symbol>(
                    "Symbol",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Symbol {
    fn clear(&mut self) {
        self.clear_package();
        self.clear_module();
        self.clear_local();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Symbol {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cmodule.proto\x1a\nexpr.proto\x1a\x0binstr.proto\"\x9b\x03\n\x07HOb\
    ject\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12#\n\rmajor_version\
    \x18\x02\x20\x01(\x03R\x0cmajorVersion\x12#\n\rminor_version\x18\x03\x20\
    \x01(\x03R\x0cminorVersion\x12\x1f\n\x0bentry_point\x18\x05\x20\x01(\x06\
    R\nentryPoint\x12!\n\x0clocal_offset\x18\x06\x20\x01(\x06R\x0blocalOffse\
    t\x12/\n\x07imports\x18\n\x20\x03(\x0b2\x15.HObject.ImportsEntryR\x07imp\
    orts\x12\x1f\n\x0bcaf_exports\x18\x0b\x20\x03(\x06R\ncafExports\x12!\n\
    \x0ccode_exports\x18\x0c\x20\x03(\x06R\x0bcodeExports\x12\x19\n\x04cafs\
    \x18\r\x20\x03(\x0b2\x05.ExprR\x04cafs\x12\x19\n\x04text\x18\x0e\x20\x03\
    (\x0b2\x05.FuncR\x04text\x1aC\n\x0cImportsEntry\x12\x10\n\x03key\x18\x01\
    \x20\x01(\x06R\x03key\x12\x1d\n\x05value\x18\x02\x20\x01(\x0b2\x07.Symbo\
    lR\x05value:\x028\x01\"d\n\x06Symbol\x12\x18\n\x07package\x18\x01\x20\
    \x01(\x05R\x07package\x12\x16\n\x06module\x18\x02\x20\x01(\x05R\x06modul\
    e\x12\x14\n\x05local\x18\x03\x20\x01(\x06R\x05local\x12\x12\n\x04name\
    \x18\x04\x20\x01(\tR\x04nameJ\xef\x08\n\x06\x12\x04\0\0\x1d\x01\n\x08\n\
    \x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\x07\x13\n\t\n\x02\
    \x03\x01\x12\x03\x03\x07\x14\n\x1f\n\x02\x04\0\x12\x04\x06\0\x16\x01\x1a\
    \x13\x20Holon\x20Object\x20File\n\n\n\n\x03\x04\0\x01\x12\x03\x06\x08\
    \x0f\n\x0b\n\x04\x04\0\x02\0\x12\x03\x07\x04\x14\n\r\n\x05\x04\0\x02\0\
    \x04\x12\x04\x07\x04\x06\x11\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x07\x04\
    \n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x07\x0b\x0f\n\x0c\n\x05\x04\0\x02\
    \0\x03\x12\x03\x07\x12\x13\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x08\x04\x1d\
    \n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x08\x04\x07\x14\n\x0c\n\x05\x04\0\
    \x02\x01\x05\x12\x03\x08\x04\t\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x08\
    \x0b\x18\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x08\x1b\x1c\n\x0b\n\x04\
    \x04\0\x02\x02\x12\x03\t\x04\x1d\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\t\
    \x04\x08\x1d\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\t\x04\t\n\x0c\n\x05\
    \x04\0\x02\x02\x01\x12\x03\t\x0b\x18\n\x0c\n\x05\x04\0\x02\x02\x03\x12\
    \x03\t\x1b\x1c\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x0b\x04\x1d\n\r\n\x05\
    \x04\0\x02\x03\x04\x12\x04\x0b\x04\t\x1d\n\x0c\n\x05\x04\0\x02\x03\x05\
    \x12\x03\x0b\x04\x0b\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x0b\x0c\x17\n\
    \x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x0b\x1b\x1c\n\x0b\n\x04\x04\0\x02\
    \x04\x12\x03\x0c\x04\x1d\n\r\n\x05\x04\0\x02\x04\x04\x12\x04\x0c\x04\x0b\
    \x1d\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\x0c\x04\x0b\n\x0c\n\x05\x04\0\
    \x02\x04\x01\x12\x03\x0c\x0c\x18\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\
    \x0c\x1b\x1c\n\x0b\n\x04\x04\0\x02\x05\x12\x03\x0e\x04'\n\r\n\x05\x04\0\
    \x02\x05\x04\x12\x04\x0e\x04\x0c\x1d\n\x0c\n\x05\x04\0\x02\x05\x06\x12\
    \x03\x0e\x04\x18\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\x0e\x19\x20\n\x0c\
    \n\x05\x04\0\x02\x05\x03\x12\x03\x0e$&\n\x0b\n\x04\x04\0\x02\x06\x12\x03\
    \x0f\x04'\n\x0c\n\x05\x04\0\x02\x06\x04\x12\x03\x0f\x04\x0c\n\x0c\n\x05\
    \x04\0\x02\x06\x05\x12\x03\x0f\r\x14\n\x0c\n\x05\x04\0\x02\x06\x01\x12\
    \x03\x0f\x15\x20\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\x0f$&\n\x0b\n\x04\
    \x04\0\x02\x07\x12\x03\x10\x04'\n\x0c\n\x05\x04\0\x02\x07\x04\x12\x03\
    \x10\x04\x0c\n\x0c\n\x05\x04\0\x02\x07\x05\x12\x03\x10\r\x14\n\x0c\n\x05\
    \x04\0\x02\x07\x01\x12\x03\x10\x15!\n\x0c\n\x05\x04\0\x02\x07\x03\x12\
    \x03\x10$&\n=\n\x04\x04\0\x02\x08\x12\x03\x14\x04\x1c\x1a0\x20fixed64\
    \x20caf_num\x20\x20\x20=\x203;\n\x20fixed64\x20table_num\x20=\x204;\n\n\
    \x0c\n\x05\x04\0\x02\x08\x04\x12\x03\x14\x04\x0c\n\x0c\n\x05\x04\0\x02\
    \x08\x06\x12\x03\x14\r\x11\n\x0c\n\x05\x04\0\x02\x08\x01\x12\x03\x14\x12\
    \x16\n\x0c\n\x05\x04\0\x02\x08\x03\x12\x03\x14\x19\x1b\n\x0b\n\x04\x04\0\
    \x02\t\x12\x03\x15\x04\x1c\n\x0c\n\x05\x04\0\x02\t\x04\x12\x03\x15\x04\
    \x0c\n\x0c\n\x05\x04\0\x02\t\x06\x12\x03\x15\r\x11\n\x0c\n\x05\x04\0\x02\
    \t\x01\x12\x03\x15\x12\x16\n\x0c\n\x05\x04\0\x02\t\x03\x12\x03\x15\x19\
    \x1b\n\n\n\x02\x04\x01\x12\x04\x18\0\x1d\x01\n\n\n\x03\x04\x01\x01\x12\
    \x03\x18\x08\x0e\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x19\x04\x18\n\r\n\x05\
    \x04\x01\x02\0\x04\x12\x04\x19\x04\x18\x10\n\x0c\n\x05\x04\x01\x02\0\x05\
    \x12\x03\x19\x04\t\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x19\x0c\x13\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x19\x16\x17\n\x0b\n\x04\x04\x01\x02\
    \x01\x12\x03\x1a\x04\x18\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x1a\x04\
    \x19\x18\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x1a\x04\t\n\x0c\n\x05\
    \x04\x01\x02\x01\x01\x12\x03\x1a\x0c\x12\n\x0c\n\x05\x04\x01\x02\x01\x03\
    \x12\x03\x1a\x16\x17\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x1b\x04\x18\n\r\
    \n\x05\x04\x01\x02\x02\x04\x12\x04\x1b\x04\x1a\x18\n\x0c\n\x05\x04\x01\
    \x02\x02\x05\x12\x03\x1b\x04\x0b\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\
    \x1b\x0c\x11\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x1b\x16\x17\n\x0b\n\
    \x04\x04\x01\x02\x03\x12\x03\x1c\x04\x18\n\r\n\x05\x04\x01\x02\x03\x04\
    \x12\x04\x1c\x04\x1b\x18\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03\x1c\x04\
    \n\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\x1c\x0c\x10\n\x0c\n\x05\x04\
    \x01\x02\x03\x03\x12\x03\x1c\x16\x17b\x06proto3\
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
