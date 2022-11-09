//! Test that messages can only be sent to objects.
use objc2::msg_send;

fn main() {
    unsafe { msg_send![1, new] };
}
