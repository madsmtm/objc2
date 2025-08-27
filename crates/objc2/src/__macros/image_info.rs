#[doc(hidden)]
#[macro_export]
#[cfg(target_vendor = "apple")]
macro_rules! __statics_image_info {
    ($hash:expr) => {
        /// We always emit the image info tag, since we need it to:
        /// - End up in the same codegen unit as the other statics below.
        /// - End up in the final binary so it can be read by dyld.
        ///
        /// If it's not present in the codegen unit, then `ld64` won't set
        /// `hasObjC` for that specific object file, and in turn it might
        /// disable processing of the special Objective-C sections (currently
        /// a category merging pass, in the future who knows what).
        ///
        /// Unfortunately however, this leads to duplicated tags - the linker
        /// reports `__DATA/__objc_imageinfo has unexpectedly large size XXX`,
        /// but things still seems to work.
        #[cfg_attr(
            not(all(target_os = "macos", target_arch = "x86")),
            link_section = "__DATA,__objc_imageinfo,regular,no_dead_strip"
        )]
        #[cfg_attr(
            all(target_os = "macos", target_arch = "x86"),
            link_section = "__OBJC,__image_info,regular"
        )]
        #[export_name = $crate::__macros::concat!("\x01L_OBJC_IMAGE_INFO_", $hash)]
        #[used] // Make sure this reaches the linker
        static _IMAGE_INFO: $crate::__macros::ImageInfo = $crate::__macros::ImageInfo::system();
    };
}

#[repr(C)]
#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct ImageInfo {
    // These are not actually `unsigned int`, even though the docs say so
    /// The version of the image info struct.
    version: u32,
    flags: u32,
}

#[allow(unused)]
impl ImageInfo {
    /// Unused
    const FIX_AND_CONTINUE: u32 = 1 << 0;
    const SUPPORTS_GARBAGE_COLLECTED: u32 = 1 << 1;
    const REQUIRES_GARBAGE_COLLECTION: u32 = 1 << 2;
    const OPTIMIZED_BY_DYLD: u32 = 1 << 3; // TODO
    /// Unused
    const CORRECTED_SYNTHESIZE: u32 = 1 << 4;
    /// Whether we're compiling this to run on a simulator.
    const IMAGE_IS_SIMULATED: u32 = 1 << 5;
    /// Whether we are generating class properties.
    const CLASS_PROPERTIES: u32 = 1 << 6;
    const DYLD_PREOPTIMIZED: u32 = 1 << 7;

    const SWIFT_ABI_VERSION_SHIFT: u32 = 8;
    const SWIFT_ABI_VERSION_MASK: u32 = 0xff << Self::SWIFT_ABI_VERSION_SHIFT;
    const SWIFT_MINOR_VERSION_SHIFT: u32 = 16;
    const SWIFT_MINOR_VERSION_MASK: u32 = 0xff << Self::SWIFT_MINOR_VERSION_SHIFT;
    const SWIFT_MAJOR_VERSION_SHIFT: u32 = 24;
    const SWIFT_MAJOR_VERSION_MASK: u32 = 0xff << Self::SWIFT_MAJOR_VERSION_SHIFT;

    /// Fetches the image info for the current runtime + target combination
    #[inline]
    pub const fn system() -> Self {
        // We don't currently do anything relating to class properties, but
        // let's mimic what Clang does!
        let mut flags = Self::CLASS_PROPERTIES;

        if cfg!(target_simulator) {
            flags |= Self::IMAGE_IS_SIMULATED;
        }

        Self { version: 0, flags }
    }
}
