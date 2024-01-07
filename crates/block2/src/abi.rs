//! The documentation for these bindings is a mix from GNUStep's and Apple's
//! sources, but the [ABI specification][ABI] is really the place you should
//! be looking!
//!
//! [ABI]: https://clang.llvm.org/docs/Block-ABI-Apple.html
#![allow(
    unreachable_pub,
    unused,
    non_camel_case_types,
    missing_debug_implementations
)]

use core::ffi::c_void;
use std::os::raw::{c_char, c_ulong};

use crate::ffi::Class;

/// Block descriptor flags.
/// Values for Block_layout->flags to describe block objects
#[allow(non_camel_case_types)]
pub type block_flags = i32;

#[cfg(any(doc, feature = "apple"))]
pub const BLOCK_DEALLOCATING: block_flags = 0x0001;

pub const BLOCK_REFCOUNT_MASK: block_flags = if cfg!(feature = "gnustep-1-7") {
    // Mask for the reference count in byref structure's flags field. The low
    // 3 bytes are reserved for the reference count, the top byte for the flags.
    0x00ffffff
} else if cfg!(any(feature = "compiler-rt", feature = "objfw")) {
    0xffff
} else if cfg!(feature = "apple") {
    0xfffe // runtime
} else {
    0
};

#[cfg(any(doc, feature = "apple"))]
/// compiler
pub const BLOCK_INLINE_LAYOUT_STRING: block_flags = 1 << 21;

#[cfg(any(doc, feature = "apple"))]
/// compiler
pub const BLOCK_SMALL_DESCRIPTOR: block_flags = 1 << 22;

#[cfg(any(doc, feature = "apple"))] // Part of ABI?
/// compiler
pub const BLOCK_IS_NOESCAPE: block_flags = 1 << 23;

#[cfg(any(doc, feature = "apple"))]
/// runtime
pub const BLOCK_NEEDS_FREE: block_flags = 1 << 24;

/// The block descriptor contains copy and dispose helpers.
/// compiler
pub const BLOCK_HAS_COPY_DISPOSE: block_flags = 1 << 25;

/// The helpers have C++ code.
/// compiler: helpers have C++ code
pub const BLOCK_HAS_CTOR: block_flags = 1 << 26;

#[cfg(any(doc, feature = "apple"))]
/// compiler
pub const BLOCK_IS_GC: block_flags = 1 << 27;

/// Block is stored in global memory and does not need to be copied.
/// compiler
pub const BLOCK_IS_GLOBAL: block_flags = 1 << 28;

/// Block function uses a calling convention that returns a structure via a
/// pointer passed in by the caller.
///
/// match (BLOCK_USE_STRET, BLOCK_HAS_SIGNATURE) {
///     (false, false) => 10.6.ABI, no signature field available
///     (true, false)  => 10.6.ABI, no signature field available
///     (false, true)  => ABI.2010.3.16, regular calling convention, presence of signature field
///     (true, true)   => ABI.2010.3.16, stret calling convention, presence of signature field,
/// }
///
/// See <https://clang.llvm.org/docs/Block-ABI-Apple.html#high-level>
#[doc(alias = "BLOCK_USE_SRET")]
#[doc(alias = "BLOCK_HAS_DESCRIPTOR")] // compiler-rt || macOS 10.6
pub const BLOCK_USE_STRET: block_flags = 1 << 29;

/// Block has an Objective-C type encoding.
/// compiler
pub const BLOCK_HAS_SIGNATURE: block_flags = 1 << 30;

#[cfg(any(doc, feature = "apple"))]
/// compiler
pub const BLOCK_HAS_EXTENDED_LAYOUT: block_flags = 1 << 31;

/// Flags used in the final argument to _Block_object_assign() and
/// _Block_object_dispose().  These indicate the type of copy or dispose to
/// perform.
/// Values for _Block_object_assign() and _Block_object_dispose() parameters
///
/// This is a helper type, in the sources this type does not have a name!
#[allow(non_camel_case_types)]
pub type block_assign_dispose_flags = i32;

/// The value is of some id-like type, and should be copied as an Objective-C
/// object: i.e. by sending -retain or via the GC assign functions in GC mode
/// (not yet supported).
///
/// id, NSObject, __attribute__((NSObject)), block, ...
pub const BLOCK_FIELD_IS_OBJECT: block_assign_dispose_flags = 3;

/// The field is a block.  This must be copied by the block copy functions.
///
/// a block variable
pub const BLOCK_FIELD_IS_BLOCK: block_assign_dispose_flags = 7;

/// The field is an indirect reference to a variable declared with the __block
/// storage qualifier.
///
/// the on stack structure holding the __block variable
pub const BLOCK_FIELD_IS_BYREF: block_assign_dispose_flags = 8;

/// The field is an indirect reference to a variable declared with the __block
/// storage qualifier.
///
/// declared __weak, only used in byref copy helpers
pub const BLOCK_FIELD_IS_WEAK: block_assign_dispose_flags = 16;

/// The field is an indirect reference to a variable declared with the __block
/// storage qualifier.
///
/// called from __block (byref) copy/dispose support routines.
pub const BLOCK_BYREF_CALLER: block_assign_dispose_flags = 128;

#[cfg(any(doc, feature = "apple"))]
pub const BLOCK_ALL_COPY_DISPOSE_FLAGS: block_assign_dispose_flags = BLOCK_FIELD_IS_OBJECT
    | BLOCK_FIELD_IS_BLOCK
    | BLOCK_FIELD_IS_BYREF
    | BLOCK_FIELD_IS_WEAK
    | BLOCK_BYREF_CALLER;

// TODO: BLOCK_LAYOUT_X

/// The expected layout of every block.
#[repr(C)]
#[doc(alias = "__block_literal")]
pub struct Block_layout {
    /// Class pointer. Always initialised to &_NSConcreteStackBlock for blocks
    /// that are created on the stack or &_NSConcreteGlobalBlock for blocks
    /// that are created in global storage.
    pub isa: *const Class,
    /// Flags.
    /// See the `block_flags` enumerated type for possible values.
    /// Contains ref count in Apple and ObjFW.
    pub flags: block_flags,
    /// Reserved - always initialised to 0 by the compiler (but this is not
    /// said in the specification).
    ///
    /// Used for the reference count in GNUStep and WinObjC.
    pub reserved: i32,
    /// The function that implements the block.  The first argument is this
    /// structure, the subsequent arguments are the block's explicit
    /// parameters. If the BLOCK_USE_SRET & BLOCK_HAS_SIGNATURE flag is set,
    /// there is an additional hidden argument, which is a pointer to the
    /// space on the stack allocated to hold the return value.
    pub invoke: Option<unsafe extern "C" fn()>,
    /// The block's descriptor. The actual type of this is:
    /// ```pseudo-code
    /// match (BLOCK_HAS_COPY_DISPOSE, BLOCK_HAS_SIGNATURE) {
    ///     (false, false) => Block_descriptor_header,
    ///     (true, false) => Block_descriptor,
    ///     (false, true) => Block_descriptor_basic,
    ///     (true, true) => Block_descriptor_with_signature,
    /// }
    /// ```
    ///
    /// But since all of these start with `Block_descriptor_header`, it is
    /// always safe to reinterpret this pointer as that.
    // Note: Important to use `*const c_void` until we know which type it is!
    pub descriptor: *const c_void,
}

#[repr(C)]
#[allow(missing_copy_implementations)]
#[doc(alias = "__block_descriptor")]
#[doc(alias = "Block_descriptor_1")]
pub struct Block_descriptor_header {
    /// Reserved for future use. Currently always 0.
    pub reserved: c_ulong, // usize
    /// Size of the block.
    pub size: c_ulong, // usize
}

/// Block descriptor that contains copy and dispose operations.
///
/// Requires BLOCK_HAS_COPY_DISPOSE
#[repr(C)]
#[doc(alias = "Block_descriptor_2")]
pub struct Block_descriptor {
    pub header: Block_descriptor_header,

    /// Copy function, generated by the compiler to help copy the block if it
    /// contains nontrivial copy operations.
    pub copy: Option<unsafe extern "C" fn(dst: *mut c_void, src: *mut c_void)>,
    /// Dispose function, generated by the compiler to help copy the block if
    /// it contains nontrivial destructors.
    pub dispose: Option<unsafe extern "C" fn(src: *mut c_void)>,
}

/// Extended block descriptor that does not contain copy and dispose helper
/// functions.
///
/// Requires BLOCK_HAS_SIGNATURE
#[repr(C)]
#[doc(alias = "Block_descriptor_3")]
pub struct Block_descriptor_basic {
    pub header: Block_descriptor_header,

    /// Objective-C type encoding of the block.
    #[doc(alias = "signature")]
    pub encoding: *const c_char,
}

/// Requires BLOCK_HAS_COPY_DISPOSE and BLOCK_HAS_SIGNATURE
#[repr(C)]
#[doc(alias = "Block_descriptor_2")]
#[doc(alias = "Block_descriptor_3")]
pub struct Block_descriptor_with_signature {
    pub header: Block_descriptor_header,

    /// Same as [`Block_descriptor::copy`].
    pub copy: Option<unsafe extern "C" fn(dst: *mut c_void, src: *mut c_void)>,
    /// Same as [`Block_descriptor::dispose`].
    pub dispose: Option<unsafe extern "C" fn(src: *mut c_void)>,

    /// Objective-C type encoding of the block.
    #[doc(alias = "signature")]
    pub encoding: *const c_char,
}

// #[cfg(any(doc, feature = "apple"))]
// pub layout: *const c_char,

// #[repr(C)]
// pub struct Block_descriptor_small {
//     pub size: u32,
//     pub signature: i32,
//     pub layout: i32,
//     pub copy: i32,
//     pub dispose: i32,
// }

// #[repr(C)]
// pub struct Block_basic {
//     pub isa: *mut Class,
//     pub Block_flags: i32,
//     pub Block_size: i32,
//     pub Block_invoke: Option<unsafe extern "C" fn()>,
//     pub Block_copy: Option<unsafe extern "C" fn(dst: *mut c_void, src: *mut c_void)>,
//     pub Block_dispose: Option<unsafe extern "C" fn(block: *mut c_void)>,
// }
// Example usage: https://github.com/apple-oss-distributions/libdispatch/blob/libdispatch-84.5/src/once.c

/// Structure used for on-stack variables that are referenced by blocks.
#[repr(C)]
#[doc(alias = "Block_byref_1")]
#[doc(alias = "_block_byref")]
pub struct Block_byref_header {
    /// Class pointer. Currently unused on GNUStep and always NULL. Could be
    /// used in the future to support introspection.
    pub isa: *const Class,
    /// The pointer to the structure that contains the real version of the
    /// data. All accesses go via this pointer. If an on-stack byref structure
    /// is copied to the heap, then its forwarding pointer should point to the
    /// heap version. Otherwise it should point to itself.
    pub forwarding: *mut Block_byref_header,
    /// Flags and reference count.
    ///
    /// TODO: Volatile!
    pub flags: block_flags,
    #[cfg(feature = "apple")]
    /// Size of this structure.
    pub size: u32,
    #[cfg(not(feature = "apple"))]
    /// Size of this structure.
    pub size: i32,
}

/// Structure used for on-stack variables that are referenced by blocks.
///
/// requires BLOCK_BYREF_HAS_COPY_DISPOSE
#[repr(C)]
#[doc(alias = "Block_byref_2")]
pub struct Block_byref {
    pub header: Block_byref_header,
    /// Copy function.
    pub keep: Option<unsafe extern "C" fn(dst: *mut c_void, src: *mut c_void)>,
    /// Dispose function.
    pub destroy: Option<unsafe extern "C" fn(src: *mut c_void)>,
}

#[cfg(any(doc, feature = "apple"))]
/// Structure used for on-stack variables that are referenced by blocks.
///
/// requires BLOCK_BYREF_LAYOUT_EXTENDED
#[repr(C)]
#[doc(alias = "Block_byref_3")]
pub struct Block_byref_extended {
    pub header: Block_byref_header,
    /// Same as [`Block_byref::keep`].
    pub keep: Option<unsafe extern "C" fn(dst: *mut c_void, src: *mut c_void)>,
    /// Same as [`Block_byref::destroy`].
    pub destroy: Option<unsafe extern "C" fn(src: *mut c_void)>,
    pub layout: *const c_char,
}
