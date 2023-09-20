#[cfg(all(debug_assertions, feature = "verify"))]
use alloc::vec::Vec;
#[cfg(all(debug_assertions, feature = "verify"))]
use std::collections::HashSet;

use crate::declare::ClassBuilder;
#[cfg(all(debug_assertions, feature = "verify"))]
use crate::runtime::MethodDescription;
use crate::runtime::{AnyClass, AnyProtocol, MethodImplementation, Sel};
use crate::Message;

pub use core::borrow::{Borrow, BorrowMut};
pub use core::cell::UnsafeCell;
pub use core::convert::{AsMut, AsRef};
pub use core::marker::{PhantomData, Sized};
pub use core::mem::{needs_drop, size_of, ManuallyDrop};
pub use core::ops::{Deref, DerefMut};
pub use core::option::Option::{self, None, Some};
pub use core::primitive::{bool, str, u8};
pub use core::ptr::drop_in_place;
pub use core::{compile_error, concat, panic, stringify};
// TODO: Use `core::cell::LazyCell`
pub use std::sync::Once;

mod cache;
mod common_selectors;
mod convert;
mod declare_class;
mod method_family;
mod msg_send_id;
mod writeback;

pub use self::cache::{CachedClass, CachedSel};
pub use self::common_selectors::{alloc_sel, dealloc_sel, init_sel, new_sel};
pub use self::convert::{ConvertArgument, ConvertReturn};
pub use self::declare_class::{
    assert_mutability_matches_superclass_mutability, IdReturnValue, MaybeOptionId,
    MessageRecieveId, ValidSubclassMutability,
};
pub use self::method_family::{
    retain_semantics, Alloc, CopyOrMutCopy, Init, New, Other, RetainSemantics,
};
pub use self::msg_send_id::{MaybeUnwrap, MsgSendId};

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

impl ClassBuilder {
    #[doc(hidden)]
    pub fn __add_protocol_methods<'a, 'b>(
        &'a mut self,
        protocol: Option<&'b AnyProtocol>,
    ) -> ClassProtocolMethodsBuilder<'a, 'b> {
        if let Some(protocol) = protocol {
            self.add_protocol(protocol);
        }

        #[cfg(all(debug_assertions, feature = "verify"))]
        {
            ClassProtocolMethodsBuilder {
                builder: self,
                protocol,
                required_instance_methods: protocol
                    .map(|p| p.method_descriptions(true))
                    .unwrap_or_default(),
                optional_instance_methods: protocol
                    .map(|p| p.method_descriptions(false))
                    .unwrap_or_default(),
                registered_instance_methods: HashSet::new(),
                required_class_methods: protocol
                    .map(|p| p.class_method_descriptions(true))
                    .unwrap_or_default(),
                optional_class_methods: protocol
                    .map(|p| p.class_method_descriptions(false))
                    .unwrap_or_default(),
                registered_class_methods: HashSet::new(),
            }
        }

        #[cfg(not(all(debug_assertions, feature = "verify")))]
        {
            ClassProtocolMethodsBuilder {
                builder: self,
                protocol,
            }
        }
    }
}

/// Helper for ensuring that:
/// - Only methods on the protocol are overriden.
/// - TODO: The methods have the correct signature.
/// - All required methods are overridden.
#[derive(Debug)]
pub struct ClassProtocolMethodsBuilder<'a, 'b> {
    builder: &'a mut ClassBuilder,
    #[allow(unused)]
    protocol: Option<&'b AnyProtocol>,
    #[cfg(all(debug_assertions, feature = "verify"))]
    required_instance_methods: Vec<MethodDescription>,
    #[cfg(all(debug_assertions, feature = "verify"))]
    optional_instance_methods: Vec<MethodDescription>,
    #[cfg(all(debug_assertions, feature = "verify"))]
    registered_instance_methods: HashSet<Sel>,
    #[cfg(all(debug_assertions, feature = "verify"))]
    required_class_methods: Vec<MethodDescription>,
    #[cfg(all(debug_assertions, feature = "verify"))]
    optional_class_methods: Vec<MethodDescription>,
    #[cfg(all(debug_assertions, feature = "verify"))]
    registered_class_methods: HashSet<Sel>,
}

impl ClassProtocolMethodsBuilder<'_, '_> {
    #[inline]
    pub unsafe fn add_method<T, F>(&mut self, sel: Sel, func: F)
    where
        T: Message + ?Sized,
        F: MethodImplementation<Callee = T>,
    {
        #[cfg(all(debug_assertions, feature = "verify"))]
        if let Some(protocol) = self.protocol {
            let _types = self
                .required_instance_methods
                .iter()
                .chain(&self.optional_instance_methods)
                .find(|desc| desc.sel == sel)
                .map(|desc| desc.types)
                .unwrap_or_else(|| {
                    panic!(
                        "failed overriding protocol method -[{protocol} {sel}]: method not found"
                    )
                });
        }

        // SAFETY: Checked by caller
        unsafe { self.builder.add_method(sel, func) };

        #[cfg(all(debug_assertions, feature = "verify"))]
        if !self.registered_instance_methods.insert(sel) {
            unreachable!("already added")
        }
    }

    #[inline]
    pub unsafe fn add_class_method<F>(&mut self, sel: Sel, func: F)
    where
        F: MethodImplementation<Callee = AnyClass>,
    {
        #[cfg(all(debug_assertions, feature = "verify"))]
        if let Some(protocol) = self.protocol {
            let _types = self
                .required_class_methods
                .iter()
                .chain(&self.optional_class_methods)
                .find(|desc| desc.sel == sel)
                .map(|desc| desc.types)
                .unwrap_or_else(|| {
                    panic!(
                        "failed overriding protocol method +[{protocol} {sel}]: method not found"
                    )
                });
        }

        // SAFETY: Checked by caller
        unsafe { self.builder.add_class_method(sel, func) };

        #[cfg(all(debug_assertions, feature = "verify"))]
        if !self.registered_class_methods.insert(sel) {
            unreachable!("already added")
        }
    }

    #[cfg(all(debug_assertions, feature = "verify"))]
    pub fn __finish(self) {
        let superclass = self.builder.superclass();

        if let Some(protocol) = self.protocol {
            for desc in &self.required_instance_methods {
                if self.registered_instance_methods.contains(&desc.sel) {
                    continue;
                }

                // TODO: Don't do this when `NS_PROTOCOL_REQUIRES_EXPLICIT_IMPLEMENTATION`
                if superclass
                    .and_then(|superclass| superclass.instance_method(desc.sel))
                    .is_some()
                {
                    continue;
                }

                panic!(
                    "must implement required protocol method -[{protocol} {}]",
                    desc.sel
                )
            }
        }

        if let Some(protocol) = self.protocol {
            for desc in &self.required_class_methods {
                if self.registered_class_methods.contains(&desc.sel) {
                    continue;
                }

                // TODO: Don't do this when `NS_PROTOCOL_REQUIRES_EXPLICIT_IMPLEMENTATION`
                if superclass
                    .and_then(|superclass| superclass.class_method(desc.sel))
                    .is_some()
                {
                    continue;
                }

                panic!(
                    "must implement required protocol method +[{protocol} {}]",
                    desc.sel
                );
            }
        }
    }

    #[inline]
    #[cfg(not(all(debug_assertions, feature = "verify")))]
    pub fn __finish(self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "objc2-proc-macros")]
    use crate::__hash_idents;

    #[test]
    #[cfg(feature = "objc2-proc-macros")]
    fn hash_idents_different() {
        assert_ne!(__hash_idents!(abc), __hash_idents!(def));
    }

    #[test]
    #[cfg(feature = "objc2-proc-macros")]
    fn hash_idents_same_no_equal() {
        assert_ne!(__hash_idents!(abc), __hash_idents!(abc));
        assert_ne!(__hash_idents!(abc def ghi), __hash_idents!(abc def ghi));
    }

    #[test]
    #[cfg(feature = "objc2-proc-macros")]
    fn hash_idents_exact_same_ident() {
        macro_rules! x {
            ($x:ident) => {
                (__hash_idents!($x), __hash_idents!($x))
            };
        }
        let (ident1, ident2) = x!(abc);
        // This is a limitation of `__hash_idents`, ideally we'd like these
        // to be different!
        assert_eq!(ident1, ident2);
    }

    #[test]
    #[cfg_attr(
        not(all(feature = "apple", target_os = "macos", target_arch = "x86")),
        ignore = "Only relevant on macOS 32-bit"
    )]
    fn ensure_size_of_module_info() {
        assert_eq!(core::mem::size_of::<ModuleInfo>(), 16);
    }
}
