// @generated
impl serde::Serialize for DoubleMaxVelocity {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("entity.parameters.constraint.maxvelocity.DoubleMaxVelocity", len)?;
        if self.max != 0. {
            struct_ser.serialize_field("max", &self.max)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DoubleMaxVelocity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Max,
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.constraint.maxvelocity.DoubleMaxVelocity")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DoubleMaxVelocity, V::Error>
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
                            max__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DoubleMaxVelocity {
                    max: max__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("entity.parameters.constraint.maxvelocity.DoubleMaxVelocity", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExprMaxVelocity {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("entity.parameters.constraint.maxvelocity.ExprMaxVelocity", len)?;
        if let Some(v) = self.max.as_ref() {
            struct_ser.serialize_field("max", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExprMaxVelocity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Max,
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.constraint.maxvelocity.ExprMaxVelocity")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExprMaxVelocity, V::Error>
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
                Ok(ExprMaxVelocity {
                    max: max__,
                })
            }
        }
        deserializer.deserialize_struct("entity.parameters.constraint.maxvelocity.ExprMaxVelocity", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestDouble {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.test.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("entity.parameters.constraint.maxvelocity.TestDouble", len)?;
        if !self.test.is_empty() {
            struct_ser.serialize_field("test", &self.test)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestDouble {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Test,
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.constraint.maxvelocity.TestDouble")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestDouble, V::Error>
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
        deserializer.deserialize_struct("entity.parameters.constraint.maxvelocity.TestDouble", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestExpr {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.test.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("entity.parameters.constraint.maxvelocity.TestExpr", len)?;
        if !self.test.is_empty() {
            struct_ser.serialize_field("test", &self.test)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestExpr {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Test,
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.constraint.maxvelocity.TestExpr")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestExpr, V::Error>
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
        deserializer.deserialize_struct("entity.parameters.constraint.maxvelocity.TestExpr", FIELDS, GeneratedVisitor)
    }
}
