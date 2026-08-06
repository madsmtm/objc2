//! Test that upcasting CoreFoundation collections requires `'static`.
use objc2_core_foundation::{CFArray, CFRetained, CFType};

struct Foo<'a>(&'a u32);

fn get_foo_array<'a>(_foo: &'a u32) -> CFRetained<CFArray<Foo<'a>>> {
    unimplemented!()
}

fn main() {
    let foo = 42;
    let arr = get_foo_array(&foo);

    let _: &CFType = arr.as_ref();
}
