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
pub struct Package {
    // message fields
    pub magic: u64,
    pub name: ::std::string::String,
    pub major: u32,
    pub minor: u32,
    pub revision: u32,
    pub build: u32,
    pub deps: ::protobuf::RepeatedField<::std::string::String>,
    pub modules: ::protobuf::RepeatedField<Module>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Package {}

impl Package {
    pub fn new() -> Package {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Package {
        static mut instance: ::protobuf::lazy::Lazy<Package> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Package,
        };
        unsafe {
            instance.get(Package::new)
        }
    }

    // fixed64 magic = 1;

    pub fn clear_magic(&mut self) {
        self.magic = 0;
    }

    // Param is passed by value, moved
    pub fn set_magic(&mut self, v: u64) {
        self.magic = v;
    }

    pub fn get_magic(&self) -> u64 {
        self.magic
    }

    fn get_magic_for_reflect(&self) -> &u64 {
        &self.magic
    }

    fn mut_magic_for_reflect(&mut self) -> &mut u64 {
        &mut self.magic
    }

    // string name = 2;

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

    // uint32 major = 3;

    pub fn clear_major(&mut self) {
        self.major = 0;
    }

    // Param is passed by value, moved
    pub fn set_major(&mut self, v: u32) {
        self.major = v;
    }

    pub fn get_major(&self) -> u32 {
        self.major
    }

    fn get_major_for_reflect(&self) -> &u32 {
        &self.major
    }

    fn mut_major_for_reflect(&mut self) -> &mut u32 {
        &mut self.major
    }

    // uint32 minor = 4;

    pub fn clear_minor(&mut self) {
        self.minor = 0;
    }

    // Param is passed by value, moved
    pub fn set_minor(&mut self, v: u32) {
        self.minor = v;
    }

    pub fn get_minor(&self) -> u32 {
        self.minor
    }

    fn get_minor_for_reflect(&self) -> &u32 {
        &self.minor
    }

    fn mut_minor_for_reflect(&mut self) -> &mut u32 {
        &mut self.minor
    }

    // uint32 revision = 5;

    pub fn clear_revision(&mut self) {
        self.revision = 0;
    }

    // Param is passed by value, moved
    pub fn set_revision(&mut self, v: u32) {
        self.revision = v;
    }

    pub fn get_revision(&self) -> u32 {
        self.revision
    }

    fn get_revision_for_reflect(&self) -> &u32 {
        &self.revision
    }

    fn mut_revision_for_reflect(&mut self) -> &mut u32 {
        &mut self.revision
    }

    // uint32 build = 6;

    pub fn clear_build(&mut self) {
        self.build = 0;
    }

    // Param is passed by value, moved
    pub fn set_build(&mut self, v: u32) {
        self.build = v;
    }

    pub fn get_build(&self) -> u32 {
        self.build
    }

    fn get_build_for_reflect(&self) -> &u32 {
        &self.build
    }

    fn mut_build_for_reflect(&mut self) -> &mut u32 {
        &mut self.build
    }

    // repeated string deps = 10;

    pub fn clear_deps(&mut self) {
        self.deps.clear();
    }

    // Param is passed by value, moved
    pub fn set_deps(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.deps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_deps(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.deps
    }

    // Take field
    pub fn take_deps(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.deps, ::protobuf::RepeatedField::new())
    }

    pub fn get_deps(&self) -> &[::std::string::String] {
        &self.deps
    }

    fn get_deps_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.deps
    }

    fn mut_deps_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.deps
    }

    // repeated .Module modules = 11;

    pub fn clear_modules(&mut self) {
        self.modules.clear();
    }

    // Param is passed by value, moved
    pub fn set_modules(&mut self, v: ::protobuf::RepeatedField<Module>) {
        self.modules = v;
    }

    // Mutable pointer to the field.
    pub fn mut_modules(&mut self) -> &mut ::protobuf::RepeatedField<Module> {
        &mut self.modules
    }

    // Take field
    pub fn take_modules(&mut self) -> ::protobuf::RepeatedField<Module> {
        ::std::mem::replace(&mut self.modules, ::protobuf::RepeatedField::new())
    }

    pub fn get_modules(&self) -> &[Module] {
        &self.modules
    }

    fn get_modules_for_reflect(&self) -> &::protobuf::RepeatedField<Module> {
        &self.modules
    }

    fn mut_modules_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Module> {
        &mut self.modules
    }
}

impl ::protobuf::Message for Package {
    fn is_initialized(&self) -> bool {
        for v in &self.modules {
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
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.magic = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.major = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.minor = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.revision = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.build = tmp;
                },
                10 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.deps)?;
                },
                11 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.modules)?;
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
        if self.magic != 0 {
            my_size += 9;
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if self.major != 0 {
            my_size += ::protobuf::rt::value_size(3, self.major, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.minor != 0 {
            my_size += ::protobuf::rt::value_size(4, self.minor, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.revision != 0 {
            my_size += ::protobuf::rt::value_size(5, self.revision, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.build != 0 {
            my_size += ::protobuf::rt::value_size(6, self.build, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.deps {
            my_size += ::protobuf::rt::string_size(10, &value);
        };
        for value in &self.modules {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.magic != 0 {
            os.write_fixed64(1, self.magic)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if self.major != 0 {
            os.write_uint32(3, self.major)?;
        }
        if self.minor != 0 {
            os.write_uint32(4, self.minor)?;
        }
        if self.revision != 0 {
            os.write_uint32(5, self.revision)?;
        }
        if self.build != 0 {
            os.write_uint32(6, self.build)?;
        }
        for v in &self.deps {
            os.write_string(10, &v)?;
        };
        for v in &self.modules {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Package {
    fn new() -> Package {
        Package::new()
    }

    fn descriptor_static(_: ::std::option::Option<Package>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "magic",
                    Package::get_magic_for_reflect,
                    Package::mut_magic_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Package::get_name_for_reflect,
                    Package::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "major",
                    Package::get_major_for_reflect,
                    Package::mut_major_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "minor",
                    Package::get_minor_for_reflect,
                    Package::mut_minor_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "revision",
                    Package::get_revision_for_reflect,
                    Package::mut_revision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "build",
                    Package::get_build_for_reflect,
                    Package::mut_build_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "deps",
                    Package::get_deps_for_reflect,
                    Package::mut_deps_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Module>>(
                    "modules",
                    Package::get_modules_for_reflect,
                    Package::mut_modules_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Package>(
                    "Package",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Package {
    fn clear(&mut self) {
        self.clear_magic();
        self.clear_name();
        self.clear_major();
        self.clear_minor();
        self.clear_revision();
        self.clear_build();
        self.clear_deps();
        self.clear_modules();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Package {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Package {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Module {
    // message fields
    pub name: ::std::string::String,
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
unsafe impl ::std::marker::Sync for Module {}

impl Module {
    pub fn new() -> Module {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Module {
        static mut instance: ::protobuf::lazy::Lazy<Module> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Module,
        };
        unsafe {
            instance.get(Module::new)
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

    // fixed64 entry_point = 2;

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

    // fixed64 local_offset = 3;

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

    // repeated .Module.ImportsEntry imports = 8;

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

    // repeated fixed64 caf_exports = 9;

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

    // repeated fixed64 code_exports = 10;

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

    // repeated .Expr cafs = 11;

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

    // repeated .Func text = 12;

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

impl ::protobuf::Message for Module {
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
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.entry_point = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.local_offset = tmp;
                },
                8 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeFixed64, ::protobuf::types::ProtobufTypeMessage<Symbol>>(wire_type, is, &mut self.imports)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.caf_exports)?;
                },
                10 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.code_exports)?;
                },
                11 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cafs)?;
                },
                12 => {
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
        if self.entry_point != 0 {
            my_size += 9;
        }
        if self.local_offset != 0 {
            my_size += 9;
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeFixed64, ::protobuf::types::ProtobufTypeMessage<Symbol>>(8, &self.imports);
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
        if self.entry_point != 0 {
            os.write_fixed64(2, self.entry_point)?;
        }
        if self.local_offset != 0 {
            os.write_fixed64(3, self.local_offset)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeFixed64, ::protobuf::types::ProtobufTypeMessage<Symbol>>(8, &self.imports, os)?;
        for v in &self.caf_exports {
            os.write_fixed64(9, *v)?;
        };
        for v in &self.code_exports {
            os.write_fixed64(10, *v)?;
        };
        for v in &self.cafs {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.text {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Module {
    fn new() -> Module {
        Module::new()
    }

    fn descriptor_static(_: ::std::option::Option<Module>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Module::get_name_for_reflect,
                    Module::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "entry_point",
                    Module::get_entry_point_for_reflect,
                    Module::mut_entry_point_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "local_offset",
                    Module::get_local_offset_for_reflect,
                    Module::mut_local_offset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeFixed64, ::protobuf::types::ProtobufTypeMessage<Symbol>>(
                    "imports",
                    Module::get_imports_for_reflect,
                    Module::mut_imports_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "caf_exports",
                    Module::get_caf_exports_for_reflect,
                    Module::mut_caf_exports_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "code_exports",
                    Module::get_code_exports_for_reflect,
                    Module::mut_code_exports_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::expr::Expr>>(
                    "cafs",
                    Module::get_cafs_for_reflect,
                    Module::mut_cafs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::instr::Func>>(
                    "text",
                    Module::get_text_for_reflect,
                    Module::mut_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Module>(
                    "Module",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Module {
    fn clear(&mut self) {
        self.clear_name();
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

impl ::std::fmt::Debug for Module {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Module {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Symbol {
    // message fields
    pub package: u32,
    pub module: u32,
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

    // uint32 package = 1;

    pub fn clear_package(&mut self) {
        self.package = 0;
    }

    // Param is passed by value, moved
    pub fn set_package(&mut self, v: u32) {
        self.package = v;
    }

    pub fn get_package(&self) -> u32 {
        self.package
    }

    fn get_package_for_reflect(&self) -> &u32 {
        &self.package
    }

    fn mut_package_for_reflect(&mut self) -> &mut u32 {
        &mut self.package
    }

    // uint32 module = 2;

    pub fn clear_module(&mut self) {
        self.module = 0;
    }

    // Param is passed by value, moved
    pub fn set_module(&mut self, v: u32) {
        self.module = v;
    }

    pub fn get_module(&self) -> u32 {
        self.module
    }

    fn get_module_for_reflect(&self) -> &u32 {
        &self.module
    }

    fn mut_module_for_reflect(&mut self) -> &mut u32 {
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
                    let tmp = is.read_uint32()?;
                    self.package = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
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
            os.write_uint32(1, self.package)?;
        }
        if self.module != 0 {
            os.write_uint32(2, self.module)?;
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "package",
                    Symbol::get_package_for_reflect,
                    Symbol::mut_package_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
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
    \n\x0cmodule.proto\x1a\nexpr.proto\x1a\x0binstr.proto\"\xc8\x01\n\x07Pac\
    kage\x12\x14\n\x05magic\x18\x01\x20\x01(\x06R\x05magic\x12\x12\n\x04name\
    \x18\x02\x20\x01(\tR\x04name\x12\x14\n\x05major\x18\x03\x20\x01(\rR\x05m\
    ajor\x12\x14\n\x05minor\x18\x04\x20\x01(\rR\x05minor\x12\x1a\n\x08revisi\
    on\x18\x05\x20\x01(\rR\x08revision\x12\x14\n\x05build\x18\x06\x20\x01(\r\
    R\x05build\x12\x12\n\x04deps\x18\n\x20\x03(\tR\x04deps\x12!\n\x07modules\
    \x18\x0b\x20\x03(\x0b2\x07.ModuleR\x07modules\"\xcf\x02\n\x06Module\x12\
    \x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x1f\n\x0bentry_point\x18\
    \x02\x20\x01(\x06R\nentryPoint\x12!\n\x0clocal_offset\x18\x03\x20\x01(\
    \x06R\x0blocalOffset\x12.\n\x07imports\x18\x08\x20\x03(\x0b2\x14.Module.\
    ImportsEntryR\x07imports\x12\x1f\n\x0bcaf_exports\x18\t\x20\x03(\x06R\nc\
    afExports\x12!\n\x0ccode_exports\x18\n\x20\x03(\x06R\x0bcodeExports\x12\
    \x19\n\x04cafs\x18\x0b\x20\x03(\x0b2\x05.ExprR\x04cafs\x12\x19\n\x04text\
    \x18\x0c\x20\x03(\x0b2\x05.FuncR\x04text\x1aC\n\x0cImportsEntry\x12\x10\
    \n\x03key\x18\x01\x20\x01(\x06R\x03key\x12\x1d\n\x05value\x18\x02\x20\
    \x01(\x0b2\x07.SymbolR\x05value:\x028\x01\"d\n\x06Symbol\x12\x18\n\x07pa\
    ckage\x18\x01\x20\x01(\rR\x07package\x12\x16\n\x06module\x18\x02\x20\x01\
    (\rR\x06module\x12\x14\n\x05local\x18\x03\x20\x01(\x06R\x05local\x12\x12\
    \n\x04name\x18\x04\x20\x01(\tR\x04nameJ\xa3\x0c\n\x06\x12\x04\0\0(\x01\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\x07\x13\n\t\n\
    \x02\x03\x01\x12\x03\x03\x07\x14\n\x1a\n\x02\x04\0\x12\x04\x06\0\x12\x01\
    \x1a\x0e\x20Package\x20File\n\n\n\n\x03\x04\0\x01\x12\x03\x06\x08\x0f\n\
    \x0b\n\x04\x04\0\x02\0\x12\x03\x07\x04\x16\n\r\n\x05\x04\0\x02\0\x04\x12\
    \x04\x07\x04\x06\x11\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x07\x04\x0b\n\
    \x0c\n\x05\x04\0\x02\0\x01\x12\x03\x07\x0c\x11\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\x07\x14\x15\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x08\x04\x16\n\
    \r\n\x05\x04\0\x02\x01\x04\x12\x04\x08\x04\x07\x16\n\x0c\n\x05\x04\0\x02\
    \x01\x05\x12\x03\x08\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x08\x0c\
    \x10\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x08\x14\x15\n\x16\n\x04\x04\0\
    \x02\x02\x12\x03\x0b\x04\x19\x1a\t\x20version\n\n\r\n\x05\x04\0\x02\x02\
    \x04\x12\x04\x0b\x04\x08\x16\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x0b\
    \x04\n\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0b\x0c\x11\n\x0c\n\x05\x04\
    \0\x02\x02\x03\x12\x03\x0b\x17\x18\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x0c\
    \x04\x19\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\x0c\x04\x0b\x19\n\x0c\n\x05\
    \x04\0\x02\x03\x05\x12\x03\x0c\x04\n\n\x0c\n\x05\x04\0\x02\x03\x01\x12\
    \x03\x0c\x0c\x11\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x0c\x17\x18\n\x0b\
    \n\x04\x04\0\x02\x04\x12\x03\r\x04\x19\n\r\n\x05\x04\0\x02\x04\x04\x12\
    \x04\r\x04\x0c\x19\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\r\x04\n\n\x0c\n\
    \x05\x04\0\x02\x04\x01\x12\x03\r\x0c\x14\n\x0c\n\x05\x04\0\x02\x04\x03\
    \x12\x03\r\x17\x18\n\x0b\n\x04\x04\0\x02\x05\x12\x03\x0e\x04\x19\n\r\n\
    \x05\x04\0\x02\x05\x04\x12\x04\x0e\x04\r\x19\n\x0c\n\x05\x04\0\x02\x05\
    \x05\x12\x03\x0e\x04\n\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\x0e\x0c\x11\
    \n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\x0e\x17\x18\n!\n\x04\x04\0\x02\
    \x06\x12\x03\x10\x04!\"\x14\x20Dependent\x20packages\n\n\x0c\n\x05\x04\0\
    \x02\x06\x04\x12\x03\x10\x04\x0c\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03\
    \x10\r\x13\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03\x10\x14\x18\n\x0c\n\x05\
    \x04\0\x02\x06\x03\x12\x03\x10\x1e\x20\n\x0b\n\x04\x04\0\x02\x07\x12\x03\
    \x11\x04!\n\x0c\n\x05\x04\0\x02\x07\x04\x12\x03\x11\x04\x0c\n\x0c\n\x05\
    \x04\0\x02\x07\x06\x12\x03\x11\r\x13\n\x0c\n\x05\x04\0\x02\x07\x01\x12\
    \x03\x11\x14\x1b\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03\x11\x1e\x20\n\x1a\
    \n\x02\x04\x01\x12\x04\x15\0!\x01\x1a\x0e\x20Holon\x20Module\n\n\n\n\x03\
    \x04\x01\x01\x12\x03\x15\x08\x0e\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x16\
    \x04\x14\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x16\x04\x15\x10\n\x0c\n\x05\
    \x04\x01\x02\0\x05\x12\x03\x16\x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\
    \x03\x16\x0b\x0f\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x16\x12\x13\n\x0b\
    \n\x04\x04\x01\x02\x01\x12\x03\x18\x04\x1d\n\r\n\x05\x04\x01\x02\x01\x04\
    \x12\x04\x18\x04\x16\x14\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x18\x04\
    \x0b\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x18\x0c\x17\n\x0c\n\x05\x04\
    \x01\x02\x01\x03\x12\x03\x18\x1b\x1c\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\
    \x19\x04\x1d\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04\x19\x04\x18\x1d\n\x0c\
    \n\x05\x04\x01\x02\x02\x05\x12\x03\x19\x04\x0b\n\x0c\n\x05\x04\x01\x02\
    \x02\x01\x12\x03\x19\x0c\x18\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x19\
    \x1b\x1c\n\x0b\n\x04\x04\x01\x02\x03\x12\x03\x1b\x04&\n\r\n\x05\x04\x01\
    \x02\x03\x04\x12\x04\x1b\x04\x19\x1d\n\x0c\n\x05\x04\x01\x02\x03\x06\x12\
    \x03\x1b\x04\x18\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\x1b\x19\x20\n\
    \x0c\n\x05\x04\x01\x02\x03\x03\x12\x03\x1b$%\n\x0b\n\x04\x04\x01\x02\x04\
    \x12\x03\x1c\x04&\n\x0c\n\x05\x04\x01\x02\x04\x04\x12\x03\x1c\x04\x0c\n\
    \x0c\n\x05\x04\x01\x02\x04\x05\x12\x03\x1c\r\x14\n\x0c\n\x05\x04\x01\x02\
    \x04\x01\x12\x03\x1c\x15\x20\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03\x1c\
    $%\n\x0b\n\x04\x04\x01\x02\x05\x12\x03\x1d\x04'\n\x0c\n\x05\x04\x01\x02\
    \x05\x04\x12\x03\x1d\x04\x0c\n\x0c\n\x05\x04\x01\x02\x05\x05\x12\x03\x1d\
    \r\x14\n\x0c\n\x05\x04\x01\x02\x05\x01\x12\x03\x1d\x15!\n\x0c\n\x05\x04\
    \x01\x02\x05\x03\x12\x03\x1d$&\n\x0b\n\x04\x04\x01\x02\x06\x12\x03\x1f\
    \x04\x1c\n\x0c\n\x05\x04\x01\x02\x06\x04\x12\x03\x1f\x04\x0c\n\x0c\n\x05\
    \x04\x01\x02\x06\x06\x12\x03\x1f\r\x11\n\x0c\n\x05\x04\x01\x02\x06\x01\
    \x12\x03\x1f\x12\x16\n\x0c\n\x05\x04\x01\x02\x06\x03\x12\x03\x1f\x19\x1b\
    \n\x0b\n\x04\x04\x01\x02\x07\x12\x03\x20\x04\x1c\n\x0c\n\x05\x04\x01\x02\
    \x07\x04\x12\x03\x20\x04\x0c\n\x0c\n\x05\x04\x01\x02\x07\x06\x12\x03\x20\
    \r\x11\n\x0c\n\x05\x04\x01\x02\x07\x01\x12\x03\x20\x12\x16\n\x0c\n\x05\
    \x04\x01\x02\x07\x03\x12\x03\x20\x19\x1b\n\n\n\x02\x04\x02\x12\x04#\0(\
    \x01\n\n\n\x03\x04\x02\x01\x12\x03#\x08\x0e\n\x0b\n\x04\x04\x02\x02\0\
    \x12\x03$\x04\x18\n\r\n\x05\x04\x02\x02\0\x04\x12\x04$\x04#\x10\n\x0c\n\
    \x05\x04\x02\x02\0\x05\x12\x03$\x04\n\n\x0c\n\x05\x04\x02\x02\0\x01\x12\
    \x03$\x0c\x13\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03$\x16\x17\n\x0b\n\x04\
    \x04\x02\x02\x01\x12\x03%\x04\x18\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04%\
    \x04$\x18\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03%\x04\n\n\x0c\n\x05\x04\
    \x02\x02\x01\x01\x12\x03%\x0c\x12\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03%\x16\x17\n\x0b\n\x04\x04\x02\x02\x02\x12\x03&\x04\x18\n\r\n\x05\x04\
    \x02\x02\x02\x04\x12\x04&\x04%\x18\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\
    \x03&\x04\x0b\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03&\x0c\x11\n\x0c\n\
    \x05\x04\x02\x02\x02\x03\x12\x03&\x16\x17\n\x0b\n\x04\x04\x02\x02\x03\
    \x12\x03'\x04\x18\n\r\n\x05\x04\x02\x02\x03\x04\x12\x04'\x04&\x18\n\x0c\
    \n\x05\x04\x02\x02\x03\x05\x12\x03'\x04\n\n\x0c\n\x05\x04\x02\x02\x03\
    \x01\x12\x03'\x0c\x10\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\x03'\x16\x17b\
    \x06proto3\
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
