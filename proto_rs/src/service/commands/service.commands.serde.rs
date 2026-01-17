// @generated
impl serde::Serialize for EchoSwerveSampleRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sample.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("service.commands.EchoSwerveSampleRequest", len)?;
        if let Some(v) = self.sample.as_ref() {
            struct_ser.serialize_field("sample", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EchoSwerveSampleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sample",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sample,
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
                            "sample" => Ok(GeneratedField::Sample),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EchoSwerveSampleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct service.commands.EchoSwerveSampleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EchoSwerveSampleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sample__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sample => {
                            if sample__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sample"));
                            }
                            sample__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EchoSwerveSampleRequest {
                    sample: sample__,
                })
            }
        }
        deserializer.deserialize_struct("service.commands.EchoSwerveSampleRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EchoSwerveSampleResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sample.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("service.commands.EchoSwerveSampleResponse", len)?;
        if let Some(v) = self.sample.as_ref() {
            struct_ser.serialize_field("sample", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EchoSwerveSampleResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sample",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sample,
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
                            "sample" => Ok(GeneratedField::Sample),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EchoSwerveSampleResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct service.commands.EchoSwerveSampleResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EchoSwerveSampleResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sample__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sample => {
                            if sample__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sample"));
                            }
                            sample__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EchoSwerveSampleResponse {
                    sample: sample__,
                })
            }
        }
        deserializer.deserialize_struct("service.commands.EchoSwerveSampleResponse", FIELDS, GeneratedVisitor)
    }
}
