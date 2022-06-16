//! https://developer.apple.com/library/archive/documentation/DeveloperTools/Conceptual/cross_development/Introduction/Introduction.html
//! https://developer.apple.com/library/archive/technotes/tn2064/_index.html
//! See also AvailabilityMacros.h and Availability.h
//! https://clang.llvm.org/docs/LanguageExtensions.html#objective-c-available

// Generate relevant cfg attributes based on:
// - MACOSX_DEPLOYMENT_TARGET (default 10.7)
// - IPHONEOS_DEPLOYMENT_TARGET (default 7.0)
// - TVOS_DEPLOYMENT_TARGET
// - WATCHOS_DEPLOYMENT_TARGET
// - BRIDGEOS_DEPLOYMENT_TARGET

// And figure out the macabi thing

// Clang flags:
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

// Cargo's compilation process works as follows:
//
// `std::env::var`
// mod deployment_target {
//     pub const MACOSX: &'static str = option_env!("MACOSX_DEPLOYMENT_TARGET").unwrap_or("10.7");
//     pub const IPHONEOS: &'static str = option_env!("IPHONEOS_DEPLOYMENT_TARGET").unwrap_or("7.0");
//     pub const TVOS: &'static str = option_env!("TVOS_DEPLOYMENT_TARGET").unwrap_or("");
//     pub const WATCHOS: &'static str = option_env!("WATCHOS_DEPLOYMENT_TARGET").unwrap_or("");
//     pub const BRIDGEOS: &'static str = option_env!("BRIDGEOS_DEPLOYMENT_TARGET").unwrap_or("");
// }

#[cfg(ideas)]
mod ideas {
    //! Ideas for usage

    // Procedural macros

    // Mimic Objective-C:
    // __API_AVAILABLE(macos(10.4), ios(8.0), watchos(2.0), tvos(10.0))
    #[available(macos(10.9), ios(8.0))]
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
    #[available(macos > 10.9, ios > 8.0)]
    fn my_fn() {}

    // Simple macros

    available! {
        [macos(10.9), ios(8.0)]

        fn my_fn1() {}
        fn my_fn2() {}
    }

    // Configs (the user would have to have a build script that made these)

    #[cfg(macos_10_9, ios_8_0)]
    fn my_fn() {}

    // Consider using darwin instead of macos? And iphoneos instead of ios?

    // Consider conditional deprecation?
    // __API_DEPRECATED("No longer supported", macos(10.4, 10.8))

    // Control when they are enabled like this:
    // - MACOSX_DEPLOYMENT_TARGET=10.9
    // - IPHONEOS_DEPLOYMENT_TARGET=8.0
    // - availability = { features = ["macos-10-9", "ios-8-0"] } ???
    //
    // Should we try to handle weak linking? Or just point to options on how
    // to do that?
    //
    // The user's SDK version has an effect as well?
    // Nope (unless compiling some C files), rather the SDK version that our
    // Rust code is created from is relevant.

    // OBJC_OSX_DEPRECATED_OTHERS_UNAVAILABLE / OBJC_OSX_AVAILABLE_OTHERS_UNAVAILABLE
    // OBJC2_UNAVAILABLE

    // The user setting `cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.11` in their build scripts...

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
