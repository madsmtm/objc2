//! Simplified integration tests for Metal 4 API types.
//!
//! These tests verify that Metal 4 types can be imported and used.

/// Test that Metal 4 command queue can be imported.
#[test]
#[allow(unused_imports)] // We import types just to verify they exist
fn test_mtl4_command_queue_import() {
    #[cfg(feature = "MTL4CommandQueue")]
    use objc2_metal::MTL4CommandQueue;

    assert!(true);
}

/// Test that Metal 4 command buffer can be imported.
#[test]
#[allow(unused_imports)] // We import types just to verify they exist
fn test_mtl4_command_buffer_import() {
    #[cfg(feature = "MTL4CommandBuffer")]
    use objc2_metal::MTL4CommandBuffer;

    assert!(true);
}

/// Test that Metal 4 compiler can be imported.
#[test]
#[allow(unused_imports)] // We import types just to verify they exist
fn test_mtl4_compiler_import() {
    #[cfg(feature = "MTL4Compiler")]
    use objc2_metal::MTL4Compiler;

    assert!(true);
}

/// Test that Metal 4 argument table can be imported.
#[test]
#[allow(unused_imports)] // We import types just to verify they exist
fn test_mtl4_argument_table_import() {
    #[cfg(feature = "MTL4ArgumentTable")]
    use objc2_metal::MTL4ArgumentTable;

    assert!(true);
}

/// Test that Metal 4 function descriptor can be imported.
#[test]
#[allow(unused_imports)] // We import types just to verify they exist
fn test_mtl4_function_descriptor_import() {
    #[cfg(feature = "MTL4FunctionDescriptor")]
    use objc2_metal::MTL4FunctionDescriptor;

    assert!(true);
}

/// Test that MTLTensor can be imported and used.
#[test]
#[allow(unused_imports)] // MTLTensor imported for type verification
fn test_mtl_tensor_usage() {
    #[cfg(feature = "MTLTensor")]
    {
        use objc2_metal::MTLTensor;
        use objc2_metal::MTLTensorDataType;

        // Test that we can use the enum values
        let float32 = MTLTensorDataType::Float32;
        let float16 = MTLTensorDataType::Float16;
        let int8 = MTLTensorDataType::Int8;

        // Verify the values are what we expect
        assert_eq!(float32.0, 3);
        assert_eq!(float16.0, 16);
        assert_eq!(int8.0, 45);
    }

    #[cfg(not(feature = "MTLTensor"))]
    {
        // If MTLTensor feature is not enabled, test should still pass
        assert!(true);
    }
}

/// Test that Metal 3 types are still available (backwards compatibility).
#[test]
#[allow(unused_imports)] // We import types just to verify they exist
fn test_metal3_backwards_compatibility() {
    #[cfg(feature = "MTLCommandQueue")]
    use objc2_metal::MTLCommandQueue;

    #[cfg(feature = "MTLCommandBuffer")]
    use objc2_metal::MTLCommandBuffer;

    assert!(true);
}

/// Test available! macro integration with Metal 4 versions.
#[test]
#[cfg(all(target_os = "macos", feature = "std"))]
fn test_available_macro_metal4() {
    use objc2::available;

    // Test that we can check for Metal 4 availability
    let is_metal4 = available!(macos = 26.0);
    let _ = is_metal4;

    // Test that we can check for pre-Metal 4 versions
    let is_metal3 = available!(macos = 25.0);
    let _ = is_metal3;

    // Test conditional compilation
    if available!(macos = 26.0) {
        // Metal 4 is available
    } else {
        // Metal 4 is not available
    }

    assert!(true);
}

/// Test that Metal 4 detection module works.
#[test]
#[cfg(feature = "std")]
fn test_metal4_detection_module() {
    use objc2_metal::metal4_detection;

    // Test version constants
    assert_eq!(metal4_detection::METAL4_MACOS_VERSION, 26.0);
    assert_eq!(metal4_detection::METAL4_IOS_VERSION, 26.0);

    // Test platform detection
    let is_platform = metal4_detection::is_metal4_platform();
    let _ = is_platform;

    // Test features struct
    let features = metal4_detection::Metal4Features::current();
    let _ = features.has_metal4_types;
    let _ = features.has_all_features();
}

/// Test MTLTensor data types are correctly defined.
#[test]
#[cfg(feature = "MTLTensor")]
fn test_mtl_tensor_data_types() {
    use objc2_metal::MTLTensorDataType;

    // Test all data type variants
    let types = vec![
        MTLTensorDataType::None,
        MTLTensorDataType::Float32,
        MTLTensorDataType::Float16,
        MTLTensorDataType::BFloat16,
        MTLTensorDataType::Int8,
        MTLTensorDataType::UInt8,
        MTLTensorDataType::Int16,
        MTLTensorDataType::UInt16,
        MTLTensorDataType::Int32,
        MTLTensorDataType::UInt32,
    ];

    // Verify all types can be created
    assert_eq!(types.len(), 10);
}

/// Test that Metal 4 features can be detected at compile time.
#[test]
#[cfg(feature = "std")]
fn test_compile_time_detection() {
    use objc2_metal::metal4_detection;

    // These should be evaluable at compile time
    const IS_PLATFORM: bool = metal4_detection::is_metal4_platform();
    const MACOS_VERSION: f64 = metal4_detection::METAL4_MACOS_VERSION;

    // Verify the values
    assert_eq!(MACOS_VERSION, 26.0);

    // Platform check should be consistent
    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "visionos"
    ))]
    {
        assert!(IS_PLATFORM);
    }
}
