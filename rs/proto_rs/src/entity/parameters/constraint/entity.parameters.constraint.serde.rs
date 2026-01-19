// @generated
impl serde::Serialize for DoubleConstraint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
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
        let mut struct_ser = serializer.serialize_struct("entity.parameters.constraint.DoubleConstraint", len)?;
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
                            "enabled" => Ok(GeneratedField::Enabled),
                            "from" => Ok(GeneratedField::From),
                            "to" => Ok(GeneratedField::To),
                            "maxVelocity" | "max_velocity" => Ok(GeneratedField::MaxVelocity),
                            "maxAcceleration" | "max_acceleration" => Ok(GeneratedField::MaxAcceleration),
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.constraint.DoubleConstraint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DoubleConstraint, V::Error>
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
                                return Err(serde::de::Error::duplicate_field("maxVelocity"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(double_constraint::Data::MaxVelocity)
;
                        }
                        GeneratedField::MaxAcceleration => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAcceleration"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(double_constraint::Data::MaxAcceleration)
;
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
        deserializer.deserialize_struct("entity.parameters.constraint.DoubleConstraint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExprConstraint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
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
        let mut struct_ser = serializer.serialize_struct("entity.parameters.constraint.ExprConstraint", len)?;
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
                            "enabled" => Ok(GeneratedField::Enabled),
                            "from" => Ok(GeneratedField::From),
                            "to" => Ok(GeneratedField::To),
                            "maxVelocity" | "max_velocity" => Ok(GeneratedField::MaxVelocity),
                            "maxAcceleration" | "max_acceleration" => Ok(GeneratedField::MaxAcceleration),
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.constraint.ExprConstraint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExprConstraint, V::Error>
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
                                return Err(serde::de::Error::duplicate_field("maxVelocity"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(expr_constraint::Data::MaxVelocity)
;
                        }
                        GeneratedField::MaxAcceleration => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAcceleration"));
                            }
                            data__ = map_.next_value::<::std::option::Option<_>>()?.map(expr_constraint::Data::MaxAcceleration)
;
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
        deserializer.deserialize_struct("entity.parameters.constraint.ExprConstraint", FIELDS, GeneratedVisitor)
    }
}
