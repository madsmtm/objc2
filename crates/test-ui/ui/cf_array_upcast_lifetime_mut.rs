//! Test that upcasting mutable CoreFoundation collections requires `'static`.
use objc2_core_foundation::{CFArray, CFMutableArray, CFRetained, CFType};

struct Foo<'a>(&'a u32);

fn get_foo_array<'a>(_foo: &'a u32) -> CFRetained<CFMutableArray<Foo<'a>>> {
    unimplemented!()
}

fn main() {
    let foo = 42;
    let arr = get_foo_array(&foo);

    // This should work.
    let _: &CFArray<Foo<'_>> = arr.as_ref();
    let _: &CFArray<Foo<'_>> = &arr;

    // This must fail.
    let _: &CFType = arr.as_ref();
}
