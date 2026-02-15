use core::{borrow::Borrow, ops::Deref};

use crate::{DispatchQueue, DispatchWorkloop};

impl Deref for DispatchWorkloop {
    type Target = DispatchQueue;

    /// Access the workloop as a [`DispatchQueue`].
    #[inline]
    fn deref(&self) -> &Self::Target {
        let ptr: *const DispatchWorkloop = self;
        let ptr: *const DispatchQueue = ptr.cast();
        // SAFETY: Workloop queues are "subclasses" of queues (they can be
        // used in all the same places that normal queues can).
        unsafe { &*ptr }
    }
}

impl AsRef<DispatchQueue> for DispatchWorkloop {
    #[inline]
    fn as_ref(&self) -> &DispatchQueue {
        self
    }
}

// PartialEq, Eq and Hash work the same for workloops and queues.
impl Borrow<DispatchQueue> for DispatchWorkloop {
    #[inline]
    fn borrow(&self) -> &DispatchQueue {
        self
    }
}
