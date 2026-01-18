#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
pub mod entity {
    #[required(prefix = "Required")]
    pub struct DifferentialSample {
        #[prost(double, tag = "1")]
        pub t: f64,
        #[prost(double, tag = "2")]
        pub x: f64,
        #[prost(double, tag = "3")]
        pub y: f64,
        #[prost(double, tag = "4")]
        pub heading: f64,
        #[prost(double, tag = "5")]
        pub vl: f64,
        #[prost(double, tag = "6")]
        pub vr: f64,
        #[prost(double, tag = "7")]
        pub omega: f64,
        #[prost(double, tag = "8")]
        pub al: f64,
        #[prost(double, tag = "9")]
        pub ar: f64,
        #[prost(double, tag = "10")]
        pub alpha: f64,
        #[prost(double, tag = "11")]
        pub fl: f64,
        #[prost(double, tag = "12")]
        pub fr: f64,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DifferentialSample {
        #[inline]
        fn clone(&self) -> DifferentialSample {
            let _: ::core::clone::AssertParamIsClone<f64>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for DifferentialSample {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DifferentialSample {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DifferentialSample {
        #[inline]
        fn eq(&self, other: &DifferentialSample) -> bool {
            self.t == other.t && self.x == other.x && self.y == other.y
                && self.heading == other.heading && self.vl == other.vl
                && self.vr == other.vr && self.omega == other.omega
                && self.al == other.al && self.ar == other.ar
                && self.alpha == other.alpha && self.fl == other.fl
                && self.fr == other.fr
        }
    }
    impl ::prost::Message for DifferentialSample {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.t != 0f64 {
                ::prost::encoding::double::encode(1u32, &self.t, buf);
            }
            if self.x != 0f64 {
                ::prost::encoding::double::encode(2u32, &self.x, buf);
            }
            if self.y != 0f64 {
                ::prost::encoding::double::encode(3u32, &self.y, buf);
            }
            if self.heading != 0f64 {
                ::prost::encoding::double::encode(4u32, &self.heading, buf);
            }
            if self.vl != 0f64 {
                ::prost::encoding::double::encode(5u32, &self.vl, buf);
            }
            if self.vr != 0f64 {
                ::prost::encoding::double::encode(6u32, &self.vr, buf);
            }
            if self.omega != 0f64 {
                ::prost::encoding::double::encode(7u32, &self.omega, buf);
            }
            if self.al != 0f64 {
                ::prost::encoding::double::encode(8u32, &self.al, buf);
            }
            if self.ar != 0f64 {
                ::prost::encoding::double::encode(9u32, &self.ar, buf);
            }
            if self.alpha != 0f64 {
                ::prost::encoding::double::encode(10u32, &self.alpha, buf);
            }
            if self.fl != 0f64 {
                ::prost::encoding::double::encode(11u32, &self.fl, buf);
            }
            if self.fr != 0f64 {
                ::prost::encoding::double::encode(12u32, &self.fr, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "DifferentialSample";
            match tag {
                1u32 => {
                    let mut value = &mut self.t;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "t");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.x;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "x");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.y;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "y");
                            error
                        })
                }
                4u32 => {
                    let mut value = &mut self.heading;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "heading");
                            error
                        })
                }
                5u32 => {
                    let mut value = &mut self.vl;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "vl");
                            error
                        })
                }
                6u32 => {
                    let mut value = &mut self.vr;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "vr");
                            error
                        })
                }
                7u32 => {
                    let mut value = &mut self.omega;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "omega");
                            error
                        })
                }
                8u32 => {
                    let mut value = &mut self.al;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "al");
                            error
                        })
                }
                9u32 => {
                    let mut value = &mut self.ar;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "ar");
                            error
                        })
                }
                10u32 => {
                    let mut value = &mut self.alpha;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "alpha");
                            error
                        })
                }
                11u32 => {
                    let mut value = &mut self.fl;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "fl");
                            error
                        })
                }
                12u32 => {
                    let mut value = &mut self.fr;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "fr");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.t != 0f64 {
                    ::prost::encoding::double::encoded_len(1u32, &self.t)
                } else {
                    0
                }
                + if self.x != 0f64 {
                    ::prost::encoding::double::encoded_len(2u32, &self.x)
                } else {
                    0
                }
                + if self.y != 0f64 {
                    ::prost::encoding::double::encoded_len(3u32, &self.y)
                } else {
                    0
                }
                + if self.heading != 0f64 {
                    ::prost::encoding::double::encoded_len(4u32, &self.heading)
                } else {
                    0
                }
                + if self.vl != 0f64 {
                    ::prost::encoding::double::encoded_len(5u32, &self.vl)
                } else {
                    0
                }
                + if self.vr != 0f64 {
                    ::prost::encoding::double::encoded_len(6u32, &self.vr)
                } else {
                    0
                }
                + if self.omega != 0f64 {
                    ::prost::encoding::double::encoded_len(7u32, &self.omega)
                } else {
                    0
                }
                + if self.al != 0f64 {
                    ::prost::encoding::double::encoded_len(8u32, &self.al)
                } else {
                    0
                }
                + if self.ar != 0f64 {
                    ::prost::encoding::double::encoded_len(9u32, &self.ar)
                } else {
                    0
                }
                + if self.alpha != 0f64 {
                    ::prost::encoding::double::encoded_len(10u32, &self.alpha)
                } else {
                    0
                }
                + if self.fl != 0f64 {
                    ::prost::encoding::double::encoded_len(11u32, &self.fl)
                } else {
                    0
                }
                + if self.fr != 0f64 {
                    ::prost::encoding::double::encoded_len(12u32, &self.fr)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.t = 0f64;
            self.x = 0f64;
            self.y = 0f64;
            self.heading = 0f64;
            self.vl = 0f64;
            self.vr = 0f64;
            self.omega = 0f64;
            self.al = 0f64;
            self.ar = 0f64;
            self.alpha = 0f64;
            self.fl = 0f64;
            self.fr = 0f64;
        }
    }
    impl ::core::default::Default for DifferentialSample {
        fn default() -> Self {
            DifferentialSample {
                t: 0f64,
                x: 0f64,
                y: 0f64,
                heading: 0f64,
                vl: 0f64,
                vr: 0f64,
                omega: 0f64,
                al: 0f64,
                ar: 0f64,
                alpha: 0f64,
                fl: 0f64,
                fr: 0f64,
            }
        }
    }
    impl ::core::fmt::Debug for DifferentialSample {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("DifferentialSample");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.t)
                };
                builder.field("t", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.x)
                };
                builder.field("x", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.y)
                };
                builder.field("y", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.heading)
                };
                builder.field("heading", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.vl)
                };
                builder.field("vl", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.vr)
                };
                builder.field("vr", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.omega)
                };
                builder.field("omega", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.al)
                };
                builder.field("al", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.ar)
                };
                builder.field("ar", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.alpha)
                };
                builder.field("alpha", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.fl)
                };
                builder.field("fl", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.fr)
                };
                builder.field("fr", &wrapper)
            };
            builder.finish()
        }
    }
    pub struct RequiredDifferentialSample {
        pub t: f64,
        pub x: f64,
        pub y: f64,
        pub heading: f64,
        pub vl: f64,
        pub vr: f64,
        pub omega: f64,
        pub al: f64,
        pub ar: f64,
        pub alpha: f64,
        pub fl: f64,
        pub fr: f64,
    }
    impl From<RequiredDifferentialSample> for DifferentialSample {
        fn from(value: RequiredDifferentialSample) -> Self {
            DifferentialSample {
                t: value.t.into(),
                x: value.x.into(),
                y: value.y.into(),
                heading: value.heading.into(),
                vl: value.vl.into(),
                vr: value.vr.into(),
                omega: value.omega.into(),
                al: value.al.into(),
                ar: value.ar.into(),
                alpha: value.alpha.into(),
                fl: value.fl.into(),
                fr: value.fr.into(),
            }
        }
    }
    #[required(prefix = "Required")]
    #[repr(i32)]
    pub enum DriveType {
        DrivetypeSwerve = 0,
        DrivetypeDifferential = 1,
        DrivetypeMecanum = 2,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DriveType {
        #[inline]
        fn clone(&self) -> DriveType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for DriveType {}
    #[automatically_derived]
    impl ::core::fmt::Debug for DriveType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    DriveType::DrivetypeSwerve => "DrivetypeSwerve",
                    DriveType::DrivetypeDifferential => "DrivetypeDifferential",
                    DriveType::DrivetypeMecanum => "DrivetypeMecanum",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DriveType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DriveType {
        #[inline]
        fn eq(&self, other: &DriveType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DriveType {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DriveType {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for DriveType {
        #[inline]
        fn partial_cmp(
            &self,
            other: &DriveType,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for DriveType {
        #[inline]
        fn cmp(&self, other: &DriveType) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    impl DriveType {
        ///Returns `true` if `value` is a variant of `DriveType`.
        pub fn is_valid(value: i32) -> bool {
            match value {
                0 => true,
                1 => true,
                2 => true,
                _ => false,
            }
        }
        #[deprecated = "Use the TryFrom<i32> implementation instead"]
        ///Converts an `i32` to a `DriveType`, or `None` if `value` is not a valid variant.
        pub fn from_i32(value: i32) -> ::core::option::Option<DriveType> {
            match value {
                0 => ::core::option::Option::Some(DriveType::DrivetypeSwerve),
                1 => ::core::option::Option::Some(DriveType::DrivetypeDifferential),
                2 => ::core::option::Option::Some(DriveType::DrivetypeMecanum),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl ::core::default::Default for DriveType {
        fn default() -> DriveType {
            DriveType::DrivetypeSwerve
        }
    }
    impl ::core::convert::From<DriveType> for i32 {
        fn from(value: DriveType) -> i32 {
            value as i32
        }
    }
    impl ::core::convert::TryFrom<i32> for DriveType {
        type Error = ::prost::UnknownEnumValue;
        fn try_from(
            value: i32,
        ) -> ::core::result::Result<DriveType, ::prost::UnknownEnumValue> {
            match value {
                0 => ::core::result::Result::Ok(DriveType::DrivetypeSwerve),
                1 => ::core::result::Result::Ok(DriveType::DrivetypeDifferential),
                2 => ::core::result::Result::Ok(DriveType::DrivetypeMecanum),
                _ => ::core::result::Result::Err(::prost::UnknownEnumValue(value)),
            }
        }
    }
    pub type RequiredDriveType = DriveType;
    impl DriveType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::DrivetypeSwerve => "DRIVETYPE_SWERVE",
                Self::DrivetypeDifferential => "DRIVETYPE_DIFFERENTIAL",
                Self::DrivetypeMecanum => "DRIVETYPE_MECANUM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DRIVETYPE_SWERVE" => Some(Self::DrivetypeSwerve),
                "DRIVETYPE_DIFFERENTIAL" => Some(Self::DrivetypeDifferential),
                "DRIVETYPE_MECANUM" => Some(Self::DrivetypeMecanum),
                _ => None,
            }
        }
    }
    #[required(prefix = "Required")]
    pub struct ForceVector {
        #[prost(double, tag = "1")]
        pub x: f64,
        #[prost(double, tag = "2")]
        pub y: f64,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ForceVector {
        #[inline]
        fn clone(&self) -> ForceVector {
            let _: ::core::clone::AssertParamIsClone<f64>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ForceVector {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ForceVector {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ForceVector {
        #[inline]
        fn eq(&self, other: &ForceVector) -> bool {
            self.x == other.x && self.y == other.y
        }
    }
    impl ::prost::Message for ForceVector {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.x != 0f64 {
                ::prost::encoding::double::encode(1u32, &self.x, buf);
            }
            if self.y != 0f64 {
                ::prost::encoding::double::encode(2u32, &self.y, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "ForceVector";
            match tag {
                1u32 => {
                    let mut value = &mut self.x;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "x");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.y;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "y");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.x != 0f64 {
                    ::prost::encoding::double::encoded_len(1u32, &self.x)
                } else {
                    0
                }
                + if self.y != 0f64 {
                    ::prost::encoding::double::encoded_len(2u32, &self.y)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.x = 0f64;
            self.y = 0f64;
        }
    }
    impl ::core::default::Default for ForceVector {
        fn default() -> Self {
            ForceVector { x: 0f64, y: 0f64 }
        }
    }
    impl ::core::fmt::Debug for ForceVector {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("ForceVector");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.x)
                };
                builder.field("x", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.y)
                };
                builder.field("y", &wrapper)
            };
            builder.finish()
        }
    }
    pub struct RequiredForceVector {
        pub x: f64,
        pub y: f64,
    }
    impl From<RequiredForceVector> for ForceVector {
        fn from(value: RequiredForceVector) -> Self {
            ForceVector {
                x: value.x.into(),
                y: value.y.into(),
            }
        }
    }
    #[required(prefix = "Required")]
    pub struct SwerveSample {
        #[prost(double, tag = "1")]
        pub t: f64,
        #[prost(double, tag = "2")]
        pub x: f64,
        #[prost(double, tag = "3")]
        pub y: f64,
        #[prost(double, tag = "4")]
        pub heading: f64,
        #[prost(double, tag = "5")]
        pub vx: f64,
        #[prost(double, tag = "6")]
        pub vy: f64,
        #[prost(double, tag = "7")]
        pub omega: f64,
        #[prost(double, tag = "8")]
        pub ax: f64,
        #[prost(double, tag = "9")]
        pub ay: f64,
        #[prost(double, tag = "10")]
        pub alpha: f64,
        #[prost(message, optional, tag = "11")]
        pub fl: ::core::option::Option<ForceVector>,
        #[prost(message, optional, tag = "12")]
        pub fr: ::core::option::Option<ForceVector>,
        #[prost(message, optional, tag = "13")]
        pub bl: ::core::option::Option<ForceVector>,
        #[prost(message, optional, tag = "14")]
        pub br: ::core::option::Option<ForceVector>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SwerveSample {
        #[inline]
        fn clone(&self) -> SwerveSample {
            let _: ::core::clone::AssertParamIsClone<f64>;
            let _: ::core::clone::AssertParamIsClone<
                ::core::option::Option<ForceVector>,
            >;
            let _: ::core::clone::AssertParamIsClone<
                ::core::option::Option<ForceVector>,
            >;
            let _: ::core::clone::AssertParamIsClone<
                ::core::option::Option<ForceVector>,
            >;
            let _: ::core::clone::AssertParamIsClone<
                ::core::option::Option<ForceVector>,
            >;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for SwerveSample {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SwerveSample {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SwerveSample {
        #[inline]
        fn eq(&self, other: &SwerveSample) -> bool {
            self.t == other.t && self.x == other.x && self.y == other.y
                && self.heading == other.heading && self.vx == other.vx
                && self.vy == other.vy && self.omega == other.omega
                && self.ax == other.ax && self.ay == other.ay
                && self.alpha == other.alpha && self.fl == other.fl
                && self.fr == other.fr && self.bl == other.bl && self.br == other.br
        }
    }
    impl ::prost::Message for SwerveSample {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.t != 0f64 {
                ::prost::encoding::double::encode(1u32, &self.t, buf);
            }
            if self.x != 0f64 {
                ::prost::encoding::double::encode(2u32, &self.x, buf);
            }
            if self.y != 0f64 {
                ::prost::encoding::double::encode(3u32, &self.y, buf);
            }
            if self.heading != 0f64 {
                ::prost::encoding::double::encode(4u32, &self.heading, buf);
            }
            if self.vx != 0f64 {
                ::prost::encoding::double::encode(5u32, &self.vx, buf);
            }
            if self.vy != 0f64 {
                ::prost::encoding::double::encode(6u32, &self.vy, buf);
            }
            if self.omega != 0f64 {
                ::prost::encoding::double::encode(7u32, &self.omega, buf);
            }
            if self.ax != 0f64 {
                ::prost::encoding::double::encode(8u32, &self.ax, buf);
            }
            if self.ay != 0f64 {
                ::prost::encoding::double::encode(9u32, &self.ay, buf);
            }
            if self.alpha != 0f64 {
                ::prost::encoding::double::encode(10u32, &self.alpha, buf);
            }
            if let Some(ref msg) = self.fl {
                ::prost::encoding::message::encode(11u32, msg, buf);
            }
            if let Some(ref msg) = self.fr {
                ::prost::encoding::message::encode(12u32, msg, buf);
            }
            if let Some(ref msg) = self.bl {
                ::prost::encoding::message::encode(13u32, msg, buf);
            }
            if let Some(ref msg) = self.br {
                ::prost::encoding::message::encode(14u32, msg, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "SwerveSample";
            match tag {
                1u32 => {
                    let mut value = &mut self.t;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "t");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.x;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "x");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.y;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "y");
                            error
                        })
                }
                4u32 => {
                    let mut value = &mut self.heading;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "heading");
                            error
                        })
                }
                5u32 => {
                    let mut value = &mut self.vx;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "vx");
                            error
                        })
                }
                6u32 => {
                    let mut value = &mut self.vy;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "vy");
                            error
                        })
                }
                7u32 => {
                    let mut value = &mut self.omega;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "omega");
                            error
                        })
                }
                8u32 => {
                    let mut value = &mut self.ax;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "ax");
                            error
                        })
                }
                9u32 => {
                    let mut value = &mut self.ay;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "ay");
                            error
                        })
                }
                10u32 => {
                    let mut value = &mut self.alpha;
                    ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "alpha");
                            error
                        })
                }
                11u32 => {
                    let mut value = &mut self.fl;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "fl");
                            error
                        })
                }
                12u32 => {
                    let mut value = &mut self.fr;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "fr");
                            error
                        })
                }
                13u32 => {
                    let mut value = &mut self.bl;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "bl");
                            error
                        })
                }
                14u32 => {
                    let mut value = &mut self.br;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "br");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.t != 0f64 {
                    ::prost::encoding::double::encoded_len(1u32, &self.t)
                } else {
                    0
                }
                + if self.x != 0f64 {
                    ::prost::encoding::double::encoded_len(2u32, &self.x)
                } else {
                    0
                }
                + if self.y != 0f64 {
                    ::prost::encoding::double::encoded_len(3u32, &self.y)
                } else {
                    0
                }
                + if self.heading != 0f64 {
                    ::prost::encoding::double::encoded_len(4u32, &self.heading)
                } else {
                    0
                }
                + if self.vx != 0f64 {
                    ::prost::encoding::double::encoded_len(5u32, &self.vx)
                } else {
                    0
                }
                + if self.vy != 0f64 {
                    ::prost::encoding::double::encoded_len(6u32, &self.vy)
                } else {
                    0
                }
                + if self.omega != 0f64 {
                    ::prost::encoding::double::encoded_len(7u32, &self.omega)
                } else {
                    0
                }
                + if self.ax != 0f64 {
                    ::prost::encoding::double::encoded_len(8u32, &self.ax)
                } else {
                    0
                }
                + if self.ay != 0f64 {
                    ::prost::encoding::double::encoded_len(9u32, &self.ay)
                } else {
                    0
                }
                + if self.alpha != 0f64 {
                    ::prost::encoding::double::encoded_len(10u32, &self.alpha)
                } else {
                    0
                }
                + self
                    .fl
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(11u32, msg))
                + self
                    .fr
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(12u32, msg))
                + self
                    .bl
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(13u32, msg))
                + self
                    .br
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(14u32, msg))
        }
        fn clear(&mut self) {
            self.t = 0f64;
            self.x = 0f64;
            self.y = 0f64;
            self.heading = 0f64;
            self.vx = 0f64;
            self.vy = 0f64;
            self.omega = 0f64;
            self.ax = 0f64;
            self.ay = 0f64;
            self.alpha = 0f64;
            self.fl = ::core::option::Option::None;
            self.fr = ::core::option::Option::None;
            self.bl = ::core::option::Option::None;
            self.br = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for SwerveSample {
        fn default() -> Self {
            SwerveSample {
                t: 0f64,
                x: 0f64,
                y: 0f64,
                heading: 0f64,
                vx: 0f64,
                vy: 0f64,
                omega: 0f64,
                ax: 0f64,
                ay: 0f64,
                alpha: 0f64,
                fl: ::core::default::Default::default(),
                fr: ::core::default::Default::default(),
                bl: ::core::default::Default::default(),
                br: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for SwerveSample {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("SwerveSample");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.t)
                };
                builder.field("t", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.x)
                };
                builder.field("x", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.y)
                };
                builder.field("y", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.heading)
                };
                builder.field("heading", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.vx)
                };
                builder.field("vx", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.vy)
                };
                builder.field("vy", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.omega)
                };
                builder.field("omega", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.ax)
                };
                builder.field("ax", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.ay)
                };
                builder.field("ay", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.alpha)
                };
                builder.field("alpha", &wrapper)
            };
            let builder = {
                let wrapper = &self.fl;
                builder.field("fl", &wrapper)
            };
            let builder = {
                let wrapper = &self.fr;
                builder.field("fr", &wrapper)
            };
            let builder = {
                let wrapper = &self.bl;
                builder.field("bl", &wrapper)
            };
            let builder = {
                let wrapper = &self.br;
                builder.field("br", &wrapper)
            };
            builder.finish()
        }
    }
    pub struct RequiredSwerveSample {
        pub t: f64,
        pub x: f64,
        pub y: f64,
        pub heading: f64,
        pub vx: f64,
        pub vy: f64,
        pub omega: f64,
        pub ax: f64,
        pub ay: f64,
        pub alpha: f64,
        pub fl: RequiredForceVector,
        pub fr: RequiredForceVector,
        pub bl: RequiredForceVector,
        pub br: RequiredForceVector,
    }
    impl From<RequiredSwerveSample> for SwerveSample {
        fn from(value: RequiredSwerveSample) -> Self {
            SwerveSample {
                t: value.t.into(),
                x: value.x.into(),
                y: value.y.into(),
                heading: value.heading.into(),
                vx: value.vx.into(),
                vy: value.vy.into(),
                omega: value.omega.into(),
                ax: value.ax.into(),
                ay: value.ay.into(),
                alpha: value.alpha.into(),
                fl: Some(value.fl.into()),
                fr: Some(value.fr.into()),
                bl: Some(value.bl.into()),
                br: Some(value.br.into()),
            }
        }
    }
    #[required(prefix = "Required")]
    pub struct SwerveTrajectory {
        #[prost(message, repeated, tag = "1")]
        pub samples: ::prost::alloc::vec::Vec<SwerveSample>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SwerveTrajectory {
        #[inline]
        fn clone(&self) -> SwerveTrajectory {
            SwerveTrajectory {
                samples: ::core::clone::Clone::clone(&self.samples),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SwerveTrajectory {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SwerveTrajectory {
        #[inline]
        fn eq(&self, other: &SwerveTrajectory) -> bool {
            self.samples == other.samples
        }
    }
    impl ::prost::Message for SwerveTrajectory {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            for msg in &self.samples {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "SwerveTrajectory";
            match tag {
                1u32 => {
                    let mut value = &mut self.samples;
                    ::prost::encoding::message::merge_repeated(
                            wire_type,
                            value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "samples");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::prost::encoding::message::encoded_len_repeated(1u32, &self.samples)
        }
        fn clear(&mut self) {
            self.samples.clear();
        }
    }
    impl ::core::default::Default for SwerveTrajectory {
        fn default() -> Self {
            SwerveTrajectory {
                samples: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for SwerveTrajectory {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("SwerveTrajectory");
            let builder = {
                let wrapper = &self.samples;
                builder.field("samples", &wrapper)
            };
            builder.finish()
        }
    }
    pub struct RequiredSwerveTrajectory {
        pub samples: ::prost::alloc::vec::Vec<SwerveSample>,
    }
    impl From<RequiredSwerveTrajectory> for SwerveTrajectory {
        fn from(value: RequiredSwerveTrajectory) -> Self {
            SwerveTrajectory {
                samples: value.samples.into(),
            }
        }
    }
    #[required(prefix = "Required")]
    pub struct DifferentialTrajectory {
        #[prost(message, repeated, tag = "1")]
        pub samples: ::prost::alloc::vec::Vec<DifferentialSample>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DifferentialTrajectory {
        #[inline]
        fn clone(&self) -> DifferentialTrajectory {
            DifferentialTrajectory {
                samples: ::core::clone::Clone::clone(&self.samples),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DifferentialTrajectory {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DifferentialTrajectory {
        #[inline]
        fn eq(&self, other: &DifferentialTrajectory) -> bool {
            self.samples == other.samples
        }
    }
    impl ::prost::Message for DifferentialTrajectory {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            for msg in &self.samples {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "DifferentialTrajectory";
            match tag {
                1u32 => {
                    let mut value = &mut self.samples;
                    ::prost::encoding::message::merge_repeated(
                            wire_type,
                            value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "samples");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::prost::encoding::message::encoded_len_repeated(1u32, &self.samples)
        }
        fn clear(&mut self) {
            self.samples.clear();
        }
    }
    impl ::core::default::Default for DifferentialTrajectory {
        fn default() -> Self {
            DifferentialTrajectory {
                samples: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for DifferentialTrajectory {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("DifferentialTrajectory");
            let builder = {
                let wrapper = &self.samples;
                builder.field("samples", &wrapper)
            };
            builder.finish()
        }
    }
    pub struct RequiredDifferentialTrajectory {
        pub samples: ::prost::alloc::vec::Vec<DifferentialSample>,
    }
    impl From<RequiredDifferentialTrajectory> for DifferentialTrajectory {
        fn from(value: RequiredDifferentialTrajectory) -> Self {
            DifferentialTrajectory {
                samples: value.samples.into(),
            }
        }
    }
    #[required(prefix = "Required")]
    pub struct GenerationOutput {
        #[prost(uint64, repeated, tag = "3")]
        pub splits: ::prost::alloc::vec::Vec<u64>,
        #[prost(double, repeated, tag = "4")]
        pub waypoints: ::prost::alloc::vec::Vec<f64>,
        #[prost(message, optional, tag = "5")]
        pub config: ::core::option::Option<parameters::robotconfig::DoubleRobotConfig>,
        #[prost(oneof = "generation_output::Trajectory", tags = "1, 2")]
        pub trajectory: ::core::option::Option<generation_output::Trajectory>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for GenerationOutput {
        #[inline]
        fn clone(&self) -> GenerationOutput {
            GenerationOutput {
                splits: ::core::clone::Clone::clone(&self.splits),
                waypoints: ::core::clone::Clone::clone(&self.waypoints),
                config: ::core::clone::Clone::clone(&self.config),
                trajectory: ::core::clone::Clone::clone(&self.trajectory),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for GenerationOutput {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for GenerationOutput {
        #[inline]
        fn eq(&self, other: &GenerationOutput) -> bool {
            self.splits == other.splits && self.waypoints == other.waypoints
                && self.config == other.config && self.trajectory == other.trajectory
        }
    }
    impl ::prost::Message for GenerationOutput {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if let Some(ref oneof) = self.trajectory {
                oneof.encode(buf)
            }
            ::prost::encoding::uint64::encode_packed(3u32, &self.splits, buf);
            ::prost::encoding::double::encode_packed(4u32, &self.waypoints, buf);
            if let Some(ref msg) = self.config {
                ::prost::encoding::message::encode(5u32, msg, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "GenerationOutput";
            match tag {
                1u32 | 2u32 => {
                    let mut value = &mut self.trajectory;
                    generation_output::Trajectory::merge(value, tag, wire_type, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "trajectory");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.splits;
                    ::prost::encoding::uint64::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "splits");
                            error
                        })
                }
                4u32 => {
                    let mut value = &mut self.waypoints;
                    ::prost::encoding::double::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "waypoints");
                            error
                        })
                }
                5u32 => {
                    let mut value = &mut self.config;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "config");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + self
                    .trajectory
                    .as_ref()
                    .map_or(0, generation_output::Trajectory::encoded_len)
                + ::prost::encoding::uint64::encoded_len_packed(3u32, &self.splits)
                + ::prost::encoding::double::encoded_len_packed(4u32, &self.waypoints)
                + self
                    .config
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(5u32, msg))
        }
        fn clear(&mut self) {
            self.trajectory = ::core::option::Option::None;
            self.splits.clear();
            self.waypoints.clear();
            self.config = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for GenerationOutput {
        fn default() -> Self {
            GenerationOutput {
                trajectory: ::core::default::Default::default(),
                splits: ::prost::alloc::vec::Vec::new(),
                waypoints: ::prost::alloc::vec::Vec::new(),
                config: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for GenerationOutput {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("GenerationOutput");
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<u64>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.splits)
                };
                builder.field("splits", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<f64>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.waypoints)
                };
                builder.field("waypoints", &wrapper)
            };
            let builder = {
                let wrapper = &self.config;
                builder.field("config", &wrapper)
            };
            let builder = {
                let wrapper = &self.trajectory;
                builder.field("trajectory", &wrapper)
            };
            builder.finish()
        }
    }
    pub struct RequiredGenerationOutput {
        pub splits: ::prost::alloc::vec::Vec<u64>,
        pub waypoints: ::prost::alloc::vec::Vec<f64>,
        pub config: parameters::robotconfig::RequiredDoubleRobotConfig,
        pub trajectory: generation_output::RequiredTrajectory,
    }
    impl From<RequiredGenerationOutput> for GenerationOutput {
        fn from(value: RequiredGenerationOutput) -> Self {
            GenerationOutput {
                splits: value.splits.into(),
                waypoints: value.waypoints.into(),
                config: Some(value.config.into()),
                trajectory: Some(value.trajectory.into()),
            }
        }
    }
    /// Nested message and enum types in `GenerationOutput`.
    pub mod generation_output {
        #[required(prefix = "Required")]
        pub enum Trajectory {
            #[prost(message, tag = "1")]
            Swerve(super::SwerveTrajectory),
            #[prost(message, tag = "2")]
            Differential(super::DifferentialTrajectory),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Trajectory {
            #[inline]
            fn clone(&self) -> Trajectory {
                match self {
                    Trajectory::Swerve(__self_0) => {
                        Trajectory::Swerve(::core::clone::Clone::clone(__self_0))
                    }
                    Trajectory::Differential(__self_0) => {
                        Trajectory::Differential(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Trajectory {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Trajectory {
            #[inline]
            fn eq(&self, other: &Trajectory) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (Trajectory::Swerve(__self_0), Trajectory::Swerve(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (
                            Trajectory::Differential(__self_0),
                            Trajectory::Differential(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        impl Trajectory {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    Trajectory::Swerve(ref value) => {
                        ::prost::encoding::message::encode(1u32, &*value, buf);
                    }
                    Trajectory::Differential(ref value) => {
                        ::prost::encoding::message::encode(2u32, &*value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<Trajectory>,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    1u32 => {
                        if let ::core::option::Option::Some(Trajectory::Swerve(value)) = field {
                            ::prost::encoding::message::merge(wire_type, value, buf, ctx)
                        } else {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::prost::encoding::message::merge(wire_type, value, buf, ctx)
                                .map(|_| {
                                    *field = ::core::option::Option::Some(
                                        Trajectory::Swerve(owned_value),
                                    );
                                })
                        }
                    }
                    2u32 => {
                        if let ::core::option::Option::Some(
                            Trajectory::Differential(value),
                        ) = field {
                            ::prost::encoding::message::merge(wire_type, value, buf, ctx)
                        } else {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::prost::encoding::message::merge(wire_type, value, buf, ctx)
                                .map(|_| {
                                    *field = ::core::option::Option::Some(
                                        Trajectory::Differential(owned_value),
                                    );
                                })
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("invalid Trajectory tag: {0}", tag),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    Trajectory::Swerve(ref value) => {
                        ::prost::encoding::message::encoded_len(1u32, &*value)
                    }
                    Trajectory::Differential(ref value) => {
                        ::prost::encoding::message::encoded_len(2u32, &*value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for Trajectory {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Trajectory::Swerve(ref value) => {
                        let wrapper = &*value;
                        f.debug_tuple("Swerve").field(&wrapper).finish()
                    }
                    Trajectory::Differential(ref value) => {
                        let wrapper = &*value;
                        f.debug_tuple("Differential").field(&wrapper).finish()
                    }
                }
            }
        }
        pub type RequiredTrajectory = Trajectory;
    }
    #[required(prefix = "Required")]
    pub struct TrajectoryFile {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub params: ::core::option::Option<parameters::ExprParameters>,
        #[prost(message, optional, tag = "3")]
        #[required(optional)]
        pub snapshot: ::core::option::Option<parameters::DoubleParameters>,
        #[prost(message, optional, tag = "4")]
        pub trajectory: ::core::option::Option<GenerationOutput>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TrajectoryFile {
        #[inline]
        fn clone(&self) -> TrajectoryFile {
            TrajectoryFile {
                name: ::core::clone::Clone::clone(&self.name),
                params: ::core::clone::Clone::clone(&self.params),
                snapshot: ::core::clone::Clone::clone(&self.snapshot),
                trajectory: ::core::clone::Clone::clone(&self.trajectory),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TrajectoryFile {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TrajectoryFile {
        #[inline]
        fn eq(&self, other: &TrajectoryFile) -> bool {
            self.name == other.name && self.params == other.params
                && self.snapshot == other.snapshot && self.trajectory == other.trajectory
        }
    }
    impl ::prost::Message for TrajectoryFile {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.name != "" {
                ::prost::encoding::string::encode(1u32, &self.name, buf);
            }
            if let Some(ref msg) = self.params {
                ::prost::encoding::message::encode(2u32, msg, buf);
            }
            if let Some(ref msg) = self.snapshot {
                ::prost::encoding::message::encode(3u32, msg, buf);
            }
            if let Some(ref msg) = self.trajectory {
                ::prost::encoding::message::encode(4u32, msg, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "TrajectoryFile";
            match tag {
                1u32 => {
                    let mut value = &mut self.name;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "name");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.params;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "params");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.snapshot;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "snapshot");
                            error
                        })
                }
                4u32 => {
                    let mut value = &mut self.trajectory;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "trajectory");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.name != "" {
                    ::prost::encoding::string::encoded_len(1u32, &self.name)
                } else {
                    0
                }
                + self
                    .params
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(2u32, msg))
                + self
                    .snapshot
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(3u32, msg))
                + self
                    .trajectory
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(4u32, msg))
        }
        fn clear(&mut self) {
            self.name.clear();
            self.params = ::core::option::Option::None;
            self.snapshot = ::core::option::Option::None;
            self.trajectory = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for TrajectoryFile {
        fn default() -> Self {
            TrajectoryFile {
                name: ::prost::alloc::string::String::new(),
                params: ::core::default::Default::default(),
                snapshot: ::core::default::Default::default(),
                trajectory: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for TrajectoryFile {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("TrajectoryFile");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.name)
                };
                builder.field("name", &wrapper)
            };
            let builder = {
                let wrapper = &self.params;
                builder.field("params", &wrapper)
            };
            let builder = {
                let wrapper = &self.snapshot;
                builder.field("snapshot", &wrapper)
            };
            let builder = {
                let wrapper = &self.trajectory;
                builder.field("trajectory", &wrapper)
            };
            builder.finish()
        }
    }
    pub struct RequiredTrajectoryFile {
        pub name: ::prost::alloc::string::String,
        pub params: parameters::RequiredExprParameters,
        pub snapshot: ::core::option::Option<parameters::DoubleParameters>,
        pub trajectory: RequiredGenerationOutput,
    }
    impl From<RequiredTrajectoryFile> for TrajectoryFile {
        fn from(value: RequiredTrajectoryFile) -> Self {
            TrajectoryFile {
                name: value.name.into(),
                params: Some(value.params.into()),
                snapshot: value.snapshot.into(),
                trajectory: Some(value.trajectory.into()),
            }
        }
    }
    impl serde::Serialize for DifferentialSample {
        #[allow(deprecated)]
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::SerializeStruct;
            let mut len = 0;
            if self.t != 0. {
                len += 1;
            }
            if self.x != 0. {
                len += 1;
            }
            if self.y != 0. {
                len += 1;
            }
            if self.heading != 0. {
                len += 1;
            }
            if self.vl != 0. {
                len += 1;
            }
            if self.vr != 0. {
                len += 1;
            }
            if self.omega != 0. {
                len += 1;
            }
            if self.al != 0. {
                len += 1;
            }
            if self.ar != 0. {
                len += 1;
            }
            if self.alpha != 0. {
                len += 1;
            }
            if self.fl != 0. {
                len += 1;
            }
            if self.fr != 0. {
                len += 1;
            }
            let mut struct_ser = serializer
                .serialize_struct("entity.DifferentialSample", len)?;
            if self.t != 0. {
                struct_ser.serialize_field("t", &self.t)?;
            }
            if self.x != 0. {
                struct_ser.serialize_field("x", &self.x)?;
            }
            if self.y != 0. {
                struct_ser.serialize_field("y", &self.y)?;
            }
            if self.heading != 0. {
                struct_ser.serialize_field("heading", &self.heading)?;
            }
            if self.vl != 0. {
                struct_ser.serialize_field("vl", &self.vl)?;
            }
            if self.vr != 0. {
                struct_ser.serialize_field("vr", &self.vr)?;
            }
            if self.omega != 0. {
                struct_ser.serialize_field("omega", &self.omega)?;
            }
            if self.al != 0. {
                struct_ser.serialize_field("al", &self.al)?;
            }
            if self.ar != 0. {
                struct_ser.serialize_field("ar", &self.ar)?;
            }
            if self.alpha != 0. {
                struct_ser.serialize_field("alpha", &self.alpha)?;
            }
            if self.fl != 0. {
                struct_ser.serialize_field("fl", &self.fl)?;
            }
            if self.fr != 0. {
                struct_ser.serialize_field("fr", &self.fr)?;
            }
            struct_ser.end()
        }
    }
    impl<'de> serde::Deserialize<'de> for DifferentialSample {
        #[allow(deprecated)]
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "t",
                "x",
                "y",
                "heading",
                "vl",
                "vr",
                "omega",
                "al",
                "ar",
                "alpha",
                "fl",
                "fr",
            ];
            #[allow(clippy::enum_variant_names)]
            enum GeneratedField {
                T,
                X,
                Y,
                Heading,
                Vl,
                Vr,
                Omega,
                Al,
                Ar,
                Alpha,
                Fl,
                Fr,
            }
            impl<'de> serde::Deserialize<'de> for GeneratedField {
                fn deserialize<D>(
                    deserializer: D,
                ) -> std::result::Result<GeneratedField, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    struct GeneratedVisitor;
                    impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                        type Value = GeneratedField;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> std::fmt::Result {
                            formatter
                                .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                        }
                        #[allow(unused_variables)]
                        fn visit_str<E>(
                            self,
                            value: &str,
                        ) -> std::result::Result<GeneratedField, E>
                        where
                            E: serde::de::Error,
                        {
                            match value {
                                "t" => Ok(GeneratedField::T),
                                "x" => Ok(GeneratedField::X),
                                "y" => Ok(GeneratedField::Y),
                                "heading" => Ok(GeneratedField::Heading),
                                "vl" => Ok(GeneratedField::Vl),
                                "vr" => Ok(GeneratedField::Vr),
                                "omega" => Ok(GeneratedField::Omega),
                                "al" => Ok(GeneratedField::Al),
                                "ar" => Ok(GeneratedField::Ar),
                                "alpha" => Ok(GeneratedField::Alpha),
                                "fl" => Ok(GeneratedField::Fl),
                                "fr" => Ok(GeneratedField::Fr),
                                _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                            }
                        }
                    }
                    deserializer.deserialize_identifier(GeneratedVisitor)
                }
            }
            struct GeneratedVisitor;
            impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                type Value = DifferentialSample;
                fn expecting(
                    &self,
                    formatter: &mut std::fmt::Formatter<'_>,
                ) -> std::fmt::Result {
                    formatter.write_str("struct entity.DifferentialSample")
                }
                fn visit_map<V>(
                    self,
                    mut map_: V,
                ) -> std::result::Result<DifferentialSample, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
                {
                    let mut t__ = None;
                    let mut x__ = None;
                    let mut y__ = None;
                    let mut heading__ = None;
                    let mut vl__ = None;
                    let mut vr__ = None;
                    let mut omega__ = None;
                    let mut al__ = None;
                    let mut ar__ = None;
                    let mut alpha__ = None;
                    let mut fl__ = None;
                    let mut fr__ = None;
                    while let Some(k) = map_.next_key()? {
                        match k {
                            GeneratedField::T => {
                                if t__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("t"));
                                }
                                t__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::X => {
                                if x__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("x"));
                                }
                                x__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Y => {
                                if y__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("y"));
                                }
                                y__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Heading => {
                                if heading__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("heading"));
                                }
                                heading__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Vl => {
                                if vl__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("vl"));
                                }
                                vl__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Vr => {
                                if vr__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("vr"));
                                }
                                vr__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Omega => {
                                if omega__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("omega"));
                                }
                                omega__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Al => {
                                if al__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("al"));
                                }
                                al__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Ar => {
                                if ar__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("ar"));
                                }
                                ar__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Alpha => {
                                if alpha__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("alpha"));
                                }
                                alpha__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Fl => {
                                if fl__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fl"));
                                }
                                fl__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Fr => {
                                if fr__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fr"));
                                }
                                fr__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                        }
                    }
                    Ok(DifferentialSample {
                        t: t__.unwrap_or_default(),
                        x: x__.unwrap_or_default(),
                        y: y__.unwrap_or_default(),
                        heading: heading__.unwrap_or_default(),
                        vl: vl__.unwrap_or_default(),
                        vr: vr__.unwrap_or_default(),
                        omega: omega__.unwrap_or_default(),
                        al: al__.unwrap_or_default(),
                        ar: ar__.unwrap_or_default(),
                        alpha: alpha__.unwrap_or_default(),
                        fl: fl__.unwrap_or_default(),
                        fr: fr__.unwrap_or_default(),
                    })
                }
            }
            deserializer
                .deserialize_struct(
                    "entity.DifferentialSample",
                    FIELDS,
                    GeneratedVisitor,
                )
        }
    }
    impl serde::Serialize for DifferentialTrajectory {
        #[allow(deprecated)]
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::SerializeStruct;
            let mut len = 0;
            if !self.samples.is_empty() {
                len += 1;
            }
            let mut struct_ser = serializer
                .serialize_struct("entity.DifferentialTrajectory", len)?;
            if !self.samples.is_empty() {
                struct_ser.serialize_field("samples", &self.samples)?;
            }
            struct_ser.end()
        }
    }
    impl<'de> serde::Deserialize<'de> for DifferentialTrajectory {
        #[allow(deprecated)]
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["samples"];
            #[allow(clippy::enum_variant_names)]
            enum GeneratedField {
                Samples,
            }
            impl<'de> serde::Deserialize<'de> for GeneratedField {
                fn deserialize<D>(
                    deserializer: D,
                ) -> std::result::Result<GeneratedField, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    struct GeneratedVisitor;
                    impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                        type Value = GeneratedField;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> std::fmt::Result {
                            formatter
                                .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                        }
                        #[allow(unused_variables)]
                        fn visit_str<E>(
                            self,
                            value: &str,
                        ) -> std::result::Result<GeneratedField, E>
                        where
                            E: serde::de::Error,
                        {
                            match value {
                                "samples" => Ok(GeneratedField::Samples),
                                _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                            }
                        }
                    }
                    deserializer.deserialize_identifier(GeneratedVisitor)
                }
            }
            struct GeneratedVisitor;
            impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                type Value = DifferentialTrajectory;
                fn expecting(
                    &self,
                    formatter: &mut std::fmt::Formatter<'_>,
                ) -> std::fmt::Result {
                    formatter.write_str("struct entity.DifferentialTrajectory")
                }
                fn visit_map<V>(
                    self,
                    mut map_: V,
                ) -> std::result::Result<DifferentialTrajectory, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
                {
                    let mut samples__ = None;
                    while let Some(k) = map_.next_key()? {
                        match k {
                            GeneratedField::Samples => {
                                if samples__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("samples"));
                                }
                                samples__ = Some(map_.next_value()?);
                            }
                        }
                    }
                    Ok(DifferentialTrajectory {
                        samples: samples__.unwrap_or_default(),
                    })
                }
            }
            deserializer
                .deserialize_struct(
                    "entity.DifferentialTrajectory",
                    FIELDS,
                    GeneratedVisitor,
                )
        }
    }
    impl serde::Serialize for DriveType {
        #[allow(deprecated)]
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let variant = match self {
                Self::DrivetypeSwerve => "DRIVETYPE_SWERVE",
                Self::DrivetypeDifferential => "DRIVETYPE_DIFFERENTIAL",
                Self::DrivetypeMecanum => "DRIVETYPE_MECANUM",
            };
            serializer.serialize_str(variant)
        }
    }
    impl<'de> serde::Deserialize<'de> for DriveType {
        #[allow(deprecated)]
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "DRIVETYPE_SWERVE",
                "DRIVETYPE_DIFFERENTIAL",
                "DRIVETYPE_MECANUM",
            ];
            struct GeneratedVisitor;
            impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                type Value = DriveType;
                fn expecting(
                    &self,
                    formatter: &mut std::fmt::Formatter<'_>,
                ) -> std::fmt::Result {
                    formatter.write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                }
                fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    i32::try_from(v)
                        .ok()
                        .and_then(|x| x.try_into().ok())
                        .ok_or_else(|| {
                            serde::de::Error::invalid_value(
                                serde::de::Unexpected::Signed(v),
                                &self,
                            )
                        })
                }
                fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    i32::try_from(v)
                        .ok()
                        .and_then(|x| x.try_into().ok())
                        .ok_or_else(|| {
                            serde::de::Error::invalid_value(
                                serde::de::Unexpected::Unsigned(v),
                                &self,
                            )
                        })
                }
                fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    match value {
                        "DRIVETYPE_SWERVE" => Ok(DriveType::DrivetypeSwerve),
                        "DRIVETYPE_DIFFERENTIAL" => Ok(DriveType::DrivetypeDifferential),
                        "DRIVETYPE_MECANUM" => Ok(DriveType::DrivetypeMecanum),
                        _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                    }
                }
            }
            deserializer.deserialize_any(GeneratedVisitor)
        }
    }
    impl serde::Serialize for ForceVector {
        #[allow(deprecated)]
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::SerializeStruct;
            let mut len = 0;
            if self.x != 0. {
                len += 1;
            }
            if self.y != 0. {
                len += 1;
            }
            let mut struct_ser = serializer.serialize_struct("entity.ForceVector", len)?;
            if self.x != 0. {
                struct_ser.serialize_field("x", &self.x)?;
            }
            if self.y != 0. {
                struct_ser.serialize_field("y", &self.y)?;
            }
            struct_ser.end()
        }
    }
    impl<'de> serde::Deserialize<'de> for ForceVector {
        #[allow(deprecated)]
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["x", "y"];
            #[allow(clippy::enum_variant_names)]
            enum GeneratedField {
                X,
                Y,
            }
            impl<'de> serde::Deserialize<'de> for GeneratedField {
                fn deserialize<D>(
                    deserializer: D,
                ) -> std::result::Result<GeneratedField, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    struct GeneratedVisitor;
                    impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                        type Value = GeneratedField;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> std::fmt::Result {
                            formatter
                                .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                        }
                        #[allow(unused_variables)]
                        fn visit_str<E>(
                            self,
                            value: &str,
                        ) -> std::result::Result<GeneratedField, E>
                        where
                            E: serde::de::Error,
                        {
                            match value {
                                "x" => Ok(GeneratedField::X),
                                "y" => Ok(GeneratedField::Y),
                                _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                            }
                        }
                    }
                    deserializer.deserialize_identifier(GeneratedVisitor)
                }
            }
            struct GeneratedVisitor;
            impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                type Value = ForceVector;
                fn expecting(
                    &self,
                    formatter: &mut std::fmt::Formatter<'_>,
                ) -> std::fmt::Result {
                    formatter.write_str("struct entity.ForceVector")
                }
                fn visit_map<V>(
                    self,
                    mut map_: V,
                ) -> std::result::Result<ForceVector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
                {
                    let mut x__ = None;
                    let mut y__ = None;
                    while let Some(k) = map_.next_key()? {
                        match k {
                            GeneratedField::X => {
                                if x__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("x"));
                                }
                                x__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Y => {
                                if y__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("y"));
                                }
                                y__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                        }
                    }
                    Ok(ForceVector {
                        x: x__.unwrap_or_default(),
                        y: y__.unwrap_or_default(),
                    })
                }
            }
            deserializer
                .deserialize_struct("entity.ForceVector", FIELDS, GeneratedVisitor)
        }
    }
    impl serde::Serialize for GenerationOutput {
        #[allow(deprecated)]
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::SerializeStruct;
            let mut len = 0;
            if !self.splits.is_empty() {
                len += 1;
            }
            if !self.waypoints.is_empty() {
                len += 1;
            }
            if self.config.is_some() {
                len += 1;
            }
            if self.trajectory.is_some() {
                len += 1;
            }
            let mut struct_ser = serializer
                .serialize_struct("entity.GenerationOutput", len)?;
            if !self.splits.is_empty() {
                struct_ser
                    .serialize_field(
                        "splits",
                        &self.splits.iter().map(ToString::to_string).collect::<Vec<_>>(),
                    )?;
            }
            if !self.waypoints.is_empty() {
                struct_ser.serialize_field("waypoints", &self.waypoints)?;
            }
            if let Some(v) = self.config.as_ref() {
                struct_ser.serialize_field("config", v)?;
            }
            if let Some(v) = self.trajectory.as_ref() {
                match v {
                    generation_output::Trajectory::Swerve(v) => {
                        struct_ser.serialize_field("swerve", v)?;
                    }
                    generation_output::Trajectory::Differential(v) => {
                        struct_ser.serialize_field("differential", v)?;
                    }
                }
            }
            struct_ser.end()
        }
    }
    impl<'de> serde::Deserialize<'de> for GenerationOutput {
        #[allow(deprecated)]
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "splits",
                "waypoints",
                "config",
                "swerve",
                "differential",
            ];
            #[allow(clippy::enum_variant_names)]
            enum GeneratedField {
                Splits,
                Waypoints,
                Config,
                Swerve,
                Differential,
            }
            impl<'de> serde::Deserialize<'de> for GeneratedField {
                fn deserialize<D>(
                    deserializer: D,
                ) -> std::result::Result<GeneratedField, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    struct GeneratedVisitor;
                    impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                        type Value = GeneratedField;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> std::fmt::Result {
                            formatter
                                .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                        }
                        #[allow(unused_variables)]
                        fn visit_str<E>(
                            self,
                            value: &str,
                        ) -> std::result::Result<GeneratedField, E>
                        where
                            E: serde::de::Error,
                        {
                            match value {
                                "splits" => Ok(GeneratedField::Splits),
                                "waypoints" => Ok(GeneratedField::Waypoints),
                                "config" => Ok(GeneratedField::Config),
                                "swerve" => Ok(GeneratedField::Swerve),
                                "differential" => Ok(GeneratedField::Differential),
                                _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                            }
                        }
                    }
                    deserializer.deserialize_identifier(GeneratedVisitor)
                }
            }
            struct GeneratedVisitor;
            impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                type Value = GenerationOutput;
                fn expecting(
                    &self,
                    formatter: &mut std::fmt::Formatter<'_>,
                ) -> std::fmt::Result {
                    formatter.write_str("struct entity.GenerationOutput")
                }
                fn visit_map<V>(
                    self,
                    mut map_: V,
                ) -> std::result::Result<GenerationOutput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
                {
                    let mut splits__ = None;
                    let mut waypoints__ = None;
                    let mut config__ = None;
                    let mut trajectory__ = None;
                    while let Some(k) = map_.next_key()? {
                        match k {
                            GeneratedField::Splits => {
                                if splits__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("splits"));
                                }
                                splits__ = Some(
                                    map_
                                        .next_value::<
                                            Vec<::pbjson::private::NumberDeserialize<_>>,
                                        >()?
                                        .into_iter()
                                        .map(|x| x.0)
                                        .collect(),
                                );
                            }
                            GeneratedField::Waypoints => {
                                if waypoints__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("waypoints"));
                                }
                                waypoints__ = Some(
                                    map_
                                        .next_value::<
                                            Vec<::pbjson::private::NumberDeserialize<_>>,
                                        >()?
                                        .into_iter()
                                        .map(|x| x.0)
                                        .collect(),
                                );
                            }
                            GeneratedField::Config => {
                                if config__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("config"));
                                }
                                config__ = map_.next_value()?;
                            }
                            GeneratedField::Swerve => {
                                if trajectory__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("swerve"));
                                }
                                trajectory__ = map_
                                    .next_value::<::std::option::Option<_>>()?
                                    .map(generation_output::Trajectory::Swerve);
                            }
                            GeneratedField::Differential => {
                                if trajectory__.is_some() {
                                    return Err(
                                        serde::de::Error::duplicate_field("differential"),
                                    );
                                }
                                trajectory__ = map_
                                    .next_value::<::std::option::Option<_>>()?
                                    .map(generation_output::Trajectory::Differential);
                            }
                        }
                    }
                    Ok(GenerationOutput {
                        splits: splits__.unwrap_or_default(),
                        waypoints: waypoints__.unwrap_or_default(),
                        config: config__,
                        trajectory: trajectory__,
                    })
                }
            }
            deserializer
                .deserialize_struct("entity.GenerationOutput", FIELDS, GeneratedVisitor)
        }
    }
    impl serde::Serialize for SwerveSample {
        #[allow(deprecated)]
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::SerializeStruct;
            let mut len = 0;
            if self.t != 0. {
                len += 1;
            }
            if self.x != 0. {
                len += 1;
            }
            if self.y != 0. {
                len += 1;
            }
            if self.heading != 0. {
                len += 1;
            }
            if self.vx != 0. {
                len += 1;
            }
            if self.vy != 0. {
                len += 1;
            }
            if self.omega != 0. {
                len += 1;
            }
            if self.ax != 0. {
                len += 1;
            }
            if self.ay != 0. {
                len += 1;
            }
            if self.alpha != 0. {
                len += 1;
            }
            if self.fl.is_some() {
                len += 1;
            }
            if self.fr.is_some() {
                len += 1;
            }
            if self.bl.is_some() {
                len += 1;
            }
            if self.br.is_some() {
                len += 1;
            }
            let mut struct_ser = serializer
                .serialize_struct("entity.SwerveSample", len)?;
            if self.t != 0. {
                struct_ser.serialize_field("t", &self.t)?;
            }
            if self.x != 0. {
                struct_ser.serialize_field("x", &self.x)?;
            }
            if self.y != 0. {
                struct_ser.serialize_field("y", &self.y)?;
            }
            if self.heading != 0. {
                struct_ser.serialize_field("heading", &self.heading)?;
            }
            if self.vx != 0. {
                struct_ser.serialize_field("vx", &self.vx)?;
            }
            if self.vy != 0. {
                struct_ser.serialize_field("vy", &self.vy)?;
            }
            if self.omega != 0. {
                struct_ser.serialize_field("omega", &self.omega)?;
            }
            if self.ax != 0. {
                struct_ser.serialize_field("ax", &self.ax)?;
            }
            if self.ay != 0. {
                struct_ser.serialize_field("ay", &self.ay)?;
            }
            if self.alpha != 0. {
                struct_ser.serialize_field("alpha", &self.alpha)?;
            }
            if let Some(v) = self.fl.as_ref() {
                struct_ser.serialize_field("fl", v)?;
            }
            if let Some(v) = self.fr.as_ref() {
                struct_ser.serialize_field("fr", v)?;
            }
            if let Some(v) = self.bl.as_ref() {
                struct_ser.serialize_field("bl", v)?;
            }
            if let Some(v) = self.br.as_ref() {
                struct_ser.serialize_field("br", v)?;
            }
            struct_ser.end()
        }
    }
    impl<'de> serde::Deserialize<'de> for SwerveSample {
        #[allow(deprecated)]
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "t",
                "x",
                "y",
                "heading",
                "vx",
                "vy",
                "omega",
                "ax",
                "ay",
                "alpha",
                "fl",
                "fr",
                "bl",
                "br",
            ];
            #[allow(clippy::enum_variant_names)]
            enum GeneratedField {
                T,
                X,
                Y,
                Heading,
                Vx,
                Vy,
                Omega,
                Ax,
                Ay,
                Alpha,
                Fl,
                Fr,
                Bl,
                Br,
            }
            impl<'de> serde::Deserialize<'de> for GeneratedField {
                fn deserialize<D>(
                    deserializer: D,
                ) -> std::result::Result<GeneratedField, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    struct GeneratedVisitor;
                    impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                        type Value = GeneratedField;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> std::fmt::Result {
                            formatter
                                .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                        }
                        #[allow(unused_variables)]
                        fn visit_str<E>(
                            self,
                            value: &str,
                        ) -> std::result::Result<GeneratedField, E>
                        where
                            E: serde::de::Error,
                        {
                            match value {
                                "t" => Ok(GeneratedField::T),
                                "x" => Ok(GeneratedField::X),
                                "y" => Ok(GeneratedField::Y),
                                "heading" => Ok(GeneratedField::Heading),
                                "vx" => Ok(GeneratedField::Vx),
                                "vy" => Ok(GeneratedField::Vy),
                                "omega" => Ok(GeneratedField::Omega),
                                "ax" => Ok(GeneratedField::Ax),
                                "ay" => Ok(GeneratedField::Ay),
                                "alpha" => Ok(GeneratedField::Alpha),
                                "fl" => Ok(GeneratedField::Fl),
                                "fr" => Ok(GeneratedField::Fr),
                                "bl" => Ok(GeneratedField::Bl),
                                "br" => Ok(GeneratedField::Br),
                                _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                            }
                        }
                    }
                    deserializer.deserialize_identifier(GeneratedVisitor)
                }
            }
            struct GeneratedVisitor;
            impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                type Value = SwerveSample;
                fn expecting(
                    &self,
                    formatter: &mut std::fmt::Formatter<'_>,
                ) -> std::fmt::Result {
                    formatter.write_str("struct entity.SwerveSample")
                }
                fn visit_map<V>(
                    self,
                    mut map_: V,
                ) -> std::result::Result<SwerveSample, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
                {
                    let mut t__ = None;
                    let mut x__ = None;
                    let mut y__ = None;
                    let mut heading__ = None;
                    let mut vx__ = None;
                    let mut vy__ = None;
                    let mut omega__ = None;
                    let mut ax__ = None;
                    let mut ay__ = None;
                    let mut alpha__ = None;
                    let mut fl__ = None;
                    let mut fr__ = None;
                    let mut bl__ = None;
                    let mut br__ = None;
                    while let Some(k) = map_.next_key()? {
                        match k {
                            GeneratedField::T => {
                                if t__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("t"));
                                }
                                t__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::X => {
                                if x__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("x"));
                                }
                                x__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Y => {
                                if y__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("y"));
                                }
                                y__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Heading => {
                                if heading__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("heading"));
                                }
                                heading__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Vx => {
                                if vx__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("vx"));
                                }
                                vx__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Vy => {
                                if vy__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("vy"));
                                }
                                vy__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Omega => {
                                if omega__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("omega"));
                                }
                                omega__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Ax => {
                                if ax__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("ax"));
                                }
                                ax__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Ay => {
                                if ay__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("ay"));
                                }
                                ay__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Alpha => {
                                if alpha__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("alpha"));
                                }
                                alpha__ = Some(
                                    map_
                                        .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                        .0,
                                );
                            }
                            GeneratedField::Fl => {
                                if fl__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fl"));
                                }
                                fl__ = map_.next_value()?;
                            }
                            GeneratedField::Fr => {
                                if fr__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fr"));
                                }
                                fr__ = map_.next_value()?;
                            }
                            GeneratedField::Bl => {
                                if bl__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("bl"));
                                }
                                bl__ = map_.next_value()?;
                            }
                            GeneratedField::Br => {
                                if br__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("br"));
                                }
                                br__ = map_.next_value()?;
                            }
                        }
                    }
                    Ok(SwerveSample {
                        t: t__.unwrap_or_default(),
                        x: x__.unwrap_or_default(),
                        y: y__.unwrap_or_default(),
                        heading: heading__.unwrap_or_default(),
                        vx: vx__.unwrap_or_default(),
                        vy: vy__.unwrap_or_default(),
                        omega: omega__.unwrap_or_default(),
                        ax: ax__.unwrap_or_default(),
                        ay: ay__.unwrap_or_default(),
                        alpha: alpha__.unwrap_or_default(),
                        fl: fl__,
                        fr: fr__,
                        bl: bl__,
                        br: br__,
                    })
                }
            }
            deserializer
                .deserialize_struct("entity.SwerveSample", FIELDS, GeneratedVisitor)
        }
    }
    impl serde::Serialize for SwerveTrajectory {
        #[allow(deprecated)]
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::SerializeStruct;
            let mut len = 0;
            if !self.samples.is_empty() {
                len += 1;
            }
            let mut struct_ser = serializer
                .serialize_struct("entity.SwerveTrajectory", len)?;
            if !self.samples.is_empty() {
                struct_ser.serialize_field("samples", &self.samples)?;
            }
            struct_ser.end()
        }
    }
    impl<'de> serde::Deserialize<'de> for SwerveTrajectory {
        #[allow(deprecated)]
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["samples"];
            #[allow(clippy::enum_variant_names)]
            enum GeneratedField {
                Samples,
            }
            impl<'de> serde::Deserialize<'de> for GeneratedField {
                fn deserialize<D>(
                    deserializer: D,
                ) -> std::result::Result<GeneratedField, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    struct GeneratedVisitor;
                    impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                        type Value = GeneratedField;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> std::fmt::Result {
                            formatter
                                .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                        }
                        #[allow(unused_variables)]
                        fn visit_str<E>(
                            self,
                            value: &str,
                        ) -> std::result::Result<GeneratedField, E>
                        where
                            E: serde::de::Error,
                        {
                            match value {
                                "samples" => Ok(GeneratedField::Samples),
                                _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                            }
                        }
                    }
                    deserializer.deserialize_identifier(GeneratedVisitor)
                }
            }
            struct GeneratedVisitor;
            impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                type Value = SwerveTrajectory;
                fn expecting(
                    &self,
                    formatter: &mut std::fmt::Formatter<'_>,
                ) -> std::fmt::Result {
                    formatter.write_str("struct entity.SwerveTrajectory")
                }
                fn visit_map<V>(
                    self,
                    mut map_: V,
                ) -> std::result::Result<SwerveTrajectory, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
                {
                    let mut samples__ = None;
                    while let Some(k) = map_.next_key()? {
                        match k {
                            GeneratedField::Samples => {
                                if samples__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("samples"));
                                }
                                samples__ = Some(map_.next_value()?);
                            }
                        }
                    }
                    Ok(SwerveTrajectory {
                        samples: samples__.unwrap_or_default(),
                    })
                }
            }
            deserializer
                .deserialize_struct("entity.SwerveTrajectory", FIELDS, GeneratedVisitor)
        }
    }
    impl serde::Serialize for TrajectoryFile {
        #[allow(deprecated)]
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::SerializeStruct;
            let mut len = 0;
            if !self.name.is_empty() {
                len += 1;
            }
            if self.params.is_some() {
                len += 1;
            }
            if self.snapshot.is_some() {
                len += 1;
            }
            if self.trajectory.is_some() {
                len += 1;
            }
            let mut struct_ser = serializer
                .serialize_struct("entity.TrajectoryFile", len)?;
            if !self.name.is_empty() {
                struct_ser.serialize_field("name", &self.name)?;
            }
            if let Some(v) = self.params.as_ref() {
                struct_ser.serialize_field("params", v)?;
            }
            if let Some(v) = self.snapshot.as_ref() {
                struct_ser.serialize_field("snapshot", v)?;
            }
            if let Some(v) = self.trajectory.as_ref() {
                struct_ser.serialize_field("trajectory", v)?;
            }
            struct_ser.end()
        }
    }
    impl<'de> serde::Deserialize<'de> for TrajectoryFile {
        #[allow(deprecated)]
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["name", "params", "snapshot", "trajectory"];
            #[allow(clippy::enum_variant_names)]
            enum GeneratedField {
                Name,
                Params,
                Snapshot,
                Trajectory,
            }
            impl<'de> serde::Deserialize<'de> for GeneratedField {
                fn deserialize<D>(
                    deserializer: D,
                ) -> std::result::Result<GeneratedField, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    struct GeneratedVisitor;
                    impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                        type Value = GeneratedField;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> std::fmt::Result {
                            formatter
                                .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                        }
                        #[allow(unused_variables)]
                        fn visit_str<E>(
                            self,
                            value: &str,
                        ) -> std::result::Result<GeneratedField, E>
                        where
                            E: serde::de::Error,
                        {
                            match value {
                                "name" => Ok(GeneratedField::Name),
                                "params" => Ok(GeneratedField::Params),
                                "snapshot" => Ok(GeneratedField::Snapshot),
                                "trajectory" => Ok(GeneratedField::Trajectory),
                                _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                            }
                        }
                    }
                    deserializer.deserialize_identifier(GeneratedVisitor)
                }
            }
            struct GeneratedVisitor;
            impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                type Value = TrajectoryFile;
                fn expecting(
                    &self,
                    formatter: &mut std::fmt::Formatter<'_>,
                ) -> std::fmt::Result {
                    formatter.write_str("struct entity.TrajectoryFile")
                }
                fn visit_map<V>(
                    self,
                    mut map_: V,
                ) -> std::result::Result<TrajectoryFile, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
                {
                    let mut name__ = None;
                    let mut params__ = None;
                    let mut snapshot__ = None;
                    let mut trajectory__ = None;
                    while let Some(k) = map_.next_key()? {
                        match k {
                            GeneratedField::Name => {
                                if name__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                name__ = Some(map_.next_value()?);
                            }
                            GeneratedField::Params => {
                                if params__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("params"));
                                }
                                params__ = map_.next_value()?;
                            }
                            GeneratedField::Snapshot => {
                                if snapshot__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("snapshot"));
                                }
                                snapshot__ = map_.next_value()?;
                            }
                            GeneratedField::Trajectory => {
                                if trajectory__.is_some() {
                                    return Err(serde::de::Error::duplicate_field("trajectory"));
                                }
                                trajectory__ = map_.next_value()?;
                            }
                        }
                    }
                    Ok(TrajectoryFile {
                        name: name__.unwrap_or_default(),
                        params: params__,
                        snapshot: snapshot__,
                        trajectory: trajectory__,
                    })
                }
            }
            deserializer
                .deserialize_struct("entity.TrajectoryFile", FIELDS, GeneratedVisitor)
        }
    }
    pub mod parameters {
        #[required(prefix = "Required")]
        pub struct WaypointIdFirst {}
        #[automatically_derived]
        impl ::core::clone::Clone for WaypointIdFirst {
            #[inline]
            fn clone(&self) -> WaypointIdFirst {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for WaypointIdFirst {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for WaypointIdFirst {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for WaypointIdFirst {
            #[inline]
            fn eq(&self, other: &WaypointIdFirst) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for WaypointIdFirst {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::hash::Hash for WaypointIdFirst {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
        }
        impl ::prost::Message for WaypointIdFirst {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {}
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
            }
            fn clear(&mut self) {}
        }
        impl ::core::default::Default for WaypointIdFirst {
            fn default() -> Self {
                WaypointIdFirst {}
            }
        }
        impl ::core::fmt::Debug for WaypointIdFirst {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("WaypointIdFirst");
                builder.finish()
            }
        }
        pub struct RequiredWaypointIdFirst {}
        impl From<RequiredWaypointIdFirst> for WaypointIdFirst {
            fn from(value: RequiredWaypointIdFirst) -> Self {
                WaypointIdFirst {}
            }
        }
        #[required(prefix = "Required")]
        pub struct WaypointIdLast {}
        #[automatically_derived]
        impl ::core::clone::Clone for WaypointIdLast {
            #[inline]
            fn clone(&self) -> WaypointIdLast {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for WaypointIdLast {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for WaypointIdLast {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for WaypointIdLast {
            #[inline]
            fn eq(&self, other: &WaypointIdLast) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for WaypointIdLast {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::hash::Hash for WaypointIdLast {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
        }
        impl ::prost::Message for WaypointIdLast {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {}
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
            }
            fn clear(&mut self) {}
        }
        impl ::core::default::Default for WaypointIdLast {
            fn default() -> Self {
                WaypointIdLast {}
            }
        }
        impl ::core::fmt::Debug for WaypointIdLast {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("WaypointIdLast");
                builder.finish()
            }
        }
        pub struct RequiredWaypointIdLast {}
        impl From<RequiredWaypointIdLast> for WaypointIdLast {
            fn from(value: RequiredWaypointIdLast) -> Self {
                WaypointIdLast {}
            }
        }
        #[required(prefix = "Required")]
        pub struct WaypointIdx {
            #[prost(uint64, tag = "1")]
            pub idx: u64,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for WaypointIdx {
            #[inline]
            fn clone(&self) -> WaypointIdx {
                let _: ::core::clone::AssertParamIsClone<u64>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for WaypointIdx {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for WaypointIdx {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for WaypointIdx {
            #[inline]
            fn eq(&self, other: &WaypointIdx) -> bool {
                self.idx == other.idx
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for WaypointIdx {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u64>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for WaypointIdx {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.idx, state)
            }
        }
        impl ::prost::Message for WaypointIdx {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if self.idx != 0u64 {
                    ::prost::encoding::uint64::encode(1u32, &self.idx, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "WaypointIdx";
                match tag {
                    1u32 => {
                        let mut value = &mut self.idx;
                        ::prost::encoding::uint64::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "idx");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.idx != 0u64 {
                        ::prost::encoding::uint64::encoded_len(1u32, &self.idx)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.idx = 0u64;
            }
        }
        impl ::core::default::Default for WaypointIdx {
            fn default() -> Self {
                WaypointIdx { idx: 0u64 }
            }
        }
        impl ::core::fmt::Debug for WaypointIdx {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("WaypointIdx");
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.idx)
                    };
                    builder.field("idx", &wrapper)
                };
                builder.finish()
            }
        }
        pub struct RequiredWaypointIdx {
            pub idx: u64,
        }
        impl From<RequiredWaypointIdx> for WaypointIdx {
            fn from(value: RequiredWaypointIdx) -> Self {
                WaypointIdx {
                    idx: value.idx.into(),
                }
            }
        }
        #[required(prefix = "Required")]
        pub struct WaypointId {
            #[prost(oneof = "waypoint_id::Id", tags = "1, 2, 3")]
            pub id: ::core::option::Option<waypoint_id::Id>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for WaypointId {
            #[inline]
            fn clone(&self) -> WaypointId {
                let _: ::core::clone::AssertParamIsClone<
                    ::core::option::Option<waypoint_id::Id>,
                >;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for WaypointId {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for WaypointId {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for WaypointId {
            #[inline]
            fn eq(&self, other: &WaypointId) -> bool {
                self.id == other.id
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for WaypointId {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<
                    ::core::option::Option<waypoint_id::Id>,
                >;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for WaypointId {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.id, state)
            }
        }
        impl ::prost::Message for WaypointId {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if let Some(ref oneof) = self.id {
                    oneof.encode(buf)
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "WaypointId";
                match tag {
                    1u32 | 2u32 | 3u32 => {
                        let mut value = &mut self.id;
                        waypoint_id::Id::merge(value, tag, wire_type, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "id");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + self.id.as_ref().map_or(0, waypoint_id::Id::encoded_len)
            }
            fn clear(&mut self) {
                self.id = ::core::option::Option::None;
            }
        }
        impl ::core::default::Default for WaypointId {
            fn default() -> Self {
                WaypointId {
                    id: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for WaypointId {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("WaypointId");
                let builder = {
                    let wrapper = &self.id;
                    builder.field("id", &wrapper)
                };
                builder.finish()
            }
        }
        pub struct RequiredWaypointId {
            pub id: waypoint_id::RequiredId,
        }
        impl From<RequiredWaypointId> for WaypointId {
            fn from(value: RequiredWaypointId) -> Self {
                WaypointId {
                    id: Some(value.id.into()),
                }
            }
        }
        /// Nested message and enum types in `WaypointID`.
        pub mod waypoint_id {
            #[required(prefix = "Required")]
            pub enum Id {
                #[prost(message, tag = "1")]
                First(super::WaypointIdFirst),
                #[prost(message, tag = "2")]
                Last(super::WaypointIdLast),
                #[prost(message, tag = "3")]
                Idx(super::WaypointIdx),
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Id {
                #[inline]
                fn clone(&self) -> Id {
                    let _: ::core::clone::AssertParamIsClone<super::WaypointIdFirst>;
                    let _: ::core::clone::AssertParamIsClone<super::WaypointIdLast>;
                    let _: ::core::clone::AssertParamIsClone<super::WaypointIdx>;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Id {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Id {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Id {
                #[inline]
                fn eq(&self, other: &Id) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                        && match (self, other) {
                            (Id::First(__self_0), Id::First(__arg1_0)) => {
                                __self_0 == __arg1_0
                            }
                            (Id::Last(__self_0), Id::Last(__arg1_0)) => {
                                __self_0 == __arg1_0
                            }
                            (Id::Idx(__self_0), Id::Idx(__arg1_0)) => {
                                __self_0 == __arg1_0
                            }
                            _ => unsafe { ::core::intrinsics::unreachable() }
                        }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for Id {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<super::WaypointIdFirst>;
                    let _: ::core::cmp::AssertParamIsEq<super::WaypointIdLast>;
                    let _: ::core::cmp::AssertParamIsEq<super::WaypointIdx>;
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for Id {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    ::core::hash::Hash::hash(&__self_discr, state);
                    match self {
                        Id::First(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                        Id::Last(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                        Id::Idx(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                    }
                }
            }
            impl Id {
                /// Encodes the message to a buffer.
                pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                    match *self {
                        Id::First(ref value) => {
                            ::prost::encoding::message::encode(1u32, &*value, buf);
                        }
                        Id::Last(ref value) => {
                            ::prost::encoding::message::encode(2u32, &*value, buf);
                        }
                        Id::Idx(ref value) => {
                            ::prost::encoding::message::encode(3u32, &*value, buf);
                        }
                    }
                }
                /// Decodes an instance of the message from a buffer, and merges it into self.
                pub fn merge(
                    field: &mut ::core::option::Option<Id>,
                    tag: u32,
                    wire_type: ::prost::encoding::wire_type::WireType,
                    buf: &mut impl ::prost::bytes::Buf,
                    ctx: ::prost::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::prost::DecodeError> {
                    match tag {
                        1u32 => {
                            if let ::core::option::Option::Some(Id::First(value)) = field {
                                ::prost::encoding::message::merge(
                                    wire_type,
                                    value,
                                    buf,
                                    ctx,
                                )
                            } else {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            Id::First(owned_value),
                                        );
                                    })
                            }
                        }
                        2u32 => {
                            if let ::core::option::Option::Some(Id::Last(value)) = field {
                                ::prost::encoding::message::merge(
                                    wire_type,
                                    value,
                                    buf,
                                    ctx,
                                )
                            } else {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(Id::Last(owned_value));
                                    })
                            }
                        }
                        3u32 => {
                            if let ::core::option::Option::Some(Id::Idx(value)) = field {
                                ::prost::encoding::message::merge(
                                    wire_type,
                                    value,
                                    buf,
                                    ctx,
                                )
                            } else {
                                let mut owned_value = ::core::default::Default::default();
                                let value = &mut owned_value;
                                ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(Id::Idx(owned_value));
                                    })
                            }
                        }
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "internal error: entered unreachable code: {0}",
                                    format_args!("invalid Id tag: {0}", tag),
                                ),
                            );
                        }
                    }
                }
                /// Returns the encoded length of the message without a length delimiter.
                #[inline]
                pub fn encoded_len(&self) -> usize {
                    match *self {
                        Id::First(ref value) => {
                            ::prost::encoding::message::encoded_len(1u32, &*value)
                        }
                        Id::Last(ref value) => {
                            ::prost::encoding::message::encoded_len(2u32, &*value)
                        }
                        Id::Idx(ref value) => {
                            ::prost::encoding::message::encoded_len(3u32, &*value)
                        }
                    }
                }
            }
            impl ::core::fmt::Debug for Id {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        Id::First(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("First").field(&wrapper).finish()
                        }
                        Id::Last(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("Last").field(&wrapper).finish()
                        }
                        Id::Idx(ref value) => {
                            let wrapper = &*value;
                            f.debug_tuple("Idx").field(&wrapper).finish()
                        }
                    }
                }
            }
            pub type RequiredId = Id;
        }
        #[required(prefix = "Required")]
        pub struct DoubleParameters {
            #[prost(double, tag = "1")]
            pub target_dt: f64,
            #[prost(message, repeated, tag = "2")]
            pub waypoints: ::prost::alloc::vec::Vec<waypoint::DoubleWaypoint>,
            #[prost(message, repeated, tag = "3")]
            pub constraints: ::prost::alloc::vec::Vec<constraint::DoubleConstraint>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DoubleParameters {
            #[inline]
            fn clone(&self) -> DoubleParameters {
                DoubleParameters {
                    target_dt: ::core::clone::Clone::clone(&self.target_dt),
                    waypoints: ::core::clone::Clone::clone(&self.waypoints),
                    constraints: ::core::clone::Clone::clone(&self.constraints),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DoubleParameters {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DoubleParameters {
            #[inline]
            fn eq(&self, other: &DoubleParameters) -> bool {
                self.target_dt == other.target_dt && self.waypoints == other.waypoints
                    && self.constraints == other.constraints
            }
        }
        impl ::prost::Message for DoubleParameters {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if self.target_dt != 0f64 {
                    ::prost::encoding::double::encode(1u32, &self.target_dt, buf);
                }
                for msg in &self.waypoints {
                    ::prost::encoding::message::encode(2u32, msg, buf);
                }
                for msg in &self.constraints {
                    ::prost::encoding::message::encode(3u32, msg, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "DoubleParameters";
                match tag {
                    1u32 => {
                        let mut value = &mut self.target_dt;
                        ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "target_dt");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.waypoints;
                        ::prost::encoding::message::merge_repeated(
                                wire_type,
                                value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "waypoints");
                                error
                            })
                    }
                    3u32 => {
                        let mut value = &mut self.constraints;
                        ::prost::encoding::message::merge_repeated(
                                wire_type,
                                value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "constraints");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.target_dt != 0f64 {
                        ::prost::encoding::double::encoded_len(1u32, &self.target_dt)
                    } else {
                        0
                    }
                    + ::prost::encoding::message::encoded_len_repeated(
                        2u32,
                        &self.waypoints,
                    )
                    + ::prost::encoding::message::encoded_len_repeated(
                        3u32,
                        &self.constraints,
                    )
            }
            fn clear(&mut self) {
                self.target_dt = 0f64;
                self.waypoints.clear();
                self.constraints.clear();
            }
        }
        impl ::core::default::Default for DoubleParameters {
            fn default() -> Self {
                DoubleParameters {
                    target_dt: 0f64,
                    waypoints: ::core::default::Default::default(),
                    constraints: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for DoubleParameters {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("DoubleParameters");
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.target_dt)
                    };
                    builder.field("target_dt", &wrapper)
                };
                let builder = {
                    let wrapper = &self.waypoints;
                    builder.field("waypoints", &wrapper)
                };
                let builder = {
                    let wrapper = &self.constraints;
                    builder.field("constraints", &wrapper)
                };
                builder.finish()
            }
        }
        pub struct RequiredDoubleParameters {
            pub target_dt: f64,
            pub waypoints: ::prost::alloc::vec::Vec<waypoint::DoubleWaypoint>,
            pub constraints: ::prost::alloc::vec::Vec<constraint::DoubleConstraint>,
        }
        impl From<RequiredDoubleParameters> for DoubleParameters {
            fn from(value: RequiredDoubleParameters) -> Self {
                DoubleParameters {
                    target_dt: value.target_dt.into(),
                    waypoints: value.waypoints.into(),
                    constraints: value.constraints.into(),
                }
            }
        }
        #[required(prefix = "Required")]
        pub struct Expr {
            #[prost(double, tag = "1")]
            pub value: f64,
            #[prost(string, tag = "2")]
            pub expr: ::prost::alloc::string::String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Expr {
            #[inline]
            fn clone(&self) -> Expr {
                Expr {
                    value: ::core::clone::Clone::clone(&self.value),
                    expr: ::core::clone::Clone::clone(&self.expr),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Expr {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Expr {
            #[inline]
            fn eq(&self, other: &Expr) -> bool {
                self.value == other.value && self.expr == other.expr
            }
        }
        impl ::prost::Message for Expr {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if self.value != 0f64 {
                    ::prost::encoding::double::encode(1u32, &self.value, buf);
                }
                if self.expr != "" {
                    ::prost::encoding::string::encode(2u32, &self.expr, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "Expr";
                match tag {
                    1u32 => {
                        let mut value = &mut self.value;
                        ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "value");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.expr;
                        ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "expr");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.value != 0f64 {
                        ::prost::encoding::double::encoded_len(1u32, &self.value)
                    } else {
                        0
                    }
                    + if self.expr != "" {
                        ::prost::encoding::string::encoded_len(2u32, &self.expr)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.value = 0f64;
                self.expr.clear();
            }
        }
        impl ::core::default::Default for Expr {
            fn default() -> Self {
                Expr {
                    value: 0f64,
                    expr: ::prost::alloc::string::String::new(),
                }
            }
        }
        impl ::core::fmt::Debug for Expr {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Expr");
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.value)
                    };
                    builder.field("value", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.expr)
                    };
                    builder.field("expr", &wrapper)
                };
                builder.finish()
            }
        }
        pub struct RequiredExpr {
            pub value: f64,
            pub expr: ::prost::alloc::string::String,
        }
        impl From<RequiredExpr> for Expr {
            fn from(value: RequiredExpr) -> Self {
                Expr {
                    value: value.value.into(),
                    expr: value.expr.into(),
                }
            }
        }
        #[required(prefix = "Required")]
        pub struct ExprParameters {
            #[prost(message, optional, tag = "1")]
            pub target_dt: ::core::option::Option<Expr>,
            #[prost(message, repeated, tag = "2")]
            pub waypoints: ::prost::alloc::vec::Vec<waypoint::ExprWaypoint>,
            #[prost(message, repeated, tag = "3")]
            pub constraints: ::prost::alloc::vec::Vec<constraint::ExprConstraint>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ExprParameters {
            #[inline]
            fn clone(&self) -> ExprParameters {
                ExprParameters {
                    target_dt: ::core::clone::Clone::clone(&self.target_dt),
                    waypoints: ::core::clone::Clone::clone(&self.waypoints),
                    constraints: ::core::clone::Clone::clone(&self.constraints),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ExprParameters {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ExprParameters {
            #[inline]
            fn eq(&self, other: &ExprParameters) -> bool {
                self.target_dt == other.target_dt && self.waypoints == other.waypoints
                    && self.constraints == other.constraints
            }
        }
        impl ::prost::Message for ExprParameters {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if let Some(ref msg) = self.target_dt {
                    ::prost::encoding::message::encode(1u32, msg, buf);
                }
                for msg in &self.waypoints {
                    ::prost::encoding::message::encode(2u32, msg, buf);
                }
                for msg in &self.constraints {
                    ::prost::encoding::message::encode(3u32, msg, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "ExprParameters";
                match tag {
                    1u32 => {
                        let mut value = &mut self.target_dt;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "target_dt");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.waypoints;
                        ::prost::encoding::message::merge_repeated(
                                wire_type,
                                value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "waypoints");
                                error
                            })
                    }
                    3u32 => {
                        let mut value = &mut self.constraints;
                        ::prost::encoding::message::merge_repeated(
                                wire_type,
                                value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "constraints");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + self
                        .target_dt
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(1u32, msg),
                        )
                    + ::prost::encoding::message::encoded_len_repeated(
                        2u32,
                        &self.waypoints,
                    )
                    + ::prost::encoding::message::encoded_len_repeated(
                        3u32,
                        &self.constraints,
                    )
            }
            fn clear(&mut self) {
                self.target_dt = ::core::option::Option::None;
                self.waypoints.clear();
                self.constraints.clear();
            }
        }
        impl ::core::default::Default for ExprParameters {
            fn default() -> Self {
                ExprParameters {
                    target_dt: ::core::default::Default::default(),
                    waypoints: ::core::default::Default::default(),
                    constraints: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for ExprParameters {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("ExprParameters");
                let builder = {
                    let wrapper = &self.target_dt;
                    builder.field("target_dt", &wrapper)
                };
                let builder = {
                    let wrapper = &self.waypoints;
                    builder.field("waypoints", &wrapper)
                };
                let builder = {
                    let wrapper = &self.constraints;
                    builder.field("constraints", &wrapper)
                };
                builder.finish()
            }
        }
        pub struct RequiredExprParameters {
            pub target_dt: RequiredExpr,
            pub waypoints: ::prost::alloc::vec::Vec<waypoint::ExprWaypoint>,
            pub constraints: ::prost::alloc::vec::Vec<constraint::ExprConstraint>,
        }
        impl From<RequiredExprParameters> for ExprParameters {
            fn from(value: RequiredExprParameters) -> Self {
                ExprParameters {
                    target_dt: Some(value.target_dt.into()),
                    waypoints: value.waypoints.into(),
                    constraints: value.constraints.into(),
                }
            }
        }
        impl serde::Serialize for DoubleParameters {
            #[allow(deprecated)]
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeStruct;
                let mut len = 0;
                if self.target_dt != 0. {
                    len += 1;
                }
                if !self.waypoints.is_empty() {
                    len += 1;
                }
                if !self.constraints.is_empty() {
                    len += 1;
                }
                let mut struct_ser = serializer
                    .serialize_struct("entity.parameters.DoubleParameters", len)?;
                if self.target_dt != 0. {
                    struct_ser.serialize_field("targetDt", &self.target_dt)?;
                }
                if !self.waypoints.is_empty() {
                    struct_ser.serialize_field("waypoints", &self.waypoints)?;
                }
                if !self.constraints.is_empty() {
                    struct_ser.serialize_field("constraints", &self.constraints)?;
                }
                struct_ser.end()
            }
        }
        impl<'de> serde::Deserialize<'de> for DoubleParameters {
            #[allow(deprecated)]
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                const FIELDS: &[&str] = &[
                    "target_dt",
                    "targetDt",
                    "waypoints",
                    "constraints",
                ];
                #[allow(clippy::enum_variant_names)]
                enum GeneratedField {
                    TargetDt,
                    Waypoints,
                    Constraints,
                }
                impl<'de> serde::Deserialize<'de> for GeneratedField {
                    fn deserialize<D>(
                        deserializer: D,
                    ) -> std::result::Result<GeneratedField, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        struct GeneratedVisitor;
                        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                            type Value = GeneratedField;
                            fn expecting(
                                &self,
                                formatter: &mut std::fmt::Formatter<'_>,
                            ) -> std::fmt::Result {
                                formatter
                                    .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                            }
                            #[allow(unused_variables)]
                            fn visit_str<E>(
                                self,
                                value: &str,
                            ) -> std::result::Result<GeneratedField, E>
                            where
                                E: serde::de::Error,
                            {
                                match value {
                                    "targetDt" | "target_dt" => Ok(GeneratedField::TargetDt),
                                    "waypoints" => Ok(GeneratedField::Waypoints),
                                    "constraints" => Ok(GeneratedField::Constraints),
                                    _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                }
                            }
                        }
                        deserializer.deserialize_identifier(GeneratedVisitor)
                    }
                }
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = DoubleParameters;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        formatter.write_str("struct entity.parameters.DoubleParameters")
                    }
                    fn visit_map<V>(
                        self,
                        mut map_: V,
                    ) -> std::result::Result<DoubleParameters, V::Error>
                    where
                        V: serde::de::MapAccess<'de>,
                    {
                        let mut target_dt__ = None;
                        let mut waypoints__ = None;
                        let mut constraints__ = None;
                        while let Some(k) = map_.next_key()? {
                            match k {
                                GeneratedField::TargetDt => {
                                    if target_dt__.is_some() {
                                        return Err(serde::de::Error::duplicate_field("targetDt"));
                                    }
                                    target_dt__ = Some(
                                        map_
                                            .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                            .0,
                                    );
                                }
                                GeneratedField::Waypoints => {
                                    if waypoints__.is_some() {
                                        return Err(serde::de::Error::duplicate_field("waypoints"));
                                    }
                                    waypoints__ = Some(map_.next_value()?);
                                }
                                GeneratedField::Constraints => {
                                    if constraints__.is_some() {
                                        return Err(
                                            serde::de::Error::duplicate_field("constraints"),
                                        );
                                    }
                                    constraints__ = Some(map_.next_value()?);
                                }
                            }
                        }
                        Ok(DoubleParameters {
                            target_dt: target_dt__.unwrap_or_default(),
                            waypoints: waypoints__.unwrap_or_default(),
                            constraints: constraints__.unwrap_or_default(),
                        })
                    }
                }
                deserializer
                    .deserialize_struct(
                        "entity.parameters.DoubleParameters",
                        FIELDS,
                        GeneratedVisitor,
                    )
            }
        }
        impl serde::Serialize for Expr {
            #[allow(deprecated)]
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeStruct;
                let mut len = 0;
                if self.value != 0. {
                    len += 1;
                }
                if !self.expr.is_empty() {
                    len += 1;
                }
                let mut struct_ser = serializer
                    .serialize_struct("entity.parameters.Expr", len)?;
                if self.value != 0. {
                    struct_ser.serialize_field("value", &self.value)?;
                }
                if !self.expr.is_empty() {
                    struct_ser.serialize_field("expr", &self.expr)?;
                }
                struct_ser.end()
            }
        }
        impl<'de> serde::Deserialize<'de> for Expr {
            #[allow(deprecated)]
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                const FIELDS: &[&str] = &["value", "expr"];
                #[allow(clippy::enum_variant_names)]
                enum GeneratedField {
                    Value,
                    Expr,
                }
                impl<'de> serde::Deserialize<'de> for GeneratedField {
                    fn deserialize<D>(
                        deserializer: D,
                    ) -> std::result::Result<GeneratedField, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        struct GeneratedVisitor;
                        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                            type Value = GeneratedField;
                            fn expecting(
                                &self,
                                formatter: &mut std::fmt::Formatter<'_>,
                            ) -> std::fmt::Result {
                                formatter
                                    .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                            }
                            #[allow(unused_variables)]
                            fn visit_str<E>(
                                self,
                                value: &str,
                            ) -> std::result::Result<GeneratedField, E>
                            where
                                E: serde::de::Error,
                            {
                                match value {
                                    "value" => Ok(GeneratedField::Value),
                                    "expr" => Ok(GeneratedField::Expr),
                                    _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                }
                            }
                        }
                        deserializer.deserialize_identifier(GeneratedVisitor)
                    }
                }
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = Expr;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        formatter.write_str("struct entity.parameters.Expr")
                    }
                    fn visit_map<V>(
                        self,
                        mut map_: V,
                    ) -> std::result::Result<Expr, V::Error>
                    where
                        V: serde::de::MapAccess<'de>,
                    {
                        let mut value__ = None;
                        let mut expr__ = None;
                        while let Some(k) = map_.next_key()? {
                            match k {
                                GeneratedField::Value => {
                                    if value__.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    value__ = Some(
                                        map_
                                            .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                            .0,
                                    );
                                }
                                GeneratedField::Expr => {
                                    if expr__.is_some() {
                                        return Err(serde::de::Error::duplicate_field("expr"));
                                    }
                                    expr__ = Some(map_.next_value()?);
                                }
                            }
                        }
                        Ok(Expr {
                            value: value__.unwrap_or_default(),
                            expr: expr__.unwrap_or_default(),
                        })
                    }
                }
                deserializer
                    .deserialize_struct(
                        "entity.parameters.Expr",
                        FIELDS,
                        GeneratedVisitor,
                    )
            }
        }
        impl serde::Serialize for ExprParameters {
            #[allow(deprecated)]
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeStruct;
                let mut len = 0;
                if self.target_dt.is_some() {
                    len += 1;
                }
                if !self.waypoints.is_empty() {
                    len += 1;
                }
                if !self.constraints.is_empty() {
                    len += 1;
                }
                let mut struct_ser = serializer
                    .serialize_struct("entity.parameters.ExprParameters", len)?;
                if let Some(v) = self.target_dt.as_ref() {
                    struct_ser.serialize_field("targetDt", v)?;
                }
                if !self.waypoints.is_empty() {
                    struct_ser.serialize_field("waypoints", &self.waypoints)?;
                }
                if !self.constraints.is_empty() {
                    struct_ser.serialize_field("constraints", &self.constraints)?;
                }
                struct_ser.end()
            }
        }
        impl<'de> serde::Deserialize<'de> for ExprParameters {
            #[allow(deprecated)]
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                const FIELDS: &[&str] = &[
                    "target_dt",
                    "targetDt",
                    "waypoints",
                    "constraints",
                ];
                #[allow(clippy::enum_variant_names)]
                enum GeneratedField {
                    TargetDt,
                    Waypoints,
                    Constraints,
                }
                impl<'de> serde::Deserialize<'de> for GeneratedField {
                    fn deserialize<D>(
                        deserializer: D,
                    ) -> std::result::Result<GeneratedField, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        struct GeneratedVisitor;
                        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                            type Value = GeneratedField;
                            fn expecting(
                                &self,
                                formatter: &mut std::fmt::Formatter<'_>,
                            ) -> std::fmt::Result {
                                formatter
                                    .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                            }
                            #[allow(unused_variables)]
                            fn visit_str<E>(
                                self,
                                value: &str,
                            ) -> std::result::Result<GeneratedField, E>
                            where
                                E: serde::de::Error,
                            {
                                match value {
                                    "targetDt" | "target_dt" => Ok(GeneratedField::TargetDt),
                                    "waypoints" => Ok(GeneratedField::Waypoints),
                                    "constraints" => Ok(GeneratedField::Constraints),
                                    _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                }
                            }
                        }
                        deserializer.deserialize_identifier(GeneratedVisitor)
                    }
                }
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = ExprParameters;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        formatter.write_str("struct entity.parameters.ExprParameters")
                    }
                    fn visit_map<V>(
                        self,
                        mut map_: V,
                    ) -> std::result::Result<ExprParameters, V::Error>
                    where
                        V: serde::de::MapAccess<'de>,
                    {
                        let mut target_dt__ = None;
                        let mut waypoints__ = None;
                        let mut constraints__ = None;
                        while let Some(k) = map_.next_key()? {
                            match k {
                                GeneratedField::TargetDt => {
                                    if target_dt__.is_some() {
                                        return Err(serde::de::Error::duplicate_field("targetDt"));
                                    }
                                    target_dt__ = map_.next_value()?;
                                }
                                GeneratedField::Waypoints => {
                                    if waypoints__.is_some() {
                                        return Err(serde::de::Error::duplicate_field("waypoints"));
                                    }
                                    waypoints__ = Some(map_.next_value()?);
                                }
                                GeneratedField::Constraints => {
                                    if constraints__.is_some() {
                                        return Err(
                                            serde::de::Error::duplicate_field("constraints"),
                                        );
                                    }
                                    constraints__ = Some(map_.next_value()?);
                                }
                            }
                        }
                        Ok(ExprParameters {
                            target_dt: target_dt__,
                            waypoints: waypoints__.unwrap_or_default(),
                            constraints: constraints__.unwrap_or_default(),
                        })
                    }
                }
                deserializer
                    .deserialize_struct(
                        "entity.parameters.ExprParameters",
                        FIELDS,
                        GeneratedVisitor,
                    )
            }
        }
        impl serde::Serialize for WaypointId {
            #[allow(deprecated)]
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeStruct;
                let mut len = 0;
                if self.id.is_some() {
                    len += 1;
                }
                let mut struct_ser = serializer
                    .serialize_struct("entity.parameters.WaypointID", len)?;
                if let Some(v) = self.id.as_ref() {
                    match v {
                        waypoint_id::Id::First(v) => {
                            struct_ser.serialize_field("first", v)?;
                        }
                        waypoint_id::Id::Last(v) => {
                            struct_ser.serialize_field("last", v)?;
                        }
                        waypoint_id::Id::Idx(v) => {
                            struct_ser.serialize_field("idx", v)?;
                        }
                    }
                }
                struct_ser.end()
            }
        }
        impl<'de> serde::Deserialize<'de> for WaypointId {
            #[allow(deprecated)]
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                const FIELDS: &[&str] = &["first", "last", "idx"];
                #[allow(clippy::enum_variant_names)]
                enum GeneratedField {
                    First,
                    Last,
                    Idx,
                }
                impl<'de> serde::Deserialize<'de> for GeneratedField {
                    fn deserialize<D>(
                        deserializer: D,
                    ) -> std::result::Result<GeneratedField, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        struct GeneratedVisitor;
                        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                            type Value = GeneratedField;
                            fn expecting(
                                &self,
                                formatter: &mut std::fmt::Formatter<'_>,
                            ) -> std::fmt::Result {
                                formatter
                                    .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                            }
                            #[allow(unused_variables)]
                            fn visit_str<E>(
                                self,
                                value: &str,
                            ) -> std::result::Result<GeneratedField, E>
                            where
                                E: serde::de::Error,
                            {
                                match value {
                                    "first" => Ok(GeneratedField::First),
                                    "last" => Ok(GeneratedField::Last),
                                    "idx" => Ok(GeneratedField::Idx),
                                    _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                }
                            }
                        }
                        deserializer.deserialize_identifier(GeneratedVisitor)
                    }
                }
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = WaypointId;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        formatter.write_str("struct entity.parameters.WaypointID")
                    }
                    fn visit_map<V>(
                        self,
                        mut map_: V,
                    ) -> std::result::Result<WaypointId, V::Error>
                    where
                        V: serde::de::MapAccess<'de>,
                    {
                        let mut id__ = None;
                        while let Some(k) = map_.next_key()? {
                            match k {
                                GeneratedField::First => {
                                    if id__.is_some() {
                                        return Err(serde::de::Error::duplicate_field("first"));
                                    }
                                    id__ = map_
                                        .next_value::<::std::option::Option<_>>()?
                                        .map(waypoint_id::Id::First);
                                }
                                GeneratedField::Last => {
                                    if id__.is_some() {
                                        return Err(serde::de::Error::duplicate_field("last"));
                                    }
                                    id__ = map_
                                        .next_value::<::std::option::Option<_>>()?
                                        .map(waypoint_id::Id::Last);
                                }
                                GeneratedField::Idx => {
                                    if id__.is_some() {
                                        return Err(serde::de::Error::duplicate_field("idx"));
                                    }
                                    id__ = map_
                                        .next_value::<::std::option::Option<_>>()?
                                        .map(waypoint_id::Id::Idx);
                                }
                            }
                        }
                        Ok(WaypointId { id: id__ })
                    }
                }
                deserializer
                    .deserialize_struct(
                        "entity.parameters.WaypointID",
                        FIELDS,
                        GeneratedVisitor,
                    )
            }
        }
        impl serde::Serialize for WaypointIdFirst {
            #[allow(deprecated)]
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeStruct;
                let len = 0;
                let struct_ser = serializer
                    .serialize_struct("entity.parameters.WaypointIDFirst", len)?;
                struct_ser.end()
            }
        }
        impl<'de> serde::Deserialize<'de> for WaypointIdFirst {
            #[allow(deprecated)]
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                const FIELDS: &[&str] = &[];
                #[allow(clippy::enum_variant_names)]
                enum GeneratedField {}
                impl<'de> serde::Deserialize<'de> for GeneratedField {
                    fn deserialize<D>(
                        deserializer: D,
                    ) -> std::result::Result<GeneratedField, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        struct GeneratedVisitor;
                        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                            type Value = GeneratedField;
                            fn expecting(
                                &self,
                                formatter: &mut std::fmt::Formatter<'_>,
                            ) -> std::fmt::Result {
                                formatter
                                    .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                            }
                            #[allow(unused_variables)]
                            fn visit_str<E>(
                                self,
                                value: &str,
                            ) -> std::result::Result<GeneratedField, E>
                            where
                                E: serde::de::Error,
                            {
                                Err(serde::de::Error::unknown_field(value, FIELDS))
                            }
                        }
                        deserializer.deserialize_identifier(GeneratedVisitor)
                    }
                }
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = WaypointIdFirst;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        formatter.write_str("struct entity.parameters.WaypointIDFirst")
                    }
                    fn visit_map<V>(
                        self,
                        mut map_: V,
                    ) -> std::result::Result<WaypointIdFirst, V::Error>
                    where
                        V: serde::de::MapAccess<'de>,
                    {
                        while map_.next_key::<GeneratedField>()?.is_some() {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                        Ok(WaypointIdFirst {})
                    }
                }
                deserializer
                    .deserialize_struct(
                        "entity.parameters.WaypointIDFirst",
                        FIELDS,
                        GeneratedVisitor,
                    )
            }
        }
        impl serde::Serialize for WaypointIdLast {
            #[allow(deprecated)]
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeStruct;
                let len = 0;
                let struct_ser = serializer
                    .serialize_struct("entity.parameters.WaypointIDLast", len)?;
                struct_ser.end()
            }
        }
        impl<'de> serde::Deserialize<'de> for WaypointIdLast {
            #[allow(deprecated)]
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                const FIELDS: &[&str] = &[];
                #[allow(clippy::enum_variant_names)]
                enum GeneratedField {}
                impl<'de> serde::Deserialize<'de> for GeneratedField {
                    fn deserialize<D>(
                        deserializer: D,
                    ) -> std::result::Result<GeneratedField, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        struct GeneratedVisitor;
                        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                            type Value = GeneratedField;
                            fn expecting(
                                &self,
                                formatter: &mut std::fmt::Formatter<'_>,
                            ) -> std::fmt::Result {
                                formatter
                                    .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                            }
                            #[allow(unused_variables)]
                            fn visit_str<E>(
                                self,
                                value: &str,
                            ) -> std::result::Result<GeneratedField, E>
                            where
                                E: serde::de::Error,
                            {
                                Err(serde::de::Error::unknown_field(value, FIELDS))
                            }
                        }
                        deserializer.deserialize_identifier(GeneratedVisitor)
                    }
                }
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = WaypointIdLast;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        formatter.write_str("struct entity.parameters.WaypointIDLast")
                    }
                    fn visit_map<V>(
                        self,
                        mut map_: V,
                    ) -> std::result::Result<WaypointIdLast, V::Error>
                    where
                        V: serde::de::MapAccess<'de>,
                    {
                        while map_.next_key::<GeneratedField>()?.is_some() {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                        Ok(WaypointIdLast {})
                    }
                }
                deserializer
                    .deserialize_struct(
                        "entity.parameters.WaypointIDLast",
                        FIELDS,
                        GeneratedVisitor,
                    )
            }
        }
        impl serde::Serialize for WaypointIdx {
            #[allow(deprecated)]
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeStruct;
                let mut len = 0;
                if self.idx != 0 {
                    len += 1;
                }
                let mut struct_ser = serializer
                    .serialize_struct("entity.parameters.WaypointIDX", len)?;
                if self.idx != 0 {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser
                        .serialize_field(
                            "idx",
                            ToString::to_string(&self.idx).as_str(),
                        )?;
                }
                struct_ser.end()
            }
        }
        impl<'de> serde::Deserialize<'de> for WaypointIdx {
            #[allow(deprecated)]
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                const FIELDS: &[&str] = &["idx"];
                #[allow(clippy::enum_variant_names)]
                enum GeneratedField {
                    Idx,
                }
                impl<'de> serde::Deserialize<'de> for GeneratedField {
                    fn deserialize<D>(
                        deserializer: D,
                    ) -> std::result::Result<GeneratedField, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        struct GeneratedVisitor;
                        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                            type Value = GeneratedField;
                            fn expecting(
                                &self,
                                formatter: &mut std::fmt::Formatter<'_>,
                            ) -> std::fmt::Result {
                                formatter
                                    .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                            }
                            #[allow(unused_variables)]
                            fn visit_str<E>(
                                self,
                                value: &str,
                            ) -> std::result::Result<GeneratedField, E>
                            where
                                E: serde::de::Error,
                            {
                                match value {
                                    "idx" => Ok(GeneratedField::Idx),
                                    _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                }
                            }
                        }
                        deserializer.deserialize_identifier(GeneratedVisitor)
                    }
                }
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = WaypointIdx;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        formatter.write_str("struct entity.parameters.WaypointIDX")
                    }
                    fn visit_map<V>(
                        self,
                        mut map_: V,
                    ) -> std::result::Result<WaypointIdx, V::Error>
                    where
                        V: serde::de::MapAccess<'de>,
                    {
                        let mut idx__ = None;
                        while let Some(k) = map_.next_key()? {
                            match k {
                                GeneratedField::Idx => {
                                    if idx__.is_some() {
                                        return Err(serde::de::Error::duplicate_field("idx"));
                                    }
                                    idx__ = Some(
                                        map_
                                            .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                            .0,
                                    );
                                }
                            }
                        }
                        Ok(WaypointIdx {
                            idx: idx__.unwrap_or_default(),
                        })
                    }
                }
                deserializer
                    .deserialize_struct(
                        "entity.parameters.WaypointIDX",
                        FIELDS,
                        GeneratedVisitor,
                    )
            }
        }
        pub mod constraint {
            #[required(prefix = "Required")]
            pub struct DoubleConstraint {
                #[prost(bool, tag = "1")]
                pub enabled: bool,
                #[prost(message, optional, tag = "2")]
                pub from: ::core::option::Option<super::WaypointId>,
                #[prost(message, optional, tag = "3")]
                pub to: ::core::option::Option<super::WaypointId>,
                /// ExprMaxVelocity maxvelocity = 4;
                /// DoubleMaxAcceleration max_acceleration = 5;
                #[prost(oneof = "double_constraint::Data", tags = "4, 5")]
                pub data: ::core::option::Option<double_constraint::Data>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for DoubleConstraint {
                #[inline]
                fn clone(&self) -> DoubleConstraint {
                    let _: ::core::clone::AssertParamIsClone<bool>;
                    let _: ::core::clone::AssertParamIsClone<
                        ::core::option::Option<super::WaypointId>,
                    >;
                    let _: ::core::clone::AssertParamIsClone<
                        ::core::option::Option<super::WaypointId>,
                    >;
                    let _: ::core::clone::AssertParamIsClone<
                        ::core::option::Option<double_constraint::Data>,
                    >;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for DoubleConstraint {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for DoubleConstraint {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for DoubleConstraint {
                #[inline]
                fn eq(&self, other: &DoubleConstraint) -> bool {
                    self.enabled == other.enabled && self.from == other.from
                        && self.to == other.to && self.data == other.data
                }
            }
            impl ::prost::Message for DoubleConstraint {
                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                    if self.enabled != false {
                        ::prost::encoding::bool::encode(1u32, &self.enabled, buf);
                    }
                    if let Some(ref msg) = self.from {
                        ::prost::encoding::message::encode(2u32, msg, buf);
                    }
                    if let Some(ref msg) = self.to {
                        ::prost::encoding::message::encode(3u32, msg, buf);
                    }
                    if let Some(ref oneof) = self.data {
                        oneof.encode(buf)
                    }
                }
                #[allow(unused_variables)]
                fn merge_field(
                    &mut self,
                    tag: u32,
                    wire_type: ::prost::encoding::wire_type::WireType,
                    buf: &mut impl ::prost::bytes::Buf,
                    ctx: ::prost::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::prost::DecodeError> {
                    const STRUCT_NAME: &'static str = "DoubleConstraint";
                    match tag {
                        1u32 => {
                            let mut value = &mut self.enabled;
                            ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "enabled");
                                    error
                                })
                        }
                        2u32 => {
                            let mut value = &mut self.from;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "from");
                                    error
                                })
                        }
                        3u32 => {
                            let mut value = &mut self.to;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "to");
                                    error
                                })
                        }
                        4u32 | 5u32 => {
                            let mut value = &mut self.data;
                            double_constraint::Data::merge(
                                    value,
                                    tag,
                                    wire_type,
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "data");
                                    error
                                })
                        }
                        _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0
                        + if self.enabled != false {
                            ::prost::encoding::bool::encoded_len(1u32, &self.enabled)
                        } else {
                            0
                        }
                        + self
                            .from
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(2u32, msg),
                            )
                        + self
                            .to
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(3u32, msg),
                            )
                        + self
                            .data
                            .as_ref()
                            .map_or(0, double_constraint::Data::encoded_len)
                }
                fn clear(&mut self) {
                    self.enabled = false;
                    self.from = ::core::option::Option::None;
                    self.to = ::core::option::Option::None;
                    self.data = ::core::option::Option::None;
                }
            }
            impl ::core::default::Default for DoubleConstraint {
                fn default() -> Self {
                    DoubleConstraint {
                        enabled: false,
                        from: ::core::default::Default::default(),
                        to: ::core::default::Default::default(),
                        data: ::core::default::Default::default(),
                    }
                }
            }
            impl ::core::fmt::Debug for DoubleConstraint {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let mut builder = f.debug_struct("DoubleConstraint");
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.enabled)
                        };
                        builder.field("enabled", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.from;
                        builder.field("from", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.to;
                        builder.field("to", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.data;
                        builder.field("data", &wrapper)
                    };
                    builder.finish()
                }
            }
            pub struct RequiredDoubleConstraint {
                pub enabled: bool,
                pub from: super::RequiredWaypointId,
                pub to: super::RequiredWaypointId,
                pub data: double_constraint::RequiredData,
            }
            impl From<RequiredDoubleConstraint> for DoubleConstraint {
                fn from(value: RequiredDoubleConstraint) -> Self {
                    DoubleConstraint {
                        enabled: value.enabled.into(),
                        from: Some(value.from.into()),
                        to: Some(value.to.into()),
                        data: Some(value.data.into()),
                    }
                }
            }
            /// Nested message and enum types in `DoubleConstraint`.
            pub mod double_constraint {
                /// ExprMaxVelocity maxvelocity = 4;
                /// DoubleMaxAcceleration max_acceleration = 5;
                #[required(prefix = "Required")]
                pub enum Data {
                    #[prost(message, tag = "4")]
                    MaxVelocity(super::maxvelocity::DoubleMaxVelocity),
                    #[prost(message, tag = "5")]
                    MaxAcceleration(super::max_acceleration::DoubleMaxAcceleration),
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Data {
                    #[inline]
                    fn clone(&self) -> Data {
                        let _: ::core::clone::AssertParamIsClone<
                            super::maxvelocity::DoubleMaxVelocity,
                        >;
                        let _: ::core::clone::AssertParamIsClone<
                            super::max_acceleration::DoubleMaxAcceleration,
                        >;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for Data {}
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for Data {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for Data {
                    #[inline]
                    fn eq(&self, other: &Data) -> bool {
                        let __self_discr = ::core::intrinsics::discriminant_value(self);
                        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                        __self_discr == __arg1_discr
                            && match (self, other) {
                                (
                                    Data::MaxVelocity(__self_0),
                                    Data::MaxVelocity(__arg1_0),
                                ) => __self_0 == __arg1_0,
                                (
                                    Data::MaxAcceleration(__self_0),
                                    Data::MaxAcceleration(__arg1_0),
                                ) => __self_0 == __arg1_0,
                                _ => unsafe { ::core::intrinsics::unreachable() }
                            }
                    }
                }
                impl Data {
                    /// Encodes the message to a buffer.
                    pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                        match *self {
                            Data::MaxVelocity(ref value) => {
                                ::prost::encoding::message::encode(4u32, &*value, buf);
                            }
                            Data::MaxAcceleration(ref value) => {
                                ::prost::encoding::message::encode(5u32, &*value, buf);
                            }
                        }
                    }
                    /// Decodes an instance of the message from a buffer, and merges it into self.
                    pub fn merge(
                        field: &mut ::core::option::Option<Data>,
                        tag: u32,
                        wire_type: ::prost::encoding::wire_type::WireType,
                        buf: &mut impl ::prost::bytes::Buf,
                        ctx: ::prost::encoding::DecodeContext,
                    ) -> ::core::result::Result<(), ::prost::DecodeError> {
                        match tag {
                            4u32 => {
                                if let ::core::option::Option::Some(
                                    Data::MaxVelocity(value),
                                ) = field {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                } else {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Data::MaxVelocity(owned_value),
                                            );
                                        })
                                }
                            }
                            5u32 => {
                                if let ::core::option::Option::Some(
                                    Data::MaxAcceleration(value),
                                ) = field {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                } else {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Data::MaxAcceleration(owned_value),
                                            );
                                        })
                                }
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("invalid Data tag: {0}", tag),
                                    ),
                                );
                            }
                        }
                    }
                    /// Returns the encoded length of the message without a length delimiter.
                    #[inline]
                    pub fn encoded_len(&self) -> usize {
                        match *self {
                            Data::MaxVelocity(ref value) => {
                                ::prost::encoding::message::encoded_len(4u32, &*value)
                            }
                            Data::MaxAcceleration(ref value) => {
                                ::prost::encoding::message::encoded_len(5u32, &*value)
                            }
                        }
                    }
                }
                impl ::core::fmt::Debug for Data {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        match *self {
                            Data::MaxVelocity(ref value) => {
                                let wrapper = &*value;
                                f.debug_tuple("MaxVelocity").field(&wrapper).finish()
                            }
                            Data::MaxAcceleration(ref value) => {
                                let wrapper = &*value;
                                f.debug_tuple("MaxAcceleration").field(&wrapper).finish()
                            }
                        }
                    }
                }
                pub type RequiredData = Data;
            }
            #[required(prefix = "Required")]
            pub struct ExprConstraint {
                #[prost(bool, tag = "1")]
                pub enabled: bool,
                #[prost(message, optional, tag = "2")]
                pub from: ::core::option::Option<super::WaypointId>,
                #[prost(message, optional, tag = "3")]
                pub to: ::core::option::Option<super::WaypointId>,
                /// ExprMaxVelocity maxvelocity = 4;
                /// ExprMaxAcceleration max_acceleration = 5;
                #[prost(oneof = "expr_constraint::Data", tags = "4, 5")]
                pub data: ::core::option::Option<expr_constraint::Data>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ExprConstraint {
                #[inline]
                fn clone(&self) -> ExprConstraint {
                    ExprConstraint {
                        enabled: ::core::clone::Clone::clone(&self.enabled),
                        from: ::core::clone::Clone::clone(&self.from),
                        to: ::core::clone::Clone::clone(&self.to),
                        data: ::core::clone::Clone::clone(&self.data),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ExprConstraint {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ExprConstraint {
                #[inline]
                fn eq(&self, other: &ExprConstraint) -> bool {
                    self.enabled == other.enabled && self.from == other.from
                        && self.to == other.to && self.data == other.data
                }
            }
            impl ::prost::Message for ExprConstraint {
                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                    if self.enabled != false {
                        ::prost::encoding::bool::encode(1u32, &self.enabled, buf);
                    }
                    if let Some(ref msg) = self.from {
                        ::prost::encoding::message::encode(2u32, msg, buf);
                    }
                    if let Some(ref msg) = self.to {
                        ::prost::encoding::message::encode(3u32, msg, buf);
                    }
                    if let Some(ref oneof) = self.data {
                        oneof.encode(buf)
                    }
                }
                #[allow(unused_variables)]
                fn merge_field(
                    &mut self,
                    tag: u32,
                    wire_type: ::prost::encoding::wire_type::WireType,
                    buf: &mut impl ::prost::bytes::Buf,
                    ctx: ::prost::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::prost::DecodeError> {
                    const STRUCT_NAME: &'static str = "ExprConstraint";
                    match tag {
                        1u32 => {
                            let mut value = &mut self.enabled;
                            ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "enabled");
                                    error
                                })
                        }
                        2u32 => {
                            let mut value = &mut self.from;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "from");
                                    error
                                })
                        }
                        3u32 => {
                            let mut value = &mut self.to;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "to");
                                    error
                                })
                        }
                        4u32 | 5u32 => {
                            let mut value = &mut self.data;
                            expr_constraint::Data::merge(value, tag, wire_type, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "data");
                                    error
                                })
                        }
                        _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0
                        + if self.enabled != false {
                            ::prost::encoding::bool::encoded_len(1u32, &self.enabled)
                        } else {
                            0
                        }
                        + self
                            .from
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(2u32, msg),
                            )
                        + self
                            .to
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(3u32, msg),
                            )
                        + self
                            .data
                            .as_ref()
                            .map_or(0, expr_constraint::Data::encoded_len)
                }
                fn clear(&mut self) {
                    self.enabled = false;
                    self.from = ::core::option::Option::None;
                    self.to = ::core::option::Option::None;
                    self.data = ::core::option::Option::None;
                }
            }
            impl ::core::default::Default for ExprConstraint {
                fn default() -> Self {
                    ExprConstraint {
                        enabled: false,
                        from: ::core::default::Default::default(),
                        to: ::core::default::Default::default(),
                        data: ::core::default::Default::default(),
                    }
                }
            }
            impl ::core::fmt::Debug for ExprConstraint {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let mut builder = f.debug_struct("ExprConstraint");
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.enabled)
                        };
                        builder.field("enabled", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.from;
                        builder.field("from", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.to;
                        builder.field("to", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.data;
                        builder.field("data", &wrapper)
                    };
                    builder.finish()
                }
            }
            pub struct RequiredExprConstraint {
                pub enabled: bool,
                pub from: super::RequiredWaypointId,
                pub to: super::RequiredWaypointId,
                pub data: expr_constraint::RequiredData,
            }
            impl From<RequiredExprConstraint> for ExprConstraint {
                fn from(value: RequiredExprConstraint) -> Self {
                    ExprConstraint {
                        enabled: value.enabled.into(),
                        from: Some(value.from.into()),
                        to: Some(value.to.into()),
                        data: Some(value.data.into()),
                    }
                }
            }
            /// Nested message and enum types in `ExprConstraint`.
            pub mod expr_constraint {
                /// ExprMaxVelocity maxvelocity = 4;
                /// ExprMaxAcceleration max_acceleration = 5;
                #[required(prefix = "Required")]
                pub enum Data {
                    #[prost(message, tag = "4")]
                    MaxVelocity(super::maxvelocity::ExprMaxVelocity),
                    #[prost(message, tag = "5")]
                    MaxAcceleration(super::max_acceleration::ExprMaxAcceleration),
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Data {
                    #[inline]
                    fn clone(&self) -> Data {
                        match self {
                            Data::MaxVelocity(__self_0) => {
                                Data::MaxVelocity(::core::clone::Clone::clone(__self_0))
                            }
                            Data::MaxAcceleration(__self_0) => {
                                Data::MaxAcceleration(::core::clone::Clone::clone(__self_0))
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for Data {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for Data {
                    #[inline]
                    fn eq(&self, other: &Data) -> bool {
                        let __self_discr = ::core::intrinsics::discriminant_value(self);
                        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                        __self_discr == __arg1_discr
                            && match (self, other) {
                                (
                                    Data::MaxVelocity(__self_0),
                                    Data::MaxVelocity(__arg1_0),
                                ) => __self_0 == __arg1_0,
                                (
                                    Data::MaxAcceleration(__self_0),
                                    Data::MaxAcceleration(__arg1_0),
                                ) => __self_0 == __arg1_0,
                                _ => unsafe { ::core::intrinsics::unreachable() }
                            }
                    }
                }
                impl Data {
                    /// Encodes the message to a buffer.
                    pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                        match *self {
                            Data::MaxVelocity(ref value) => {
                                ::prost::encoding::message::encode(4u32, &*value, buf);
                            }
                            Data::MaxAcceleration(ref value) => {
                                ::prost::encoding::message::encode(5u32, &*value, buf);
                            }
                        }
                    }
                    /// Decodes an instance of the message from a buffer, and merges it into self.
                    pub fn merge(
                        field: &mut ::core::option::Option<Data>,
                        tag: u32,
                        wire_type: ::prost::encoding::wire_type::WireType,
                        buf: &mut impl ::prost::bytes::Buf,
                        ctx: ::prost::encoding::DecodeContext,
                    ) -> ::core::result::Result<(), ::prost::DecodeError> {
                        match tag {
                            4u32 => {
                                if let ::core::option::Option::Some(
                                    Data::MaxVelocity(value),
                                ) = field {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                } else {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Data::MaxVelocity(owned_value),
                                            );
                                        })
                                }
                            }
                            5u32 => {
                                if let ::core::option::Option::Some(
                                    Data::MaxAcceleration(value),
                                ) = field {
                                    ::prost::encoding::message::merge(
                                        wire_type,
                                        value,
                                        buf,
                                        ctx,
                                    )
                                } else {
                                    let mut owned_value = ::core::default::Default::default();
                                    let value = &mut owned_value;
                                    ::prost::encoding::message::merge(
                                            wire_type,
                                            value,
                                            buf,
                                            ctx,
                                        )
                                        .map(|_| {
                                            *field = ::core::option::Option::Some(
                                                Data::MaxAcceleration(owned_value),
                                            );
                                        })
                                }
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("invalid Data tag: {0}", tag),
                                    ),
                                );
                            }
                        }
                    }
                    /// Returns the encoded length of the message without a length delimiter.
                    #[inline]
                    pub fn encoded_len(&self) -> usize {
                        match *self {
                            Data::MaxVelocity(ref value) => {
                                ::prost::encoding::message::encoded_len(4u32, &*value)
                            }
                            Data::MaxAcceleration(ref value) => {
                                ::prost::encoding::message::encoded_len(5u32, &*value)
                            }
                        }
                    }
                }
                impl ::core::fmt::Debug for Data {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        match *self {
                            Data::MaxVelocity(ref value) => {
                                let wrapper = &*value;
                                f.debug_tuple("MaxVelocity").field(&wrapper).finish()
                            }
                            Data::MaxAcceleration(ref value) => {
                                let wrapper = &*value;
                                f.debug_tuple("MaxAcceleration").field(&wrapper).finish()
                            }
                        }
                    }
                }
                pub type RequiredData = Data;
            }
            impl serde::Serialize for DoubleConstraint {
                #[allow(deprecated)]
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> std::result::Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeStruct;
                    let mut len = 0;
                    if self.enabled {
                        len += 1;
                    }
                    if self.from.is_some() {
                        len += 1;
                    }
                    if self.to.is_some() {
                        len += 1;
                    }
                    if self.data.is_some() {
                        len += 1;
                    }
                    let mut struct_ser = serializer
                        .serialize_struct(
                            "entity.parameters.constraint.DoubleConstraint",
                            len,
                        )?;
                    if self.enabled {
                        struct_ser.serialize_field("enabled", &self.enabled)?;
                    }
                    if let Some(v) = self.from.as_ref() {
                        struct_ser.serialize_field("from", v)?;
                    }
                    if let Some(v) = self.to.as_ref() {
                        struct_ser.serialize_field("to", v)?;
                    }
                    if let Some(v) = self.data.as_ref() {
                        match v {
                            double_constraint::Data::MaxVelocity(v) => {
                                struct_ser.serialize_field("maxVelocity", v)?;
                            }
                            double_constraint::Data::MaxAcceleration(v) => {
                                struct_ser.serialize_field("maxAcceleration", v)?;
                            }
                        }
                    }
                    struct_ser.end()
                }
            }
            impl<'de> serde::Deserialize<'de> for DoubleConstraint {
                #[allow(deprecated)]
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    const FIELDS: &[&str] = &[
                        "enabled",
                        "from",
                        "to",
                        "max_velocity",
                        "maxVelocity",
                        "max_acceleration",
                        "maxAcceleration",
                    ];
                    #[allow(clippy::enum_variant_names)]
                    enum GeneratedField {
                        Enabled,
                        From,
                        To,
                        MaxVelocity,
                        MaxAcceleration,
                    }
                    impl<'de> serde::Deserialize<'de> for GeneratedField {
                        fn deserialize<D>(
                            deserializer: D,
                        ) -> std::result::Result<GeneratedField, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            struct GeneratedVisitor;
                            impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                                type Value = GeneratedField;
                                fn expecting(
                                    &self,
                                    formatter: &mut std::fmt::Formatter<'_>,
                                ) -> std::fmt::Result {
                                    formatter
                                        .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                                }
                                #[allow(unused_variables)]
                                fn visit_str<E>(
                                    self,
                                    value: &str,
                                ) -> std::result::Result<GeneratedField, E>
                                where
                                    E: serde::de::Error,
                                {
                                    match value {
                                        "enabled" => Ok(GeneratedField::Enabled),
                                        "from" => Ok(GeneratedField::From),
                                        "to" => Ok(GeneratedField::To),
                                        "maxVelocity" | "max_velocity" => {
                                            Ok(GeneratedField::MaxVelocity)
                                        }
                                        "maxAcceleration" | "max_acceleration" => {
                                            Ok(GeneratedField::MaxAcceleration)
                                        }
                                        _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                    }
                                }
                            }
                            deserializer.deserialize_identifier(GeneratedVisitor)
                        }
                    }
                    struct GeneratedVisitor;
                    impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                        type Value = DoubleConstraint;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> std::fmt::Result {
                            formatter
                                .write_str(
                                    "struct entity.parameters.constraint.DoubleConstraint",
                                )
                        }
                        fn visit_map<V>(
                            self,
                            mut map_: V,
                        ) -> std::result::Result<DoubleConstraint, V::Error>
                        where
                            V: serde::de::MapAccess<'de>,
                        {
                            let mut enabled__ = None;
                            let mut from__ = None;
                            let mut to__ = None;
                            let mut data__ = None;
                            while let Some(k) = map_.next_key()? {
                                match k {
                                    GeneratedField::Enabled => {
                                        if enabled__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("enabled"));
                                        }
                                        enabled__ = Some(map_.next_value()?);
                                    }
                                    GeneratedField::From => {
                                        if from__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("from"));
                                        }
                                        from__ = map_.next_value()?;
                                    }
                                    GeneratedField::To => {
                                        if to__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("to"));
                                        }
                                        to__ = map_.next_value()?;
                                    }
                                    GeneratedField::MaxVelocity => {
                                        if data__.is_some() {
                                            return Err(
                                                serde::de::Error::duplicate_field("maxVelocity"),
                                            );
                                        }
                                        data__ = map_
                                            .next_value::<::std::option::Option<_>>()?
                                            .map(double_constraint::Data::MaxVelocity);
                                    }
                                    GeneratedField::MaxAcceleration => {
                                        if data__.is_some() {
                                            return Err(
                                                serde::de::Error::duplicate_field("maxAcceleration"),
                                            );
                                        }
                                        data__ = map_
                                            .next_value::<::std::option::Option<_>>()?
                                            .map(double_constraint::Data::MaxAcceleration);
                                    }
                                }
                            }
                            Ok(DoubleConstraint {
                                enabled: enabled__.unwrap_or_default(),
                                from: from__,
                                to: to__,
                                data: data__,
                            })
                        }
                    }
                    deserializer
                        .deserialize_struct(
                            "entity.parameters.constraint.DoubleConstraint",
                            FIELDS,
                            GeneratedVisitor,
                        )
                }
            }
            impl serde::Serialize for ExprConstraint {
                #[allow(deprecated)]
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> std::result::Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeStruct;
                    let mut len = 0;
                    if self.enabled {
                        len += 1;
                    }
                    if self.from.is_some() {
                        len += 1;
                    }
                    if self.to.is_some() {
                        len += 1;
                    }
                    if self.data.is_some() {
                        len += 1;
                    }
                    let mut struct_ser = serializer
                        .serialize_struct(
                            "entity.parameters.constraint.ExprConstraint",
                            len,
                        )?;
                    if self.enabled {
                        struct_ser.serialize_field("enabled", &self.enabled)?;
                    }
                    if let Some(v) = self.from.as_ref() {
                        struct_ser.serialize_field("from", v)?;
                    }
                    if let Some(v) = self.to.as_ref() {
                        struct_ser.serialize_field("to", v)?;
                    }
                    if let Some(v) = self.data.as_ref() {
                        match v {
                            expr_constraint::Data::MaxVelocity(v) => {
                                struct_ser.serialize_field("maxVelocity", v)?;
                            }
                            expr_constraint::Data::MaxAcceleration(v) => {
                                struct_ser.serialize_field("maxAcceleration", v)?;
                            }
                        }
                    }
                    struct_ser.end()
                }
            }
            impl<'de> serde::Deserialize<'de> for ExprConstraint {
                #[allow(deprecated)]
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    const FIELDS: &[&str] = &[
                        "enabled",
                        "from",
                        "to",
                        "max_velocity",
                        "maxVelocity",
                        "max_acceleration",
                        "maxAcceleration",
                    ];
                    #[allow(clippy::enum_variant_names)]
                    enum GeneratedField {
                        Enabled,
                        From,
                        To,
                        MaxVelocity,
                        MaxAcceleration,
                    }
                    impl<'de> serde::Deserialize<'de> for GeneratedField {
                        fn deserialize<D>(
                            deserializer: D,
                        ) -> std::result::Result<GeneratedField, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            struct GeneratedVisitor;
                            impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                                type Value = GeneratedField;
                                fn expecting(
                                    &self,
                                    formatter: &mut std::fmt::Formatter<'_>,
                                ) -> std::fmt::Result {
                                    formatter
                                        .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                                }
                                #[allow(unused_variables)]
                                fn visit_str<E>(
                                    self,
                                    value: &str,
                                ) -> std::result::Result<GeneratedField, E>
                                where
                                    E: serde::de::Error,
                                {
                                    match value {
                                        "enabled" => Ok(GeneratedField::Enabled),
                                        "from" => Ok(GeneratedField::From),
                                        "to" => Ok(GeneratedField::To),
                                        "maxVelocity" | "max_velocity" => {
                                            Ok(GeneratedField::MaxVelocity)
                                        }
                                        "maxAcceleration" | "max_acceleration" => {
                                            Ok(GeneratedField::MaxAcceleration)
                                        }
                                        _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                    }
                                }
                            }
                            deserializer.deserialize_identifier(GeneratedVisitor)
                        }
                    }
                    struct GeneratedVisitor;
                    impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                        type Value = ExprConstraint;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> std::fmt::Result {
                            formatter
                                .write_str(
                                    "struct entity.parameters.constraint.ExprConstraint",
                                )
                        }
                        fn visit_map<V>(
                            self,
                            mut map_: V,
                        ) -> std::result::Result<ExprConstraint, V::Error>
                        where
                            V: serde::de::MapAccess<'de>,
                        {
                            let mut enabled__ = None;
                            let mut from__ = None;
                            let mut to__ = None;
                            let mut data__ = None;
                            while let Some(k) = map_.next_key()? {
                                match k {
                                    GeneratedField::Enabled => {
                                        if enabled__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("enabled"));
                                        }
                                        enabled__ = Some(map_.next_value()?);
                                    }
                                    GeneratedField::From => {
                                        if from__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("from"));
                                        }
                                        from__ = map_.next_value()?;
                                    }
                                    GeneratedField::To => {
                                        if to__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("to"));
                                        }
                                        to__ = map_.next_value()?;
                                    }
                                    GeneratedField::MaxVelocity => {
                                        if data__.is_some() {
                                            return Err(
                                                serde::de::Error::duplicate_field("maxVelocity"),
                                            );
                                        }
                                        data__ = map_
                                            .next_value::<::std::option::Option<_>>()?
                                            .map(expr_constraint::Data::MaxVelocity);
                                    }
                                    GeneratedField::MaxAcceleration => {
                                        if data__.is_some() {
                                            return Err(
                                                serde::de::Error::duplicate_field("maxAcceleration"),
                                            );
                                        }
                                        data__ = map_
                                            .next_value::<::std::option::Option<_>>()?
                                            .map(expr_constraint::Data::MaxAcceleration);
                                    }
                                }
                            }
                            Ok(ExprConstraint {
                                enabled: enabled__.unwrap_or_default(),
                                from: from__,
                                to: to__,
                                data: data__,
                            })
                        }
                    }
                    deserializer
                        .deserialize_struct(
                            "entity.parameters.constraint.ExprConstraint",
                            FIELDS,
                            GeneratedVisitor,
                        )
                }
            }
            pub mod max_acceleration {
                #[required(prefix = "Required")]
                pub struct DoubleMaxAcceleration {
                    #[prost(double, tag = "1")]
                    pub max: f64,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for DoubleMaxAcceleration {
                    #[inline]
                    fn clone(&self) -> DoubleMaxAcceleration {
                        let _: ::core::clone::AssertParamIsClone<f64>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for DoubleMaxAcceleration {}
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for DoubleMaxAcceleration {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for DoubleMaxAcceleration {
                    #[inline]
                    fn eq(&self, other: &DoubleMaxAcceleration) -> bool {
                        self.max == other.max
                    }
                }
                impl ::prost::Message for DoubleMaxAcceleration {
                    #[allow(unused_variables)]
                    fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                        if self.max != 0f64 {
                            ::prost::encoding::double::encode(1u32, &self.max, buf);
                        }
                    }
                    #[allow(unused_variables)]
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        wire_type: ::prost::encoding::wire_type::WireType,
                        buf: &mut impl ::prost::bytes::Buf,
                        ctx: ::prost::encoding::DecodeContext,
                    ) -> ::core::result::Result<(), ::prost::DecodeError> {
                        const STRUCT_NAME: &'static str = "DoubleMaxAcceleration";
                        match tag {
                            1u32 => {
                                let mut value = &mut self.max;
                                ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                    .map_err(|mut error| {
                                        error.push(STRUCT_NAME, "max");
                                        error
                                    })
                            }
                            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                        }
                    }
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0
                            + if self.max != 0f64 {
                                ::prost::encoding::double::encoded_len(1u32, &self.max)
                            } else {
                                0
                            }
                    }
                    fn clear(&mut self) {
                        self.max = 0f64;
                    }
                }
                impl ::core::default::Default for DoubleMaxAcceleration {
                    fn default() -> Self {
                        DoubleMaxAcceleration { max: 0f64 }
                    }
                }
                impl ::core::fmt::Debug for DoubleMaxAcceleration {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        let mut builder = f.debug_struct("DoubleMaxAcceleration");
                        let builder = {
                            let wrapper = {
                                #[allow(non_snake_case)]
                                fn ScalarWrapper<T>(v: T) -> T {
                                    v
                                }
                                ScalarWrapper(&self.max)
                            };
                            builder.field("max", &wrapper)
                        };
                        builder.finish()
                    }
                }
                pub struct RequiredDoubleMaxAcceleration {
                    pub max: f64,
                }
                impl From<RequiredDoubleMaxAcceleration> for DoubleMaxAcceleration {
                    fn from(value: RequiredDoubleMaxAcceleration) -> Self {
                        DoubleMaxAcceleration {
                            max: value.max.into(),
                        }
                    }
                }
                #[required(prefix = "Required")]
                pub struct ExprMaxAcceleration {
                    #[prost(message, optional, tag = "1")]
                    pub max: ::core::option::Option<super::super::Expr>,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ExprMaxAcceleration {
                    #[inline]
                    fn clone(&self) -> ExprMaxAcceleration {
                        ExprMaxAcceleration {
                            max: ::core::clone::Clone::clone(&self.max),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for ExprMaxAcceleration {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for ExprMaxAcceleration {
                    #[inline]
                    fn eq(&self, other: &ExprMaxAcceleration) -> bool {
                        self.max == other.max
                    }
                }
                impl ::prost::Message for ExprMaxAcceleration {
                    #[allow(unused_variables)]
                    fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                        if let Some(ref msg) = self.max {
                            ::prost::encoding::message::encode(1u32, msg, buf);
                        }
                    }
                    #[allow(unused_variables)]
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        wire_type: ::prost::encoding::wire_type::WireType,
                        buf: &mut impl ::prost::bytes::Buf,
                        ctx: ::prost::encoding::DecodeContext,
                    ) -> ::core::result::Result<(), ::prost::DecodeError> {
                        const STRUCT_NAME: &'static str = "ExprMaxAcceleration";
                        match tag {
                            1u32 => {
                                let mut value = &mut self.max;
                                ::prost::encoding::message::merge(
                                        wire_type,
                                        value.get_or_insert_with(::core::default::Default::default),
                                        buf,
                                        ctx,
                                    )
                                    .map_err(|mut error| {
                                        error.push(STRUCT_NAME, "max");
                                        error
                                    })
                            }
                            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                        }
                    }
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0
                            + self
                                .max
                                .as_ref()
                                .map_or(
                                    0,
                                    |msg| ::prost::encoding::message::encoded_len(1u32, msg),
                                )
                    }
                    fn clear(&mut self) {
                        self.max = ::core::option::Option::None;
                    }
                }
                impl ::core::default::Default for ExprMaxAcceleration {
                    fn default() -> Self {
                        ExprMaxAcceleration {
                            max: ::core::default::Default::default(),
                        }
                    }
                }
                impl ::core::fmt::Debug for ExprMaxAcceleration {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        let mut builder = f.debug_struct("ExprMaxAcceleration");
                        let builder = {
                            let wrapper = &self.max;
                            builder.field("max", &wrapper)
                        };
                        builder.finish()
                    }
                }
                pub struct RequiredExprMaxAcceleration {
                    pub max: super::super::RequiredExpr,
                }
                impl From<RequiredExprMaxAcceleration> for ExprMaxAcceleration {
                    fn from(value: RequiredExprMaxAcceleration) -> Self {
                        ExprMaxAcceleration {
                            max: Some(value.max.into()),
                        }
                    }
                }
                impl serde::Serialize for DoubleMaxAcceleration {
                    #[allow(deprecated)]
                    fn serialize<S>(
                        &self,
                        serializer: S,
                    ) -> std::result::Result<S::Ok, S::Error>
                    where
                        S: serde::Serializer,
                    {
                        use serde::ser::SerializeStruct;
                        let mut len = 0;
                        if self.max != 0. {
                            len += 1;
                        }
                        let mut struct_ser = serializer
                            .serialize_struct(
                                "entity.parameters.constraint.max_acceleration.DoubleMaxAcceleration",
                                len,
                            )?;
                        if self.max != 0. {
                            struct_ser.serialize_field("max", &self.max)?;
                        }
                        struct_ser.end()
                    }
                }
                impl<'de> serde::Deserialize<'de> for DoubleMaxAcceleration {
                    #[allow(deprecated)]
                    fn deserialize<D>(
                        deserializer: D,
                    ) -> std::result::Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        const FIELDS: &[&str] = &["max"];
                        #[allow(clippy::enum_variant_names)]
                        enum GeneratedField {
                            Max,
                        }
                        impl<'de> serde::Deserialize<'de> for GeneratedField {
                            fn deserialize<D>(
                                deserializer: D,
                            ) -> std::result::Result<GeneratedField, D::Error>
                            where
                                D: serde::Deserializer<'de>,
                            {
                                struct GeneratedVisitor;
                                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                                    type Value = GeneratedField;
                                    fn expecting(
                                        &self,
                                        formatter: &mut std::fmt::Formatter<'_>,
                                    ) -> std::fmt::Result {
                                        formatter
                                            .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                                    }
                                    #[allow(unused_variables)]
                                    fn visit_str<E>(
                                        self,
                                        value: &str,
                                    ) -> std::result::Result<GeneratedField, E>
                                    where
                                        E: serde::de::Error,
                                    {
                                        match value {
                                            "max" => Ok(GeneratedField::Max),
                                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                        }
                                    }
                                }
                                deserializer.deserialize_identifier(GeneratedVisitor)
                            }
                        }
                        struct GeneratedVisitor;
                        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                            type Value = DoubleMaxAcceleration;
                            fn expecting(
                                &self,
                                formatter: &mut std::fmt::Formatter<'_>,
                            ) -> std::fmt::Result {
                                formatter
                                    .write_str(
                                        "struct entity.parameters.constraint.max_acceleration.DoubleMaxAcceleration",
                                    )
                            }
                            fn visit_map<V>(
                                self,
                                mut map_: V,
                            ) -> std::result::Result<DoubleMaxAcceleration, V::Error>
                            where
                                V: serde::de::MapAccess<'de>,
                            {
                                let mut max__ = None;
                                while let Some(k) = map_.next_key()? {
                                    match k {
                                        GeneratedField::Max => {
                                            if max__.is_some() {
                                                return Err(serde::de::Error::duplicate_field("max"));
                                            }
                                            max__ = Some(
                                                map_
                                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                    .0,
                                            );
                                        }
                                    }
                                }
                                Ok(DoubleMaxAcceleration {
                                    max: max__.unwrap_or_default(),
                                })
                            }
                        }
                        deserializer
                            .deserialize_struct(
                                "entity.parameters.constraint.max_acceleration.DoubleMaxAcceleration",
                                FIELDS,
                                GeneratedVisitor,
                            )
                    }
                }
                impl serde::Serialize for ExprMaxAcceleration {
                    #[allow(deprecated)]
                    fn serialize<S>(
                        &self,
                        serializer: S,
                    ) -> std::result::Result<S::Ok, S::Error>
                    where
                        S: serde::Serializer,
                    {
                        use serde::ser::SerializeStruct;
                        let mut len = 0;
                        if self.max.is_some() {
                            len += 1;
                        }
                        let mut struct_ser = serializer
                            .serialize_struct(
                                "entity.parameters.constraint.max_acceleration.ExprMaxAcceleration",
                                len,
                            )?;
                        if let Some(v) = self.max.as_ref() {
                            struct_ser.serialize_field("max", v)?;
                        }
                        struct_ser.end()
                    }
                }
                impl<'de> serde::Deserialize<'de> for ExprMaxAcceleration {
                    #[allow(deprecated)]
                    fn deserialize<D>(
                        deserializer: D,
                    ) -> std::result::Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        const FIELDS: &[&str] = &["max"];
                        #[allow(clippy::enum_variant_names)]
                        enum GeneratedField {
                            Max,
                        }
                        impl<'de> serde::Deserialize<'de> for GeneratedField {
                            fn deserialize<D>(
                                deserializer: D,
                            ) -> std::result::Result<GeneratedField, D::Error>
                            where
                                D: serde::Deserializer<'de>,
                            {
                                struct GeneratedVisitor;
                                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                                    type Value = GeneratedField;
                                    fn expecting(
                                        &self,
                                        formatter: &mut std::fmt::Formatter<'_>,
                                    ) -> std::fmt::Result {
                                        formatter
                                            .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                                    }
                                    #[allow(unused_variables)]
                                    fn visit_str<E>(
                                        self,
                                        value: &str,
                                    ) -> std::result::Result<GeneratedField, E>
                                    where
                                        E: serde::de::Error,
                                    {
                                        match value {
                                            "max" => Ok(GeneratedField::Max),
                                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                        }
                                    }
                                }
                                deserializer.deserialize_identifier(GeneratedVisitor)
                            }
                        }
                        struct GeneratedVisitor;
                        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                            type Value = ExprMaxAcceleration;
                            fn expecting(
                                &self,
                                formatter: &mut std::fmt::Formatter<'_>,
                            ) -> std::fmt::Result {
                                formatter
                                    .write_str(
                                        "struct entity.parameters.constraint.max_acceleration.ExprMaxAcceleration",
                                    )
                            }
                            fn visit_map<V>(
                                self,
                                mut map_: V,
                            ) -> std::result::Result<ExprMaxAcceleration, V::Error>
                            where
                                V: serde::de::MapAccess<'de>,
                            {
                                let mut max__ = None;
                                while let Some(k) = map_.next_key()? {
                                    match k {
                                        GeneratedField::Max => {
                                            if max__.is_some() {
                                                return Err(serde::de::Error::duplicate_field("max"));
                                            }
                                            max__ = map_.next_value()?;
                                        }
                                    }
                                }
                                Ok(ExprMaxAcceleration { max: max__ })
                            }
                        }
                        deserializer
                            .deserialize_struct(
                                "entity.parameters.constraint.max_acceleration.ExprMaxAcceleration",
                                FIELDS,
                                GeneratedVisitor,
                            )
                    }
                }
            }
            pub mod maxvelocity {
                #[required(prefix = "Required")]
                pub struct DoubleMaxVelocity {
                    #[prost(double, tag = "1")]
                    pub max: f64,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for DoubleMaxVelocity {
                    #[inline]
                    fn clone(&self) -> DoubleMaxVelocity {
                        let _: ::core::clone::AssertParamIsClone<f64>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for DoubleMaxVelocity {}
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for DoubleMaxVelocity {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for DoubleMaxVelocity {
                    #[inline]
                    fn eq(&self, other: &DoubleMaxVelocity) -> bool {
                        self.max == other.max
                    }
                }
                impl ::prost::Message for DoubleMaxVelocity {
                    #[allow(unused_variables)]
                    fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                        if self.max != 0f64 {
                            ::prost::encoding::double::encode(1u32, &self.max, buf);
                        }
                    }
                    #[allow(unused_variables)]
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        wire_type: ::prost::encoding::wire_type::WireType,
                        buf: &mut impl ::prost::bytes::Buf,
                        ctx: ::prost::encoding::DecodeContext,
                    ) -> ::core::result::Result<(), ::prost::DecodeError> {
                        const STRUCT_NAME: &'static str = "DoubleMaxVelocity";
                        match tag {
                            1u32 => {
                                let mut value = &mut self.max;
                                ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                    .map_err(|mut error| {
                                        error.push(STRUCT_NAME, "max");
                                        error
                                    })
                            }
                            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                        }
                    }
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0
                            + if self.max != 0f64 {
                                ::prost::encoding::double::encoded_len(1u32, &self.max)
                            } else {
                                0
                            }
                    }
                    fn clear(&mut self) {
                        self.max = 0f64;
                    }
                }
                impl ::core::default::Default for DoubleMaxVelocity {
                    fn default() -> Self {
                        DoubleMaxVelocity { max: 0f64 }
                    }
                }
                impl ::core::fmt::Debug for DoubleMaxVelocity {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        let mut builder = f.debug_struct("DoubleMaxVelocity");
                        let builder = {
                            let wrapper = {
                                #[allow(non_snake_case)]
                                fn ScalarWrapper<T>(v: T) -> T {
                                    v
                                }
                                ScalarWrapper(&self.max)
                            };
                            builder.field("max", &wrapper)
                        };
                        builder.finish()
                    }
                }
                pub struct RequiredDoubleMaxVelocity {
                    pub max: f64,
                }
                impl From<RequiredDoubleMaxVelocity> for DoubleMaxVelocity {
                    fn from(value: RequiredDoubleMaxVelocity) -> Self {
                        DoubleMaxVelocity {
                            max: value.max.into(),
                        }
                    }
                }
                #[required(prefix = "Required")]
                pub struct TestDouble {
                    #[prost(string, tag = "1")]
                    pub test: ::prost::alloc::string::String,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for TestDouble {
                    #[inline]
                    fn clone(&self) -> TestDouble {
                        TestDouble {
                            test: ::core::clone::Clone::clone(&self.test),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for TestDouble {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for TestDouble {
                    #[inline]
                    fn eq(&self, other: &TestDouble) -> bool {
                        self.test == other.test
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::Eq for TestDouble {
                    #[inline]
                    #[doc(hidden)]
                    #[coverage(off)]
                    fn assert_receiver_is_total_eq(&self) -> () {
                        let _: ::core::cmp::AssertParamIsEq<
                            ::prost::alloc::string::String,
                        >;
                    }
                }
                #[automatically_derived]
                impl ::core::hash::Hash for TestDouble {
                    #[inline]
                    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                        ::core::hash::Hash::hash(&self.test, state)
                    }
                }
                impl ::prost::Message for TestDouble {
                    #[allow(unused_variables)]
                    fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                        if self.test != "" {
                            ::prost::encoding::string::encode(1u32, &self.test, buf);
                        }
                    }
                    #[allow(unused_variables)]
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        wire_type: ::prost::encoding::wire_type::WireType,
                        buf: &mut impl ::prost::bytes::Buf,
                        ctx: ::prost::encoding::DecodeContext,
                    ) -> ::core::result::Result<(), ::prost::DecodeError> {
                        const STRUCT_NAME: &'static str = "TestDouble";
                        match tag {
                            1u32 => {
                                let mut value = &mut self.test;
                                ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                                    .map_err(|mut error| {
                                        error.push(STRUCT_NAME, "test");
                                        error
                                    })
                            }
                            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                        }
                    }
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0
                            + if self.test != "" {
                                ::prost::encoding::string::encoded_len(1u32, &self.test)
                            } else {
                                0
                            }
                    }
                    fn clear(&mut self) {
                        self.test.clear();
                    }
                }
                impl ::core::default::Default for TestDouble {
                    fn default() -> Self {
                        TestDouble {
                            test: ::prost::alloc::string::String::new(),
                        }
                    }
                }
                impl ::core::fmt::Debug for TestDouble {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        let mut builder = f.debug_struct("TestDouble");
                        let builder = {
                            let wrapper = {
                                #[allow(non_snake_case)]
                                fn ScalarWrapper<T>(v: T) -> T {
                                    v
                                }
                                ScalarWrapper(&self.test)
                            };
                            builder.field("test", &wrapper)
                        };
                        builder.finish()
                    }
                }
                pub struct RequiredTestDouble {
                    pub test: ::prost::alloc::string::String,
                }
                impl From<RequiredTestDouble> for TestDouble {
                    fn from(value: RequiredTestDouble) -> Self {
                        TestDouble {
                            test: value.test.into(),
                        }
                    }
                }
                #[required(prefix = "Required")]
                pub struct ExprMaxVelocity {
                    #[prost(message, optional, tag = "1")]
                    pub max: ::core::option::Option<super::super::Expr>,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ExprMaxVelocity {
                    #[inline]
                    fn clone(&self) -> ExprMaxVelocity {
                        ExprMaxVelocity {
                            max: ::core::clone::Clone::clone(&self.max),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for ExprMaxVelocity {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for ExprMaxVelocity {
                    #[inline]
                    fn eq(&self, other: &ExprMaxVelocity) -> bool {
                        self.max == other.max
                    }
                }
                impl ::prost::Message for ExprMaxVelocity {
                    #[allow(unused_variables)]
                    fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                        if let Some(ref msg) = self.max {
                            ::prost::encoding::message::encode(1u32, msg, buf);
                        }
                    }
                    #[allow(unused_variables)]
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        wire_type: ::prost::encoding::wire_type::WireType,
                        buf: &mut impl ::prost::bytes::Buf,
                        ctx: ::prost::encoding::DecodeContext,
                    ) -> ::core::result::Result<(), ::prost::DecodeError> {
                        const STRUCT_NAME: &'static str = "ExprMaxVelocity";
                        match tag {
                            1u32 => {
                                let mut value = &mut self.max;
                                ::prost::encoding::message::merge(
                                        wire_type,
                                        value.get_or_insert_with(::core::default::Default::default),
                                        buf,
                                        ctx,
                                    )
                                    .map_err(|mut error| {
                                        error.push(STRUCT_NAME, "max");
                                        error
                                    })
                            }
                            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                        }
                    }
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0
                            + self
                                .max
                                .as_ref()
                                .map_or(
                                    0,
                                    |msg| ::prost::encoding::message::encoded_len(1u32, msg),
                                )
                    }
                    fn clear(&mut self) {
                        self.max = ::core::option::Option::None;
                    }
                }
                impl ::core::default::Default for ExprMaxVelocity {
                    fn default() -> Self {
                        ExprMaxVelocity {
                            max: ::core::default::Default::default(),
                        }
                    }
                }
                impl ::core::fmt::Debug for ExprMaxVelocity {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        let mut builder = f.debug_struct("ExprMaxVelocity");
                        let builder = {
                            let wrapper = &self.max;
                            builder.field("max", &wrapper)
                        };
                        builder.finish()
                    }
                }
                pub struct RequiredExprMaxVelocity {
                    pub max: super::super::RequiredExpr,
                }
                impl From<RequiredExprMaxVelocity> for ExprMaxVelocity {
                    fn from(value: RequiredExprMaxVelocity) -> Self {
                        ExprMaxVelocity {
                            max: Some(value.max.into()),
                        }
                    }
                }
                #[required(prefix = "Required")]
                pub struct TestExpr {
                    #[prost(string, tag = "1")]
                    pub test: ::prost::alloc::string::String,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for TestExpr {
                    #[inline]
                    fn clone(&self) -> TestExpr {
                        TestExpr {
                            test: ::core::clone::Clone::clone(&self.test),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for TestExpr {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for TestExpr {
                    #[inline]
                    fn eq(&self, other: &TestExpr) -> bool {
                        self.test == other.test
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::Eq for TestExpr {
                    #[inline]
                    #[doc(hidden)]
                    #[coverage(off)]
                    fn assert_receiver_is_total_eq(&self) -> () {
                        let _: ::core::cmp::AssertParamIsEq<
                            ::prost::alloc::string::String,
                        >;
                    }
                }
                #[automatically_derived]
                impl ::core::hash::Hash for TestExpr {
                    #[inline]
                    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                        ::core::hash::Hash::hash(&self.test, state)
                    }
                }
                impl ::prost::Message for TestExpr {
                    #[allow(unused_variables)]
                    fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                        if self.test != "" {
                            ::prost::encoding::string::encode(1u32, &self.test, buf);
                        }
                    }
                    #[allow(unused_variables)]
                    fn merge_field(
                        &mut self,
                        tag: u32,
                        wire_type: ::prost::encoding::wire_type::WireType,
                        buf: &mut impl ::prost::bytes::Buf,
                        ctx: ::prost::encoding::DecodeContext,
                    ) -> ::core::result::Result<(), ::prost::DecodeError> {
                        const STRUCT_NAME: &'static str = "TestExpr";
                        match tag {
                            1u32 => {
                                let mut value = &mut self.test;
                                ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                                    .map_err(|mut error| {
                                        error.push(STRUCT_NAME, "test");
                                        error
                                    })
                            }
                            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                        }
                    }
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0
                            + if self.test != "" {
                                ::prost::encoding::string::encoded_len(1u32, &self.test)
                            } else {
                                0
                            }
                    }
                    fn clear(&mut self) {
                        self.test.clear();
                    }
                }
                impl ::core::default::Default for TestExpr {
                    fn default() -> Self {
                        TestExpr {
                            test: ::prost::alloc::string::String::new(),
                        }
                    }
                }
                impl ::core::fmt::Debug for TestExpr {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        let mut builder = f.debug_struct("TestExpr");
                        let builder = {
                            let wrapper = {
                                #[allow(non_snake_case)]
                                fn ScalarWrapper<T>(v: T) -> T {
                                    v
                                }
                                ScalarWrapper(&self.test)
                            };
                            builder.field("test", &wrapper)
                        };
                        builder.finish()
                    }
                }
                pub struct RequiredTestExpr {
                    pub test: ::prost::alloc::string::String,
                }
                impl From<RequiredTestExpr> for TestExpr {
                    fn from(value: RequiredTestExpr) -> Self {
                        TestExpr {
                            test: value.test.into(),
                        }
                    }
                }
                impl serde::Serialize for DoubleMaxVelocity {
                    #[allow(deprecated)]
                    fn serialize<S>(
                        &self,
                        serializer: S,
                    ) -> std::result::Result<S::Ok, S::Error>
                    where
                        S: serde::Serializer,
                    {
                        use serde::ser::SerializeStruct;
                        let mut len = 0;
                        if self.max != 0. {
                            len += 1;
                        }
                        let mut struct_ser = serializer
                            .serialize_struct(
                                "entity.parameters.constraint.maxvelocity.DoubleMaxVelocity",
                                len,
                            )?;
                        if self.max != 0. {
                            struct_ser.serialize_field("max", &self.max)?;
                        }
                        struct_ser.end()
                    }
                }
                impl<'de> serde::Deserialize<'de> for DoubleMaxVelocity {
                    #[allow(deprecated)]
                    fn deserialize<D>(
                        deserializer: D,
                    ) -> std::result::Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        const FIELDS: &[&str] = &["max"];
                        #[allow(clippy::enum_variant_names)]
                        enum GeneratedField {
                            Max,
                        }
                        impl<'de> serde::Deserialize<'de> for GeneratedField {
                            fn deserialize<D>(
                                deserializer: D,
                            ) -> std::result::Result<GeneratedField, D::Error>
                            where
                                D: serde::Deserializer<'de>,
                            {
                                struct GeneratedVisitor;
                                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                                    type Value = GeneratedField;
                                    fn expecting(
                                        &self,
                                        formatter: &mut std::fmt::Formatter<'_>,
                                    ) -> std::fmt::Result {
                                        formatter
                                            .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                                    }
                                    #[allow(unused_variables)]
                                    fn visit_str<E>(
                                        self,
                                        value: &str,
                                    ) -> std::result::Result<GeneratedField, E>
                                    where
                                        E: serde::de::Error,
                                    {
                                        match value {
                                            "max" => Ok(GeneratedField::Max),
                                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                        }
                                    }
                                }
                                deserializer.deserialize_identifier(GeneratedVisitor)
                            }
                        }
                        struct GeneratedVisitor;
                        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                            type Value = DoubleMaxVelocity;
                            fn expecting(
                                &self,
                                formatter: &mut std::fmt::Formatter<'_>,
                            ) -> std::fmt::Result {
                                formatter
                                    .write_str(
                                        "struct entity.parameters.constraint.maxvelocity.DoubleMaxVelocity",
                                    )
                            }
                            fn visit_map<V>(
                                self,
                                mut map_: V,
                            ) -> std::result::Result<DoubleMaxVelocity, V::Error>
                            where
                                V: serde::de::MapAccess<'de>,
                            {
                                let mut max__ = None;
                                while let Some(k) = map_.next_key()? {
                                    match k {
                                        GeneratedField::Max => {
                                            if max__.is_some() {
                                                return Err(serde::de::Error::duplicate_field("max"));
                                            }
                                            max__ = Some(
                                                map_
                                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                    .0,
                                            );
                                        }
                                    }
                                }
                                Ok(DoubleMaxVelocity {
                                    max: max__.unwrap_or_default(),
                                })
                            }
                        }
                        deserializer
                            .deserialize_struct(
                                "entity.parameters.constraint.maxvelocity.DoubleMaxVelocity",
                                FIELDS,
                                GeneratedVisitor,
                            )
                    }
                }
                impl serde::Serialize for ExprMaxVelocity {
                    #[allow(deprecated)]
                    fn serialize<S>(
                        &self,
                        serializer: S,
                    ) -> std::result::Result<S::Ok, S::Error>
                    where
                        S: serde::Serializer,
                    {
                        use serde::ser::SerializeStruct;
                        let mut len = 0;
                        if self.max.is_some() {
                            len += 1;
                        }
                        let mut struct_ser = serializer
                            .serialize_struct(
                                "entity.parameters.constraint.maxvelocity.ExprMaxVelocity",
                                len,
                            )?;
                        if let Some(v) = self.max.as_ref() {
                            struct_ser.serialize_field("max", v)?;
                        }
                        struct_ser.end()
                    }
                }
                impl<'de> serde::Deserialize<'de> for ExprMaxVelocity {
                    #[allow(deprecated)]
                    fn deserialize<D>(
                        deserializer: D,
                    ) -> std::result::Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        const FIELDS: &[&str] = &["max"];
                        #[allow(clippy::enum_variant_names)]
                        enum GeneratedField {
                            Max,
                        }
                        impl<'de> serde::Deserialize<'de> for GeneratedField {
                            fn deserialize<D>(
                                deserializer: D,
                            ) -> std::result::Result<GeneratedField, D::Error>
                            where
                                D: serde::Deserializer<'de>,
                            {
                                struct GeneratedVisitor;
                                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                                    type Value = GeneratedField;
                                    fn expecting(
                                        &self,
                                        formatter: &mut std::fmt::Formatter<'_>,
                                    ) -> std::fmt::Result {
                                        formatter
                                            .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                                    }
                                    #[allow(unused_variables)]
                                    fn visit_str<E>(
                                        self,
                                        value: &str,
                                    ) -> std::result::Result<GeneratedField, E>
                                    where
                                        E: serde::de::Error,
                                    {
                                        match value {
                                            "max" => Ok(GeneratedField::Max),
                                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                        }
                                    }
                                }
                                deserializer.deserialize_identifier(GeneratedVisitor)
                            }
                        }
                        struct GeneratedVisitor;
                        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                            type Value = ExprMaxVelocity;
                            fn expecting(
                                &self,
                                formatter: &mut std::fmt::Formatter<'_>,
                            ) -> std::fmt::Result {
                                formatter
                                    .write_str(
                                        "struct entity.parameters.constraint.maxvelocity.ExprMaxVelocity",
                                    )
                            }
                            fn visit_map<V>(
                                self,
                                mut map_: V,
                            ) -> std::result::Result<ExprMaxVelocity, V::Error>
                            where
                                V: serde::de::MapAccess<'de>,
                            {
                                let mut max__ = None;
                                while let Some(k) = map_.next_key()? {
                                    match k {
                                        GeneratedField::Max => {
                                            if max__.is_some() {
                                                return Err(serde::de::Error::duplicate_field("max"));
                                            }
                                            max__ = map_.next_value()?;
                                        }
                                    }
                                }
                                Ok(ExprMaxVelocity { max: max__ })
                            }
                        }
                        deserializer
                            .deserialize_struct(
                                "entity.parameters.constraint.maxvelocity.ExprMaxVelocity",
                                FIELDS,
                                GeneratedVisitor,
                            )
                    }
                }
                impl serde::Serialize for TestDouble {
                    #[allow(deprecated)]
                    fn serialize<S>(
                        &self,
                        serializer: S,
                    ) -> std::result::Result<S::Ok, S::Error>
                    where
                        S: serde::Serializer,
                    {
                        use serde::ser::SerializeStruct;
                        let mut len = 0;
                        if !self.test.is_empty() {
                            len += 1;
                        }
                        let mut struct_ser = serializer
                            .serialize_struct(
                                "entity.parameters.constraint.maxvelocity.TestDouble",
                                len,
                            )?;
                        if !self.test.is_empty() {
                            struct_ser.serialize_field("test", &self.test)?;
                        }
                        struct_ser.end()
                    }
                }
                impl<'de> serde::Deserialize<'de> for TestDouble {
                    #[allow(deprecated)]
                    fn deserialize<D>(
                        deserializer: D,
                    ) -> std::result::Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        const FIELDS: &[&str] = &["test"];
                        #[allow(clippy::enum_variant_names)]
                        enum GeneratedField {
                            Test,
                        }
                        impl<'de> serde::Deserialize<'de> for GeneratedField {
                            fn deserialize<D>(
                                deserializer: D,
                            ) -> std::result::Result<GeneratedField, D::Error>
                            where
                                D: serde::Deserializer<'de>,
                            {
                                struct GeneratedVisitor;
                                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                                    type Value = GeneratedField;
                                    fn expecting(
                                        &self,
                                        formatter: &mut std::fmt::Formatter<'_>,
                                    ) -> std::fmt::Result {
                                        formatter
                                            .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                                    }
                                    #[allow(unused_variables)]
                                    fn visit_str<E>(
                                        self,
                                        value: &str,
                                    ) -> std::result::Result<GeneratedField, E>
                                    where
                                        E: serde::de::Error,
                                    {
                                        match value {
                                            "test" => Ok(GeneratedField::Test),
                                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                        }
                                    }
                                }
                                deserializer.deserialize_identifier(GeneratedVisitor)
                            }
                        }
                        struct GeneratedVisitor;
                        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                            type Value = TestDouble;
                            fn expecting(
                                &self,
                                formatter: &mut std::fmt::Formatter<'_>,
                            ) -> std::fmt::Result {
                                formatter
                                    .write_str(
                                        "struct entity.parameters.constraint.maxvelocity.TestDouble",
                                    )
                            }
                            fn visit_map<V>(
                                self,
                                mut map_: V,
                            ) -> std::result::Result<TestDouble, V::Error>
                            where
                                V: serde::de::MapAccess<'de>,
                            {
                                let mut test__ = None;
                                while let Some(k) = map_.next_key()? {
                                    match k {
                                        GeneratedField::Test => {
                                            if test__.is_some() {
                                                return Err(serde::de::Error::duplicate_field("test"));
                                            }
                                            test__ = Some(map_.next_value()?);
                                        }
                                    }
                                }
                                Ok(TestDouble {
                                    test: test__.unwrap_or_default(),
                                })
                            }
                        }
                        deserializer
                            .deserialize_struct(
                                "entity.parameters.constraint.maxvelocity.TestDouble",
                                FIELDS,
                                GeneratedVisitor,
                            )
                    }
                }
                impl serde::Serialize for TestExpr {
                    #[allow(deprecated)]
                    fn serialize<S>(
                        &self,
                        serializer: S,
                    ) -> std::result::Result<S::Ok, S::Error>
                    where
                        S: serde::Serializer,
                    {
                        use serde::ser::SerializeStruct;
                        let mut len = 0;
                        if !self.test.is_empty() {
                            len += 1;
                        }
                        let mut struct_ser = serializer
                            .serialize_struct(
                                "entity.parameters.constraint.maxvelocity.TestExpr",
                                len,
                            )?;
                        if !self.test.is_empty() {
                            struct_ser.serialize_field("test", &self.test)?;
                        }
                        struct_ser.end()
                    }
                }
                impl<'de> serde::Deserialize<'de> for TestExpr {
                    #[allow(deprecated)]
                    fn deserialize<D>(
                        deserializer: D,
                    ) -> std::result::Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        const FIELDS: &[&str] = &["test"];
                        #[allow(clippy::enum_variant_names)]
                        enum GeneratedField {
                            Test,
                        }
                        impl<'de> serde::Deserialize<'de> for GeneratedField {
                            fn deserialize<D>(
                                deserializer: D,
                            ) -> std::result::Result<GeneratedField, D::Error>
                            where
                                D: serde::Deserializer<'de>,
                            {
                                struct GeneratedVisitor;
                                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                                    type Value = GeneratedField;
                                    fn expecting(
                                        &self,
                                        formatter: &mut std::fmt::Formatter<'_>,
                                    ) -> std::fmt::Result {
                                        formatter
                                            .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                                    }
                                    #[allow(unused_variables)]
                                    fn visit_str<E>(
                                        self,
                                        value: &str,
                                    ) -> std::result::Result<GeneratedField, E>
                                    where
                                        E: serde::de::Error,
                                    {
                                        match value {
                                            "test" => Ok(GeneratedField::Test),
                                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                        }
                                    }
                                }
                                deserializer.deserialize_identifier(GeneratedVisitor)
                            }
                        }
                        struct GeneratedVisitor;
                        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                            type Value = TestExpr;
                            fn expecting(
                                &self,
                                formatter: &mut std::fmt::Formatter<'_>,
                            ) -> std::fmt::Result {
                                formatter
                                    .write_str(
                                        "struct entity.parameters.constraint.maxvelocity.TestExpr",
                                    )
                            }
                            fn visit_map<V>(
                                self,
                                mut map_: V,
                            ) -> std::result::Result<TestExpr, V::Error>
                            where
                                V: serde::de::MapAccess<'de>,
                            {
                                let mut test__ = None;
                                while let Some(k) = map_.next_key()? {
                                    match k {
                                        GeneratedField::Test => {
                                            if test__.is_some() {
                                                return Err(serde::de::Error::duplicate_field("test"));
                                            }
                                            test__ = Some(map_.next_value()?);
                                        }
                                    }
                                }
                                Ok(TestExpr {
                                    test: test__.unwrap_or_default(),
                                })
                            }
                        }
                        deserializer
                            .deserialize_struct(
                                "entity.parameters.constraint.maxvelocity.TestExpr",
                                FIELDS,
                                GeneratedVisitor,
                            )
                    }
                }
            }
        }
        pub mod robotconfig {
            #[required(prefix = "Required")]
            pub struct DoubleModule {
                #[prost(double, tag = "1")]
                pub x: f64,
                #[prost(double, tag = "2")]
                pub y: f64,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for DoubleModule {
                #[inline]
                fn clone(&self) -> DoubleModule {
                    let _: ::core::clone::AssertParamIsClone<f64>;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for DoubleModule {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for DoubleModule {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for DoubleModule {
                #[inline]
                fn eq(&self, other: &DoubleModule) -> bool {
                    self.x == other.x && self.y == other.y
                }
            }
            impl ::prost::Message for DoubleModule {
                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                    if self.x != 0f64 {
                        ::prost::encoding::double::encode(1u32, &self.x, buf);
                    }
                    if self.y != 0f64 {
                        ::prost::encoding::double::encode(2u32, &self.y, buf);
                    }
                }
                #[allow(unused_variables)]
                fn merge_field(
                    &mut self,
                    tag: u32,
                    wire_type: ::prost::encoding::wire_type::WireType,
                    buf: &mut impl ::prost::bytes::Buf,
                    ctx: ::prost::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::prost::DecodeError> {
                    const STRUCT_NAME: &'static str = "DoubleModule";
                    match tag {
                        1u32 => {
                            let mut value = &mut self.x;
                            ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "x");
                                    error
                                })
                        }
                        2u32 => {
                            let mut value = &mut self.y;
                            ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "y");
                                    error
                                })
                        }
                        _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0
                        + if self.x != 0f64 {
                            ::prost::encoding::double::encoded_len(1u32, &self.x)
                        } else {
                            0
                        }
                        + if self.y != 0f64 {
                            ::prost::encoding::double::encoded_len(2u32, &self.y)
                        } else {
                            0
                        }
                }
                fn clear(&mut self) {
                    self.x = 0f64;
                    self.y = 0f64;
                }
            }
            impl ::core::default::Default for DoubleModule {
                fn default() -> Self {
                    DoubleModule { x: 0f64, y: 0f64 }
                }
            }
            impl ::core::fmt::Debug for DoubleModule {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let mut builder = f.debug_struct("DoubleModule");
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.x)
                        };
                        builder.field("x", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.y)
                        };
                        builder.field("y", &wrapper)
                    };
                    builder.finish()
                }
            }
            pub struct RequiredDoubleModule {
                pub x: f64,
                pub y: f64,
            }
            impl From<RequiredDoubleModule> for DoubleModule {
                fn from(value: RequiredDoubleModule) -> Self {
                    DoubleModule {
                        x: value.x.into(),
                        y: value.y.into(),
                    }
                }
            }
            #[required(prefix = "Required")]
            pub struct DoubleBumper {
                #[prost(double, tag = "1")]
                pub front: f64,
                #[prost(double, tag = "2")]
                pub left: f64,
                #[prost(double, tag = "3")]
                pub right: f64,
                #[prost(double, tag = "4")]
                pub back: f64,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for DoubleBumper {
                #[inline]
                fn clone(&self) -> DoubleBumper {
                    let _: ::core::clone::AssertParamIsClone<f64>;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for DoubleBumper {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for DoubleBumper {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for DoubleBumper {
                #[inline]
                fn eq(&self, other: &DoubleBumper) -> bool {
                    self.front == other.front && self.left == other.left
                        && self.right == other.right && self.back == other.back
                }
            }
            impl ::prost::Message for DoubleBumper {
                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                    if self.front != 0f64 {
                        ::prost::encoding::double::encode(1u32, &self.front, buf);
                    }
                    if self.left != 0f64 {
                        ::prost::encoding::double::encode(2u32, &self.left, buf);
                    }
                    if self.right != 0f64 {
                        ::prost::encoding::double::encode(3u32, &self.right, buf);
                    }
                    if self.back != 0f64 {
                        ::prost::encoding::double::encode(4u32, &self.back, buf);
                    }
                }
                #[allow(unused_variables)]
                fn merge_field(
                    &mut self,
                    tag: u32,
                    wire_type: ::prost::encoding::wire_type::WireType,
                    buf: &mut impl ::prost::bytes::Buf,
                    ctx: ::prost::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::prost::DecodeError> {
                    const STRUCT_NAME: &'static str = "DoubleBumper";
                    match tag {
                        1u32 => {
                            let mut value = &mut self.front;
                            ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "front");
                                    error
                                })
                        }
                        2u32 => {
                            let mut value = &mut self.left;
                            ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "left");
                                    error
                                })
                        }
                        3u32 => {
                            let mut value = &mut self.right;
                            ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "right");
                                    error
                                })
                        }
                        4u32 => {
                            let mut value = &mut self.back;
                            ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "back");
                                    error
                                })
                        }
                        _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0
                        + if self.front != 0f64 {
                            ::prost::encoding::double::encoded_len(1u32, &self.front)
                        } else {
                            0
                        }
                        + if self.left != 0f64 {
                            ::prost::encoding::double::encoded_len(2u32, &self.left)
                        } else {
                            0
                        }
                        + if self.right != 0f64 {
                            ::prost::encoding::double::encoded_len(3u32, &self.right)
                        } else {
                            0
                        }
                        + if self.back != 0f64 {
                            ::prost::encoding::double::encoded_len(4u32, &self.back)
                        } else {
                            0
                        }
                }
                fn clear(&mut self) {
                    self.front = 0f64;
                    self.left = 0f64;
                    self.right = 0f64;
                    self.back = 0f64;
                }
            }
            impl ::core::default::Default for DoubleBumper {
                fn default() -> Self {
                    DoubleBumper {
                        front: 0f64,
                        left: 0f64,
                        right: 0f64,
                        back: 0f64,
                    }
                }
            }
            impl ::core::fmt::Debug for DoubleBumper {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let mut builder = f.debug_struct("DoubleBumper");
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.front)
                        };
                        builder.field("front", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.left)
                        };
                        builder.field("left", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.right)
                        };
                        builder.field("right", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.back)
                        };
                        builder.field("back", &wrapper)
                    };
                    builder.finish()
                }
            }
            pub struct RequiredDoubleBumper {
                pub front: f64,
                pub left: f64,
                pub right: f64,
                pub back: f64,
            }
            impl From<RequiredDoubleBumper> for DoubleBumper {
                fn from(value: RequiredDoubleBumper) -> Self {
                    DoubleBumper {
                        front: value.front.into(),
                        left: value.left.into(),
                        right: value.right.into(),
                        back: value.back.into(),
                    }
                }
            }
            #[required(prefix = "Required")]
            pub struct DoubleRobotConfig {
                #[prost(double, tag = "1")]
                pub mass: f64,
                #[prost(double, tag = "2")]
                pub inertia: f64,
                #[prost(double, tag = "3")]
                pub gearing: f64,
                #[prost(double, tag = "4")]
                pub radius: f64,
                #[prost(double, tag = "5")]
                pub vmax: f64,
                #[prost(double, tag = "6")]
                pub tmax: f64,
                #[prost(double, tag = "7")]
                pub cof: f64,
                #[prost(double, tag = "8")]
                pub differential_track_width: f64,
                /// double bumper_front = 9;
                /// double bumper_left = 10;
                /// double bumper_back = 11;
                /// double bumper_right = 12;
                /// double fl_x = 13;
                /// double fl_y = 14;
                /// double fr_x = 15;
                /// double fl_
                #[prost(message, optional, tag = "9")]
                pub bumper: ::core::option::Option<DoubleBumper>,
                #[prost(message, optional, tag = "10")]
                pub front_left: ::core::option::Option<DoubleModule>,
                #[prost(message, optional, tag = "11")]
                pub front_right: ::core::option::Option<DoubleModule>,
                #[prost(message, optional, tag = "12")]
                pub back_left: ::core::option::Option<DoubleModule>,
                #[prost(message, optional, tag = "13")]
                pub back_right: ::core::option::Option<DoubleModule>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for DoubleRobotConfig {
                #[inline]
                fn clone(&self) -> DoubleRobotConfig {
                    let _: ::core::clone::AssertParamIsClone<f64>;
                    let _: ::core::clone::AssertParamIsClone<
                        ::core::option::Option<DoubleBumper>,
                    >;
                    let _: ::core::clone::AssertParamIsClone<
                        ::core::option::Option<DoubleModule>,
                    >;
                    let _: ::core::clone::AssertParamIsClone<
                        ::core::option::Option<DoubleModule>,
                    >;
                    let _: ::core::clone::AssertParamIsClone<
                        ::core::option::Option<DoubleModule>,
                    >;
                    let _: ::core::clone::AssertParamIsClone<
                        ::core::option::Option<DoubleModule>,
                    >;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for DoubleRobotConfig {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for DoubleRobotConfig {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for DoubleRobotConfig {
                #[inline]
                fn eq(&self, other: &DoubleRobotConfig) -> bool {
                    self.mass == other.mass && self.inertia == other.inertia
                        && self.gearing == other.gearing && self.radius == other.radius
                        && self.vmax == other.vmax && self.tmax == other.tmax
                        && self.cof == other.cof
                        && self.differential_track_width
                            == other.differential_track_width
                        && self.bumper == other.bumper
                        && self.front_left == other.front_left
                        && self.front_right == other.front_right
                        && self.back_left == other.back_left
                        && self.back_right == other.back_right
                }
            }
            impl ::prost::Message for DoubleRobotConfig {
                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                    if self.mass != 0f64 {
                        ::prost::encoding::double::encode(1u32, &self.mass, buf);
                    }
                    if self.inertia != 0f64 {
                        ::prost::encoding::double::encode(2u32, &self.inertia, buf);
                    }
                    if self.gearing != 0f64 {
                        ::prost::encoding::double::encode(3u32, &self.gearing, buf);
                    }
                    if self.radius != 0f64 {
                        ::prost::encoding::double::encode(4u32, &self.radius, buf);
                    }
                    if self.vmax != 0f64 {
                        ::prost::encoding::double::encode(5u32, &self.vmax, buf);
                    }
                    if self.tmax != 0f64 {
                        ::prost::encoding::double::encode(6u32, &self.tmax, buf);
                    }
                    if self.cof != 0f64 {
                        ::prost::encoding::double::encode(7u32, &self.cof, buf);
                    }
                    if self.differential_track_width != 0f64 {
                        ::prost::encoding::double::encode(
                            8u32,
                            &self.differential_track_width,
                            buf,
                        );
                    }
                    if let Some(ref msg) = self.bumper {
                        ::prost::encoding::message::encode(9u32, msg, buf);
                    }
                    if let Some(ref msg) = self.front_left {
                        ::prost::encoding::message::encode(10u32, msg, buf);
                    }
                    if let Some(ref msg) = self.front_right {
                        ::prost::encoding::message::encode(11u32, msg, buf);
                    }
                    if let Some(ref msg) = self.back_left {
                        ::prost::encoding::message::encode(12u32, msg, buf);
                    }
                    if let Some(ref msg) = self.back_right {
                        ::prost::encoding::message::encode(13u32, msg, buf);
                    }
                }
                #[allow(unused_variables)]
                fn merge_field(
                    &mut self,
                    tag: u32,
                    wire_type: ::prost::encoding::wire_type::WireType,
                    buf: &mut impl ::prost::bytes::Buf,
                    ctx: ::prost::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::prost::DecodeError> {
                    const STRUCT_NAME: &'static str = "DoubleRobotConfig";
                    match tag {
                        1u32 => {
                            let mut value = &mut self.mass;
                            ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "mass");
                                    error
                                })
                        }
                        2u32 => {
                            let mut value = &mut self.inertia;
                            ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "inertia");
                                    error
                                })
                        }
                        3u32 => {
                            let mut value = &mut self.gearing;
                            ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "gearing");
                                    error
                                })
                        }
                        4u32 => {
                            let mut value = &mut self.radius;
                            ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "radius");
                                    error
                                })
                        }
                        5u32 => {
                            let mut value = &mut self.vmax;
                            ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "vmax");
                                    error
                                })
                        }
                        6u32 => {
                            let mut value = &mut self.tmax;
                            ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "tmax");
                                    error
                                })
                        }
                        7u32 => {
                            let mut value = &mut self.cof;
                            ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "cof");
                                    error
                                })
                        }
                        8u32 => {
                            let mut value = &mut self.differential_track_width;
                            ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "differential_track_width");
                                    error
                                })
                        }
                        9u32 => {
                            let mut value = &mut self.bumper;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "bumper");
                                    error
                                })
                        }
                        10u32 => {
                            let mut value = &mut self.front_left;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "front_left");
                                    error
                                })
                        }
                        11u32 => {
                            let mut value = &mut self.front_right;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "front_right");
                                    error
                                })
                        }
                        12u32 => {
                            let mut value = &mut self.back_left;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "back_left");
                                    error
                                })
                        }
                        13u32 => {
                            let mut value = &mut self.back_right;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "back_right");
                                    error
                                })
                        }
                        _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0
                        + if self.mass != 0f64 {
                            ::prost::encoding::double::encoded_len(1u32, &self.mass)
                        } else {
                            0
                        }
                        + if self.inertia != 0f64 {
                            ::prost::encoding::double::encoded_len(2u32, &self.inertia)
                        } else {
                            0
                        }
                        + if self.gearing != 0f64 {
                            ::prost::encoding::double::encoded_len(3u32, &self.gearing)
                        } else {
                            0
                        }
                        + if self.radius != 0f64 {
                            ::prost::encoding::double::encoded_len(4u32, &self.radius)
                        } else {
                            0
                        }
                        + if self.vmax != 0f64 {
                            ::prost::encoding::double::encoded_len(5u32, &self.vmax)
                        } else {
                            0
                        }
                        + if self.tmax != 0f64 {
                            ::prost::encoding::double::encoded_len(6u32, &self.tmax)
                        } else {
                            0
                        }
                        + if self.cof != 0f64 {
                            ::prost::encoding::double::encoded_len(7u32, &self.cof)
                        } else {
                            0
                        }
                        + if self.differential_track_width != 0f64 {
                            ::prost::encoding::double::encoded_len(
                                8u32,
                                &self.differential_track_width,
                            )
                        } else {
                            0
                        }
                        + self
                            .bumper
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(9u32, msg),
                            )
                        + self
                            .front_left
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(10u32, msg),
                            )
                        + self
                            .front_right
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(11u32, msg),
                            )
                        + self
                            .back_left
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(12u32, msg),
                            )
                        + self
                            .back_right
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(13u32, msg),
                            )
                }
                fn clear(&mut self) {
                    self.mass = 0f64;
                    self.inertia = 0f64;
                    self.gearing = 0f64;
                    self.radius = 0f64;
                    self.vmax = 0f64;
                    self.tmax = 0f64;
                    self.cof = 0f64;
                    self.differential_track_width = 0f64;
                    self.bumper = ::core::option::Option::None;
                    self.front_left = ::core::option::Option::None;
                    self.front_right = ::core::option::Option::None;
                    self.back_left = ::core::option::Option::None;
                    self.back_right = ::core::option::Option::None;
                }
            }
            impl ::core::default::Default for DoubleRobotConfig {
                fn default() -> Self {
                    DoubleRobotConfig {
                        mass: 0f64,
                        inertia: 0f64,
                        gearing: 0f64,
                        radius: 0f64,
                        vmax: 0f64,
                        tmax: 0f64,
                        cof: 0f64,
                        differential_track_width: 0f64,
                        bumper: ::core::default::Default::default(),
                        front_left: ::core::default::Default::default(),
                        front_right: ::core::default::Default::default(),
                        back_left: ::core::default::Default::default(),
                        back_right: ::core::default::Default::default(),
                    }
                }
            }
            impl ::core::fmt::Debug for DoubleRobotConfig {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let mut builder = f.debug_struct("DoubleRobotConfig");
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.mass)
                        };
                        builder.field("mass", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.inertia)
                        };
                        builder.field("inertia", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.gearing)
                        };
                        builder.field("gearing", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.radius)
                        };
                        builder.field("radius", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.vmax)
                        };
                        builder.field("vmax", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.tmax)
                        };
                        builder.field("tmax", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.cof)
                        };
                        builder.field("cof", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.differential_track_width)
                        };
                        builder.field("differential_track_width", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.bumper;
                        builder.field("bumper", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.front_left;
                        builder.field("front_left", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.front_right;
                        builder.field("front_right", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.back_left;
                        builder.field("back_left", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.back_right;
                        builder.field("back_right", &wrapper)
                    };
                    builder.finish()
                }
            }
            pub struct RequiredDoubleRobotConfig {
                pub mass: f64,
                pub inertia: f64,
                pub gearing: f64,
                pub radius: f64,
                pub vmax: f64,
                pub tmax: f64,
                pub cof: f64,
                pub differential_track_width: f64,
                pub bumper: RequiredDoubleBumper,
                pub front_left: RequiredDoubleModule,
                pub front_right: RequiredDoubleModule,
                pub back_left: RequiredDoubleModule,
                pub back_right: RequiredDoubleModule,
            }
            impl From<RequiredDoubleRobotConfig> for DoubleRobotConfig {
                fn from(value: RequiredDoubleRobotConfig) -> Self {
                    DoubleRobotConfig {
                        mass: value.mass.into(),
                        inertia: value.inertia.into(),
                        gearing: value.gearing.into(),
                        radius: value.radius.into(),
                        vmax: value.vmax.into(),
                        tmax: value.tmax.into(),
                        cof: value.cof.into(),
                        differential_track_width: value.differential_track_width.into(),
                        bumper: Some(value.bumper.into()),
                        front_left: Some(value.front_left.into()),
                        front_right: Some(value.front_right.into()),
                        back_left: Some(value.back_left.into()),
                        back_right: Some(value.back_right.into()),
                    }
                }
            }
            #[required(prefix = "Required")]
            pub struct ExprModule {
                #[prost(message, optional, tag = "1")]
                pub x: ::core::option::Option<super::Expr>,
                #[prost(message, optional, tag = "2")]
                pub y: ::core::option::Option<super::Expr>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ExprModule {
                #[inline]
                fn clone(&self) -> ExprModule {
                    ExprModule {
                        x: ::core::clone::Clone::clone(&self.x),
                        y: ::core::clone::Clone::clone(&self.y),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ExprModule {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ExprModule {
                #[inline]
                fn eq(&self, other: &ExprModule) -> bool {
                    self.x == other.x && self.y == other.y
                }
            }
            impl ::prost::Message for ExprModule {
                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                    if let Some(ref msg) = self.x {
                        ::prost::encoding::message::encode(1u32, msg, buf);
                    }
                    if let Some(ref msg) = self.y {
                        ::prost::encoding::message::encode(2u32, msg, buf);
                    }
                }
                #[allow(unused_variables)]
                fn merge_field(
                    &mut self,
                    tag: u32,
                    wire_type: ::prost::encoding::wire_type::WireType,
                    buf: &mut impl ::prost::bytes::Buf,
                    ctx: ::prost::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::prost::DecodeError> {
                    const STRUCT_NAME: &'static str = "ExprModule";
                    match tag {
                        1u32 => {
                            let mut value = &mut self.x;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "x");
                                    error
                                })
                        }
                        2u32 => {
                            let mut value = &mut self.y;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "y");
                                    error
                                })
                        }
                        _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0
                        + self
                            .x
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(1u32, msg),
                            )
                        + self
                            .y
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(2u32, msg),
                            )
                }
                fn clear(&mut self) {
                    self.x = ::core::option::Option::None;
                    self.y = ::core::option::Option::None;
                }
            }
            impl ::core::default::Default for ExprModule {
                fn default() -> Self {
                    ExprModule {
                        x: ::core::default::Default::default(),
                        y: ::core::default::Default::default(),
                    }
                }
            }
            impl ::core::fmt::Debug for ExprModule {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let mut builder = f.debug_struct("ExprModule");
                    let builder = {
                        let wrapper = &self.x;
                        builder.field("x", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.y;
                        builder.field("y", &wrapper)
                    };
                    builder.finish()
                }
            }
            pub struct RequiredExprModule {
                pub x: super::RequiredExpr,
                pub y: super::RequiredExpr,
            }
            impl From<RequiredExprModule> for ExprModule {
                fn from(value: RequiredExprModule) -> Self {
                    ExprModule {
                        x: Some(value.x.into()),
                        y: Some(value.y.into()),
                    }
                }
            }
            #[required(prefix = "Required")]
            pub struct ExprBumper {
                #[prost(message, optional, tag = "1")]
                pub front: ::core::option::Option<super::Expr>,
                #[prost(message, optional, tag = "2")]
                pub left: ::core::option::Option<super::Expr>,
                #[prost(message, optional, tag = "3")]
                pub right: ::core::option::Option<super::Expr>,
                #[prost(message, optional, tag = "4")]
                pub back: ::core::option::Option<super::Expr>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ExprBumper {
                #[inline]
                fn clone(&self) -> ExprBumper {
                    ExprBumper {
                        front: ::core::clone::Clone::clone(&self.front),
                        left: ::core::clone::Clone::clone(&self.left),
                        right: ::core::clone::Clone::clone(&self.right),
                        back: ::core::clone::Clone::clone(&self.back),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ExprBumper {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ExprBumper {
                #[inline]
                fn eq(&self, other: &ExprBumper) -> bool {
                    self.front == other.front && self.left == other.left
                        && self.right == other.right && self.back == other.back
                }
            }
            impl ::prost::Message for ExprBumper {
                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                    if let Some(ref msg) = self.front {
                        ::prost::encoding::message::encode(1u32, msg, buf);
                    }
                    if let Some(ref msg) = self.left {
                        ::prost::encoding::message::encode(2u32, msg, buf);
                    }
                    if let Some(ref msg) = self.right {
                        ::prost::encoding::message::encode(3u32, msg, buf);
                    }
                    if let Some(ref msg) = self.back {
                        ::prost::encoding::message::encode(4u32, msg, buf);
                    }
                }
                #[allow(unused_variables)]
                fn merge_field(
                    &mut self,
                    tag: u32,
                    wire_type: ::prost::encoding::wire_type::WireType,
                    buf: &mut impl ::prost::bytes::Buf,
                    ctx: ::prost::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::prost::DecodeError> {
                    const STRUCT_NAME: &'static str = "ExprBumper";
                    match tag {
                        1u32 => {
                            let mut value = &mut self.front;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "front");
                                    error
                                })
                        }
                        2u32 => {
                            let mut value = &mut self.left;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "left");
                                    error
                                })
                        }
                        3u32 => {
                            let mut value = &mut self.right;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "right");
                                    error
                                })
                        }
                        4u32 => {
                            let mut value = &mut self.back;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "back");
                                    error
                                })
                        }
                        _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0
                        + self
                            .front
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(1u32, msg),
                            )
                        + self
                            .left
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(2u32, msg),
                            )
                        + self
                            .right
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(3u32, msg),
                            )
                        + self
                            .back
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(4u32, msg),
                            )
                }
                fn clear(&mut self) {
                    self.front = ::core::option::Option::None;
                    self.left = ::core::option::Option::None;
                    self.right = ::core::option::Option::None;
                    self.back = ::core::option::Option::None;
                }
            }
            impl ::core::default::Default for ExprBumper {
                fn default() -> Self {
                    ExprBumper {
                        front: ::core::default::Default::default(),
                        left: ::core::default::Default::default(),
                        right: ::core::default::Default::default(),
                        back: ::core::default::Default::default(),
                    }
                }
            }
            impl ::core::fmt::Debug for ExprBumper {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let mut builder = f.debug_struct("ExprBumper");
                    let builder = {
                        let wrapper = &self.front;
                        builder.field("front", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.left;
                        builder.field("left", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.right;
                        builder.field("right", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.back;
                        builder.field("back", &wrapper)
                    };
                    builder.finish()
                }
            }
            pub struct RequiredExprBumper {
                pub front: super::RequiredExpr,
                pub left: super::RequiredExpr,
                pub right: super::RequiredExpr,
                pub back: super::RequiredExpr,
            }
            impl From<RequiredExprBumper> for ExprBumper {
                fn from(value: RequiredExprBumper) -> Self {
                    ExprBumper {
                        front: Some(value.front.into()),
                        left: Some(value.left.into()),
                        right: Some(value.right.into()),
                        back: Some(value.back.into()),
                    }
                }
            }
            #[required(prefix = "Required")]
            pub struct ExprRobotConfig {
                #[prost(message, optional, tag = "1")]
                pub mass: ::core::option::Option<super::Expr>,
                #[prost(message, optional, tag = "2")]
                pub inertia: ::core::option::Option<super::Expr>,
                #[prost(message, optional, tag = "3")]
                pub gearing: ::core::option::Option<super::Expr>,
                #[prost(message, optional, tag = "4")]
                pub radius: ::core::option::Option<super::Expr>,
                #[prost(message, optional, tag = "5")]
                pub vmax: ::core::option::Option<super::Expr>,
                #[prost(message, optional, tag = "6")]
                pub tmax: ::core::option::Option<super::Expr>,
                #[prost(message, optional, tag = "7")]
                pub cof: ::core::option::Option<super::Expr>,
                #[prost(message, optional, tag = "8")]
                pub differential_track_width: ::core::option::Option<super::Expr>,
                /// Expr bumper_front = 9;
                /// Expr bumper_left = 10;
                /// Expr bumper_back = 11;
                /// Expr bumper_right = 12;
                /// Expr fl_x = 13;
                /// Expr fl_y = 14;
                /// Expr fr_x = 15;
                /// Expr fl_
                #[prost(message, optional, tag = "9")]
                pub bumper: ::core::option::Option<ExprBumper>,
                #[prost(message, optional, tag = "10")]
                pub front_left: ::core::option::Option<ExprModule>,
                #[prost(message, optional, tag = "11")]
                pub front_right: ::core::option::Option<ExprModule>,
                #[prost(message, optional, tag = "12")]
                pub back_left: ::core::option::Option<ExprModule>,
                #[prost(message, optional, tag = "13")]
                pub back_right: ::core::option::Option<ExprModule>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ExprRobotConfig {
                #[inline]
                fn clone(&self) -> ExprRobotConfig {
                    ExprRobotConfig {
                        mass: ::core::clone::Clone::clone(&self.mass),
                        inertia: ::core::clone::Clone::clone(&self.inertia),
                        gearing: ::core::clone::Clone::clone(&self.gearing),
                        radius: ::core::clone::Clone::clone(&self.radius),
                        vmax: ::core::clone::Clone::clone(&self.vmax),
                        tmax: ::core::clone::Clone::clone(&self.tmax),
                        cof: ::core::clone::Clone::clone(&self.cof),
                        differential_track_width: ::core::clone::Clone::clone(
                            &self.differential_track_width,
                        ),
                        bumper: ::core::clone::Clone::clone(&self.bumper),
                        front_left: ::core::clone::Clone::clone(&self.front_left),
                        front_right: ::core::clone::Clone::clone(&self.front_right),
                        back_left: ::core::clone::Clone::clone(&self.back_left),
                        back_right: ::core::clone::Clone::clone(&self.back_right),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ExprRobotConfig {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ExprRobotConfig {
                #[inline]
                fn eq(&self, other: &ExprRobotConfig) -> bool {
                    self.mass == other.mass && self.inertia == other.inertia
                        && self.gearing == other.gearing && self.radius == other.radius
                        && self.vmax == other.vmax && self.tmax == other.tmax
                        && self.cof == other.cof
                        && self.differential_track_width
                            == other.differential_track_width
                        && self.bumper == other.bumper
                        && self.front_left == other.front_left
                        && self.front_right == other.front_right
                        && self.back_left == other.back_left
                        && self.back_right == other.back_right
                }
            }
            impl ::prost::Message for ExprRobotConfig {
                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                    if let Some(ref msg) = self.mass {
                        ::prost::encoding::message::encode(1u32, msg, buf);
                    }
                    if let Some(ref msg) = self.inertia {
                        ::prost::encoding::message::encode(2u32, msg, buf);
                    }
                    if let Some(ref msg) = self.gearing {
                        ::prost::encoding::message::encode(3u32, msg, buf);
                    }
                    if let Some(ref msg) = self.radius {
                        ::prost::encoding::message::encode(4u32, msg, buf);
                    }
                    if let Some(ref msg) = self.vmax {
                        ::prost::encoding::message::encode(5u32, msg, buf);
                    }
                    if let Some(ref msg) = self.tmax {
                        ::prost::encoding::message::encode(6u32, msg, buf);
                    }
                    if let Some(ref msg) = self.cof {
                        ::prost::encoding::message::encode(7u32, msg, buf);
                    }
                    if let Some(ref msg) = self.differential_track_width {
                        ::prost::encoding::message::encode(8u32, msg, buf);
                    }
                    if let Some(ref msg) = self.bumper {
                        ::prost::encoding::message::encode(9u32, msg, buf);
                    }
                    if let Some(ref msg) = self.front_left {
                        ::prost::encoding::message::encode(10u32, msg, buf);
                    }
                    if let Some(ref msg) = self.front_right {
                        ::prost::encoding::message::encode(11u32, msg, buf);
                    }
                    if let Some(ref msg) = self.back_left {
                        ::prost::encoding::message::encode(12u32, msg, buf);
                    }
                    if let Some(ref msg) = self.back_right {
                        ::prost::encoding::message::encode(13u32, msg, buf);
                    }
                }
                #[allow(unused_variables)]
                fn merge_field(
                    &mut self,
                    tag: u32,
                    wire_type: ::prost::encoding::wire_type::WireType,
                    buf: &mut impl ::prost::bytes::Buf,
                    ctx: ::prost::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::prost::DecodeError> {
                    const STRUCT_NAME: &'static str = "ExprRobotConfig";
                    match tag {
                        1u32 => {
                            let mut value = &mut self.mass;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "mass");
                                    error
                                })
                        }
                        2u32 => {
                            let mut value = &mut self.inertia;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "inertia");
                                    error
                                })
                        }
                        3u32 => {
                            let mut value = &mut self.gearing;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "gearing");
                                    error
                                })
                        }
                        4u32 => {
                            let mut value = &mut self.radius;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "radius");
                                    error
                                })
                        }
                        5u32 => {
                            let mut value = &mut self.vmax;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "vmax");
                                    error
                                })
                        }
                        6u32 => {
                            let mut value = &mut self.tmax;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "tmax");
                                    error
                                })
                        }
                        7u32 => {
                            let mut value = &mut self.cof;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "cof");
                                    error
                                })
                        }
                        8u32 => {
                            let mut value = &mut self.differential_track_width;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "differential_track_width");
                                    error
                                })
                        }
                        9u32 => {
                            let mut value = &mut self.bumper;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "bumper");
                                    error
                                })
                        }
                        10u32 => {
                            let mut value = &mut self.front_left;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "front_left");
                                    error
                                })
                        }
                        11u32 => {
                            let mut value = &mut self.front_right;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "front_right");
                                    error
                                })
                        }
                        12u32 => {
                            let mut value = &mut self.back_left;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "back_left");
                                    error
                                })
                        }
                        13u32 => {
                            let mut value = &mut self.back_right;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "back_right");
                                    error
                                })
                        }
                        _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0
                        + self
                            .mass
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(1u32, msg),
                            )
                        + self
                            .inertia
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(2u32, msg),
                            )
                        + self
                            .gearing
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(3u32, msg),
                            )
                        + self
                            .radius
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(4u32, msg),
                            )
                        + self
                            .vmax
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(5u32, msg),
                            )
                        + self
                            .tmax
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(6u32, msg),
                            )
                        + self
                            .cof
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(7u32, msg),
                            )
                        + self
                            .differential_track_width
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(8u32, msg),
                            )
                        + self
                            .bumper
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(9u32, msg),
                            )
                        + self
                            .front_left
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(10u32, msg),
                            )
                        + self
                            .front_right
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(11u32, msg),
                            )
                        + self
                            .back_left
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(12u32, msg),
                            )
                        + self
                            .back_right
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(13u32, msg),
                            )
                }
                fn clear(&mut self) {
                    self.mass = ::core::option::Option::None;
                    self.inertia = ::core::option::Option::None;
                    self.gearing = ::core::option::Option::None;
                    self.radius = ::core::option::Option::None;
                    self.vmax = ::core::option::Option::None;
                    self.tmax = ::core::option::Option::None;
                    self.cof = ::core::option::Option::None;
                    self.differential_track_width = ::core::option::Option::None;
                    self.bumper = ::core::option::Option::None;
                    self.front_left = ::core::option::Option::None;
                    self.front_right = ::core::option::Option::None;
                    self.back_left = ::core::option::Option::None;
                    self.back_right = ::core::option::Option::None;
                }
            }
            impl ::core::default::Default for ExprRobotConfig {
                fn default() -> Self {
                    ExprRobotConfig {
                        mass: ::core::default::Default::default(),
                        inertia: ::core::default::Default::default(),
                        gearing: ::core::default::Default::default(),
                        radius: ::core::default::Default::default(),
                        vmax: ::core::default::Default::default(),
                        tmax: ::core::default::Default::default(),
                        cof: ::core::default::Default::default(),
                        differential_track_width: ::core::default::Default::default(),
                        bumper: ::core::default::Default::default(),
                        front_left: ::core::default::Default::default(),
                        front_right: ::core::default::Default::default(),
                        back_left: ::core::default::Default::default(),
                        back_right: ::core::default::Default::default(),
                    }
                }
            }
            impl ::core::fmt::Debug for ExprRobotConfig {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let mut builder = f.debug_struct("ExprRobotConfig");
                    let builder = {
                        let wrapper = &self.mass;
                        builder.field("mass", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.inertia;
                        builder.field("inertia", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.gearing;
                        builder.field("gearing", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.radius;
                        builder.field("radius", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.vmax;
                        builder.field("vmax", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.tmax;
                        builder.field("tmax", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.cof;
                        builder.field("cof", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.differential_track_width;
                        builder.field("differential_track_width", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.bumper;
                        builder.field("bumper", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.front_left;
                        builder.field("front_left", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.front_right;
                        builder.field("front_right", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.back_left;
                        builder.field("back_left", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.back_right;
                        builder.field("back_right", &wrapper)
                    };
                    builder.finish()
                }
            }
            pub struct RequiredExprRobotConfig {
                pub mass: super::RequiredExpr,
                pub inertia: super::RequiredExpr,
                pub gearing: super::RequiredExpr,
                pub radius: super::RequiredExpr,
                pub vmax: super::RequiredExpr,
                pub tmax: super::RequiredExpr,
                pub cof: super::RequiredExpr,
                pub differential_track_width: super::RequiredExpr,
                pub bumper: RequiredExprBumper,
                pub front_left: RequiredExprModule,
                pub front_right: RequiredExprModule,
                pub back_left: RequiredExprModule,
                pub back_right: RequiredExprModule,
            }
            impl From<RequiredExprRobotConfig> for ExprRobotConfig {
                fn from(value: RequiredExprRobotConfig) -> Self {
                    ExprRobotConfig {
                        mass: Some(value.mass.into()),
                        inertia: Some(value.inertia.into()),
                        gearing: Some(value.gearing.into()),
                        radius: Some(value.radius.into()),
                        vmax: Some(value.vmax.into()),
                        tmax: Some(value.tmax.into()),
                        cof: Some(value.cof.into()),
                        differential_track_width: Some(
                            value.differential_track_width.into(),
                        ),
                        bumper: Some(value.bumper.into()),
                        front_left: Some(value.front_left.into()),
                        front_right: Some(value.front_right.into()),
                        back_left: Some(value.back_left.into()),
                        back_right: Some(value.back_right.into()),
                    }
                }
            }
            impl serde::Serialize for DoubleBumper {
                #[allow(deprecated)]
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> std::result::Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeStruct;
                    let mut len = 0;
                    if self.front != 0. {
                        len += 1;
                    }
                    if self.left != 0. {
                        len += 1;
                    }
                    if self.right != 0. {
                        len += 1;
                    }
                    if self.back != 0. {
                        len += 1;
                    }
                    let mut struct_ser = serializer
                        .serialize_struct(
                            "entity.parameters.robotconfig.DoubleBumper",
                            len,
                        )?;
                    if self.front != 0. {
                        struct_ser.serialize_field("front", &self.front)?;
                    }
                    if self.left != 0. {
                        struct_ser.serialize_field("left", &self.left)?;
                    }
                    if self.right != 0. {
                        struct_ser.serialize_field("right", &self.right)?;
                    }
                    if self.back != 0. {
                        struct_ser.serialize_field("back", &self.back)?;
                    }
                    struct_ser.end()
                }
            }
            impl<'de> serde::Deserialize<'de> for DoubleBumper {
                #[allow(deprecated)]
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    const FIELDS: &[&str] = &["front", "left", "right", "back"];
                    #[allow(clippy::enum_variant_names)]
                    enum GeneratedField {
                        Front,
                        Left,
                        Right,
                        Back,
                    }
                    impl<'de> serde::Deserialize<'de> for GeneratedField {
                        fn deserialize<D>(
                            deserializer: D,
                        ) -> std::result::Result<GeneratedField, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            struct GeneratedVisitor;
                            impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                                type Value = GeneratedField;
                                fn expecting(
                                    &self,
                                    formatter: &mut std::fmt::Formatter<'_>,
                                ) -> std::fmt::Result {
                                    formatter
                                        .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                                }
                                #[allow(unused_variables)]
                                fn visit_str<E>(
                                    self,
                                    value: &str,
                                ) -> std::result::Result<GeneratedField, E>
                                where
                                    E: serde::de::Error,
                                {
                                    match value {
                                        "front" => Ok(GeneratedField::Front),
                                        "left" => Ok(GeneratedField::Left),
                                        "right" => Ok(GeneratedField::Right),
                                        "back" => Ok(GeneratedField::Back),
                                        _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                    }
                                }
                            }
                            deserializer.deserialize_identifier(GeneratedVisitor)
                        }
                    }
                    struct GeneratedVisitor;
                    impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                        type Value = DoubleBumper;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> std::fmt::Result {
                            formatter
                                .write_str(
                                    "struct entity.parameters.robotconfig.DoubleBumper",
                                )
                        }
                        fn visit_map<V>(
                            self,
                            mut map_: V,
                        ) -> std::result::Result<DoubleBumper, V::Error>
                        where
                            V: serde::de::MapAccess<'de>,
                        {
                            let mut front__ = None;
                            let mut left__ = None;
                            let mut right__ = None;
                            let mut back__ = None;
                            while let Some(k) = map_.next_key()? {
                                match k {
                                    GeneratedField::Front => {
                                        if front__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("front"));
                                        }
                                        front__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                    GeneratedField::Left => {
                                        if left__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("left"));
                                        }
                                        left__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                    GeneratedField::Right => {
                                        if right__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("right"));
                                        }
                                        right__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                    GeneratedField::Back => {
                                        if back__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("back"));
                                        }
                                        back__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                }
                            }
                            Ok(DoubleBumper {
                                front: front__.unwrap_or_default(),
                                left: left__.unwrap_or_default(),
                                right: right__.unwrap_or_default(),
                                back: back__.unwrap_or_default(),
                            })
                        }
                    }
                    deserializer
                        .deserialize_struct(
                            "entity.parameters.robotconfig.DoubleBumper",
                            FIELDS,
                            GeneratedVisitor,
                        )
                }
            }
            impl serde::Serialize for DoubleModule {
                #[allow(deprecated)]
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> std::result::Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeStruct;
                    let mut len = 0;
                    if self.x != 0. {
                        len += 1;
                    }
                    if self.y != 0. {
                        len += 1;
                    }
                    let mut struct_ser = serializer
                        .serialize_struct(
                            "entity.parameters.robotconfig.DoubleModule",
                            len,
                        )?;
                    if self.x != 0. {
                        struct_ser.serialize_field("x", &self.x)?;
                    }
                    if self.y != 0. {
                        struct_ser.serialize_field("y", &self.y)?;
                    }
                    struct_ser.end()
                }
            }
            impl<'de> serde::Deserialize<'de> for DoubleModule {
                #[allow(deprecated)]
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    const FIELDS: &[&str] = &["x", "y"];
                    #[allow(clippy::enum_variant_names)]
                    enum GeneratedField {
                        X,
                        Y,
                    }
                    impl<'de> serde::Deserialize<'de> for GeneratedField {
                        fn deserialize<D>(
                            deserializer: D,
                        ) -> std::result::Result<GeneratedField, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            struct GeneratedVisitor;
                            impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                                type Value = GeneratedField;
                                fn expecting(
                                    &self,
                                    formatter: &mut std::fmt::Formatter<'_>,
                                ) -> std::fmt::Result {
                                    formatter
                                        .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                                }
                                #[allow(unused_variables)]
                                fn visit_str<E>(
                                    self,
                                    value: &str,
                                ) -> std::result::Result<GeneratedField, E>
                                where
                                    E: serde::de::Error,
                                {
                                    match value {
                                        "x" => Ok(GeneratedField::X),
                                        "y" => Ok(GeneratedField::Y),
                                        _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                    }
                                }
                            }
                            deserializer.deserialize_identifier(GeneratedVisitor)
                        }
                    }
                    struct GeneratedVisitor;
                    impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                        type Value = DoubleModule;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> std::fmt::Result {
                            formatter
                                .write_str(
                                    "struct entity.parameters.robotconfig.DoubleModule",
                                )
                        }
                        fn visit_map<V>(
                            self,
                            mut map_: V,
                        ) -> std::result::Result<DoubleModule, V::Error>
                        where
                            V: serde::de::MapAccess<'de>,
                        {
                            let mut x__ = None;
                            let mut y__ = None;
                            while let Some(k) = map_.next_key()? {
                                match k {
                                    GeneratedField::X => {
                                        if x__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("x"));
                                        }
                                        x__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                    GeneratedField::Y => {
                                        if y__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("y"));
                                        }
                                        y__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                }
                            }
                            Ok(DoubleModule {
                                x: x__.unwrap_or_default(),
                                y: y__.unwrap_or_default(),
                            })
                        }
                    }
                    deserializer
                        .deserialize_struct(
                            "entity.parameters.robotconfig.DoubleModule",
                            FIELDS,
                            GeneratedVisitor,
                        )
                }
            }
            impl serde::Serialize for DoubleRobotConfig {
                #[allow(deprecated)]
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> std::result::Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeStruct;
                    let mut len = 0;
                    if self.mass != 0. {
                        len += 1;
                    }
                    if self.inertia != 0. {
                        len += 1;
                    }
                    if self.gearing != 0. {
                        len += 1;
                    }
                    if self.radius != 0. {
                        len += 1;
                    }
                    if self.vmax != 0. {
                        len += 1;
                    }
                    if self.tmax != 0. {
                        len += 1;
                    }
                    if self.cof != 0. {
                        len += 1;
                    }
                    if self.differential_track_width != 0. {
                        len += 1;
                    }
                    if self.bumper.is_some() {
                        len += 1;
                    }
                    if self.front_left.is_some() {
                        len += 1;
                    }
                    if self.front_right.is_some() {
                        len += 1;
                    }
                    if self.back_left.is_some() {
                        len += 1;
                    }
                    if self.back_right.is_some() {
                        len += 1;
                    }
                    let mut struct_ser = serializer
                        .serialize_struct(
                            "entity.parameters.robotconfig.DoubleRobotConfig",
                            len,
                        )?;
                    if self.mass != 0. {
                        struct_ser.serialize_field("mass", &self.mass)?;
                    }
                    if self.inertia != 0. {
                        struct_ser.serialize_field("inertia", &self.inertia)?;
                    }
                    if self.gearing != 0. {
                        struct_ser.serialize_field("gearing", &self.gearing)?;
                    }
                    if self.radius != 0. {
                        struct_ser.serialize_field("radius", &self.radius)?;
                    }
                    if self.vmax != 0. {
                        struct_ser.serialize_field("vmax", &self.vmax)?;
                    }
                    if self.tmax != 0. {
                        struct_ser.serialize_field("tmax", &self.tmax)?;
                    }
                    if self.cof != 0. {
                        struct_ser.serialize_field("cof", &self.cof)?;
                    }
                    if self.differential_track_width != 0. {
                        struct_ser
                            .serialize_field(
                                "differentialTrackWidth",
                                &self.differential_track_width,
                            )?;
                    }
                    if let Some(v) = self.bumper.as_ref() {
                        struct_ser.serialize_field("bumper", v)?;
                    }
                    if let Some(v) = self.front_left.as_ref() {
                        struct_ser.serialize_field("frontLeft", v)?;
                    }
                    if let Some(v) = self.front_right.as_ref() {
                        struct_ser.serialize_field("frontRight", v)?;
                    }
                    if let Some(v) = self.back_left.as_ref() {
                        struct_ser.serialize_field("backLeft", v)?;
                    }
                    if let Some(v) = self.back_right.as_ref() {
                        struct_ser.serialize_field("backRight", v)?;
                    }
                    struct_ser.end()
                }
            }
            impl<'de> serde::Deserialize<'de> for DoubleRobotConfig {
                #[allow(deprecated)]
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    const FIELDS: &[&str] = &[
                        "mass",
                        "inertia",
                        "gearing",
                        "radius",
                        "vmax",
                        "tmax",
                        "cof",
                        "differential_track_width",
                        "differentialTrackWidth",
                        "bumper",
                        "front_left",
                        "frontLeft",
                        "front_right",
                        "frontRight",
                        "back_left",
                        "backLeft",
                        "back_right",
                        "backRight",
                    ];
                    #[allow(clippy::enum_variant_names)]
                    enum GeneratedField {
                        Mass,
                        Inertia,
                        Gearing,
                        Radius,
                        Vmax,
                        Tmax,
                        Cof,
                        DifferentialTrackWidth,
                        Bumper,
                        FrontLeft,
                        FrontRight,
                        BackLeft,
                        BackRight,
                    }
                    impl<'de> serde::Deserialize<'de> for GeneratedField {
                        fn deserialize<D>(
                            deserializer: D,
                        ) -> std::result::Result<GeneratedField, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            struct GeneratedVisitor;
                            impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                                type Value = GeneratedField;
                                fn expecting(
                                    &self,
                                    formatter: &mut std::fmt::Formatter<'_>,
                                ) -> std::fmt::Result {
                                    formatter
                                        .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                                }
                                #[allow(unused_variables)]
                                fn visit_str<E>(
                                    self,
                                    value: &str,
                                ) -> std::result::Result<GeneratedField, E>
                                where
                                    E: serde::de::Error,
                                {
                                    match value {
                                        "mass" => Ok(GeneratedField::Mass),
                                        "inertia" => Ok(GeneratedField::Inertia),
                                        "gearing" => Ok(GeneratedField::Gearing),
                                        "radius" => Ok(GeneratedField::Radius),
                                        "vmax" => Ok(GeneratedField::Vmax),
                                        "tmax" => Ok(GeneratedField::Tmax),
                                        "cof" => Ok(GeneratedField::Cof),
                                        "differentialTrackWidth" | "differential_track_width" => {
                                            Ok(GeneratedField::DifferentialTrackWidth)
                                        }
                                        "bumper" => Ok(GeneratedField::Bumper),
                                        "frontLeft" | "front_left" => Ok(GeneratedField::FrontLeft),
                                        "frontRight" | "front_right" => {
                                            Ok(GeneratedField::FrontRight)
                                        }
                                        "backLeft" | "back_left" => Ok(GeneratedField::BackLeft),
                                        "backRight" | "back_right" => Ok(GeneratedField::BackRight),
                                        _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                    }
                                }
                            }
                            deserializer.deserialize_identifier(GeneratedVisitor)
                        }
                    }
                    struct GeneratedVisitor;
                    impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                        type Value = DoubleRobotConfig;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> std::fmt::Result {
                            formatter
                                .write_str(
                                    "struct entity.parameters.robotconfig.DoubleRobotConfig",
                                )
                        }
                        fn visit_map<V>(
                            self,
                            mut map_: V,
                        ) -> std::result::Result<DoubleRobotConfig, V::Error>
                        where
                            V: serde::de::MapAccess<'de>,
                        {
                            let mut mass__ = None;
                            let mut inertia__ = None;
                            let mut gearing__ = None;
                            let mut radius__ = None;
                            let mut vmax__ = None;
                            let mut tmax__ = None;
                            let mut cof__ = None;
                            let mut differential_track_width__ = None;
                            let mut bumper__ = None;
                            let mut front_left__ = None;
                            let mut front_right__ = None;
                            let mut back_left__ = None;
                            let mut back_right__ = None;
                            while let Some(k) = map_.next_key()? {
                                match k {
                                    GeneratedField::Mass => {
                                        if mass__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("mass"));
                                        }
                                        mass__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                    GeneratedField::Inertia => {
                                        if inertia__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("inertia"));
                                        }
                                        inertia__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                    GeneratedField::Gearing => {
                                        if gearing__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("gearing"));
                                        }
                                        gearing__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                    GeneratedField::Radius => {
                                        if radius__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("radius"));
                                        }
                                        radius__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                    GeneratedField::Vmax => {
                                        if vmax__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("vmax"));
                                        }
                                        vmax__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                    GeneratedField::Tmax => {
                                        if tmax__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("tmax"));
                                        }
                                        tmax__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                    GeneratedField::Cof => {
                                        if cof__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("cof"));
                                        }
                                        cof__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                    GeneratedField::DifferentialTrackWidth => {
                                        if differential_track_width__.is_some() {
                                            return Err(
                                                serde::de::Error::duplicate_field("differentialTrackWidth"),
                                            );
                                        }
                                        differential_track_width__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                    GeneratedField::Bumper => {
                                        if bumper__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("bumper"));
                                        }
                                        bumper__ = map_.next_value()?;
                                    }
                                    GeneratedField::FrontLeft => {
                                        if front_left__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("frontLeft"));
                                        }
                                        front_left__ = map_.next_value()?;
                                    }
                                    GeneratedField::FrontRight => {
                                        if front_right__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("frontRight"));
                                        }
                                        front_right__ = map_.next_value()?;
                                    }
                                    GeneratedField::BackLeft => {
                                        if back_left__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("backLeft"));
                                        }
                                        back_left__ = map_.next_value()?;
                                    }
                                    GeneratedField::BackRight => {
                                        if back_right__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("backRight"));
                                        }
                                        back_right__ = map_.next_value()?;
                                    }
                                }
                            }
                            Ok(DoubleRobotConfig {
                                mass: mass__.unwrap_or_default(),
                                inertia: inertia__.unwrap_or_default(),
                                gearing: gearing__.unwrap_or_default(),
                                radius: radius__.unwrap_or_default(),
                                vmax: vmax__.unwrap_or_default(),
                                tmax: tmax__.unwrap_or_default(),
                                cof: cof__.unwrap_or_default(),
                                differential_track_width: differential_track_width__
                                    .unwrap_or_default(),
                                bumper: bumper__,
                                front_left: front_left__,
                                front_right: front_right__,
                                back_left: back_left__,
                                back_right: back_right__,
                            })
                        }
                    }
                    deserializer
                        .deserialize_struct(
                            "entity.parameters.robotconfig.DoubleRobotConfig",
                            FIELDS,
                            GeneratedVisitor,
                        )
                }
            }
            impl serde::Serialize for ExprBumper {
                #[allow(deprecated)]
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> std::result::Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeStruct;
                    let mut len = 0;
                    if self.front.is_some() {
                        len += 1;
                    }
                    if self.left.is_some() {
                        len += 1;
                    }
                    if self.right.is_some() {
                        len += 1;
                    }
                    if self.back.is_some() {
                        len += 1;
                    }
                    let mut struct_ser = serializer
                        .serialize_struct(
                            "entity.parameters.robotconfig.ExprBumper",
                            len,
                        )?;
                    if let Some(v) = self.front.as_ref() {
                        struct_ser.serialize_field("front", v)?;
                    }
                    if let Some(v) = self.left.as_ref() {
                        struct_ser.serialize_field("left", v)?;
                    }
                    if let Some(v) = self.right.as_ref() {
                        struct_ser.serialize_field("right", v)?;
                    }
                    if let Some(v) = self.back.as_ref() {
                        struct_ser.serialize_field("back", v)?;
                    }
                    struct_ser.end()
                }
            }
            impl<'de> serde::Deserialize<'de> for ExprBumper {
                #[allow(deprecated)]
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    const FIELDS: &[&str] = &["front", "left", "right", "back"];
                    #[allow(clippy::enum_variant_names)]
                    enum GeneratedField {
                        Front,
                        Left,
                        Right,
                        Back,
                    }
                    impl<'de> serde::Deserialize<'de> for GeneratedField {
                        fn deserialize<D>(
                            deserializer: D,
                        ) -> std::result::Result<GeneratedField, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            struct GeneratedVisitor;
                            impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                                type Value = GeneratedField;
                                fn expecting(
                                    &self,
                                    formatter: &mut std::fmt::Formatter<'_>,
                                ) -> std::fmt::Result {
                                    formatter
                                        .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                                }
                                #[allow(unused_variables)]
                                fn visit_str<E>(
                                    self,
                                    value: &str,
                                ) -> std::result::Result<GeneratedField, E>
                                where
                                    E: serde::de::Error,
                                {
                                    match value {
                                        "front" => Ok(GeneratedField::Front),
                                        "left" => Ok(GeneratedField::Left),
                                        "right" => Ok(GeneratedField::Right),
                                        "back" => Ok(GeneratedField::Back),
                                        _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                    }
                                }
                            }
                            deserializer.deserialize_identifier(GeneratedVisitor)
                        }
                    }
                    struct GeneratedVisitor;
                    impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                        type Value = ExprBumper;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> std::fmt::Result {
                            formatter
                                .write_str(
                                    "struct entity.parameters.robotconfig.ExprBumper",
                                )
                        }
                        fn visit_map<V>(
                            self,
                            mut map_: V,
                        ) -> std::result::Result<ExprBumper, V::Error>
                        where
                            V: serde::de::MapAccess<'de>,
                        {
                            let mut front__ = None;
                            let mut left__ = None;
                            let mut right__ = None;
                            let mut back__ = None;
                            while let Some(k) = map_.next_key()? {
                                match k {
                                    GeneratedField::Front => {
                                        if front__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("front"));
                                        }
                                        front__ = map_.next_value()?;
                                    }
                                    GeneratedField::Left => {
                                        if left__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("left"));
                                        }
                                        left__ = map_.next_value()?;
                                    }
                                    GeneratedField::Right => {
                                        if right__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("right"));
                                        }
                                        right__ = map_.next_value()?;
                                    }
                                    GeneratedField::Back => {
                                        if back__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("back"));
                                        }
                                        back__ = map_.next_value()?;
                                    }
                                }
                            }
                            Ok(ExprBumper {
                                front: front__,
                                left: left__,
                                right: right__,
                                back: back__,
                            })
                        }
                    }
                    deserializer
                        .deserialize_struct(
                            "entity.parameters.robotconfig.ExprBumper",
                            FIELDS,
                            GeneratedVisitor,
                        )
                }
            }
            impl serde::Serialize for ExprModule {
                #[allow(deprecated)]
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> std::result::Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeStruct;
                    let mut len = 0;
                    if self.x.is_some() {
                        len += 1;
                    }
                    if self.y.is_some() {
                        len += 1;
                    }
                    let mut struct_ser = serializer
                        .serialize_struct(
                            "entity.parameters.robotconfig.ExprModule",
                            len,
                        )?;
                    if let Some(v) = self.x.as_ref() {
                        struct_ser.serialize_field("x", v)?;
                    }
                    if let Some(v) = self.y.as_ref() {
                        struct_ser.serialize_field("y", v)?;
                    }
                    struct_ser.end()
                }
            }
            impl<'de> serde::Deserialize<'de> for ExprModule {
                #[allow(deprecated)]
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    const FIELDS: &[&str] = &["x", "y"];
                    #[allow(clippy::enum_variant_names)]
                    enum GeneratedField {
                        X,
                        Y,
                    }
                    impl<'de> serde::Deserialize<'de> for GeneratedField {
                        fn deserialize<D>(
                            deserializer: D,
                        ) -> std::result::Result<GeneratedField, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            struct GeneratedVisitor;
                            impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                                type Value = GeneratedField;
                                fn expecting(
                                    &self,
                                    formatter: &mut std::fmt::Formatter<'_>,
                                ) -> std::fmt::Result {
                                    formatter
                                        .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                                }
                                #[allow(unused_variables)]
                                fn visit_str<E>(
                                    self,
                                    value: &str,
                                ) -> std::result::Result<GeneratedField, E>
                                where
                                    E: serde::de::Error,
                                {
                                    match value {
                                        "x" => Ok(GeneratedField::X),
                                        "y" => Ok(GeneratedField::Y),
                                        _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                    }
                                }
                            }
                            deserializer.deserialize_identifier(GeneratedVisitor)
                        }
                    }
                    struct GeneratedVisitor;
                    impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                        type Value = ExprModule;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> std::fmt::Result {
                            formatter
                                .write_str(
                                    "struct entity.parameters.robotconfig.ExprModule",
                                )
                        }
                        fn visit_map<V>(
                            self,
                            mut map_: V,
                        ) -> std::result::Result<ExprModule, V::Error>
                        where
                            V: serde::de::MapAccess<'de>,
                        {
                            let mut x__ = None;
                            let mut y__ = None;
                            while let Some(k) = map_.next_key()? {
                                match k {
                                    GeneratedField::X => {
                                        if x__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("x"));
                                        }
                                        x__ = map_.next_value()?;
                                    }
                                    GeneratedField::Y => {
                                        if y__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("y"));
                                        }
                                        y__ = map_.next_value()?;
                                    }
                                }
                            }
                            Ok(ExprModule { x: x__, y: y__ })
                        }
                    }
                    deserializer
                        .deserialize_struct(
                            "entity.parameters.robotconfig.ExprModule",
                            FIELDS,
                            GeneratedVisitor,
                        )
                }
            }
            impl serde::Serialize for ExprRobotConfig {
                #[allow(deprecated)]
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> std::result::Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeStruct;
                    let mut len = 0;
                    if self.mass.is_some() {
                        len += 1;
                    }
                    if self.inertia.is_some() {
                        len += 1;
                    }
                    if self.gearing.is_some() {
                        len += 1;
                    }
                    if self.radius.is_some() {
                        len += 1;
                    }
                    if self.vmax.is_some() {
                        len += 1;
                    }
                    if self.tmax.is_some() {
                        len += 1;
                    }
                    if self.cof.is_some() {
                        len += 1;
                    }
                    if self.differential_track_width.is_some() {
                        len += 1;
                    }
                    if self.bumper.is_some() {
                        len += 1;
                    }
                    if self.front_left.is_some() {
                        len += 1;
                    }
                    if self.front_right.is_some() {
                        len += 1;
                    }
                    if self.back_left.is_some() {
                        len += 1;
                    }
                    if self.back_right.is_some() {
                        len += 1;
                    }
                    let mut struct_ser = serializer
                        .serialize_struct(
                            "entity.parameters.robotconfig.ExprRobotConfig",
                            len,
                        )?;
                    if let Some(v) = self.mass.as_ref() {
                        struct_ser.serialize_field("mass", v)?;
                    }
                    if let Some(v) = self.inertia.as_ref() {
                        struct_ser.serialize_field("inertia", v)?;
                    }
                    if let Some(v) = self.gearing.as_ref() {
                        struct_ser.serialize_field("gearing", v)?;
                    }
                    if let Some(v) = self.radius.as_ref() {
                        struct_ser.serialize_field("radius", v)?;
                    }
                    if let Some(v) = self.vmax.as_ref() {
                        struct_ser.serialize_field("vmax", v)?;
                    }
                    if let Some(v) = self.tmax.as_ref() {
                        struct_ser.serialize_field("tmax", v)?;
                    }
                    if let Some(v) = self.cof.as_ref() {
                        struct_ser.serialize_field("cof", v)?;
                    }
                    if let Some(v) = self.differential_track_width.as_ref() {
                        struct_ser.serialize_field("differentialTrackWidth", v)?;
                    }
                    if let Some(v) = self.bumper.as_ref() {
                        struct_ser.serialize_field("bumper", v)?;
                    }
                    if let Some(v) = self.front_left.as_ref() {
                        struct_ser.serialize_field("frontLeft", v)?;
                    }
                    if let Some(v) = self.front_right.as_ref() {
                        struct_ser.serialize_field("frontRight", v)?;
                    }
                    if let Some(v) = self.back_left.as_ref() {
                        struct_ser.serialize_field("backLeft", v)?;
                    }
                    if let Some(v) = self.back_right.as_ref() {
                        struct_ser.serialize_field("backRight", v)?;
                    }
                    struct_ser.end()
                }
            }
            impl<'de> serde::Deserialize<'de> for ExprRobotConfig {
                #[allow(deprecated)]
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    const FIELDS: &[&str] = &[
                        "mass",
                        "inertia",
                        "gearing",
                        "radius",
                        "vmax",
                        "tmax",
                        "cof",
                        "differential_track_width",
                        "differentialTrackWidth",
                        "bumper",
                        "front_left",
                        "frontLeft",
                        "front_right",
                        "frontRight",
                        "back_left",
                        "backLeft",
                        "back_right",
                        "backRight",
                    ];
                    #[allow(clippy::enum_variant_names)]
                    enum GeneratedField {
                        Mass,
                        Inertia,
                        Gearing,
                        Radius,
                        Vmax,
                        Tmax,
                        Cof,
                        DifferentialTrackWidth,
                        Bumper,
                        FrontLeft,
                        FrontRight,
                        BackLeft,
                        BackRight,
                    }
                    impl<'de> serde::Deserialize<'de> for GeneratedField {
                        fn deserialize<D>(
                            deserializer: D,
                        ) -> std::result::Result<GeneratedField, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            struct GeneratedVisitor;
                            impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                                type Value = GeneratedField;
                                fn expecting(
                                    &self,
                                    formatter: &mut std::fmt::Formatter<'_>,
                                ) -> std::fmt::Result {
                                    formatter
                                        .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                                }
                                #[allow(unused_variables)]
                                fn visit_str<E>(
                                    self,
                                    value: &str,
                                ) -> std::result::Result<GeneratedField, E>
                                where
                                    E: serde::de::Error,
                                {
                                    match value {
                                        "mass" => Ok(GeneratedField::Mass),
                                        "inertia" => Ok(GeneratedField::Inertia),
                                        "gearing" => Ok(GeneratedField::Gearing),
                                        "radius" => Ok(GeneratedField::Radius),
                                        "vmax" => Ok(GeneratedField::Vmax),
                                        "tmax" => Ok(GeneratedField::Tmax),
                                        "cof" => Ok(GeneratedField::Cof),
                                        "differentialTrackWidth" | "differential_track_width" => {
                                            Ok(GeneratedField::DifferentialTrackWidth)
                                        }
                                        "bumper" => Ok(GeneratedField::Bumper),
                                        "frontLeft" | "front_left" => Ok(GeneratedField::FrontLeft),
                                        "frontRight" | "front_right" => {
                                            Ok(GeneratedField::FrontRight)
                                        }
                                        "backLeft" | "back_left" => Ok(GeneratedField::BackLeft),
                                        "backRight" | "back_right" => Ok(GeneratedField::BackRight),
                                        _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                    }
                                }
                            }
                            deserializer.deserialize_identifier(GeneratedVisitor)
                        }
                    }
                    struct GeneratedVisitor;
                    impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                        type Value = ExprRobotConfig;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> std::fmt::Result {
                            formatter
                                .write_str(
                                    "struct entity.parameters.robotconfig.ExprRobotConfig",
                                )
                        }
                        fn visit_map<V>(
                            self,
                            mut map_: V,
                        ) -> std::result::Result<ExprRobotConfig, V::Error>
                        where
                            V: serde::de::MapAccess<'de>,
                        {
                            let mut mass__ = None;
                            let mut inertia__ = None;
                            let mut gearing__ = None;
                            let mut radius__ = None;
                            let mut vmax__ = None;
                            let mut tmax__ = None;
                            let mut cof__ = None;
                            let mut differential_track_width__ = None;
                            let mut bumper__ = None;
                            let mut front_left__ = None;
                            let mut front_right__ = None;
                            let mut back_left__ = None;
                            let mut back_right__ = None;
                            while let Some(k) = map_.next_key()? {
                                match k {
                                    GeneratedField::Mass => {
                                        if mass__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("mass"));
                                        }
                                        mass__ = map_.next_value()?;
                                    }
                                    GeneratedField::Inertia => {
                                        if inertia__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("inertia"));
                                        }
                                        inertia__ = map_.next_value()?;
                                    }
                                    GeneratedField::Gearing => {
                                        if gearing__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("gearing"));
                                        }
                                        gearing__ = map_.next_value()?;
                                    }
                                    GeneratedField::Radius => {
                                        if radius__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("radius"));
                                        }
                                        radius__ = map_.next_value()?;
                                    }
                                    GeneratedField::Vmax => {
                                        if vmax__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("vmax"));
                                        }
                                        vmax__ = map_.next_value()?;
                                    }
                                    GeneratedField::Tmax => {
                                        if tmax__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("tmax"));
                                        }
                                        tmax__ = map_.next_value()?;
                                    }
                                    GeneratedField::Cof => {
                                        if cof__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("cof"));
                                        }
                                        cof__ = map_.next_value()?;
                                    }
                                    GeneratedField::DifferentialTrackWidth => {
                                        if differential_track_width__.is_some() {
                                            return Err(
                                                serde::de::Error::duplicate_field("differentialTrackWidth"),
                                            );
                                        }
                                        differential_track_width__ = map_.next_value()?;
                                    }
                                    GeneratedField::Bumper => {
                                        if bumper__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("bumper"));
                                        }
                                        bumper__ = map_.next_value()?;
                                    }
                                    GeneratedField::FrontLeft => {
                                        if front_left__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("frontLeft"));
                                        }
                                        front_left__ = map_.next_value()?;
                                    }
                                    GeneratedField::FrontRight => {
                                        if front_right__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("frontRight"));
                                        }
                                        front_right__ = map_.next_value()?;
                                    }
                                    GeneratedField::BackLeft => {
                                        if back_left__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("backLeft"));
                                        }
                                        back_left__ = map_.next_value()?;
                                    }
                                    GeneratedField::BackRight => {
                                        if back_right__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("backRight"));
                                        }
                                        back_right__ = map_.next_value()?;
                                    }
                                }
                            }
                            Ok(ExprRobotConfig {
                                mass: mass__,
                                inertia: inertia__,
                                gearing: gearing__,
                                radius: radius__,
                                vmax: vmax__,
                                tmax: tmax__,
                                cof: cof__,
                                differential_track_width: differential_track_width__,
                                bumper: bumper__,
                                front_left: front_left__,
                                front_right: front_right__,
                                back_left: back_left__,
                                back_right: back_right__,
                            })
                        }
                    }
                    deserializer
                        .deserialize_struct(
                            "entity.parameters.robotconfig.ExprRobotConfig",
                            FIELDS,
                            GeneratedVisitor,
                        )
                }
            }
        }
        pub mod waypoint {
            #[required(prefix = "Required")]
            pub struct DoubleWaypoint {
                #[prost(double, tag = "1")]
                pub x: f64,
                #[prost(double, tag = "2")]
                pub y: f64,
                #[prost(double, tag = "3")]
                pub heading: f64,
                #[prost(uint64, tag = "4")]
                pub intervals: u64,
                #[prost(bool, tag = "5")]
                pub split: bool,
                #[prost(bool, tag = "6")]
                pub fix_translation: bool,
                #[prost(bool, tag = "7")]
                pub fix_heading: bool,
                #[prost(bool, tag = "8")]
                pub override_intervals: bool,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for DoubleWaypoint {
                #[inline]
                fn clone(&self) -> DoubleWaypoint {
                    let _: ::core::clone::AssertParamIsClone<f64>;
                    let _: ::core::clone::AssertParamIsClone<u64>;
                    let _: ::core::clone::AssertParamIsClone<bool>;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for DoubleWaypoint {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for DoubleWaypoint {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for DoubleWaypoint {
                #[inline]
                fn eq(&self, other: &DoubleWaypoint) -> bool {
                    self.x == other.x && self.y == other.y
                        && self.heading == other.heading
                        && self.intervals == other.intervals && self.split == other.split
                        && self.fix_translation == other.fix_translation
                        && self.fix_heading == other.fix_heading
                        && self.override_intervals == other.override_intervals
                }
            }
            impl ::prost::Message for DoubleWaypoint {
                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                    if self.x != 0f64 {
                        ::prost::encoding::double::encode(1u32, &self.x, buf);
                    }
                    if self.y != 0f64 {
                        ::prost::encoding::double::encode(2u32, &self.y, buf);
                    }
                    if self.heading != 0f64 {
                        ::prost::encoding::double::encode(3u32, &self.heading, buf);
                    }
                    if self.intervals != 0u64 {
                        ::prost::encoding::uint64::encode(4u32, &self.intervals, buf);
                    }
                    if self.split != false {
                        ::prost::encoding::bool::encode(5u32, &self.split, buf);
                    }
                    if self.fix_translation != false {
                        ::prost::encoding::bool::encode(
                            6u32,
                            &self.fix_translation,
                            buf,
                        );
                    }
                    if self.fix_heading != false {
                        ::prost::encoding::bool::encode(7u32, &self.fix_heading, buf);
                    }
                    if self.override_intervals != false {
                        ::prost::encoding::bool::encode(
                            8u32,
                            &self.override_intervals,
                            buf,
                        );
                    }
                }
                #[allow(unused_variables)]
                fn merge_field(
                    &mut self,
                    tag: u32,
                    wire_type: ::prost::encoding::wire_type::WireType,
                    buf: &mut impl ::prost::bytes::Buf,
                    ctx: ::prost::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::prost::DecodeError> {
                    const STRUCT_NAME: &'static str = "DoubleWaypoint";
                    match tag {
                        1u32 => {
                            let mut value = &mut self.x;
                            ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "x");
                                    error
                                })
                        }
                        2u32 => {
                            let mut value = &mut self.y;
                            ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "y");
                                    error
                                })
                        }
                        3u32 => {
                            let mut value = &mut self.heading;
                            ::prost::encoding::double::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "heading");
                                    error
                                })
                        }
                        4u32 => {
                            let mut value = &mut self.intervals;
                            ::prost::encoding::uint64::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "intervals");
                                    error
                                })
                        }
                        5u32 => {
                            let mut value = &mut self.split;
                            ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "split");
                                    error
                                })
                        }
                        6u32 => {
                            let mut value = &mut self.fix_translation;
                            ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "fix_translation");
                                    error
                                })
                        }
                        7u32 => {
                            let mut value = &mut self.fix_heading;
                            ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "fix_heading");
                                    error
                                })
                        }
                        8u32 => {
                            let mut value = &mut self.override_intervals;
                            ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "override_intervals");
                                    error
                                })
                        }
                        _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0
                        + if self.x != 0f64 {
                            ::prost::encoding::double::encoded_len(1u32, &self.x)
                        } else {
                            0
                        }
                        + if self.y != 0f64 {
                            ::prost::encoding::double::encoded_len(2u32, &self.y)
                        } else {
                            0
                        }
                        + if self.heading != 0f64 {
                            ::prost::encoding::double::encoded_len(3u32, &self.heading)
                        } else {
                            0
                        }
                        + if self.intervals != 0u64 {
                            ::prost::encoding::uint64::encoded_len(4u32, &self.intervals)
                        } else {
                            0
                        }
                        + if self.split != false {
                            ::prost::encoding::bool::encoded_len(5u32, &self.split)
                        } else {
                            0
                        }
                        + if self.fix_translation != false {
                            ::prost::encoding::bool::encoded_len(
                                6u32,
                                &self.fix_translation,
                            )
                        } else {
                            0
                        }
                        + if self.fix_heading != false {
                            ::prost::encoding::bool::encoded_len(7u32, &self.fix_heading)
                        } else {
                            0
                        }
                        + if self.override_intervals != false {
                            ::prost::encoding::bool::encoded_len(
                                8u32,
                                &self.override_intervals,
                            )
                        } else {
                            0
                        }
                }
                fn clear(&mut self) {
                    self.x = 0f64;
                    self.y = 0f64;
                    self.heading = 0f64;
                    self.intervals = 0u64;
                    self.split = false;
                    self.fix_translation = false;
                    self.fix_heading = false;
                    self.override_intervals = false;
                }
            }
            impl ::core::default::Default for DoubleWaypoint {
                fn default() -> Self {
                    DoubleWaypoint {
                        x: 0f64,
                        y: 0f64,
                        heading: 0f64,
                        intervals: 0u64,
                        split: false,
                        fix_translation: false,
                        fix_heading: false,
                        override_intervals: false,
                    }
                }
            }
            impl ::core::fmt::Debug for DoubleWaypoint {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let mut builder = f.debug_struct("DoubleWaypoint");
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.x)
                        };
                        builder.field("x", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.y)
                        };
                        builder.field("y", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.heading)
                        };
                        builder.field("heading", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.intervals)
                        };
                        builder.field("intervals", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.split)
                        };
                        builder.field("split", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.fix_translation)
                        };
                        builder.field("fix_translation", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.fix_heading)
                        };
                        builder.field("fix_heading", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.override_intervals)
                        };
                        builder.field("override_intervals", &wrapper)
                    };
                    builder.finish()
                }
            }
            pub struct RequiredDoubleWaypoint {
                pub x: f64,
                pub y: f64,
                pub heading: f64,
                pub intervals: u64,
                pub split: bool,
                pub fix_translation: bool,
                pub fix_heading: bool,
                pub override_intervals: bool,
            }
            impl From<RequiredDoubleWaypoint> for DoubleWaypoint {
                fn from(value: RequiredDoubleWaypoint) -> Self {
                    DoubleWaypoint {
                        x: value.x.into(),
                        y: value.y.into(),
                        heading: value.heading.into(),
                        intervals: value.intervals.into(),
                        split: value.split.into(),
                        fix_translation: value.fix_translation.into(),
                        fix_heading: value.fix_heading.into(),
                        override_intervals: value.override_intervals.into(),
                    }
                }
            }
            #[required(prefix = "Required")]
            pub struct ExprWaypoint {
                #[prost(message, optional, tag = "1")]
                pub x: ::core::option::Option<super::Expr>,
                #[prost(message, optional, tag = "2")]
                pub y: ::core::option::Option<super::Expr>,
                #[prost(message, optional, tag = "3")]
                pub heading: ::core::option::Option<super::Expr>,
                #[prost(uint64, tag = "4")]
                pub intervals: u64,
                #[prost(bool, tag = "5")]
                pub split: bool,
                #[prost(bool, tag = "6")]
                pub fix_translation: bool,
                #[prost(bool, tag = "7")]
                pub fix_heading: bool,
                #[prost(bool, tag = "8")]
                pub override_intervals: bool,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ExprWaypoint {
                #[inline]
                fn clone(&self) -> ExprWaypoint {
                    ExprWaypoint {
                        x: ::core::clone::Clone::clone(&self.x),
                        y: ::core::clone::Clone::clone(&self.y),
                        heading: ::core::clone::Clone::clone(&self.heading),
                        intervals: ::core::clone::Clone::clone(&self.intervals),
                        split: ::core::clone::Clone::clone(&self.split),
                        fix_translation: ::core::clone::Clone::clone(
                            &self.fix_translation,
                        ),
                        fix_heading: ::core::clone::Clone::clone(&self.fix_heading),
                        override_intervals: ::core::clone::Clone::clone(
                            &self.override_intervals,
                        ),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ExprWaypoint {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ExprWaypoint {
                #[inline]
                fn eq(&self, other: &ExprWaypoint) -> bool {
                    self.intervals == other.intervals && self.split == other.split
                        && self.fix_translation == other.fix_translation
                        && self.fix_heading == other.fix_heading
                        && self.override_intervals == other.override_intervals
                        && self.x == other.x && self.y == other.y
                        && self.heading == other.heading
                }
            }
            impl ::prost::Message for ExprWaypoint {
                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                    if let Some(ref msg) = self.x {
                        ::prost::encoding::message::encode(1u32, msg, buf);
                    }
                    if let Some(ref msg) = self.y {
                        ::prost::encoding::message::encode(2u32, msg, buf);
                    }
                    if let Some(ref msg) = self.heading {
                        ::prost::encoding::message::encode(3u32, msg, buf);
                    }
                    if self.intervals != 0u64 {
                        ::prost::encoding::uint64::encode(4u32, &self.intervals, buf);
                    }
                    if self.split != false {
                        ::prost::encoding::bool::encode(5u32, &self.split, buf);
                    }
                    if self.fix_translation != false {
                        ::prost::encoding::bool::encode(
                            6u32,
                            &self.fix_translation,
                            buf,
                        );
                    }
                    if self.fix_heading != false {
                        ::prost::encoding::bool::encode(7u32, &self.fix_heading, buf);
                    }
                    if self.override_intervals != false {
                        ::prost::encoding::bool::encode(
                            8u32,
                            &self.override_intervals,
                            buf,
                        );
                    }
                }
                #[allow(unused_variables)]
                fn merge_field(
                    &mut self,
                    tag: u32,
                    wire_type: ::prost::encoding::wire_type::WireType,
                    buf: &mut impl ::prost::bytes::Buf,
                    ctx: ::prost::encoding::DecodeContext,
                ) -> ::core::result::Result<(), ::prost::DecodeError> {
                    const STRUCT_NAME: &'static str = "ExprWaypoint";
                    match tag {
                        1u32 => {
                            let mut value = &mut self.x;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "x");
                                    error
                                })
                        }
                        2u32 => {
                            let mut value = &mut self.y;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "y");
                                    error
                                })
                        }
                        3u32 => {
                            let mut value = &mut self.heading;
                            ::prost::encoding::message::merge(
                                    wire_type,
                                    value.get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "heading");
                                    error
                                })
                        }
                        4u32 => {
                            let mut value = &mut self.intervals;
                            ::prost::encoding::uint64::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "intervals");
                                    error
                                })
                        }
                        5u32 => {
                            let mut value = &mut self.split;
                            ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "split");
                                    error
                                })
                        }
                        6u32 => {
                            let mut value = &mut self.fix_translation;
                            ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "fix_translation");
                                    error
                                })
                        }
                        7u32 => {
                            let mut value = &mut self.fix_heading;
                            ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "fix_heading");
                                    error
                                })
                        }
                        8u32 => {
                            let mut value = &mut self.override_intervals;
                            ::prost::encoding::bool::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "override_intervals");
                                    error
                                })
                        }
                        _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
                #[inline]
                fn encoded_len(&self) -> usize {
                    0
                        + self
                            .x
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(1u32, msg),
                            )
                        + self
                            .y
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(2u32, msg),
                            )
                        + self
                            .heading
                            .as_ref()
                            .map_or(
                                0,
                                |msg| ::prost::encoding::message::encoded_len(3u32, msg),
                            )
                        + if self.intervals != 0u64 {
                            ::prost::encoding::uint64::encoded_len(4u32, &self.intervals)
                        } else {
                            0
                        }
                        + if self.split != false {
                            ::prost::encoding::bool::encoded_len(5u32, &self.split)
                        } else {
                            0
                        }
                        + if self.fix_translation != false {
                            ::prost::encoding::bool::encoded_len(
                                6u32,
                                &self.fix_translation,
                            )
                        } else {
                            0
                        }
                        + if self.fix_heading != false {
                            ::prost::encoding::bool::encoded_len(7u32, &self.fix_heading)
                        } else {
                            0
                        }
                        + if self.override_intervals != false {
                            ::prost::encoding::bool::encoded_len(
                                8u32,
                                &self.override_intervals,
                            )
                        } else {
                            0
                        }
                }
                fn clear(&mut self) {
                    self.x = ::core::option::Option::None;
                    self.y = ::core::option::Option::None;
                    self.heading = ::core::option::Option::None;
                    self.intervals = 0u64;
                    self.split = false;
                    self.fix_translation = false;
                    self.fix_heading = false;
                    self.override_intervals = false;
                }
            }
            impl ::core::default::Default for ExprWaypoint {
                fn default() -> Self {
                    ExprWaypoint {
                        x: ::core::default::Default::default(),
                        y: ::core::default::Default::default(),
                        heading: ::core::default::Default::default(),
                        intervals: 0u64,
                        split: false,
                        fix_translation: false,
                        fix_heading: false,
                        override_intervals: false,
                    }
                }
            }
            impl ::core::fmt::Debug for ExprWaypoint {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let mut builder = f.debug_struct("ExprWaypoint");
                    let builder = {
                        let wrapper = &self.x;
                        builder.field("x", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.y;
                        builder.field("y", &wrapper)
                    };
                    let builder = {
                        let wrapper = &self.heading;
                        builder.field("heading", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.intervals)
                        };
                        builder.field("intervals", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.split)
                        };
                        builder.field("split", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.fix_translation)
                        };
                        builder.field("fix_translation", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.fix_heading)
                        };
                        builder.field("fix_heading", &wrapper)
                    };
                    let builder = {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&self.override_intervals)
                        };
                        builder.field("override_intervals", &wrapper)
                    };
                    builder.finish()
                }
            }
            pub struct RequiredExprWaypoint {
                pub x: super::RequiredExpr,
                pub y: super::RequiredExpr,
                pub heading: super::RequiredExpr,
                pub intervals: u64,
                pub split: bool,
                pub fix_translation: bool,
                pub fix_heading: bool,
                pub override_intervals: bool,
            }
            impl From<RequiredExprWaypoint> for ExprWaypoint {
                fn from(value: RequiredExprWaypoint) -> Self {
                    ExprWaypoint {
                        x: Some(value.x.into()),
                        y: Some(value.y.into()),
                        heading: Some(value.heading.into()),
                        intervals: value.intervals.into(),
                        split: value.split.into(),
                        fix_translation: value.fix_translation.into(),
                        fix_heading: value.fix_heading.into(),
                        override_intervals: value.override_intervals.into(),
                    }
                }
            }
            impl serde::Serialize for DoubleWaypoint {
                #[allow(deprecated)]
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> std::result::Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeStruct;
                    let mut len = 0;
                    if self.x != 0. {
                        len += 1;
                    }
                    if self.y != 0. {
                        len += 1;
                    }
                    if self.heading != 0. {
                        len += 1;
                    }
                    if self.intervals != 0 {
                        len += 1;
                    }
                    if self.split {
                        len += 1;
                    }
                    if self.fix_translation {
                        len += 1;
                    }
                    if self.fix_heading {
                        len += 1;
                    }
                    if self.override_intervals {
                        len += 1;
                    }
                    let mut struct_ser = serializer
                        .serialize_struct(
                            "entity.parameters.waypoint.DoubleWaypoint",
                            len,
                        )?;
                    if self.x != 0. {
                        struct_ser.serialize_field("x", &self.x)?;
                    }
                    if self.y != 0. {
                        struct_ser.serialize_field("y", &self.y)?;
                    }
                    if self.heading != 0. {
                        struct_ser.serialize_field("heading", &self.heading)?;
                    }
                    if self.intervals != 0 {
                        #[allow(clippy::needless_borrow)]
                        #[allow(clippy::needless_borrows_for_generic_args)]
                        struct_ser
                            .serialize_field(
                                "intervals",
                                ToString::to_string(&self.intervals).as_str(),
                            )?;
                    }
                    if self.split {
                        struct_ser.serialize_field("split", &self.split)?;
                    }
                    if self.fix_translation {
                        struct_ser
                            .serialize_field("fixTranslation", &self.fix_translation)?;
                    }
                    if self.fix_heading {
                        struct_ser.serialize_field("fixHeading", &self.fix_heading)?;
                    }
                    if self.override_intervals {
                        struct_ser
                            .serialize_field(
                                "overrideIntervals",
                                &self.override_intervals,
                            )?;
                    }
                    struct_ser.end()
                }
            }
            impl<'de> serde::Deserialize<'de> for DoubleWaypoint {
                #[allow(deprecated)]
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    const FIELDS: &[&str] = &[
                        "x",
                        "y",
                        "heading",
                        "intervals",
                        "split",
                        "fix_translation",
                        "fixTranslation",
                        "fix_heading",
                        "fixHeading",
                        "override_intervals",
                        "overrideIntervals",
                    ];
                    #[allow(clippy::enum_variant_names)]
                    enum GeneratedField {
                        X,
                        Y,
                        Heading,
                        Intervals,
                        Split,
                        FixTranslation,
                        FixHeading,
                        OverrideIntervals,
                    }
                    impl<'de> serde::Deserialize<'de> for GeneratedField {
                        fn deserialize<D>(
                            deserializer: D,
                        ) -> std::result::Result<GeneratedField, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            struct GeneratedVisitor;
                            impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                                type Value = GeneratedField;
                                fn expecting(
                                    &self,
                                    formatter: &mut std::fmt::Formatter<'_>,
                                ) -> std::fmt::Result {
                                    formatter
                                        .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                                }
                                #[allow(unused_variables)]
                                fn visit_str<E>(
                                    self,
                                    value: &str,
                                ) -> std::result::Result<GeneratedField, E>
                                where
                                    E: serde::de::Error,
                                {
                                    match value {
                                        "x" => Ok(GeneratedField::X),
                                        "y" => Ok(GeneratedField::Y),
                                        "heading" => Ok(GeneratedField::Heading),
                                        "intervals" => Ok(GeneratedField::Intervals),
                                        "split" => Ok(GeneratedField::Split),
                                        "fixTranslation" | "fix_translation" => {
                                            Ok(GeneratedField::FixTranslation)
                                        }
                                        "fixHeading" | "fix_heading" => {
                                            Ok(GeneratedField::FixHeading)
                                        }
                                        "overrideIntervals" | "override_intervals" => {
                                            Ok(GeneratedField::OverrideIntervals)
                                        }
                                        _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                    }
                                }
                            }
                            deserializer.deserialize_identifier(GeneratedVisitor)
                        }
                    }
                    struct GeneratedVisitor;
                    impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                        type Value = DoubleWaypoint;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> std::fmt::Result {
                            formatter
                                .write_str(
                                    "struct entity.parameters.waypoint.DoubleWaypoint",
                                )
                        }
                        fn visit_map<V>(
                            self,
                            mut map_: V,
                        ) -> std::result::Result<DoubleWaypoint, V::Error>
                        where
                            V: serde::de::MapAccess<'de>,
                        {
                            let mut x__ = None;
                            let mut y__ = None;
                            let mut heading__ = None;
                            let mut intervals__ = None;
                            let mut split__ = None;
                            let mut fix_translation__ = None;
                            let mut fix_heading__ = None;
                            let mut override_intervals__ = None;
                            while let Some(k) = map_.next_key()? {
                                match k {
                                    GeneratedField::X => {
                                        if x__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("x"));
                                        }
                                        x__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                    GeneratedField::Y => {
                                        if y__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("y"));
                                        }
                                        y__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                    GeneratedField::Heading => {
                                        if heading__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("heading"));
                                        }
                                        heading__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                    GeneratedField::Intervals => {
                                        if intervals__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("intervals"));
                                        }
                                        intervals__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                    GeneratedField::Split => {
                                        if split__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("split"));
                                        }
                                        split__ = Some(map_.next_value()?);
                                    }
                                    GeneratedField::FixTranslation => {
                                        if fix_translation__.is_some() {
                                            return Err(
                                                serde::de::Error::duplicate_field("fixTranslation"),
                                            );
                                        }
                                        fix_translation__ = Some(map_.next_value()?);
                                    }
                                    GeneratedField::FixHeading => {
                                        if fix_heading__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("fixHeading"));
                                        }
                                        fix_heading__ = Some(map_.next_value()?);
                                    }
                                    GeneratedField::OverrideIntervals => {
                                        if override_intervals__.is_some() {
                                            return Err(
                                                serde::de::Error::duplicate_field("overrideIntervals"),
                                            );
                                        }
                                        override_intervals__ = Some(map_.next_value()?);
                                    }
                                }
                            }
                            Ok(DoubleWaypoint {
                                x: x__.unwrap_or_default(),
                                y: y__.unwrap_or_default(),
                                heading: heading__.unwrap_or_default(),
                                intervals: intervals__.unwrap_or_default(),
                                split: split__.unwrap_or_default(),
                                fix_translation: fix_translation__.unwrap_or_default(),
                                fix_heading: fix_heading__.unwrap_or_default(),
                                override_intervals: override_intervals__.unwrap_or_default(),
                            })
                        }
                    }
                    deserializer
                        .deserialize_struct(
                            "entity.parameters.waypoint.DoubleWaypoint",
                            FIELDS,
                            GeneratedVisitor,
                        )
                }
            }
            impl serde::Serialize for ExprWaypoint {
                #[allow(deprecated)]
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> std::result::Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeStruct;
                    let mut len = 0;
                    if self.x.is_some() {
                        len += 1;
                    }
                    if self.y.is_some() {
                        len += 1;
                    }
                    if self.heading.is_some() {
                        len += 1;
                    }
                    if self.intervals != 0 {
                        len += 1;
                    }
                    if self.split {
                        len += 1;
                    }
                    if self.fix_translation {
                        len += 1;
                    }
                    if self.fix_heading {
                        len += 1;
                    }
                    if self.override_intervals {
                        len += 1;
                    }
                    let mut struct_ser = serializer
                        .serialize_struct(
                            "entity.parameters.waypoint.ExprWaypoint",
                            len,
                        )?;
                    if let Some(v) = self.x.as_ref() {
                        struct_ser.serialize_field("x", v)?;
                    }
                    if let Some(v) = self.y.as_ref() {
                        struct_ser.serialize_field("y", v)?;
                    }
                    if let Some(v) = self.heading.as_ref() {
                        struct_ser.serialize_field("heading", v)?;
                    }
                    if self.intervals != 0 {
                        #[allow(clippy::needless_borrow)]
                        #[allow(clippy::needless_borrows_for_generic_args)]
                        struct_ser
                            .serialize_field(
                                "intervals",
                                ToString::to_string(&self.intervals).as_str(),
                            )?;
                    }
                    if self.split {
                        struct_ser.serialize_field("split", &self.split)?;
                    }
                    if self.fix_translation {
                        struct_ser
                            .serialize_field("fixTranslation", &self.fix_translation)?;
                    }
                    if self.fix_heading {
                        struct_ser.serialize_field("fixHeading", &self.fix_heading)?;
                    }
                    if self.override_intervals {
                        struct_ser
                            .serialize_field(
                                "overrideIntervals",
                                &self.override_intervals,
                            )?;
                    }
                    struct_ser.end()
                }
            }
            impl<'de> serde::Deserialize<'de> for ExprWaypoint {
                #[allow(deprecated)]
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    const FIELDS: &[&str] = &[
                        "x",
                        "y",
                        "heading",
                        "intervals",
                        "split",
                        "fix_translation",
                        "fixTranslation",
                        "fix_heading",
                        "fixHeading",
                        "override_intervals",
                        "overrideIntervals",
                    ];
                    #[allow(clippy::enum_variant_names)]
                    enum GeneratedField {
                        X,
                        Y,
                        Heading,
                        Intervals,
                        Split,
                        FixTranslation,
                        FixHeading,
                        OverrideIntervals,
                    }
                    impl<'de> serde::Deserialize<'de> for GeneratedField {
                        fn deserialize<D>(
                            deserializer: D,
                        ) -> std::result::Result<GeneratedField, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            struct GeneratedVisitor;
                            impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                                type Value = GeneratedField;
                                fn expecting(
                                    &self,
                                    formatter: &mut std::fmt::Formatter<'_>,
                                ) -> std::fmt::Result {
                                    formatter
                                        .write_fmt(format_args!("expected one of: {0:?}", &FIELDS))
                                }
                                #[allow(unused_variables)]
                                fn visit_str<E>(
                                    self,
                                    value: &str,
                                ) -> std::result::Result<GeneratedField, E>
                                where
                                    E: serde::de::Error,
                                {
                                    match value {
                                        "x" => Ok(GeneratedField::X),
                                        "y" => Ok(GeneratedField::Y),
                                        "heading" => Ok(GeneratedField::Heading),
                                        "intervals" => Ok(GeneratedField::Intervals),
                                        "split" => Ok(GeneratedField::Split),
                                        "fixTranslation" | "fix_translation" => {
                                            Ok(GeneratedField::FixTranslation)
                                        }
                                        "fixHeading" | "fix_heading" => {
                                            Ok(GeneratedField::FixHeading)
                                        }
                                        "overrideIntervals" | "override_intervals" => {
                                            Ok(GeneratedField::OverrideIntervals)
                                        }
                                        _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                                    }
                                }
                            }
                            deserializer.deserialize_identifier(GeneratedVisitor)
                        }
                    }
                    struct GeneratedVisitor;
                    impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                        type Value = ExprWaypoint;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter<'_>,
                        ) -> std::fmt::Result {
                            formatter
                                .write_str("struct entity.parameters.waypoint.ExprWaypoint")
                        }
                        fn visit_map<V>(
                            self,
                            mut map_: V,
                        ) -> std::result::Result<ExprWaypoint, V::Error>
                        where
                            V: serde::de::MapAccess<'de>,
                        {
                            let mut x__ = None;
                            let mut y__ = None;
                            let mut heading__ = None;
                            let mut intervals__ = None;
                            let mut split__ = None;
                            let mut fix_translation__ = None;
                            let mut fix_heading__ = None;
                            let mut override_intervals__ = None;
                            while let Some(k) = map_.next_key()? {
                                match k {
                                    GeneratedField::X => {
                                        if x__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("x"));
                                        }
                                        x__ = map_.next_value()?;
                                    }
                                    GeneratedField::Y => {
                                        if y__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("y"));
                                        }
                                        y__ = map_.next_value()?;
                                    }
                                    GeneratedField::Heading => {
                                        if heading__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("heading"));
                                        }
                                        heading__ = map_.next_value()?;
                                    }
                                    GeneratedField::Intervals => {
                                        if intervals__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("intervals"));
                                        }
                                        intervals__ = Some(
                                            map_
                                                .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                                .0,
                                        );
                                    }
                                    GeneratedField::Split => {
                                        if split__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("split"));
                                        }
                                        split__ = Some(map_.next_value()?);
                                    }
                                    GeneratedField::FixTranslation => {
                                        if fix_translation__.is_some() {
                                            return Err(
                                                serde::de::Error::duplicate_field("fixTranslation"),
                                            );
                                        }
                                        fix_translation__ = Some(map_.next_value()?);
                                    }
                                    GeneratedField::FixHeading => {
                                        if fix_heading__.is_some() {
                                            return Err(serde::de::Error::duplicate_field("fixHeading"));
                                        }
                                        fix_heading__ = Some(map_.next_value()?);
                                    }
                                    GeneratedField::OverrideIntervals => {
                                        if override_intervals__.is_some() {
                                            return Err(
                                                serde::de::Error::duplicate_field("overrideIntervals"),
                                            );
                                        }
                                        override_intervals__ = Some(map_.next_value()?);
                                    }
                                }
                            }
                            Ok(ExprWaypoint {
                                x: x__,
                                y: y__,
                                heading: heading__,
                                intervals: intervals__.unwrap_or_default(),
                                split: split__.unwrap_or_default(),
                                fix_translation: fix_translation__.unwrap_or_default(),
                                fix_heading: fix_heading__.unwrap_or_default(),
                                override_intervals: override_intervals__.unwrap_or_default(),
                            })
                        }
                    }
                    deserializer
                        .deserialize_struct(
                            "entity.parameters.waypoint.ExprWaypoint",
                            FIELDS,
                            GeneratedVisitor,
                        )
                }
            }
        }
    }
}
