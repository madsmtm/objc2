use crate::common::*;
#[cfg(feature = "apple")]
use crate::CoreFoundation::*;

pub type CFByteOrder = CFIndex;
// NOTE: CFIndex is c_long_long on __LLP64__ / Windows 64-bit (doesn't matter for us)
pub type CFIndex = c_long;
pub type CFPropertyListRef = CFTypeRef;
pub type CFTypeRef = *mut c_void;

pub const CFByteOrderUnknown: CFByteOrder = 0;
pub const CFByteOrderLittleEndian: CFByteOrder = 1;
pub const CFByteOrderBigEndian: CFByteOrder = 2;

extern_enum!(
    #[underlying(u32)]
    pub enum CGRectEdge {
        CGRectMinXEdge = 0,
        CGRectMinYEdge = 1,
        CGRectMaxXEdge = 2,
        CGRectMaxYEdge = 3,
    }
);

// Kernel definitions
// TODO: move these into `Kernel` someday
pub type cpu_type_t = integer_t;
pub type integer_t = c_int;
pub type UniChar = UInt16;
#[cfg(feature = "apple")]
pub type mach_port_t = libc::mach_port_t;

#[cfg(feature = "apple")]
extern_struct!(
    #[encoding_name("?")]
    pub struct CFRunLoopSourceContext1 {
        pub version: CFIndex,
        pub info: *mut c_void,
        pub retain: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        pub release: Option<unsafe extern "C" fn(*mut c_void)>,
        pub copyDescription: Option<unsafe extern "C" fn(*mut c_void) -> CFStringRef>,
        pub equal: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> Boolean>,
        pub hash: Option<unsafe extern "C" fn(*mut c_void) -> CFHashCode>,
        pub getPort: Option<unsafe extern "C" fn(*mut c_void) -> mach_port_t>,
        pub perform: Option<
            unsafe extern "C" fn(*mut c_void, CFIndex, CFAllocatorRef, *mut c_void) -> *mut c_void,
        >,
    }
);

#[cfg(feature = "apple")]
extern_fn!(
    pub unsafe fn CFMachPortCreateWithPort(
        allocator: CFAllocatorRef,
        port_num: mach_port_t,
        callout: CFMachPortCallBack,
        context: *mut CFMachPortContext,
        should_free_info: *mut Boolean,
    ) -> CFMachPortRef;
);

#[cfg(feature = "apple")]
extern_fn!(
    pub unsafe fn CFMachPortGetPort(port: CFMachPortRef) -> mach_port_t;
);

#[cfg(feature = "apple")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _acl {
    _unused: [u8; 0],
}
#[cfg(feature = "apple")]
pub type acl_t = *mut _acl;

#[cfg(feature = "apple")]
extern_fn!(
    pub unsafe fn CFFileSecurityCopyAccessControlList(
        file_sec: CFFileSecurityRef,
        access_control_list: *mut acl_t,
    ) -> Boolean;
);

#[cfg(feature = "apple")]
extern_fn!(
    pub unsafe fn CFFileSecuritySetAccessControlList(
        file_sec: CFFileSecurityRef,
        access_control_list: acl_t,
    ) -> Boolean;
);
