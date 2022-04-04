use alloc::format;
use core::ffi::c_void;
use core::fmt::{Debug, Error, Formatter};
use std::ffi::CStr;

use crate::{ffi, Block};

impl<A, R> Debug for Block<A, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut f = f.debug_struct("Block");

        let layout: &ffi::Block_layout =
            unsafe { &*(self as *const Self as *const ffi::Block_layout) };

        f.field("isa", debug_isa(&layout.isa));
        f.field("flags", &BlockFlags(layout.flags));
        f.field("reserved", &layout.reserved);
        f.field("invoke", &layout.invoke);
        f.field(
            "descriptor",
            &BlockDescriptor {
                has_copy_dispose: layout.flags & ffi::BLOCK_HAS_COPY_DISPOSE != 0,
                has_signature: layout.flags & ffi::BLOCK_HAS_SIGNATURE != 0,
                descriptor: layout.descriptor.cast(),
            },
        );
        f.finish_non_exhaustive()
    }
}

fn debug_isa(isa: &*mut ffi::Class) -> &dyn Debug {
    if unsafe { &ffi::_NSConcreteGlobalBlock } as *const ffi::Class == *isa {
        &"Global"
    } else if unsafe { &ffi::_NSConcreteStackBlock } as *const ffi::Class == *isa {
        &"Stack"
    } else if unsafe { &ffi::_NSConcreteMallocBlock } as *const ffi::Class == *isa {
        &"Malloc"
    } else {
        isa
    }
}

#[derive(Clone, Copy, PartialEq)]
struct BlockFlags(ffi::block_flags);

impl Debug for BlockFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut f = f.debug_struct("BlockFlags");
        f.field("value", &format!("{:032b}", self.0));

        macro_rules! test_flags {
            ($($name:ident: $flag:ident);* $(;)?) => ($(
                f.field(stringify!($name), &(self.0 & ffi::$flag != 0));
            )*)
        }
        test_flags! {
            deallocating: BLOCK_DEALLOCATING;
            inline_layout_string: BLOCK_INLINE_LAYOUT_STRING;
            small_descriptor: BLOCK_SMALL_DESCRIPTOR;
            is_noescape: BLOCK_IS_NOESCAPE;
            needs_free: BLOCK_NEEDS_FREE;
            has_copy_dispose: BLOCK_HAS_COPY_DISPOSE;
            has_ctor: BLOCK_HAS_CTOR;
            is_gc: BLOCK_IS_GC;
            is_global: BLOCK_IS_GLOBAL;
            use_stret: BLOCK_USE_STRET;
            has_signature: BLOCK_HAS_SIGNATURE;
            has_extended_layout: BLOCK_HAS_EXTENDED_LAYOUT;
        }

        f.field(
            "over_referenced",
            &(self.0 & ffi::BLOCK_REFCOUNT_MASK == ffi::BLOCK_REFCOUNT_MASK),
        );
        f.field(
            "reference_count",
            &((self.0 & ffi::BLOCK_REFCOUNT_MASK) >> 1),
        );
        f.finish_non_exhaustive()
    }
}

#[derive(Clone, Copy, PartialEq)]
struct BlockDescriptor {
    has_copy_dispose: bool,
    has_signature: bool,
    descriptor: *mut c_void,
}

impl Debug for BlockDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.descriptor.is_null() {
            return f.write_str("(null)");
        }

        let mut f = f.debug_struct("BlockDescriptor");

        let header = unsafe { &*(self.descriptor as *mut ffi::Block_descriptor_header) };

        f.field("reserved", &header.reserved);
        f.field("size", &header.size);

        match (self.has_copy_dispose, self.has_signature) {
            (false, false) => {}
            (true, false) => {
                let descriptor = unsafe { &*(self.descriptor as *mut ffi::Block_descriptor) };
                f.field("copy", &descriptor.copy);
                f.field("dispose", &descriptor.dispose);
            }
            (false, true) => {
                let descriptor = unsafe { &*(self.descriptor as *mut ffi::Block_descriptor_basic) };
                f.field(
                    "encoding",
                    &if descriptor.encoding.is_null() {
                        None
                    } else {
                        Some(unsafe { CStr::from_ptr(descriptor.encoding) })
                    },
                );
            }
            (true, true) => {
                let descriptor =
                    unsafe { &*(self.descriptor as *mut ffi::Block_descriptor_with_signature) };
                f.field("copy", &descriptor.inner.copy);
                f.field("dispose", &descriptor.inner.dispose);
                f.field(
                    "encoding",
                    &if descriptor.encoding.is_null() {
                        None
                    } else {
                        Some(unsafe { CStr::from_ptr(descriptor.encoding) })
                    },
                );
            }
        }

        f.finish()
    }
}
