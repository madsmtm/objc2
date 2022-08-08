#![allow(non_camel_case_types)]

//use block2::Block;
use libc::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

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
pub type dispatch_block_t = *const c_void;
pub type dispatch_function_t = extern "C" fn(*mut c_void);
pub type dispatch_once_t = c_long;

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
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_source_mach_send_flags_t(pub c_ulong) {
        DISPATCH_MACH_SEND_DEAD = 0x1
    }
}

enum_with_val! {
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_source_mach_recv_flags_t(pub c_ulong) {
        // no definition
    }
}

enum_with_val! {
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_source_memorypressure_flags_t(pub c_ulong) {
        DISPATCH_MEMORYPRESSURE_NORMAL = 0x1,
        DISPATCH_MEMORYPRESSURE_WARN = 0x2,
        DISPATCH_MEMORYPRESSURE_CRITICAL = 0x4,
    }
}

enum_with_val! {
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_source_proc_flags_t(pub c_ulong) {
        DISPATCH_PROC_EXIT = 0x80000000,
        DISPATCH_PROC_FORK = 0x40000000,
        DISPATCH_PROC_EXEC = 0x20000000,
        DISPATCH_PROC_SIGNAL = 0x08000000,
    }
}

enum_with_val! {
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
    pub fn dispatch_retain(object: dispatch_object_t);
    pub fn dispatch_release(object: dispatch_object_t);
    pub fn dispatch_get_context(object: dispatch_object_t) -> *mut c_void;
    pub fn dispatch_set_context(object: dispatch_object_t, context: *mut c_void);
    pub fn dispatch_set_finalizer_f(object: dispatch_object_t, finalizer: dispatch_function_t);
    pub fn dispatch_activate(object: dispatch_object_t);
    pub fn dispatch_suspend(object: dispatch_object_t);
    pub fn dispatch_resume(object: dispatch_object_t);
    pub fn dispatch_set_qos_class_floor(
        object: dispatch_object_t,
        qos_class: dispatch_qos_class_t,
        relative_priority: i32,
    );

    pub fn dispatch_once(predicate: *mut dispatch_once_t, block: dispatch_block_t);
    pub fn dispatch_once_f(
        predicate: *mut dispatch_once_t,
        context: *mut c_void,
        function: dispatch_function_t,
    );

    pub fn dispatch_block_create(
        flags: dispatch_block_flags_t,
        block: dispatch_block_t,
    ) -> dispatch_block_t;
    pub fn dispatch_block_create_with_qos_class(
        flags: dispatch_block_flags_t,
        qos_class: dispatch_qos_class_t,
        relative_priority: i32,
        block: dispatch_block_t,
    ) -> dispatch_block_t;
    pub fn dispatch_block_perform(flags: dispatch_block_flags_t, block: dispatch_block_t);
    pub fn dispatch_block_wait(block: dispatch_block_t, timeout: dispatch_time_t) -> isize;
    pub fn dispatch_block_notify(
        block: dispatch_block_t,
        queue: dispatch_queue_t,
        notification_block: dispatch_block_t,
    );
    pub fn dispatch_block_cancel(block: dispatch_block_t);
    pub fn dispatch_block_testcancel(block: dispatch_block_t) -> isize;

    pub static _dispatch_data_empty: dispatch_data_s;
    pub static _dispatch_data_destructor_free: dispatch_block_t;
    pub static _dispatch_data_destructor_munmap: dispatch_block_t;
    pub fn dispatch_data_create(
        buffer: *const c_void,
        size: usize,
        queue: dispatch_queue_t,
        destructor: dispatch_block_t,
    ) -> dispatch_data_t;
    pub fn dispatch_data_get_size(data: dispatch_data_t) -> usize;
    pub fn dispatch_data_create_map(
        data: dispatch_data_t,
        buffer_ptr: *const c_void,
        size_ptr: *mut usize,
    ) -> dispatch_data_t;
    pub fn dispatch_data_create_concat(
        data1: dispatch_data_t,
        data2: dispatch_data_t,
    ) -> dispatch_data_t;
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
    pub fn dispatch_data_copy_region(
        data: dispatch_data_t,
        location: usize,
        offset_ptr: *mut usize,
    ) -> dispatch_data_t;

    pub fn dispatch_group_create() -> dispatch_group_t;
    pub fn dispatch_group_async(
        group: dispatch_group_t,
        queue: dispatch_queue_t,
        block: dispatch_block_t,
    );
    pub fn dispatch_group_async_f(
        group: dispatch_group_t,
        queue: dispatch_queue_t,
        context: *const c_void,
        work: dispatch_function_t,
    );
    pub fn dispatch_group_wait(group: dispatch_group_t, timeout: dispatch_time_t) -> usize;
    pub fn dispatch_group_notify(
        group: dispatch_group_t,
        queue: dispatch_queue_t,
        block: dispatch_block_t,
    );
    pub fn dispatch_group_notify_f(
        group: dispatch_group_t,
        queue: dispatch_queue_t,
        context: *const c_void,
        work: dispatch_function_t,
    );
    pub fn dispatch_group_enter(group: dispatch_group_t);
    pub fn dispatch_group_leave(group: dispatch_group_t);

    pub fn dispatch_async(queue: dispatch_queue_t, block: dispatch_block_t);
    pub fn dispatch_async_f(
        queue: dispatch_queue_t,
        context: *mut c_void,
        work: dispatch_function_t,
    );
    pub fn dispatch_sync(queue: dispatch_queue_t, block: dispatch_block_t);
    pub fn dispatch_sync_f(
        queue: dispatch_queue_t,
        context: *mut c_void,
        work: dispatch_function_t,
    );
    pub fn dispatch_async_and_wait(queue: dispatch_queue_t, block: dispatch_block_t);
    pub fn dispatch_async_and_wait_f(
        queue: dispatch_queue_t,
        context: *mut c_void,
        work: dispatch_function_t,
    );
    // TODO: check if block2::Block layout is safe to use via FFI directly.
    //pub fn dispatch_apply(iterations: usize, queue: dispatch_queue_t, block: Block<(usize,), ()>);
    pub fn dispatch_apply_f(
        iterations: usize,
        queue: dispatch_queue_t,
        context: *mut c_void,
        work: extern "C" fn(context: *mut c_void, iteration: usize),
    );
    pub static _dispatch_main_q: dispatch_queue_s;
    pub fn dispatch_get_global_queue(identifier: usize, flags: usize) -> dispatch_queue_global_t;
    pub static _dispatch_queue_attr_concurrent: dispatch_queue_attr_s;
    pub fn dispatch_queue_attr_make_initially_inactive(
        attr: dispatch_queue_attr_t,
    ) -> dispatch_queue_attr_t;
    pub fn dispatch_queue_attr_make_with_autorelease_frequency(
        attr: dispatch_queue_attr_t,
        frequency: dispatch_autorelease_frequency_t,
    ) -> dispatch_queue_attr_t;
    pub fn dispatch_queue_attr_make_with_qos_class(
        attr: dispatch_queue_attr_t,
        qos_class: dispatch_qos_class_t,
        relative_priority: c_int,
    ) -> dispatch_queue_attr_t;
    #[cfg_attr(
        any(target_os = "macos", target_os = "ios", target_os = "tvos"),
        link_name = "dispatch_queue_create_with_target$V2"
    )]
    pub fn dispatch_queue_create_with_target(
        label: *const c_char,
        attr: dispatch_queue_attr_t,
        target: dispatch_queue_t,
    ) -> dispatch_queue_t;
    pub fn dispatch_queue_create(
        label: *const c_char,
        attr: dispatch_queue_attr_t,
    ) -> dispatch_queue_t;
    pub fn dispatch_queue_get_label(queue: dispatch_queue_t) -> *const c_char;
    pub fn dispatch_queue_get_qos_class(
        queue: dispatch_queue_t,
        relative_priority_ptr: *mut c_int,
    ) -> dispatch_qos_class_t;
    pub fn dispatch_set_target_queue(object: dispatch_object_t, queue: dispatch_queue_t);
    pub fn dispatch_main() -> !;
    pub fn dispatch_after(when: dispatch_time_t, queue: dispatch_queue_t, block: dispatch_block_t);
    pub fn dispatch_after_f(
        when: dispatch_time_t,
        queue: dispatch_queue_t,
        context: *mut c_void,
        work: dispatch_function_t,
    );
    pub fn dispatch_barrier_async(queue: dispatch_queue_t, block: dispatch_block_t);
    pub fn dispatch_barrier_async_f(
        queue: dispatch_queue_t,
        context: *mut c_void,
        work: dispatch_function_t,
    );
    pub fn dispatch_barrier_sync(queue: dispatch_queue_t, block: dispatch_block_t);
    pub fn dispatch_barrier_sync_f(
        queue: dispatch_queue_t,
        context: *mut c_void,
        work: dispatch_function_t,
    );
    pub fn dispatch_barrier_async_and_wait(queue: dispatch_queue_t, block: dispatch_block_t);
    pub fn dispatch_barrier_async_and_wait_f(
        queue: dispatch_queue_t,
        context: *mut c_void,
        work: dispatch_function_t,
    );
    pub fn dispatch_queue_set_specific(
        queue: dispatch_queue_t,
        key: *const c_void,
        context: *mut c_void,
        destructor: dispatch_function_t,
    );
    pub fn dispatch_queue_get_specific(queue: dispatch_queue_t, key: *const c_void) -> *mut c_void;
    pub fn dispatch_get_specific(key: *const c_void) -> *mut c_void;
    #[cfg_attr(
        any(target_os = "macos", target_os = "ios", target_os = "tvos"),
        link_name = "dispatch_assert_queue$V2"
    )]
    pub fn dispatch_assert_queue(queue: dispatch_queue_t);
    pub fn dispatch_assert_queue_barrier(queue: dispatch_queue_t);
    #[cfg_attr(
        any(target_os = "macos", target_os = "ios", target_os = "tvos"),
        link_name = "dispatch_assert_queue_not$V2"
    )]
    pub fn dispatch_assert_queue_not(queue: dispatch_queue_t);

    pub fn dispatch_semaphore_create(value: isize) -> dispatch_semaphore_t;
    pub fn dispatch_semaphore_wait(dsema: dispatch_semaphore_t, timeout: dispatch_time_t) -> usize;
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
    pub fn dispatch_source_create(
        r#type: dispatch_source_type_t,
        handle: usize,
        mask: usize,
        queue: dispatch_queue_t,
    ) -> dispatch_source_t;
    pub fn dispatch_source_set_event_handler(source: dispatch_source_t, handler: dispatch_block_t);
    pub fn dispatch_source_set_event_handler_f(
        source: dispatch_source_t,
        handler: dispatch_function_t,
    );
    pub fn dispatch_source_set_cancel_handler(source: dispatch_source_t, handler: dispatch_block_t);
    pub fn dispatch_source_set_cancel_handler_f(
        source: dispatch_source_t,
        handler: dispatch_function_t,
    );
    pub fn dispatch_source_cancel(source: dispatch_source_t);
    pub fn dispatch_source_testcancel(source: dispatch_source_t) -> usize;
    pub fn dispatch_source_get_handle(source: dispatch_source_t) -> usize;
    pub fn dispatch_source_get_mask(source: dispatch_source_t) -> usize;
    pub fn dispatch_source_get_data(source: dispatch_source_t) -> usize;
    pub fn dispatch_source_merge_data(source: dispatch_source_t, value: usize);
    pub fn dispatch_source_set_timer(
        source: dispatch_source_t,
        start: dispatch_time_t,
        interval: u64,
        leeway: u64,
    );
    pub fn dispatch_source_set_registration_handler(
        source: dispatch_source_t,
        handler: dispatch_block_t,
    );
    pub fn dispatch_source_set_registration_handler_f(
        source: dispatch_source_t,
        handler: dispatch_function_t,
    );

    pub fn dispatch_time(when: dispatch_time_t, delta: i64) -> dispatch_time_t;
    pub fn dispatch_walltime(when: *const libc::timespec, delta: i64) -> dispatch_time_t;

    pub fn dispatch_workloop_create(label: *const c_char) -> dispatch_workloop_t;
    pub fn dispatch_workloop_create_inactive(label: *const c_char) -> dispatch_workloop_t;
    pub fn dispatch_workloop_set_autorelease_frequency(
        workloop: dispatch_workloop_t,
        frequency: dispatch_autorelease_frequency_t,
    );
    // TODO: dispatch_workloop_set_os_workgroup
}
