//! Integration tests for Metal 4 detection utilities.
//!
//! These tests verify that the Metal 4 detection module works correctly
//! across different platforms and feature configurations.

#[cfg(feature = "std")]
use objc2_metal::metal4_detection;

/// Test that version constants are correctly defined.
#[test]
#[cfg(feature = "std")]
fn test_metal4_version_constants() {
    // Metal 4 requires OS version 26.0+
    assert_eq!(metal4_detection::METAL4_MACOS_VERSION, 26.0);
    assert_eq!(metal4_detection::METAL4_IOS_VERSION, 26.0);
    assert_eq!(metal4_detection::METAL4_TVOS_VERSION, 26.0);
    assert_eq!(metal4_detection::METAL4_VISIONOS_VERSION, 26.0);
}

/// Test platform detection function.
#[test]
#[cfg(feature = "std")]
fn test_is_metal4_platform() {
    // This should compile and return a boolean
    let is_platform = metal4_detection::is_metal4_platform();
    // We don't assert a specific value since it depends on the compilation target
    let _ = is_platform;
}

/// Test Metal4Features struct creation and access.
#[test]
#[cfg(feature = "std")]
fn test_metal4_features_struct() {
    let features = metal4_detection::Metal4Features::current();

    // All fields should be accessible
    let _ = features.has_metal4_types;
    let _ = features.is_supported_platform;
    let _ = features.has_all_features();
}

/// Test that Metal4Features::current() is a const fn.
#[test]
#[cfg(feature = "std")]
fn test_metal4_features_const() {
    // This should be evaluable at compile time
    const FEATURES: metal4_detection::Metal4Features = metal4_detection::Metal4Features::current();
    let _ = FEATURES;
}

/// Test Metal4Features default implementation.
#[test]
#[cfg(feature = "std")]
fn test_metal4_features_default() {
    let features = metal4_detection::Metal4Features::default();

    // Default should indicate no features available
    assert!(!features.is_supported_platform);
    assert!(!features.has_metal4_types);
    assert!(!features.has_all_features());
}

/// Test that the module is properly exported and accessible.
#[test]
#[cfg(feature = "std")]
fn test_module_exports() {
    // Test that the constants are accessible
    let _ = metal4_detection::METAL4_MACOS_VERSION;
    let _ = metal4_detection::METAL4_IOS_VERSION;
    let _ = metal4_detection::METAL4_TVOS_VERSION;
    let _ = metal4_detection::METAL4_VISIONOS_VERSION;

    // Test that functions are accessible
    let _ = metal4_detection::is_metal4_platform();

    // Test that types are accessible
    let _ = metal4_detection::Metal4Features::current();
}

/// Test that Metal4Features can be cloned and copied.
#[test]
#[cfg(feature = "std")]
fn test_metal4_features_clone_copy() {
    let features1 = metal4_detection::Metal4Features::current();
    let features2 = features1;

    // Should implement Copy
    let _ = features1;

    // Should implement Clone
    let features3 = features2.clone();
    let _ = (features2, features3);
}

/// Test that Metal4Features can be debug formatted.
#[test]
#[cfg(feature = "std")]
fn test_metal4_features_debug() {
    let features = metal4_detection::Metal4Features::current();
    let debug_str = format!("{:?}", features);
    assert!(!debug_str.is_empty());
}

/// Test platform-specific expectations.
#[test]
#[cfg(feature = "std")]
fn test_platform_specific_expectations() {
    let features = metal4_detection::Metal4Features::current();

    // If we're on a supported platform, is_supported_platform should be true
    if cfg!(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "visionos"
    )) {
        assert!(features.is_supported_platform);
    } else {
        assert!(!features.is_supported_platform);
    }
}

/// Test that Metal 4 types are available when features are enabled.
#[test]
#[allow(unused_imports)] // We import types just to verify they exist
fn test_metal4_types_available_when_features_enabled() {
    // This test verifies that Metal 4 types compile when features are enabled
    // The fact that this test compiles proves the types are accessible

    #[cfg(feature = "MTL4CommandQueue")]
    use objc2_metal::MTL4CommandQueue;

    #[cfg(feature = "MTL4CommandBuffer")]
    use objc2_metal::MTL4CommandBuffer;

    #[cfg(feature = "MTL4Compiler")]
    use objc2_metal::MTL4Compiler;

    #[cfg(feature = "MTL4ArgumentTable")]
    use objc2_metal::MTL4ArgumentTable;

    // If we get here, all imports were successful
    assert!(true);
}

/// Test that Metal 4 features are properly detected.
#[test]
#[cfg(feature = "std")]
fn test_metal4_features_detection() {
    let features = metal4_detection::Metal4Features::current();

    // If MTL4CommandQueue, MTL4CommandBuffer, or MTL4Compiler features are enabled,
    // has_metal4_types should be true
    #[cfg(any(
        feature = "MTL4CommandQueue",
        feature = "MTL4CommandBuffer",
        feature = "MTL4Compiler"
    ))]
    {
        assert!(
            features.has_metal4_types,
            "has_metal4_types should be true when Metal 4 features are enabled"
        );
    }
}
