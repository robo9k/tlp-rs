// https://www.first.org/tlp/

//! # Features
//!
//! - `serde` — Enable serializing and deserializing [`Label`] using `serde` v1
//! - `schemars` — Enable JSON schema for [`Label`] using `schemars` v1

#![deny(unsafe_code)]
#![cfg_attr(not(any(test)), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(auto_cfg(hide(docsrs))))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Label {
    /// TLP:RED
    Red,
    /// TLP:AMBER+STRICT
    AmberStrict,
    /// TLP:AMBER
    Amber,
    /// TLP:GREEN
    Green,
    /// TLP:CLEAR
    Clear,
}

impl Label {
    // TODO: pub?
    const RED: &str = "TLP:RED";
    const AMBER_STRICT: &str = "TLP:AMBER+STRICT";
    const AMBER: &str = "TLP:AMBER";
    const GREEN: &str = "TLP:GREEN";
    const CLEAR: &str = "TLP:CLEAR";

    // FIXME: this could be [u8]
    pub const fn from_str(src: &str) -> Result<Self, ParseLabelError> {
        if src.eq_ignore_ascii_case(Self::RED) {
            Ok(Self::Red)
        } else if src.eq_ignore_ascii_case(Self::AMBER_STRICT) {
            Ok(Self::AmberStrict)
        } else if src.eq_ignore_ascii_case(Self::AMBER) {
            Ok(Self::Amber)
        } else if src.eq_ignore_ascii_case(Self::GREEN) {
            Ok(Self::Green)
        } else if src.eq_ignore_ascii_case(Self::CLEAR) {
            Ok(Self::Clear)
        } else {
            Err(ParseLabelError())
        }
    }

    pub const fn as_str(&self) -> &str {
        match self {
            Self::Red => Self::RED,
            Self::AmberStrict => Self::AMBER_STRICT,
            Self::Amber => Self::AMBER,
            Self::Green => Self::GREEN,
            Self::Clear => Self::CLEAR,
        }
    }

    // TODO: const as_ansi_str / as_ansi_bytes
    // TODO: const as_rgb, as_cmyk, as_hex_str + background?
}

impl core::str::FromStr for Label {
    type Err = ParseLabelError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::from_str(src)
    }
}

impl core::fmt::Display for Label {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for Label {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<str> for Label {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for Label {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

#[cfg(feature = "alloc")]
impl PartialEq<alloc::string::String> for Label {
    fn eq(&self, other: &alloc::string::String) -> bool {
        self.as_str() == other
    }
}

#[cfg(feature = "alloc")]
impl PartialEq<alloc::borrow::Cow<'_, str>> for Label {
    fn eq(&self, other: &alloc::borrow::Cow<'_, str>) -> bool {
        self.as_str() == other
    }
}

/// Parse error for [`Label`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseLabelError();

impl core::fmt::Display for ParseLabelError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "can not parse TLP label")
    }
}

impl core::error::Error for ParseLabelError {}

#[cfg(feature = "serde")]
mod serde {
    use crate::Label;

    use serde_core::de::{Deserialize, Deserializer};
    use serde_core::ser::{Serialize, Serializer};

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Label {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                match self {
                    Label::Red => serializer.serialize_str(Label::RED),
                    Label::AmberStrict => serializer.serialize_str(Label::AMBER_STRICT),
                    Label::Amber => serializer.serialize_str(Label::AMBER),
                    Label::Green => serializer.serialize_str(Label::GREEN),
                    Label::Clear => serializer.serialize_str(Label::CLEAR),
                }
            } else {
                const NAME: &str = "Label";
                match self {
                    Label::Red => serializer.serialize_unit_variant(NAME, 0, "Red"),
                    Label::AmberStrict => serializer.serialize_unit_variant(NAME, 1, "AmberStrict"),
                    Label::Amber => serializer.serialize_unit_variant(NAME, 2, "Amber"),
                    Label::Green => serializer.serialize_unit_variant(NAME, 3, "Green"),
                    Label::Clear => serializer.serialize_unit_variant(NAME, 4, "Clear"),
                }
            }
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Label {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            use serde_core::de::{EnumAccess, VariantAccess as _, Visitor};
            use serde_core::de::{Error, Unexpected};

            const NAME: &str = "Label";

            const VARIANTS: &[&str] = &["Red", "AmberStrict", "Amber", "Green", "Clear"];

            enum LabelVariant {
                Red,
                AmberStrict,
                Amber,
                Green,
                Clear,
            }

            struct LabelVariantVisitor;

            impl<'de> Visitor<'de> for LabelVariantVisitor {
                type Value = LabelVariant;

                fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    formatter.write_str("Label variant identifier")
                }

                fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    match v {
                        0 => Ok(LabelVariant::Red),
                        1 => Ok(LabelVariant::AmberStrict),
                        2 => Ok(LabelVariant::Amber),
                        3 => Ok(LabelVariant::Green),
                        4 => Ok(LabelVariant::Clear),

                        _ => Err(Error::invalid_value(
                            Unexpected::Unsigned(v),
                            &"variant index 0 <= i < 5",
                        )),
                    }
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    match v {
                        "Red" => Ok(LabelVariant::Red),
                        "AmberStrict" => Ok(LabelVariant::AmberStrict),
                        "Amber" => Ok(LabelVariant::Amber),
                        "Green" => Ok(LabelVariant::Green),
                        "Clear" => Ok(LabelVariant::Clear),

                        _ => Err(Error::unknown_variant(v, VARIANTS)),
                    }
                }
            }

            impl<'de> Deserialize<'de> for LabelVariant {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    deserializer.deserialize_identifier(LabelVariantVisitor)
                }
            }

            struct LabelVisitor;

            impl<'de> Visitor<'de> for LabelVisitor {
                type Value = Label;

                fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    formatter.write_str("enum Label")
                }

                fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
                where
                    A: EnumAccess<'de>,
                {
                    match data.variant()? {
                        (LabelVariant::Red, variant) => {
                            variant.unit_variant()?;
                            Ok(Label::Red)
                        }
                        (LabelVariant::AmberStrict, variant) => {
                            variant.unit_variant()?;
                            Ok(Label::AmberStrict)
                        }
                        (LabelVariant::Amber, variant) => {
                            variant.unit_variant()?;
                            Ok(Label::Amber)
                        }
                        (LabelVariant::Green, variant) => {
                            variant.unit_variant()?;
                            Ok(Label::Green)
                        }
                        (LabelVariant::Clear, variant) => {
                            variant.unit_variant()?;
                            Ok(Label::Clear)
                        }
                    }
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    match v {
                        Label::RED => Ok(Label::Red),
                        Label::AMBER_STRICT => Ok(Label::AmberStrict),
                        Label::AMBER => Ok(Label::Amber),
                        Label::GREEN => Ok(Label::Green),
                        Label::CLEAR => Ok(Label::Clear),

                        _ => Err(Error::invalid_value(
                            Unexpected::Str(v),
                            &"one of \"TLP:RED\", \"TLP:AMBER+STRICT\", \"TLP:AMBER\", \"TLP:GREEN\", \"TLP:CLEAR\"",
                        )),
                    }
                }
            }

            if deserializer.is_human_readable() {
                deserializer.deserialize_str(LabelVisitor)
            } else {
                deserializer.deserialize_enum(NAME, VARIANTS, LabelVisitor)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use serde_test2::{
            Compact, Configure as _, Readable, Token, assert_de_tokens_error, assert_tokens,
        };

        #[test]
        fn ser_de() {
            let label = Label::AmberStrict;

            assert_tokens(
                &label.compact(),
                &[Token::UnitVariant {
                    name: "Label",
                    variant_index: 1,
                    variant: "AmberStrict",
                }],
            );

            assert_tokens(&label.readable(), &[Token::Str("TLP:AMBER+STRICT")]);
        }

        #[test]
        fn de_error() {
            assert_de_tokens_error::<Compact<Label>>(
                &[Token::UnitVariant {
                    name: "Label",
                    variant_index: 5,
                    variant: "hurz",
                }],
                "invalid value: unit variant, expected one of `Red`, `AmberStrict`, `Amber`, `Green`, `Clear`",
            );

            assert_de_tokens_error::<Readable<Label>>(
                &[Token::Str("hurz")],
                "invalid value: string \"hurz\", expected one of \"TLP:RED\", \"TLP:AMBER+STRICT\", \"TLP:AMBER\", \"TLP:GREEN\", \"TLP:CLEAR\"",
            );
        }
    }
}

#[cfg(feature = "schemars")]
mod schemars {
    use crate::Label;
    use alloc::borrow::Cow;
    use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};

    #[cfg_attr(docsrs, doc(cfg(feature = "schemars")))]
    impl JsonSchema for Label {
        fn schema_name() -> Cow<'static, str> {
            todo!()
        }

        fn schema_id() -> Cow<'static, str> {
            todo!()
        }

        fn json_schema(_: &mut SchemaGenerator) -> Schema {
            todo!()
        }

        fn inline_schema() -> bool {
            todo!()
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::Label;
        use schemars::JsonSchema;

        #[test]
        fn test_jsonschema() {
            fn assert_jsonschema<T: JsonSchema>() {}
            assert_jsonschema::<Label>();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Label>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Label>();
    }

    #[test]
    fn label_from_str() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(Label::Red, Label::from_str("TLP:red")?);
        assert_eq!(Label::AmberStrict, Label::from_str("TLP:amber+STRICT")?);
        assert_eq!(Label::Amber, Label::from_str("TLP:amBeR")?);
        assert_eq!(Label::Green, Label::from_str("TLP:GrEEn")?);
        assert_eq!(Label::Clear, Label::from_str("TLP:CLEAR")?);

        Ok(())
    }

    #[test]
    fn label_as_str() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(Label::Red.as_str(), "TLP:RED");
        assert_eq!(Label::AmberStrict.as_str(), "TLP:AMBER+STRICT");
        assert_eq!(Label::Amber.as_str(), "TLP:AMBER");
        assert_eq!(Label::Green.as_str(), "TLP:GREEN");
        assert_eq!(Label::Clear.as_str(), "TLP:CLEAR");

        Ok(())
    }

    // TODO: test_debug
    // TODO: test_display
    // TODO: test_fromstr

    // TODO: test_parselabelerror_display
}

#[cfg(doctest)]
#[doc=include_str!("../README-crate.md")]
mod readme {}
