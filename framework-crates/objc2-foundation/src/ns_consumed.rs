extern "C" {
    #[cfg(feature = "NSMapTable")]
    pub fn NSFreeMapTable(table: *mut crate::NSMapTable);
}

// TODO: Add `-[NSKeyedUnarchiverDelegate unarchiver:didDecodeObject:]`
