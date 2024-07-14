use objc2::{rc::Id, runtime::ProtocolObject};
use objc2_foundation::NSArray;

use crate::MTLDevice;

pub fn MTLCreateSystemDefaultDevice() -> Option<Id<ProtocolObject<dyn MTLDevice>>> {
    unsafe { Id::from_raw(crate::generated::MTLCreateSystemDefaultDevice()) }
}

pub fn MTLCopyAllDevices() -> Id<NSArray<ProtocolObject<dyn MTLDevice>>> {
    unsafe { Id::new_nonnull(crate::generated::MTLCopyAllDevices()) }
}
