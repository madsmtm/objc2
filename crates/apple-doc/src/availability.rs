//! The structure of the data in `availability.index`.
//!
//! This is stored as a binary PList.
use std::{collections::BTreeMap, fmt, path::Path};

use bytemuck::{Pod, Zeroable};
use plist::Value;
use serde::{Deserialize, Deserializer};

use crate::Lang;

/// Data for converting platform availability IDs to the availability data.
#[derive(Clone, PartialEq, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AvailabilityLookup {
    pub data: BTreeMap<PlatformAvailabilityId, PlatformAvailability>,
    #[serde(rename = "interfaceLanguages")]
    pub interface_languages: Vec<InterfaceLanguages>,
    #[serde(rename = "languageToPlatforms")]
    pub language_to_platforms: Vec<Value>,
    pub platforms: Vec<Platform>,
}

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Deserialize, Debug, Zeroable, Pod)]
#[repr(transparent)]
pub struct AvailabilityMask(pub u64);

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Zeroable, Pod)]
#[repr(transparent)]
pub struct AvailabilityId(pub u64);

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Zeroable, Pod)]
#[repr(transparent)]
pub struct PlatformAvailabilityId(pub u64);

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlatformAvailability {
    #[serde(rename = "platformName")]
    pub platform_name: Platform,
    pub introduced: Option<AvailabilityVersion>,
    pub deprecated: Option<AvailabilityVersion>,
}

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AvailabilityVersion {
    #[serde(rename = "majorVersion")]
    pub major: u16,
    #[serde(rename = "minorVersion")]
    pub minor: u8,
    #[serde(rename = "patchVersion")]
    pub patch: u8,
}

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct InterfaceLanguages {
    pub id: String,
    pub mask: Lang,
    pub name: String,
}

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Platform {
    pub mask: AvailabilityMask,
    pub name: String,
}

impl AvailabilityLookup {
    pub fn from_external_dir(external_dir: &Path) -> Result<Self, plist::Error> {
        plist::from_file(external_dir.join("index/availability.index"))
    }
}

impl<'de> Deserialize<'de> for PlatformAvailabilityId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, Visitor};

        struct IdVisitor;

        impl<'de> Visitor<'de> for IdVisitor {
            type Value = PlatformAvailabilityId;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an u64 or a string that resembles that")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PlatformAvailabilityId(value))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(PlatformAvailabilityId(v.parse().map_err(E::custom)?))
            }
        }

        deserializer.deserialize_u64(IdVisitor)
    }
}
