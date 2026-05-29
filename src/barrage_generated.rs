pub use root::*;

const _: () = ::planus::check_version_compatibility("planus-1.3.0");

/// The root namespace
///
/// Generated from these locations:
/// * File `E:\barrage\format\Barrage.fbs`
#[no_implicit_prelude]
#[allow(clippy::needless_lifetimes)]
mod root {
    /// The namespace `io`
    ///
    /// Generated from these locations:
    /// * File `E:\barrage\format\Barrage.fbs`
    pub mod io {
        /// The namespace `io.deephaven`
        ///
        /// Generated from these locations:
        /// * File `E:\barrage\format\Barrage.fbs`
        pub mod deephaven {
            /// The namespace `io.deephaven.barrage`
            ///
            /// Generated from these locations:
            /// * File `E:\barrage\format\Barrage.fbs`
            pub mod barrage {
                /// The namespace `io.deephaven.barrage.flatbuf`
                ///
                /// Generated from these locations:
                /// * File `E:\barrage\format\Barrage.fbs`
                pub mod flatbuf {
                    /// The enum `BarrageMessageType` in the namespace `io.deephaven.barrage.flatbuf`
                    ///
                    /// Generated from these locations:
                    /// * Enum `BarrageMessageType` in the file `E:\barrage\format\Barrage.fbs:17`
                    #[derive(
                        Copy,
                        Clone,
                        Debug,
                        PartialEq,
                        Eq,
                        PartialOrd,
                        Ord,
                        Hash,
                    )]
                    #[repr(i8)]
                    pub enum BarrageMessageType {
                        ///  A barrage message wrapper might send a None message type
                        ///  if the msg_payload is empty.
                        None = 0,

                        ///  enum values 1 - 3 are reserved for future use
                        Unused1 = 1,

                        /// The variant `UNUSED_2` in the enum `BarrageMessageType`
                        Unused2 = 2,

                        /// The variant `UNUSED_3` in the enum `BarrageMessageType`
                        Unused3 = 3,

                        ///  for subscription parsing/management (aka DoPut, DoExchange)
                        BarrageSerializationOptions = 4,

                        /// The variant `BarrageSubscriptionRequest` in the enum `BarrageMessageType`
                        BarrageSubscriptionRequest = 5,

                        /// The variant `BarrageUpdateMetadata` in the enum `BarrageMessageType`
                        BarrageUpdateMetadata = 6,

                        /// The variant `BarrageSnapshotRequest` in the enum `BarrageMessageType`
                        BarrageSnapshotRequest = 7,

                        /// The variant `BarragePublicationRequest` in the enum `BarrageMessageType`
                        BarragePublicationRequest = 8,
                    }

                    impl BarrageMessageType {
                        /// Array containing all valid variants of BarrageMessageType
                        pub const ENUM_VALUES: [Self; 9] = [
                            Self::None,
                            Self::Unused1,
                            Self::Unused2,
                            Self::Unused3,
                            Self::BarrageSerializationOptions,
                            Self::BarrageSubscriptionRequest,
                            Self::BarrageUpdateMetadata,
                            Self::BarrageSnapshotRequest,
                            Self::BarragePublicationRequest,
                        ];
                    }

                    impl ::core::convert::TryFrom<i8> for BarrageMessageType {
                        type Error = ::planus::errors::UnknownEnumTagKind;
                        #[inline]
                        fn try_from(
                            value: i8,
                        ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTagKind>
                        {
                            #[allow(clippy::match_single_binding)]
                            match value {
                                0 => ::core::result::Result::Ok(BarrageMessageType::None),
                                1 => ::core::result::Result::Ok(BarrageMessageType::Unused1),
                                2 => ::core::result::Result::Ok(BarrageMessageType::Unused2),
                                3 => ::core::result::Result::Ok(BarrageMessageType::Unused3),
                                4 => ::core::result::Result::Ok(
                                    BarrageMessageType::BarrageSerializationOptions,
                                ),
                                5 => ::core::result::Result::Ok(
                                    BarrageMessageType::BarrageSubscriptionRequest,
                                ),
                                6 => ::core::result::Result::Ok(
                                    BarrageMessageType::BarrageUpdateMetadata,
                                ),
                                7 => ::core::result::Result::Ok(
                                    BarrageMessageType::BarrageSnapshotRequest,
                                ),
                                8 => ::core::result::Result::Ok(
                                    BarrageMessageType::BarragePublicationRequest,
                                ),

                                _ => ::core::result::Result::Err(
                                    ::planus::errors::UnknownEnumTagKind { tag: value as i128 },
                                ),
                            }
                        }
                    }

                    impl ::core::convert::From<BarrageMessageType> for i8 {
                        #[inline]
                        fn from(value: BarrageMessageType) -> Self {
                            value as i8
                        }
                    }

                    /// # Safety
                    /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
                    unsafe impl ::planus::Primitive for BarrageMessageType {
                        const ALIGNMENT: usize = 1;
                        const SIZE: usize = 1;
                    }

                    impl ::planus::WriteAsPrimitive<BarrageMessageType> for BarrageMessageType {
                        #[inline]
                        fn write<const N: usize>(
                            &self,
                            cursor: ::planus::Cursor<'_, N>,
                            buffer_position: u32,
                        ) {
                            (*self as i8).write(cursor, buffer_position);
                        }
                    }

                    impl ::planus::WriteAs<BarrageMessageType> for BarrageMessageType {
                        type Prepared = Self;

                        #[inline]
                        fn prepare(&self, _builder: &mut ::planus::Builder) -> BarrageMessageType {
                            *self
                        }
                    }

                    impl ::planus::WriteAsDefault<BarrageMessageType, BarrageMessageType> for BarrageMessageType {
                        type Prepared = Self;

                        #[inline]
                        fn prepare(
                            &self,
                            _builder: &mut ::planus::Builder,
                            default: &BarrageMessageType,
                        ) -> ::core::option::Option<BarrageMessageType> {
                            if self == default {
                                ::core::option::Option::None
                            } else {
                                ::core::option::Option::Some(*self)
                            }
                        }
                    }

                    impl ::planus::WriteAsOptional<BarrageMessageType> for BarrageMessageType {
                        type Prepared = Self;

                        #[inline]
                        fn prepare(
                            &self,
                            _builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<BarrageMessageType> {
                            ::core::option::Option::Some(*self)
                        }
                    }

                    impl<'buf> ::planus::TableRead<'buf> for BarrageMessageType {
                        #[inline]
                        fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'buf>,
                            offset: usize,
                        ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind>
                        {
                            let n: i8 = ::planus::TableRead::from_buffer(buffer, offset)?;
                            ::core::result::Result::Ok(::core::convert::TryInto::try_into(n)?)
                        }
                    }

                    impl<'buf> ::planus::VectorReadInner<'buf> for BarrageMessageType {
                        type Error = ::planus::errors::UnknownEnumTag;
                        const STRIDE: usize = 1;
                        #[inline]
                        unsafe fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'buf>,
                            offset: usize,
                        ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTag>
                        {
                            let value = unsafe { *buffer.buffer.get_unchecked(offset) as i8 };
                            let value: ::core::result::Result<Self, _> =
                                ::core::convert::TryInto::try_into(value);
                            value.map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "BarrageMessageType",
                                    "VectorRead::from_buffer",
                                    buffer.offset_from_start,
                                )
                            })
                        }
                    }

                    /// # Safety
                    /// The planus compiler generates implementations that initialize
                    /// the bytes in `write_values`.
                    unsafe impl ::planus::VectorWrite<BarrageMessageType> for BarrageMessageType {
                        const STRIDE: usize = 1;

                        type Value = Self;

                        #[inline]
                        fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                            *self
                        }

                        #[inline]
                        unsafe fn write_values(
                            values: &[Self],
                            bytes: *mut ::core::mem::MaybeUninit<u8>,
                            buffer_position: u32,
                        ) {
                            let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 1];
                            for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                                ::planus::WriteAsPrimitive::write(
                                    v,
                                    ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                                    buffer_position - i as u32,
                                );
                            }
                        }
                    }

                    ///  The message wrapper used for all barrage app_metadata fields.
                    ///
                    /// Generated from these locations:
                    /// * Table `BarrageMessageWrapper` in the file `E:\barrage\format\Barrage.fbs:38`
                    #[derive(
                        Clone,
                        Debug,
                        PartialEq,
                        PartialOrd,
                        Eq,
                        Ord,
                        Hash,
                    )]
                    pub struct BarrageMessageWrapper {
                        ///  Used to identify this type of app_metadata vs other applications.
                        ///  The magic value is '0x6E687064'. It is the numerical representation of the ASCII "dphn".
                        pub magic: u32,
                        ///  The msg type being sent.
                        pub msg_type: self::BarrageMessageType,
                        ///  The msg payload.
                        pub msg_payload: ::core::option::Option<::planus::alloc::vec::Vec<i8>>,
                    }

                    #[allow(clippy::derivable_impls)]
                    impl ::core::default::Default for BarrageMessageWrapper {
                        fn default() -> Self {
                            Self {
                                magic: 0,
                                msg_type: self::BarrageMessageType::None,
                                msg_payload: ::core::default::Default::default(),
                            }
                        }
                    }

                    impl BarrageMessageWrapper {
                        /// Creates a [BarrageMessageWrapperBuilder] for serializing an instance of this table.
                        #[inline]
                        pub fn builder() -> BarrageMessageWrapperBuilder<()> {
                            BarrageMessageWrapperBuilder(())
                        }

                        #[allow(clippy::too_many_arguments)]
                        pub fn create(
                            builder: &mut ::planus::Builder,
                            field_magic: impl ::planus::WriteAsDefault<u32, u32>,
                            field_msg_type: impl ::planus::WriteAsDefault<
                                self::BarrageMessageType,
                                self::BarrageMessageType,
                            >,
                            field_msg_payload: impl ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        ) -> ::planus::Offset<Self> {
                            let prepared_magic = field_magic.prepare(builder, &0);
                            let prepared_msg_type =
                                field_msg_type.prepare(builder, &self::BarrageMessageType::None);
                            let prepared_msg_payload = field_msg_payload.prepare(builder);

                            let mut table_writer: ::planus::table_writer::TableWriter<10> =
                                ::core::default::Default::default();
                            if prepared_magic.is_some() {
                                table_writer.write_entry::<u32>(0);
                            }
                            if prepared_msg_payload.is_some() {
                                table_writer.write_entry::<::planus::Offset<[i8]>>(2);
                            }
                            if prepared_msg_type.is_some() {
                                table_writer.write_entry::<self::BarrageMessageType>(1);
                            }

                            unsafe {
                                table_writer.finish(builder, |object_writer| {
                                    if let ::core::option::Option::Some(prepared_magic) =
                                        prepared_magic
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_magic);
                                    }
                                    if let ::core::option::Option::Some(prepared_msg_payload) =
                                        prepared_msg_payload
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_msg_payload);
                                    }
                                    if let ::core::option::Option::Some(prepared_msg_type) =
                                        prepared_msg_type
                                    {
                                        object_writer.write::<_, _, 1>(&prepared_msg_type);
                                    }
                                });
                            }
                            builder.current_offset()
                        }
                    }

                    impl ::planus::WriteAs<::planus::Offset<BarrageMessageWrapper>> for BarrageMessageWrapper {
                        type Prepared = ::planus::Offset<Self>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageMessageWrapper> {
                            ::planus::WriteAsOffset::prepare(self, builder)
                        }
                    }

                    impl ::planus::WriteAsOptional<::planus::Offset<BarrageMessageWrapper>> for BarrageMessageWrapper {
                        type Prepared = ::planus::Offset<Self>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<::planus::Offset<BarrageMessageWrapper>>
                        {
                            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(
                                self, builder,
                            ))
                        }
                    }

                    impl ::planus::WriteAsOffset<BarrageMessageWrapper> for BarrageMessageWrapper {
                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageMessageWrapper> {
                            BarrageMessageWrapper::create(
                                builder,
                                self.magic,
                                self.msg_type,
                                &self.msg_payload,
                            )
                        }
                    }

                    /// Builder for serializing an instance of the [BarrageMessageWrapper] type.
                    ///
                    /// Can be created using the [BarrageMessageWrapper::builder] method.
                    #[derive(Debug)]
                    #[must_use]
                    pub struct BarrageMessageWrapperBuilder<State>(State);

                    impl BarrageMessageWrapperBuilder<()> {
                        /// Setter for the [`magic` field](BarrageMessageWrapper#structfield.magic).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn magic<T0>(self, value: T0) -> BarrageMessageWrapperBuilder<(T0,)>
                        where
                            T0: ::planus::WriteAsDefault<u32, u32>,
                        {
                            BarrageMessageWrapperBuilder((value,))
                        }

                        /// Sets the [`magic` field](BarrageMessageWrapper#structfield.magic) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn magic_as_default(
                            self,
                        ) -> BarrageMessageWrapperBuilder<(::planus::DefaultValue,)>
                        {
                            self.magic(::planus::DefaultValue)
                        }
                    }

                    impl<T0> BarrageMessageWrapperBuilder<(T0,)> {
                        /// Setter for the [`msg_type` field](BarrageMessageWrapper#structfield.msg_type).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn msg_type<T1>(
                            self,
                            value: T1,
                        ) -> BarrageMessageWrapperBuilder<(T0, T1)>
                        where
                            T1: ::planus::WriteAsDefault<
                                self::BarrageMessageType,
                                self::BarrageMessageType,
                            >,
                        {
                            let (v0,) = self.0;
                            BarrageMessageWrapperBuilder((v0, value))
                        }

                        /// Sets the [`msg_type` field](BarrageMessageWrapper#structfield.msg_type) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn msg_type_as_default(
                            self,
                        ) -> BarrageMessageWrapperBuilder<(T0, ::planus::DefaultValue)>
                        {
                            self.msg_type(::planus::DefaultValue)
                        }
                    }

                    impl<T0, T1> BarrageMessageWrapperBuilder<(T0, T1)> {
                        /// Setter for the [`msg_payload` field](BarrageMessageWrapper#structfield.msg_payload).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn msg_payload<T2>(
                            self,
                            value: T2,
                        ) -> BarrageMessageWrapperBuilder<(T0, T1, T2)>
                        where
                            T2: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        {
                            let (v0, v1) = self.0;
                            BarrageMessageWrapperBuilder((v0, v1, value))
                        }

                        /// Sets the [`msg_payload` field](BarrageMessageWrapper#structfield.msg_payload) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn msg_payload_as_null(
                            self,
                        ) -> BarrageMessageWrapperBuilder<(T0, T1, ())> {
                            self.msg_payload(())
                        }
                    }

                    impl<T0, T1, T2> BarrageMessageWrapperBuilder<(T0, T1, T2)> {
                        /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [BarrageMessageWrapper].
                        #[inline]
                        pub fn finish(
                            self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageMessageWrapper>
                        where
                            Self: ::planus::WriteAsOffset<BarrageMessageWrapper>,
                        {
                            ::planus::WriteAsOffset::prepare(&self, builder)
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsDefault<u32, u32>,
                            T1: ::planus::WriteAsDefault<
                                self::BarrageMessageType,
                                self::BarrageMessageType,
                            >,
                            T2: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        >
                        ::planus::WriteAs<::planus::Offset<BarrageMessageWrapper>>
                        for BarrageMessageWrapperBuilder<(T0, T1, T2)>
                    {
                        type Prepared = ::planus::Offset<BarrageMessageWrapper>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageMessageWrapper> {
                            ::planus::WriteAsOffset::prepare(self, builder)
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsDefault<u32, u32>,
                            T1: ::planus::WriteAsDefault<
                                self::BarrageMessageType,
                                self::BarrageMessageType,
                            >,
                            T2: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        >
                        ::planus::WriteAsOptional<::planus::Offset<BarrageMessageWrapper>>
                        for BarrageMessageWrapperBuilder<(T0, T1, T2)>
                    {
                        type Prepared = ::planus::Offset<BarrageMessageWrapper>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<::planus::Offset<BarrageMessageWrapper>>
                        {
                            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(
                                self, builder,
                            ))
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsDefault<u32, u32>,
                            T1: ::planus::WriteAsDefault<
                                self::BarrageMessageType,
                                self::BarrageMessageType,
                            >,
                            T2: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        > ::planus::WriteAsOffset<BarrageMessageWrapper>
                        for BarrageMessageWrapperBuilder<(T0, T1, T2)>
                    {
                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageMessageWrapper> {
                            let (v0, v1, v2) = &self.0;
                            BarrageMessageWrapper::create(builder, v0, v1, v2)
                        }
                    }

                    /// Reference to a deserialized [BarrageMessageWrapper].
                    #[derive(Copy, Clone)]
                    pub struct BarrageMessageWrapperRef<'a>(
                        #[allow(dead_code)] ::planus::table_reader::Table<'a>,
                    );

                    impl<'a> BarrageMessageWrapperRef<'a> {
                        /// Getter for the [`magic` field](BarrageMessageWrapper#structfield.magic).
                        #[inline]
                        pub fn magic(&self) -> ::planus::Result<u32> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(0, "BarrageMessageWrapper", "magic")?
                                    .unwrap_or(0),
                            )
                        }

                        /// Getter for the [`msg_type` field](BarrageMessageWrapper#structfield.msg_type).
                        #[inline]
                        pub fn msg_type(&self) -> ::planus::Result<self::BarrageMessageType> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(1, "BarrageMessageWrapper", "msg_type")?
                                    .unwrap_or(self::BarrageMessageType::None),
                            )
                        }

                        /// Getter for the [`msg_payload` field](BarrageMessageWrapper#structfield.msg_payload).
                        #[inline]
                        pub fn msg_payload(
                            &self,
                        ) -> ::planus::Result<::core::option::Option<&'a [i8]>>
                        {
                            self.0.access(2, "BarrageMessageWrapper", "msg_payload")
                        }
                    }

                    impl<'a> ::core::fmt::Debug for BarrageMessageWrapperRef<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            let mut f = f.debug_struct("BarrageMessageWrapperRef");
                            f.field("magic", &self.magic());
                            f.field("msg_type", &self.msg_type());
                            if let ::core::option::Option::Some(field_msg_payload) =
                                self.msg_payload().transpose()
                            {
                                f.field("msg_payload", &field_msg_payload);
                            }
                            f.finish()
                        }
                    }

                    impl<'a> ::core::convert::TryFrom<BarrageMessageWrapperRef<'a>> for BarrageMessageWrapper {
                        type Error = ::planus::Error;

                        #[allow(unreachable_code)]
                        fn try_from(value: BarrageMessageWrapperRef<'a>) -> ::planus::Result<Self> {
                            ::core::result::Result::Ok(Self {
                                magic: ::core::convert::TryInto::try_into(value.magic()?)?,
                                msg_type: ::core::convert::TryInto::try_into(value.msg_type()?)?,
                                msg_payload: value.msg_payload()?.map(|v| v.to_vec()),
                            })
                        }
                    }

                    impl<'a> ::planus::TableRead<'a> for BarrageMessageWrapperRef<'a> {
                        #[inline]
                        fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'a>,
                            offset: usize,
                        ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind>
                        {
                            ::core::result::Result::Ok(Self(
                                ::planus::table_reader::Table::from_buffer(buffer, offset)?,
                            ))
                        }
                    }

                    impl<'a> ::planus::VectorReadInner<'a> for BarrageMessageWrapperRef<'a> {
                        type Error = ::planus::Error;
                        const STRIDE: usize = 4;

                        unsafe fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'a>,
                            offset: usize,
                        ) -> ::planus::Result<Self> {
                            ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "[BarrageMessageWrapperRef]",
                                    "get",
                                    buffer.offset_from_start,
                                )
                            })
                        }
                    }

                    /// # Safety
                    /// The planus compiler generates implementations that initialize
                    /// the bytes in `write_values`.
                    unsafe impl ::planus::VectorWrite<::planus::Offset<BarrageMessageWrapper>>
                        for BarrageMessageWrapper
                    {
                        type Value = ::planus::Offset<BarrageMessageWrapper>;
                        const STRIDE: usize = 4;
                        #[inline]
                        fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                            ::planus::WriteAs::prepare(self, builder)
                        }

                        #[inline]
                        unsafe fn write_values(
                            values: &[::planus::Offset<BarrageMessageWrapper>],
                            bytes: *mut ::core::mem::MaybeUninit<u8>,
                            buffer_position: u32,
                        ) {
                            let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                            for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                                ::planus::WriteAsPrimitive::write(
                                    v,
                                    ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                                    buffer_position - (Self::STRIDE * i) as u32,
                                );
                            }
                        }
                    }

                    impl<'a> ::planus::ReadAsRoot<'a> for BarrageMessageWrapperRef<'a> {
                        fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                            ::planus::TableRead::from_buffer(
                                ::planus::SliceWithStartOffset {
                                    buffer: slice,
                                    offset_from_start: 0,
                                },
                                0,
                            )
                            .map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "[BarrageMessageWrapperRef]",
                                    "read_as_root",
                                    0,
                                )
                            })
                        }
                    }

                    ///  There will always be types that cannot be easily supported over IPC. While column conversion mode is no longer
                    ///  supported, users can more explicitly configure the encoding/decoding behavior of the server.
                    ///
                    /// Generated from these locations:
                    /// * Enum `ColumnConversionMode` in the file `E:\barrage\format\Barrage.fbs:52`
                    #[derive(
                        Copy,
                        Clone,
                        Debug,
                        PartialEq,
                        Eq,
                        PartialOrd,
                        Ord,
                        Hash,
                    )]
                    #[repr(i8)]
                    pub enum ColumnConversionMode {
                        /// The variant `Stringify` in the enum `ColumnConversionMode`
                        Stringify = 1,

                        /// The variant `JavaSerialization` in the enum `ColumnConversionMode`
                        JavaSerialization = 2,

                        /// The variant `ThrowError` in the enum `ColumnConversionMode`
                        ThrowError = 3,
                    }

                    impl ColumnConversionMode {
                        /// Array containing all valid variants of ColumnConversionMode
                        pub const ENUM_VALUES: [Self; 3] =
                            [Self::Stringify, Self::JavaSerialization, Self::ThrowError];
                    }

                    impl ::core::convert::TryFrom<i8> for ColumnConversionMode {
                        type Error = ::planus::errors::UnknownEnumTagKind;
                        #[inline]
                        fn try_from(
                            value: i8,
                        ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTagKind>
                        {
                            #[allow(clippy::match_single_binding)]
                            match value {
                                1 => ::core::result::Result::Ok(ColumnConversionMode::Stringify),
                                2 => ::core::result::Result::Ok(
                                    ColumnConversionMode::JavaSerialization,
                                ),
                                3 => ::core::result::Result::Ok(ColumnConversionMode::ThrowError),

                                _ => ::core::result::Result::Err(
                                    ::planus::errors::UnknownEnumTagKind { tag: value as i128 },
                                ),
                            }
                        }
                    }

                    impl ::core::convert::From<ColumnConversionMode> for i8 {
                        #[inline]
                        fn from(value: ColumnConversionMode) -> Self {
                            value as i8
                        }
                    }

                    /// # Safety
                    /// The Planus compiler correctly calculates `ALIGNMENT` and `SIZE`.
                    unsafe impl ::planus::Primitive for ColumnConversionMode {
                        const ALIGNMENT: usize = 1;
                        const SIZE: usize = 1;
                    }

                    impl ::planus::WriteAsPrimitive<ColumnConversionMode> for ColumnConversionMode {
                        #[inline]
                        fn write<const N: usize>(
                            &self,
                            cursor: ::planus::Cursor<'_, N>,
                            buffer_position: u32,
                        ) {
                            (*self as i8).write(cursor, buffer_position);
                        }
                    }

                    impl ::planus::WriteAs<ColumnConversionMode> for ColumnConversionMode {
                        type Prepared = Self;

                        #[inline]
                        fn prepare(
                            &self,
                            _builder: &mut ::planus::Builder,
                        ) -> ColumnConversionMode {
                            *self
                        }
                    }

                    impl ::planus::WriteAsDefault<ColumnConversionMode, ColumnConversionMode> for ColumnConversionMode {
                        type Prepared = Self;

                        #[inline]
                        fn prepare(
                            &self,
                            _builder: &mut ::planus::Builder,
                            default: &ColumnConversionMode,
                        ) -> ::core::option::Option<ColumnConversionMode> {
                            if self == default {
                                ::core::option::Option::None
                            } else {
                                ::core::option::Option::Some(*self)
                            }
                        }
                    }

                    impl ::planus::WriteAsOptional<ColumnConversionMode> for ColumnConversionMode {
                        type Prepared = Self;

                        #[inline]
                        fn prepare(
                            &self,
                            _builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<ColumnConversionMode> {
                            ::core::option::Option::Some(*self)
                        }
                    }

                    impl<'buf> ::planus::TableRead<'buf> for ColumnConversionMode {
                        #[inline]
                        fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'buf>,
                            offset: usize,
                        ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind>
                        {
                            let n: i8 = ::planus::TableRead::from_buffer(buffer, offset)?;
                            ::core::result::Result::Ok(::core::convert::TryInto::try_into(n)?)
                        }
                    }

                    impl<'buf> ::planus::VectorReadInner<'buf> for ColumnConversionMode {
                        type Error = ::planus::errors::UnknownEnumTag;
                        const STRIDE: usize = 1;
                        #[inline]
                        unsafe fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'buf>,
                            offset: usize,
                        ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTag>
                        {
                            let value = unsafe { *buffer.buffer.get_unchecked(offset) as i8 };
                            let value: ::core::result::Result<Self, _> =
                                ::core::convert::TryInto::try_into(value);
                            value.map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "ColumnConversionMode",
                                    "VectorRead::from_buffer",
                                    buffer.offset_from_start,
                                )
                            })
                        }
                    }

                    /// # Safety
                    /// The planus compiler generates implementations that initialize
                    /// the bytes in `write_values`.
                    unsafe impl ::planus::VectorWrite<ColumnConversionMode> for ColumnConversionMode {
                        const STRIDE: usize = 1;

                        type Value = Self;

                        #[inline]
                        fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                            *self
                        }

                        #[inline]
                        unsafe fn write_values(
                            values: &[Self],
                            bytes: *mut ::core::mem::MaybeUninit<u8>,
                            buffer_position: u32,
                        ) {
                            let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 1];
                            for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                                ::planus::WriteAsPrimitive::write(
                                    v,
                                    ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                                    buffer_position - i as u32,
                                );
                            }
                        }
                    }

                    /// The table `BarrageSubscriptionOptions` in the namespace `io.deephaven.barrage.flatbuf`
                    ///
                    /// Generated from these locations:
                    /// * Table `BarrageSubscriptionOptions` in the file `E:\barrage\format\Barrage.fbs:54`
                    #[derive(
                        Clone,
                        Debug,
                        PartialEq,
                        PartialOrd,
                        Eq,
                        Ord,
                        Hash,
                    )]
                    pub struct BarrageSubscriptionOptions {
                        ///  Deephaven reserves a value in the range of primitives as a custom NULL value. This enables more efficient transmission
                        ///  by eliminating the additional complexity of the validity buffer.
                        pub use_deephaven_nulls: bool,
                        ///  Explicitly set the update interval for this subscription. Note that subscriptions with different update intervals
                        ///  cannot share intermediary state with other subscriptions and greatly increases the footprint of the non-conforming subscription.
                        ///
                        ///  Note: if not supplied (default of zero) then the server uses a consistent value to be efficient and fair to all clients
                        pub min_update_interval_ms: i32,
                        ///  Specify a preferred batch size. Server is allowed to be configured to restrict possible values. Too small of a
                        ///  batch size may be dominated with header costs as each batch is wrapped into a separate RecordBatch. Too large of
                        ///  a payload and it may not fit within the maximum payload size. A good default might be 4096.
                        ///
                        ///  a batch_size of -1 indicates that the server should avoid batching a single logical message
                        pub batch_size: i32,
                        ///  Specify a maximum allowed message size. Server will enforce this limit by reducing batch size (to a lower limit
                        ///  of one row per batch). If the message size limit cannot be met due to large row sizes, the server will throw a
                        ///  `Status.RESOURCE_EXHAUSTED` exception
                        pub max_message_size: i32,
                        ///  If true, the server will wrap columns with a list. This is useful for clients that do not support modified batches
                        ///  with columns of differing lengths.
                        pub columns_as_list: bool,
                        ///  The maximum length of any list / array to encode.
                        ///  - If zero, list lengths will not be limited.
                        ///  - If non-zero, the server will limit the length of any encoded list / array to the absolute value of the returned length.
                        ///  - If less than zero, the server will encode elements from the end of the list / array, rather than from the beginning.
                        ///
                        ///  Note: The server is unable to indicate when truncation occurs. To detect truncation request one more element than
                        ///  the maximum number you wish to display.
                        pub preview_list_length_limit: i64,
                    }

                    #[allow(clippy::derivable_impls)]
                    impl ::core::default::Default for BarrageSubscriptionOptions {
                        fn default() -> Self {
                            Self {
                                use_deephaven_nulls: false,
                                min_update_interval_ms: 0,
                                batch_size: 0,
                                max_message_size: 0,
                                columns_as_list: false,
                                preview_list_length_limit: 0,
                            }
                        }
                    }

                    impl BarrageSubscriptionOptions {
                        /// Creates a [BarrageSubscriptionOptionsBuilder] for serializing an instance of this table.
                        #[inline]
                        pub fn builder() -> BarrageSubscriptionOptionsBuilder<()> {
                            BarrageSubscriptionOptionsBuilder(())
                        }

                        #[allow(clippy::too_many_arguments)]
                        pub fn create(
                            builder: &mut ::planus::Builder,
                            field_use_deephaven_nulls: impl ::planus::WriteAsDefault<bool, bool>,
                            field_min_update_interval_ms: impl ::planus::WriteAsDefault<i32, i32>,
                            field_batch_size: impl ::planus::WriteAsDefault<i32, i32>,
                            field_max_message_size: impl ::planus::WriteAsDefault<i32, i32>,
                            field_columns_as_list: impl ::planus::WriteAsDefault<bool, bool>,
                            field_preview_list_length_limit: impl ::planus::WriteAsDefault<i64, i64>,
                        ) -> ::planus::Offset<Self> {
                            let prepared_use_deephaven_nulls =
                                field_use_deephaven_nulls.prepare(builder, &false);
                            let prepared_min_update_interval_ms =
                                field_min_update_interval_ms.prepare(builder, &0);
                            let prepared_batch_size = field_batch_size.prepare(builder, &0);
                            let prepared_max_message_size =
                                field_max_message_size.prepare(builder, &0);
                            let prepared_columns_as_list =
                                field_columns_as_list.prepare(builder, &false);
                            let prepared_preview_list_length_limit =
                                field_preview_list_length_limit.prepare(builder, &0);

                            let mut table_writer: ::planus::table_writer::TableWriter<18> =
                                ::core::default::Default::default();
                            if prepared_preview_list_length_limit.is_some() {
                                table_writer.write_entry::<i64>(6);
                            }
                            if prepared_min_update_interval_ms.is_some() {
                                table_writer.write_entry::<i32>(2);
                            }
                            if prepared_batch_size.is_some() {
                                table_writer.write_entry::<i32>(3);
                            }
                            if prepared_max_message_size.is_some() {
                                table_writer.write_entry::<i32>(4);
                            }
                            if prepared_use_deephaven_nulls.is_some() {
                                table_writer.write_entry::<bool>(1);
                            }
                            if prepared_columns_as_list.is_some() {
                                table_writer.write_entry::<bool>(5);
                            }

                            unsafe {
                                table_writer.finish(builder, |object_writer| {
                                    if let ::core::option::Option::Some(
                                        prepared_preview_list_length_limit,
                                    ) = prepared_preview_list_length_limit
                                    {
                                        object_writer
                                            .write::<_, _, 8>(&prepared_preview_list_length_limit);
                                    }
                                    if let ::core::option::Option::Some(
                                        prepared_min_update_interval_ms,
                                    ) = prepared_min_update_interval_ms
                                    {
                                        object_writer
                                            .write::<_, _, 4>(&prepared_min_update_interval_ms);
                                    }
                                    if let ::core::option::Option::Some(prepared_batch_size) =
                                        prepared_batch_size
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_batch_size);
                                    }
                                    if let ::core::option::Option::Some(prepared_max_message_size) =
                                        prepared_max_message_size
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_max_message_size);
                                    }
                                    if let ::core::option::Option::Some(
                                        prepared_use_deephaven_nulls,
                                    ) = prepared_use_deephaven_nulls
                                    {
                                        object_writer
                                            .write::<_, _, 1>(&prepared_use_deephaven_nulls);
                                    }
                                    if let ::core::option::Option::Some(prepared_columns_as_list) =
                                        prepared_columns_as_list
                                    {
                                        object_writer.write::<_, _, 1>(&prepared_columns_as_list);
                                    }
                                });
                            }
                            builder.current_offset()
                        }
                    }

                    impl ::planus::WriteAs<::planus::Offset<BarrageSubscriptionOptions>>
                        for BarrageSubscriptionOptions
                    {
                        type Prepared = ::planus::Offset<Self>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSubscriptionOptions> {
                            ::planus::WriteAsOffset::prepare(self, builder)
                        }
                    }

                    impl ::planus::WriteAsOptional<::planus::Offset<BarrageSubscriptionOptions>>
                        for BarrageSubscriptionOptions
                    {
                        type Prepared = ::planus::Offset<Self>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<::planus::Offset<BarrageSubscriptionOptions>>
                        {
                            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(
                                self, builder,
                            ))
                        }
                    }

                    impl ::planus::WriteAsOffset<BarrageSubscriptionOptions> for BarrageSubscriptionOptions {
                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSubscriptionOptions> {
                            BarrageSubscriptionOptions::create(
                                builder,
                                self.use_deephaven_nulls,
                                self.min_update_interval_ms,
                                self.batch_size,
                                self.max_message_size,
                                self.columns_as_list,
                                self.preview_list_length_limit,
                            )
                        }
                    }

                    /// Builder for serializing an instance of the [BarrageSubscriptionOptions] type.
                    ///
                    /// Can be created using the [BarrageSubscriptionOptions::builder] method.
                    #[derive(Debug)]
                    #[must_use]
                    pub struct BarrageSubscriptionOptionsBuilder<State>(State);

                    impl BarrageSubscriptionOptionsBuilder<()> {
                        /// Setter for the [`use_deephaven_nulls` field](BarrageSubscriptionOptions#structfield.use_deephaven_nulls).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn use_deephaven_nulls<T0>(
                            self,
                            value: T0,
                        ) -> BarrageSubscriptionOptionsBuilder<(T0,)>
                        where
                            T0: ::planus::WriteAsDefault<bool, bool>,
                        {
                            BarrageSubscriptionOptionsBuilder((value,))
                        }

                        /// Sets the [`use_deephaven_nulls` field](BarrageSubscriptionOptions#structfield.use_deephaven_nulls) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn use_deephaven_nulls_as_default(
                            self,
                        ) -> BarrageSubscriptionOptionsBuilder<(::planus::DefaultValue,)>
                        {
                            self.use_deephaven_nulls(::planus::DefaultValue)
                        }
                    }

                    impl<T0> BarrageSubscriptionOptionsBuilder<(T0,)> {
                        /// Setter for the [`min_update_interval_ms` field](BarrageSubscriptionOptions#structfield.min_update_interval_ms).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn min_update_interval_ms<T1>(
                            self,
                            value: T1,
                        ) -> BarrageSubscriptionOptionsBuilder<(T0, T1)>
                        where
                            T1: ::planus::WriteAsDefault<i32, i32>,
                        {
                            let (v0,) = self.0;
                            BarrageSubscriptionOptionsBuilder((v0, value))
                        }

                        /// Sets the [`min_update_interval_ms` field](BarrageSubscriptionOptions#structfield.min_update_interval_ms) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn min_update_interval_ms_as_default(
                            self,
                        ) -> BarrageSubscriptionOptionsBuilder<(T0, ::planus::DefaultValue)>
                        {
                            self.min_update_interval_ms(::planus::DefaultValue)
                        }
                    }

                    impl<T0, T1> BarrageSubscriptionOptionsBuilder<(T0, T1)> {
                        /// Setter for the [`batch_size` field](BarrageSubscriptionOptions#structfield.batch_size).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn batch_size<T2>(
                            self,
                            value: T2,
                        ) -> BarrageSubscriptionOptionsBuilder<(T0, T1, T2)>
                        where
                            T2: ::planus::WriteAsDefault<i32, i32>,
                        {
                            let (v0, v1) = self.0;
                            BarrageSubscriptionOptionsBuilder((v0, v1, value))
                        }

                        /// Sets the [`batch_size` field](BarrageSubscriptionOptions#structfield.batch_size) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn batch_size_as_default(
                            self,
                        ) -> BarrageSubscriptionOptionsBuilder<(T0, T1, ::planus::DefaultValue)>
                        {
                            self.batch_size(::planus::DefaultValue)
                        }
                    }

                    impl<T0, T1, T2> BarrageSubscriptionOptionsBuilder<(T0, T1, T2)> {
                        /// Setter for the [`max_message_size` field](BarrageSubscriptionOptions#structfield.max_message_size).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn max_message_size<T3>(
                            self,
                            value: T3,
                        ) -> BarrageSubscriptionOptionsBuilder<(T0, T1, T2, T3)>
                        where
                            T3: ::planus::WriteAsDefault<i32, i32>,
                        {
                            let (v0, v1, v2) = self.0;
                            BarrageSubscriptionOptionsBuilder((v0, v1, v2, value))
                        }

                        /// Sets the [`max_message_size` field](BarrageSubscriptionOptions#structfield.max_message_size) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn max_message_size_as_default(
                            self,
                        ) -> BarrageSubscriptionOptionsBuilder<(T0, T1, T2, ::planus::DefaultValue)>
                        {
                            self.max_message_size(::planus::DefaultValue)
                        }
                    }

                    impl<T0, T1, T2, T3> BarrageSubscriptionOptionsBuilder<(T0, T1, T2, T3)> {
                        /// Setter for the [`columns_as_list` field](BarrageSubscriptionOptions#structfield.columns_as_list).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn columns_as_list<T4>(
                            self,
                            value: T4,
                        ) -> BarrageSubscriptionOptionsBuilder<(T0, T1, T2, T3, T4)>
                        where
                            T4: ::planus::WriteAsDefault<bool, bool>,
                        {
                            let (v0, v1, v2, v3) = self.0;
                            BarrageSubscriptionOptionsBuilder((v0, v1, v2, v3, value))
                        }

                        /// Sets the [`columns_as_list` field](BarrageSubscriptionOptions#structfield.columns_as_list) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn columns_as_list_as_default(
                            self,
                        ) -> BarrageSubscriptionOptionsBuilder<(
                            T0,
                            T1,
                            T2,
                            T3,
                            ::planus::DefaultValue,
                        )> {
                            self.columns_as_list(::planus::DefaultValue)
                        }
                    }

                    impl<T0, T1, T2, T3, T4> BarrageSubscriptionOptionsBuilder<(T0, T1, T2, T3, T4)> {
                        /// Setter for the [`preview_list_length_limit` field](BarrageSubscriptionOptions#structfield.preview_list_length_limit).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn preview_list_length_limit<T5>(
                            self,
                            value: T5,
                        ) -> BarrageSubscriptionOptionsBuilder<(T0, T1, T2, T3, T4, T5)>
                        where
                            T5: ::planus::WriteAsDefault<i64, i64>,
                        {
                            let (v0, v1, v2, v3, v4) = self.0;
                            BarrageSubscriptionOptionsBuilder((v0, v1, v2, v3, v4, value))
                        }

                        /// Sets the [`preview_list_length_limit` field](BarrageSubscriptionOptions#structfield.preview_list_length_limit) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn preview_list_length_limit_as_default(
                            self,
                        ) -> BarrageSubscriptionOptionsBuilder<(
                            T0,
                            T1,
                            T2,
                            T3,
                            T4,
                            ::planus::DefaultValue,
                        )> {
                            self.preview_list_length_limit(::planus::DefaultValue)
                        }
                    }

                    impl<T0, T1, T2, T3, T4, T5> BarrageSubscriptionOptionsBuilder<(T0, T1, T2, T3, T4, T5)> {
                        /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [BarrageSubscriptionOptions].
                        #[inline]
                        pub fn finish(
                            self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSubscriptionOptions>
                        where
                            Self: ::planus::WriteAsOffset<BarrageSubscriptionOptions>,
                        {
                            ::planus::WriteAsOffset::prepare(&self, builder)
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsDefault<bool, bool>,
                            T1: ::planus::WriteAsDefault<i32, i32>,
                            T2: ::planus::WriteAsDefault<i32, i32>,
                            T3: ::planus::WriteAsDefault<i32, i32>,
                            T4: ::planus::WriteAsDefault<bool, bool>,
                            T5: ::planus::WriteAsDefault<i64, i64>,
                        >
                        ::planus::WriteAs<::planus::Offset<BarrageSubscriptionOptions>>
                        for BarrageSubscriptionOptionsBuilder<(T0, T1, T2, T3, T4, T5)>
                    {
                        type Prepared = ::planus::Offset<BarrageSubscriptionOptions>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSubscriptionOptions> {
                            ::planus::WriteAsOffset::prepare(self, builder)
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsDefault<bool, bool>,
                            T1: ::planus::WriteAsDefault<i32, i32>,
                            T2: ::planus::WriteAsDefault<i32, i32>,
                            T3: ::planus::WriteAsDefault<i32, i32>,
                            T4: ::planus::WriteAsDefault<bool, bool>,
                            T5: ::planus::WriteAsDefault<i64, i64>,
                        >
                        ::planus::WriteAsOptional<::planus::Offset<BarrageSubscriptionOptions>>
                        for BarrageSubscriptionOptionsBuilder<(T0, T1, T2, T3, T4, T5)>
                    {
                        type Prepared = ::planus::Offset<BarrageSubscriptionOptions>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<::planus::Offset<BarrageSubscriptionOptions>>
                        {
                            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(
                                self, builder,
                            ))
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsDefault<bool, bool>,
                            T1: ::planus::WriteAsDefault<i32, i32>,
                            T2: ::planus::WriteAsDefault<i32, i32>,
                            T3: ::planus::WriteAsDefault<i32, i32>,
                            T4: ::planus::WriteAsDefault<bool, bool>,
                            T5: ::planus::WriteAsDefault<i64, i64>,
                        > ::planus::WriteAsOffset<BarrageSubscriptionOptions>
                        for BarrageSubscriptionOptionsBuilder<(T0, T1, T2, T3, T4, T5)>
                    {
                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSubscriptionOptions> {
                            let (v0, v1, v2, v3, v4, v5) = &self.0;
                            BarrageSubscriptionOptions::create(builder, v0, v1, v2, v3, v4, v5)
                        }
                    }

                    /// Reference to a deserialized [BarrageSubscriptionOptions].
                    #[derive(Copy, Clone)]
                    pub struct BarrageSubscriptionOptionsRef<'a>(
                        #[allow(dead_code)] ::planus::table_reader::Table<'a>,
                    );

                    impl<'a> BarrageSubscriptionOptionsRef<'a> {
                        /// Getter for the [`use_deephaven_nulls` field](BarrageSubscriptionOptions#structfield.use_deephaven_nulls).
                        #[inline]
                        pub fn use_deephaven_nulls(&self) -> ::planus::Result<bool> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(1, "BarrageSubscriptionOptions", "use_deephaven_nulls")?
                                    .unwrap_or(false),
                            )
                        }

                        /// Getter for the [`min_update_interval_ms` field](BarrageSubscriptionOptions#structfield.min_update_interval_ms).
                        #[inline]
                        pub fn min_update_interval_ms(&self) -> ::planus::Result<i32> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(
                                        2,
                                        "BarrageSubscriptionOptions",
                                        "min_update_interval_ms",
                                    )?
                                    .unwrap_or(0),
                            )
                        }

                        /// Getter for the [`batch_size` field](BarrageSubscriptionOptions#structfield.batch_size).
                        #[inline]
                        pub fn batch_size(&self) -> ::planus::Result<i32> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(3, "BarrageSubscriptionOptions", "batch_size")?
                                    .unwrap_or(0),
                            )
                        }

                        /// Getter for the [`max_message_size` field](BarrageSubscriptionOptions#structfield.max_message_size).
                        #[inline]
                        pub fn max_message_size(&self) -> ::planus::Result<i32> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(4, "BarrageSubscriptionOptions", "max_message_size")?
                                    .unwrap_or(0),
                            )
                        }

                        /// Getter for the [`columns_as_list` field](BarrageSubscriptionOptions#structfield.columns_as_list).
                        #[inline]
                        pub fn columns_as_list(&self) -> ::planus::Result<bool> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(5, "BarrageSubscriptionOptions", "columns_as_list")?
                                    .unwrap_or(false),
                            )
                        }

                        /// Getter for the [`preview_list_length_limit` field](BarrageSubscriptionOptions#structfield.preview_list_length_limit).
                        #[inline]
                        pub fn preview_list_length_limit(&self) -> ::planus::Result<i64> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(
                                        6,
                                        "BarrageSubscriptionOptions",
                                        "preview_list_length_limit",
                                    )?
                                    .unwrap_or(0),
                            )
                        }
                    }

                    impl<'a> ::core::fmt::Debug for BarrageSubscriptionOptionsRef<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            let mut f = f.debug_struct("BarrageSubscriptionOptionsRef");
                            f.field("use_deephaven_nulls", &self.use_deephaven_nulls());
                            f.field("min_update_interval_ms", &self.min_update_interval_ms());
                            f.field("batch_size", &self.batch_size());
                            f.field("max_message_size", &self.max_message_size());
                            f.field("columns_as_list", &self.columns_as_list());
                            f.field(
                                "preview_list_length_limit",
                                &self.preview_list_length_limit(),
                            );
                            f.finish()
                        }
                    }

                    impl<'a> ::core::convert::TryFrom<BarrageSubscriptionOptionsRef<'a>>
                        for BarrageSubscriptionOptions
                    {
                        type Error = ::planus::Error;

                        #[allow(unreachable_code)]
                        fn try_from(
                            value: BarrageSubscriptionOptionsRef<'a>,
                        ) -> ::planus::Result<Self> {
                            ::core::result::Result::Ok(Self {
                                use_deephaven_nulls: ::core::convert::TryInto::try_into(
                                    value.use_deephaven_nulls()?,
                                )?,
                                min_update_interval_ms: ::core::convert::TryInto::try_into(
                                    value.min_update_interval_ms()?,
                                )?,
                                batch_size: ::core::convert::TryInto::try_into(
                                    value.batch_size()?,
                                )?,
                                max_message_size: ::core::convert::TryInto::try_into(
                                    value.max_message_size()?,
                                )?,
                                columns_as_list: ::core::convert::TryInto::try_into(
                                    value.columns_as_list()?,
                                )?,
                                preview_list_length_limit: ::core::convert::TryInto::try_into(
                                    value.preview_list_length_limit()?,
                                )?,
                            })
                        }
                    }

                    impl<'a> ::planus::TableRead<'a> for BarrageSubscriptionOptionsRef<'a> {
                        #[inline]
                        fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'a>,
                            offset: usize,
                        ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind>
                        {
                            ::core::result::Result::Ok(Self(
                                ::planus::table_reader::Table::from_buffer(buffer, offset)?,
                            ))
                        }
                    }

                    impl<'a> ::planus::VectorReadInner<'a> for BarrageSubscriptionOptionsRef<'a> {
                        type Error = ::planus::Error;
                        const STRIDE: usize = 4;

                        unsafe fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'a>,
                            offset: usize,
                        ) -> ::planus::Result<Self> {
                            ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "[BarrageSubscriptionOptionsRef]",
                                    "get",
                                    buffer.offset_from_start,
                                )
                            })
                        }
                    }

                    /// # Safety
                    /// The planus compiler generates implementations that initialize
                    /// the bytes in `write_values`.
                    unsafe impl ::planus::VectorWrite<::planus::Offset<BarrageSubscriptionOptions>>
                        for BarrageSubscriptionOptions
                    {
                        type Value = ::planus::Offset<BarrageSubscriptionOptions>;
                        const STRIDE: usize = 4;
                        #[inline]
                        fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                            ::planus::WriteAs::prepare(self, builder)
                        }

                        #[inline]
                        unsafe fn write_values(
                            values: &[::planus::Offset<BarrageSubscriptionOptions>],
                            bytes: *mut ::core::mem::MaybeUninit<u8>,
                            buffer_position: u32,
                        ) {
                            let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                            for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                                ::planus::WriteAsPrimitive::write(
                                    v,
                                    ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                                    buffer_position - (Self::STRIDE * i) as u32,
                                );
                            }
                        }
                    }

                    impl<'a> ::planus::ReadAsRoot<'a> for BarrageSubscriptionOptionsRef<'a> {
                        fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                            ::planus::TableRead::from_buffer(
                                ::planus::SliceWithStartOffset {
                                    buffer: slice,
                                    offset_from_start: 0,
                                },
                                0,
                            )
                            .map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "[BarrageSubscriptionOptionsRef]",
                                    "read_as_root",
                                    0,
                                )
                            })
                        }
                    }

                    ///  Describes the subscription the client would like to acquire.
                    ///
                    /// Generated from these locations:
                    /// * Table `BarrageSubscriptionRequest` in the file `E:\barrage\format\Barrage.fbs:95`
                    #[derive(
                        Clone,
                        Debug,
                        PartialEq,
                        PartialOrd,
                        Eq,
                        Ord,
                        Hash,
                    )]
                    pub struct BarrageSubscriptionRequest {
                        ///  Ticket for the source data set.
                        pub ticket: ::core::option::Option<::planus::alloc::vec::Vec<i8>>,
                        ///  The bitset of columns to subscribe. If not provided then all columns are subscribed.
                        pub columns: ::core::option::Option<::planus::alloc::vec::Vec<i8>>,
                        ///  This is an encoded and compressed RowSet in position-space to subscribe to. If not provided then the entire
                        ///  table is requested.
                        pub viewport: ::core::option::Option<::planus::alloc::vec::Vec<i8>>,
                        ///  Options to configure your subscription.
                        pub subscription_options: ::core::option::Option<
                            ::planus::alloc::boxed::Box<self::BarrageSubscriptionOptions>,
                        >,
                        ///  When this is set the viewport RowSet will be inverted against the length of the table. That is to say
                        ///  every position value is converted from `i` to `n - i - 1` if the table has `n` rows.
                        pub reverse_viewport: bool,
                        ///  If this is set, the server will parrot this subscription token in the response. This token can be used to identify
                        ///  which subscription the server is now respecting.
                        pub subscription_token:
                            ::core::option::Option<::planus::alloc::vec::Vec<i8>>,
                    }

                    #[allow(clippy::derivable_impls)]
                    impl ::core::default::Default for BarrageSubscriptionRequest {
                        fn default() -> Self {
                            Self {
                                ticket: ::core::default::Default::default(),
                                columns: ::core::default::Default::default(),
                                viewport: ::core::default::Default::default(),
                                subscription_options: ::core::default::Default::default(),
                                reverse_viewport: false,
                                subscription_token: ::core::default::Default::default(),
                            }
                        }
                    }

                    impl BarrageSubscriptionRequest {
                        /// Creates a [BarrageSubscriptionRequestBuilder] for serializing an instance of this table.
                        #[inline]
                        pub fn builder() -> BarrageSubscriptionRequestBuilder<()> {
                            BarrageSubscriptionRequestBuilder(())
                        }

                        #[allow(clippy::too_many_arguments)]
                        pub fn create(
                            builder: &mut ::planus::Builder,
                            field_ticket: impl ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            field_columns: impl ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            field_viewport: impl ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            field_subscription_options: impl ::planus::WriteAsOptional<
                                ::planus::Offset<self::BarrageSubscriptionOptions>,
                            >,
                            field_reverse_viewport: impl ::planus::WriteAsDefault<bool, bool>,
                            field_subscription_token: impl ::planus::WriteAsOptional<
                                ::planus::Offset<[i8]>,
                            >,
                        ) -> ::planus::Offset<Self> {
                            let prepared_ticket = field_ticket.prepare(builder);
                            let prepared_columns = field_columns.prepare(builder);
                            let prepared_viewport = field_viewport.prepare(builder);
                            let prepared_subscription_options =
                                field_subscription_options.prepare(builder);
                            let prepared_reverse_viewport =
                                field_reverse_viewport.prepare(builder, &false);
                            let prepared_subscription_token =
                                field_subscription_token.prepare(builder);

                            let mut table_writer: ::planus::table_writer::TableWriter<16> =
                                ::core::default::Default::default();
                            if prepared_ticket.is_some() {
                                table_writer.write_entry::<::planus::Offset<[i8]>>(0);
                            }
                            if prepared_columns.is_some() {
                                table_writer.write_entry::<::planus::Offset<[i8]>>(1);
                            }
                            if prepared_viewport.is_some() {
                                table_writer.write_entry::<::planus::Offset<[i8]>>(2);
                            }
                            if prepared_subscription_options.is_some() {
                                table_writer.write_entry::<::planus::Offset<self::BarrageSubscriptionOptions>>(3);
                            }
                            if prepared_subscription_token.is_some() {
                                table_writer.write_entry::<::planus::Offset<[i8]>>(5);
                            }
                            if prepared_reverse_viewport.is_some() {
                                table_writer.write_entry::<bool>(4);
                            }

                            unsafe {
                                table_writer.finish(builder, |object_writer| {
                                    if let ::core::option::Option::Some(prepared_ticket) =
                                        prepared_ticket
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_ticket);
                                    }
                                    if let ::core::option::Option::Some(prepared_columns) =
                                        prepared_columns
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_columns);
                                    }
                                    if let ::core::option::Option::Some(prepared_viewport) =
                                        prepared_viewport
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_viewport);
                                    }
                                    if let ::core::option::Option::Some(
                                        prepared_subscription_options,
                                    ) = prepared_subscription_options
                                    {
                                        object_writer
                                            .write::<_, _, 4>(&prepared_subscription_options);
                                    }
                                    if let ::core::option::Option::Some(
                                        prepared_subscription_token,
                                    ) = prepared_subscription_token
                                    {
                                        object_writer
                                            .write::<_, _, 4>(&prepared_subscription_token);
                                    }
                                    if let ::core::option::Option::Some(prepared_reverse_viewport) =
                                        prepared_reverse_viewport
                                    {
                                        object_writer.write::<_, _, 1>(&prepared_reverse_viewport);
                                    }
                                });
                            }
                            builder.current_offset()
                        }
                    }

                    impl ::planus::WriteAs<::planus::Offset<BarrageSubscriptionRequest>>
                        for BarrageSubscriptionRequest
                    {
                        type Prepared = ::planus::Offset<Self>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSubscriptionRequest> {
                            ::planus::WriteAsOffset::prepare(self, builder)
                        }
                    }

                    impl ::planus::WriteAsOptional<::planus::Offset<BarrageSubscriptionRequest>>
                        for BarrageSubscriptionRequest
                    {
                        type Prepared = ::planus::Offset<Self>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<::planus::Offset<BarrageSubscriptionRequest>>
                        {
                            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(
                                self, builder,
                            ))
                        }
                    }

                    impl ::planus::WriteAsOffset<BarrageSubscriptionRequest> for BarrageSubscriptionRequest {
                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSubscriptionRequest> {
                            BarrageSubscriptionRequest::create(
                                builder,
                                &self.ticket,
                                &self.columns,
                                &self.viewport,
                                &self.subscription_options,
                                self.reverse_viewport,
                                &self.subscription_token,
                            )
                        }
                    }

                    /// Builder for serializing an instance of the [BarrageSubscriptionRequest] type.
                    ///
                    /// Can be created using the [BarrageSubscriptionRequest::builder] method.
                    #[derive(Debug)]
                    #[must_use]
                    pub struct BarrageSubscriptionRequestBuilder<State>(State);

                    impl BarrageSubscriptionRequestBuilder<()> {
                        /// Setter for the [`ticket` field](BarrageSubscriptionRequest#structfield.ticket).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn ticket<T0>(
                            self,
                            value: T0,
                        ) -> BarrageSubscriptionRequestBuilder<(T0,)>
                        where
                            T0: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        {
                            BarrageSubscriptionRequestBuilder((value,))
                        }

                        /// Sets the [`ticket` field](BarrageSubscriptionRequest#structfield.ticket) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn ticket_as_null(self) -> BarrageSubscriptionRequestBuilder<((),)> {
                            self.ticket(())
                        }
                    }

                    impl<T0> BarrageSubscriptionRequestBuilder<(T0,)> {
                        /// Setter for the [`columns` field](BarrageSubscriptionRequest#structfield.columns).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn columns<T1>(
                            self,
                            value: T1,
                        ) -> BarrageSubscriptionRequestBuilder<(T0, T1)>
                        where
                            T1: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        {
                            let (v0,) = self.0;
                            BarrageSubscriptionRequestBuilder((v0, value))
                        }

                        /// Sets the [`columns` field](BarrageSubscriptionRequest#structfield.columns) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn columns_as_null(
                            self,
                        ) -> BarrageSubscriptionRequestBuilder<(T0, ())> {
                            self.columns(())
                        }
                    }

                    impl<T0, T1> BarrageSubscriptionRequestBuilder<(T0, T1)> {
                        /// Setter for the [`viewport` field](BarrageSubscriptionRequest#structfield.viewport).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn viewport<T2>(
                            self,
                            value: T2,
                        ) -> BarrageSubscriptionRequestBuilder<(T0, T1, T2)>
                        where
                            T2: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        {
                            let (v0, v1) = self.0;
                            BarrageSubscriptionRequestBuilder((v0, v1, value))
                        }

                        /// Sets the [`viewport` field](BarrageSubscriptionRequest#structfield.viewport) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn viewport_as_null(
                            self,
                        ) -> BarrageSubscriptionRequestBuilder<(T0, T1, ())>
                        {
                            self.viewport(())
                        }
                    }

                    impl<T0, T1, T2> BarrageSubscriptionRequestBuilder<(T0, T1, T2)> {
                        /// Setter for the [`subscription_options` field](BarrageSubscriptionRequest#structfield.subscription_options).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn subscription_options<T3>(
                            self,
                            value: T3,
                        ) -> BarrageSubscriptionRequestBuilder<(T0, T1, T2, T3)>
                        where
                            T3: ::planus::WriteAsOptional<
                                ::planus::Offset<self::BarrageSubscriptionOptions>,
                            >,
                        {
                            let (v0, v1, v2) = self.0;
                            BarrageSubscriptionRequestBuilder((v0, v1, v2, value))
                        }

                        /// Sets the [`subscription_options` field](BarrageSubscriptionRequest#structfield.subscription_options) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn subscription_options_as_null(
                            self,
                        ) -> BarrageSubscriptionRequestBuilder<(T0, T1, T2, ())>
                        {
                            self.subscription_options(())
                        }
                    }

                    impl<T0, T1, T2, T3> BarrageSubscriptionRequestBuilder<(T0, T1, T2, T3)> {
                        /// Setter for the [`reverse_viewport` field](BarrageSubscriptionRequest#structfield.reverse_viewport).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn reverse_viewport<T4>(
                            self,
                            value: T4,
                        ) -> BarrageSubscriptionRequestBuilder<(T0, T1, T2, T3, T4)>
                        where
                            T4: ::planus::WriteAsDefault<bool, bool>,
                        {
                            let (v0, v1, v2, v3) = self.0;
                            BarrageSubscriptionRequestBuilder((v0, v1, v2, v3, value))
                        }

                        /// Sets the [`reverse_viewport` field](BarrageSubscriptionRequest#structfield.reverse_viewport) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn reverse_viewport_as_default(
                            self,
                        ) -> BarrageSubscriptionRequestBuilder<(
                            T0,
                            T1,
                            T2,
                            T3,
                            ::planus::DefaultValue,
                        )> {
                            self.reverse_viewport(::planus::DefaultValue)
                        }
                    }

                    impl<T0, T1, T2, T3, T4> BarrageSubscriptionRequestBuilder<(T0, T1, T2, T3, T4)> {
                        /// Setter for the [`subscription_token` field](BarrageSubscriptionRequest#structfield.subscription_token).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn subscription_token<T5>(
                            self,
                            value: T5,
                        ) -> BarrageSubscriptionRequestBuilder<(T0, T1, T2, T3, T4, T5)>
                        where
                            T5: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        {
                            let (v0, v1, v2, v3, v4) = self.0;
                            BarrageSubscriptionRequestBuilder((v0, v1, v2, v3, v4, value))
                        }

                        /// Sets the [`subscription_token` field](BarrageSubscriptionRequest#structfield.subscription_token) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn subscription_token_as_null(
                            self,
                        ) -> BarrageSubscriptionRequestBuilder<(T0, T1, T2, T3, T4, ())>
                        {
                            self.subscription_token(())
                        }
                    }

                    impl<T0, T1, T2, T3, T4, T5> BarrageSubscriptionRequestBuilder<(T0, T1, T2, T3, T4, T5)> {
                        /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [BarrageSubscriptionRequest].
                        #[inline]
                        pub fn finish(
                            self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSubscriptionRequest>
                        where
                            Self: ::planus::WriteAsOffset<BarrageSubscriptionRequest>,
                        {
                            ::planus::WriteAsOffset::prepare(&self, builder)
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T1: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T2: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T3: ::planus::WriteAsOptional<
                                ::planus::Offset<self::BarrageSubscriptionOptions>,
                            >,
                            T4: ::planus::WriteAsDefault<bool, bool>,
                            T5: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        >
                        ::planus::WriteAs<::planus::Offset<BarrageSubscriptionRequest>>
                        for BarrageSubscriptionRequestBuilder<(T0, T1, T2, T3, T4, T5)>
                    {
                        type Prepared = ::planus::Offset<BarrageSubscriptionRequest>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSubscriptionRequest> {
                            ::planus::WriteAsOffset::prepare(self, builder)
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T1: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T2: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T3: ::planus::WriteAsOptional<
                                ::planus::Offset<self::BarrageSubscriptionOptions>,
                            >,
                            T4: ::planus::WriteAsDefault<bool, bool>,
                            T5: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        >
                        ::planus::WriteAsOptional<::planus::Offset<BarrageSubscriptionRequest>>
                        for BarrageSubscriptionRequestBuilder<(T0, T1, T2, T3, T4, T5)>
                    {
                        type Prepared = ::planus::Offset<BarrageSubscriptionRequest>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<::planus::Offset<BarrageSubscriptionRequest>>
                        {
                            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(
                                self, builder,
                            ))
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T1: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T2: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T3: ::planus::WriteAsOptional<
                                ::planus::Offset<self::BarrageSubscriptionOptions>,
                            >,
                            T4: ::planus::WriteAsDefault<bool, bool>,
                            T5: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        > ::planus::WriteAsOffset<BarrageSubscriptionRequest>
                        for BarrageSubscriptionRequestBuilder<(T0, T1, T2, T3, T4, T5)>
                    {
                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSubscriptionRequest> {
                            let (v0, v1, v2, v3, v4, v5) = &self.0;
                            BarrageSubscriptionRequest::create(builder, v0, v1, v2, v3, v4, v5)
                        }
                    }

                    /// Reference to a deserialized [BarrageSubscriptionRequest].
                    #[derive(Copy, Clone)]
                    pub struct BarrageSubscriptionRequestRef<'a>(
                        #[allow(dead_code)] ::planus::table_reader::Table<'a>,
                    );

                    impl<'a> BarrageSubscriptionRequestRef<'a> {
                        /// Getter for the [`ticket` field](BarrageSubscriptionRequest#structfield.ticket).
                        #[inline]
                        pub fn ticket(&self) -> ::planus::Result<::core::option::Option<&'a [i8]>> {
                            self.0.access(0, "BarrageSubscriptionRequest", "ticket")
                        }

                        /// Getter for the [`columns` field](BarrageSubscriptionRequest#structfield.columns).
                        #[inline]
                        pub fn columns(
                            &self,
                        ) -> ::planus::Result<::core::option::Option<&'a [i8]>>
                        {
                            self.0.access(1, "BarrageSubscriptionRequest", "columns")
                        }

                        /// Getter for the [`viewport` field](BarrageSubscriptionRequest#structfield.viewport).
                        #[inline]
                        pub fn viewport(
                            &self,
                        ) -> ::planus::Result<::core::option::Option<&'a [i8]>>
                        {
                            self.0.access(2, "BarrageSubscriptionRequest", "viewport")
                        }

                        /// Getter for the [`subscription_options` field](BarrageSubscriptionRequest#structfield.subscription_options).
                        #[inline]
                        pub fn subscription_options(
                            &self,
                        ) -> ::planus::Result<
                            ::core::option::Option<self::BarrageSubscriptionOptionsRef<'a>>,
                        > {
                            self.0
                                .access(3, "BarrageSubscriptionRequest", "subscription_options")
                        }

                        /// Getter for the [`reverse_viewport` field](BarrageSubscriptionRequest#structfield.reverse_viewport).
                        #[inline]
                        pub fn reverse_viewport(&self) -> ::planus::Result<bool> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(4, "BarrageSubscriptionRequest", "reverse_viewport")?
                                    .unwrap_or(false),
                            )
                        }

                        /// Getter for the [`subscription_token` field](BarrageSubscriptionRequest#structfield.subscription_token).
                        #[inline]
                        pub fn subscription_token(
                            &self,
                        ) -> ::planus::Result<::core::option::Option<&'a [i8]>>
                        {
                            self.0
                                .access(5, "BarrageSubscriptionRequest", "subscription_token")
                        }
                    }

                    impl<'a> ::core::fmt::Debug for BarrageSubscriptionRequestRef<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            let mut f = f.debug_struct("BarrageSubscriptionRequestRef");
                            if let ::core::option::Option::Some(field_ticket) =
                                self.ticket().transpose()
                            {
                                f.field("ticket", &field_ticket);
                            }
                            if let ::core::option::Option::Some(field_columns) =
                                self.columns().transpose()
                            {
                                f.field("columns", &field_columns);
                            }
                            if let ::core::option::Option::Some(field_viewport) =
                                self.viewport().transpose()
                            {
                                f.field("viewport", &field_viewport);
                            }
                            if let ::core::option::Option::Some(field_subscription_options) =
                                self.subscription_options().transpose()
                            {
                                f.field("subscription_options", &field_subscription_options);
                            }
                            f.field("reverse_viewport", &self.reverse_viewport());
                            if let ::core::option::Option::Some(field_subscription_token) =
                                self.subscription_token().transpose()
                            {
                                f.field("subscription_token", &field_subscription_token);
                            }
                            f.finish()
                        }
                    }

                    impl<'a> ::core::convert::TryFrom<BarrageSubscriptionRequestRef<'a>>
                        for BarrageSubscriptionRequest
                    {
                        type Error = ::planus::Error;

                        #[allow(unreachable_code)]
                        fn try_from(
                            value: BarrageSubscriptionRequestRef<'a>,
                        ) -> ::planus::Result<Self> {
                            ::core::result::Result::Ok(Self {
                                ticket: value.ticket()?.map(|v| v.to_vec()),
                                columns: value.columns()?.map(|v| v.to_vec()),
                                viewport: value.viewport()?.map(|v| v.to_vec()),
                                subscription_options: if let ::core::option::Option::Some(
                                    subscription_options,
                                ) = value.subscription_options()?
                                {
                                    ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                                        ::core::convert::TryInto::try_into(subscription_options)?,
                                    ))
                                } else {
                                    ::core::option::Option::None
                                },
                                reverse_viewport: ::core::convert::TryInto::try_into(
                                    value.reverse_viewport()?,
                                )?,
                                subscription_token: value.subscription_token()?.map(|v| v.to_vec()),
                            })
                        }
                    }

                    impl<'a> ::planus::TableRead<'a> for BarrageSubscriptionRequestRef<'a> {
                        #[inline]
                        fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'a>,
                            offset: usize,
                        ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind>
                        {
                            ::core::result::Result::Ok(Self(
                                ::planus::table_reader::Table::from_buffer(buffer, offset)?,
                            ))
                        }
                    }

                    impl<'a> ::planus::VectorReadInner<'a> for BarrageSubscriptionRequestRef<'a> {
                        type Error = ::planus::Error;
                        const STRIDE: usize = 4;

                        unsafe fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'a>,
                            offset: usize,
                        ) -> ::planus::Result<Self> {
                            ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "[BarrageSubscriptionRequestRef]",
                                    "get",
                                    buffer.offset_from_start,
                                )
                            })
                        }
                    }

                    /// # Safety
                    /// The planus compiler generates implementations that initialize
                    /// the bytes in `write_values`.
                    unsafe impl ::planus::VectorWrite<::planus::Offset<BarrageSubscriptionRequest>>
                        for BarrageSubscriptionRequest
                    {
                        type Value = ::planus::Offset<BarrageSubscriptionRequest>;
                        const STRIDE: usize = 4;
                        #[inline]
                        fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                            ::planus::WriteAs::prepare(self, builder)
                        }

                        #[inline]
                        unsafe fn write_values(
                            values: &[::planus::Offset<BarrageSubscriptionRequest>],
                            bytes: *mut ::core::mem::MaybeUninit<u8>,
                            buffer_position: u32,
                        ) {
                            let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                            for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                                ::planus::WriteAsPrimitive::write(
                                    v,
                                    ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                                    buffer_position - (Self::STRIDE * i) as u32,
                                );
                            }
                        }
                    }

                    impl<'a> ::planus::ReadAsRoot<'a> for BarrageSubscriptionRequestRef<'a> {
                        fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                            ::planus::TableRead::from_buffer(
                                ::planus::SliceWithStartOffset {
                                    buffer: slice,
                                    offset_from_start: 0,
                                },
                                0,
                            )
                            .map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "[BarrageSubscriptionRequestRef]",
                                    "read_as_root",
                                    0,
                                )
                            })
                        }
                    }

                    /// The table `BarrageSnapshotOptions` in the namespace `io.deephaven.barrage.flatbuf`
                    ///
                    /// Generated from these locations:
                    /// * Table `BarrageSnapshotOptions` in the file `E:\barrage\format\Barrage.fbs:118`
                    #[derive(
                        Clone,
                        Debug,
                        PartialEq,
                        PartialOrd,
                        Eq,
                        Ord,
                        Hash,
                    )]
                    pub struct BarrageSnapshotOptions {
                        ///  Deephaven reserves a value in the range of primitives as a custom NULL value. This enables more efficient transmission
                        ///  by eliminating the additional complexity of the validity buffer.
                        pub use_deephaven_nulls: bool,
                        ///  Specify a preferred batch size. Server is allowed to be configured to restrict possible values. Too small of a
                        ///  batch size may be dominated with header costs as each batch is wrapped into a separate RecordBatch. Too large of
                        ///  a payload and it may not fit within the maximum payload size. A good default might be 4096.
                        pub batch_size: i32,
                        ///  Specify a maximum allowed message size. Server will enforce this limit by reducing batch size (to a lower limit
                        ///  of one row per batch). If the message size limit cannot be met due to large row sizes, the server will throw a
                        ///  `Status.RESOURCE_EXHAUSTED` exception
                        pub max_message_size: i32,
                        ///  The maximum length of any list / array to encode.
                        ///  - If zero, list lengths will not be limited.
                        ///  - If non-zero, the server will limit the length of any encoded list / array to the absolute value of the returned length.
                        ///  - If less than zero, the server will encode elements from the end of the list / array, rather than from the beginning.
                        ///
                        ///  Note: The server is unable to indicate when truncation occurs. To detect truncation request one more element than
                        ///  the maximum number you wish to display.
                        pub preview_list_length_limit: i64,
                    }

                    #[allow(clippy::derivable_impls)]
                    impl ::core::default::Default for BarrageSnapshotOptions {
                        fn default() -> Self {
                            Self {
                                use_deephaven_nulls: false,
                                batch_size: 0,
                                max_message_size: 0,
                                preview_list_length_limit: 0,
                            }
                        }
                    }

                    impl BarrageSnapshotOptions {
                        /// Creates a [BarrageSnapshotOptionsBuilder] for serializing an instance of this table.
                        #[inline]
                        pub fn builder() -> BarrageSnapshotOptionsBuilder<()> {
                            BarrageSnapshotOptionsBuilder(())
                        }

                        #[allow(clippy::too_many_arguments)]
                        pub fn create(
                            builder: &mut ::planus::Builder,
                            field_use_deephaven_nulls: impl ::planus::WriteAsDefault<bool, bool>,
                            field_batch_size: impl ::planus::WriteAsDefault<i32, i32>,
                            field_max_message_size: impl ::planus::WriteAsDefault<i32, i32>,
                            field_preview_list_length_limit: impl ::planus::WriteAsDefault<i64, i64>,
                        ) -> ::planus::Offset<Self> {
                            let prepared_use_deephaven_nulls =
                                field_use_deephaven_nulls.prepare(builder, &false);
                            let prepared_batch_size = field_batch_size.prepare(builder, &0);
                            let prepared_max_message_size =
                                field_max_message_size.prepare(builder, &0);
                            let prepared_preview_list_length_limit =
                                field_preview_list_length_limit.prepare(builder, &0);

                            let mut table_writer: ::planus::table_writer::TableWriter<14> =
                                ::core::default::Default::default();
                            if prepared_preview_list_length_limit.is_some() {
                                table_writer.write_entry::<i64>(4);
                            }
                            if prepared_batch_size.is_some() {
                                table_writer.write_entry::<i32>(2);
                            }
                            if prepared_max_message_size.is_some() {
                                table_writer.write_entry::<i32>(3);
                            }
                            if prepared_use_deephaven_nulls.is_some() {
                                table_writer.write_entry::<bool>(1);
                            }

                            unsafe {
                                table_writer.finish(builder, |object_writer| {
                                    if let ::core::option::Option::Some(
                                        prepared_preview_list_length_limit,
                                    ) = prepared_preview_list_length_limit
                                    {
                                        object_writer
                                            .write::<_, _, 8>(&prepared_preview_list_length_limit);
                                    }
                                    if let ::core::option::Option::Some(prepared_batch_size) =
                                        prepared_batch_size
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_batch_size);
                                    }
                                    if let ::core::option::Option::Some(prepared_max_message_size) =
                                        prepared_max_message_size
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_max_message_size);
                                    }
                                    if let ::core::option::Option::Some(
                                        prepared_use_deephaven_nulls,
                                    ) = prepared_use_deephaven_nulls
                                    {
                                        object_writer
                                            .write::<_, _, 1>(&prepared_use_deephaven_nulls);
                                    }
                                });
                            }
                            builder.current_offset()
                        }
                    }

                    impl ::planus::WriteAs<::planus::Offset<BarrageSnapshotOptions>> for BarrageSnapshotOptions {
                        type Prepared = ::planus::Offset<Self>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSnapshotOptions> {
                            ::planus::WriteAsOffset::prepare(self, builder)
                        }
                    }

                    impl ::planus::WriteAsOptional<::planus::Offset<BarrageSnapshotOptions>>
                        for BarrageSnapshotOptions
                    {
                        type Prepared = ::planus::Offset<Self>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<::planus::Offset<BarrageSnapshotOptions>>
                        {
                            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(
                                self, builder,
                            ))
                        }
                    }

                    impl ::planus::WriteAsOffset<BarrageSnapshotOptions> for BarrageSnapshotOptions {
                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSnapshotOptions> {
                            BarrageSnapshotOptions::create(
                                builder,
                                self.use_deephaven_nulls,
                                self.batch_size,
                                self.max_message_size,
                                self.preview_list_length_limit,
                            )
                        }
                    }

                    /// Builder for serializing an instance of the [BarrageSnapshotOptions] type.
                    ///
                    /// Can be created using the [BarrageSnapshotOptions::builder] method.
                    #[derive(Debug)]
                    #[must_use]
                    pub struct BarrageSnapshotOptionsBuilder<State>(State);

                    impl BarrageSnapshotOptionsBuilder<()> {
                        /// Setter for the [`use_deephaven_nulls` field](BarrageSnapshotOptions#structfield.use_deephaven_nulls).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn use_deephaven_nulls<T0>(
                            self,
                            value: T0,
                        ) -> BarrageSnapshotOptionsBuilder<(T0,)>
                        where
                            T0: ::planus::WriteAsDefault<bool, bool>,
                        {
                            BarrageSnapshotOptionsBuilder((value,))
                        }

                        /// Sets the [`use_deephaven_nulls` field](BarrageSnapshotOptions#structfield.use_deephaven_nulls) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn use_deephaven_nulls_as_default(
                            self,
                        ) -> BarrageSnapshotOptionsBuilder<(::planus::DefaultValue,)>
                        {
                            self.use_deephaven_nulls(::planus::DefaultValue)
                        }
                    }

                    impl<T0> BarrageSnapshotOptionsBuilder<(T0,)> {
                        /// Setter for the [`batch_size` field](BarrageSnapshotOptions#structfield.batch_size).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn batch_size<T1>(
                            self,
                            value: T1,
                        ) -> BarrageSnapshotOptionsBuilder<(T0, T1)>
                        where
                            T1: ::planus::WriteAsDefault<i32, i32>,
                        {
                            let (v0,) = self.0;
                            BarrageSnapshotOptionsBuilder((v0, value))
                        }

                        /// Sets the [`batch_size` field](BarrageSnapshotOptions#structfield.batch_size) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn batch_size_as_default(
                            self,
                        ) -> BarrageSnapshotOptionsBuilder<(T0, ::planus::DefaultValue)>
                        {
                            self.batch_size(::planus::DefaultValue)
                        }
                    }

                    impl<T0, T1> BarrageSnapshotOptionsBuilder<(T0, T1)> {
                        /// Setter for the [`max_message_size` field](BarrageSnapshotOptions#structfield.max_message_size).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn max_message_size<T2>(
                            self,
                            value: T2,
                        ) -> BarrageSnapshotOptionsBuilder<(T0, T1, T2)>
                        where
                            T2: ::planus::WriteAsDefault<i32, i32>,
                        {
                            let (v0, v1) = self.0;
                            BarrageSnapshotOptionsBuilder((v0, v1, value))
                        }

                        /// Sets the [`max_message_size` field](BarrageSnapshotOptions#structfield.max_message_size) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn max_message_size_as_default(
                            self,
                        ) -> BarrageSnapshotOptionsBuilder<(T0, T1, ::planus::DefaultValue)>
                        {
                            self.max_message_size(::planus::DefaultValue)
                        }
                    }

                    impl<T0, T1, T2> BarrageSnapshotOptionsBuilder<(T0, T1, T2)> {
                        /// Setter for the [`preview_list_length_limit` field](BarrageSnapshotOptions#structfield.preview_list_length_limit).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn preview_list_length_limit<T3>(
                            self,
                            value: T3,
                        ) -> BarrageSnapshotOptionsBuilder<(T0, T1, T2, T3)>
                        where
                            T3: ::planus::WriteAsDefault<i64, i64>,
                        {
                            let (v0, v1, v2) = self.0;
                            BarrageSnapshotOptionsBuilder((v0, v1, v2, value))
                        }

                        /// Sets the [`preview_list_length_limit` field](BarrageSnapshotOptions#structfield.preview_list_length_limit) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn preview_list_length_limit_as_default(
                            self,
                        ) -> BarrageSnapshotOptionsBuilder<(T0, T1, T2, ::planus::DefaultValue)>
                        {
                            self.preview_list_length_limit(::planus::DefaultValue)
                        }
                    }

                    impl<T0, T1, T2, T3> BarrageSnapshotOptionsBuilder<(T0, T1, T2, T3)> {
                        /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [BarrageSnapshotOptions].
                        #[inline]
                        pub fn finish(
                            self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSnapshotOptions>
                        where
                            Self: ::planus::WriteAsOffset<BarrageSnapshotOptions>,
                        {
                            ::planus::WriteAsOffset::prepare(&self, builder)
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsDefault<bool, bool>,
                            T1: ::planus::WriteAsDefault<i32, i32>,
                            T2: ::planus::WriteAsDefault<i32, i32>,
                            T3: ::planus::WriteAsDefault<i64, i64>,
                        >
                        ::planus::WriteAs<::planus::Offset<BarrageSnapshotOptions>>
                        for BarrageSnapshotOptionsBuilder<(T0, T1, T2, T3)>
                    {
                        type Prepared = ::planus::Offset<BarrageSnapshotOptions>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSnapshotOptions> {
                            ::planus::WriteAsOffset::prepare(self, builder)
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsDefault<bool, bool>,
                            T1: ::planus::WriteAsDefault<i32, i32>,
                            T2: ::planus::WriteAsDefault<i32, i32>,
                            T3: ::planus::WriteAsDefault<i64, i64>,
                        >
                        ::planus::WriteAsOptional<::planus::Offset<BarrageSnapshotOptions>>
                        for BarrageSnapshotOptionsBuilder<(T0, T1, T2, T3)>
                    {
                        type Prepared = ::planus::Offset<BarrageSnapshotOptions>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<::planus::Offset<BarrageSnapshotOptions>>
                        {
                            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(
                                self, builder,
                            ))
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsDefault<bool, bool>,
                            T1: ::planus::WriteAsDefault<i32, i32>,
                            T2: ::planus::WriteAsDefault<i32, i32>,
                            T3: ::planus::WriteAsDefault<i64, i64>,
                        > ::planus::WriteAsOffset<BarrageSnapshotOptions>
                        for BarrageSnapshotOptionsBuilder<(T0, T1, T2, T3)>
                    {
                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSnapshotOptions> {
                            let (v0, v1, v2, v3) = &self.0;
                            BarrageSnapshotOptions::create(builder, v0, v1, v2, v3)
                        }
                    }

                    /// Reference to a deserialized [BarrageSnapshotOptions].
                    #[derive(Copy, Clone)]
                    pub struct BarrageSnapshotOptionsRef<'a>(
                        #[allow(dead_code)] ::planus::table_reader::Table<'a>,
                    );

                    impl<'a> BarrageSnapshotOptionsRef<'a> {
                        /// Getter for the [`use_deephaven_nulls` field](BarrageSnapshotOptions#structfield.use_deephaven_nulls).
                        #[inline]
                        pub fn use_deephaven_nulls(&self) -> ::planus::Result<bool> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(1, "BarrageSnapshotOptions", "use_deephaven_nulls")?
                                    .unwrap_or(false),
                            )
                        }

                        /// Getter for the [`batch_size` field](BarrageSnapshotOptions#structfield.batch_size).
                        #[inline]
                        pub fn batch_size(&self) -> ::planus::Result<i32> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(2, "BarrageSnapshotOptions", "batch_size")?
                                    .unwrap_or(0),
                            )
                        }

                        /// Getter for the [`max_message_size` field](BarrageSnapshotOptions#structfield.max_message_size).
                        #[inline]
                        pub fn max_message_size(&self) -> ::planus::Result<i32> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(3, "BarrageSnapshotOptions", "max_message_size")?
                                    .unwrap_or(0),
                            )
                        }

                        /// Getter for the [`preview_list_length_limit` field](BarrageSnapshotOptions#structfield.preview_list_length_limit).
                        #[inline]
                        pub fn preview_list_length_limit(&self) -> ::planus::Result<i64> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(
                                        4,
                                        "BarrageSnapshotOptions",
                                        "preview_list_length_limit",
                                    )?
                                    .unwrap_or(0),
                            )
                        }
                    }

                    impl<'a> ::core::fmt::Debug for BarrageSnapshotOptionsRef<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            let mut f = f.debug_struct("BarrageSnapshotOptionsRef");
                            f.field("use_deephaven_nulls", &self.use_deephaven_nulls());
                            f.field("batch_size", &self.batch_size());
                            f.field("max_message_size", &self.max_message_size());
                            f.field(
                                "preview_list_length_limit",
                                &self.preview_list_length_limit(),
                            );
                            f.finish()
                        }
                    }

                    impl<'a> ::core::convert::TryFrom<BarrageSnapshotOptionsRef<'a>> for BarrageSnapshotOptions {
                        type Error = ::planus::Error;

                        #[allow(unreachable_code)]
                        fn try_from(
                            value: BarrageSnapshotOptionsRef<'a>,
                        ) -> ::planus::Result<Self> {
                            ::core::result::Result::Ok(Self {
                                use_deephaven_nulls: ::core::convert::TryInto::try_into(
                                    value.use_deephaven_nulls()?,
                                )?,
                                batch_size: ::core::convert::TryInto::try_into(
                                    value.batch_size()?,
                                )?,
                                max_message_size: ::core::convert::TryInto::try_into(
                                    value.max_message_size()?,
                                )?,
                                preview_list_length_limit: ::core::convert::TryInto::try_into(
                                    value.preview_list_length_limit()?,
                                )?,
                            })
                        }
                    }

                    impl<'a> ::planus::TableRead<'a> for BarrageSnapshotOptionsRef<'a> {
                        #[inline]
                        fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'a>,
                            offset: usize,
                        ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind>
                        {
                            ::core::result::Result::Ok(Self(
                                ::planus::table_reader::Table::from_buffer(buffer, offset)?,
                            ))
                        }
                    }

                    impl<'a> ::planus::VectorReadInner<'a> for BarrageSnapshotOptionsRef<'a> {
                        type Error = ::planus::Error;
                        const STRIDE: usize = 4;

                        unsafe fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'a>,
                            offset: usize,
                        ) -> ::planus::Result<Self> {
                            ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "[BarrageSnapshotOptionsRef]",
                                    "get",
                                    buffer.offset_from_start,
                                )
                            })
                        }
                    }

                    /// # Safety
                    /// The planus compiler generates implementations that initialize
                    /// the bytes in `write_values`.
                    unsafe impl ::planus::VectorWrite<::planus::Offset<BarrageSnapshotOptions>>
                        for BarrageSnapshotOptions
                    {
                        type Value = ::planus::Offset<BarrageSnapshotOptions>;
                        const STRIDE: usize = 4;
                        #[inline]
                        fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                            ::planus::WriteAs::prepare(self, builder)
                        }

                        #[inline]
                        unsafe fn write_values(
                            values: &[::planus::Offset<BarrageSnapshotOptions>],
                            bytes: *mut ::core::mem::MaybeUninit<u8>,
                            buffer_position: u32,
                        ) {
                            let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                            for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                                ::planus::WriteAsPrimitive::write(
                                    v,
                                    ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                                    buffer_position - (Self::STRIDE * i) as u32,
                                );
                            }
                        }
                    }

                    impl<'a> ::planus::ReadAsRoot<'a> for BarrageSnapshotOptionsRef<'a> {
                        fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                            ::planus::TableRead::from_buffer(
                                ::planus::SliceWithStartOffset {
                                    buffer: slice,
                                    offset_from_start: 0,
                                },
                                0,
                            )
                            .map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "[BarrageSnapshotOptionsRef]",
                                    "read_as_root",
                                    0,
                                )
                            })
                        }
                    }

                    ///  Describes the snapshot the client would like to acquire.
                    ///
                    /// Generated from these locations:
                    /// * Table `BarrageSnapshotRequest` in the file `E:\barrage\format\Barrage.fbs:147`
                    #[derive(
                        Clone,
                        Debug,
                        PartialEq,
                        PartialOrd,
                        Eq,
                        Ord,
                        Hash,
                    )]
                    pub struct BarrageSnapshotRequest {
                        ///  Ticket for the source data set.
                        pub ticket: ::core::option::Option<::planus::alloc::vec::Vec<i8>>,
                        ///  The bitset of columns to request. If not provided then all columns are requested.
                        pub columns: ::core::option::Option<::planus::alloc::vec::Vec<i8>>,
                        ///  This is an encoded and compressed RowSet in position-space to subscribe to. If not provided then the entire
                        ///  table is requested.
                        pub viewport: ::core::option::Option<::planus::alloc::vec::Vec<i8>>,
                        ///  Options to configure your subscription.
                        pub snapshot_options: ::core::option::Option<
                            ::planus::alloc::boxed::Box<self::BarrageSnapshotOptions>,
                        >,
                        ///  When this is set the viewport RowSet will be inverted against the length of the table. That is to say
                        ///  every position value is converted from `i` to `n - i - 1` if the table has `n` rows.
                        pub reverse_viewport: bool,
                    }

                    #[allow(clippy::derivable_impls)]
                    impl ::core::default::Default for BarrageSnapshotRequest {
                        fn default() -> Self {
                            Self {
                                ticket: ::core::default::Default::default(),
                                columns: ::core::default::Default::default(),
                                viewport: ::core::default::Default::default(),
                                snapshot_options: ::core::default::Default::default(),
                                reverse_viewport: false,
                            }
                        }
                    }

                    impl BarrageSnapshotRequest {
                        /// Creates a [BarrageSnapshotRequestBuilder] for serializing an instance of this table.
                        #[inline]
                        pub fn builder() -> BarrageSnapshotRequestBuilder<()> {
                            BarrageSnapshotRequestBuilder(())
                        }

                        #[allow(clippy::too_many_arguments)]
                        pub fn create(
                            builder: &mut ::planus::Builder,
                            field_ticket: impl ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            field_columns: impl ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            field_viewport: impl ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            field_snapshot_options: impl ::planus::WriteAsOptional<
                                ::planus::Offset<self::BarrageSnapshotOptions>,
                            >,
                            field_reverse_viewport: impl ::planus::WriteAsDefault<bool, bool>,
                        ) -> ::planus::Offset<Self> {
                            let prepared_ticket = field_ticket.prepare(builder);
                            let prepared_columns = field_columns.prepare(builder);
                            let prepared_viewport = field_viewport.prepare(builder);
                            let prepared_snapshot_options = field_snapshot_options.prepare(builder);
                            let prepared_reverse_viewport =
                                field_reverse_viewport.prepare(builder, &false);

                            let mut table_writer: ::planus::table_writer::TableWriter<14> =
                                ::core::default::Default::default();
                            if prepared_ticket.is_some() {
                                table_writer.write_entry::<::planus::Offset<[i8]>>(0);
                            }
                            if prepared_columns.is_some() {
                                table_writer.write_entry::<::planus::Offset<[i8]>>(1);
                            }
                            if prepared_viewport.is_some() {
                                table_writer.write_entry::<::planus::Offset<[i8]>>(2);
                            }
                            if prepared_snapshot_options.is_some() {
                                table_writer
                                    .write_entry::<::planus::Offset<self::BarrageSnapshotOptions>>(
                                        3,
                                    );
                            }
                            if prepared_reverse_viewport.is_some() {
                                table_writer.write_entry::<bool>(4);
                            }

                            unsafe {
                                table_writer.finish(builder, |object_writer| {
                                    if let ::core::option::Option::Some(prepared_ticket) =
                                        prepared_ticket
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_ticket);
                                    }
                                    if let ::core::option::Option::Some(prepared_columns) =
                                        prepared_columns
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_columns);
                                    }
                                    if let ::core::option::Option::Some(prepared_viewport) =
                                        prepared_viewport
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_viewport);
                                    }
                                    if let ::core::option::Option::Some(prepared_snapshot_options) =
                                        prepared_snapshot_options
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_snapshot_options);
                                    }
                                    if let ::core::option::Option::Some(prepared_reverse_viewport) =
                                        prepared_reverse_viewport
                                    {
                                        object_writer.write::<_, _, 1>(&prepared_reverse_viewport);
                                    }
                                });
                            }
                            builder.current_offset()
                        }
                    }

                    impl ::planus::WriteAs<::planus::Offset<BarrageSnapshotRequest>> for BarrageSnapshotRequest {
                        type Prepared = ::planus::Offset<Self>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSnapshotRequest> {
                            ::planus::WriteAsOffset::prepare(self, builder)
                        }
                    }

                    impl ::planus::WriteAsOptional<::planus::Offset<BarrageSnapshotRequest>>
                        for BarrageSnapshotRequest
                    {
                        type Prepared = ::planus::Offset<Self>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<::planus::Offset<BarrageSnapshotRequest>>
                        {
                            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(
                                self, builder,
                            ))
                        }
                    }

                    impl ::planus::WriteAsOffset<BarrageSnapshotRequest> for BarrageSnapshotRequest {
                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSnapshotRequest> {
                            BarrageSnapshotRequest::create(
                                builder,
                                &self.ticket,
                                &self.columns,
                                &self.viewport,
                                &self.snapshot_options,
                                self.reverse_viewport,
                            )
                        }
                    }

                    /// Builder for serializing an instance of the [BarrageSnapshotRequest] type.
                    ///
                    /// Can be created using the [BarrageSnapshotRequest::builder] method.
                    #[derive(Debug)]
                    #[must_use]
                    pub struct BarrageSnapshotRequestBuilder<State>(State);

                    impl BarrageSnapshotRequestBuilder<()> {
                        /// Setter for the [`ticket` field](BarrageSnapshotRequest#structfield.ticket).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn ticket<T0>(self, value: T0) -> BarrageSnapshotRequestBuilder<(T0,)>
                        where
                            T0: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        {
                            BarrageSnapshotRequestBuilder((value,))
                        }

                        /// Sets the [`ticket` field](BarrageSnapshotRequest#structfield.ticket) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn ticket_as_null(self) -> BarrageSnapshotRequestBuilder<((),)> {
                            self.ticket(())
                        }
                    }

                    impl<T0> BarrageSnapshotRequestBuilder<(T0,)> {
                        /// Setter for the [`columns` field](BarrageSnapshotRequest#structfield.columns).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn columns<T1>(
                            self,
                            value: T1,
                        ) -> BarrageSnapshotRequestBuilder<(T0, T1)>
                        where
                            T1: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        {
                            let (v0,) = self.0;
                            BarrageSnapshotRequestBuilder((v0, value))
                        }

                        /// Sets the [`columns` field](BarrageSnapshotRequest#structfield.columns) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn columns_as_null(self) -> BarrageSnapshotRequestBuilder<(T0, ())> {
                            self.columns(())
                        }
                    }

                    impl<T0, T1> BarrageSnapshotRequestBuilder<(T0, T1)> {
                        /// Setter for the [`viewport` field](BarrageSnapshotRequest#structfield.viewport).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn viewport<T2>(
                            self,
                            value: T2,
                        ) -> BarrageSnapshotRequestBuilder<(T0, T1, T2)>
                        where
                            T2: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        {
                            let (v0, v1) = self.0;
                            BarrageSnapshotRequestBuilder((v0, v1, value))
                        }

                        /// Sets the [`viewport` field](BarrageSnapshotRequest#structfield.viewport) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn viewport_as_null(
                            self,
                        ) -> BarrageSnapshotRequestBuilder<(T0, T1, ())> {
                            self.viewport(())
                        }
                    }

                    impl<T0, T1, T2> BarrageSnapshotRequestBuilder<(T0, T1, T2)> {
                        /// Setter for the [`snapshot_options` field](BarrageSnapshotRequest#structfield.snapshot_options).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn snapshot_options<T3>(
                            self,
                            value: T3,
                        ) -> BarrageSnapshotRequestBuilder<(T0, T1, T2, T3)>
                        where
                            T3: ::planus::WriteAsOptional<
                                ::planus::Offset<self::BarrageSnapshotOptions>,
                            >,
                        {
                            let (v0, v1, v2) = self.0;
                            BarrageSnapshotRequestBuilder((v0, v1, v2, value))
                        }

                        /// Sets the [`snapshot_options` field](BarrageSnapshotRequest#structfield.snapshot_options) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn snapshot_options_as_null(
                            self,
                        ) -> BarrageSnapshotRequestBuilder<(T0, T1, T2, ())>
                        {
                            self.snapshot_options(())
                        }
                    }

                    impl<T0, T1, T2, T3> BarrageSnapshotRequestBuilder<(T0, T1, T2, T3)> {
                        /// Setter for the [`reverse_viewport` field](BarrageSnapshotRequest#structfield.reverse_viewport).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn reverse_viewport<T4>(
                            self,
                            value: T4,
                        ) -> BarrageSnapshotRequestBuilder<(T0, T1, T2, T3, T4)>
                        where
                            T4: ::planus::WriteAsDefault<bool, bool>,
                        {
                            let (v0, v1, v2, v3) = self.0;
                            BarrageSnapshotRequestBuilder((v0, v1, v2, v3, value))
                        }

                        /// Sets the [`reverse_viewport` field](BarrageSnapshotRequest#structfield.reverse_viewport) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn reverse_viewport_as_default(
                            self,
                        ) -> BarrageSnapshotRequestBuilder<(T0, T1, T2, T3, ::planus::DefaultValue)>
                        {
                            self.reverse_viewport(::planus::DefaultValue)
                        }
                    }

                    impl<T0, T1, T2, T3, T4> BarrageSnapshotRequestBuilder<(T0, T1, T2, T3, T4)> {
                        /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [BarrageSnapshotRequest].
                        #[inline]
                        pub fn finish(
                            self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSnapshotRequest>
                        where
                            Self: ::planus::WriteAsOffset<BarrageSnapshotRequest>,
                        {
                            ::planus::WriteAsOffset::prepare(&self, builder)
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T1: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T2: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T3: ::planus::WriteAsOptional<
                                ::planus::Offset<self::BarrageSnapshotOptions>,
                            >,
                            T4: ::planus::WriteAsDefault<bool, bool>,
                        >
                        ::planus::WriteAs<::planus::Offset<BarrageSnapshotRequest>>
                        for BarrageSnapshotRequestBuilder<(T0, T1, T2, T3, T4)>
                    {
                        type Prepared = ::planus::Offset<BarrageSnapshotRequest>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSnapshotRequest> {
                            ::planus::WriteAsOffset::prepare(self, builder)
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T1: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T2: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T3: ::planus::WriteAsOptional<
                                ::planus::Offset<self::BarrageSnapshotOptions>,
                            >,
                            T4: ::planus::WriteAsDefault<bool, bool>,
                        >
                        ::planus::WriteAsOptional<::planus::Offset<BarrageSnapshotRequest>>
                        for BarrageSnapshotRequestBuilder<(T0, T1, T2, T3, T4)>
                    {
                        type Prepared = ::planus::Offset<BarrageSnapshotRequest>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<::planus::Offset<BarrageSnapshotRequest>>
                        {
                            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(
                                self, builder,
                            ))
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T1: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T2: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T3: ::planus::WriteAsOptional<
                                ::planus::Offset<self::BarrageSnapshotOptions>,
                            >,
                            T4: ::planus::WriteAsDefault<bool, bool>,
                        > ::planus::WriteAsOffset<BarrageSnapshotRequest>
                        for BarrageSnapshotRequestBuilder<(T0, T1, T2, T3, T4)>
                    {
                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageSnapshotRequest> {
                            let (v0, v1, v2, v3, v4) = &self.0;
                            BarrageSnapshotRequest::create(builder, v0, v1, v2, v3, v4)
                        }
                    }

                    /// Reference to a deserialized [BarrageSnapshotRequest].
                    #[derive(Copy, Clone)]
                    pub struct BarrageSnapshotRequestRef<'a>(
                        #[allow(dead_code)] ::planus::table_reader::Table<'a>,
                    );

                    impl<'a> BarrageSnapshotRequestRef<'a> {
                        /// Getter for the [`ticket` field](BarrageSnapshotRequest#structfield.ticket).
                        #[inline]
                        pub fn ticket(&self) -> ::planus::Result<::core::option::Option<&'a [i8]>> {
                            self.0.access(0, "BarrageSnapshotRequest", "ticket")
                        }

                        /// Getter for the [`columns` field](BarrageSnapshotRequest#structfield.columns).
                        #[inline]
                        pub fn columns(
                            &self,
                        ) -> ::planus::Result<::core::option::Option<&'a [i8]>>
                        {
                            self.0.access(1, "BarrageSnapshotRequest", "columns")
                        }

                        /// Getter for the [`viewport` field](BarrageSnapshotRequest#structfield.viewport).
                        #[inline]
                        pub fn viewport(
                            &self,
                        ) -> ::planus::Result<::core::option::Option<&'a [i8]>>
                        {
                            self.0.access(2, "BarrageSnapshotRequest", "viewport")
                        }

                        /// Getter for the [`snapshot_options` field](BarrageSnapshotRequest#structfield.snapshot_options).
                        #[inline]
                        pub fn snapshot_options(
                            &self,
                        ) -> ::planus::Result<
                            ::core::option::Option<self::BarrageSnapshotOptionsRef<'a>>,
                        > {
                            self.0
                                .access(3, "BarrageSnapshotRequest", "snapshot_options")
                        }

                        /// Getter for the [`reverse_viewport` field](BarrageSnapshotRequest#structfield.reverse_viewport).
                        #[inline]
                        pub fn reverse_viewport(&self) -> ::planus::Result<bool> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(4, "BarrageSnapshotRequest", "reverse_viewport")?
                                    .unwrap_or(false),
                            )
                        }
                    }

                    impl<'a> ::core::fmt::Debug for BarrageSnapshotRequestRef<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            let mut f = f.debug_struct("BarrageSnapshotRequestRef");
                            if let ::core::option::Option::Some(field_ticket) =
                                self.ticket().transpose()
                            {
                                f.field("ticket", &field_ticket);
                            }
                            if let ::core::option::Option::Some(field_columns) =
                                self.columns().transpose()
                            {
                                f.field("columns", &field_columns);
                            }
                            if let ::core::option::Option::Some(field_viewport) =
                                self.viewport().transpose()
                            {
                                f.field("viewport", &field_viewport);
                            }
                            if let ::core::option::Option::Some(field_snapshot_options) =
                                self.snapshot_options().transpose()
                            {
                                f.field("snapshot_options", &field_snapshot_options);
                            }
                            f.field("reverse_viewport", &self.reverse_viewport());
                            f.finish()
                        }
                    }

                    impl<'a> ::core::convert::TryFrom<BarrageSnapshotRequestRef<'a>> for BarrageSnapshotRequest {
                        type Error = ::planus::Error;

                        #[allow(unreachable_code)]
                        fn try_from(
                            value: BarrageSnapshotRequestRef<'a>,
                        ) -> ::planus::Result<Self> {
                            ::core::result::Result::Ok(Self {
                                ticket: value.ticket()?.map(|v| v.to_vec()),
                                columns: value.columns()?.map(|v| v.to_vec()),
                                viewport: value.viewport()?.map(|v| v.to_vec()),
                                snapshot_options: if let ::core::option::Option::Some(
                                    snapshot_options,
                                ) = value.snapshot_options()?
                                {
                                    ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                                        ::core::convert::TryInto::try_into(snapshot_options)?,
                                    ))
                                } else {
                                    ::core::option::Option::None
                                },
                                reverse_viewport: ::core::convert::TryInto::try_into(
                                    value.reverse_viewport()?,
                                )?,
                            })
                        }
                    }

                    impl<'a> ::planus::TableRead<'a> for BarrageSnapshotRequestRef<'a> {
                        #[inline]
                        fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'a>,
                            offset: usize,
                        ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind>
                        {
                            ::core::result::Result::Ok(Self(
                                ::planus::table_reader::Table::from_buffer(buffer, offset)?,
                            ))
                        }
                    }

                    impl<'a> ::planus::VectorReadInner<'a> for BarrageSnapshotRequestRef<'a> {
                        type Error = ::planus::Error;
                        const STRIDE: usize = 4;

                        unsafe fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'a>,
                            offset: usize,
                        ) -> ::planus::Result<Self> {
                            ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "[BarrageSnapshotRequestRef]",
                                    "get",
                                    buffer.offset_from_start,
                                )
                            })
                        }
                    }

                    /// # Safety
                    /// The planus compiler generates implementations that initialize
                    /// the bytes in `write_values`.
                    unsafe impl ::planus::VectorWrite<::planus::Offset<BarrageSnapshotRequest>>
                        for BarrageSnapshotRequest
                    {
                        type Value = ::planus::Offset<BarrageSnapshotRequest>;
                        const STRIDE: usize = 4;
                        #[inline]
                        fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                            ::planus::WriteAs::prepare(self, builder)
                        }

                        #[inline]
                        unsafe fn write_values(
                            values: &[::planus::Offset<BarrageSnapshotRequest>],
                            bytes: *mut ::core::mem::MaybeUninit<u8>,
                            buffer_position: u32,
                        ) {
                            let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                            for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                                ::planus::WriteAsPrimitive::write(
                                    v,
                                    ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                                    buffer_position - (Self::STRIDE * i) as u32,
                                );
                            }
                        }
                    }

                    impl<'a> ::planus::ReadAsRoot<'a> for BarrageSnapshotRequestRef<'a> {
                        fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                            ::planus::TableRead::from_buffer(
                                ::planus::SliceWithStartOffset {
                                    buffer: slice,
                                    offset_from_start: 0,
                                },
                                0,
                            )
                            .map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "[BarrageSnapshotRequestRef]",
                                    "read_as_root",
                                    0,
                                )
                            })
                        }
                    }

                    /// The table `BarragePublicationOptions` in the namespace `io.deephaven.barrage.flatbuf`
                    ///
                    /// Generated from these locations:
                    /// * Table `BarragePublicationOptions` in the file `E:\barrage\format\Barrage.fbs:166`
                    #[derive(
                        Clone,
                        Debug,
                        PartialEq,
                        PartialOrd,
                        Eq,
                        Ord,
                        Hash,
                    )]
                    pub struct BarragePublicationOptions {
                        ///  Deephaven reserves a value in the range of primitives as a custom NULL value. This enables more efficient transmission
                        ///  by eliminating the additional complexity of the validity buffer.
                        pub use_deephaven_nulls: bool,
                    }

                    #[allow(clippy::derivable_impls)]
                    impl ::core::default::Default for BarragePublicationOptions {
                        fn default() -> Self {
                            Self {
                                use_deephaven_nulls: false,
                            }
                        }
                    }

                    impl BarragePublicationOptions {
                        /// Creates a [BarragePublicationOptionsBuilder] for serializing an instance of this table.
                        #[inline]
                        pub fn builder() -> BarragePublicationOptionsBuilder<()> {
                            BarragePublicationOptionsBuilder(())
                        }

                        #[allow(clippy::too_many_arguments)]
                        pub fn create(
                            builder: &mut ::planus::Builder,
                            field_use_deephaven_nulls: impl ::planus::WriteAsDefault<bool, bool>,
                        ) -> ::planus::Offset<Self> {
                            let prepared_use_deephaven_nulls =
                                field_use_deephaven_nulls.prepare(builder, &false);

                            let mut table_writer: ::planus::table_writer::TableWriter<6> =
                                ::core::default::Default::default();
                            if prepared_use_deephaven_nulls.is_some() {
                                table_writer.write_entry::<bool>(0);
                            }

                            unsafe {
                                table_writer.finish(builder, |object_writer| {
                                    if let ::core::option::Option::Some(
                                        prepared_use_deephaven_nulls,
                                    ) = prepared_use_deephaven_nulls
                                    {
                                        object_writer
                                            .write::<_, _, 1>(&prepared_use_deephaven_nulls);
                                    }
                                });
                            }
                            builder.current_offset()
                        }
                    }

                    impl ::planus::WriteAs<::planus::Offset<BarragePublicationOptions>> for BarragePublicationOptions {
                        type Prepared = ::planus::Offset<Self>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarragePublicationOptions> {
                            ::planus::WriteAsOffset::prepare(self, builder)
                        }
                    }

                    impl ::planus::WriteAsOptional<::planus::Offset<BarragePublicationOptions>>
                        for BarragePublicationOptions
                    {
                        type Prepared = ::planus::Offset<Self>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<::planus::Offset<BarragePublicationOptions>>
                        {
                            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(
                                self, builder,
                            ))
                        }
                    }

                    impl ::planus::WriteAsOffset<BarragePublicationOptions> for BarragePublicationOptions {
                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarragePublicationOptions> {
                            BarragePublicationOptions::create(builder, self.use_deephaven_nulls)
                        }
                    }

                    /// Builder for serializing an instance of the [BarragePublicationOptions] type.
                    ///
                    /// Can be created using the [BarragePublicationOptions::builder] method.
                    #[derive(Debug)]
                    #[must_use]
                    pub struct BarragePublicationOptionsBuilder<State>(State);

                    impl BarragePublicationOptionsBuilder<()> {
                        /// Setter for the [`use_deephaven_nulls` field](BarragePublicationOptions#structfield.use_deephaven_nulls).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn use_deephaven_nulls<T0>(
                            self,
                            value: T0,
                        ) -> BarragePublicationOptionsBuilder<(T0,)>
                        where
                            T0: ::planus::WriteAsDefault<bool, bool>,
                        {
                            BarragePublicationOptionsBuilder((value,))
                        }

                        /// Sets the [`use_deephaven_nulls` field](BarragePublicationOptions#structfield.use_deephaven_nulls) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn use_deephaven_nulls_as_default(
                            self,
                        ) -> BarragePublicationOptionsBuilder<(::planus::DefaultValue,)>
                        {
                            self.use_deephaven_nulls(::planus::DefaultValue)
                        }
                    }

                    impl<T0> BarragePublicationOptionsBuilder<(T0,)> {
                        /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [BarragePublicationOptions].
                        #[inline]
                        pub fn finish(
                            self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarragePublicationOptions>
                        where
                            Self: ::planus::WriteAsOffset<BarragePublicationOptions>,
                        {
                            ::planus::WriteAsOffset::prepare(&self, builder)
                        }
                    }

                    impl<T0: ::planus::WriteAsDefault<bool, bool>>
                        ::planus::WriteAs<::planus::Offset<BarragePublicationOptions>>
                        for BarragePublicationOptionsBuilder<(T0,)>
                    {
                        type Prepared = ::planus::Offset<BarragePublicationOptions>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarragePublicationOptions> {
                            ::planus::WriteAsOffset::prepare(self, builder)
                        }
                    }

                    impl<T0: ::planus::WriteAsDefault<bool, bool>>
                        ::planus::WriteAsOptional<::planus::Offset<BarragePublicationOptions>>
                        for BarragePublicationOptionsBuilder<(T0,)>
                    {
                        type Prepared = ::planus::Offset<BarragePublicationOptions>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<::planus::Offset<BarragePublicationOptions>>
                        {
                            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(
                                self, builder,
                            ))
                        }
                    }

                    impl<T0: ::planus::WriteAsDefault<bool, bool>>
                        ::planus::WriteAsOffset<BarragePublicationOptions>
                        for BarragePublicationOptionsBuilder<(T0,)>
                    {
                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarragePublicationOptions> {
                            let (v0,) = &self.0;
                            BarragePublicationOptions::create(builder, v0)
                        }
                    }

                    /// Reference to a deserialized [BarragePublicationOptions].
                    #[derive(Copy, Clone)]
                    pub struct BarragePublicationOptionsRef<'a>(
                        #[allow(dead_code)] ::planus::table_reader::Table<'a>,
                    );

                    impl<'a> BarragePublicationOptionsRef<'a> {
                        /// Getter for the [`use_deephaven_nulls` field](BarragePublicationOptions#structfield.use_deephaven_nulls).
                        #[inline]
                        pub fn use_deephaven_nulls(&self) -> ::planus::Result<bool> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(0, "BarragePublicationOptions", "use_deephaven_nulls")?
                                    .unwrap_or(false),
                            )
                        }
                    }

                    impl<'a> ::core::fmt::Debug for BarragePublicationOptionsRef<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            let mut f = f.debug_struct("BarragePublicationOptionsRef");
                            f.field("use_deephaven_nulls", &self.use_deephaven_nulls());
                            f.finish()
                        }
                    }

                    impl<'a> ::core::convert::TryFrom<BarragePublicationOptionsRef<'a>> for BarragePublicationOptions {
                        type Error = ::planus::Error;

                        #[allow(unreachable_code)]
                        fn try_from(
                            value: BarragePublicationOptionsRef<'a>,
                        ) -> ::planus::Result<Self> {
                            ::core::result::Result::Ok(Self {
                                use_deephaven_nulls: ::core::convert::TryInto::try_into(
                                    value.use_deephaven_nulls()?,
                                )?,
                            })
                        }
                    }

                    impl<'a> ::planus::TableRead<'a> for BarragePublicationOptionsRef<'a> {
                        #[inline]
                        fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'a>,
                            offset: usize,
                        ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind>
                        {
                            ::core::result::Result::Ok(Self(
                                ::planus::table_reader::Table::from_buffer(buffer, offset)?,
                            ))
                        }
                    }

                    impl<'a> ::planus::VectorReadInner<'a> for BarragePublicationOptionsRef<'a> {
                        type Error = ::planus::Error;
                        const STRIDE: usize = 4;

                        unsafe fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'a>,
                            offset: usize,
                        ) -> ::planus::Result<Self> {
                            ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "[BarragePublicationOptionsRef]",
                                    "get",
                                    buffer.offset_from_start,
                                )
                            })
                        }
                    }

                    /// # Safety
                    /// The planus compiler generates implementations that initialize
                    /// the bytes in `write_values`.
                    unsafe impl ::planus::VectorWrite<::planus::Offset<BarragePublicationOptions>>
                        for BarragePublicationOptions
                    {
                        type Value = ::planus::Offset<BarragePublicationOptions>;
                        const STRIDE: usize = 4;
                        #[inline]
                        fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                            ::planus::WriteAs::prepare(self, builder)
                        }

                        #[inline]
                        unsafe fn write_values(
                            values: &[::planus::Offset<BarragePublicationOptions>],
                            bytes: *mut ::core::mem::MaybeUninit<u8>,
                            buffer_position: u32,
                        ) {
                            let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                            for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                                ::planus::WriteAsPrimitive::write(
                                    v,
                                    ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                                    buffer_position - (Self::STRIDE * i) as u32,
                                );
                            }
                        }
                    }

                    impl<'a> ::planus::ReadAsRoot<'a> for BarragePublicationOptionsRef<'a> {
                        fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                            ::planus::TableRead::from_buffer(
                                ::planus::SliceWithStartOffset {
                                    buffer: slice,
                                    offset_from_start: 0,
                                },
                                0,
                            )
                            .map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "[BarragePublicationOptionsRef]",
                                    "read_as_root",
                                    0,
                                )
                            })
                        }
                    }

                    ///  Describes the table update stream the client would like to push to. This is similar to a DoPut but the client
                    ///  will send BarrageUpdateMetadata to explicitly describe the row key space. The updates sent adhere to the table
                    ///  update model semantics; thus BarragePublication enables the client to upload a ticking table.
                    ///
                    /// Generated from these locations:
                    /// * Table `BarragePublicationRequest` in the file `E:\barrage\format\Barrage.fbs:175`
                    #[derive(
                        Clone,
                        Debug,
                        PartialEq,
                        PartialOrd,
                        Eq,
                        Ord,
                        Hash,
                    )]
                    pub struct BarragePublicationRequest {
                        ///  The destination Ticket.
                        pub ticket: ::core::option::Option<::planus::alloc::vec::Vec<i8>>,
                        ///  Options to configure your request.
                        pub publish_options: ::core::option::Option<
                            ::planus::alloc::boxed::Box<self::BarragePublicationOptions>,
                        >,
                    }

                    #[allow(clippy::derivable_impls)]
                    impl ::core::default::Default for BarragePublicationRequest {
                        fn default() -> Self {
                            Self {
                                ticket: ::core::default::Default::default(),
                                publish_options: ::core::default::Default::default(),
                            }
                        }
                    }

                    impl BarragePublicationRequest {
                        /// Creates a [BarragePublicationRequestBuilder] for serializing an instance of this table.
                        #[inline]
                        pub fn builder() -> BarragePublicationRequestBuilder<()> {
                            BarragePublicationRequestBuilder(())
                        }

                        #[allow(clippy::too_many_arguments)]
                        pub fn create(
                            builder: &mut ::planus::Builder,
                            field_ticket: impl ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            field_publish_options: impl ::planus::WriteAsOptional<
                                ::planus::Offset<self::BarragePublicationOptions>,
                            >,
                        ) -> ::planus::Offset<Self> {
                            let prepared_ticket = field_ticket.prepare(builder);
                            let prepared_publish_options = field_publish_options.prepare(builder);

                            let mut table_writer: ::planus::table_writer::TableWriter<8> =
                                ::core::default::Default::default();
                            if prepared_ticket.is_some() {
                                table_writer.write_entry::<::planus::Offset<[i8]>>(0);
                            }
                            if prepared_publish_options.is_some() {
                                table_writer.write_entry::<::planus::Offset<self::BarragePublicationOptions>>(1);
                            }

                            unsafe {
                                table_writer.finish(builder, |object_writer| {
                                    if let ::core::option::Option::Some(prepared_ticket) =
                                        prepared_ticket
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_ticket);
                                    }
                                    if let ::core::option::Option::Some(prepared_publish_options) =
                                        prepared_publish_options
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_publish_options);
                                    }
                                });
                            }
                            builder.current_offset()
                        }
                    }

                    impl ::planus::WriteAs<::planus::Offset<BarragePublicationRequest>> for BarragePublicationRequest {
                        type Prepared = ::planus::Offset<Self>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarragePublicationRequest> {
                            ::planus::WriteAsOffset::prepare(self, builder)
                        }
                    }

                    impl ::planus::WriteAsOptional<::planus::Offset<BarragePublicationRequest>>
                        for BarragePublicationRequest
                    {
                        type Prepared = ::planus::Offset<Self>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<::planus::Offset<BarragePublicationRequest>>
                        {
                            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(
                                self, builder,
                            ))
                        }
                    }

                    impl ::planus::WriteAsOffset<BarragePublicationRequest> for BarragePublicationRequest {
                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarragePublicationRequest> {
                            BarragePublicationRequest::create(
                                builder,
                                &self.ticket,
                                &self.publish_options,
                            )
                        }
                    }

                    /// Builder for serializing an instance of the [BarragePublicationRequest] type.
                    ///
                    /// Can be created using the [BarragePublicationRequest::builder] method.
                    #[derive(Debug)]
                    #[must_use]
                    pub struct BarragePublicationRequestBuilder<State>(State);

                    impl BarragePublicationRequestBuilder<()> {
                        /// Setter for the [`ticket` field](BarragePublicationRequest#structfield.ticket).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn ticket<T0>(
                            self,
                            value: T0,
                        ) -> BarragePublicationRequestBuilder<(T0,)>
                        where
                            T0: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        {
                            BarragePublicationRequestBuilder((value,))
                        }

                        /// Sets the [`ticket` field](BarragePublicationRequest#structfield.ticket) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn ticket_as_null(self) -> BarragePublicationRequestBuilder<((),)> {
                            self.ticket(())
                        }
                    }

                    impl<T0> BarragePublicationRequestBuilder<(T0,)> {
                        /// Setter for the [`publish_options` field](BarragePublicationRequest#structfield.publish_options).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn publish_options<T1>(
                            self,
                            value: T1,
                        ) -> BarragePublicationRequestBuilder<(T0, T1)>
                        where
                            T1: ::planus::WriteAsOptional<
                                ::planus::Offset<self::BarragePublicationOptions>,
                            >,
                        {
                            let (v0,) = self.0;
                            BarragePublicationRequestBuilder((v0, value))
                        }

                        /// Sets the [`publish_options` field](BarragePublicationRequest#structfield.publish_options) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn publish_options_as_null(
                            self,
                        ) -> BarragePublicationRequestBuilder<(T0, ())> {
                            self.publish_options(())
                        }
                    }

                    impl<T0, T1> BarragePublicationRequestBuilder<(T0, T1)> {
                        /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [BarragePublicationRequest].
                        #[inline]
                        pub fn finish(
                            self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarragePublicationRequest>
                        where
                            Self: ::planus::WriteAsOffset<BarragePublicationRequest>,
                        {
                            ::planus::WriteAsOffset::prepare(&self, builder)
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T1: ::planus::WriteAsOptional<
                                ::planus::Offset<self::BarragePublicationOptions>,
                            >,
                        >
                        ::planus::WriteAs<::planus::Offset<BarragePublicationRequest>>
                        for BarragePublicationRequestBuilder<(T0, T1)>
                    {
                        type Prepared = ::planus::Offset<BarragePublicationRequest>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarragePublicationRequest> {
                            ::planus::WriteAsOffset::prepare(self, builder)
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T1: ::planus::WriteAsOptional<
                                ::planus::Offset<self::BarragePublicationOptions>,
                            >,
                        >
                        ::planus::WriteAsOptional<::planus::Offset<BarragePublicationRequest>>
                        for BarragePublicationRequestBuilder<(T0, T1)>
                    {
                        type Prepared = ::planus::Offset<BarragePublicationRequest>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<::planus::Offset<BarragePublicationRequest>>
                        {
                            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(
                                self, builder,
                            ))
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T1: ::planus::WriteAsOptional<
                                ::planus::Offset<self::BarragePublicationOptions>,
                            >,
                        > ::planus::WriteAsOffset<BarragePublicationRequest>
                        for BarragePublicationRequestBuilder<(T0, T1)>
                    {
                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarragePublicationRequest> {
                            let (v0, v1) = &self.0;
                            BarragePublicationRequest::create(builder, v0, v1)
                        }
                    }

                    /// Reference to a deserialized [BarragePublicationRequest].
                    #[derive(Copy, Clone)]
                    pub struct BarragePublicationRequestRef<'a>(
                        #[allow(dead_code)] ::planus::table_reader::Table<'a>,
                    );

                    impl<'a> BarragePublicationRequestRef<'a> {
                        /// Getter for the [`ticket` field](BarragePublicationRequest#structfield.ticket).
                        #[inline]
                        pub fn ticket(&self) -> ::planus::Result<::core::option::Option<&'a [i8]>> {
                            self.0.access(0, "BarragePublicationRequest", "ticket")
                        }

                        /// Getter for the [`publish_options` field](BarragePublicationRequest#structfield.publish_options).
                        #[inline]
                        pub fn publish_options(
                            &self,
                        ) -> ::planus::Result<
                            ::core::option::Option<self::BarragePublicationOptionsRef<'a>>,
                        > {
                            self.0
                                .access(1, "BarragePublicationRequest", "publish_options")
                        }
                    }

                    impl<'a> ::core::fmt::Debug for BarragePublicationRequestRef<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            let mut f = f.debug_struct("BarragePublicationRequestRef");
                            if let ::core::option::Option::Some(field_ticket) =
                                self.ticket().transpose()
                            {
                                f.field("ticket", &field_ticket);
                            }
                            if let ::core::option::Option::Some(field_publish_options) =
                                self.publish_options().transpose()
                            {
                                f.field("publish_options", &field_publish_options);
                            }
                            f.finish()
                        }
                    }

                    impl<'a> ::core::convert::TryFrom<BarragePublicationRequestRef<'a>> for BarragePublicationRequest {
                        type Error = ::planus::Error;

                        #[allow(unreachable_code)]
                        fn try_from(
                            value: BarragePublicationRequestRef<'a>,
                        ) -> ::planus::Result<Self> {
                            ::core::result::Result::Ok(Self {
                                ticket: value.ticket()?.map(|v| v.to_vec()),
                                publish_options: if let ::core::option::Option::Some(
                                    publish_options,
                                ) = value.publish_options()?
                                {
                                    ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                                        ::core::convert::TryInto::try_into(publish_options)?,
                                    ))
                                } else {
                                    ::core::option::Option::None
                                },
                            })
                        }
                    }

                    impl<'a> ::planus::TableRead<'a> for BarragePublicationRequestRef<'a> {
                        #[inline]
                        fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'a>,
                            offset: usize,
                        ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind>
                        {
                            ::core::result::Result::Ok(Self(
                                ::planus::table_reader::Table::from_buffer(buffer, offset)?,
                            ))
                        }
                    }

                    impl<'a> ::planus::VectorReadInner<'a> for BarragePublicationRequestRef<'a> {
                        type Error = ::planus::Error;
                        const STRIDE: usize = 4;

                        unsafe fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'a>,
                            offset: usize,
                        ) -> ::planus::Result<Self> {
                            ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "[BarragePublicationRequestRef]",
                                    "get",
                                    buffer.offset_from_start,
                                )
                            })
                        }
                    }

                    /// # Safety
                    /// The planus compiler generates implementations that initialize
                    /// the bytes in `write_values`.
                    unsafe impl ::planus::VectorWrite<::planus::Offset<BarragePublicationRequest>>
                        for BarragePublicationRequest
                    {
                        type Value = ::planus::Offset<BarragePublicationRequest>;
                        const STRIDE: usize = 4;
                        #[inline]
                        fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                            ::planus::WriteAs::prepare(self, builder)
                        }

                        #[inline]
                        unsafe fn write_values(
                            values: &[::planus::Offset<BarragePublicationRequest>],
                            bytes: *mut ::core::mem::MaybeUninit<u8>,
                            buffer_position: u32,
                        ) {
                            let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                            for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                                ::planus::WriteAsPrimitive::write(
                                    v,
                                    ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                                    buffer_position - (Self::STRIDE * i) as u32,
                                );
                            }
                        }
                    }

                    impl<'a> ::planus::ReadAsRoot<'a> for BarragePublicationRequestRef<'a> {
                        fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                            ::planus::TableRead::from_buffer(
                                ::planus::SliceWithStartOffset {
                                    buffer: slice,
                                    offset_from_start: 0,
                                },
                                0,
                            )
                            .map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "[BarragePublicationRequestRef]",
                                    "read_as_root",
                                    0,
                                )
                            })
                        }
                    }

                    ///  Holds all of the rowset data structures for the column being modified.
                    ///
                    /// Generated from these locations:
                    /// * Table `BarrageModColumnMetadata` in the file `E:\barrage\format\Barrage.fbs:184`
                    #[derive(
                        Clone,
                        Debug,
                        PartialEq,
                        PartialOrd,
                        Eq,
                        Ord,
                        Hash,
                    )]
                    pub struct BarrageModColumnMetadata {
                        ///  This is an encoded and compressed RowSet for this column (within the viewport) that were modified.
                        ///  There is no notification for modifications outside of the viewport.
                        pub modified_rows: ::core::option::Option<::planus::alloc::vec::Vec<i8>>,
                    }

                    #[allow(clippy::derivable_impls)]
                    impl ::core::default::Default for BarrageModColumnMetadata {
                        fn default() -> Self {
                            Self {
                                modified_rows: ::core::default::Default::default(),
                            }
                        }
                    }

                    impl BarrageModColumnMetadata {
                        /// Creates a [BarrageModColumnMetadataBuilder] for serializing an instance of this table.
                        #[inline]
                        pub fn builder() -> BarrageModColumnMetadataBuilder<()> {
                            BarrageModColumnMetadataBuilder(())
                        }

                        #[allow(clippy::too_many_arguments)]
                        pub fn create(
                            builder: &mut ::planus::Builder,
                            field_modified_rows: impl ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        ) -> ::planus::Offset<Self> {
                            let prepared_modified_rows = field_modified_rows.prepare(builder);

                            let mut table_writer: ::planus::table_writer::TableWriter<6> =
                                ::core::default::Default::default();
                            if prepared_modified_rows.is_some() {
                                table_writer.write_entry::<::planus::Offset<[i8]>>(0);
                            }

                            unsafe {
                                table_writer.finish(builder, |object_writer| {
                                    if let ::core::option::Option::Some(prepared_modified_rows) =
                                        prepared_modified_rows
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_modified_rows);
                                    }
                                });
                            }
                            builder.current_offset()
                        }
                    }

                    impl ::planus::WriteAs<::planus::Offset<BarrageModColumnMetadata>> for BarrageModColumnMetadata {
                        type Prepared = ::planus::Offset<Self>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageModColumnMetadata> {
                            ::planus::WriteAsOffset::prepare(self, builder)
                        }
                    }

                    impl ::planus::WriteAsOptional<::planus::Offset<BarrageModColumnMetadata>>
                        for BarrageModColumnMetadata
                    {
                        type Prepared = ::planus::Offset<Self>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<::planus::Offset<BarrageModColumnMetadata>>
                        {
                            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(
                                self, builder,
                            ))
                        }
                    }

                    impl ::planus::WriteAsOffset<BarrageModColumnMetadata> for BarrageModColumnMetadata {
                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageModColumnMetadata> {
                            BarrageModColumnMetadata::create(builder, &self.modified_rows)
                        }
                    }

                    /// Builder for serializing an instance of the [BarrageModColumnMetadata] type.
                    ///
                    /// Can be created using the [BarrageModColumnMetadata::builder] method.
                    #[derive(Debug)]
                    #[must_use]
                    pub struct BarrageModColumnMetadataBuilder<State>(State);

                    impl BarrageModColumnMetadataBuilder<()> {
                        /// Setter for the [`modified_rows` field](BarrageModColumnMetadata#structfield.modified_rows).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn modified_rows<T0>(
                            self,
                            value: T0,
                        ) -> BarrageModColumnMetadataBuilder<(T0,)>
                        where
                            T0: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        {
                            BarrageModColumnMetadataBuilder((value,))
                        }

                        /// Sets the [`modified_rows` field](BarrageModColumnMetadata#structfield.modified_rows) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn modified_rows_as_null(
                            self,
                        ) -> BarrageModColumnMetadataBuilder<((),)> {
                            self.modified_rows(())
                        }
                    }

                    impl<T0> BarrageModColumnMetadataBuilder<(T0,)> {
                        /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [BarrageModColumnMetadata].
                        #[inline]
                        pub fn finish(
                            self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageModColumnMetadata>
                        where
                            Self: ::planus::WriteAsOffset<BarrageModColumnMetadata>,
                        {
                            ::planus::WriteAsOffset::prepare(&self, builder)
                        }
                    }

                    impl<T0: ::planus::WriteAsOptional<::planus::Offset<[i8]>>>
                        ::planus::WriteAs<::planus::Offset<BarrageModColumnMetadata>>
                        for BarrageModColumnMetadataBuilder<(T0,)>
                    {
                        type Prepared = ::planus::Offset<BarrageModColumnMetadata>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageModColumnMetadata> {
                            ::planus::WriteAsOffset::prepare(self, builder)
                        }
                    }

                    impl<T0: ::planus::WriteAsOptional<::planus::Offset<[i8]>>>
                        ::planus::WriteAsOptional<::planus::Offset<BarrageModColumnMetadata>>
                        for BarrageModColumnMetadataBuilder<(T0,)>
                    {
                        type Prepared = ::planus::Offset<BarrageModColumnMetadata>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<::planus::Offset<BarrageModColumnMetadata>>
                        {
                            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(
                                self, builder,
                            ))
                        }
                    }

                    impl<T0: ::planus::WriteAsOptional<::planus::Offset<[i8]>>>
                        ::planus::WriteAsOffset<BarrageModColumnMetadata>
                        for BarrageModColumnMetadataBuilder<(T0,)>
                    {
                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageModColumnMetadata> {
                            let (v0,) = &self.0;
                            BarrageModColumnMetadata::create(builder, v0)
                        }
                    }

                    /// Reference to a deserialized [BarrageModColumnMetadata].
                    #[derive(Copy, Clone)]
                    pub struct BarrageModColumnMetadataRef<'a>(
                        #[allow(dead_code)] ::planus::table_reader::Table<'a>,
                    );

                    impl<'a> BarrageModColumnMetadataRef<'a> {
                        /// Getter for the [`modified_rows` field](BarrageModColumnMetadata#structfield.modified_rows).
                        #[inline]
                        pub fn modified_rows(
                            &self,
                        ) -> ::planus::Result<::core::option::Option<&'a [i8]>>
                        {
                            self.0
                                .access(0, "BarrageModColumnMetadata", "modified_rows")
                        }
                    }

                    impl<'a> ::core::fmt::Debug for BarrageModColumnMetadataRef<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            let mut f = f.debug_struct("BarrageModColumnMetadataRef");
                            if let ::core::option::Option::Some(field_modified_rows) =
                                self.modified_rows().transpose()
                            {
                                f.field("modified_rows", &field_modified_rows);
                            }
                            f.finish()
                        }
                    }

                    impl<'a> ::core::convert::TryFrom<BarrageModColumnMetadataRef<'a>> for BarrageModColumnMetadata {
                        type Error = ::planus::Error;

                        #[allow(unreachable_code)]
                        fn try_from(
                            value: BarrageModColumnMetadataRef<'a>,
                        ) -> ::planus::Result<Self> {
                            ::core::result::Result::Ok(Self {
                                modified_rows: value.modified_rows()?.map(|v| v.to_vec()),
                            })
                        }
                    }

                    impl<'a> ::planus::TableRead<'a> for BarrageModColumnMetadataRef<'a> {
                        #[inline]
                        fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'a>,
                            offset: usize,
                        ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind>
                        {
                            ::core::result::Result::Ok(Self(
                                ::planus::table_reader::Table::from_buffer(buffer, offset)?,
                            ))
                        }
                    }

                    impl<'a> ::planus::VectorReadInner<'a> for BarrageModColumnMetadataRef<'a> {
                        type Error = ::planus::Error;
                        const STRIDE: usize = 4;

                        unsafe fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'a>,
                            offset: usize,
                        ) -> ::planus::Result<Self> {
                            ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "[BarrageModColumnMetadataRef]",
                                    "get",
                                    buffer.offset_from_start,
                                )
                            })
                        }
                    }

                    /// # Safety
                    /// The planus compiler generates implementations that initialize
                    /// the bytes in `write_values`.
                    unsafe impl ::planus::VectorWrite<::planus::Offset<BarrageModColumnMetadata>>
                        for BarrageModColumnMetadata
                    {
                        type Value = ::planus::Offset<BarrageModColumnMetadata>;
                        const STRIDE: usize = 4;
                        #[inline]
                        fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                            ::planus::WriteAs::prepare(self, builder)
                        }

                        #[inline]
                        unsafe fn write_values(
                            values: &[::planus::Offset<BarrageModColumnMetadata>],
                            bytes: *mut ::core::mem::MaybeUninit<u8>,
                            buffer_position: u32,
                        ) {
                            let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                            for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                                ::planus::WriteAsPrimitive::write(
                                    v,
                                    ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                                    buffer_position - (Self::STRIDE * i) as u32,
                                );
                            }
                        }
                    }

                    impl<'a> ::planus::ReadAsRoot<'a> for BarrageModColumnMetadataRef<'a> {
                        fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                            ::planus::TableRead::from_buffer(
                                ::planus::SliceWithStartOffset {
                                    buffer: slice,
                                    offset_from_start: 0,
                                },
                                0,
                            )
                            .map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "[BarrageModColumnMetadataRef]",
                                    "read_as_root",
                                    0,
                                )
                            })
                        }
                    }

                    ///  A data header describing the shared memory layout of a "record" or "row"
                    ///  batch for a ticking barrage table.
                    ///
                    /// Generated from these locations:
                    /// * Table `BarrageUpdateMetadata` in the file `E:\barrage\format\Barrage.fbs:192`
                    #[derive(
                        Clone,
                        Debug,
                        PartialEq,
                        PartialOrd,
                        Eq,
                        Ord,
                        Hash,
                    )]
                    pub struct BarrageUpdateMetadata {
                        ///  This batch is generated from an upstream table that ticks independently of the stream. If
                        ///  multiple events are coalesced into one update, the server may communicate that here for
                        ///  informational purposes.
                        pub first_seq: i64,
                        /// The field `last_seq` in the table `BarrageUpdateMetadata`
                        pub last_seq: i64,
                        ///  Indicates if this message was sent due to upstream ticks or due to a subscription change.
                        pub is_snapshot: bool,
                        ///  If this is a snapshot and the subscription is a viewport, then the effectively subscribed viewport
                        ///  will be included in the payload. It is an encoded and compressed RowSet.
                        pub effective_viewport:
                            ::core::option::Option<::planus::alloc::vec::Vec<i8>>,
                        ///  When this is set the viewport RowSet will be inverted against the length of the table. That is to say
                        ///  every position value is converted from `i` to `n - i - 1` if the table has `n` rows.
                        pub effective_reverse_viewport: bool,
                        ///  If this is a snapshot, then the effectively subscribed column set will be included in the payload.
                        pub effective_column_set:
                            ::core::option::Option<::planus::alloc::vec::Vec<i8>>,
                        ///  This is an encoded and compressed RowSet that was added in this update.
                        pub added_rows: ::core::option::Option<::planus::alloc::vec::Vec<i8>>,
                        ///  This is an encoded and compressed RowSet that was removed in this update.
                        pub removed_rows: ::core::option::Option<::planus::alloc::vec::Vec<i8>>,
                        ///  This is an encoded and compressed RowSetShiftData describing how the keyspace of unmodified rows changed.
                        pub shift_data: ::core::option::Option<::planus::alloc::vec::Vec<i8>>,
                        ///  This is an encoded and compressed RowSet that was included with this update.
                        ///  (the server may include rows not in addedRows if this is a viewport subscription to refresh
                        ///   unmodified rows that were scoped into view)
                        pub added_rows_included:
                            ::core::option::Option<::planus::alloc::vec::Vec<i8>>,
                        ///  The list of modified column data are in the same order as the field nodes on the schema.
                        pub mod_column_nodes: ::core::option::Option<
                            ::planus::alloc::vec::Vec<self::BarrageModColumnMetadata>,
                        >,
                        ///  The current size of the table.
                        pub table_size: i64,
                    }

                    #[allow(clippy::derivable_impls)]
                    impl ::core::default::Default for BarrageUpdateMetadata {
                        fn default() -> Self {
                            Self {
                                first_seq: 0,
                                last_seq: 0,
                                is_snapshot: false,
                                effective_viewport: ::core::default::Default::default(),
                                effective_reverse_viewport: false,
                                effective_column_set: ::core::default::Default::default(),
                                added_rows: ::core::default::Default::default(),
                                removed_rows: ::core::default::Default::default(),
                                shift_data: ::core::default::Default::default(),
                                added_rows_included: ::core::default::Default::default(),
                                mod_column_nodes: ::core::default::Default::default(),
                                table_size: 0,
                            }
                        }
                    }

                    impl BarrageUpdateMetadata {
                        /// Creates a [BarrageUpdateMetadataBuilder] for serializing an instance of this table.
                        #[inline]
                        pub fn builder() -> BarrageUpdateMetadataBuilder<()> {
                            BarrageUpdateMetadataBuilder(())
                        }

                        #[allow(clippy::too_many_arguments)]
                        pub fn create(
                            builder: &mut ::planus::Builder,
                            field_first_seq: impl ::planus::WriteAsDefault<i64, i64>,
                            field_last_seq: impl ::planus::WriteAsDefault<i64, i64>,
                            field_is_snapshot: impl ::planus::WriteAsDefault<bool, bool>,
                            field_effective_viewport: impl ::planus::WriteAsOptional<
                                ::planus::Offset<[i8]>,
                            >,
                            field_effective_reverse_viewport: impl ::planus::WriteAsDefault<bool, bool>,
                            field_effective_column_set: impl ::planus::WriteAsOptional<
                                ::planus::Offset<[i8]>,
                            >,
                            field_added_rows: impl ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            field_removed_rows: impl ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            field_shift_data: impl ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            field_added_rows_included: impl ::planus::WriteAsOptional<
                                ::planus::Offset<[i8]>,
                            >,
                            field_mod_column_nodes: impl ::planus::WriteAsOptional<
                                ::planus::Offset<
                                    [::planus::Offset<self::BarrageModColumnMetadata>],
                                >,
                            >,
                            field_table_size: impl ::planus::WriteAsDefault<i64, i64>,
                        ) -> ::planus::Offset<Self> {
                            let prepared_first_seq = field_first_seq.prepare(builder, &0);
                            let prepared_last_seq = field_last_seq.prepare(builder, &0);
                            let prepared_is_snapshot = field_is_snapshot.prepare(builder, &false);
                            let prepared_effective_viewport =
                                field_effective_viewport.prepare(builder);
                            let prepared_effective_reverse_viewport =
                                field_effective_reverse_viewport.prepare(builder, &false);
                            let prepared_effective_column_set =
                                field_effective_column_set.prepare(builder);
                            let prepared_added_rows = field_added_rows.prepare(builder);
                            let prepared_removed_rows = field_removed_rows.prepare(builder);
                            let prepared_shift_data = field_shift_data.prepare(builder);
                            let prepared_added_rows_included =
                                field_added_rows_included.prepare(builder);
                            let prepared_mod_column_nodes = field_mod_column_nodes.prepare(builder);
                            let prepared_table_size = field_table_size.prepare(builder, &0);

                            let mut table_writer: ::planus::table_writer::TableWriter<28> =
                                ::core::default::Default::default();
                            if prepared_first_seq.is_some() {
                                table_writer.write_entry::<i64>(0);
                            }
                            if prepared_last_seq.is_some() {
                                table_writer.write_entry::<i64>(1);
                            }
                            if prepared_table_size.is_some() {
                                table_writer.write_entry::<i64>(11);
                            }
                            if prepared_effective_viewport.is_some() {
                                table_writer.write_entry::<::planus::Offset<[i8]>>(3);
                            }
                            if prepared_effective_column_set.is_some() {
                                table_writer.write_entry::<::planus::Offset<[i8]>>(5);
                            }
                            if prepared_added_rows.is_some() {
                                table_writer.write_entry::<::planus::Offset<[i8]>>(6);
                            }
                            if prepared_removed_rows.is_some() {
                                table_writer.write_entry::<::planus::Offset<[i8]>>(7);
                            }
                            if prepared_shift_data.is_some() {
                                table_writer.write_entry::<::planus::Offset<[i8]>>(8);
                            }
                            if prepared_added_rows_included.is_some() {
                                table_writer.write_entry::<::planus::Offset<[i8]>>(9);
                            }
                            if prepared_mod_column_nodes.is_some() {
                                table_writer.write_entry::<::planus::Offset<
                                    [::planus::Offset<self::BarrageModColumnMetadata>],
                                >>(10);
                            }
                            if prepared_is_snapshot.is_some() {
                                table_writer.write_entry::<bool>(2);
                            }
                            if prepared_effective_reverse_viewport.is_some() {
                                table_writer.write_entry::<bool>(4);
                            }

                            unsafe {
                                table_writer.finish(builder, |object_writer| {
                                    if let ::core::option::Option::Some(prepared_first_seq) =
                                        prepared_first_seq
                                    {
                                        object_writer.write::<_, _, 8>(&prepared_first_seq);
                                    }
                                    if let ::core::option::Option::Some(prepared_last_seq) =
                                        prepared_last_seq
                                    {
                                        object_writer.write::<_, _, 8>(&prepared_last_seq);
                                    }
                                    if let ::core::option::Option::Some(prepared_table_size) =
                                        prepared_table_size
                                    {
                                        object_writer.write::<_, _, 8>(&prepared_table_size);
                                    }
                                    if let ::core::option::Option::Some(
                                        prepared_effective_viewport,
                                    ) = prepared_effective_viewport
                                    {
                                        object_writer
                                            .write::<_, _, 4>(&prepared_effective_viewport);
                                    }
                                    if let ::core::option::Option::Some(
                                        prepared_effective_column_set,
                                    ) = prepared_effective_column_set
                                    {
                                        object_writer
                                            .write::<_, _, 4>(&prepared_effective_column_set);
                                    }
                                    if let ::core::option::Option::Some(prepared_added_rows) =
                                        prepared_added_rows
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_added_rows);
                                    }
                                    if let ::core::option::Option::Some(prepared_removed_rows) =
                                        prepared_removed_rows
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_removed_rows);
                                    }
                                    if let ::core::option::Option::Some(prepared_shift_data) =
                                        prepared_shift_data
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_shift_data);
                                    }
                                    if let ::core::option::Option::Some(
                                        prepared_added_rows_included,
                                    ) = prepared_added_rows_included
                                    {
                                        object_writer
                                            .write::<_, _, 4>(&prepared_added_rows_included);
                                    }
                                    if let ::core::option::Option::Some(prepared_mod_column_nodes) =
                                        prepared_mod_column_nodes
                                    {
                                        object_writer.write::<_, _, 4>(&prepared_mod_column_nodes);
                                    }
                                    if let ::core::option::Option::Some(prepared_is_snapshot) =
                                        prepared_is_snapshot
                                    {
                                        object_writer.write::<_, _, 1>(&prepared_is_snapshot);
                                    }
                                    if let ::core::option::Option::Some(
                                        prepared_effective_reverse_viewport,
                                    ) = prepared_effective_reverse_viewport
                                    {
                                        object_writer
                                            .write::<_, _, 1>(&prepared_effective_reverse_viewport);
                                    }
                                });
                            }
                            builder.current_offset()
                        }
                    }

                    impl ::planus::WriteAs<::planus::Offset<BarrageUpdateMetadata>> for BarrageUpdateMetadata {
                        type Prepared = ::planus::Offset<Self>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageUpdateMetadata> {
                            ::planus::WriteAsOffset::prepare(self, builder)
                        }
                    }

                    impl ::planus::WriteAsOptional<::planus::Offset<BarrageUpdateMetadata>> for BarrageUpdateMetadata {
                        type Prepared = ::planus::Offset<Self>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<::planus::Offset<BarrageUpdateMetadata>>
                        {
                            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(
                                self, builder,
                            ))
                        }
                    }

                    impl ::planus::WriteAsOffset<BarrageUpdateMetadata> for BarrageUpdateMetadata {
                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageUpdateMetadata> {
                            BarrageUpdateMetadata::create(
                                builder,
                                self.first_seq,
                                self.last_seq,
                                self.is_snapshot,
                                &self.effective_viewport,
                                self.effective_reverse_viewport,
                                &self.effective_column_set,
                                &self.added_rows,
                                &self.removed_rows,
                                &self.shift_data,
                                &self.added_rows_included,
                                &self.mod_column_nodes,
                                self.table_size,
                            )
                        }
                    }

                    /// Builder for serializing an instance of the [BarrageUpdateMetadata] type.
                    ///
                    /// Can be created using the [BarrageUpdateMetadata::builder] method.
                    #[derive(Debug)]
                    #[must_use]
                    pub struct BarrageUpdateMetadataBuilder<State>(State);

                    impl BarrageUpdateMetadataBuilder<()> {
                        /// Setter for the [`first_seq` field](BarrageUpdateMetadata#structfield.first_seq).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn first_seq<T0>(self, value: T0) -> BarrageUpdateMetadataBuilder<(T0,)>
                        where
                            T0: ::planus::WriteAsDefault<i64, i64>,
                        {
                            BarrageUpdateMetadataBuilder((value,))
                        }

                        /// Sets the [`first_seq` field](BarrageUpdateMetadata#structfield.first_seq) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn first_seq_as_default(
                            self,
                        ) -> BarrageUpdateMetadataBuilder<(::planus::DefaultValue,)>
                        {
                            self.first_seq(::planus::DefaultValue)
                        }
                    }

                    impl<T0> BarrageUpdateMetadataBuilder<(T0,)> {
                        /// Setter for the [`last_seq` field](BarrageUpdateMetadata#structfield.last_seq).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn last_seq<T1>(
                            self,
                            value: T1,
                        ) -> BarrageUpdateMetadataBuilder<(T0, T1)>
                        where
                            T1: ::planus::WriteAsDefault<i64, i64>,
                        {
                            let (v0,) = self.0;
                            BarrageUpdateMetadataBuilder((v0, value))
                        }

                        /// Sets the [`last_seq` field](BarrageUpdateMetadata#structfield.last_seq) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn last_seq_as_default(
                            self,
                        ) -> BarrageUpdateMetadataBuilder<(T0, ::planus::DefaultValue)>
                        {
                            self.last_seq(::planus::DefaultValue)
                        }
                    }

                    impl<T0, T1> BarrageUpdateMetadataBuilder<(T0, T1)> {
                        /// Setter for the [`is_snapshot` field](BarrageUpdateMetadata#structfield.is_snapshot).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn is_snapshot<T2>(
                            self,
                            value: T2,
                        ) -> BarrageUpdateMetadataBuilder<(T0, T1, T2)>
                        where
                            T2: ::planus::WriteAsDefault<bool, bool>,
                        {
                            let (v0, v1) = self.0;
                            BarrageUpdateMetadataBuilder((v0, v1, value))
                        }

                        /// Sets the [`is_snapshot` field](BarrageUpdateMetadata#structfield.is_snapshot) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn is_snapshot_as_default(
                            self,
                        ) -> BarrageUpdateMetadataBuilder<(T0, T1, ::planus::DefaultValue)>
                        {
                            self.is_snapshot(::planus::DefaultValue)
                        }
                    }

                    impl<T0, T1, T2> BarrageUpdateMetadataBuilder<(T0, T1, T2)> {
                        /// Setter for the [`effective_viewport` field](BarrageUpdateMetadata#structfield.effective_viewport).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn effective_viewport<T3>(
                            self,
                            value: T3,
                        ) -> BarrageUpdateMetadataBuilder<(T0, T1, T2, T3)>
                        where
                            T3: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        {
                            let (v0, v1, v2) = self.0;
                            BarrageUpdateMetadataBuilder((v0, v1, v2, value))
                        }

                        /// Sets the [`effective_viewport` field](BarrageUpdateMetadata#structfield.effective_viewport) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn effective_viewport_as_null(
                            self,
                        ) -> BarrageUpdateMetadataBuilder<(T0, T1, T2, ())>
                        {
                            self.effective_viewport(())
                        }
                    }

                    impl<T0, T1, T2, T3> BarrageUpdateMetadataBuilder<(T0, T1, T2, T3)> {
                        /// Setter for the [`effective_reverse_viewport` field](BarrageUpdateMetadata#structfield.effective_reverse_viewport).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn effective_reverse_viewport<T4>(
                            self,
                            value: T4,
                        ) -> BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, T4)>
                        where
                            T4: ::planus::WriteAsDefault<bool, bool>,
                        {
                            let (v0, v1, v2, v3) = self.0;
                            BarrageUpdateMetadataBuilder((v0, v1, v2, v3, value))
                        }

                        /// Sets the [`effective_reverse_viewport` field](BarrageUpdateMetadata#structfield.effective_reverse_viewport) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn effective_reverse_viewport_as_default(
                            self,
                        ) -> BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, ::planus::DefaultValue)>
                        {
                            self.effective_reverse_viewport(::planus::DefaultValue)
                        }
                    }

                    impl<T0, T1, T2, T3, T4> BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, T4)> {
                        /// Setter for the [`effective_column_set` field](BarrageUpdateMetadata#structfield.effective_column_set).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn effective_column_set<T5>(
                            self,
                            value: T5,
                        ) -> BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, T4, T5)>
                        where
                            T5: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        {
                            let (v0, v1, v2, v3, v4) = self.0;
                            BarrageUpdateMetadataBuilder((v0, v1, v2, v3, v4, value))
                        }

                        /// Sets the [`effective_column_set` field](BarrageUpdateMetadata#structfield.effective_column_set) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn effective_column_set_as_null(
                            self,
                        ) -> BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, T4, ())>
                        {
                            self.effective_column_set(())
                        }
                    }

                    impl<T0, T1, T2, T3, T4, T5> BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, T4, T5)> {
                        /// Setter for the [`added_rows` field](BarrageUpdateMetadata#structfield.added_rows).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn added_rows<T6>(
                            self,
                            value: T6,
                        ) -> BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, T4, T5, T6)>
                        where
                            T6: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        {
                            let (v0, v1, v2, v3, v4, v5) = self.0;
                            BarrageUpdateMetadataBuilder((v0, v1, v2, v3, v4, v5, value))
                        }

                        /// Sets the [`added_rows` field](BarrageUpdateMetadata#structfield.added_rows) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn added_rows_as_null(
                            self,
                        ) -> BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, T4, T5, ())>
                        {
                            self.added_rows(())
                        }
                    }

                    impl<T0, T1, T2, T3, T4, T5, T6> BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, T4, T5, T6)> {
                        /// Setter for the [`removed_rows` field](BarrageUpdateMetadata#structfield.removed_rows).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn removed_rows<T7>(
                            self,
                            value: T7,
                        ) -> BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, T4, T5, T6, T7)>
                        where
                            T7: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        {
                            let (v0, v1, v2, v3, v4, v5, v6) = self.0;
                            BarrageUpdateMetadataBuilder((v0, v1, v2, v3, v4, v5, v6, value))
                        }

                        /// Sets the [`removed_rows` field](BarrageUpdateMetadata#structfield.removed_rows) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn removed_rows_as_null(
                            self,
                        ) -> BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, T4, T5, T6, ())>
                        {
                            self.removed_rows(())
                        }
                    }

                    impl<T0, T1, T2, T3, T4, T5, T6, T7>
                        BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, T4, T5, T6, T7)>
                    {
                        /// Setter for the [`shift_data` field](BarrageUpdateMetadata#structfield.shift_data).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn shift_data<T8>(
                            self,
                            value: T8,
                        ) -> BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>
                        where
                            T8: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        {
                            let (v0, v1, v2, v3, v4, v5, v6, v7) = self.0;
                            BarrageUpdateMetadataBuilder((v0, v1, v2, v3, v4, v5, v6, v7, value))
                        }

                        /// Sets the [`shift_data` field](BarrageUpdateMetadata#structfield.shift_data) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn shift_data_as_null(
                            self,
                        ) -> BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, ())>
                        {
                            self.shift_data(())
                        }
                    }

                    impl<T0, T1, T2, T3, T4, T5, T6, T7, T8>
                        BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>
                    {
                        /// Setter for the [`added_rows_included` field](BarrageUpdateMetadata#structfield.added_rows_included).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn added_rows_included<T9>(
                            self,
                            value: T9,
                        ) -> BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>
                        where
                            T9: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                        {
                            let (v0, v1, v2, v3, v4, v5, v6, v7, v8) = self.0;
                            BarrageUpdateMetadataBuilder((
                                v0, v1, v2, v3, v4, v5, v6, v7, v8, value,
                            ))
                        }

                        /// Sets the [`added_rows_included` field](BarrageUpdateMetadata#structfield.added_rows_included) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn added_rows_included_as_null(
                            self,
                        ) -> BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, ())>
                        {
                            self.added_rows_included(())
                        }
                    }

                    impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>
                        BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>
                    {
                        /// Setter for the [`mod_column_nodes` field](BarrageUpdateMetadata#structfield.mod_column_nodes).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn mod_column_nodes<T10>(
                            self,
                            value: T10,
                        ) -> BarrageUpdateMetadataBuilder<(
                            T0,
                            T1,
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                        )>
                        where
                            T10: ::planus::WriteAsOptional<
                                ::planus::Offset<
                                    [::planus::Offset<self::BarrageModColumnMetadata>],
                                >,
                            >,
                        {
                            let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = self.0;
                            BarrageUpdateMetadataBuilder((
                                v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, value,
                            ))
                        }

                        /// Sets the [`mod_column_nodes` field](BarrageUpdateMetadata#structfield.mod_column_nodes) to null.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn mod_column_nodes_as_null(
                            self,
                        ) -> BarrageUpdateMetadataBuilder<(
                            T0,
                            T1,
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            (),
                        )> {
                            self.mod_column_nodes(())
                        }
                    }

                    impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
                        BarrageUpdateMetadataBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>
                    {
                        /// Setter for the [`table_size` field](BarrageUpdateMetadata#structfield.table_size).
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn table_size<T11>(
                            self,
                            value: T11,
                        ) -> BarrageUpdateMetadataBuilder<(
                            T0,
                            T1,
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            T11,
                        )>
                        where
                            T11: ::planus::WriteAsDefault<i64, i64>,
                        {
                            let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10) = self.0;
                            BarrageUpdateMetadataBuilder((
                                v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, value,
                            ))
                        }

                        /// Sets the [`table_size` field](BarrageUpdateMetadata#structfield.table_size) to the default value.
                        #[inline]
                        #[allow(clippy::type_complexity)]
                        pub fn table_size_as_default(
                            self,
                        ) -> BarrageUpdateMetadataBuilder<(
                            T0,
                            T1,
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            ::planus::DefaultValue,
                        )> {
                            self.table_size(::planus::DefaultValue)
                        }
                    }

                    impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
                        BarrageUpdateMetadataBuilder<(
                            T0,
                            T1,
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            T11,
                        )>
                    {
                        /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [BarrageUpdateMetadata].
                        #[inline]
                        pub fn finish(
                            self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageUpdateMetadata>
                        where
                            Self: ::planus::WriteAsOffset<BarrageUpdateMetadata>,
                        {
                            ::planus::WriteAsOffset::prepare(&self, builder)
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsDefault<i64, i64>,
                            T1: ::planus::WriteAsDefault<i64, i64>,
                            T2: ::planus::WriteAsDefault<bool, bool>,
                            T3: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T4: ::planus::WriteAsDefault<bool, bool>,
                            T5: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T6: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T7: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T8: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T9: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T10: ::planus::WriteAsOptional<
                                ::planus::Offset<
                                    [::planus::Offset<self::BarrageModColumnMetadata>],
                                >,
                            >,
                            T11: ::planus::WriteAsDefault<i64, i64>,
                        >
                        ::planus::WriteAs<::planus::Offset<BarrageUpdateMetadata>>
                        for BarrageUpdateMetadataBuilder<(
                            T0,
                            T1,
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            T11,
                        )>
                    {
                        type Prepared = ::planus::Offset<BarrageUpdateMetadata>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageUpdateMetadata> {
                            ::planus::WriteAsOffset::prepare(self, builder)
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsDefault<i64, i64>,
                            T1: ::planus::WriteAsDefault<i64, i64>,
                            T2: ::planus::WriteAsDefault<bool, bool>,
                            T3: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T4: ::planus::WriteAsDefault<bool, bool>,
                            T5: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T6: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T7: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T8: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T9: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T10: ::planus::WriteAsOptional<
                                ::planus::Offset<
                                    [::planus::Offset<self::BarrageModColumnMetadata>],
                                >,
                            >,
                            T11: ::planus::WriteAsDefault<i64, i64>,
                        >
                        ::planus::WriteAsOptional<::planus::Offset<BarrageUpdateMetadata>>
                        for BarrageUpdateMetadataBuilder<(
                            T0,
                            T1,
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            T11,
                        )>
                    {
                        type Prepared = ::planus::Offset<BarrageUpdateMetadata>;

                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::core::option::Option<::planus::Offset<BarrageUpdateMetadata>>
                        {
                            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(
                                self, builder,
                            ))
                        }
                    }

                    impl<
                            T0: ::planus::WriteAsDefault<i64, i64>,
                            T1: ::planus::WriteAsDefault<i64, i64>,
                            T2: ::planus::WriteAsDefault<bool, bool>,
                            T3: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T4: ::planus::WriteAsDefault<bool, bool>,
                            T5: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T6: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T7: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T8: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T9: ::planus::WriteAsOptional<::planus::Offset<[i8]>>,
                            T10: ::planus::WriteAsOptional<
                                ::planus::Offset<
                                    [::planus::Offset<self::BarrageModColumnMetadata>],
                                >,
                            >,
                            T11: ::planus::WriteAsDefault<i64, i64>,
                        > ::planus::WriteAsOffset<BarrageUpdateMetadata>
                        for BarrageUpdateMetadataBuilder<(
                            T0,
                            T1,
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            T8,
                            T9,
                            T10,
                            T11,
                        )>
                    {
                        #[inline]
                        fn prepare(
                            &self,
                            builder: &mut ::planus::Builder,
                        ) -> ::planus::Offset<BarrageUpdateMetadata> {
                            let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) = &self.0;
                            BarrageUpdateMetadata::create(
                                builder, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11,
                            )
                        }
                    }

                    /// Reference to a deserialized [BarrageUpdateMetadata].
                    #[derive(Copy, Clone)]
                    pub struct BarrageUpdateMetadataRef<'a>(
                        #[allow(dead_code)] ::planus::table_reader::Table<'a>,
                    );

                    impl<'a> BarrageUpdateMetadataRef<'a> {
                        /// Getter for the [`first_seq` field](BarrageUpdateMetadata#structfield.first_seq).
                        #[inline]
                        pub fn first_seq(&self) -> ::planus::Result<i64> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(0, "BarrageUpdateMetadata", "first_seq")?
                                    .unwrap_or(0),
                            )
                        }

                        /// Getter for the [`last_seq` field](BarrageUpdateMetadata#structfield.last_seq).
                        #[inline]
                        pub fn last_seq(&self) -> ::planus::Result<i64> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(1, "BarrageUpdateMetadata", "last_seq")?
                                    .unwrap_or(0),
                            )
                        }

                        /// Getter for the [`is_snapshot` field](BarrageUpdateMetadata#structfield.is_snapshot).
                        #[inline]
                        pub fn is_snapshot(&self) -> ::planus::Result<bool> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(2, "BarrageUpdateMetadata", "is_snapshot")?
                                    .unwrap_or(false),
                            )
                        }

                        /// Getter for the [`effective_viewport` field](BarrageUpdateMetadata#structfield.effective_viewport).
                        #[inline]
                        pub fn effective_viewport(
                            &self,
                        ) -> ::planus::Result<::core::option::Option<&'a [i8]>>
                        {
                            self.0
                                .access(3, "BarrageUpdateMetadata", "effective_viewport")
                        }

                        /// Getter for the [`effective_reverse_viewport` field](BarrageUpdateMetadata#structfield.effective_reverse_viewport).
                        #[inline]
                        pub fn effective_reverse_viewport(&self) -> ::planus::Result<bool> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(
                                        4,
                                        "BarrageUpdateMetadata",
                                        "effective_reverse_viewport",
                                    )?
                                    .unwrap_or(false),
                            )
                        }

                        /// Getter for the [`effective_column_set` field](BarrageUpdateMetadata#structfield.effective_column_set).
                        #[inline]
                        pub fn effective_column_set(
                            &self,
                        ) -> ::planus::Result<::core::option::Option<&'a [i8]>>
                        {
                            self.0
                                .access(5, "BarrageUpdateMetadata", "effective_column_set")
                        }

                        /// Getter for the [`added_rows` field](BarrageUpdateMetadata#structfield.added_rows).
                        #[inline]
                        pub fn added_rows(
                            &self,
                        ) -> ::planus::Result<::core::option::Option<&'a [i8]>>
                        {
                            self.0.access(6, "BarrageUpdateMetadata", "added_rows")
                        }

                        /// Getter for the [`removed_rows` field](BarrageUpdateMetadata#structfield.removed_rows).
                        #[inline]
                        pub fn removed_rows(
                            &self,
                        ) -> ::planus::Result<::core::option::Option<&'a [i8]>>
                        {
                            self.0.access(7, "BarrageUpdateMetadata", "removed_rows")
                        }

                        /// Getter for the [`shift_data` field](BarrageUpdateMetadata#structfield.shift_data).
                        #[inline]
                        pub fn shift_data(
                            &self,
                        ) -> ::planus::Result<::core::option::Option<&'a [i8]>>
                        {
                            self.0.access(8, "BarrageUpdateMetadata", "shift_data")
                        }

                        /// Getter for the [`added_rows_included` field](BarrageUpdateMetadata#structfield.added_rows_included).
                        #[inline]
                        pub fn added_rows_included(
                            &self,
                        ) -> ::planus::Result<::core::option::Option<&'a [i8]>>
                        {
                            self.0
                                .access(9, "BarrageUpdateMetadata", "added_rows_included")
                        }

                        /// Getter for the [`mod_column_nodes` field](BarrageUpdateMetadata#structfield.mod_column_nodes).
                        #[inline]
                        pub fn mod_column_nodes(
                            &self,
                        ) -> ::planus::Result<
                            ::core::option::Option<
                                ::planus::Vector<
                                    'a,
                                    ::planus::Result<self::BarrageModColumnMetadataRef<'a>>,
                                >,
                            >,
                        > {
                            self.0
                                .access(10, "BarrageUpdateMetadata", "mod_column_nodes")
                        }

                        /// Getter for the [`table_size` field](BarrageUpdateMetadata#structfield.table_size).
                        #[inline]
                        pub fn table_size(&self) -> ::planus::Result<i64> {
                            ::core::result::Result::Ok(
                                self.0
                                    .access(11, "BarrageUpdateMetadata", "table_size")?
                                    .unwrap_or(0),
                            )
                        }
                    }

                    impl<'a> ::core::fmt::Debug for BarrageUpdateMetadataRef<'a> {
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                            let mut f = f.debug_struct("BarrageUpdateMetadataRef");
                            f.field("first_seq", &self.first_seq());
                            f.field("last_seq", &self.last_seq());
                            f.field("is_snapshot", &self.is_snapshot());
                            if let ::core::option::Option::Some(field_effective_viewport) =
                                self.effective_viewport().transpose()
                            {
                                f.field("effective_viewport", &field_effective_viewport);
                            }
                            f.field(
                                "effective_reverse_viewport",
                                &self.effective_reverse_viewport(),
                            );
                            if let ::core::option::Option::Some(field_effective_column_set) =
                                self.effective_column_set().transpose()
                            {
                                f.field("effective_column_set", &field_effective_column_set);
                            }
                            if let ::core::option::Option::Some(field_added_rows) =
                                self.added_rows().transpose()
                            {
                                f.field("added_rows", &field_added_rows);
                            }
                            if let ::core::option::Option::Some(field_removed_rows) =
                                self.removed_rows().transpose()
                            {
                                f.field("removed_rows", &field_removed_rows);
                            }
                            if let ::core::option::Option::Some(field_shift_data) =
                                self.shift_data().transpose()
                            {
                                f.field("shift_data", &field_shift_data);
                            }
                            if let ::core::option::Option::Some(field_added_rows_included) =
                                self.added_rows_included().transpose()
                            {
                                f.field("added_rows_included", &field_added_rows_included);
                            }
                            if let ::core::option::Option::Some(field_mod_column_nodes) =
                                self.mod_column_nodes().transpose()
                            {
                                f.field("mod_column_nodes", &field_mod_column_nodes);
                            }
                            f.field("table_size", &self.table_size());
                            f.finish()
                        }
                    }

                    impl<'a> ::core::convert::TryFrom<BarrageUpdateMetadataRef<'a>> for BarrageUpdateMetadata {
                        type Error = ::planus::Error;

                        #[allow(unreachable_code)]
                        fn try_from(value: BarrageUpdateMetadataRef<'a>) -> ::planus::Result<Self> {
                            ::core::result::Result::Ok(Self {
                                first_seq: ::core::convert::TryInto::try_into(value.first_seq()?)?,
                                last_seq: ::core::convert::TryInto::try_into(value.last_seq()?)?,
                                is_snapshot: ::core::convert::TryInto::try_into(
                                    value.is_snapshot()?,
                                )?,
                                effective_viewport: value.effective_viewport()?.map(|v| v.to_vec()),
                                effective_reverse_viewport: ::core::convert::TryInto::try_into(
                                    value.effective_reverse_viewport()?,
                                )?,
                                effective_column_set: value
                                    .effective_column_set()?
                                    .map(|v| v.to_vec()),
                                added_rows: value.added_rows()?.map(|v| v.to_vec()),
                                removed_rows: value.removed_rows()?.map(|v| v.to_vec()),
                                shift_data: value.shift_data()?.map(|v| v.to_vec()),
                                added_rows_included: value
                                    .added_rows_included()?
                                    .map(|v| v.to_vec()),
                                mod_column_nodes: if let ::core::option::Option::Some(
                                    mod_column_nodes,
                                ) = value.mod_column_nodes()?
                                {
                                    ::core::option::Option::Some(mod_column_nodes.to_vec_result()?)
                                } else {
                                    ::core::option::Option::None
                                },
                                table_size: ::core::convert::TryInto::try_into(
                                    value.table_size()?,
                                )?,
                            })
                        }
                    }

                    impl<'a> ::planus::TableRead<'a> for BarrageUpdateMetadataRef<'a> {
                        #[inline]
                        fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'a>,
                            offset: usize,
                        ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind>
                        {
                            ::core::result::Result::Ok(Self(
                                ::planus::table_reader::Table::from_buffer(buffer, offset)?,
                            ))
                        }
                    }

                    impl<'a> ::planus::VectorReadInner<'a> for BarrageUpdateMetadataRef<'a> {
                        type Error = ::planus::Error;
                        const STRIDE: usize = 4;

                        unsafe fn from_buffer(
                            buffer: ::planus::SliceWithStartOffset<'a>,
                            offset: usize,
                        ) -> ::planus::Result<Self> {
                            ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "[BarrageUpdateMetadataRef]",
                                    "get",
                                    buffer.offset_from_start,
                                )
                            })
                        }
                    }

                    /// # Safety
                    /// The planus compiler generates implementations that initialize
                    /// the bytes in `write_values`.
                    unsafe impl ::planus::VectorWrite<::planus::Offset<BarrageUpdateMetadata>>
                        for BarrageUpdateMetadata
                    {
                        type Value = ::planus::Offset<BarrageUpdateMetadata>;
                        const STRIDE: usize = 4;
                        #[inline]
                        fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                            ::planus::WriteAs::prepare(self, builder)
                        }

                        #[inline]
                        unsafe fn write_values(
                            values: &[::planus::Offset<BarrageUpdateMetadata>],
                            bytes: *mut ::core::mem::MaybeUninit<u8>,
                            buffer_position: u32,
                        ) {
                            let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                            for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                                ::planus::WriteAsPrimitive::write(
                                    v,
                                    ::planus::Cursor::new(unsafe { &mut *bytes.add(i) }),
                                    buffer_position - (Self::STRIDE * i) as u32,
                                );
                            }
                        }
                    }

                    impl<'a> ::planus::ReadAsRoot<'a> for BarrageUpdateMetadataRef<'a> {
                        fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                            ::planus::TableRead::from_buffer(
                                ::planus::SliceWithStartOffset {
                                    buffer: slice,
                                    offset_from_start: 0,
                                },
                                0,
                            )
                            .map_err(|error_kind| {
                                error_kind.with_error_location(
                                    "[BarrageUpdateMetadataRef]",
                                    "read_as_root",
                                    0,
                                )
                            })
                        }
                    }
                }
            }
        }
    }
}
