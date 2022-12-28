use icrate::Foundation::NSArray;
use objc2::runtime::Object;

fn needs_sync<T: ?Sized + Sync>() {}
fn needs_send<T: ?Sized + Send>() {}

fn main() {
    needs_sync::<NSArray<Object>>();
    needs_send::<NSArray<Object>>();
}
