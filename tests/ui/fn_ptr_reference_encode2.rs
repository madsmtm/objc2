//! Extra test for `fn_ptr_reference_encode`
//! (They fail at different compilation passes).
use objc2::Encode;

fn main() {
    let encoding = <extern "C" fn(&i32) as Encode>::ENCODING;
}
