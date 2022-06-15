//! Test compiler output of msg_send_id when ownership is not specified.
//!
//! Don't think it's really possible for us to improve this diagnostic?
use objc2::msg_send_id;
use objc2::runtime::Object;

fn main() {
    let obj: &Object;
    let _: &Object = &*unsafe { msg_send_id![obj, description].unwrap() };
}
