#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use crate::{entity::Expr, to_double::ToDouble};
pub mod entity {
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
    pub type ValidDifferentialSample = DifferentialSample;
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
    pub type ValidDriveType = DriveType;
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
    pub type ValidExpr = Expr;
    pub struct ProjectFile {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub config: ::core::option::Option<parameters::robotconfig::ExprRobotConfig>,
        #[prost(enumeration = "DriveType", tag = "3")]
        pub drive_type: i32,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ProjectFile {
        #[inline]
        fn clone(&self) -> ProjectFile {
            ProjectFile {
                name: ::core::clone::Clone::clone(&self.name),
                config: ::core::clone::Clone::clone(&self.config),
                drive_type: ::core::clone::Clone::clone(&self.drive_type),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ProjectFile {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ProjectFile {
        #[inline]
        fn eq(&self, other: &ProjectFile) -> bool {
            self.drive_type == other.drive_type && self.name == other.name
                && self.config == other.config
        }
    }
    impl ::prost::Message for ProjectFile {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.name != "" {
                ::prost::encoding::string::encode(1u32, &self.name, buf);
            }
            if let Some(ref msg) = self.config {
                ::prost::encoding::message::encode(2u32, msg, buf);
            }
            if self.drive_type != DriveType::default() as i32 {
                ::prost::encoding::int32::encode(3u32, &self.drive_type, buf);
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
            const STRUCT_NAME: &'static str = "ProjectFile";
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
                3u32 => {
                    let mut value = &mut self.drive_type;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "drive_type");
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
                    .config
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(2u32, msg))
                + if self.drive_type != DriveType::default() as i32 {
                    ::prost::encoding::int32::encoded_len(3u32, &self.drive_type)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.name.clear();
            self.config = ::core::option::Option::None;
            self.drive_type = DriveType::default() as i32;
        }
    }
    impl ::core::default::Default for ProjectFile {
        fn default() -> Self {
            ProjectFile {
                name: ::prost::alloc::string::String::new(),
                config: ::core::default::Default::default(),
                drive_type: DriveType::default() as i32,
            }
        }
    }
    impl ::core::fmt::Debug for ProjectFile {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("ProjectFile");
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
                let wrapper = &self.config;
                builder.field("config", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a i32);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let res: ::core::result::Result<DriveType, _> = ::core::convert::TryFrom::try_from(
                                *self.0,
                            );
                            match res {
                                Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                            }
                        }
                    }
                    ScalarWrapper(&self.drive_type)
                };
                builder.field("drive_type", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl ProjectFile {
        ///Returns the enum value of `drive_type`, or the default if the field is set to an invalid enum value.
        pub fn drive_type(&self) -> DriveType {
            ::core::convert::TryFrom::try_from(self.drive_type)
                .unwrap_or(DriveType::default())
        }
        ///Sets `drive_type` to the provided enum value.
        pub fn set_drive_type(&mut self, value: DriveType) {
            self.drive_type = value as i32;
        }
    }
    pub struct ValidProjectFile {
        pub name: ::prost::alloc::string::String,
        pub config: parameters::robotconfig::ValidExprRobotConfig,
        pub drive_type: i32,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ValidProjectFile {
        #[inline]
        fn clone(&self) -> ValidProjectFile {
            ValidProjectFile {
                name: ::core::clone::Clone::clone(&self.name),
                config: ::core::clone::Clone::clone(&self.config),
                drive_type: ::core::clone::Clone::clone(&self.drive_type),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ValidProjectFile {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ValidProjectFile {
        #[inline]
        fn eq(&self, other: &ValidProjectFile) -> bool {
            self.drive_type == other.drive_type && self.name == other.name
                && self.config == other.config
        }
    }
    impl TryFrom<ProjectFile> for ValidProjectFile {
        type Error = String;
        fn try_from(optional: ProjectFile) -> Result<ValidProjectFile, Self::Error> {
            Ok(ValidProjectFile {
                name: optional.name,
                config: match optional.config {
                    Some(object) => {
                        match object
                            .try_into()
                            .map_err(|e| {
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("{0}", e))
                                    })
                                    .to_string()
                            })
                        {
                            Ok(valid_object) => valid_object,
                            Err(e) => return Err(e),
                        }
                    }
                    None => return Err("config is missing".to_string()),
                },
                drive_type: optional.drive_type,
            })
        }
    }
    impl From<ValidProjectFile> for ProjectFile {
        fn from(valid: ValidProjectFile) -> ProjectFile {
            ProjectFile {
                name: valid.name,
                config: Some(valid.config.into()),
                drive_type: valid.drive_type,
            }
        }
    }
    impl crate::validate::Valid for ValidProjectFile {
        type Optional = ProjectFile;
        fn optionize(self) -> ProjectFile {
            self.into()
        }
    }
    impl crate::validate::Validate for ProjectFile {
        type Valid = ValidProjectFile;
        fn validate(self) -> Result<ValidProjectFile, String> {
            self.try_into()
        }
    }
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
    pub type ValidForceVector = ForceVector;
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
    pub struct ValidSwerveSample {
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
        pub fl: ValidForceVector,
        pub fr: ValidForceVector,
        pub bl: ValidForceVector,
        pub br: ValidForceVector,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ValidSwerveSample {
        #[inline]
        fn clone(&self) -> ValidSwerveSample {
            let _: ::core::clone::AssertParamIsClone<f64>;
            let _: ::core::clone::AssertParamIsClone<ValidForceVector>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ValidSwerveSample {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ValidSwerveSample {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ValidSwerveSample {
        #[inline]
        fn eq(&self, other: &ValidSwerveSample) -> bool {
            self.t == other.t && self.x == other.x && self.y == other.y
                && self.heading == other.heading && self.vx == other.vx
                && self.vy == other.vy && self.omega == other.omega
                && self.ax == other.ax && self.ay == other.ay
                && self.alpha == other.alpha && self.fl == other.fl
                && self.fr == other.fr && self.bl == other.bl && self.br == other.br
        }
    }
    impl TryFrom<SwerveSample> for ValidSwerveSample {
        type Error = String;
        fn try_from(optional: SwerveSample) -> Result<ValidSwerveSample, Self::Error> {
            Ok(ValidSwerveSample {
                t: optional.t,
                x: optional.x,
                y: optional.y,
                heading: optional.heading,
                vx: optional.vx,
                vy: optional.vy,
                omega: optional.omega,
                ax: optional.ax,
                ay: optional.ay,
                alpha: optional.alpha,
                fl: match optional.fl {
                    Some(object) => {
                        match object
                            .try_into()
                            .map_err(|e| {
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("{0}", e))
                                    })
                                    .to_string()
                            })
                        {
                            Ok(valid_object) => valid_object,
                            Err(e) => return Err(e),
                        }
                    }
                    None => return Err("fl is missing".to_string()),
                },
                fr: match optional.fr {
                    Some(object) => {
                        match object
                            .try_into()
                            .map_err(|e| {
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("{0}", e))
                                    })
                                    .to_string()
                            })
                        {
                            Ok(valid_object) => valid_object,
                            Err(e) => return Err(e),
                        }
                    }
                    None => return Err("fr is missing".to_string()),
                },
                bl: match optional.bl {
                    Some(object) => {
                        match object
                            .try_into()
                            .map_err(|e| {
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("{0}", e))
                                    })
                                    .to_string()
                            })
                        {
                            Ok(valid_object) => valid_object,
                            Err(e) => return Err(e),
                        }
                    }
                    None => return Err("bl is missing".to_string()),
                },
                br: match optional.br {
                    Some(object) => {
                        match object
                            .try_into()
                            .map_err(|e| {
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("{0}", e))
                                    })
                                    .to_string()
                            })
                        {
                            Ok(valid_object) => valid_object,
                            Err(e) => return Err(e),
                        }
                    }
                    None => return Err("br is missing".to_string()),
                },
            })
        }
    }
    impl From<ValidSwerveSample> for SwerveSample {
        fn from(valid: ValidSwerveSample) -> SwerveSample {
            SwerveSample {
                t: valid.t,
                x: valid.x,
                y: valid.y,
                heading: valid.heading,
                vx: valid.vx,
                vy: valid.vy,
                omega: valid.omega,
                ax: valid.ax,
                ay: valid.ay,
                alpha: valid.alpha,
                fl: Some(valid.fl.into()),
                fr: Some(valid.fr.into()),
                bl: Some(valid.bl.into()),
                br: Some(valid.br.into()),
            }
        }
    }
    impl crate::validate::Valid for ValidSwerveSample {
        type Optional = SwerveSample;
        fn optionize(self) -> SwerveSample {
            self.into()
        }
    }
    impl crate::validate::Validate for SwerveSample {
        type Valid = ValidSwerveSample;
        fn validate(self) -> Result<ValidSwerveSample, String> {
            self.try_into()
        }
    }
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
    pub type ValidSwerveTrajectory = SwerveTrajectory;
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
    pub type ValidDifferentialTrajectory = DifferentialTrajectory;
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
    pub struct ValidGenerationOutput {
        pub splits: ::prost::alloc::vec::Vec<u64>,
        pub waypoints: ::prost::alloc::vec::Vec<f64>,
        pub config: parameters::robotconfig::ValidDoubleRobotConfig,
        pub trajectory: generation_output::ValidTrajectory,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ValidGenerationOutput {
        #[inline]
        fn clone(&self) -> ValidGenerationOutput {
            ValidGenerationOutput {
                splits: ::core::clone::Clone::clone(&self.splits),
                waypoints: ::core::clone::Clone::clone(&self.waypoints),
                config: ::core::clone::Clone::clone(&self.config),
                trajectory: ::core::clone::Clone::clone(&self.trajectory),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ValidGenerationOutput {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ValidGenerationOutput {
        #[inline]
        fn eq(&self, other: &ValidGenerationOutput) -> bool {
            self.splits == other.splits && self.waypoints == other.waypoints
                && self.config == other.config && self.trajectory == other.trajectory
        }
    }
    impl TryFrom<GenerationOutput> for ValidGenerationOutput {
        type Error = String;
        fn try_from(
            optional: GenerationOutput,
        ) -> Result<ValidGenerationOutput, Self::Error> {
            Ok(ValidGenerationOutput {
                splits: optional.splits,
                waypoints: optional.waypoints,
                config: match optional.config {
                    Some(object) => {
                        match object
                            .try_into()
                            .map_err(|e| {
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("{0}", e))
                                    })
                                    .to_string()
                            })
                        {
                            Ok(valid_object) => valid_object,
                            Err(e) => return Err(e),
                        }
                    }
                    None => return Err("config is missing".to_string()),
                },
                trajectory: match optional.trajectory {
                    Some(object) => {
                        match object
                            .try_into()
                            .map_err(|e| {
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("{0}", e))
                                    })
                                    .to_string()
                            })
                        {
                            Ok(valid_object) => valid_object,
                            Err(e) => return Err(e),
                        }
                    }
                    None => return Err("trajectory is missing".to_string()),
                },
            })
        }
    }
    impl From<ValidGenerationOutput> for GenerationOutput {
        fn from(valid: ValidGenerationOutput) -> GenerationOutput {
            GenerationOutput {
                splits: valid.splits,
                waypoints: valid.waypoints,
                config: Some(valid.config.into()),
                trajectory: Some(valid.trajectory.into()),
            }
        }
    }
    impl crate::validate::Valid for ValidGenerationOutput {
        type Optional = GenerationOutput;
        fn optionize(self) -> GenerationOutput {
            self.into()
        }
    }
    impl crate::validate::Validate for GenerationOutput {
        type Valid = ValidGenerationOutput;
        fn validate(self) -> Result<ValidGenerationOutput, String> {
            self.try_into()
        }
    }
    /// Nested message and enum types in `GenerationOutput`.
    pub mod generation_output {
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
        pub type ValidTrajectory = Trajectory;
    }
    pub struct TrajectoryFile {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub params: ::core::option::Option<parameters::ExprParameters>,
        #[prost(message, optional, tag = "3")]
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
    pub struct ValidTrajectoryFile {
        pub name: ::prost::alloc::string::String,
        pub params: parameters::ValidExprParameters,
        pub snapshot: parameters::ValidDoubleParameters,
        pub trajectory: ValidGenerationOutput,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ValidTrajectoryFile {
        #[inline]
        fn clone(&self) -> ValidTrajectoryFile {
            ValidTrajectoryFile {
                name: ::core::clone::Clone::clone(&self.name),
                params: ::core::clone::Clone::clone(&self.params),
                snapshot: ::core::clone::Clone::clone(&self.snapshot),
                trajectory: ::core::clone::Clone::clone(&self.trajectory),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ValidTrajectoryFile {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ValidTrajectoryFile {
        #[inline]
        fn eq(&self, other: &ValidTrajectoryFile) -> bool {
            self.name == other.name && self.params == other.params
                && self.snapshot == other.snapshot && self.trajectory == other.trajectory
        }
    }
    impl TryFrom<TrajectoryFile> for ValidTrajectoryFile {
        type Error = String;
        fn try_from(
            optional: TrajectoryFile,
        ) -> Result<ValidTrajectoryFile, Self::Error> {
            Ok(ValidTrajectoryFile {
                name: optional.name,
                params: match optional.params {
                    Some(object) => {
                        match object
                            .try_into()
                            .map_err(|e| {
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("{0}", e))
                                    })
                                    .to_string()
                            })
                        {
                            Ok(valid_object) => valid_object,
                            Err(e) => return Err(e),
                        }
                    }
                    None => return Err("params is missing".to_string()),
                },
                snapshot: match optional.snapshot {
                    Some(object) => {
                        match object
                            .try_into()
                            .map_err(|e| {
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("{0}", e))
                                    })
                                    .to_string()
                            })
                        {
                            Ok(valid_object) => valid_object,
                            Err(e) => return Err(e),
                        }
                    }
                    None => return Err("snapshot is missing".to_string()),
                },
                trajectory: match optional.trajectory {
                    Some(object) => {
                        match object
                            .try_into()
                            .map_err(|e| {
                                ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("{0}", e))
                                    })
                                    .to_string()
                            })
                        {
                            Ok(valid_object) => valid_object,
                            Err(e) => return Err(e),
                        }
                    }
                    None => return Err("trajectory is missing".to_string()),
                },
            })
        }
    }
    impl From<ValidTrajectoryFile> for TrajectoryFile {
        fn from(valid: ValidTrajectoryFile) -> TrajectoryFile {
            TrajectoryFile {
                name: valid.name,
                params: Some(valid.params.into()),
                snapshot: Some(valid.snapshot.into()),
                trajectory: Some(valid.trajectory.into()),
            }
        }
    }
    impl crate::validate::Valid for ValidTrajectoryFile {
        type Optional = TrajectoryFile;
        fn optionize(self) -> TrajectoryFile {
            self.into()
        }
    }
    impl crate::validate::Validate for TrajectoryFile {
        type Valid = ValidTrajectoryFile;
        fn validate(self) -> Result<ValidTrajectoryFile, String> {
            self.try_into()
        }
    }
    pub mod parameters {
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
        impl crate::to_double::ToDouble for WaypointIdFirst {
            type DoubleType = WaypointIdFirst;
        }
        pub type ValidWaypointIdFirst = WaypointIdFirst;
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
        impl crate::to_double::ToDouble for WaypointIdLast {
            type DoubleType = WaypointIdLast;
        }
        pub type ValidWaypointIdLast = WaypointIdLast;
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
        impl crate::to_double::ToDouble for WaypointIdx {
            type DoubleType = WaypointIdx;
        }
        pub type ValidWaypointIdx = WaypointIdx;
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
        impl crate::to_double::ToDouble for WaypointId {
            type DoubleType = WaypointId;
        }
        pub struct ValidWaypointId {
            pub id: waypoint_id::ValidId,
        }
        impl crate::to_double::ToDouble for ValidWaypointId {
            type DoubleType = ValidWaypointId;
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ValidWaypointId {
            #[inline]
            fn clone(&self) -> ValidWaypointId {
                let _: ::core::clone::AssertParamIsClone<waypoint_id::ValidId>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for ValidWaypointId {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ValidWaypointId {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ValidWaypointId {
            #[inline]
            fn eq(&self, other: &ValidWaypointId) -> bool {
                self.id == other.id
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ValidWaypointId {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<waypoint_id::ValidId>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for ValidWaypointId {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.id, state)
            }
        }
        impl TryFrom<WaypointId> for ValidWaypointId {
            type Error = String;
            fn try_from(optional: WaypointId) -> Result<ValidWaypointId, Self::Error> {
                Ok(ValidWaypointId {
                    id: match optional.id {
                        Some(object) => {
                            match object
                                .try_into()
                                .map_err(|e| {
                                    ::alloc::__export::must_use({
                                            ::alloc::fmt::format(format_args!("{0}", e))
                                        })
                                        .to_string()
                                })
                            {
                                Ok(valid_object) => valid_object,
                                Err(e) => return Err(e),
                            }
                        }
                        None => return Err("id is missing".to_string()),
                    },
                })
            }
        }
        impl From<ValidWaypointId> for WaypointId {
            fn from(valid: ValidWaypointId) -> WaypointId {
                WaypointId {
                    id: Some(valid.id.into()),
                }
            }
        }
        impl crate::validate::Valid for ValidWaypointId {
            type Optional = WaypointId;
            fn optionize(self) -> WaypointId {
                self.into()
            }
        }
        impl crate::validate::Validate for WaypointId {
            type Valid = ValidWaypointId;
            fn validate(self) -> Result<ValidWaypointId, String> {
                self.try_into()
            }
        }
        /// Nested message and enum types in `WaypointID`.
        pub mod waypoint_id {
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
            impl crate::to_double::ToDouble for Id {
                type DoubleType = Id;
            }
            pub type ValidId = Id;
        }
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
        impl crate::to_double::ToDouble for DoubleParameters {
            type DoubleType = DoubleParameters;
        }
        pub type ValidDoubleParameters = DoubleParameters;
        pub struct ExprParameters {
            #[prost(message, optional, tag = "1")]
            pub target_dt: ::core::option::Option<super::Expr>,
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
        impl crate::to_double::ToDouble for ExprParameters {
            type DoubleType = DoubleParameters;
        }
        impl Into<DoubleParameters> for ExprParameters {
            fn into(self) -> DoubleParameters {
                DoubleParameters {
                    target_dt: self.target_dt.map(Into::into).into(),
                    waypoints: self.waypoints.into_iter().map(Into::into).collect(),
                    constraints: self.constraints.into_iter().map(Into::into).collect(),
                }
            }
        }
        pub struct ValidExprParameters {
            pub target_dt: super::ValidExpr,
            pub waypoints: ::prost::alloc::vec::Vec<waypoint::ExprWaypoint>,
            pub constraints: ::prost::alloc::vec::Vec<constraint::ExprConstraint>,
        }
        impl crate::to_double::ToDouble for ValidExprParameters {
            type DoubleType = ValidDoubleParameters;
        }
        impl Into<ValidDoubleParameters> for ValidExprParameters {
            fn into(self) -> ValidDoubleParameters {
                ValidDoubleParameters {
                    target_dt: self.target_dt.into(),
                    waypoints: self.waypoints.into_iter().map(Into::into).collect(),
                    constraints: self.constraints.into_iter().map(Into::into).collect(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ValidExprParameters {
            #[inline]
            fn clone(&self) -> ValidExprParameters {
                ValidExprParameters {
                    target_dt: ::core::clone::Clone::clone(&self.target_dt),
                    waypoints: ::core::clone::Clone::clone(&self.waypoints),
                    constraints: ::core::clone::Clone::clone(&self.constraints),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ValidExprParameters {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ValidExprParameters {
            #[inline]
            fn eq(&self, other: &ValidExprParameters) -> bool {
                self.target_dt == other.target_dt && self.waypoints == other.waypoints
                    && self.constraints == other.constraints
            }
        }
        impl TryFrom<ExprParameters> for ValidExprParameters {
            type Error = String;
            fn try_from(
                optional: ExprParameters,
            ) -> Result<ValidExprParameters, Self::Error> {
                Ok(ValidExprParameters {
                    target_dt: match optional.target_dt {
                        Some(object) => {
                            match object
                                .try_into()
                                .map_err(|e| {
                                    ::alloc::__export::must_use({
                                            ::alloc::fmt::format(format_args!("{0}", e))
                                        })
                                        .to_string()
                                })
                            {
                                Ok(valid_object) => valid_object,
                                Err(e) => return Err(e),
                            }
                        }
                        None => return Err("target_dt is missing".to_string()),
                    },
                    waypoints: optional.waypoints,
                    constraints: optional.constraints,
                })
            }
        }
        impl From<ValidExprParameters> for ExprParameters {
            fn from(valid: ValidExprParameters) -> ExprParameters {
                ExprParameters {
                    target_dt: Some(valid.target_dt.into()),
                    waypoints: valid.waypoints,
                    constraints: valid.constraints,
                }
            }
        }
        impl crate::validate::Valid for ValidExprParameters {
            type Optional = ExprParameters;
            fn optionize(self) -> ExprParameters {
                self.into()
            }
        }
        impl crate::validate::Validate for ExprParameters {
            type Valid = ValidExprParameters;
            fn validate(self) -> Result<ValidExprParameters, String> {
                self.try_into()
            }
        }
        pub mod constraint {
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
            impl crate::to_double::ToDouble for DoubleConstraint {
                type DoubleType = DoubleConstraint;
            }
            pub struct ValidDoubleConstraint {
                pub enabled: bool,
                pub from: super::ValidWaypointId,
                pub to: super::ValidWaypointId,
                /// ExprMaxVelocity maxvelocity = 4;
                /// DoubleMaxAcceleration max_acceleration = 5;
                pub data: double_constraint::ValidData,
            }
            impl crate::to_double::ToDouble for ValidDoubleConstraint {
                type DoubleType = ValidDoubleConstraint;
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ValidDoubleConstraint {
                #[inline]
                fn clone(&self) -> ValidDoubleConstraint {
                    let _: ::core::clone::AssertParamIsClone<bool>;
                    let _: ::core::clone::AssertParamIsClone<super::ValidWaypointId>;
                    let _: ::core::clone::AssertParamIsClone<super::ValidWaypointId>;
                    let _: ::core::clone::AssertParamIsClone<
                        double_constraint::ValidData,
                    >;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ValidDoubleConstraint {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ValidDoubleConstraint {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ValidDoubleConstraint {
                #[inline]
                fn eq(&self, other: &ValidDoubleConstraint) -> bool {
                    self.enabled == other.enabled && self.from == other.from
                        && self.to == other.to && self.data == other.data
                }
            }
            impl TryFrom<DoubleConstraint> for ValidDoubleConstraint {
                type Error = String;
                fn try_from(
                    optional: DoubleConstraint,
                ) -> Result<ValidDoubleConstraint, Self::Error> {
                    Ok(ValidDoubleConstraint {
                        enabled: optional.enabled,
                        from: match optional.from {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("from is missing".to_string()),
                        },
                        to: match optional.to {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("to is missing".to_string()),
                        },
                        data: match optional.data {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("data is missing".to_string()),
                        },
                    })
                }
            }
            impl From<ValidDoubleConstraint> for DoubleConstraint {
                fn from(valid: ValidDoubleConstraint) -> DoubleConstraint {
                    DoubleConstraint {
                        enabled: valid.enabled,
                        from: Some(valid.from.into()),
                        to: Some(valid.to.into()),
                        data: Some(valid.data.into()),
                    }
                }
            }
            impl crate::validate::Valid for ValidDoubleConstraint {
                type Optional = DoubleConstraint;
                fn optionize(self) -> DoubleConstraint {
                    self.into()
                }
            }
            impl crate::validate::Validate for DoubleConstraint {
                type Valid = ValidDoubleConstraint;
                fn validate(self) -> Result<ValidDoubleConstraint, String> {
                    self.try_into()
                }
            }
            /// Nested message and enum types in `DoubleConstraint`.
            pub mod double_constraint {
                /// ExprMaxVelocity maxvelocity = 4;
                /// DoubleMaxAcceleration max_acceleration = 5;
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
                impl crate::to_double::ToDouble for Data {
                    type DoubleType = Data;
                }
                pub type ValidData = Data;
            }
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
            impl crate::to_double::ToDouble for ExprConstraint {
                type DoubleType = DoubleConstraint;
            }
            impl Into<DoubleConstraint> for ExprConstraint {
                fn into(self) -> DoubleConstraint {
                    DoubleConstraint {
                        enabled: self.enabled.into(),
                        from: self.from.map(Into::into).into(),
                        to: self.to.map(Into::into).into(),
                        data: self.data.map(Into::into).into(),
                    }
                }
            }
            pub struct ValidExprConstraint {
                pub enabled: bool,
                pub from: super::ValidWaypointId,
                pub to: super::ValidWaypointId,
                /// ExprMaxVelocity maxvelocity = 4;
                /// ExprMaxAcceleration max_acceleration = 5;
                pub data: expr_constraint::ValidData,
            }
            impl crate::to_double::ToDouble for ValidExprConstraint {
                type DoubleType = ValidDoubleConstraint;
            }
            impl Into<ValidDoubleConstraint> for ValidExprConstraint {
                fn into(self) -> ValidDoubleConstraint {
                    ValidDoubleConstraint {
                        enabled: self.enabled.into(),
                        from: self.from.into(),
                        to: self.to.into(),
                        data: self.data.into(),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ValidExprConstraint {
                #[inline]
                fn clone(&self) -> ValidExprConstraint {
                    ValidExprConstraint {
                        enabled: ::core::clone::Clone::clone(&self.enabled),
                        from: ::core::clone::Clone::clone(&self.from),
                        to: ::core::clone::Clone::clone(&self.to),
                        data: ::core::clone::Clone::clone(&self.data),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ValidExprConstraint {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ValidExprConstraint {
                #[inline]
                fn eq(&self, other: &ValidExprConstraint) -> bool {
                    self.enabled == other.enabled && self.from == other.from
                        && self.to == other.to && self.data == other.data
                }
            }
            impl TryFrom<ExprConstraint> for ValidExprConstraint {
                type Error = String;
                fn try_from(
                    optional: ExprConstraint,
                ) -> Result<ValidExprConstraint, Self::Error> {
                    Ok(ValidExprConstraint {
                        enabled: optional.enabled,
                        from: match optional.from {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("from is missing".to_string()),
                        },
                        to: match optional.to {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("to is missing".to_string()),
                        },
                        data: match optional.data {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("data is missing".to_string()),
                        },
                    })
                }
            }
            impl From<ValidExprConstraint> for ExprConstraint {
                fn from(valid: ValidExprConstraint) -> ExprConstraint {
                    ExprConstraint {
                        enabled: valid.enabled,
                        from: Some(valid.from.into()),
                        to: Some(valid.to.into()),
                        data: Some(valid.data.into()),
                    }
                }
            }
            impl crate::validate::Valid for ValidExprConstraint {
                type Optional = ExprConstraint;
                fn optionize(self) -> ExprConstraint {
                    self.into()
                }
            }
            impl crate::validate::Validate for ExprConstraint {
                type Valid = ValidExprConstraint;
                fn validate(self) -> Result<ValidExprConstraint, String> {
                    self.try_into()
                }
            }
            /// Nested message and enum types in `ExprConstraint`.
            pub mod expr_constraint {
                /// ExprMaxVelocity maxvelocity = 4;
                /// ExprMaxAcceleration max_acceleration = 5;
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
                impl crate::to_double::ToDouble for Data {
                    type DoubleType = Data;
                }
                pub type ValidData = Data;
            }
            pub mod max_acceleration {
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
                impl crate::to_double::ToDouble for DoubleMaxAcceleration {
                    type DoubleType = DoubleMaxAcceleration;
                }
                pub type ValidDoubleMaxAcceleration = DoubleMaxAcceleration;
                pub struct ExprMaxAcceleration {
                    #[prost(message, optional, tag = "1")]
                    pub max: ::core::option::Option<super::super::super::Expr>,
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
                impl crate::to_double::ToDouble for ExprMaxAcceleration {
                    type DoubleType = DoubleMaxAcceleration;
                }
                impl Into<DoubleMaxAcceleration> for ExprMaxAcceleration {
                    fn into(self) -> DoubleMaxAcceleration {
                        DoubleMaxAcceleration {
                            max: self.max.map(Into::into).into(),
                        }
                    }
                }
                pub struct ValidExprMaxAcceleration {
                    pub max: super::super::super::ValidExpr,
                }
                impl crate::to_double::ToDouble for ValidExprMaxAcceleration {
                    type DoubleType = ValidDoubleMaxAcceleration;
                }
                impl Into<ValidDoubleMaxAcceleration> for ValidExprMaxAcceleration {
                    fn into(self) -> ValidDoubleMaxAcceleration {
                        ValidDoubleMaxAcceleration {
                            max: self.max.into(),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ValidExprMaxAcceleration {
                    #[inline]
                    fn clone(&self) -> ValidExprMaxAcceleration {
                        ValidExprMaxAcceleration {
                            max: ::core::clone::Clone::clone(&self.max),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for ValidExprMaxAcceleration {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for ValidExprMaxAcceleration {
                    #[inline]
                    fn eq(&self, other: &ValidExprMaxAcceleration) -> bool {
                        self.max == other.max
                    }
                }
                impl TryFrom<ExprMaxAcceleration> for ValidExprMaxAcceleration {
                    type Error = String;
                    fn try_from(
                        optional: ExprMaxAcceleration,
                    ) -> Result<ValidExprMaxAcceleration, Self::Error> {
                        Ok(ValidExprMaxAcceleration {
                            max: match optional.max {
                                Some(object) => {
                                    match object
                                        .try_into()
                                        .map_err(|e| {
                                            ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(format_args!("{0}", e))
                                                })
                                                .to_string()
                                        })
                                    {
                                        Ok(valid_object) => valid_object,
                                        Err(e) => return Err(e),
                                    }
                                }
                                None => return Err("max is missing".to_string()),
                            },
                        })
                    }
                }
                impl From<ValidExprMaxAcceleration> for ExprMaxAcceleration {
                    fn from(valid: ValidExprMaxAcceleration) -> ExprMaxAcceleration {
                        ExprMaxAcceleration {
                            max: Some(valid.max.into()),
                        }
                    }
                }
                impl crate::validate::Valid for ValidExprMaxAcceleration {
                    type Optional = ExprMaxAcceleration;
                    fn optionize(self) -> ExprMaxAcceleration {
                        self.into()
                    }
                }
                impl crate::validate::Validate for ExprMaxAcceleration {
                    type Valid = ValidExprMaxAcceleration;
                    fn validate(self) -> Result<ValidExprMaxAcceleration, String> {
                        self.try_into()
                    }
                }
            }
            pub mod maxvelocity {
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
                impl crate::to_double::ToDouble for DoubleMaxVelocity {
                    type DoubleType = DoubleMaxVelocity;
                }
                pub type ValidDoubleMaxVelocity = DoubleMaxVelocity;
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
                impl crate::to_double::ToDouble for TestDouble {
                    type DoubleType = TestDouble;
                }
                pub type ValidTestDouble = TestDouble;
                pub struct ExprMaxVelocity {
                    #[prost(message, optional, tag = "1")]
                    pub max: ::core::option::Option<super::super::super::Expr>,
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
                impl crate::to_double::ToDouble for ExprMaxVelocity {
                    type DoubleType = DoubleMaxVelocity;
                }
                impl Into<DoubleMaxVelocity> for ExprMaxVelocity {
                    fn into(self) -> DoubleMaxVelocity {
                        DoubleMaxVelocity {
                            max: self.max.map(Into::into).into(),
                        }
                    }
                }
                pub struct ValidExprMaxVelocity {
                    pub max: super::super::super::ValidExpr,
                }
                impl crate::to_double::ToDouble for ValidExprMaxVelocity {
                    type DoubleType = ValidDoubleMaxVelocity;
                }
                impl Into<ValidDoubleMaxVelocity> for ValidExprMaxVelocity {
                    fn into(self) -> ValidDoubleMaxVelocity {
                        ValidDoubleMaxVelocity {
                            max: self.max.into(),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ValidExprMaxVelocity {
                    #[inline]
                    fn clone(&self) -> ValidExprMaxVelocity {
                        ValidExprMaxVelocity {
                            max: ::core::clone::Clone::clone(&self.max),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for ValidExprMaxVelocity {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for ValidExprMaxVelocity {
                    #[inline]
                    fn eq(&self, other: &ValidExprMaxVelocity) -> bool {
                        self.max == other.max
                    }
                }
                impl TryFrom<ExprMaxVelocity> for ValidExprMaxVelocity {
                    type Error = String;
                    fn try_from(
                        optional: ExprMaxVelocity,
                    ) -> Result<ValidExprMaxVelocity, Self::Error> {
                        Ok(ValidExprMaxVelocity {
                            max: match optional.max {
                                Some(object) => {
                                    match object
                                        .try_into()
                                        .map_err(|e| {
                                            ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(format_args!("{0}", e))
                                                })
                                                .to_string()
                                        })
                                    {
                                        Ok(valid_object) => valid_object,
                                        Err(e) => return Err(e),
                                    }
                                }
                                None => return Err("max is missing".to_string()),
                            },
                        })
                    }
                }
                impl From<ValidExprMaxVelocity> for ExprMaxVelocity {
                    fn from(valid: ValidExprMaxVelocity) -> ExprMaxVelocity {
                        ExprMaxVelocity {
                            max: Some(valid.max.into()),
                        }
                    }
                }
                impl crate::validate::Valid for ValidExprMaxVelocity {
                    type Optional = ExprMaxVelocity;
                    fn optionize(self) -> ExprMaxVelocity {
                        self.into()
                    }
                }
                impl crate::validate::Validate for ExprMaxVelocity {
                    type Valid = ValidExprMaxVelocity;
                    fn validate(self) -> Result<ValidExprMaxVelocity, String> {
                        self.try_into()
                    }
                }
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
                impl crate::to_double::ToDouble for TestExpr {
                    type DoubleType = TestDouble;
                }
                impl Into<TestDouble> for TestExpr {
                    fn into(self) -> TestDouble {
                        TestDouble {
                            test: self.test.into(),
                        }
                    }
                }
                pub type ValidTestExpr = TestExpr;
            }
        }
        pub mod robotconfig {
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
            impl crate::to_double::ToDouble for DoubleModule {
                type DoubleType = DoubleModule;
            }
            pub type ValidDoubleModule = DoubleModule;
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
            impl crate::to_double::ToDouble for DoubleBumper {
                type DoubleType = DoubleBumper;
            }
            pub type ValidDoubleBumper = DoubleBumper;
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
            impl crate::to_double::ToDouble for DoubleRobotConfig {
                type DoubleType = DoubleRobotConfig;
            }
            pub struct ValidDoubleRobotConfig {
                pub mass: f64,
                pub inertia: f64,
                pub gearing: f64,
                pub radius: f64,
                pub vmax: f64,
                pub tmax: f64,
                pub cof: f64,
                pub differential_track_width: f64,
                /// double bumper_front = 9;
                /// double bumper_left = 10;
                /// double bumper_back = 11;
                /// double bumper_right = 12;
                /// double fl_x = 13;
                /// double fl_y = 14;
                /// double fr_x = 15;
                /// double fl_
                pub bumper: ValidDoubleBumper,
                pub front_left: ValidDoubleModule,
                pub front_right: ValidDoubleModule,
                pub back_left: ValidDoubleModule,
                pub back_right: ValidDoubleModule,
            }
            impl crate::to_double::ToDouble for ValidDoubleRobotConfig {
                type DoubleType = ValidDoubleRobotConfig;
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ValidDoubleRobotConfig {
                #[inline]
                fn clone(&self) -> ValidDoubleRobotConfig {
                    let _: ::core::clone::AssertParamIsClone<f64>;
                    let _: ::core::clone::AssertParamIsClone<ValidDoubleBumper>;
                    let _: ::core::clone::AssertParamIsClone<ValidDoubleModule>;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ValidDoubleRobotConfig {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ValidDoubleRobotConfig {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ValidDoubleRobotConfig {
                #[inline]
                fn eq(&self, other: &ValidDoubleRobotConfig) -> bool {
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
            impl TryFrom<DoubleRobotConfig> for ValidDoubleRobotConfig {
                type Error = String;
                fn try_from(
                    optional: DoubleRobotConfig,
                ) -> Result<ValidDoubleRobotConfig, Self::Error> {
                    Ok(ValidDoubleRobotConfig {
                        mass: optional.mass,
                        inertia: optional.inertia,
                        gearing: optional.gearing,
                        radius: optional.radius,
                        vmax: optional.vmax,
                        tmax: optional.tmax,
                        cof: optional.cof,
                        differential_track_width: optional.differential_track_width,
                        bumper: match optional.bumper {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("bumper is missing".to_string()),
                        },
                        front_left: match optional.front_left {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("front_left is missing".to_string()),
                        },
                        front_right: match optional.front_right {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("front_right is missing".to_string()),
                        },
                        back_left: match optional.back_left {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("back_left is missing".to_string()),
                        },
                        back_right: match optional.back_right {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("back_right is missing".to_string()),
                        },
                    })
                }
            }
            impl From<ValidDoubleRobotConfig> for DoubleRobotConfig {
                fn from(valid: ValidDoubleRobotConfig) -> DoubleRobotConfig {
                    DoubleRobotConfig {
                        mass: valid.mass,
                        inertia: valid.inertia,
                        gearing: valid.gearing,
                        radius: valid.radius,
                        vmax: valid.vmax,
                        tmax: valid.tmax,
                        cof: valid.cof,
                        differential_track_width: valid.differential_track_width,
                        bumper: Some(valid.bumper.into()),
                        front_left: Some(valid.front_left.into()),
                        front_right: Some(valid.front_right.into()),
                        back_left: Some(valid.back_left.into()),
                        back_right: Some(valid.back_right.into()),
                    }
                }
            }
            impl crate::validate::Valid for ValidDoubleRobotConfig {
                type Optional = DoubleRobotConfig;
                fn optionize(self) -> DoubleRobotConfig {
                    self.into()
                }
            }
            impl crate::validate::Validate for DoubleRobotConfig {
                type Valid = ValidDoubleRobotConfig;
                fn validate(self) -> Result<ValidDoubleRobotConfig, String> {
                    self.try_into()
                }
            }
            pub struct ExprModule {
                #[prost(message, optional, tag = "1")]
                pub x: ::core::option::Option<super::super::Expr>,
                #[prost(message, optional, tag = "2")]
                pub y: ::core::option::Option<super::super::Expr>,
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
            impl crate::to_double::ToDouble for ExprModule {
                type DoubleType = DoubleModule;
            }
            impl Into<DoubleModule> for ExprModule {
                fn into(self) -> DoubleModule {
                    DoubleModule {
                        x: self.x.map(Into::into).into(),
                        y: self.y.map(Into::into).into(),
                    }
                }
            }
            pub struct ValidExprModule {
                pub x: super::super::ValidExpr,
                pub y: super::super::ValidExpr,
            }
            impl crate::to_double::ToDouble for ValidExprModule {
                type DoubleType = ValidDoubleModule;
            }
            impl Into<ValidDoubleModule> for ValidExprModule {
                fn into(self) -> ValidDoubleModule {
                    ValidDoubleModule {
                        x: self.x.into(),
                        y: self.y.into(),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ValidExprModule {
                #[inline]
                fn clone(&self) -> ValidExprModule {
                    ValidExprModule {
                        x: ::core::clone::Clone::clone(&self.x),
                        y: ::core::clone::Clone::clone(&self.y),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ValidExprModule {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ValidExprModule {
                #[inline]
                fn eq(&self, other: &ValidExprModule) -> bool {
                    self.x == other.x && self.y == other.y
                }
            }
            impl TryFrom<ExprModule> for ValidExprModule {
                type Error = String;
                fn try_from(
                    optional: ExprModule,
                ) -> Result<ValidExprModule, Self::Error> {
                    Ok(ValidExprModule {
                        x: match optional.x {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("x is missing".to_string()),
                        },
                        y: match optional.y {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("y is missing".to_string()),
                        },
                    })
                }
            }
            impl From<ValidExprModule> for ExprModule {
                fn from(valid: ValidExprModule) -> ExprModule {
                    ExprModule {
                        x: Some(valid.x.into()),
                        y: Some(valid.y.into()),
                    }
                }
            }
            impl crate::validate::Valid for ValidExprModule {
                type Optional = ExprModule;
                fn optionize(self) -> ExprModule {
                    self.into()
                }
            }
            impl crate::validate::Validate for ExprModule {
                type Valid = ValidExprModule;
                fn validate(self) -> Result<ValidExprModule, String> {
                    self.try_into()
                }
            }
            pub struct ExprBumper {
                #[prost(message, optional, tag = "1")]
                pub front: ::core::option::Option<super::super::Expr>,
                #[prost(message, optional, tag = "2")]
                pub left: ::core::option::Option<super::super::Expr>,
                #[prost(message, optional, tag = "3")]
                pub right: ::core::option::Option<super::super::Expr>,
                #[prost(message, optional, tag = "4")]
                pub back: ::core::option::Option<super::super::Expr>,
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
            impl crate::to_double::ToDouble for ExprBumper {
                type DoubleType = DoubleBumper;
            }
            impl Into<DoubleBumper> for ExprBumper {
                fn into(self) -> DoubleBumper {
                    DoubleBumper {
                        front: self.front.map(Into::into).into(),
                        left: self.left.map(Into::into).into(),
                        right: self.right.map(Into::into).into(),
                        back: self.back.map(Into::into).into(),
                    }
                }
            }
            pub struct ValidExprBumper {
                pub front: super::super::ValidExpr,
                pub left: super::super::ValidExpr,
                pub right: super::super::ValidExpr,
                pub back: super::super::ValidExpr,
            }
            impl crate::to_double::ToDouble for ValidExprBumper {
                type DoubleType = ValidDoubleBumper;
            }
            impl Into<ValidDoubleBumper> for ValidExprBumper {
                fn into(self) -> ValidDoubleBumper {
                    ValidDoubleBumper {
                        front: self.front.into(),
                        left: self.left.into(),
                        right: self.right.into(),
                        back: self.back.into(),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ValidExprBumper {
                #[inline]
                fn clone(&self) -> ValidExprBumper {
                    ValidExprBumper {
                        front: ::core::clone::Clone::clone(&self.front),
                        left: ::core::clone::Clone::clone(&self.left),
                        right: ::core::clone::Clone::clone(&self.right),
                        back: ::core::clone::Clone::clone(&self.back),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ValidExprBumper {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ValidExprBumper {
                #[inline]
                fn eq(&self, other: &ValidExprBumper) -> bool {
                    self.front == other.front && self.left == other.left
                        && self.right == other.right && self.back == other.back
                }
            }
            impl TryFrom<ExprBumper> for ValidExprBumper {
                type Error = String;
                fn try_from(
                    optional: ExprBumper,
                ) -> Result<ValidExprBumper, Self::Error> {
                    Ok(ValidExprBumper {
                        front: match optional.front {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("front is missing".to_string()),
                        },
                        left: match optional.left {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("left is missing".to_string()),
                        },
                        right: match optional.right {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("right is missing".to_string()),
                        },
                        back: match optional.back {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("back is missing".to_string()),
                        },
                    })
                }
            }
            impl From<ValidExprBumper> for ExprBumper {
                fn from(valid: ValidExprBumper) -> ExprBumper {
                    ExprBumper {
                        front: Some(valid.front.into()),
                        left: Some(valid.left.into()),
                        right: Some(valid.right.into()),
                        back: Some(valid.back.into()),
                    }
                }
            }
            impl crate::validate::Valid for ValidExprBumper {
                type Optional = ExprBumper;
                fn optionize(self) -> ExprBumper {
                    self.into()
                }
            }
            impl crate::validate::Validate for ExprBumper {
                type Valid = ValidExprBumper;
                fn validate(self) -> Result<ValidExprBumper, String> {
                    self.try_into()
                }
            }
            pub struct ExprRobotConfig {
                #[prost(message, optional, tag = "1")]
                pub mass: ::core::option::Option<super::super::Expr>,
                #[prost(message, optional, tag = "2")]
                pub inertia: ::core::option::Option<super::super::Expr>,
                #[prost(message, optional, tag = "3")]
                pub gearing: ::core::option::Option<super::super::Expr>,
                #[prost(message, optional, tag = "4")]
                pub radius: ::core::option::Option<super::super::Expr>,
                #[prost(message, optional, tag = "5")]
                pub vmax: ::core::option::Option<super::super::Expr>,
                #[prost(message, optional, tag = "6")]
                pub tmax: ::core::option::Option<super::super::Expr>,
                #[prost(message, optional, tag = "7")]
                pub cof: ::core::option::Option<super::super::Expr>,
                #[prost(message, optional, tag = "8")]
                pub differential_track_width: ::core::option::Option<super::super::Expr>,
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
            impl crate::to_double::ToDouble for ExprRobotConfig {
                type DoubleType = DoubleRobotConfig;
            }
            impl Into<DoubleRobotConfig> for ExprRobotConfig {
                fn into(self) -> DoubleRobotConfig {
                    DoubleRobotConfig {
                        mass: self.mass.map(Into::into).into(),
                        inertia: self.inertia.map(Into::into).into(),
                        gearing: self.gearing.map(Into::into).into(),
                        radius: self.radius.map(Into::into).into(),
                        vmax: self.vmax.map(Into::into).into(),
                        tmax: self.tmax.map(Into::into).into(),
                        cof: self.cof.map(Into::into).into(),
                        differential_track_width: self
                            .differential_track_width
                            .map(Into::into)
                            .into(),
                        bumper: self.bumper.map(Into::into).into(),
                        front_left: self.front_left.map(Into::into).into(),
                        front_right: self.front_right.map(Into::into).into(),
                        back_left: self.back_left.map(Into::into).into(),
                        back_right: self.back_right.map(Into::into).into(),
                    }
                }
            }
            pub struct ValidExprRobotConfig {
                pub mass: super::super::ValidExpr,
                pub inertia: super::super::ValidExpr,
                pub gearing: super::super::ValidExpr,
                pub radius: super::super::ValidExpr,
                pub vmax: super::super::ValidExpr,
                pub tmax: super::super::ValidExpr,
                pub cof: super::super::ValidExpr,
                pub differential_track_width: super::super::ValidExpr,
                /// Expr bumper_front = 9;
                /// Expr bumper_left = 10;
                /// Expr bumper_back = 11;
                /// Expr bumper_right = 12;
                /// Expr fl_x = 13;
                /// Expr fl_y = 14;
                /// Expr fr_x = 15;
                /// Expr fl_
                pub bumper: ValidExprBumper,
                pub front_left: ValidExprModule,
                pub front_right: ValidExprModule,
                pub back_left: ValidExprModule,
                pub back_right: ValidExprModule,
            }
            impl crate::to_double::ToDouble for ValidExprRobotConfig {
                type DoubleType = ValidDoubleRobotConfig;
            }
            impl Into<ValidDoubleRobotConfig> for ValidExprRobotConfig {
                fn into(self) -> ValidDoubleRobotConfig {
                    ValidDoubleRobotConfig {
                        mass: self.mass.into(),
                        inertia: self.inertia.into(),
                        gearing: self.gearing.into(),
                        radius: self.radius.into(),
                        vmax: self.vmax.into(),
                        tmax: self.tmax.into(),
                        cof: self.cof.into(),
                        differential_track_width: self.differential_track_width.into(),
                        bumper: self.bumper.into(),
                        front_left: self.front_left.into(),
                        front_right: self.front_right.into(),
                        back_left: self.back_left.into(),
                        back_right: self.back_right.into(),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ValidExprRobotConfig {
                #[inline]
                fn clone(&self) -> ValidExprRobotConfig {
                    ValidExprRobotConfig {
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
            impl ::core::marker::StructuralPartialEq for ValidExprRobotConfig {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ValidExprRobotConfig {
                #[inline]
                fn eq(&self, other: &ValidExprRobotConfig) -> bool {
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
            impl TryFrom<ExprRobotConfig> for ValidExprRobotConfig {
                type Error = String;
                fn try_from(
                    optional: ExprRobotConfig,
                ) -> Result<ValidExprRobotConfig, Self::Error> {
                    Ok(ValidExprRobotConfig {
                        mass: match optional.mass {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("mass is missing".to_string()),
                        },
                        inertia: match optional.inertia {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("inertia is missing".to_string()),
                        },
                        gearing: match optional.gearing {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("gearing is missing".to_string()),
                        },
                        radius: match optional.radius {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("radius is missing".to_string()),
                        },
                        vmax: match optional.vmax {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("vmax is missing".to_string()),
                        },
                        tmax: match optional.tmax {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("tmax is missing".to_string()),
                        },
                        cof: match optional.cof {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("cof is missing".to_string()),
                        },
                        differential_track_width: match optional.differential_track_width
                        {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => {
                                return Err(
                                    "differential_track_width is missing".to_string(),
                                );
                            }
                        },
                        bumper: match optional.bumper {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("bumper is missing".to_string()),
                        },
                        front_left: match optional.front_left {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("front_left is missing".to_string()),
                        },
                        front_right: match optional.front_right {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("front_right is missing".to_string()),
                        },
                        back_left: match optional.back_left {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("back_left is missing".to_string()),
                        },
                        back_right: match optional.back_right {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("back_right is missing".to_string()),
                        },
                    })
                }
            }
            impl From<ValidExprRobotConfig> for ExprRobotConfig {
                fn from(valid: ValidExprRobotConfig) -> ExprRobotConfig {
                    ExprRobotConfig {
                        mass: Some(valid.mass.into()),
                        inertia: Some(valid.inertia.into()),
                        gearing: Some(valid.gearing.into()),
                        radius: Some(valid.radius.into()),
                        vmax: Some(valid.vmax.into()),
                        tmax: Some(valid.tmax.into()),
                        cof: Some(valid.cof.into()),
                        differential_track_width: Some(
                            valid.differential_track_width.into(),
                        ),
                        bumper: Some(valid.bumper.into()),
                        front_left: Some(valid.front_left.into()),
                        front_right: Some(valid.front_right.into()),
                        back_left: Some(valid.back_left.into()),
                        back_right: Some(valid.back_right.into()),
                    }
                }
            }
            impl crate::validate::Valid for ValidExprRobotConfig {
                type Optional = ExprRobotConfig;
                fn optionize(self) -> ExprRobotConfig {
                    self.into()
                }
            }
            impl crate::validate::Validate for ExprRobotConfig {
                type Valid = ValidExprRobotConfig;
                fn validate(self) -> Result<ValidExprRobotConfig, String> {
                    self.try_into()
                }
            }
        }
        pub mod waypoint {
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
            impl crate::to_double::ToDouble for DoubleWaypoint {
                type DoubleType = DoubleWaypoint;
            }
            pub type ValidDoubleWaypoint = DoubleWaypoint;
            pub struct ExprWaypoint {
                #[prost(message, optional, tag = "1")]
                pub x: ::core::option::Option<super::super::Expr>,
                #[prost(message, optional, tag = "2")]
                pub y: ::core::option::Option<super::super::Expr>,
                #[prost(message, optional, tag = "3")]
                pub heading: ::core::option::Option<super::super::Expr>,
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
            impl crate::to_double::ToDouble for ExprWaypoint {
                type DoubleType = DoubleWaypoint;
            }
            impl Into<DoubleWaypoint> for ExprWaypoint {
                fn into(self) -> DoubleWaypoint {
                    DoubleWaypoint {
                        x: self.x.map(Into::into).into(),
                        y: self.y.map(Into::into).into(),
                        heading: self.heading.map(Into::into).into(),
                        intervals: self.intervals.into(),
                        split: self.split.into(),
                        fix_translation: self.fix_translation.into(),
                        fix_heading: self.fix_heading.into(),
                        override_intervals: self.override_intervals.into(),
                    }
                }
            }
            pub struct ValidExprWaypoint {
                pub x: super::super::ValidExpr,
                pub y: super::super::ValidExpr,
                pub heading: super::super::ValidExpr,
                pub intervals: u64,
                pub split: bool,
                pub fix_translation: bool,
                pub fix_heading: bool,
                pub override_intervals: bool,
            }
            impl crate::to_double::ToDouble for ValidExprWaypoint {
                type DoubleType = ValidDoubleWaypoint;
            }
            impl Into<ValidDoubleWaypoint> for ValidExprWaypoint {
                fn into(self) -> ValidDoubleWaypoint {
                    ValidDoubleWaypoint {
                        x: self.x.into(),
                        y: self.y.into(),
                        heading: self.heading.into(),
                        intervals: self.intervals.into(),
                        split: self.split.into(),
                        fix_translation: self.fix_translation.into(),
                        fix_heading: self.fix_heading.into(),
                        override_intervals: self.override_intervals.into(),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ValidExprWaypoint {
                #[inline]
                fn clone(&self) -> ValidExprWaypoint {
                    ValidExprWaypoint {
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
            impl ::core::marker::StructuralPartialEq for ValidExprWaypoint {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ValidExprWaypoint {
                #[inline]
                fn eq(&self, other: &ValidExprWaypoint) -> bool {
                    self.intervals == other.intervals && self.split == other.split
                        && self.fix_translation == other.fix_translation
                        && self.fix_heading == other.fix_heading
                        && self.override_intervals == other.override_intervals
                        && self.x == other.x && self.y == other.y
                        && self.heading == other.heading
                }
            }
            impl TryFrom<ExprWaypoint> for ValidExprWaypoint {
                type Error = String;
                fn try_from(
                    optional: ExprWaypoint,
                ) -> Result<ValidExprWaypoint, Self::Error> {
                    Ok(ValidExprWaypoint {
                        x: match optional.x {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("x is missing".to_string()),
                        },
                        y: match optional.y {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("y is missing".to_string()),
                        },
                        heading: match optional.heading {
                            Some(object) => {
                                match object
                                    .try_into()
                                    .map_err(|e| {
                                        ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0}", e))
                                            })
                                            .to_string()
                                    })
                                {
                                    Ok(valid_object) => valid_object,
                                    Err(e) => return Err(e),
                                }
                            }
                            None => return Err("heading is missing".to_string()),
                        },
                        intervals: optional.intervals,
                        split: optional.split,
                        fix_translation: optional.fix_translation,
                        fix_heading: optional.fix_heading,
                        override_intervals: optional.override_intervals,
                    })
                }
            }
            impl From<ValidExprWaypoint> for ExprWaypoint {
                fn from(valid: ValidExprWaypoint) -> ExprWaypoint {
                    ExprWaypoint {
                        x: Some(valid.x.into()),
                        y: Some(valid.y.into()),
                        heading: Some(valid.heading.into()),
                        intervals: valid.intervals,
                        split: valid.split,
                        fix_translation: valid.fix_translation,
                        fix_heading: valid.fix_heading,
                        override_intervals: valid.override_intervals,
                    }
                }
            }
            impl crate::validate::Valid for ValidExprWaypoint {
                type Optional = ExprWaypoint;
                fn optionize(self) -> ExprWaypoint {
                    self.into()
                }
            }
            impl crate::validate::Validate for ExprWaypoint {
                type Valid = ValidExprWaypoint;
                fn validate(self) -> Result<ValidExprWaypoint, String> {
                    self.try_into()
                }
            }
        }
    }
}
pub mod service {
    /// Generated client implementations.
    pub mod choreo_service_client {
        #![allow(
            unused_variables,
            dead_code,
            missing_docs,
            clippy::wildcard_imports,
            clippy::let_unit_value,
        )]
        use tonic::codegen::*;
        use tonic::codegen::http::Uri;
        ///
        pub struct ChoreoServiceClient<T> {
            inner: tonic::client::Grpc<T>,
        }
        #[automatically_derived]
        impl<T: ::core::fmt::Debug> ::core::fmt::Debug for ChoreoServiceClient<T> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ChoreoServiceClient",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl<T: ::core::clone::Clone> ::core::clone::Clone for ChoreoServiceClient<T> {
            #[inline]
            fn clone(&self) -> ChoreoServiceClient<T> {
                ChoreoServiceClient {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        impl ChoreoServiceClient<tonic::transport::Channel> {
            /// Attempt to create a new client by connecting to a given endpoint.
            pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
            where
                D: TryInto<tonic::transport::Endpoint>,
                D::Error: Into<StdError>,
            {
                let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
                Ok(Self::new(conn))
            }
        }
        impl<T> ChoreoServiceClient<T>
        where
            T: tonic::client::GrpcService<tonic::body::Body>,
            T::Error: Into<StdError>,
            T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
            <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
        {
            pub fn new(inner: T) -> Self {
                let inner = tonic::client::Grpc::new(inner);
                Self { inner }
            }
            pub fn with_origin(inner: T, origin: Uri) -> Self {
                let inner = tonic::client::Grpc::with_origin(inner, origin);
                Self { inner }
            }
            pub fn with_interceptor<F>(
                inner: T,
                interceptor: F,
            ) -> ChoreoServiceClient<InterceptedService<T, F>>
            where
                F: tonic::service::Interceptor,
                T::ResponseBody: Default,
                T: tonic::codegen::Service<
                    http::Request<tonic::body::Body>,
                    Response = http::Response<
                        <T as tonic::client::GrpcService<
                            tonic::body::Body,
                        >>::ResponseBody,
                    >,
                >,
                <T as tonic::codegen::Service<
                    http::Request<tonic::body::Body>,
                >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
            {
                ChoreoServiceClient::new(InterceptedService::new(inner, interceptor))
            }
            /// Compress requests with the given encoding.
            ///
            /// This requires the server to support it otherwise it might respond with an
            /// error.
            #[must_use]
            pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.inner = self.inner.send_compressed(encoding);
                self
            }
            /// Enable decompressing responses.
            #[must_use]
            pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.inner = self.inner.accept_compressed(encoding);
                self
            }
            /// Limits the maximum size of a decoded message.
            ///
            /// Default: `4MB`
            #[must_use]
            pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
                self.inner = self.inner.max_decoding_message_size(limit);
                self
            }
            /// Limits the maximum size of an encoded message.
            ///
            /// Default: `usize::MAX`
            #[must_use]
            pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
                self.inner = self.inner.max_encoding_message_size(limit);
                self
            }
            ///
            pub async fn echo_swerve_sample(
                &mut self,
                request: impl tonic::IntoRequest<
                    super::commands::EchoSwerveSampleRequest,
                >,
            ) -> std::result::Result<
                tonic::Response<super::commands::EchoSwerveSampleResponse>,
                tonic::Status,
            > {
                self.inner
                    .ready()
                    .await
                    .map_err(|e| {
                        tonic::Status::unknown(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("Service was not ready: {0}", e.into()),
                                )
                            }),
                        )
                    })?;
                let codec = tonic_prost::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static(
                    "/service.ChoreoService/EchoSwerveSample",
                );
                let mut req = request.into_request();
                req.extensions_mut()
                    .insert(
                        GrpcMethod::new("service.ChoreoService", "EchoSwerveSample"),
                    );
                self.inner.unary(req, path, codec).await
            }
            ///
            pub async fn generate(
                &mut self,
                request: impl tonic::IntoRequest<super::commands::GenerateRequest>,
            ) -> std::result::Result<
                tonic::Response<super::commands::GenerateResponse>,
                tonic::Status,
            > {
                self.inner
                    .ready()
                    .await
                    .map_err(|e| {
                        tonic::Status::unknown(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("Service was not ready: {0}", e.into()),
                                )
                            }),
                        )
                    })?;
                let codec = tonic_prost::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static(
                    "/service.ChoreoService/Generate",
                );
                let mut req = request.into_request();
                req.extensions_mut()
                    .insert(GrpcMethod::new("service.ChoreoService", "Generate"));
                self.inner.unary(req, path, codec).await
            }
            ///
            pub async fn get_default_trajectory(
                &mut self,
                request: impl tonic::IntoRequest<::pbjson_types::Empty>,
            ) -> std::result::Result<
                tonic::Response<super::commands::GetDefaultTrajectoryResponse>,
                tonic::Status,
            > {
                self.inner
                    .ready()
                    .await
                    .map_err(|e| {
                        tonic::Status::unknown(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("Service was not ready: {0}", e.into()),
                                )
                            }),
                        )
                    })?;
                let codec = tonic_prost::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static(
                    "/service.ChoreoService/GetDefaultTrajectory",
                );
                let mut req = request.into_request();
                req.extensions_mut()
                    .insert(
                        GrpcMethod::new("service.ChoreoService", "GetDefaultTrajectory"),
                    );
                self.inner.unary(req, path, codec).await
            }
        }
    }
    /// Generated server implementations.
    pub mod choreo_service_server {
        #![allow(
            unused_variables,
            dead_code,
            missing_docs,
            clippy::wildcard_imports,
            clippy::let_unit_value,
        )]
        use tonic::codegen::*;
        /// Generated trait containing gRPC methods that should be implemented for use with ChoreoServiceServer.
        pub trait ChoreoService: std::marker::Send + std::marker::Sync + 'static {
            ///
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn echo_swerve_sample<'life0, 'async_trait>(
                &'life0 self,
                request: tonic::Request<super::commands::EchoSwerveSampleRequest>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = std::result::Result<
                            tonic::Response<super::commands::EchoSwerveSampleResponse>,
                            tonic::Status,
                        >,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait;
            ///
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn generate<'life0, 'async_trait>(
                &'life0 self,
                request: tonic::Request<super::commands::GenerateRequest>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = std::result::Result<
                            tonic::Response<super::commands::GenerateResponse>,
                            tonic::Status,
                        >,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait;
            ///
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn get_default_trajectory<'life0, 'async_trait>(
                &'life0 self,
                request: tonic::Request<::pbjson_types::Empty>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = std::result::Result<
                            tonic::Response<
                                super::commands::GetDefaultTrajectoryResponse,
                            >,
                            tonic::Status,
                        >,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait;
        }
        ///
        pub struct ChoreoServiceServer<T> {
            inner: Arc<T>,
            accept_compression_encodings: EnabledCompressionEncodings,
            send_compression_encodings: EnabledCompressionEncodings,
            max_decoding_message_size: Option<usize>,
            max_encoding_message_size: Option<usize>,
        }
        #[automatically_derived]
        impl<T: ::core::fmt::Debug> ::core::fmt::Debug for ChoreoServiceServer<T> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "ChoreoServiceServer",
                    "inner",
                    &self.inner,
                    "accept_compression_encodings",
                    &self.accept_compression_encodings,
                    "send_compression_encodings",
                    &self.send_compression_encodings,
                    "max_decoding_message_size",
                    &self.max_decoding_message_size,
                    "max_encoding_message_size",
                    &&self.max_encoding_message_size,
                )
            }
        }
        impl<T> ChoreoServiceServer<T> {
            pub fn new(inner: T) -> Self {
                Self::from_arc(Arc::new(inner))
            }
            pub fn from_arc(inner: Arc<T>) -> Self {
                Self {
                    inner,
                    accept_compression_encodings: Default::default(),
                    send_compression_encodings: Default::default(),
                    max_decoding_message_size: None,
                    max_encoding_message_size: None,
                }
            }
            pub fn with_interceptor<F>(
                inner: T,
                interceptor: F,
            ) -> InterceptedService<Self, F>
            where
                F: tonic::service::Interceptor,
            {
                InterceptedService::new(Self::new(inner), interceptor)
            }
            /// Enable decompressing requests with the given encoding.
            #[must_use]
            pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.accept_compression_encodings.enable(encoding);
                self
            }
            /// Compress responses with the given encoding, if the client supports it.
            #[must_use]
            pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.send_compression_encodings.enable(encoding);
                self
            }
            /// Limits the maximum size of a decoded message.
            ///
            /// Default: `4MB`
            #[must_use]
            pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
                self.max_decoding_message_size = Some(limit);
                self
            }
            /// Limits the maximum size of an encoded message.
            ///
            /// Default: `usize::MAX`
            #[must_use]
            pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
                self.max_encoding_message_size = Some(limit);
                self
            }
        }
        impl<T, B> tonic::codegen::Service<http::Request<B>> for ChoreoServiceServer<T>
        where
            T: ChoreoService,
            B: Body + std::marker::Send + 'static,
            B::Error: Into<StdError> + std::marker::Send + 'static,
        {
            type Response = http::Response<tonic::body::Body>;
            type Error = std::convert::Infallible;
            type Future = BoxFuture<Self::Response, Self::Error>;
            fn poll_ready(
                &mut self,
                _cx: &mut Context<'_>,
            ) -> Poll<std::result::Result<(), Self::Error>> {
                Poll::Ready(Ok(()))
            }
            fn call(&mut self, req: http::Request<B>) -> Self::Future {
                match req.uri().path() {
                    "/service.ChoreoService/EchoSwerveSample" => {
                        #[allow(non_camel_case_types)]
                        struct EchoSwerveSampleSvc<T: ChoreoService>(pub Arc<T>);
                        impl<
                            T: ChoreoService,
                        > tonic::server::UnaryService<
                            super::commands::EchoSwerveSampleRequest,
                        > for EchoSwerveSampleSvc<T> {
                            type Response = super::commands::EchoSwerveSampleResponse;
                            type Future = BoxFuture<
                                tonic::Response<Self::Response>,
                                tonic::Status,
                            >;
                            fn call(
                                &mut self,
                                request: tonic::Request<
                                    super::commands::EchoSwerveSampleRequest,
                                >,
                            ) -> Self::Future {
                                let inner = Arc::clone(&self.0);
                                let fut = async move {
                                    <T as ChoreoService>::echo_swerve_sample(&inner, request)
                                        .await
                                };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self
                            .accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let max_decoding_message_size = self.max_decoding_message_size;
                        let max_encoding_message_size = self.max_encoding_message_size;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let method = EchoSwerveSampleSvc(inner);
                            let codec = tonic_prost::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                )
                                .apply_max_message_size_config(
                                    max_decoding_message_size,
                                    max_encoding_message_size,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/service.ChoreoService/Generate" => {
                        #[allow(non_camel_case_types)]
                        struct GenerateSvc<T: ChoreoService>(pub Arc<T>);
                        impl<
                            T: ChoreoService,
                        > tonic::server::UnaryService<super::commands::GenerateRequest>
                        for GenerateSvc<T> {
                            type Response = super::commands::GenerateResponse;
                            type Future = BoxFuture<
                                tonic::Response<Self::Response>,
                                tonic::Status,
                            >;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::commands::GenerateRequest>,
                            ) -> Self::Future {
                                let inner = Arc::clone(&self.0);
                                let fut = async move {
                                    <T as ChoreoService>::generate(&inner, request).await
                                };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self
                            .accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let max_decoding_message_size = self.max_decoding_message_size;
                        let max_encoding_message_size = self.max_encoding_message_size;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let method = GenerateSvc(inner);
                            let codec = tonic_prost::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                )
                                .apply_max_message_size_config(
                                    max_decoding_message_size,
                                    max_encoding_message_size,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    "/service.ChoreoService/GetDefaultTrajectory" => {
                        #[allow(non_camel_case_types)]
                        struct GetDefaultTrajectorySvc<T: ChoreoService>(pub Arc<T>);
                        impl<
                            T: ChoreoService,
                        > tonic::server::UnaryService<::pbjson_types::Empty>
                        for GetDefaultTrajectorySvc<T> {
                            type Response = super::commands::GetDefaultTrajectoryResponse;
                            type Future = BoxFuture<
                                tonic::Response<Self::Response>,
                                tonic::Status,
                            >;
                            fn call(
                                &mut self,
                                request: tonic::Request<::pbjson_types::Empty>,
                            ) -> Self::Future {
                                let inner = Arc::clone(&self.0);
                                let fut = async move {
                                    <T as ChoreoService>::get_default_trajectory(
                                            &inner,
                                            request,
                                        )
                                        .await
                                };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self
                            .accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let max_decoding_message_size = self.max_decoding_message_size;
                        let max_encoding_message_size = self.max_encoding_message_size;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let method = GetDefaultTrajectorySvc(inner);
                            let codec = tonic_prost::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                )
                                .apply_max_message_size_config(
                                    max_decoding_message_size,
                                    max_encoding_message_size,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    _ => {
                        Box::pin(async move {
                            let mut response = http::Response::new(
                                tonic::body::Body::default(),
                            );
                            let headers = response.headers_mut();
                            headers
                                .insert(
                                    tonic::Status::GRPC_STATUS,
                                    (tonic::Code::Unimplemented as i32).into(),
                                );
                            headers
                                .insert(
                                    http::header::CONTENT_TYPE,
                                    tonic::metadata::GRPC_CONTENT_TYPE,
                                );
                            Ok(response)
                        })
                    }
                }
            }
        }
        impl<T> Clone for ChoreoServiceServer<T> {
            fn clone(&self) -> Self {
                let inner = self.inner.clone();
                Self {
                    inner,
                    accept_compression_encodings: self.accept_compression_encodings,
                    send_compression_encodings: self.send_compression_encodings,
                    max_decoding_message_size: self.max_decoding_message_size,
                    max_encoding_message_size: self.max_encoding_message_size,
                }
            }
        }
        /// Generated gRPC service name
        pub const SERVICE_NAME: &str = "service.ChoreoService";
        impl<T> tonic::server::NamedService for ChoreoServiceServer<T> {
            const NAME: &'static str = SERVICE_NAME;
        }
    }
    pub mod commands {
        pub struct EchoSwerveSampleRequest {
            #[prost(message, optional, tag = "1")]
            pub sample: ::core::option::Option<super::super::entity::SwerveSample>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for EchoSwerveSampleRequest {
            #[inline]
            fn clone(&self) -> EchoSwerveSampleRequest {
                let _: ::core::clone::AssertParamIsClone<
                    ::core::option::Option<super::super::entity::SwerveSample>,
                >;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for EchoSwerveSampleRequest {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for EchoSwerveSampleRequest {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for EchoSwerveSampleRequest {
            #[inline]
            fn eq(&self, other: &EchoSwerveSampleRequest) -> bool {
                self.sample == other.sample
            }
        }
        impl ::prost::Message for EchoSwerveSampleRequest {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if let Some(ref msg) = self.sample {
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
                const STRUCT_NAME: &'static str = "EchoSwerveSampleRequest";
                match tag {
                    1u32 => {
                        let mut value = &mut self.sample;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "sample");
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
                        .sample
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(1u32, msg),
                        )
            }
            fn clear(&mut self) {
                self.sample = ::core::option::Option::None;
            }
        }
        impl ::core::default::Default for EchoSwerveSampleRequest {
            fn default() -> Self {
                EchoSwerveSampleRequest {
                    sample: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for EchoSwerveSampleRequest {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("EchoSwerveSampleRequest");
                let builder = {
                    let wrapper = &self.sample;
                    builder.field("sample", &wrapper)
                };
                builder.finish()
            }
        }
        pub struct ValidEchoSwerveSampleRequest {
            pub sample: super::super::entity::ValidSwerveSample,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ValidEchoSwerveSampleRequest {
            #[inline]
            fn clone(&self) -> ValidEchoSwerveSampleRequest {
                let _: ::core::clone::AssertParamIsClone<
                    super::super::entity::ValidSwerveSample,
                >;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for ValidEchoSwerveSampleRequest {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ValidEchoSwerveSampleRequest {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ValidEchoSwerveSampleRequest {
            #[inline]
            fn eq(&self, other: &ValidEchoSwerveSampleRequest) -> bool {
                self.sample == other.sample
            }
        }
        impl TryFrom<EchoSwerveSampleRequest> for ValidEchoSwerveSampleRequest {
            type Error = String;
            fn try_from(
                optional: EchoSwerveSampleRequest,
            ) -> Result<ValidEchoSwerveSampleRequest, Self::Error> {
                Ok(ValidEchoSwerveSampleRequest {
                    sample: match optional.sample {
                        Some(object) => {
                            match object
                                .try_into()
                                .map_err(|e| {
                                    ::alloc::__export::must_use({
                                            ::alloc::fmt::format(format_args!("{0}", e))
                                        })
                                        .to_string()
                                })
                            {
                                Ok(valid_object) => valid_object,
                                Err(e) => return Err(e),
                            }
                        }
                        None => return Err("sample is missing".to_string()),
                    },
                })
            }
        }
        impl From<ValidEchoSwerveSampleRequest> for EchoSwerveSampleRequest {
            fn from(valid: ValidEchoSwerveSampleRequest) -> EchoSwerveSampleRequest {
                EchoSwerveSampleRequest {
                    sample: Some(valid.sample.into()),
                }
            }
        }
        impl crate::validate::Valid for ValidEchoSwerveSampleRequest {
            type Optional = EchoSwerveSampleRequest;
            fn optionize(self) -> EchoSwerveSampleRequest {
                self.into()
            }
        }
        impl crate::validate::Validate for EchoSwerveSampleRequest {
            type Valid = ValidEchoSwerveSampleRequest;
            fn validate(self) -> Result<ValidEchoSwerveSampleRequest, String> {
                self.try_into()
            }
        }
        pub struct EchoSwerveSampleResponse {
            #[prost(message, optional, tag = "1")]
            pub sample: ::core::option::Option<super::super::entity::SwerveSample>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for EchoSwerveSampleResponse {
            #[inline]
            fn clone(&self) -> EchoSwerveSampleResponse {
                let _: ::core::clone::AssertParamIsClone<
                    ::core::option::Option<super::super::entity::SwerveSample>,
                >;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for EchoSwerveSampleResponse {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for EchoSwerveSampleResponse {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for EchoSwerveSampleResponse {
            #[inline]
            fn eq(&self, other: &EchoSwerveSampleResponse) -> bool {
                self.sample == other.sample
            }
        }
        impl ::prost::Message for EchoSwerveSampleResponse {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if let Some(ref msg) = self.sample {
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
                const STRUCT_NAME: &'static str = "EchoSwerveSampleResponse";
                match tag {
                    1u32 => {
                        let mut value = &mut self.sample;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "sample");
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
                        .sample
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(1u32, msg),
                        )
            }
            fn clear(&mut self) {
                self.sample = ::core::option::Option::None;
            }
        }
        impl ::core::default::Default for EchoSwerveSampleResponse {
            fn default() -> Self {
                EchoSwerveSampleResponse {
                    sample: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for EchoSwerveSampleResponse {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("EchoSwerveSampleResponse");
                let builder = {
                    let wrapper = &self.sample;
                    builder.field("sample", &wrapper)
                };
                builder.finish()
            }
        }
        pub struct ValidEchoSwerveSampleResponse {
            pub sample: super::super::entity::ValidSwerveSample,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ValidEchoSwerveSampleResponse {
            #[inline]
            fn clone(&self) -> ValidEchoSwerveSampleResponse {
                let _: ::core::clone::AssertParamIsClone<
                    super::super::entity::ValidSwerveSample,
                >;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for ValidEchoSwerveSampleResponse {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ValidEchoSwerveSampleResponse {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ValidEchoSwerveSampleResponse {
            #[inline]
            fn eq(&self, other: &ValidEchoSwerveSampleResponse) -> bool {
                self.sample == other.sample
            }
        }
        impl TryFrom<EchoSwerveSampleResponse> for ValidEchoSwerveSampleResponse {
            type Error = String;
            fn try_from(
                optional: EchoSwerveSampleResponse,
            ) -> Result<ValidEchoSwerveSampleResponse, Self::Error> {
                Ok(ValidEchoSwerveSampleResponse {
                    sample: match optional.sample {
                        Some(object) => {
                            match object
                                .try_into()
                                .map_err(|e| {
                                    ::alloc::__export::must_use({
                                            ::alloc::fmt::format(format_args!("{0}", e))
                                        })
                                        .to_string()
                                })
                            {
                                Ok(valid_object) => valid_object,
                                Err(e) => return Err(e),
                            }
                        }
                        None => return Err("sample is missing".to_string()),
                    },
                })
            }
        }
        impl From<ValidEchoSwerveSampleResponse> for EchoSwerveSampleResponse {
            fn from(valid: ValidEchoSwerveSampleResponse) -> EchoSwerveSampleResponse {
                EchoSwerveSampleResponse {
                    sample: Some(valid.sample.into()),
                }
            }
        }
        impl crate::validate::Valid for ValidEchoSwerveSampleResponse {
            type Optional = EchoSwerveSampleResponse;
            fn optionize(self) -> EchoSwerveSampleResponse {
                self.into()
            }
        }
        impl crate::validate::Validate for EchoSwerveSampleResponse {
            type Valid = ValidEchoSwerveSampleResponse;
            fn validate(self) -> Result<ValidEchoSwerveSampleResponse, String> {
                self.try_into()
            }
        }
        pub struct GenerateRequest {
            #[prost(message, optional, tag = "1")]
            pub trajectory: ::core::option::Option<super::super::entity::TrajectoryFile>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for GenerateRequest {
            #[inline]
            fn clone(&self) -> GenerateRequest {
                GenerateRequest {
                    trajectory: ::core::clone::Clone::clone(&self.trajectory),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for GenerateRequest {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for GenerateRequest {
            #[inline]
            fn eq(&self, other: &GenerateRequest) -> bool {
                self.trajectory == other.trajectory
            }
        }
        impl ::prost::Message for GenerateRequest {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if let Some(ref msg) = self.trajectory {
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
                const STRUCT_NAME: &'static str = "GenerateRequest";
                match tag {
                    1u32 => {
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
                    + self
                        .trajectory
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(1u32, msg),
                        )
            }
            fn clear(&mut self) {
                self.trajectory = ::core::option::Option::None;
            }
        }
        impl ::core::default::Default for GenerateRequest {
            fn default() -> Self {
                GenerateRequest {
                    trajectory: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for GenerateRequest {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("GenerateRequest");
                let builder = {
                    let wrapper = &self.trajectory;
                    builder.field("trajectory", &wrapper)
                };
                builder.finish()
            }
        }
        pub struct ValidGenerateRequest {
            pub trajectory: super::super::entity::ValidTrajectoryFile,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ValidGenerateRequest {
            #[inline]
            fn clone(&self) -> ValidGenerateRequest {
                ValidGenerateRequest {
                    trajectory: ::core::clone::Clone::clone(&self.trajectory),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ValidGenerateRequest {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ValidGenerateRequest {
            #[inline]
            fn eq(&self, other: &ValidGenerateRequest) -> bool {
                self.trajectory == other.trajectory
            }
        }
        impl TryFrom<GenerateRequest> for ValidGenerateRequest {
            type Error = String;
            fn try_from(
                optional: GenerateRequest,
            ) -> Result<ValidGenerateRequest, Self::Error> {
                Ok(ValidGenerateRequest {
                    trajectory: match optional.trajectory {
                        Some(object) => {
                            match object
                                .try_into()
                                .map_err(|e| {
                                    ::alloc::__export::must_use({
                                            ::alloc::fmt::format(format_args!("{0}", e))
                                        })
                                        .to_string()
                                })
                            {
                                Ok(valid_object) => valid_object,
                                Err(e) => return Err(e),
                            }
                        }
                        None => return Err("trajectory is missing".to_string()),
                    },
                })
            }
        }
        impl From<ValidGenerateRequest> for GenerateRequest {
            fn from(valid: ValidGenerateRequest) -> GenerateRequest {
                GenerateRequest {
                    trajectory: Some(valid.trajectory.into()),
                }
            }
        }
        impl crate::validate::Valid for ValidGenerateRequest {
            type Optional = GenerateRequest;
            fn optionize(self) -> GenerateRequest {
                self.into()
            }
        }
        impl crate::validate::Validate for GenerateRequest {
            type Valid = ValidGenerateRequest;
            fn validate(self) -> Result<ValidGenerateRequest, String> {
                self.try_into()
            }
        }
        pub struct GenerateResponse {
            #[prost(message, optional, tag = "1")]
            pub trajectory: ::core::option::Option<super::super::entity::TrajectoryFile>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for GenerateResponse {
            #[inline]
            fn clone(&self) -> GenerateResponse {
                GenerateResponse {
                    trajectory: ::core::clone::Clone::clone(&self.trajectory),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for GenerateResponse {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for GenerateResponse {
            #[inline]
            fn eq(&self, other: &GenerateResponse) -> bool {
                self.trajectory == other.trajectory
            }
        }
        impl ::prost::Message for GenerateResponse {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if let Some(ref msg) = self.trajectory {
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
                const STRUCT_NAME: &'static str = "GenerateResponse";
                match tag {
                    1u32 => {
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
                    + self
                        .trajectory
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(1u32, msg),
                        )
            }
            fn clear(&mut self) {
                self.trajectory = ::core::option::Option::None;
            }
        }
        impl ::core::default::Default for GenerateResponse {
            fn default() -> Self {
                GenerateResponse {
                    trajectory: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for GenerateResponse {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("GenerateResponse");
                let builder = {
                    let wrapper = &self.trajectory;
                    builder.field("trajectory", &wrapper)
                };
                builder.finish()
            }
        }
        pub struct ValidGenerateResponse {
            pub trajectory: super::super::entity::ValidTrajectoryFile,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ValidGenerateResponse {
            #[inline]
            fn clone(&self) -> ValidGenerateResponse {
                ValidGenerateResponse {
                    trajectory: ::core::clone::Clone::clone(&self.trajectory),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ValidGenerateResponse {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ValidGenerateResponse {
            #[inline]
            fn eq(&self, other: &ValidGenerateResponse) -> bool {
                self.trajectory == other.trajectory
            }
        }
        impl TryFrom<GenerateResponse> for ValidGenerateResponse {
            type Error = String;
            fn try_from(
                optional: GenerateResponse,
            ) -> Result<ValidGenerateResponse, Self::Error> {
                Ok(ValidGenerateResponse {
                    trajectory: match optional.trajectory {
                        Some(object) => {
                            match object
                                .try_into()
                                .map_err(|e| {
                                    ::alloc::__export::must_use({
                                            ::alloc::fmt::format(format_args!("{0}", e))
                                        })
                                        .to_string()
                                })
                            {
                                Ok(valid_object) => valid_object,
                                Err(e) => return Err(e),
                            }
                        }
                        None => return Err("trajectory is missing".to_string()),
                    },
                })
            }
        }
        impl From<ValidGenerateResponse> for GenerateResponse {
            fn from(valid: ValidGenerateResponse) -> GenerateResponse {
                GenerateResponse {
                    trajectory: Some(valid.trajectory.into()),
                }
            }
        }
        impl crate::validate::Valid for ValidGenerateResponse {
            type Optional = GenerateResponse;
            fn optionize(self) -> GenerateResponse {
                self.into()
            }
        }
        impl crate::validate::Validate for GenerateResponse {
            type Valid = ValidGenerateResponse;
            fn validate(self) -> Result<ValidGenerateResponse, String> {
                self.try_into()
            }
        }
        pub struct GetDefaultTrajectoryResponse {
            #[prost(message, optional, tag = "1")]
            pub trajectory: ::core::option::Option<super::super::entity::TrajectoryFile>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for GetDefaultTrajectoryResponse {
            #[inline]
            fn clone(&self) -> GetDefaultTrajectoryResponse {
                GetDefaultTrajectoryResponse {
                    trajectory: ::core::clone::Clone::clone(&self.trajectory),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for GetDefaultTrajectoryResponse {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for GetDefaultTrajectoryResponse {
            #[inline]
            fn eq(&self, other: &GetDefaultTrajectoryResponse) -> bool {
                self.trajectory == other.trajectory
            }
        }
        impl ::prost::Message for GetDefaultTrajectoryResponse {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if let Some(ref msg) = self.trajectory {
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
                const STRUCT_NAME: &'static str = "GetDefaultTrajectoryResponse";
                match tag {
                    1u32 => {
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
                    + self
                        .trajectory
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(1u32, msg),
                        )
            }
            fn clear(&mut self) {
                self.trajectory = ::core::option::Option::None;
            }
        }
        impl ::core::default::Default for GetDefaultTrajectoryResponse {
            fn default() -> Self {
                GetDefaultTrajectoryResponse {
                    trajectory: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::fmt::Debug for GetDefaultTrajectoryResponse {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("GetDefaultTrajectoryResponse");
                let builder = {
                    let wrapper = &self.trajectory;
                    builder.field("trajectory", &wrapper)
                };
                builder.finish()
            }
        }
        pub struct ValidGetDefaultTrajectoryResponse {
            pub trajectory: super::super::entity::ValidTrajectoryFile,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ValidGetDefaultTrajectoryResponse {
            #[inline]
            fn clone(&self) -> ValidGetDefaultTrajectoryResponse {
                ValidGetDefaultTrajectoryResponse {
                    trajectory: ::core::clone::Clone::clone(&self.trajectory),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ValidGetDefaultTrajectoryResponse {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ValidGetDefaultTrajectoryResponse {
            #[inline]
            fn eq(&self, other: &ValidGetDefaultTrajectoryResponse) -> bool {
                self.trajectory == other.trajectory
            }
        }
        impl TryFrom<GetDefaultTrajectoryResponse>
        for ValidGetDefaultTrajectoryResponse {
            type Error = String;
            fn try_from(
                optional: GetDefaultTrajectoryResponse,
            ) -> Result<ValidGetDefaultTrajectoryResponse, Self::Error> {
                Ok(ValidGetDefaultTrajectoryResponse {
                    trajectory: match optional.trajectory {
                        Some(object) => {
                            match object
                                .try_into()
                                .map_err(|e| {
                                    ::alloc::__export::must_use({
                                            ::alloc::fmt::format(format_args!("{0}", e))
                                        })
                                        .to_string()
                                })
                            {
                                Ok(valid_object) => valid_object,
                                Err(e) => return Err(e),
                            }
                        }
                        None => return Err("trajectory is missing".to_string()),
                    },
                })
            }
        }
        impl From<ValidGetDefaultTrajectoryResponse> for GetDefaultTrajectoryResponse {
            fn from(
                valid: ValidGetDefaultTrajectoryResponse,
            ) -> GetDefaultTrajectoryResponse {
                GetDefaultTrajectoryResponse {
                    trajectory: Some(valid.trajectory.into()),
                }
            }
        }
        impl crate::validate::Valid for ValidGetDefaultTrajectoryResponse {
            type Optional = GetDefaultTrajectoryResponse;
            fn optionize(self) -> GetDefaultTrajectoryResponse {
                self.into()
            }
        }
        impl crate::validate::Validate for GetDefaultTrajectoryResponse {
            type Valid = ValidGetDefaultTrajectoryResponse;
            fn validate(self) -> Result<ValidGetDefaultTrajectoryResponse, String> {
                self.try_into()
            }
        }
    }
}
pub mod validate {
    use tonic::{Request, Response, Status};
    pub trait Validate {
        type Valid: Valid;
        fn validate(self) -> Result<Self::Valid, String>;
    }
    pub trait Valid {
        type Optional: Validate;
        fn optionize(self) -> Self::Optional;
    }
    pub fn validate<R: Validate + TryInto<<R as Validate>::Valid>>(
        request: Request<R>,
    ) -> Result<<R as Validate>::Valid, Status> {
        request
            .into_inner()
            .validate()
            .map_err(|e| {
                {
                    ::std::io::_eprint(format_args!("{0:?}\n", e));
                };
                Status::invalid_argument(e)
            })
    }
    /**
     * This doesn't fail but it needs the right error return signature for use in server implementations.
     */
    pub fn response<R: Valid + Into<<R as Valid>::Optional>>(
        body: R,
    ) -> Result<Response<<R as Valid>::Optional>, Status> {
        Ok(Response::new(body.into()))
    }
}
pub mod to_double {
    pub trait ToDouble {
        type DoubleType;
    }
}
impl Into<f64> for Expr {
    fn into(self) -> f64 {
        self.value
    }
}
