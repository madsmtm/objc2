use icrate::Foundation::{NSArray, NSMutableArray, NSObject};

fn needs_sync<T: ?Sized + Sync>() {}
fn needs_send<T: ?Sized + Send>() {}

fn main() {
    needs_sync::<NSArray<NSObject>>();
    needs_send::<NSArray<NSObject>>();
    needs_sync::<NSMutableArray<NSObject>>();
    needs_send::<NSMutableArray<NSObject>>();
}
