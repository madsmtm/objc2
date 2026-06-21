#[cfg(feature = "NSMapTable")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn NSFreeMapTable(table: *mut crate::NSMapTable) {
    extern "C-unwind" {
        fn NSFreeMapTable(table: *mut crate::NSMapTable);
    }
    unsafe { NSFreeMapTable(table) }
}

// TODO: Add `-[NSKeyedUnarchiverDelegate unarchiver:didDecodeObject:]`
