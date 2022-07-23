//! https://developer.apple.com/library/archive/documentation/DeveloperTools/Conceptual/cross_development/Introduction/Introduction.html
//! https://developer.apple.com/library/archive/technotes/tn2064/_index.html
//! See also AvailabilityMacros.h and Availability.h
//! https://clang.llvm.org/docs/LanguageExtensions.html#objective-c-available
use std::iter;

use crate::deployment_target::{self, Version};
use proc_macro::TokenStream;

fn macos(min: Version) -> Option<&'static str> {
    // Note: Implicitly assumes that `macos_aarch64` will return an equal or
    // higher value than `macos`

    if deployment_target::macos_aarch64() >= min {
        // Available on all macOS
        Some(r#"target_os = "macos""#)
    } else if deployment_target::macos() >= min {
        // Available on Aarch64, not available on others
        Some(r#"not(all(target_os = "macos", not(target_arch = "aarch64")))"#)
    } else {
        // Not available on macOS
        None
    }
}

fn ios(min: Version) -> Option<&'static str> {
    if deployment_target::ios() >= min {
        Some(r#"target_os = "ios""#)
    } else {
        None
    }
}

fn tvos(min: Version) -> Option<&'static str> {
    match deployment_target::tvos() {
        Some(v) if v >= min => Some(r#"target_os = "tvos""#),
        // Disable everything on tvOS if deployment target is not specified
        // TODO: Change this once rustc sets a default deployment target
        _ => None,
    }
}

fn watchos(min: Version) -> Option<&'static str> {
    if deployment_target::watchos() >= min {
        Some(r#"target_os = "watchos""#)
    } else {
        None
    }
}

#[derive(Debug, Default)]
pub(crate) struct AvailableSince {
    macos: Option<Version>,
    ios: Option<Version>,
    tvos: Option<Version>,
    watchos: Option<Version>,
}

impl AvailableSince {
    pub(crate) fn from_tokenstream(attr: TokenStream) -> Self {
        todo!()
    }

    fn into_cfg_string(&self) -> String {
        let mut result = "any(".to_string();
        if let Some(s) = self.macos.and_then(macos) {
            result += s;
            result += ", ";
        }
        if let Some(s) = self.ios.and_then(ios) {
            result += s;
            result += ", ";
        }
        if let Some(s) = self.tvos.and_then(tvos) {
            result += s;
            result += ", ";
        }
        if let Some(s) = self.watchos.and_then(watchos) {
            result += s;
            result += ", ";
        }
        if result == "any(" {
            // If didn't change
            result.push_str("__not_available_anywhere");
        } else {
            // Remove extra ", "
            result.pop();
            result.pop();
        }
        result += ")";
        result
    }

    pub(crate) fn into_cfg(self) -> TokenStream {
        format!("cfg({})", self.into_cfg_string())
            .parse()
            .expect("invalid cfg string")
    }

    pub(crate) fn into_not_cfg(self) -> TokenStream {
        format!("cfg(not({}))", self.into_cfg_string())
            .parse()
            .expect("invalid cfg string")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_availability_cfg_string() {
        #[track_caller]
        fn assert_cfg(available_since: &AvailableSince, expected: &str) {
            assert_eq!(available_since.into_cfg_string(), expected);
        }

        let mut available_since = AvailableSince::default();
        assert_cfg(&available_since, r#"any(__not_available_anywhere)"#);

        available_since.macos = Some(Version::new(12, 0));
        assert_cfg(&available_since, r#"any(__not_available_anywhere)"#);
        available_since.macos = Some(Version::new(10, 0));
        assert_cfg(&available_since, r#"any(target_os = "macos")"#);

        available_since.ios = Some(Version::new(9, 0));
        assert_cfg(&available_since, r#"any(target_os = "macos")"#);
        available_since.ios = Some(Version::new(5, 0));
        assert_cfg(
            &available_since,
            r#"any(target_os = "macos", target_os = "ios")"#,
        );

        available_since.tvos = Some(Version::new(0, 0));
        assert_cfg(
            &available_since,
            r#"any(target_os = "macos", target_os = "ios")"#,
        );

        available_since.watchos = Some(Version::new(0, 0));
        assert_cfg(
            &available_since,
            r#"any(target_os = "macos", target_os = "ios", target_os = "watchos")"#,
        );
    }
}

#[cfg(ideas)]
mod ideas {
    //! Ideas for usage

    // Procedural macros

    // Mimic Objective-C:
    // __API_AVAILABLE(macos(10.4), ios(8.0), watchos(2.0), tvos(10.0))
    #[available_since(macos(10.9), ios(8.0))]
    // #[not_available(macos(10.9), ios(8.0))]
    fn my_fn() {}

    // Mimic Swift's @available
    #[available(macOS 10.15, iOS 13, watchOS 6, tvOS 13, *)]
    #[available(macOS, introduced: 10.15)]
    #[available(iOS, introduced: 13)]
    #[available(iOS, deprecated: 14)]
    #[available(iOS, obsoleted: 15)]
    #[available(iOS, unavailable)]
    fn my_fn() {}

    #[available::macos(10.9)]
    #[available::ios(8.0)]
    // #[available::macos(not(10.9))]
    // #[available::ios(not(8.0))]
    // #[available::not_macos(10.9)]
    // #[available::not_ios(8.0)]
    fn my_fn() {}

    // https://crates.io/crates/assert2
    #[available(macos >= 10.9, ios >= 8.0)]
    fn my_fn() {}

    // Helper functions / macros for doing runtime version checks (and omit
    // them when the DEPLOYMENT_TARGET is high enough).

    fn some_fn() {
        if available_macos(10.9) && available_ios(6.0) {
        } else {
        }

        if available!(macos(10.9), ios(6.0)) {
        } else {
        }

        #[cfg(target_os = "macos")]
        if available_macos(10.9) {
        } else {
        }
        #[cfg(target_os = "ios")]
        if available_ios(6.0) {
        } else {
        }

        #[is_available(macos(10.9), ios(6.0))]
        {}
        #[not(is_available(macos(10.9), ios(6.0)))]
        {}

        if available(Availability {
            macos: 10.9,
            ios: 6.0,
            ..Default::default()
        }) {
        } else {
        }
    }
}
