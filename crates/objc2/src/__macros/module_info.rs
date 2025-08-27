#[doc(hidden)]
#[macro_export]
#[cfg(target_vendor = "apple")]
macro_rules! __statics_module_info {
    ($hash:expr) => {
        #[link_section = "__TEXT,__cstring,cstring_literals"]
        #[export_name = $crate::__macros::concat!("\x01L_OBJC_CLASS_NAME_", $hash, "_MODULE_INFO")]
        static MODULE_INFO_NAME: [$crate::__macros::u8; 1] = [0];

        /// Emit module info.
        ///
        /// This is similar to image info, and must be present in the final
        /// binary on macOS 32-bit.
        #[link_section = "__OBJC,__module_info,regular,no_dead_strip"]
        #[export_name = $crate::__macros::concat!("\x01L_OBJC_MODULES_", $hash)]
        #[used] // Make sure this reaches the linker
        static _MODULE_INFO: $crate::__macros::ModuleInfo =
            $crate::__macros::ModuleInfo::new(MODULE_INFO_NAME.as_ptr());
    };
}

/// Helper struct for emitting the module info that macOS 32-bit requires.
///
/// <https://github.com/llvm/llvm-project/blob/release/13.x/clang/lib/CodeGen/CGObjCMac.cpp#L5211-L5234>
#[repr(C)]
#[derive(Debug)]
pub struct ModuleInfo {
    version: usize,
    size: usize,
    name: *const u8,
    symtab: *const (),
}

// SAFETY: ModuleInfo is immutable.
unsafe impl Sync for ModuleInfo {}

impl ModuleInfo {
    /// This is hardcoded in clang as 7.
    const VERSION: usize = 7;

    pub const fn new(name: *const u8) -> Self {
        Self {
            version: Self::VERSION,
            size: core::mem::size_of::<Self>(),
            name,
            // We don't expose any symbols
            symtab: core::ptr::null(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(
        not(all(target_vendor = "apple", target_os = "macos", target_arch = "x86")),
        ignore = "Only relevant on macOS 32-bit"
    )]
    fn ensure_size_of_module_info() {
        assert_eq!(core::mem::size_of::<ModuleInfo>(), 16);
    }
}
