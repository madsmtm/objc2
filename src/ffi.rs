//! Bindings to the Apple Grand Central Dispatch (GCD).

#![allow(non_camel_case_types)]

//use block2::Block;
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

macro_rules! create_opaque_type {
    ($type_name: ident, $typedef_name: ident) => {
        #[repr(C)]
        #[derive(Debug)]
        pub struct $type_name {
            _inner: [u8; 0],
        }

        pub type $typedef_name = *mut $type_name;
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
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    $(&$ident::$variant => write!(f, "{}::{}", stringify!($ident), stringify!($variant)),)*
                    &$ident(v) => write!(f, "UNKNOWN({})", v),
                }
            }
        }
    }
}

create_opaque_type!(dispatch_object_s, dispatch_object_t);
create_opaque_type!(dispatch_data_s, dispatch_data_t);
create_opaque_type!(dispatch_source_type_s, dispatch_source_type_t);

// As we cannot switch block from one type to another, we let the user do the pointer conversion.
/// The prototype of blocks submitted to dispatch queues, which take no arguments and have no return value.
pub type dispatch_block_t = *const c_void;
/// The prototype of functions submitted to dispatch queues.
pub type dispatch_function_t = extern "C" fn(*mut c_void);
/// A predicate for use with the dispatch_once function.
pub type dispatch_once_t = usize;

/// An abstract representation of time.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct dispatch_time_t(pub u64);

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

pub const DISPATCH_QUEUE_SERIAL: dispatch_queue_attr_t = core::ptr::null_mut();
pub static DISPATCH_QUEUE_CONCURRENT: &dispatch_queue_attr_s = {
    // Safety: immutable external definition
    unsafe { &_dispatch_queue_attr_concurrent }
};

pub const DISPATCH_APPLY_AUTO: dispatch_queue_t = core::ptr::null_mut();
pub const DISPATCH_TARGET_QUEUE_DEFAULT: dispatch_queue_t = core::ptr::null_mut();
pub const DISPATCH_CURRENT_QUEUE_LABEL: dispatch_queue_t = core::ptr::null_mut();

pub const DISPATCH_TIME_NOW: dispatch_time_t = dispatch_time_t(0);
pub const DISPATCH_TIME_FOREVER: dispatch_time_t = dispatch_time_t(u64::MAX);
pub const QOS_MIN_RELATIVE_PRIORITY: i32 = -15;

enum_with_val! {
    /// Flags to pass to the [dispatch_block_create] and [dispatch_block_create_with_qos_class] functions.
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_block_flags_t(pub c_long) {
        DISPATCH_BLOCK_BARRIER = 0x1,
        DISPATCH_BLOCK_DETACHED = 0x2,
        DISPATCH_BLOCK_ASSIGN_CURRENT = 0x4,
        DISPATCH_BLOCK_NO_QOS_CLASS = 0x8,
        DISPATCH_BLOCK_INHERIT_QOS_CLASS = 0x10,
        DISPATCH_BLOCK_ENFORCE_QOS_CLASS = 0x20,
    }
}

enum_with_val! {
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_autorelease_frequency_t(pub c_ulong) {
        DISPATCH_AUTORELEASE_FREQUENCY_INHERIT = 0x0,
        DISPATCH_AUTORELEASE_FREQUENCY_WORK_ITEM = 0x1,
        DISPATCH_AUTORELEASE_FREQUENCY_NEVER = 0x2,
    }
}

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

#[cfg_attr(
    any(target_os = "macos", target_os = "ios", target_os = "tvos"),
    link(name = "System", kind = "dylib")
)]
#[cfg_attr(
    not(any(target_os = "macos", target_os = "ios", target_os = "tvos")),
    link(name = "dispatch", kind = "dylib")
)]
extern "C" {
    /// Increments the reference count (the retain count) of a dispatch object.
    pub fn dispatch_retain(object: dispatch_object_t);
    /// Decrements the reference count (the retain count) of a dispatch object.
    pub fn dispatch_release(object: dispatch_object_t);
    /// Returns the application-defined context of an object.
    pub fn dispatch_get_context(object: dispatch_object_t) -> *mut c_void;
    /// Associates an application-defined context with the object.
    pub fn dispatch_set_context(object: dispatch_object_t, context: *mut c_void);
    /// Sets the finalizer function for a dispatch object.
    pub fn dispatch_set_finalizer_f(object: dispatch_object_t, finalizer: dispatch_function_t);
    /// Activates the dispatch object.
    pub fn dispatch_activate(object: dispatch_object_t);
    /// Suspends the invocation of block objects on a dispatch object.
    pub fn dispatch_suspend(object: dispatch_object_t);
    /// Resumes the invocation of block objects on a dispatch object.
    pub fn dispatch_resume(object: dispatch_object_t);
    /// Specifies the minimum quality-of-service level for a dispatch queue, source, or workloop.
    pub fn dispatch_set_qos_class_floor(
        object: dispatch_object_t,
        qos_class: dispatch_qos_class_t,
        relative_priority: i32,
    );

    /// Executes a block object only once for the lifetime of an application.
    pub fn dispatch_once(predicate: *mut dispatch_once_t, block: dispatch_block_t);
    /// Executes an application-defined function only once for the lifetime of an application.
    pub fn dispatch_once_f(
        predicate: *mut dispatch_once_t,
        context: *mut c_void,
        function: dispatch_function_t,
    );

    /// Creates a new dispatch block on the heap using an existing block and the given flags.
    pub fn dispatch_block_create(
        flags: dispatch_block_flags_t,
        block: dispatch_block_t,
    ) -> dispatch_block_t;
    /// Creates a new dispatch block from an existing block and the given flags, and assigns it the specified quality-of-service class and relative priority.
    pub fn dispatch_block_create_with_qos_class(
        flags: dispatch_block_flags_t,
        qos_class: dispatch_qos_class_t,
        relative_priority: i32,
        block: dispatch_block_t,
    ) -> dispatch_block_t;
    /// Creates, synchronously executes, and releases a dispatch block from the specified block and flags.
    pub fn dispatch_block_perform(flags: dispatch_block_flags_t, block: dispatch_block_t);
    /// Waits synchronously until execution of the specified dispatch block has completed or until the specified timeout has elapsed.
    pub fn dispatch_block_wait(block: dispatch_block_t, timeout: dispatch_time_t) -> isize;
    /// Schedules a notification block to be submitted to a queue when the execution of a specified dispatch block has completed.
    pub fn dispatch_block_notify(
        block: dispatch_block_t,
        queue: dispatch_queue_t,
        notification_block: dispatch_block_t,
    );
    /// Cancels the specified dispatch block asynchronously.
    pub fn dispatch_block_cancel(block: dispatch_block_t);
    /// Tests whether the given dispatch block has been canceled.
    pub fn dispatch_block_testcancel(block: dispatch_block_t) -> isize;

    pub static _dispatch_data_empty: dispatch_data_s;
    pub static _dispatch_data_destructor_free: dispatch_block_t;
    pub static _dispatch_data_destructor_munmap: dispatch_block_t;
    /// Creates a new dispatch data object with the specified memory buffer.
    pub fn dispatch_data_create(
        buffer: *const c_void,
        size: usize,
        queue: dispatch_queue_t,
        destructor: dispatch_block_t,
    ) -> dispatch_data_t;
    /// Returns the logical size of the memory managed by a dispatch data object
    pub fn dispatch_data_get_size(data: dispatch_data_t) -> usize;
    /// Returns a new dispatch data object containing a contiguous representation of the specified object’s memory.
    pub fn dispatch_data_create_map(
        data: dispatch_data_t,
        buffer_ptr: *const c_void,
        size_ptr: *mut usize,
    ) -> dispatch_data_t;
    /// Returns a new dispatch data object consisting of the concatenated data from two other data objects.
    pub fn dispatch_data_create_concat(
        data1: dispatch_data_t,
        data2: dispatch_data_t,
    ) -> dispatch_data_t;
    /// Returns a new dispatch data object whose contents consist of a portion of another object’s memory region.
    pub fn dispatch_data_create_subrange(
        data: dispatch_data_t,
        offset: usize,
        length: usize,
    ) -> dispatch_data_t;
    // TODO: check if block2::Block layout is safe to use via FFI directly.
    /*pub fn dispatch_data_apply(
        data: dispatch_data_t,
        applier: Block<(usize, *const c_void, usize), bool>,
    ) -> bool;*/
    /// Returns a data object containing a portion of the data in another data object.
    pub fn dispatch_data_copy_region(
        data: dispatch_data_t,
        location: usize,
        offset_ptr: *mut usize,
    ) -> dispatch_data_t;

    /// Creates a new group to which you can assign block objects.
    pub fn dispatch_group_create() -> dispatch_group_t;
    /// Schedules a block asynchronously for execution and simultaneously associates it with the specified dispatch group.
    pub fn dispatch_group_async(
        group: dispatch_group_t,
        queue: dispatch_queue_t,
        block: dispatch_block_t,
    );
    /// Submits an application-defined function to a dispatch queue and associates it with the specified dispatch group.
    pub fn dispatch_group_async_f(
        group: dispatch_group_t,
        queue: dispatch_queue_t,
        context: *const c_void,
        work: dispatch_function_t,
    );
    /// Waits synchronously for the previously submitted block objects to finish; returns if the blocks do not complete before the specified timeout period has elapsed.
    pub fn dispatch_group_wait(group: dispatch_group_t, timeout: dispatch_time_t) -> usize;
    /// Schedules a block object to be submitted to a queue when a group of previously submitted block objects have completed.
    pub fn dispatch_group_notify(
        group: dispatch_group_t,
        queue: dispatch_queue_t,
        block: dispatch_block_t,
    );
    /// Schedules an application-defined function to be submitted to a queue when a group of previously submitted block objects have completed.
    pub fn dispatch_group_notify_f(
        group: dispatch_group_t,
        queue: dispatch_queue_t,
        context: *const c_void,
        work: dispatch_function_t,
    );
    /// Explicitly indicates that a block has entered the group.
    pub fn dispatch_group_enter(group: dispatch_group_t);
    /// Explicitly indicates that a block in the group finished executing.
    pub fn dispatch_group_leave(group: dispatch_group_t);

    /// Submits a block for asynchronous execution on a dispatch queue and returns immediately.
    pub fn dispatch_async(queue: dispatch_queue_t, block: dispatch_block_t);
    /// Submits an app-defined function for asynchronous execution on a dispatch queue and returns immediately.
    pub fn dispatch_async_f(
        queue: dispatch_queue_t,
        context: *mut c_void,
        work: dispatch_function_t,
    );
    /// Submits a block object for execution and returns after that block finishes executing.
    pub fn dispatch_sync(queue: dispatch_queue_t, block: dispatch_block_t);
    /// Submits an app-defined function for synchronous execution on a dispatch queue.
    pub fn dispatch_sync_f(
        queue: dispatch_queue_t,
        context: *mut c_void,
        work: dispatch_function_t,
    );
    /// Submits a work item for execution and returns only after it finishes executing.
    pub fn dispatch_async_and_wait(queue: dispatch_queue_t, block: dispatch_block_t);
    /// Submits a function-based work item for execution and returns only after it finishes executing.
    pub fn dispatch_async_and_wait_f(
        queue: dispatch_queue_t,
        context: *mut c_void,
        work: dispatch_function_t,
    );
    // TODO: check if block2::Block layout is safe to use via FFI directly.
    //pub fn dispatch_apply(iterations: usize, queue: dispatch_queue_t, block: Block<(usize,), ()>);
    /// Submits a single function to the dispatch queue and causes the function to be executed the specified number of times.
    pub fn dispatch_apply_f(
        iterations: usize,
        queue: dispatch_queue_t,
        context: *mut c_void,
        work: extern "C" fn(context: *mut c_void, iteration: usize),
    );
    pub static _dispatch_main_q: dispatch_queue_s;
    /// Returns a system-defined global concurrent queue with the specified quality-of-service class.
    pub fn dispatch_get_global_queue(identifier: usize, flags: usize) -> dispatch_queue_global_t;
    pub static _dispatch_queue_attr_concurrent: dispatch_queue_attr_s;
    /// Returns an attribute that configures a dispatch queue as initially inactive.
    pub fn dispatch_queue_attr_make_initially_inactive(
        attr: dispatch_queue_attr_t,
    ) -> dispatch_queue_attr_t;
    /// Returns an attribute that specifies how the dispatch queue manages autorelease pools for the blocks it executes.
    pub fn dispatch_queue_attr_make_with_autorelease_frequency(
        attr: dispatch_queue_attr_t,
        frequency: dispatch_autorelease_frequency_t,
    ) -> dispatch_queue_attr_t;
    /// Returns attributes suitable for creating a dispatch queue with the desired quality-of-service information.
    pub fn dispatch_queue_attr_make_with_qos_class(
        attr: dispatch_queue_attr_t,
        qos_class: dispatch_qos_class_t,
        relative_priority: c_int,
    ) -> dispatch_queue_attr_t;
    /// Creates a new dispatch queue to which you can submit blocks.
    #[cfg_attr(
        any(target_os = "macos", target_os = "ios", target_os = "tvos"),
        link_name = "dispatch_queue_create_with_target$V2"
    )]
    pub fn dispatch_queue_create_with_target(
        label: *const c_char,
        attr: dispatch_queue_attr_t,
        target: dispatch_queue_t,
    ) -> dispatch_queue_t;
    /// Creates a new dispatch queue to which you can submit blocks.
    pub fn dispatch_queue_create(
        label: *const c_char,
        attr: dispatch_queue_attr_t,
    ) -> dispatch_queue_t;
    /// Returns the label you assigned to the dispatch queue at creation time.
    pub fn dispatch_queue_get_label(queue: dispatch_queue_t) -> *const c_char;
    /// Returns the quality-of-service class for the specified queue.
    pub fn dispatch_queue_get_qos_class(
        queue: dispatch_queue_t,
        relative_priority_ptr: *mut c_int,
    ) -> dispatch_qos_class_t;
    /// Specifies the dispatch queue on which to perform work associated with the current object.
    pub fn dispatch_set_target_queue(object: dispatch_object_t, queue: dispatch_queue_t);
    /// Executes blocks submitted to the main queue.
    pub fn dispatch_main() -> !;
    /// Enqueues a block for execution at the specified time.
    pub fn dispatch_after(when: dispatch_time_t, queue: dispatch_queue_t, block: dispatch_block_t);
    /// Enqueues an app-defined function for execution at the specified time.
    pub fn dispatch_after_f(
        when: dispatch_time_t,
        queue: dispatch_queue_t,
        context: *mut c_void,
        work: dispatch_function_t,
    );
    /// Submits a barrier block for asynchronous execution and returns immediately.
    pub fn dispatch_barrier_async(queue: dispatch_queue_t, block: dispatch_block_t);
    /// Submits a barrier function for asynchronous execution and returns immediately.
    pub fn dispatch_barrier_async_f(
        queue: dispatch_queue_t,
        context: *mut c_void,
        work: dispatch_function_t,
    );
    /// Submits a barrier block object for execution and waits until that block completes.
    pub fn dispatch_barrier_sync(queue: dispatch_queue_t, block: dispatch_block_t);
    /// Submits a barrier function for execution and waits until that function completes.
    pub fn dispatch_barrier_sync_f(
        queue: dispatch_queue_t,
        context: *mut c_void,
        work: dispatch_function_t,
    );
    /// Submits a work item for synchronous execution and marks the work as a barrier for subsequent concurrent tasks.
    pub fn dispatch_barrier_async_and_wait(queue: dispatch_queue_t, block: dispatch_block_t);
    /// Submits a function-based work item for synchronous execution and marks the work as a barrier for subsequent concurrent tasks.
    pub fn dispatch_barrier_async_and_wait_f(
        queue: dispatch_queue_t,
        context: *mut c_void,
        work: dispatch_function_t,
    );
    /// Sets the key/value data for the specified dispatch queue.
    pub fn dispatch_queue_set_specific(
        queue: dispatch_queue_t,
        key: *const c_void,
        context: *mut c_void,
        destructor: dispatch_function_t,
    );
    /// Gets the value for the key associated with the specified dispatch queue.
    pub fn dispatch_queue_get_specific(queue: dispatch_queue_t, key: *const c_void) -> *mut c_void;
    /// Returns the value for the key associated with the current dispatch queue.
    pub fn dispatch_get_specific(key: *const c_void) -> *mut c_void;
    #[cfg_attr(
        any(target_os = "macos", target_os = "ios", target_os = "tvos"),
        link_name = "dispatch_assert_queue$V2"
    )]
    /// Generates an assertion if the current block is not running on the specified dispatch queue.
    pub fn dispatch_assert_queue(queue: dispatch_queue_t);
    /// Generates an assertion if the current block is not running as a barrier on the specified dispatch queue.
    pub fn dispatch_assert_queue_barrier(queue: dispatch_queue_t);
    #[cfg_attr(
        any(target_os = "macos", target_os = "ios", target_os = "tvos"),
        link_name = "dispatch_assert_queue_not$V2"
    )]
    /// Generates an assertion if the current block is executing on the specified dispatch queue.
    pub fn dispatch_assert_queue_not(queue: dispatch_queue_t);

    /// Creates new counting semaphore with an initial value.
    pub fn dispatch_semaphore_create(value: isize) -> dispatch_semaphore_t;
    /// Waits for (decrements) a semaphore.
    pub fn dispatch_semaphore_wait(dsema: dispatch_semaphore_t, timeout: dispatch_time_t) -> usize;
    /// Signals (increments) a semaphore.
    pub fn dispatch_semaphore_signal(dsema: dispatch_semaphore_t) -> usize;

    pub static _dispatch_source_type_data_add: dispatch_source_type_s;
    pub static _dispatch_source_type_data_or: dispatch_source_type_s;
    pub static _dispatch_source_type_data_replace: dispatch_source_type_s;
    pub static _dispatch_source_type_mach_send: dispatch_source_type_s;
    pub static _dispatch_source_type_memorypressure: dispatch_source_type_s;
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos"))]
    pub static _dispatch_source_type_proc: dispatch_source_type_s;
    pub static _dispatch_source_type_read: dispatch_source_type_s;
    pub static _dispatch_source_type_timer: dispatch_source_type_s;
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos"))]
    pub static _dispatch_source_type_vnode: dispatch_source_type_s;
    pub static _dispatch_source_type_write: dispatch_source_type_s;
    /// Creates a new dispatch source to monitor low-level system events.
    pub fn dispatch_source_create(
        r#type: dispatch_source_type_t,
        handle: usize,
        mask: usize,
        queue: dispatch_queue_t,
    ) -> dispatch_source_t;
    /// Sets the event handler block for the given dispatch source.
    pub fn dispatch_source_set_event_handler(source: dispatch_source_t, handler: dispatch_block_t);
    /// Sets the event handler function for the given dispatch source.
    pub fn dispatch_source_set_event_handler_f(
        source: dispatch_source_t,
        handler: dispatch_function_t,
    );
    /// Sets the cancellation handler block for the given dispatch source.
    pub fn dispatch_source_set_cancel_handler(source: dispatch_source_t, handler: dispatch_block_t);
    /// Sets the cancellation handler function for the given dispatch source.
    pub fn dispatch_source_set_cancel_handler_f(
        source: dispatch_source_t,
        handler: dispatch_function_t,
    );
    /// Asynchronously cancels the dispatch source, preventing any further invocation of its event handler block.
    pub fn dispatch_source_cancel(source: dispatch_source_t);
    /// Tests whether the given dispatch source has been canceled.
    pub fn dispatch_source_testcancel(source: dispatch_source_t) -> usize;
    /// Returns the underlying system handle associated with the specified dispatch source.
    pub fn dispatch_source_get_handle(source: dispatch_source_t) -> usize;
    /// Returns the mask of events monitored by the dispatch source.
    pub fn dispatch_source_get_mask(source: dispatch_source_t) -> usize;
    /// Returns pending data for the dispatch source.
    pub fn dispatch_source_get_data(source: dispatch_source_t) -> usize;
    /// Merges data into a dispatch source and submits its event handler block to its target queue.
    pub fn dispatch_source_merge_data(source: dispatch_source_t, value: usize);
    /// Sets a start time, interval, and leeway value for a timer source.
    pub fn dispatch_source_set_timer(
        source: dispatch_source_t,
        start: dispatch_time_t,
        interval: u64,
        leeway: u64,
    );
    /// Sets the registration handler block for the given dispatch source.
    pub fn dispatch_source_set_registration_handler(
        source: dispatch_source_t,
        handler: dispatch_block_t,
    );
    /// Sets the registration handler function for the given dispatch source.
    pub fn dispatch_source_set_registration_handler_f(
        source: dispatch_source_t,
        handler: dispatch_function_t,
    );

    /// Creates a [dispatch_time_t] relative to the default clock or modifies an existing [dispatch_time_t].
    pub fn dispatch_time(when: dispatch_time_t, delta: i64) -> dispatch_time_t;
    /// Creates a [dispatch_time_t] using an absolute time according to the wall clock.
    pub fn dispatch_walltime(when: *const libc::timespec, delta: i64) -> dispatch_time_t;

    /// Creates a new workloop with the specified label.
    pub fn dispatch_workloop_create(label: *const c_char) -> dispatch_workloop_t;
    /// Creates a new inactive workloop with the specified label.
    pub fn dispatch_workloop_create_inactive(label: *const c_char) -> dispatch_workloop_t;
    /// Configures how the workloop manages the autorelease pools for the blocks it executes.
    pub fn dispatch_workloop_set_autorelease_frequency(
        workloop: dispatch_workloop_t,
        frequency: dispatch_autorelease_frequency_t,
    );
    // TODO: dispatch_workloop_set_os_workgroup
}
