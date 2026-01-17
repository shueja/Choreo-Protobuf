// @generated
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
        let mut struct_ser = serializer.serialize_struct("entity.DifferentialSample", len)?;
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.DifferentialSample")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DifferentialSample, V::Error>
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
                            t__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
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
                        GeneratedField::Vl => {
                            if vl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vl"));
                            }
                            vl__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Vr => {
                            if vr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vr"));
                            }
                            vr__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Omega => {
                            if omega__.is_some() {
                                return Err(serde::de::Error::duplicate_field("omega"));
                            }
                            omega__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Al => {
                            if al__.is_some() {
                                return Err(serde::de::Error::duplicate_field("al"));
                            }
                            al__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Ar => {
                            if ar__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ar"));
                            }
                            ar__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Alpha => {
                            if alpha__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alpha"));
                            }
                            alpha__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Fl => {
                            if fl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fl"));
                            }
                            fl__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Fr => {
                            if fr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fr"));
                            }
                            fr__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
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
        deserializer.deserialize_struct("entity.DifferentialSample", FIELDS, GeneratedVisitor)
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
            type Value = ForceVector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.ForceVector")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ForceVector, V::Error>
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
                Ok(ForceVector {
                    x: x__.unwrap_or_default(),
                    y: y__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("entity.ForceVector", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("entity.SwerveSample", len)?;
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entity.SwerveSample")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SwerveSample, V::Error>
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
                            t__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
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
                        GeneratedField::Vx => {
                            if vx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vx"));
                            }
                            vx__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Vy => {
                            if vy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vy"));
                            }
                            vy__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Omega => {
                            if omega__.is_some() {
                                return Err(serde::de::Error::duplicate_field("omega"));
                            }
                            omega__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Ax => {
                            if ax__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ax"));
                            }
                            ax__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Ay => {
                            if ay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ay"));
                            }
                            ay__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Alpha => {
                            if alpha__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alpha"));
                            }
                            alpha__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
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
        deserializer.deserialize_struct("entity.SwerveSample", FIELDS, GeneratedVisitor)
    }
}
