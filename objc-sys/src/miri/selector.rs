use core::ptr;
use std::boxed::Box;
use std::ffi::CStr;
use std::lazy::SyncLazy;
use std::os::raw::c_char;
use std::sync::Mutex;
use std::vec::Vec;

use crate::objc_selector;

static SELECTORS: SyncLazy<Mutex<Vec<&'static CStr>>> = SyncLazy::new(|| Mutex::new(Vec::new()));

#[no_mangle]
pub extern "C" fn sel_getName(sel: *const objc_selector) -> *const c_char {
    // 1-indexed, 0 is NULL selector
    if let Some(addr) = sel.addr().checked_sub(1) {
        let selectors = SELECTORS.lock().unwrap();
        if let Some(&sel) = selectors.get(addr) {
            sel.as_ptr()
        } else {
            // panic!("Unregistered selector")
            ptr::null()
        }
    } else {
        // panic!("Null selector")
        ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn sel_registerName(name: *const c_char) -> *const objc_selector {
    if name.is_null() {
        // panic!("Null name")
        return ptr::null();
    }
    let bytes = unsafe { CStr::from_ptr(name) };
    let mut selectors = SELECTORS.lock().unwrap();
    for (i, &value) in selectors.iter().enumerate() {
        if value == bytes {
            // 1-indexed, 0 is NULL selector
            return ptr::invalid(i + 1);
        }
    }

    // Not found, create new entry
    let sel = Box::leak(Box::from(bytes));
    selectors.push(sel);
    ptr::invalid(selectors.len())
}
