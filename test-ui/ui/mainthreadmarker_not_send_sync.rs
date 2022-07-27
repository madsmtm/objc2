//! Test that MainThreadMarker is neither Send nor Sync.
use objc2::foundation::MainThreadMarker;

fn needs_sync<T: Sync>() {}
fn needs_send<T: Send>() {}

fn main() {
    needs_sync::<MainThreadMarker>();
    needs_send::<MainThreadMarker>();
}
