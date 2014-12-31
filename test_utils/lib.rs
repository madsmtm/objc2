#![crate_name = "objc_test_utils"]
#![crate_type = "lib"]

extern crate libc;

use libc::c_void;

#[allow(improper_ctypes)]
#[link(name="block_utils", kind="static")]
extern {
    pub fn get_int_block() -> *mut c_void;
    pub fn get_int_block_with(i: int) -> *mut c_void;
    pub fn get_add_block() -> *mut c_void;
    pub fn get_add_block_with(i: int) -> *mut c_void;
    pub fn invoke_int_block(block: *mut c_void) -> int;
    pub fn invoke_add_block(block: *mut c_void, a: int) -> int;
}
