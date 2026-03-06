//! Metal 4 detection utilities.
//!
//! This module provides utilities for detecting Metal 4 availability.
//!
//! # Metal 4 Requirements
//!
//! Metal 4 was introduced at WWDC 2025 and requires specific OS versions:
//!
//! - macOS 26.0+ (Tahoe)
//! - iOS 26.0+
//! - iPadOS 26.0+
//! - tvOS 26.0+
//! - visionOS 26.0+
//!
//! # Usage Pattern
//!
//! To use Metal 4 APIs safely, you should check for availability at runtime
//! using the [`available!`] macro from the `objc2` crate:
//!
//! ```no_run
//! use objc2::available;
//!
//! if available!(macos = 26.0, ios = 26.0, tvos = 26.0, visionos = 26.0) {
//!     // Use Metal 4 features like MTL4CommandQueue, MTL4FXFrameInterpolator, etc.
//! } else {
//!     // Fall back to earlier Metal versions
//! }
//! ```

/// Metal 4 OS version requirements.
///
/// These constants define the minimum OS versions required for Metal 4 support.
/// Use these with the [`available!`] macro to check for Metal 4 availability.
///
///
/// # Examples
///
/// ```no_run
/// use objc2_metal::metal4_detection::METAL4_MACOS_VERSION;
///
/// assert_eq!(METAL4_MACOS_VERSION, 26.0);
/// ```

pub const METAL4_MACOS_VERSION: f64 = 26.0;
pub const METAL4_IOS_VERSION: f64 = 26.0;
pub const METAL4_TVOS_VERSION: f64 = 26.0;
pub const METAL4_VISIONOS_VERSION: f64 = 26.0;

/// Runtime check for Metal 4 availability on the current OS.
///
/// This uses objc2's [`available!`] macro and should be preferred over
/// platform-only checks when gating Metal 4 API usage.
#[must_use]
#[inline]
pub fn is_metal4_available() -> bool {
    objc2::available!(macos = 26.0, ios = 26.0, tvos = 26.0, visionos = 26.0)
}

/// Check if the current platform is likely to support Metal 4.
///
/// This is a **compile-time only** check that returns `true` if the target
/// platform is one of the platforms that support Metal 4. This does **not**
/// check the actual OS version at runtime.
///
/// # Returns
///
/// `true` if the target platform is macOS, iOS, tvOS, or visionOS.
///
///
/// # Examples
///
/// ```no_run
/// use objc2_metal::metal4_detection::is_metal4_platform;
///
/// if is_metal4_platform() {
///     println!("Running on a platform that supports Metal 4");
/// }
/// ```
///
/// # Note
///
/// This function only checks the target platform, not the actual OS version.
/// For runtime version checking, use the [`available!`] macro instead.
#[must_use]
#[inline]
pub const fn is_metal4_platform() -> bool {
    cfg!(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "visionos"
    ))
}

/// Metal 4 feature set information.
///
/// This struct provides information about which Metal 4 features are
/// supported in the current SDK.
///
///
/// # Examples
///
/// ```no_run
/// use objc2_metal::metal4_detection::Metal4Features;
///
/// let features = Metal4Features::current();
/// println!("Metal 4 types available: {}", features.has_metal4_types);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Metal4Features {
    /// Whether Metal 4 types are available in the SDK
    pub has_metal4_types: bool,
    /// Whether the platform supports Metal 4
    pub is_supported_platform: bool,
}

impl Metal4Features {
    /// Get information about Metal 4 features in the current SDK.
    ///
    /// This function provides compile-time information about which Metal 4
    /// features are available in the current SDK and target platform.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use objc2_metal::metal4_detection::Metal4Features;
    ///
    /// let features = Metal4Features::current();
    /// if features.has_metal4_types && features.is_supported_platform {
    ///     println!("Metal 4 is available in the current SDK");
    /// }
    /// ```
    #[must_use]
    #[inline]
    pub const fn current() -> Self {
        Self {
            // Metal 4 types are available if the corresponding features are enabled
            has_metal4_types: cfg!(any(
                feature = "MTL4CommandQueue",
                feature = "MTL4CommandBuffer",
                feature = "MTL4Compiler"
            )),
            // Platform support check
            is_supported_platform: is_metal4_platform(),
        }
    }

    /// Check if all Metal 4 features are available.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use objc2_metal::metal4_detection::Metal4Features;
    ///
    /// let features = Metal4Features::current();
    /// if features.has_all_features() {
    ///     println!("All Metal 4 features are available!");
    /// }
    /// ```
    #[must_use]
    #[inline]
    pub const fn has_all_features(&self) -> bool {
        self.has_metal4_types && self.is_supported_platform
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_constants() {
        assert_eq!(METAL4_MACOS_VERSION, 26.0);
        assert_eq!(METAL4_IOS_VERSION, 26.0);
        assert_eq!(METAL4_TVOS_VERSION, 26.0);
        assert_eq!(METAL4_VISIONOS_VERSION, 26.0);
    }

    #[test]
    fn test_platform_check() {
        // This will be true or false depending on the compilation target
        let _ = is_metal4_platform();
    }

    #[test]
    fn test_runtime_check() {
        let _ = is_metal4_available();
    }

    #[test]
    fn test_features_struct() {
        let features = Metal4Features::current();
        // Just verify the struct can be created and accessed
        let _ = features.has_metal4_types;
        let _ = features.is_supported_platform;
        let _ = features.has_all_features();
    }
}
