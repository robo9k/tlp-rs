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
    const LABEL_RED_STR: &str = "TLP:RED";
    const LABEL_AMBER_STRICT_STR: &str = "TLP:AMBER+STRICT";
    const LABEL_AMBER_STR: &str = "TLP:AMBER";
    const LABEL_GREEN_STR: &str = "TLP:GREEN";
    const LABEL_CLEAR_STR: &str = "TLP:CLEAR";

    // FIXME: this could be [u8]
    pub const fn from_str(_src: &str) -> Result<Self, ParseLabelError> {
        // TODO: https://doc.rust-lang.org/stable/core/primitive.str.html#method.eq_ignore_ascii_case
        todo!()
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
        let display = match self {
            Self::Red => Self::LABEL_RED_STR,
            Self::AmberStrict => Self::LABEL_AMBER_STRICT_STR,
            Self::Amber => Self::LABEL_AMBER_STR,
            Self::Green => Self::LABEL_GREEN_STR,
            Self::Clear => Self::LABEL_CLEAR_STR,
        };

        write!(f, "{}", display)
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
    use ::serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
    use ::serde::ser::{Serialize, SerializeStruct, Serializer};
    use core::fmt::Write;

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Label {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            todo!()
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Label {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            todo!()
        }
    }

    #[cfg(test)]
    mod tests {
        // TODO: test_serialize
        // TODO: test_deserialize
        // TODO: test_roundtrip
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

    // TODO: test_debug
    // TODO: test_display
    // TODO: test_from_str
    // TODO: test_fromstr

    // TODO: test_parselabelerror_display
}

#[cfg(doctest)]
#[doc=include_str!("../README-crate.md")]
mod readme {}
