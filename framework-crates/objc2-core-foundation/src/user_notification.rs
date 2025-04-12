#![allow(non_snake_case)]
use crate::{CFIndex, CFOptionFlags};

pub extern "C" fn CFUserNotificationCheckBoxChecked(i: CFIndex) -> CFOptionFlags {
    (1usize << (8 + i)) as CFOptionFlags
}

pub extern "C" fn CFUserNotificationSecureTextField(i: CFIndex) -> CFOptionFlags {
    (1usize << (16 + i)) as CFOptionFlags
}

pub extern "C" fn CFUserNotificationPopUpSelection(n: CFIndex) -> CFOptionFlags {
    (n << 24) as CFOptionFlags
}
