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
pub struct Message {
    // message fields
    pub mz: ::std::vec::Vec<i32>,
    pub tic: ::std::vec::Vec<u32>,
    pub time_stamps: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Message {}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Message {
        static mut instance: ::protobuf::lazy::Lazy<Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message,
        };
        unsafe {
            instance.get(Message::new)
        }
    }

    // repeated int32 mz = 1;

    pub fn clear_mz(&mut self) {
        self.mz.clear();
    }

    // Param is passed by value, moved
    pub fn set_mz(&mut self, v: ::std::vec::Vec<i32>) {
        self.mz = v;
    }

    // Mutable pointer to the field.
    pub fn mut_mz(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.mz
    }

    // Take field
    pub fn take_mz(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.mz, ::std::vec::Vec::new())
    }

    pub fn get_mz(&self) -> &[i32] {
        &self.mz
    }

    fn get_mz_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.mz
    }

    fn mut_mz_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.mz
    }

    // repeated uint32 tic = 2;

    pub fn clear_tic(&mut self) {
        self.tic.clear();
    }

    // Param is passed by value, moved
    pub fn set_tic(&mut self, v: ::std::vec::Vec<u32>) {
        self.tic = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tic(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.tic
    }

    // Take field
    pub fn take_tic(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.tic, ::std::vec::Vec::new())
    }

    pub fn get_tic(&self) -> &[u32] {
        &self.tic
    }

    fn get_tic_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.tic
    }

    fn mut_tic_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.tic
    }

    // repeated uint64 time_stamps = 3;

    pub fn clear_time_stamps(&mut self) {
        self.time_stamps.clear();
    }

    // Param is passed by value, moved
    pub fn set_time_stamps(&mut self, v: ::std::vec::Vec<u64>) {
        self.time_stamps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_time_stamps(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.time_stamps
    }

    // Take field
    pub fn take_time_stamps(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.time_stamps, ::std::vec::Vec::new())
    }

    pub fn get_time_stamps(&self) -> &[u64] {
        &self.time_stamps
    }

    fn get_time_stamps_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.time_stamps
    }

    fn mut_time_stamps_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.time_stamps
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.mz)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.tic)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.time_stamps)?;
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
        for value in &self.mz {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.tic {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.time_stamps {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.mz {
            os.write_int32(1, *v)?;
        };
        for v in &self.tic {
            os.write_uint32(2, *v)?;
        };
        for v in &self.time_stamps {
            os.write_uint64(3, *v)?;
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

impl ::protobuf::MessageStatic for Message {
    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static(_: ::std::option::Option<Message>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "mz",
                    Message::get_mz_for_reflect,
                    Message::mut_mz_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tic",
                    Message::get_tic_for_reflect,
                    Message::mut_tic_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "time_stamps",
                    Message::get_time_stamps_for_reflect,
                    Message::mut_time_stamps_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message>(
                    "Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.clear_mz();
        self.clear_tic();
        self.clear_time_stamps();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TofWidthMessage {
    // message fields
    pub pusher_pulse_width: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TofWidthMessage {}

impl TofWidthMessage {
    pub fn new() -> TofWidthMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TofWidthMessage {
        static mut instance: ::protobuf::lazy::Lazy<TofWidthMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TofWidthMessage,
        };
        unsafe {
            instance.get(TofWidthMessage::new)
        }
    }

    // uint64 pusher_pulse_width = 1;

    pub fn clear_pusher_pulse_width(&mut self) {
        self.pusher_pulse_width = 0;
    }

    // Param is passed by value, moved
    pub fn set_pusher_pulse_width(&mut self, v: u64) {
        self.pusher_pulse_width = v;
    }

    pub fn get_pusher_pulse_width(&self) -> u64 {
        self.pusher_pulse_width
    }

    fn get_pusher_pulse_width_for_reflect(&self) -> &u64 {
        &self.pusher_pulse_width
    }

    fn mut_pusher_pulse_width_for_reflect(&mut self) -> &mut u64 {
        &mut self.pusher_pulse_width
    }
}

impl ::protobuf::Message for TofWidthMessage {
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
                    let tmp = is.read_uint64()?;
                    self.pusher_pulse_width = tmp;
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
        if self.pusher_pulse_width != 0 {
            my_size += ::protobuf::rt::value_size(1, self.pusher_pulse_width, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.pusher_pulse_width != 0 {
            os.write_uint64(1, self.pusher_pulse_width)?;
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

impl ::protobuf::MessageStatic for TofWidthMessage {
    fn new() -> TofWidthMessage {
        TofWidthMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<TofWidthMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "pusher_pulse_width",
                    TofWidthMessage::get_pusher_pulse_width_for_reflect,
                    TofWidthMessage::mut_pusher_pulse_width_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TofWidthMessage>(
                    "TofWidthMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TofWidthMessage {
    fn clear(&mut self) {
        self.clear_pusher_pulse_width();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TofWidthMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TofWidthMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetupMessage {
    // message fields
    pub horizontal: f64,
    pub vertical: f64,
    pub samples: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetupMessage {}

impl SetupMessage {
    pub fn new() -> SetupMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetupMessage {
        static mut instance: ::protobuf::lazy::Lazy<SetupMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetupMessage,
        };
        unsafe {
            instance.get(SetupMessage::new)
        }
    }

    // double horizontal = 1;

    pub fn clear_horizontal(&mut self) {
        self.horizontal = 0.;
    }

    // Param is passed by value, moved
    pub fn set_horizontal(&mut self, v: f64) {
        self.horizontal = v;
    }

    pub fn get_horizontal(&self) -> f64 {
        self.horizontal
    }

    fn get_horizontal_for_reflect(&self) -> &f64 {
        &self.horizontal
    }

    fn mut_horizontal_for_reflect(&mut self) -> &mut f64 {
        &mut self.horizontal
    }

    // double vertical = 2;

    pub fn clear_vertical(&mut self) {
        self.vertical = 0.;
    }

    // Param is passed by value, moved
    pub fn set_vertical(&mut self, v: f64) {
        self.vertical = v;
    }

    pub fn get_vertical(&self) -> f64 {
        self.vertical
    }

    fn get_vertical_for_reflect(&self) -> &f64 {
        &self.vertical
    }

    fn mut_vertical_for_reflect(&mut self) -> &mut f64 {
        &mut self.vertical
    }

    // int32 samples = 3;

    pub fn clear_samples(&mut self) {
        self.samples = 0;
    }

    // Param is passed by value, moved
    pub fn set_samples(&mut self, v: i32) {
        self.samples = v;
    }

    pub fn get_samples(&self) -> i32 {
        self.samples
    }

    fn get_samples_for_reflect(&self) -> &i32 {
        &self.samples
    }

    fn mut_samples_for_reflect(&mut self) -> &mut i32 {
        &mut self.samples
    }
}

impl ::protobuf::Message for SetupMessage {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_double()?;
                    self.horizontal = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.vertical = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.samples = tmp;
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
        if self.horizontal != 0. {
            my_size += 9;
        }
        if self.vertical != 0. {
            my_size += 9;
        }
        if self.samples != 0 {
            my_size += ::protobuf::rt::value_size(3, self.samples, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.horizontal != 0. {
            os.write_double(1, self.horizontal)?;
        }
        if self.vertical != 0. {
            os.write_double(2, self.vertical)?;
        }
        if self.samples != 0 {
            os.write_int32(3, self.samples)?;
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

impl ::protobuf::MessageStatic for SetupMessage {
    fn new() -> SetupMessage {
        SetupMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetupMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "horizontal",
                    SetupMessage::get_horizontal_for_reflect,
                    SetupMessage::mut_horizontal_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "vertical",
                    SetupMessage::get_vertical_for_reflect,
                    SetupMessage::mut_vertical_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "samples",
                    SetupMessage::get_samples_for_reflect,
                    SetupMessage::mut_samples_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetupMessage>(
                    "SetupMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetupMessage {
    fn clear(&mut self) {
        self.clear_horizontal();
        self.clear_vertical();
        self.clear_samples();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetupMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetupMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rmessage.proto\"L\n\x07Message\x12\x0e\n\x02mz\x18\x01\x20\x03(\x05R\
    \x02mz\x12\x10\n\x03tic\x18\x02\x20\x03(\rR\x03tic\x12\x1f\n\x0btime_sta\
    mps\x18\x03\x20\x03(\x04R\ntimeStamps\"?\n\x0fTofWidthMessage\x12,\n\x12\
    pusher_pulse_width\x18\x01\x20\x01(\x04R\x10pusherPulseWidth\"d\n\x0cSet\
    upMessage\x12\x1e\n\nhorizontal\x18\x01\x20\x01(\x01R\nhorizontal\x12\
    \x1a\n\x08vertical\x18\x02\x20\x01(\x01R\x08vertical\x12\x18\n\x07sample\
    s\x18\x03\x20\x01(\x05R\x07samplesb\x06proto3\
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
