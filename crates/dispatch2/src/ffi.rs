//! Raw bindings to Apple's Grand Central Dispatch (GCD).

#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_long, c_uint, c_ulong, c_void};
use core::ptr::addr_of;

#[cfg(feature = "objc2")]
use objc2::encode::{Encode, Encoding, RefEncode};

// Try to generate as much as possible.
pub use crate::generated::*;

macro_rules! create_opaque_type {
    ($type_name: ident, $typedef_name: ident) => {
        #[repr(C)]
        #[derive(Copy, Clone, Debug)]
        #[allow(missing_docs)]
        pub struct $type_name {
            /// opaque value
            _inner: [u8; 0],
        }

        #[allow(missing_docs)]
        pub type $typedef_name = *mut $type_name;

        #[cfg(feature = "objc2")]
        // SAFETY: Dispatch types are internally objects.
        unsafe impl RefEncode for $type_name {
            const ENCODING_REF: Encoding = Encoding::Object;
        }
    };
}

macro_rules! enum_with_val {
    ($(#[$meta:meta])* $vis:vis struct $ident:ident($innervis:vis $ty:ty) {
        $($(#[$varmeta:meta])* $variant:ident = $num:expr),* $(,)*
    }) => {
        $(#[$meta])*
        #[repr(transparent)]
        $vis struct $ident($innervis $ty);
        impl $ident {
            $($(#[$varmeta])* $vis const $variant: $ident = $ident($num);)*
        }

        impl ::core::fmt::Debug for $ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    $(&$ident::$variant => write!(f, "{}::{}", stringify!($ident), stringify!($variant)),)*
                    &$ident(v) => write!(f, "UNKNOWN({})", v),
                }
            }
        }

        #[cfg(feature = "objc2")]
        // SAFETY: Marked with `#[repr(transparent)]` above.
        unsafe impl Encode for $ident {
            const ENCODING: Encoding = <$ty>::ENCODING;
        }

        #[cfg(feature = "objc2")]
        // SAFETY: Same as above.
        unsafe impl RefEncode for $ident {
            const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
        }
    }
}

create_opaque_type!(dispatch_object_s, dispatch_object_t);
create_opaque_type!(dispatch_data_s, dispatch_data_t);
create_opaque_type!(dispatch_source_type_s, dispatch_source_type_t);

/// The prototype of functions submitted to dispatch queues.
///
/// This is deliberately `extern "C"`, since libdispatch doesn't support
/// unwinding in handler functions, and this gives us better error messages
/// if that does happen.
pub type dispatch_function_t = extern "C" fn(*mut c_void);

/// An abstract representation of time.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct dispatch_time_t(pub u64);

#[cfg(feature = "objc2")]
// SAFETY: `dispatch_time_t` is `#[repr(transparent)]`.
unsafe impl Encode for dispatch_time_t {
    const ENCODING: Encoding = u64::ENCODING;
}

#[cfg(feature = "objc2")]
// SAFETY: Same as above.
unsafe impl RefEncode for dispatch_time_t {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

create_opaque_type!(dispatch_group_s, dispatch_group_t);
create_opaque_type!(dispatch_queue_global_s, dispatch_queue_global_t);
create_opaque_type!(dispatch_queue_serial_s, dispatch_queue_serial_t);
create_opaque_type!(dispatch_queue_main_s, dispatch_queue_main_t);
create_opaque_type!(dispatch_queue_concurrent_s, dispatch_queue_concurrent_t);
create_opaque_type!(dispatch_queue_attr_s, dispatch_queue_attr_t);
create_opaque_type!(dispatch_semaphore_s, dispatch_semaphore_t);
create_opaque_type!(dispatch_source_s, dispatch_source_t);
create_opaque_type!(dispatch_queue_s, dispatch_queue_t);
create_opaque_type!(dispatch_workloop_s, dispatch_workloop_t);
create_opaque_type!(dispatch_io_s, dispatch_io_t);

/// A dispatch queue that executes blocks serially in FIFO order.
pub const DISPATCH_QUEUE_SERIAL: dispatch_queue_attr_t = core::ptr::null_mut();
/// A dispatch queue that executes blocks concurrently.
pub static DISPATCH_QUEUE_CONCURRENT: &dispatch_queue_attr_s = {
    // Safety: immutable external definition
    unsafe { &_dispatch_queue_attr_concurrent }
};

pub const DISPATCH_APPLY_AUTO: dispatch_queue_t = core::ptr::null_mut();
pub const DISPATCH_TARGET_QUEUE_DEFAULT: dispatch_queue_t = core::ptr::null_mut();
pub const DISPATCH_CURRENT_QUEUE_LABEL: dispatch_queue_t = core::ptr::null_mut();

pub const DISPATCH_TIME_NOW: dispatch_time_t = dispatch_time_t(0);
pub const DISPATCH_TIME_FOREVER: dispatch_time_t = dispatch_time_t(u64::MAX);
pub const DISPATCH_WALLTIME_NOW: dispatch_time_t = dispatch_time_t(!1);
pub const QOS_MIN_RELATIVE_PRIORITY: i32 = -15;

enum_with_val! {
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_queue_priority_t(pub c_long) {
        DISPATCH_QUEUE_PRIORITY_HIGH = 0x2,
        DISPATCH_QUEUE_PRIORITY_DEFAULT = 0x0,
        DISPATCH_QUEUE_PRIORITY_LOW = -0x2,
        DISPATCH_QUEUE_PRIORITY_BACKGROUND = u16::MIN as c_long,
    }
}

enum_with_val! {
    /// Quality-of-service classes that specify the priorities for executing tasks.
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_qos_class_t(pub c_uint) {
        QOS_CLASS_USER_INTERACTIVE = 0x21,
        QOS_CLASS_USER_INITIATED = 0x19,
        QOS_CLASS_DEFAULT = 0x15,
        QOS_CLASS_UTILITY = 0x11,
        QOS_CLASS_BACKGROUND = 0x09,
        QOS_CLASS_UNSPECIFIED = 0x00,
    }
}

enum_with_val! {
    /// Mach send-right flags.
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_source_mach_send_flags_t(pub c_ulong) {
        DISPATCH_MACH_SEND_DEAD = 0x1
    }
}

enum_with_val! {
    /// Mach receive-right flags.
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_source_mach_recv_flags_t(pub c_ulong) {
        // no definition
    }
}

enum_with_val! {
    // Memory pressure events.
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_source_memorypressure_flags_t(pub c_ulong) {
        DISPATCH_MEMORYPRESSURE_NORMAL = 0x1,
        DISPATCH_MEMORYPRESSURE_WARN = 0x2,
        DISPATCH_MEMORYPRESSURE_CRITICAL = 0x4,
    }
}

enum_with_val! {
    /// Events related to a process.
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_source_proc_flags_t(pub c_ulong) {
        DISPATCH_PROC_EXIT = 0x80000000,
        DISPATCH_PROC_FORK = 0x40000000,
        DISPATCH_PROC_EXEC = 0x20000000,
        DISPATCH_PROC_SIGNAL = 0x08000000,
    }
}

enum_with_val! {
    /// Events involving a change to a file system object.
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_source_vnode_flags_t(pub c_ulong) {
        DISPATCH_VNODE_DELETE = 0x1,
        DISPATCH_VNODE_WRITE = 0x2,
        DISPATCH_VNODE_EXTEND = 0x4,
        DISPATCH_VNODE_ATTRIB = 0x8,
        DISPATCH_VNODE_LINK = 0x10,
        DISPATCH_VNODE_RENAME = 0x20,
        DISPATCH_VNODE_REVOKE = 0x40,
        DISPATCH_VNODE_FUNLOCK = 0x100,
    }
}

enum_with_val! {
    /// Flags to use when configuring a timer dispatch source.
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_source_timer_flags_t(pub c_ulong) {
        DISPATCH_TIMER_STRICT = 0x1,
    }
}

enum_with_val! {
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_io_type_t(pub c_ulong) {
        DISPATCH_IO_STREAM = 0,
        DISPATCH_IO_RANDOM = 1,
    }
}

enum_with_val! {
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_io_close_flags_t(pub c_ulong) {
        DISPATCH_IO_STOP = 0x1,
    }
}

enum_with_val! {
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_io_interval_flags_t(pub c_long) {
        DISPATCH_IO_STRICT_INTERVAL = 0x1,
    }
}

#[cfg_attr(target_vendor = "apple", link(name = "System", kind = "dylib"))]
#[cfg_attr(not(target_vendor = "apple"), link(name = "dispatch", kind = "dylib"))]
extern "C-unwind" {
    /// Executes blocks submitted to the main queue.
    pub fn dispatch_main() -> !;
}

// Inline function in the header
#[allow(unused_unsafe)] // MSRV. Also, we'd like to mark this as `const`
pub extern "C" fn dispatch_get_main_queue() -> dispatch_queue_main_t {
    // SAFETY: Always safe to get pointer from static, only needed for MSRV.
    unsafe { addr_of!(_dispatch_main_q) as dispatch_queue_main_t }
}
