//! Utilities for getting the deployment target of the application.
//!
//! There are many different ways of getting the deployment target environment
//! variable. Some possibilities include:
//! 1. Use `option_env!` in the `proc-macro`
//! 2. Use `std::env::var` in the `proc-macro`
//! 3. Add `build.rs` to proc-macro, and use `option_env!` there.
//! 4. Add `build.rs` to proc-macro, and use `std::env::var` there.
//! 5. Output an `option_env!` within the crate itself.
//! 6. Output an `std::env::var` within the crate itself.
//!
//! Option 1, 3 and 4 would require a recompilation of the `proc-macro` crate,
//! while option 2, 5 and 6 wouldn't.
//!
//! Option 6 is out of the question, since that would be a dynamic check at
//! binary runtime!
//!
//! Option 5 is interesting, but ultimately won't work since we need to emit
//! `cfg` guards, and `option_env!` can't be used in those.
//!
//! Option 4 is not really different from option 1, except adding extra
//! compile time (since environment variables are, as far as I can tell)
//!
//! Option 3 is basically just an inefficient version of option 4.
//!
//! Option 2 is interesting in that it can be influenced by the user setting
//! the environemnt variable in their build script using `cargo:rustc-env=`.
//! However, this is probably an anti-feature, since that will only work for
//! the specific crate with the build script, while any other crates will
//! assume a different deployment target!
//! But more importantly, we can't ensure that the target crate is recompiled
//! when the environment variable changes (could be hacked by _also_
//! outputting `option_env!` into the target crate, but that seems brittle,
//! and the solution , `proc_macro::tracked_env::var` is nightly-only).
//!
//! Basically, option 1 seemed the most stable.

use std::{fmt, num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub(crate) struct Version {
    major: u32,
    minor: u32,
    /// Some things (e.g. `objc_setHook_getClass`) are cfg-gated on `10.14.4`,
    /// so allow parsing that.
    subminor: u32,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.subminor == 0 {
            write!(f, "{}.{}", self.major, self.minor)
        } else {
            write!(f, "{}.{}.{}", self.major, self.minor, self.subminor)
        }
    }
}

impl Version {
    pub(crate) fn new(major: u32, minor: u32) -> Self {
        Self {
            major,
            minor,
            subminor: 0,
        }
    }

    fn new_subminor(major: u32, minor: u32, subminor: u32) -> Self {
        Self {
            major,
            minor,
            subminor,
        }
    }

    fn from_env(env: &str) -> Self {
        match Version::from_str(env) {
            Ok(v) => v,
            // TODO: Don't ignore error
            Err(_) => panic!(concat!(
                "invalid deployment target version! ",
                "Must have the format `major.minor[.subminor]`. ",
                "Example: `10.7`, `7.0`, `10.14.4`."
            )),
        }
    }
}

impl FromStr for Version {
    type Err = VersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('.');
        let major = iter.next().ok_or(VersionParseError::Empty)?.parse()?;
        let minor = iter.next().ok_or(VersionParseError::TooShort)?.parse()?;
        let res = match iter.next() {
            Some(subminor) => Self::new_subminor(major, minor, subminor.parse()?),
            None => Self::new(major, minor),
        };
        if iter.next().is_some() {
            return Err(VersionParseError::TooLong);
        }
        Ok(res)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum VersionParseError {
    Empty,
    TooShort,
    TooLong,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for VersionParseError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

// Default values are the same as rustc:
// https://github.com/rust-lang/rust/blob/e55c53c57e953a4f5716461dbaf4af5d623d80da/compiler/rustc_target/src/spec/apple_base.rs
pub(crate) fn macos_aarch64() -> Version {
    option_env!("MACOSX_DEPLOYMENT_TARGET")
        .map(Version::from_env)
        .unwrap_or_else(|| Version::new(11, 0))
}
pub(crate) fn macos() -> Version {
    option_env!("MACOSX_DEPLOYMENT_TARGET")
        .map(Version::from_env)
        .unwrap_or_else(|| Version::new(10, 7))
}
pub(crate) fn ios() -> Version {
    option_env!("IPHONEOS_DEPLOYMENT_TARGET")
        .map(Version::from_env)
        .unwrap_or_else(|| Version::new(7, 0))
}
pub(crate) fn tvos() -> Option<Version> {
    // TODO: Set a default value here when rustc does
    option_env!("TVOS_DEPLOYMENT_TARGET").map(Version::from_env)
}
pub(crate) fn watchos() -> Version {
    option_env!("WATCHOS_DEPLOYMENT_TARGET")
        .map(Version::from_env)
        .unwrap_or_else(|| Version::new(5, 0))
}
// No Rust target for `BRIDGEOS_DEPLOYMENT_TARGET`, and it probably won't come

// Clang flags for these environment variables:
// -mmacos-version-min
// -mios-version-min
// -mtvos-version-min
// -mwatchos-version-min
// -mbridgeos-version-min
// -mmaccatalyst-version-min
// -mios-simulator-version-min
// -mtvos-simulator-version-min
// -mwatchos-simulator-version-min
// -mdriverkit-version-min

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_cmp() {
        assert_eq!(Version::new(10, 0), Version::new(10, 0));
        assert!(Version::new(10, 0) < Version::new(10, 1));
        assert!(Version::new(10, 7) < Version::new(11, 0));
        assert!(Version::new(7, 0) < Version::new(9, 1));

        // Subminor
        assert!(Version::new_subminor(7, 0, 1) < Version::new_subminor(7, 0, 2));
        assert!(Version::new(7, 0) < Version::new_subminor(7, 0, 1));
        assert_eq!(Version::new(7, 0), Version::new_subminor(7, 0, 0));
    }

    #[test]
    fn version_env_parse() {
        assert_eq!(Version::from_str("7.0").unwrap(), Version::new(7, 0));
        assert_eq!(Version::from_str("7.0.0").unwrap(), Version::new(7, 0));
        assert_eq!(
            Version::from_str("7.0.1").unwrap(),
            Version::new_subminor(7, 0, 1)
        );
        assert_eq!(Version::from_str("5.0").unwrap(), Version::new(5, 0));
        assert_eq!(Version::from_str("10.7").unwrap(), Version::new(10, 7));
        assert_eq!(Version::from_str("12.1").unwrap(), Version::new(12, 1));
        assert_eq!(
            Version::from_str("10.14.4").unwrap(),
            Version::new_subminor(10, 14, 4)
        );
    }

    #[test]
    #[should_panic]
    fn version_env_not_number() {
        let _ = Version::from_str("abc").unwrap();
    }

    #[test]
    #[should_panic]
    fn version_env_large_number() {
        let _ = Version::from_str("1234567890123467890.0").unwrap();
    }

    #[test]
    #[should_panic]
    fn version_env_too_few() {
        let _ = Version::from_str("10").unwrap();
    }

    #[test]
    #[should_panic]
    fn version_env_too_many() {
        let _ = Version::from_str("10.0.0.0").unwrap();
    }
}
