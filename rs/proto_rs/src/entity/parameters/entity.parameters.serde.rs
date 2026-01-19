// @generated
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
        let mut struct_ser = serializer.serialize_struct("entity.parameters.DoubleParameters", len)?;
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.DoubleParameters")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DoubleParameters, V::Error>
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
                            target_dt__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Waypoints => {
                            if waypoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("waypoints"));
                            }
                            waypoints__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Constraints => {
                            if constraints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("constraints"));
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
        deserializer.deserialize_struct("entity.parameters.DoubleParameters", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("entity.parameters.Expr", len)?;
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
        const FIELDS: &[&str] = &[
            "value",
            "expr",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            Expr,
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.Expr")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Expr, V::Error>
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
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
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
        deserializer.deserialize_struct("entity.parameters.Expr", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("entity.parameters.ExprParameters", len)?;
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.ExprParameters")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExprParameters, V::Error>
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
                                return Err(serde::de::Error::duplicate_field("constraints"));
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
        deserializer.deserialize_struct("entity.parameters.ExprParameters", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("entity.parameters.WaypointID", len)?;
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
        const FIELDS: &[&str] = &[
            "first",
            "last",
            "idx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            First,
            Last,
            Idx,
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.WaypointID")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WaypointId, V::Error>
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
                            id__ = map_.next_value::<::std::option::Option<_>>()?.map(waypoint_id::Id::First)
;
                        }
                        GeneratedField::Last => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("last"));
                            }
                            id__ = map_.next_value::<::std::option::Option<_>>()?.map(waypoint_id::Id::Last)
;
                        }
                        GeneratedField::Idx => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idx"));
                            }
                            id__ = map_.next_value::<::std::option::Option<_>>()?.map(waypoint_id::Id::Idx)
;
                        }
                    }
                }
                Ok(WaypointId {
                    id: id__,
                })
            }
        }
        deserializer.deserialize_struct("entity.parameters.WaypointID", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("entity.parameters.WaypointIDFirst", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WaypointIdFirst {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WaypointIdFirst;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.WaypointIDFirst")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WaypointIdFirst, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(WaypointIdFirst {
                })
            }
        }
        deserializer.deserialize_struct("entity.parameters.WaypointIDFirst", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("entity.parameters.WaypointIDLast", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WaypointIdLast {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WaypointIdLast;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.WaypointIDLast")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WaypointIdLast, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(WaypointIdLast {
                })
            }
        }
        deserializer.deserialize_struct("entity.parameters.WaypointIDLast", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("entity.parameters.WaypointIDX", len)?;
        if self.idx != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("idx", ToString::to_string(&self.idx).as_str())?;
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
        const FIELDS: &[&str] = &[
            "idx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Idx,
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.parameters.WaypointIDX")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WaypointIdx, V::Error>
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
                            idx__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(WaypointIdx {
                    idx: idx__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("entity.parameters.WaypointIDX", FIELDS, GeneratedVisitor)
    }
}
