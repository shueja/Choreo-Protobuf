// @generated
impl serde::Serialize for DoubleWaypoint {
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
        let mut struct_ser = serializer.serialize_struct("entity.parameters.waypoint.DoubleWaypoint", len)?;
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
            struct_ser.serialize_field("intervals", ToString::to_string(&self.intervals).as_str())?;
        }
        if self.split {
            struct_ser.serialize_field("split", &self.split)?;
        }
        if self.fix_translation {
            struct_ser.serialize_field("fixTranslation", &self.fix_translation)?;
        }
        if self.fix_heading {
            struct_ser.serialize_field("fixHeading", &self.fix_heading)?;
        }
        if self.override_intervals {
            struct_ser.serialize_field("overrideIntervals", &self.override_intervals)?;
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
                            "heading" => Ok(GeneratedField::Heading),
                            "intervals" => Ok(GeneratedField::Intervals),
                            "split" => Ok(GeneratedField::Split),
                            "fixTranslation" | "fix_translation" => Ok(GeneratedField::FixTranslation),
                            "fixHeading" | "fix_heading" => Ok(GeneratedField::FixHeading),
                            "overrideIntervals" | "override_intervals" => Ok(GeneratedField::OverrideIntervals),
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.waypoint.DoubleWaypoint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DoubleWaypoint, V::Error>
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
                        GeneratedField::Heading => {
                            if heading__.is_some() {
                                return Err(serde::de::Error::duplicate_field("heading"));
                            }
                            heading__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Intervals => {
                            if intervals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intervals"));
                            }
                            intervals__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Split => {
                            if split__.is_some() {
                                return Err(serde::de::Error::duplicate_field("split"));
                            }
                            split__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FixTranslation => {
                            if fix_translation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixTranslation"));
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
                                return Err(serde::de::Error::duplicate_field("overrideIntervals"));
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
        deserializer.deserialize_struct("entity.parameters.waypoint.DoubleWaypoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExprWaypoint {
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
        let mut struct_ser = serializer.serialize_struct("entity.parameters.waypoint.ExprWaypoint", len)?;
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
            struct_ser.serialize_field("intervals", ToString::to_string(&self.intervals).as_str())?;
        }
        if self.split {
            struct_ser.serialize_field("split", &self.split)?;
        }
        if self.fix_translation {
            struct_ser.serialize_field("fixTranslation", &self.fix_translation)?;
        }
        if self.fix_heading {
            struct_ser.serialize_field("fixHeading", &self.fix_heading)?;
        }
        if self.override_intervals {
            struct_ser.serialize_field("overrideIntervals", &self.override_intervals)?;
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
                            "heading" => Ok(GeneratedField::Heading),
                            "intervals" => Ok(GeneratedField::Intervals),
                            "split" => Ok(GeneratedField::Split),
                            "fixTranslation" | "fix_translation" => Ok(GeneratedField::FixTranslation),
                            "fixHeading" | "fix_heading" => Ok(GeneratedField::FixHeading),
                            "overrideIntervals" | "override_intervals" => Ok(GeneratedField::OverrideIntervals),
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.waypoint.ExprWaypoint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExprWaypoint, V::Error>
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
                            intervals__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Split => {
                            if split__.is_some() {
                                return Err(serde::de::Error::duplicate_field("split"));
                            }
                            split__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FixTranslation => {
                            if fix_translation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixTranslation"));
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
                                return Err(serde::de::Error::duplicate_field("overrideIntervals"));
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
        deserializer.deserialize_struct("entity.parameters.waypoint.ExprWaypoint", FIELDS, GeneratedVisitor)
    }
}
