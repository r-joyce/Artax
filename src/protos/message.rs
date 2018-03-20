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

#[derive(PartialEq,Clone,Default)]
pub struct ReductionMessage {
    // message fields
    pub sum: u32,
    pub avg: u32,
    pub min: ::protobuf::SingularPtrField<ReductionMessage_Min>,
    pub max: ::protobuf::SingularPtrField<ReductionMessage_Max>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReductionMessage {}

impl ReductionMessage {
    pub fn new() -> ReductionMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReductionMessage {
        static mut instance: ::protobuf::lazy::Lazy<ReductionMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReductionMessage,
        };
        unsafe {
            instance.get(ReductionMessage::new)
        }
    }

    // uint32 sum = 1;

    pub fn clear_sum(&mut self) {
        self.sum = 0;
    }

    // Param is passed by value, moved
    pub fn set_sum(&mut self, v: u32) {
        self.sum = v;
    }

    pub fn get_sum(&self) -> u32 {
        self.sum
    }

    fn get_sum_for_reflect(&self) -> &u32 {
        &self.sum
    }

    fn mut_sum_for_reflect(&mut self) -> &mut u32 {
        &mut self.sum
    }

    // uint32 avg = 2;

    pub fn clear_avg(&mut self) {
        self.avg = 0;
    }

    // Param is passed by value, moved
    pub fn set_avg(&mut self, v: u32) {
        self.avg = v;
    }

    pub fn get_avg(&self) -> u32 {
        self.avg
    }

    fn get_avg_for_reflect(&self) -> &u32 {
        &self.avg
    }

    fn mut_avg_for_reflect(&mut self) -> &mut u32 {
        &mut self.avg
    }

    // .ReductionMessage.Min min = 3;

    pub fn clear_min(&mut self) {
        self.min.clear();
    }

    pub fn has_min(&self) -> bool {
        self.min.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min(&mut self, v: ReductionMessage_Min) {
        self.min = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_min(&mut self) -> &mut ReductionMessage_Min {
        if self.min.is_none() {
            self.min.set_default();
        }
        self.min.as_mut().unwrap()
    }

    // Take field
    pub fn take_min(&mut self) -> ReductionMessage_Min {
        self.min.take().unwrap_or_else(|| ReductionMessage_Min::new())
    }

    pub fn get_min(&self) -> &ReductionMessage_Min {
        self.min.as_ref().unwrap_or_else(|| ReductionMessage_Min::default_instance())
    }

    fn get_min_for_reflect(&self) -> &::protobuf::SingularPtrField<ReductionMessage_Min> {
        &self.min
    }

    fn mut_min_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ReductionMessage_Min> {
        &mut self.min
    }

    // .ReductionMessage.Max max = 4;

    pub fn clear_max(&mut self) {
        self.max.clear();
    }

    pub fn has_max(&self) -> bool {
        self.max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max(&mut self, v: ReductionMessage_Max) {
        self.max = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max(&mut self) -> &mut ReductionMessage_Max {
        if self.max.is_none() {
            self.max.set_default();
        }
        self.max.as_mut().unwrap()
    }

    // Take field
    pub fn take_max(&mut self) -> ReductionMessage_Max {
        self.max.take().unwrap_or_else(|| ReductionMessage_Max::new())
    }

    pub fn get_max(&self) -> &ReductionMessage_Max {
        self.max.as_ref().unwrap_or_else(|| ReductionMessage_Max::default_instance())
    }

    fn get_max_for_reflect(&self) -> &::protobuf::SingularPtrField<ReductionMessage_Max> {
        &self.max
    }

    fn mut_max_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ReductionMessage_Max> {
        &mut self.max
    }
}

impl ::protobuf::Message for ReductionMessage {
    fn is_initialized(&self) -> bool {
        for v in &self.min {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.max {
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
                    self.sum = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.avg = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.min)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.max)?;
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
        if self.sum != 0 {
            my_size += ::protobuf::rt::value_size(1, self.sum, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.avg != 0 {
            my_size += ::protobuf::rt::value_size(2, self.avg, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.min.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.max.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.sum != 0 {
            os.write_uint32(1, self.sum)?;
        }
        if self.avg != 0 {
            os.write_uint32(2, self.avg)?;
        }
        if let Some(ref v) = self.min.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.max.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ReductionMessage {
    fn new() -> ReductionMessage {
        ReductionMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReductionMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sum",
                    ReductionMessage::get_sum_for_reflect,
                    ReductionMessage::mut_sum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "avg",
                    ReductionMessage::get_avg_for_reflect,
                    ReductionMessage::mut_avg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ReductionMessage_Min>>(
                    "min",
                    ReductionMessage::get_min_for_reflect,
                    ReductionMessage::mut_min_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ReductionMessage_Max>>(
                    "max",
                    ReductionMessage::get_max_for_reflect,
                    ReductionMessage::mut_max_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReductionMessage>(
                    "ReductionMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReductionMessage {
    fn clear(&mut self) {
        self.clear_sum();
        self.clear_avg();
        self.clear_min();
        self.clear_max();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReductionMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReductionMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReductionMessage_Min {
    // message fields
    pub min_x: i32,
    pub min_y: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReductionMessage_Min {}

impl ReductionMessage_Min {
    pub fn new() -> ReductionMessage_Min {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReductionMessage_Min {
        static mut instance: ::protobuf::lazy::Lazy<ReductionMessage_Min> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReductionMessage_Min,
        };
        unsafe {
            instance.get(ReductionMessage_Min::new)
        }
    }

    // int32 min_x = 1;

    pub fn clear_min_x(&mut self) {
        self.min_x = 0;
    }

    // Param is passed by value, moved
    pub fn set_min_x(&mut self, v: i32) {
        self.min_x = v;
    }

    pub fn get_min_x(&self) -> i32 {
        self.min_x
    }

    fn get_min_x_for_reflect(&self) -> &i32 {
        &self.min_x
    }

    fn mut_min_x_for_reflect(&mut self) -> &mut i32 {
        &mut self.min_x
    }

    // uint32 min_y = 2;

    pub fn clear_min_y(&mut self) {
        self.min_y = 0;
    }

    // Param is passed by value, moved
    pub fn set_min_y(&mut self, v: u32) {
        self.min_y = v;
    }

    pub fn get_min_y(&self) -> u32 {
        self.min_y
    }

    fn get_min_y_for_reflect(&self) -> &u32 {
        &self.min_y
    }

    fn mut_min_y_for_reflect(&mut self) -> &mut u32 {
        &mut self.min_y
    }
}

impl ::protobuf::Message for ReductionMessage_Min {
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
                    self.min_x = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.min_y = tmp;
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
        if self.min_x != 0 {
            my_size += ::protobuf::rt::value_size(1, self.min_x, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.min_y != 0 {
            my_size += ::protobuf::rt::value_size(2, self.min_y, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.min_x != 0 {
            os.write_int32(1, self.min_x)?;
        }
        if self.min_y != 0 {
            os.write_uint32(2, self.min_y)?;
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

impl ::protobuf::MessageStatic for ReductionMessage_Min {
    fn new() -> ReductionMessage_Min {
        ReductionMessage_Min::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReductionMessage_Min>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "min_x",
                    ReductionMessage_Min::get_min_x_for_reflect,
                    ReductionMessage_Min::mut_min_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "min_y",
                    ReductionMessage_Min::get_min_y_for_reflect,
                    ReductionMessage_Min::mut_min_y_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReductionMessage_Min>(
                    "ReductionMessage_Min",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReductionMessage_Min {
    fn clear(&mut self) {
        self.clear_min_x();
        self.clear_min_y();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReductionMessage_Min {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReductionMessage_Min {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReductionMessage_Max {
    // message fields
    pub max_x: i32,
    pub max_y: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReductionMessage_Max {}

impl ReductionMessage_Max {
    pub fn new() -> ReductionMessage_Max {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReductionMessage_Max {
        static mut instance: ::protobuf::lazy::Lazy<ReductionMessage_Max> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReductionMessage_Max,
        };
        unsafe {
            instance.get(ReductionMessage_Max::new)
        }
    }

    // int32 max_x = 1;

    pub fn clear_max_x(&mut self) {
        self.max_x = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_x(&mut self, v: i32) {
        self.max_x = v;
    }

    pub fn get_max_x(&self) -> i32 {
        self.max_x
    }

    fn get_max_x_for_reflect(&self) -> &i32 {
        &self.max_x
    }

    fn mut_max_x_for_reflect(&mut self) -> &mut i32 {
        &mut self.max_x
    }

    // uint32 max_y = 2;

    pub fn clear_max_y(&mut self) {
        self.max_y = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_y(&mut self, v: u32) {
        self.max_y = v;
    }

    pub fn get_max_y(&self) -> u32 {
        self.max_y
    }

    fn get_max_y_for_reflect(&self) -> &u32 {
        &self.max_y
    }

    fn mut_max_y_for_reflect(&mut self) -> &mut u32 {
        &mut self.max_y
    }
}

impl ::protobuf::Message for ReductionMessage_Max {
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
                    self.max_x = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.max_y = tmp;
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
        if self.max_x != 0 {
            my_size += ::protobuf::rt::value_size(1, self.max_x, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.max_y != 0 {
            my_size += ::protobuf::rt::value_size(2, self.max_y, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.max_x != 0 {
            os.write_int32(1, self.max_x)?;
        }
        if self.max_y != 0 {
            os.write_uint32(2, self.max_y)?;
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

impl ::protobuf::MessageStatic for ReductionMessage_Max {
    fn new() -> ReductionMessage_Max {
        ReductionMessage_Max::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReductionMessage_Max>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_x",
                    ReductionMessage_Max::get_max_x_for_reflect,
                    ReductionMessage_Max::mut_max_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "max_y",
                    ReductionMessage_Max::get_max_y_for_reflect,
                    ReductionMessage_Max::mut_max_y_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReductionMessage_Max>(
                    "ReductionMessage_Max",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReductionMessage_Max {
    fn clear(&mut self) {
        self.clear_max_x();
        self.clear_max_y();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReductionMessage_Max {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReductionMessage_Max {
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
    s\x18\x03\x20\x01(\x05R\x07samples\"\xea\x01\n\x10ReductionMessage\x12\
    \x10\n\x03sum\x18\x01\x20\x01(\rR\x03sum\x12\x10\n\x03avg\x18\x02\x20\
    \x01(\rR\x03avg\x12'\n\x03min\x18\x03\x20\x01(\x0b2\x15.ReductionMessage\
    .MinR\x03min\x12'\n\x03max\x18\x04\x20\x01(\x0b2\x15.ReductionMessage.Ma\
    xR\x03max\x1a/\n\x03Min\x12\x13\n\x05min_x\x18\x01\x20\x01(\x05R\x04minX\
    \x12\x13\n\x05min_y\x18\x02\x20\x01(\rR\x04minY\x1a/\n\x03Max\x12\x13\n\
    \x05max_x\x18\x01\x20\x01(\x05R\x04maxX\x12\x13\n\x05max_y\x18\x02\x20\
    \x01(\rR\x04maxYJ\xe9\t\n\x06\x12\x04\0\0\x20\x01\n\x08\n\x01\x0c\x12\
    \x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x06\x01\n\n\n\x03\x04\0\x01\
    \x12\x03\x02\x08\x0f\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x04\x1a\n\x0c\n\
    \x05\x04\0\x02\0\x04\x12\x03\x03\x04\x0c\n\x0c\n\x05\x04\0\x02\0\x05\x12\
    \x03\x03\r\x12\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\x13\x15\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03\x03\x18\x19\n\x0b\n\x04\x04\0\x02\x01\x12\
    \x03\x04\x04\x1c\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x04\x04\x0c\n\x0c\
    \n\x05\x04\0\x02\x01\x05\x12\x03\x04\r\x13\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x03\x04\x14\x17\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04\x1a\x1b\n\
    \x0b\n\x04\x04\0\x02\x02\x12\x03\x05\x04$\n\x0c\n\x05\x04\0\x02\x02\x04\
    \x12\x03\x05\x04\x0c\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x05\r\x13\n\
    \x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x05\x14\x1f\n\x0c\n\x05\x04\0\x02\
    \x02\x03\x12\x03\x05\"#\n\n\n\x02\x04\x01\x12\x04\x08\0\n\x01\n\n\n\x03\
    \x04\x01\x01\x12\x03\x08\x08\x17\n\x0b\n\x04\x04\x01\x02\0\x12\x03\t\x04\
    \"\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\t\x04\x08\x19\n\x0c\n\x05\x04\x01\
    \x02\0\x05\x12\x03\t\x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\t\x0b\
    \x1d\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\t\x20!\n\n\n\x02\x04\x02\x12\
    \x04\x0c\0\x11\x01\n\n\n\x03\x04\x02\x01\x12\x03\x0c\x08\x14\n\x0b\n\x04\
    \x04\x02\x02\0\x12\x03\r\x04\x1a\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\r\
    \x04\x0c\x16\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\r\x04\n\n\x0c\n\x05\
    \x04\x02\x02\0\x01\x12\x03\r\x0b\x15\n\x0c\n\x05\x04\x02\x02\0\x03\x12\
    \x03\r\x18\x19\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x0e\x04\x18\n\r\n\x05\
    \x04\x02\x02\x01\x04\x12\x04\x0e\x04\r\x1a\n\x0c\n\x05\x04\x02\x02\x01\
    \x05\x12\x03\x0e\x04\n\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x0e\x0b\
    \x13\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x0e\x16\x17\n\x0b\n\x04\x04\
    \x02\x02\x02\x12\x03\x0f\x04\x16\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04\
    \x0f\x04\x0e\x18\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03\x0f\x04\t\n\x0c\
    \n\x05\x04\x02\x02\x02\x01\x12\x03\x0f\n\x11\n\x0c\n\x05\x04\x02\x02\x02\
    \x03\x12\x03\x0f\x14\x15\n\n\n\x02\x04\x03\x12\x04\x13\0\x20\x01\n\n\n\
    \x03\x04\x03\x01\x12\x03\x13\x08\x18\n\x0b\n\x04\x04\x03\x02\0\x12\x03\
    \x14\x04\x13\n\r\n\x05\x04\x03\x02\0\x04\x12\x04\x14\x04\x13\x1a\n\x0c\n\
    \x05\x04\x03\x02\0\x05\x12\x03\x14\x04\n\n\x0c\n\x05\x04\x03\x02\0\x01\
    \x12\x03\x14\x0b\x0e\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x14\x11\x12\n\
    \x0b\n\x04\x04\x03\x02\x01\x12\x03\x15\x04\x13\n\r\n\x05\x04\x03\x02\x01\
    \x04\x12\x04\x15\x04\x14\x13\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x03\x15\
    \x04\n\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03\x15\x0b\x0e\n\x0c\n\x05\
    \x04\x03\x02\x01\x03\x12\x03\x15\x11\x12\n\x0c\n\x04\x04\x03\x03\0\x12\
    \x04\x16\x04\x19\x05\n\x0c\n\x05\x04\x03\x03\0\x01\x12\x03\x16\x0c\x0f\n\
    \r\n\x06\x04\x03\x03\0\x02\0\x12\x03\x17\x08\x18\n\x0f\n\x07\x04\x03\x03\
    \0\x02\0\x04\x12\x04\x17\x08\x16\x11\n\x0e\n\x07\x04\x03\x03\0\x02\0\x05\
    \x12\x03\x17\x08\r\n\x0e\n\x07\x04\x03\x03\0\x02\0\x01\x12\x03\x17\x0e\
    \x13\n\x0e\n\x07\x04\x03\x03\0\x02\0\x03\x12\x03\x17\x16\x17\n\r\n\x06\
    \x04\x03\x03\0\x02\x01\x12\x03\x18\x08\x19\n\x0f\n\x07\x04\x03\x03\0\x02\
    \x01\x04\x12\x04\x18\x08\x17\x18\n\x0e\n\x07\x04\x03\x03\0\x02\x01\x05\
    \x12\x03\x18\x08\x0e\n\x0e\n\x07\x04\x03\x03\0\x02\x01\x01\x12\x03\x18\
    \x0f\x14\n\x0e\n\x07\x04\x03\x03\0\x02\x01\x03\x12\x03\x18\x17\x18\n\x0b\
    \n\x04\x04\x03\x02\x02\x12\x03\x1a\x04\x10\n\r\n\x05\x04\x03\x02\x02\x04\
    \x12\x04\x1a\x04\x19\x05\n\x0c\n\x05\x04\x03\x02\x02\x06\x12\x03\x1a\x04\
    \x07\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x03\x1a\x08\x0b\n\x0c\n\x05\x04\
    \x03\x02\x02\x03\x12\x03\x1a\x0e\x0f\n\x0c\n\x04\x04\x03\x03\x01\x12\x04\
    \x1b\x04\x1e\x05\n\x0c\n\x05\x04\x03\x03\x01\x01\x12\x03\x1b\x0c\x0f\n\r\
    \n\x06\x04\x03\x03\x01\x02\0\x12\x03\x1c\x08\x18\n\x0f\n\x07\x04\x03\x03\
    \x01\x02\0\x04\x12\x04\x1c\x08\x1b\x11\n\x0e\n\x07\x04\x03\x03\x01\x02\0\
    \x05\x12\x03\x1c\x08\r\n\x0e\n\x07\x04\x03\x03\x01\x02\0\x01\x12\x03\x1c\
    \x0e\x13\n\x0e\n\x07\x04\x03\x03\x01\x02\0\x03\x12\x03\x1c\x16\x17\n\r\n\
    \x06\x04\x03\x03\x01\x02\x01\x12\x03\x1d\x08\x19\n\x0f\n\x07\x04\x03\x03\
    \x01\x02\x01\x04\x12\x04\x1d\x08\x1c\x18\n\x0e\n\x07\x04\x03\x03\x01\x02\
    \x01\x05\x12\x03\x1d\x08\x0e\n\x0e\n\x07\x04\x03\x03\x01\x02\x01\x01\x12\
    \x03\x1d\x0f\x14\n\x0e\n\x07\x04\x03\x03\x01\x02\x01\x03\x12\x03\x1d\x17\
    \x18\n\x0b\n\x04\x04\x03\x02\x03\x12\x03\x1f\x04\x10\n\r\n\x05\x04\x03\
    \x02\x03\x04\x12\x04\x1f\x04\x1e\x05\n\x0c\n\x05\x04\x03\x02\x03\x06\x12\
    \x03\x1f\x04\x07\n\x0c\n\x05\x04\x03\x02\x03\x01\x12\x03\x1f\x08\x0b\n\
    \x0c\n\x05\x04\x03\x02\x03\x03\x12\x03\x1f\x0e\x0fb\x06proto3\
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
