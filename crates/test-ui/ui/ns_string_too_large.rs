//! Ensure that we can't create a static `ns_string!` with a too large string.
use objc2_foundation::ns_string;

fn main() {
    let _ =
        ns_string!(unsafe { std::str::from_utf8_unchecked(&[b'A'; (isize::MAX / 2) as usize]) });
}
