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
pub struct AppFile {
    // message fields
    pub magic_number: u32,
    pub app: ::protobuf::SingularPtrField<App>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AppFile {}

impl AppFile {
    pub fn new() -> AppFile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AppFile {
        static mut instance: ::protobuf::lazy::Lazy<AppFile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AppFile,
        };
        unsafe {
            instance.get(AppFile::new)
        }
    }

    // fixed32 magic_number = 1;

    pub fn clear_magic_number(&mut self) {
        self.magic_number = 0;
    }

    // Param is passed by value, moved
    pub fn set_magic_number(&mut self, v: u32) {
        self.magic_number = v;
    }

    pub fn get_magic_number(&self) -> u32 {
        self.magic_number
    }

    fn get_magic_number_for_reflect(&self) -> &u32 {
        &self.magic_number
    }

    fn mut_magic_number_for_reflect(&mut self) -> &mut u32 {
        &mut self.magic_number
    }

    // .App app = 2;

    pub fn clear_app(&mut self) {
        self.app.clear();
    }

    pub fn has_app(&self) -> bool {
        self.app.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app(&mut self, v: App) {
        self.app = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_app(&mut self) -> &mut App {
        if self.app.is_none() {
            self.app.set_default();
        }
        self.app.as_mut().unwrap()
    }

    // Take field
    pub fn take_app(&mut self) -> App {
        self.app.take().unwrap_or_else(|| App::new())
    }

    pub fn get_app(&self) -> &App {
        self.app.as_ref().unwrap_or_else(|| App::default_instance())
    }

    fn get_app_for_reflect(&self) -> &::protobuf::SingularPtrField<App> {
        &self.app
    }

    fn mut_app_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<App> {
        &mut self.app
    }
}

impl ::protobuf::Message for AppFile {
    fn is_initialized(&self) -> bool {
        for v in &self.app {
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
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.magic_number = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.app)?;
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
        if self.magic_number != 0 {
            my_size += 5;
        }
        if let Some(ref v) = self.app.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.magic_number != 0 {
            os.write_fixed32(1, self.magic_number)?;
        }
        if let Some(ref v) = self.app.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for AppFile {
    fn new() -> AppFile {
        AppFile::new()
    }

    fn descriptor_static(_: ::std::option::Option<AppFile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "magic_number",
                    AppFile::get_magic_number_for_reflect,
                    AppFile::mut_magic_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<App>>(
                    "app",
                    AppFile::get_app_for_reflect,
                    AppFile::mut_app_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AppFile>(
                    "AppFile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AppFile {
    fn clear(&mut self) {
        self.clear_magic_number();
        self.clear_app();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AppFile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AppFile {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct App {
    // message fields
    pub id: u64,
    pub version: u64,
    pub name: ::std::string::String,
    pub cafs: ::protobuf::RepeatedField<super::expr::Expr>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for App {}

impl App {
    pub fn new() -> App {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static App {
        static mut instance: ::protobuf::lazy::Lazy<App> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const App,
        };
        unsafe {
            instance.get(App::new)
        }
    }

    // fixed64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u64 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.id
    }

    // fixed64 version = 2;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }

    pub fn get_version(&self) -> u64 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &u64 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut u64 {
        &mut self.version
    }

    // string name = 3;

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

    // repeated .Expr cafs = 10;

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
}

impl ::protobuf::Message for App {
    fn is_initialized(&self) -> bool {
        for v in &self.cafs {
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
                    self.id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.version = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                10 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cafs)?;
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
        if self.id != 0 {
            my_size += 9;
        }
        if self.version != 0 {
            my_size += 9;
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.name);
        }
        for value in &self.cafs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_fixed64(1, self.id)?;
        }
        if self.version != 0 {
            os.write_fixed64(2, self.version)?;
        }
        if !self.name.is_empty() {
            os.write_string(3, &self.name)?;
        }
        for v in &self.cafs {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for App {
    fn new() -> App {
        App::new()
    }

    fn descriptor_static(_: ::std::option::Option<App>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "id",
                    App::get_id_for_reflect,
                    App::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "version",
                    App::get_version_for_reflect,
                    App::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    App::get_name_for_reflect,
                    App::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::expr::Expr>>(
                    "cafs",
                    App::get_cafs_for_reflect,
                    App::mut_cafs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<App>(
                    "App",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for App {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_version();
        self.clear_name();
        self.clear_cafs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for App {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for App {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\tapp.proto\x1a\nexpr.proto\"D\n\x07AppFile\x12!\n\x0cmagic_number\x18\
    \x01\x20\x01(\x07R\x0bmagicNumber\x12\x16\n\x03app\x18\x02\x20\x01(\x0b2\
    \x04.AppR\x03app\"^\n\x03App\x12\x0e\n\x02id\x18\x01\x20\x01(\x06R\x02id\
    \x12\x18\n\x07version\x18\x02\x20\x01(\x06R\x07version\x12\x12\n\x04name\
    \x18\x03\x20\x01(\tR\x04name\x12\x19\n\x04cafs\x18\n\x20\x03(\x0b2\x05.E\
    xprR\x04cafsJ\xde\x04\n\x06\x12\x04\0\0\x13\x01\n\x08\n\x01\x0c\x12\x03\
    \0\0\x12\n!\n\x02\x03\0\x12\x03\x02\x07\x13\"\x16\x20import\x20\"code.pr\
    oto\";\n\n$\n\x02\x04\0\x12\x04\x06\0\t\x01\x1a\x18\x20Holon\x20Applicat\
    ion\x20File\n\n\n\n\x03\x04\0\x01\x12\x03\x06\x08\x0f\n\x0b\n\x04\x04\0\
    \x02\0\x12\x03\x07\x04\x1d\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x07\x04\x06\
    \x11\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x07\x04\x0b\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\x07\x0c\x18\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x07\
    \x1b\x1c\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x08\x04\x14\n\r\n\x05\x04\0\
    \x02\x01\x04\x12\x04\x08\x04\x07\x1d\n\x0c\n\x05\x04\0\x02\x01\x06\x12\
    \x03\x08\x04\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x08\x0c\x0f\n\x0c\
    \n\x05\x04\0\x02\x01\x03\x12\x03\x08\x12\x13\n\x1f\n\x02\x04\x01\x12\x04\
    \x0c\0\x13\x01\x1a\x13\x20Holon\x20Application\n\n\n\n\x03\x04\x01\x01\
    \x12\x03\x0c\x08\x0b\n\x0b\n\x04\x04\x01\x02\0\x12\x03\r\x04\x18\n\r\n\
    \x05\x04\x01\x02\0\x04\x12\x04\r\x04\x0c\r\n\x0c\n\x05\x04\x01\x02\0\x05\
    \x12\x03\r\x04\x0b\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\r\x0c\x0e\n\x0c\
    \n\x05\x04\x01\x02\0\x03\x12\x03\r\x16\x17\n\x0b\n\x04\x04\x01\x02\x01\
    \x12\x03\x0e\x04\x18\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x0e\x04\r\x18\
    \n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x0e\x04\x0b\n\x0c\n\x05\x04\x01\
    \x02\x01\x01\x12\x03\x0e\x0c\x13\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\
    \x0e\x16\x17\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x0f\x04\x18\n\r\n\x05\
    \x04\x01\x02\x02\x04\x12\x04\x0f\x04\x0e\x18\n\x0c\n\x05\x04\x01\x02\x02\
    \x05\x12\x03\x0f\x04\n\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x0f\x0c\
    \x10\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x0f\x16\x17\n2\n\x04\x04\
    \x01\x02\x03\x12\x03\x12\x04(\x1a%\x20repeated\x20InfoTable\x20info_tabl\
    es\x20=\x209;\n\n\x0c\n\x05\x04\x01\x02\x03\x04\x12\x03\x12\x04\x0c\n\
    \x0c\n\x05\x04\x01\x02\x03\x06\x12\x03\x12\r\x11\n\x0c\n\x05\x04\x01\x02\
    \x03\x01\x12\x03\x12\x17\x1b\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03\x12\
    %'b\x06proto3\
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
