// @generated
impl serde::Serialize for DoubleBumper {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
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
        let mut struct_ser = serializer.serialize_struct("entity.parameters.robotconfig.DoubleBumper", len)?;
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
        const FIELDS: &[&str] = &[
            "front",
            "left",
            "right",
            "back",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Front,
            Left,
            Right,
            Back,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.robotconfig.DoubleBumper")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DoubleBumper, V::Error>
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
                            front__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Left => {
                            if left__.is_some() {
                                return Err(serde::de::Error::duplicate_field("left"));
                            }
                            left__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Right => {
                            if right__.is_some() {
                                return Err(serde::de::Error::duplicate_field("right"));
                            }
                            right__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Back => {
                            if back__.is_some() {
                                return Err(serde::de::Error::duplicate_field("back"));
                            }
                            back__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
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
        deserializer.deserialize_struct("entity.parameters.robotconfig.DoubleBumper", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DoubleModule {
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
        let mut struct_ser = serializer.serialize_struct("entity.parameters.robotconfig.DoubleModule", len)?;
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
        const FIELDS: &[&str] = &[
            "x",
            "y",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            X,
            Y,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.robotconfig.DoubleModule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DoubleModule, V::Error>
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
                            x__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Y => {
                            if y__.is_some() {
                                return Err(serde::de::Error::duplicate_field("y"));
                            }
                            y__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DoubleModule {
                    x: x__.unwrap_or_default(),
                    y: y__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("entity.parameters.robotconfig.DoubleModule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DoubleRobotConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
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
        let mut struct_ser = serializer.serialize_struct("entity.parameters.robotconfig.DoubleRobotConfig", len)?;
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
            struct_ser.serialize_field("differentialTrackWidth", &self.differential_track_width)?;
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
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
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
                            "differentialTrackWidth" | "differential_track_width" => Ok(GeneratedField::DifferentialTrackWidth),
                            "bumper" => Ok(GeneratedField::Bumper),
                            "frontLeft" | "front_left" => Ok(GeneratedField::FrontLeft),
                            "frontRight" | "front_right" => Ok(GeneratedField::FrontRight),
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.robotconfig.DoubleRobotConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DoubleRobotConfig, V::Error>
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
                            mass__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Inertia => {
                            if inertia__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inertia"));
                            }
                            inertia__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Gearing => {
                            if gearing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gearing"));
                            }
                            gearing__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Radius => {
                            if radius__.is_some() {
                                return Err(serde::de::Error::duplicate_field("radius"));
                            }
                            radius__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Vmax => {
                            if vmax__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vmax"));
                            }
                            vmax__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Tmax => {
                            if tmax__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tmax"));
                            }
                            tmax__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Cof => {
                            if cof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cof"));
                            }
                            cof__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DifferentialTrackWidth => {
                            if differential_track_width__.is_some() {
                                return Err(serde::de::Error::duplicate_field("differentialTrackWidth"));
                            }
                            differential_track_width__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
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
                    differential_track_width: differential_track_width__.unwrap_or_default(),
                    bumper: bumper__,
                    front_left: front_left__,
                    front_right: front_right__,
                    back_left: back_left__,
                    back_right: back_right__,
                })
            }
        }
        deserializer.deserialize_struct("entity.parameters.robotconfig.DoubleRobotConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExprBumper {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
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
        let mut struct_ser = serializer.serialize_struct("entity.parameters.robotconfig.ExprBumper", len)?;
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
        const FIELDS: &[&str] = &[
            "front",
            "left",
            "right",
            "back",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Front,
            Left,
            Right,
            Back,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.robotconfig.ExprBumper")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExprBumper, V::Error>
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
        deserializer.deserialize_struct("entity.parameters.robotconfig.ExprBumper", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExprModule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
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
        let mut struct_ser = serializer.serialize_struct("entity.parameters.robotconfig.ExprModule", len)?;
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
        const FIELDS: &[&str] = &[
            "x",
            "y",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            X,
            Y,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.robotconfig.ExprModule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExprModule, V::Error>
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
                Ok(ExprModule {
                    x: x__,
                    y: y__,
                })
            }
        }
        deserializer.deserialize_struct("entity.parameters.robotconfig.ExprModule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExprRobotConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
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
        let mut struct_ser = serializer.serialize_struct("entity.parameters.robotconfig.ExprRobotConfig", len)?;
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
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
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
                            "differentialTrackWidth" | "differential_track_width" => Ok(GeneratedField::DifferentialTrackWidth),
                            "bumper" => Ok(GeneratedField::Bumper),
                            "frontLeft" | "front_left" => Ok(GeneratedField::FrontLeft),
                            "frontRight" | "front_right" => Ok(GeneratedField::FrontRight),
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.robotconfig.ExprRobotConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExprRobotConfig, V::Error>
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
                                return Err(serde::de::Error::duplicate_field("differentialTrackWidth"));
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
        deserializer.deserialize_struct("entity.parameters.robotconfig.ExprRobotConfig", FIELDS, GeneratedVisitor)
    }
}
