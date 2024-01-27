// This file is generated by rust-protobuf 3.3.0. Do not edit
// .proto file is parsed by pure
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

//! Generated file from `data.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

// @@protoc_insertion_point(message:data.Cookie)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Cookie {
    // message fields
    // @@protoc_insertion_point(field:data.Cookie.DedeUserID)
    pub DedeUserID: ::std::string::String,
    // @@protoc_insertion_point(field:data.Cookie.DedeUserID__ckMd5)
    pub DedeUserID__ckMd5: ::std::string::String,
    // @@protoc_insertion_point(field:data.Cookie.SESSDATA)
    pub SESSDATA: ::std::string::String,
    // @@protoc_insertion_point(field:data.Cookie.bili_jct)
    pub bili_jct: ::std::string::String,
    // @@protoc_insertion_point(field:data.Cookie.buvid3)
    pub buvid3: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:data.Cookie.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Cookie {
    fn default() -> &'a Cookie {
        <Cookie as ::protobuf::Message>::default_instance()
    }
}

impl Cookie {
    pub fn new() -> Cookie {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DedeUserID",
            |m: &Cookie| { &m.DedeUserID },
            |m: &mut Cookie| { &mut m.DedeUserID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DedeUserID__ckMd5",
            |m: &Cookie| { &m.DedeUserID__ckMd5 },
            |m: &mut Cookie| { &mut m.DedeUserID__ckMd5 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "SESSDATA",
            |m: &Cookie| { &m.SESSDATA },
            |m: &mut Cookie| { &mut m.SESSDATA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "bili_jct",
            |m: &Cookie| { &m.bili_jct },
            |m: &mut Cookie| { &mut m.bili_jct },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "buvid3",
            |m: &Cookie| { &m.buvid3 },
            |m: &mut Cookie| { &mut m.buvid3 },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Cookie>(
            "Cookie",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Cookie {
    const NAME: &'static str = "Cookie";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.DedeUserID = is.read_string()?;
                },
                18 => {
                    self.DedeUserID__ckMd5 = is.read_string()?;
                },
                26 => {
                    self.SESSDATA = is.read_string()?;
                },
                34 => {
                    self.bili_jct = is.read_string()?;
                },
                42 => {
                    self.buvid3 = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.DedeUserID.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.DedeUserID);
        }
        if !self.DedeUserID__ckMd5.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.DedeUserID__ckMd5);
        }
        if !self.SESSDATA.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.SESSDATA);
        }
        if !self.bili_jct.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.bili_jct);
        }
        if !self.buvid3.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.buvid3);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.DedeUserID.is_empty() {
            os.write_string(1, &self.DedeUserID)?;
        }
        if !self.DedeUserID__ckMd5.is_empty() {
            os.write_string(2, &self.DedeUserID__ckMd5)?;
        }
        if !self.SESSDATA.is_empty() {
            os.write_string(3, &self.SESSDATA)?;
        }
        if !self.bili_jct.is_empty() {
            os.write_string(4, &self.bili_jct)?;
        }
        if !self.buvid3.is_empty() {
            os.write_string(5, &self.buvid3)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Cookie {
        Cookie::new()
    }

    fn clear(&mut self) {
        self.DedeUserID.clear();
        self.DedeUserID__ckMd5.clear();
        self.SESSDATA.clear();
        self.bili_jct.clear();
        self.buvid3.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Cookie {
        static instance: Cookie = Cookie {
            DedeUserID: ::std::string::String::new(),
            DedeUserID__ckMd5: ::std::string::String::new(),
            SESSDATA: ::std::string::String::new(),
            bili_jct: ::std::string::String::new(),
            buvid3: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Cookie {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Cookie").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Cookie {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Cookie {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:data.ListMeta)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ListMeta {
    // message fields
    // @@protoc_insertion_point(field:data.ListMeta.id)
    pub id: i64,
    // @@protoc_insertion_point(field:data.ListMeta.title)
    pub title: ::std::string::String,
    // @@protoc_insertion_point(field:data.ListMeta.media_count)
    pub media_count: i32,
    // @@protoc_insertion_point(field:data.ListMeta.track)
    pub track: bool,
    // @@protoc_insertion_point(field:data.ListMeta.expired)
    pub expired: bool,
    // @@protoc_insertion_point(field:data.ListMeta.clarity)
    pub clarity: ::std::option::Option<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:data.ListMeta.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ListMeta {
    fn default() -> &'a ListMeta {
        <ListMeta as ::protobuf::Message>::default_instance()
    }
}

impl ListMeta {
    pub fn new() -> ListMeta {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &ListMeta| { &m.id },
            |m: &mut ListMeta| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "title",
            |m: &ListMeta| { &m.title },
            |m: &mut ListMeta| { &mut m.title },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "media_count",
            |m: &ListMeta| { &m.media_count },
            |m: &mut ListMeta| { &mut m.media_count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "track",
            |m: &ListMeta| { &m.track },
            |m: &mut ListMeta| { &mut m.track },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "expired",
            |m: &ListMeta| { &m.expired },
            |m: &mut ListMeta| { &mut m.expired },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "clarity",
            |m: &ListMeta| { &m.clarity },
            |m: &mut ListMeta| { &mut m.clarity },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ListMeta>(
            "ListMeta",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ListMeta {
    const NAME: &'static str = "ListMeta";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.id = is.read_int64()?;
                },
                18 => {
                    self.title = is.read_string()?;
                },
                24 => {
                    self.media_count = is.read_int32()?;
                },
                32 => {
                    self.track = is.read_bool()?;
                },
                40 => {
                    self.expired = is.read_bool()?;
                },
                802 => {
                    self.clarity = ::std::option::Option::Some(is.read_string()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.id != 0 {
            my_size += ::protobuf::rt::int64_size(1, self.id);
        }
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.title);
        }
        if self.media_count != 0 {
            my_size += ::protobuf::rt::int32_size(3, self.media_count);
        }
        if self.track != false {
            my_size += 1 + 1;
        }
        if self.expired != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.clarity.as_ref() {
            my_size += ::protobuf::rt::string_size(100, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.id != 0 {
            os.write_int64(1, self.id)?;
        }
        if !self.title.is_empty() {
            os.write_string(2, &self.title)?;
        }
        if self.media_count != 0 {
            os.write_int32(3, self.media_count)?;
        }
        if self.track != false {
            os.write_bool(4, self.track)?;
        }
        if self.expired != false {
            os.write_bool(5, self.expired)?;
        }
        if let Some(v) = self.clarity.as_ref() {
            os.write_string(100, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ListMeta {
        ListMeta::new()
    }

    fn clear(&mut self) {
        self.id = 0;
        self.title.clear();
        self.media_count = 0;
        self.track = false;
        self.expired = false;
        self.clarity = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ListMeta {
        static instance: ListMeta = ListMeta {
            id: 0,
            title: ::std::string::String::new(),
            media_count: 0,
            track: false,
            expired: false,
            clarity: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ListMeta {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ListMeta").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ListMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListMeta {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:data.VideoMeta)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct VideoMeta {
    // message fields
    // @@protoc_insertion_point(field:data.VideoMeta.bvid)
    pub bvid: ::std::string::String,
    // @@protoc_insertion_point(field:data.VideoMeta.title)
    pub title: ::std::string::String,
    // @@protoc_insertion_point(field:data.VideoMeta.upper)
    pub upper: ::protobuf::MessageField<UserMeta>,
    // @@protoc_insertion_point(field:data.VideoMeta.type)
    pub type_: i32,
    // @@protoc_insertion_point(field:data.VideoMeta.saved)
    pub saved: bool,
    // @@protoc_insertion_point(field:data.VideoMeta.fav)
    pub fav: bool,
    // @@protoc_insertion_point(field:data.VideoMeta.expired)
    pub expired: bool,
    // @@protoc_insertion_point(field:data.VideoMeta.attr)
    pub attr: i32,
    // @@protoc_insertion_point(field:data.VideoMeta.track)
    pub track: bool,
    // @@protoc_insertion_point(field:data.VideoMeta.list_ids)
    pub list_ids: ::std::vec::Vec<i64>,
    // @@protoc_insertion_point(field:data.VideoMeta.clarity)
    pub clarity: ::std::option::Option<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:data.VideoMeta.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a VideoMeta {
    fn default() -> &'a VideoMeta {
        <VideoMeta as ::protobuf::Message>::default_instance()
    }
}

impl VideoMeta {
    pub fn new() -> VideoMeta {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(11);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "bvid",
            |m: &VideoMeta| { &m.bvid },
            |m: &mut VideoMeta| { &mut m.bvid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "title",
            |m: &VideoMeta| { &m.title },
            |m: &mut VideoMeta| { &mut m.title },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, UserMeta>(
            "upper",
            |m: &VideoMeta| { &m.upper },
            |m: &mut VideoMeta| { &mut m.upper },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &VideoMeta| { &m.type_ },
            |m: &mut VideoMeta| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "saved",
            |m: &VideoMeta| { &m.saved },
            |m: &mut VideoMeta| { &mut m.saved },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "fav",
            |m: &VideoMeta| { &m.fav },
            |m: &mut VideoMeta| { &mut m.fav },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "expired",
            |m: &VideoMeta| { &m.expired },
            |m: &mut VideoMeta| { &mut m.expired },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "attr",
            |m: &VideoMeta| { &m.attr },
            |m: &mut VideoMeta| { &mut m.attr },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "track",
            |m: &VideoMeta| { &m.track },
            |m: &mut VideoMeta| { &mut m.track },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "list_ids",
            |m: &VideoMeta| { &m.list_ids },
            |m: &mut VideoMeta| { &mut m.list_ids },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "clarity",
            |m: &VideoMeta| { &m.clarity },
            |m: &mut VideoMeta| { &mut m.clarity },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<VideoMeta>(
            "VideoMeta",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for VideoMeta {
    const NAME: &'static str = "VideoMeta";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.bvid = is.read_string()?;
                },
                18 => {
                    self.title = is.read_string()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.upper)?;
                },
                32 => {
                    self.type_ = is.read_int32()?;
                },
                40 => {
                    self.saved = is.read_bool()?;
                },
                48 => {
                    self.fav = is.read_bool()?;
                },
                56 => {
                    self.expired = is.read_bool()?;
                },
                64 => {
                    self.attr = is.read_int32()?;
                },
                72 => {
                    self.track = is.read_bool()?;
                },
                794 => {
                    is.read_repeated_packed_int64_into(&mut self.list_ids)?;
                },
                792 => {
                    self.list_ids.push(is.read_int64()?);
                },
                802 => {
                    self.clarity = ::std::option::Option::Some(is.read_string()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.bvid.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.bvid);
        }
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.title);
        }
        if let Some(v) = self.upper.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.type_ != 0 {
            my_size += ::protobuf::rt::int32_size(4, self.type_);
        }
        if self.saved != false {
            my_size += 1 + 1;
        }
        if self.fav != false {
            my_size += 1 + 1;
        }
        if self.expired != false {
            my_size += 1 + 1;
        }
        if self.attr != 0 {
            my_size += ::protobuf::rt::int32_size(8, self.attr);
        }
        if self.track != false {
            my_size += 1 + 1;
        }
        for value in &self.list_ids {
            my_size += ::protobuf::rt::int64_size(99, *value);
        };
        if let Some(v) = self.clarity.as_ref() {
            my_size += ::protobuf::rt::string_size(100, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.bvid.is_empty() {
            os.write_string(1, &self.bvid)?;
        }
        if !self.title.is_empty() {
            os.write_string(2, &self.title)?;
        }
        if let Some(v) = self.upper.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if self.type_ != 0 {
            os.write_int32(4, self.type_)?;
        }
        if self.saved != false {
            os.write_bool(5, self.saved)?;
        }
        if self.fav != false {
            os.write_bool(6, self.fav)?;
        }
        if self.expired != false {
            os.write_bool(7, self.expired)?;
        }
        if self.attr != 0 {
            os.write_int32(8, self.attr)?;
        }
        if self.track != false {
            os.write_bool(9, self.track)?;
        }
        for v in &self.list_ids {
            os.write_int64(99, *v)?;
        };
        if let Some(v) = self.clarity.as_ref() {
            os.write_string(100, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> VideoMeta {
        VideoMeta::new()
    }

    fn clear(&mut self) {
        self.bvid.clear();
        self.title.clear();
        self.upper.clear();
        self.type_ = 0;
        self.saved = false;
        self.fav = false;
        self.expired = false;
        self.attr = 0;
        self.track = false;
        self.list_ids.clear();
        self.clarity = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static VideoMeta {
        static instance: VideoMeta = VideoMeta {
            bvid: ::std::string::String::new(),
            title: ::std::string::String::new(),
            upper: ::protobuf::MessageField::none(),
            type_: 0,
            saved: false,
            fav: false,
            expired: false,
            attr: 0,
            track: false,
            list_ids: ::std::vec::Vec::new(),
            clarity: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for VideoMeta {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("VideoMeta").unwrap()).clone()
    }
}

impl ::std::fmt::Display for VideoMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VideoMeta {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:data.UserMeta)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UserMeta {
    // message fields
    // @@protoc_insertion_point(field:data.UserMeta.mid)
    pub mid: i64,
    // @@protoc_insertion_point(field:data.UserMeta.name)
    pub name: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:data.UserMeta.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UserMeta {
    fn default() -> &'a UserMeta {
        <UserMeta as ::protobuf::Message>::default_instance()
    }
}

impl UserMeta {
    pub fn new() -> UserMeta {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "mid",
            |m: &UserMeta| { &m.mid },
            |m: &mut UserMeta| { &mut m.mid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &UserMeta| { &m.name },
            |m: &mut UserMeta| { &mut m.name },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UserMeta>(
            "UserMeta",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UserMeta {
    const NAME: &'static str = "UserMeta";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.mid = is.read_int64()?;
                },
                18 => {
                    self.name = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.mid != 0 {
            my_size += ::protobuf::rt::int64_size(1, self.mid);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.mid != 0 {
            os.write_int64(1, self.mid)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> UserMeta {
        UserMeta::new()
    }

    fn clear(&mut self) {
        self.mid = 0;
        self.name.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UserMeta {
        static instance: UserMeta = UserMeta {
            mid: 0,
            name: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UserMeta {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UserMeta").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UserMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserMeta {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:data.Meta)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Meta {
    // message fields
    // @@protoc_insertion_point(field:data.Meta.videos)
    pub videos: ::std::vec::Vec<VideoMeta>,
    // @@protoc_insertion_point(field:data.Meta.lists)
    pub lists: ::std::vec::Vec<ListMeta>,
    // special fields
    // @@protoc_insertion_point(special_field:data.Meta.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Meta {
    fn default() -> &'a Meta {
        <Meta as ::protobuf::Message>::default_instance()
    }
}

impl Meta {
    pub fn new() -> Meta {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "videos",
            |m: &Meta| { &m.videos },
            |m: &mut Meta| { &mut m.videos },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "lists",
            |m: &Meta| { &m.lists },
            |m: &mut Meta| { &mut m.lists },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Meta>(
            "Meta",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Meta {
    const NAME: &'static str = "Meta";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.videos.push(is.read_message()?);
                },
                18 => {
                    self.lists.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        for value in &self.videos {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.lists {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.videos {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        for v in &self.lists {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Meta {
        Meta::new()
    }

    fn clear(&mut self) {
        self.videos.clear();
        self.lists.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Meta {
        static instance: Meta = Meta {
            videos: ::std::vec::Vec::new(),
            lists: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Meta {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Meta").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Meta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Meta {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\ndata.proto\x12\x04data\"\xa3\x01\n\x06Cookie\x12\x1e\n\nDedeUserID\
    \x18\x01\x20\x01(\tR\nDedeUserID\x12*\n\x11DedeUserID__ckMd5\x18\x02\x20\
    \x01(\tR\x0fDedeUserIDCkMd5\x12\x1a\n\x08SESSDATA\x18\x03\x20\x01(\tR\
    \x08SESSDATA\x12\x19\n\x08bili_jct\x18\x04\x20\x01(\tR\x07biliJct\x12\
    \x16\n\x06buvid3\x18\x05\x20\x01(\tR\x06buvid3\"\xac\x01\n\x08ListMeta\
    \x12\x0e\n\x02id\x18\x01\x20\x01(\x03R\x02id\x12\x14\n\x05title\x18\x02\
    \x20\x01(\tR\x05title\x12\x1f\n\x0bmedia_count\x18\x03\x20\x01(\x05R\nme\
    diaCount\x12\x14\n\x05track\x18\x04\x20\x01(\x08R\x05track\x12\x18\n\x07\
    expired\x18\x05\x20\x01(\x08R\x07expired\x12\x1d\n\x07clarity\x18d\x20\
    \x01(\tH\0R\x07clarity\x88\x01\x01B\n\n\x08_clarity\"\xa1\x02\n\tVideoMe\
    ta\x12\x12\n\x04bvid\x18\x01\x20\x01(\tR\x04bvid\x12\x14\n\x05title\x18\
    \x02\x20\x01(\tR\x05title\x12$\n\x05upper\x18\x03\x20\x01(\x0b2\x0e.data\
    .UserMetaR\x05upper\x12\x12\n\x04type\x18\x04\x20\x01(\x05R\x04type\x12\
    \x14\n\x05saved\x18\x05\x20\x01(\x08R\x05saved\x12\x10\n\x03fav\x18\x06\
    \x20\x01(\x08R\x03fav\x12\x18\n\x07expired\x18\x07\x20\x01(\x08R\x07expi\
    red\x12\x12\n\x04attr\x18\x08\x20\x01(\x05R\x04attr\x12\x14\n\x05track\
    \x18\t\x20\x01(\x08R\x05track\x12\x19\n\x08list_ids\x18c\x20\x03(\x03R\
    \x07listIds\x12\x1d\n\x07clarity\x18d\x20\x01(\tH\0R\x07clarity\x88\x01\
    \x01B\n\n\x08_clarity\"0\n\x08UserMeta\x12\x10\n\x03mid\x18\x01\x20\x01(\
    \x03R\x03mid\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\"U\n\x04Meta\
    \x12'\n\x06videos\x18\x01\x20\x03(\x0b2\x0f.data.VideoMetaR\x06videos\
    \x12$\n\x05lists\x18\x02\x20\x03(\x0b2\x0e.data.ListMetaR\x05listsb\x06p\
    roto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(5);
            messages.push(Cookie::generated_message_descriptor_data());
            messages.push(ListMeta::generated_message_descriptor_data());
            messages.push(VideoMeta::generated_message_descriptor_data());
            messages.push(UserMeta::generated_message_descriptor_data());
            messages.push(Meta::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
