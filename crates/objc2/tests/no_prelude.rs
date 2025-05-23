//! Tests macros in a hostile environment (`#![no_implicit_prelude]` and all
//! functions, modules, traits, and types replaced with custom bogus user
//! replacements).
//!
//! Heavy inspiration for this file taken from `objrs`:
//! https://gitlab.com/objrs/objrs/-/blob/b4f6598696b3fa622e6fddce7aff281770b0a8c2/test/src/no_prelude.rs
#![no_implicit_prelude]
#![allow(dead_code, non_camel_case_types)]

extern crate objc2 as new_objc2;

mod core {}
mod std {}
mod libc {}
mod objc2 {}

enum BogusType {}
type u8 = BogusType;
type u16 = BogusType;
type u32 = BogusType;
type u64 = BogusType;
type u128 = BogusType;
type usize = BogusType;

type i8 = BogusType;
type i16 = BogusType;
type i32 = BogusType;
type i64 = BogusType;
type i128 = BogusType;
type isize = BogusType;

type bool = BogusType;
type char = BogusType;
type str = BogusType;

type f32 = BogusType;
type f64 = BogusType;

type Option = BogusType;
type Some = BogusType;
type None = BogusType;

type Result = BogusType;
type Ok = BogusType;
type Err = BogusType;

type Box = BogusType;
type String = BogusType;
type Vec = BogusType;

type drop = BogusType;

type Copy = BogusType;
type Send = BogusType;
type Sized = BogusType;
type Sync = BogusType;
type Drop = BogusType;
type Fn = BogusType;
type FnMut = BogusType;
type FnOnce = BogusType;

type ToOwned = BogusType;
type Clone = BogusType;
type PartialEq = BogusType;
type PartialOrd = BogusType;
type Eq = BogusType;
type Ord = BogusType;
type AsRef = BogusType;
type AsMut = BogusType;
type Into = BogusType;
type From = BogusType;
type Default = BogusType;
type Hash = BogusType;
type Debug = BogusType;
type Iterator = BogusType;
type Extend = BogusType;
type IntoIterator = BogusType;
type DoubleEndedIterator = BogusType;
type ExactSizeIterator = BogusType;
type SliceConcatExt = BogusType;
type ToString = BogusType;

type PhantomData = BogusType;

// Test begin below this line

struct MyCustomIvars {
    ivars: i32,
}

new_objc2::define_class!(
    #[unsafe(super(new_objc2::runtime::NSObject))]
    #[ivars = MyCustomIvars]
    struct CustomObject;

    impl CustomObject {
        #[unsafe(method(a))]
        fn _a() {}

        #[unsafe(method_id(b))]
        fn _b() -> new_objc2::rc::Retained<CustomObject> {
            ::core::unimplemented!()
        }
    }
);

impl CustomObject {
    new_objc2::extern_methods!(
        #[unsafe(method(a))]
        fn a();

        #[unsafe(method(b))]
        fn b(&self);
    );
}

new_objc2::extern_class!(
    #[unsafe(super(new_objc2::runtime::NSObject))]
    #[name = "NSObject"]
    struct NSObject2;
);

new_objc2::extern_protocol!(
    #[allow(clippy::missing_safety_doc)]
    unsafe trait CustomProtocol {
        #[unsafe(method(c))]
        fn c(&self);
    }
);

#[test]
fn test_selector() {
    let _sel = new_objc2::sel!(abc);
    let _sel = new_objc2::sel!(abc:def:);
}

#[test]
fn test_class() {
    let _class = new_objc2::class!(NSObject);
}

fn test_msg_send(obj: &CustomObject) {
    let superclass = obj.class().superclass().unwrap();
    let _: () = unsafe { new_objc2::msg_send![obj, a] };
    let _: () = unsafe { new_objc2::msg_send![obj, a: obj, b: obj] };
    let _: () = unsafe { new_objc2::msg_send![super(obj), a] };
    let _: () = unsafe { new_objc2::msg_send![super(obj), a: obj, b: obj] };
    let _: () = unsafe { new_objc2::msg_send![super(obj, superclass), a] };
    let _: () = unsafe { new_objc2::msg_send![super(obj, superclass), a: obj, b: obj] };

    let _: new_objc2::rc::Retained<new_objc2::runtime::AnyObject> =
        unsafe { new_objc2::msg_send![obj, a] };
    let _: new_objc2::__macro_helpers::Option<
        new_objc2::rc::Retained<new_objc2::runtime::AnyObject>,
    > = unsafe { new_objc2::msg_send![obj, a] };
    let _: new_objc2::rc::Retained<new_objc2::runtime::AnyObject> =
        unsafe { new_objc2::msg_send![obj, a: obj, b: obj] };
}
