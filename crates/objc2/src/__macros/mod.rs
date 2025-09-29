//! Declarative macros in `objc2`.
//!
//! These are... quite complex, especially the larger ones. Some of this
//! complexity is inherent, the function rewriting that we have do in
//! `declare_class!` for example _is_ weird and special, but a lot of it also
//! just comes from not using procedural macros (which is a requirement for
//! keeping compile-times down, and to reduce the number of things we run at
//! build-time).
//!
//! Recommended resources (we use all of the patterns described therein):
//! - [The Little Book of Rust Macros](https://lukaswirth.dev/tlborm/introduction.html)
//! - [Anything you can do, I can do worse with macro_rules!](https://www.youtube.com/watch?v=7uSM60jlkBE)
//! - [Aurorans Solis' Blog](https://auroranssolis.github.io/)

mod attribute_helpers;
mod available;
mod cf_objc2_type;
mod class;
mod convert;
pub(crate) mod define_class;
mod extern_class;
mod extern_conformance;
mod extern_methods;
mod extern_protocol;
mod fallback;
mod hash_idents;
mod image_info;
mod method_family;
mod method_msg_send;
mod module_info;
mod msg_send;
mod retain_semantics;
mod rewrite_self_param;
mod sel;
mod static_helpers;
mod sync_unsafe_cell;
mod writeback;

pub use core::borrow::Borrow;
pub use core::cell::UnsafeCell;
pub use core::cmp::{Eq, PartialEq};
pub use core::convert::AsRef;
pub use core::default::Default;
pub use core::ffi::CStr;
pub use core::fmt;
pub use core::hash::{Hash, Hasher};
pub use core::marker::{PhantomData, Sized};
pub use core::mem::{size_of, transmute, ManuallyDrop, MaybeUninit};
pub use core::ops::Deref;
pub use core::option::Option::{self, None, Some};
pub use core::primitive::{bool, isize, str, u8};
pub use core::{compile_error, concat, env, module_path, panic, stringify};
pub use std::sync::Once;

#[cfg(feature = "unstable-darwin-objc")]
pub use core::os::darwin::objc as core_darwin_objc;

pub use self::available::{is_available, AvailableVersion, OSVersion};
pub use self::class::{disallow_in_static, CachedClass};
pub use self::convert::{ConvertArgument, ConvertArguments, ConvertReturn, TupleExtender};
pub use self::define_class::{
    class_c_name, define_class, ClassBuilderHelper, ClassProtocolMethodsBuilder,
    DefinedIvarsHelper, MaybeOptionRetained, MessageReceiveRetained, RetainedReturnValue,
    ThreadKindAutoTraits,
};
pub use self::extern_class::{DoesNotImplDrop, MainThreadOnlyDoesNotImplSendSync, ValidThreadKind};
#[allow(deprecated)]
pub use self::extern_methods::extern_methods_unsafe_impl;
#[cfg(any(feature = "unstable-static-sel", feature = "unstable-static-class"))]
pub use self::hash_idents::proc_macro_hash_idents;
pub use self::image_info::ImageInfo;
pub use self::method_family::{
    method_family, method_family_import, AllocFamily, AutoreleaseSelector, CopyFamily,
    DeallocSelector, InitFamily, MethodFamily, MutableCopyFamily, NewFamily, NoneFamily,
    ReleaseSelector, RetainSelector,
};
pub use self::module_info::ModuleInfo;
pub use self::msg_send::{MsgSend, MsgSendError, MsgSendSuper, MsgSendSuperError};
pub use self::retain_semantics::{
    KindDefined, KindSendMessage, KindSendMessageSuper, RetainSemantics,
};
pub use self::sel::{alloc_sel, dealloc_sel, init_sel, new_sel, CachedSel};
pub use self::sync_unsafe_cell::SyncUnsafeCell;
