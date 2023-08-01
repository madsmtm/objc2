use libc::c_void;

#[cfg(feature = "apple")]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FSRef {
    _hidden: [u8; 80],
}

#[cfg(feature = "apple")]
pub type acl_t = *mut c_void;
