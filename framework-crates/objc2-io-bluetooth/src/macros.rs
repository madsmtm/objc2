#![allow(unused_macros, unused_imports)]

macro_rules! err_system {
    ($x:expr) => {
        ((($x as u32) & 0x3f) as i32) << 26
    };
}

macro_rules! err_sub {
    ($x:expr) => {
        ($x & 0xfff) << 14
    };
}

macro_rules! iokit_bluetooth_err {
    ($return:expr) => {
        // (sys_iokit | sub_iokit_bluetooth | return)
        (err_system!(0x38) | err_sub!(8) | $return) as u32
    };
}

pub(crate) use err_sub;
pub(crate) use err_system;
pub(crate) use iokit_bluetooth_err;
