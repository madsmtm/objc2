use objc2_foundation::{ns_string, NSString};

fn main() {
    struct SyncString(&'static NSString);

    unsafe impl Sync for SyncString {}

    static STRING: SyncString = SyncString(ns_string!("abc"));
}
