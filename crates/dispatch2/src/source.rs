#![allow(missing_docs)] // TODO
use core::ffi::c_ulong;

use crate::{generated, DispatchSourceType};

impl DispatchSourceType {
    /// A dispatch source that coalesces data obtained via calls to
    /// dispatch_source_merge_data(). An ADD is used to coalesce the data.
    ///
    /// The handle is unused (pass zero for now).
    /// The mask is unused (pass zero for now).
    #[doc(alias = "DISPATCH_SOURCE_TYPE_DATA_ADD")]
    #[inline]
    pub fn data_add() -> &'static Self {
        // SAFETY: Static is immutable.
        unsafe { &generated::_dispatch_source_type_data_add }
    }

    /// A dispatch source that coalesces data obtained via calls to
    /// dispatch_source_merge_data(). A bitwise OR is used to coalesce the data.
    ///
    /// The handle is unused (pass zero for now).
    /// The mask is unused (pass zero for now).
    #[doc(alias = "DISPATCH_SOURCE_TYPE_DATA_OR")]
    #[inline]
    pub fn data_or() -> &'static Self {
        // SAFETY: Static is immutable.
        unsafe { &generated::_dispatch_source_type_data_or }
    }

    /// A dispatch source that tracks data obtained via calls to
    /// dispatch_source_merge_data(). Newly obtained data values replace existing
    /// data values not yet delivered to the source handler
    ///
    /// A data value of zero will cause the source handler to not be invoked.
    ///
    /// The handle is unused (pass zero for now).
    /// The mask is unused (pass zero for now).
    #[doc(alias = "DISPATCH_SOURCE_TYPE_DATA_REPLACE")]
    #[inline]
    pub fn data_replace() -> &'static Self {
        // SAFETY: Static is immutable.
        unsafe { &generated::_dispatch_source_type_data_replace }
    }

    /// A dispatch source that monitors a Mach port for dead name
    /// notifications (send right no longer has any corresponding receive right).
    ///
    /// The handle is a Mach port with a send or send-once right (mach_port_t).
    /// The mask is a mask of desired events from `DispatchSourceMachSendFlags`.
    #[doc(alias = "DISPATCH_SOURCE_TYPE_MACH_SEND")]
    #[inline]
    pub fn mach_send() -> &'static Self {
        // SAFETY: Static is immutable.
        unsafe { &generated::_dispatch_source_type_mach_send }
    }

    /// A dispatch source that monitors a Mach port for pending messages.
    ///
    /// The handle is a Mach port with a receive right (mach_port_t).
    /// The mask is a mask of desired events from `DispatchSourceMachReceiveFlags`,
    /// but no flags are currently defined (pass zero for now).
    #[doc(alias = "DISPATCH_SOURCE_TYPE_MACH_RECV")]
    #[inline]
    pub fn mach_receive() -> &'static Self {
        // SAFETY: Static is immutable.
        unsafe { &generated::_dispatch_source_type_mach_recv }
    }

    /// A dispatch source that monitors the system for changes in
    /// memory pressure condition.
    ///
    /// The handle is unused (pass zero for now).
    /// The mask is a mask of desired events from `DispatchSourceMemoryPressureFlags`.
    #[doc(alias = "DISPATCH_SOURCE_TYPE_MEMORYPRESSURE")]
    #[inline]
    pub fn memory_pressure() -> &'static Self {
        // SAFETY: Static is immutable.
        unsafe { &generated::_dispatch_source_type_memorypressure }
    }

    /// A dispatch source that monitors an external process for events
    /// defined by DispatchSourceProcessFlags.
    ///
    /// The handle is a process identifier (pid_t).
    /// The mask is a mask of desired events from `DispatchSourceProcessFlags`.
    #[doc(alias = "DISPATCH_SOURCE_TYPE_PROC")]
    #[inline]
    pub fn process() -> &'static Self {
        // SAFETY: Static is immutable.
        unsafe { &generated::_dispatch_source_type_proc }
    }

    /// A dispatch source that monitors a file descriptor for pending
    /// bytes available to be read.
    ///
    /// The handle is a file descriptor (int).
    /// The mask is unused (pass zero for now).
    #[doc(alias = "DISPATCH_SOURCE_TYPE_READ")]
    #[inline]
    pub fn read() -> &'static Self {
        // SAFETY: Static is immutable.
        unsafe { &generated::_dispatch_source_type_read }
    }

    /// A dispatch source that monitors the current process for signals.
    ///
    /// The handle is a signal number (int).
    /// The mask is unused (pass zero for now).
    #[doc(alias = "DISPATCH_SOURCE_TYPE_SIGNAL")]
    #[inline]
    pub fn signal() -> &'static Self {
        // SAFETY: Static is immutable.
        unsafe { &generated::_dispatch_source_type_signal }
    }

    /// A dispatch source that submits the event handler block based
    /// on a timer.
    ///
    /// The handle is unused (pass zero for now).
    /// The mask specifies which flags from `DispatchSourceTimerFlags` to apply.
    #[doc(alias = "DISPATCH_SOURCE_TYPE_TIMER")]
    #[inline]
    pub fn timer() -> &'static Self {
        // SAFETY: Static is immutable.
        unsafe { &generated::_dispatch_source_type_timer }
    }

    /// A dispatch source that monitors a file descriptor for events
    /// defined by `DispatchSourceFileSystemFlags`.
    ///
    /// The handle is a file descriptor (int).
    /// The mask is a mask of desired events from `DispatchSourceFileSystemFlags`.
    #[doc(alias = "DISPATCH_SOURCE_TYPE_VNODE")]
    #[inline]
    pub fn file_system() -> &'static Self {
        // SAFETY: Static is immutable.
        unsafe { &generated::_dispatch_source_type_vnode }
    }

    /// A dispatch source that monitors a file descriptor for available
    /// buffer space to write bytes.
    ///
    /// The handle is a file descriptor (int).
    /// The mask is unused (pass zero for now).
    #[doc(alias = "DISPATCH_SOURCE_TYPE_WRITE")]
    #[inline]
    pub fn write() -> &'static Self {
        // SAFETY: Static is immutable.
        unsafe { &generated::_dispatch_source_type_write }
    }
}

enum_with_val! {
    /// Mach send-right flags.
    #[doc(alias = "dispatch_source_mach_send_flags_t")]
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct DispatchSourceMachSendFlags(pub c_ulong) {
        #[doc(alias = "DISPATCH_MACH_SEND_DEAD")]
        Dead = 0x1
    }
}

enum_with_val! {
    /// Mach receive-right flags.
    #[doc(alias = "dispatch_source_mach_recv_flags_t")]
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct DispatchSourceMachReceiveFlags(pub c_ulong) {
        // no definition
    }
}

enum_with_val! {
    // Memory pressure events.
    #[doc(alias = "dispatch_source_memorypressure_flags_t")]
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct DispatchSourceMemoryPressureFlags(pub c_ulong) {
        #[doc(alias = "DISPATCH_MEMORYPRESSURE_NORMAL")]
        Normal = 0x1,
        #[doc(alias = "DISPATCH_MEMORYPRESSURE_WARN")]
        Warn = 0x2,
        #[doc(alias = "DISPATCH_MEMORYPRESSURE_CRITICAL")]
        Critical = 0x4,
    }
}

enum_with_val! {
    /// Events related to a process.
    #[doc(alias = "dispatch_source_proc_flags_t")]
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct DispatchSourceProcessFlags(pub c_ulong) {
        #[doc(alias = "DISPATCH_PROC_EXIT")]
        Exit = 0x80000000,
        #[doc(alias = "DISPATCH_PROC_FORK")]
        Fork = 0x40000000,
        #[doc(alias = "DISPATCH_PROC_EXEC")]
        Exec = 0x20000000,
        #[doc(alias = "DISPATCH_PROC_SIGNAL")]
        Signal = 0x08000000,
    }
}

enum_with_val! {
    /// Events involving a change to a file system object.
    #[doc(alias = "dispatch_source_vnode_flags_t")]
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct DispatchSourceFileSystemFlags(pub c_ulong) {
        #[doc(alias = "DISPATCH_VNODE_DELETE")]
        Delete = 0x1,
        #[doc(alias = "DISPATCH_VNODE_WRITE")]
        Write = 0x2,
        #[doc(alias = "DISPATCH_VNODE_EXTEND")]
        Extend = 0x4,
        #[doc(alias = "DISPATCH_VNODE_ATTRIB")]
        Attrib = 0x8,
        #[doc(alias = "DISPATCH_VNODE_LINK")]
        Link = 0x10,
        #[doc(alias = "DISPATCH_VNODE_RENAME")]
        Rename = 0x20,
        #[doc(alias = "DISPATCH_VNODE_REVOKE")]
        Revoke = 0x40,
        #[doc(alias = "DISPATCH_VNODE_FUNLOCK")]
        Funlock = 0x100,
    }
}

enum_with_val! {
    /// Flags to use when configuring a timer dispatch source.
    #[doc(alias = "dispatch_source_timer_flags_t")]
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct DispatchSourceTimerFlags(pub c_ulong) {
        #[doc(alias = "DISPATCH_TIMER_STRICT")]
        Strict = 0x1,
    }
}
