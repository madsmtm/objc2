use core::ffi::c_void;
use core::fmt::{Debug, DebugStruct, Error, Formatter};
use core::ptr;
use std::ffi::CStr;

use crate::abi::{
    BlockDescriptor, BlockDescriptorCopyDispose, BlockDescriptorCopyDisposeSignature,
    BlockDescriptorSignature, BlockFlags, BlockLayout,
};
use crate::ffi;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Isa(*const ffi::Class);

impl Isa {
    fn is_global(self) -> bool {
        ptr::eq(
            unsafe { ptr::addr_of!(ffi::_NSConcreteGlobalBlock) },
            self.0,
        )
    }

    fn is_stack(self) -> bool {
        ptr::eq(unsafe { ptr::addr_of!(ffi::_NSConcreteStackBlock) }, self.0)
    }
}

impl Debug for Isa {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.is_global() {
            f.write_str("_NSConcreteGlobalBlock")
        } else if self.is_stack() {
            f.write_str("_NSConcreteStackBlock")
        } else {
            write!(f, "{:?} (likely _NSConcreteMallocBlock)", self.0)
        }
    }
}

pub(crate) fn debug_block_layout(layout: &BlockLayout, f: &mut DebugStruct<'_, '_>) {
    f.field("isa", &Isa(layout.isa));
    f.field("flags", &layout.flags);
    f.field("reserved", &layout.reserved);
    f.field("invoke", &layout.invoke);
    f.field(
        "descriptor",
        &BlockDescriptorHelper {
            has_copy_dispose: layout.flags.0 & BlockFlags::BLOCK_HAS_COPY_DISPOSE.0 != 0,
            has_signature: layout.flags.0 & BlockFlags::BLOCK_HAS_SIGNATURE.0 != 0,
            descriptor: layout.descriptor,
        },
    );
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct BlockDescriptorHelper {
    has_copy_dispose: bool,
    has_signature: bool,
    descriptor: *const c_void,
}

impl Debug for BlockDescriptorHelper {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.descriptor.is_null() {
            return f.write_str("(null)");
        }

        let mut f = f.debug_struct("BlockDescriptor");

        let header = unsafe { self.descriptor.cast::<BlockDescriptor>().as_ref().unwrap() };

        f.field("reserved", &header.reserved);
        f.field("size", &header.size);

        match (self.has_copy_dispose, self.has_signature) {
            (false, false) => {}
            (true, false) => {
                let descriptor = unsafe {
                    self.descriptor
                        .cast::<BlockDescriptorCopyDispose>()
                        .as_ref()
                        .unwrap()
                };
                f.field("copy", &descriptor.copy);
                f.field("dispose", &descriptor.dispose);
            }
            (false, true) => {
                let descriptor = unsafe {
                    self.descriptor
                        .cast::<BlockDescriptorSignature>()
                        .as_ref()
                        .unwrap()
                };
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
                let descriptor = unsafe {
                    self.descriptor
                        .cast::<BlockDescriptorCopyDisposeSignature>()
                        .as_ref()
                        .unwrap()
                };
                f.field("copy", &descriptor.copy);
                f.field("dispose", &descriptor.dispose);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isa() {
        let isa = Isa(unsafe { ptr::addr_of!(ffi::_NSConcreteGlobalBlock) });
        assert!(isa.is_global());
        assert!(!isa.is_stack());
        let isa = Isa(unsafe { ptr::addr_of!(ffi::_NSConcreteStackBlock) });
        assert!(!isa.is_global());
        assert!(isa.is_stack());
        let isa = Isa(unsafe { ptr::addr_of!(ffi::private::_NSConcreteMallocBlock) });
        assert!(!isa.is_global());
        assert!(!isa.is_stack());
        let isa = Isa(ptr::null());
        assert!(!isa.is_global());
        assert!(!isa.is_stack());
    }
}
