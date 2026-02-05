#![cfg(target_vendor = "apple")]
use std::boxed::Box;
// The test is very Apple centric
use std::ffi::c_void;
use std::string::{String, ToString};
use std::vec::Vec;

use objc2::{define_class, extern_methods, msg_send, ClassType};
use objc2_foundation::NSObject;

/// Test an exception unwinding from Objective-C into Rust.
#[test]
#[cfg(feature = "exception")]
#[cfg_attr(feature = "catch-all", ignore = "catch-all interferes with our catch")]
fn array_exception() {
    use objc2::{available, rc::Retained};
    use objc2_foundation::{NSArray, NSException};

    // Foreign backtraces seem completely broken on macOS 10.12? It returns:
    // "___CFSortIndexesNMerge",
    // "AssociationsManager::_map",
    // "_CFBitVectorFlipBitAtIndex",
    //
    // Which is definitely not what we're calling!
    if !available!(macos = 11.0) {
        // Unsure of the exact version
        return;
    }

    #[no_mangle]
    #[inline(never)]
    fn array_exception_via_msg_send() {
        let arr = NSArray::<NSObject>::new();
        let _: Retained<NSObject> = unsafe { msg_send![&arr, objectAtIndex: 0usize] };
    }
    let expected_msg_send = [
        "__exceptionPreprocess",
        "objc_exception_throw",
        "CFArrayApply",
        #[cfg(debug_assertions)]
        "<(usize,) as objc2::encode::EncodeArguments>::__invoke::<*mut objc2::runtime::nsobject::NSObject>",
        #[cfg(debug_assertions)]
        "objc2::runtime::message_receiver::msg_send_primitive::send::<(usize,), *mut objc2::runtime::nsobject::NSObject>",
        #[cfg(debug_assertions)]
        "<*mut objc2::runtime::anyobject::AnyObject as objc2::runtime::message_receiver::MessageReceiver>::send_message::<(usize,), *mut objc2::runtime::nsobject::NSObject>",
        #[cfg(debug_assertions)]
        "<objc2::__macros::method_family::MethodFamily<6u8> as objc2::__macros::msg_send::retained::MsgSend<&objc2::rc::retained::Retained<objc2_foundation::generated::__NSArray::NSArray<objc2::runtime::nsobject::NSObject>>, objc2::rc::retained::Retained<objc2::runtime::nsobject::NSObject>>>::send_message::<(usize,)>",
        "array_exception_via_msg_send",
    ];

    #[no_mangle]
    #[inline(never)]
    fn array_exception_via_extern_methods() {
        let arr = NSArray::<NSObject>::new();
        let _ = arr.objectAtIndex(0);
    }
    let expected_extern_methods = [
        "__exceptionPreprocess",
        "objc_exception_throw",
        "CFArrayApply",
        #[cfg(debug_assertions)]
        "<(usize,) as objc2::encode::EncodeArguments>::__invoke::<*mut objc2::runtime::nsobject::NSObject>",
        #[cfg(debug_assertions)]
        "objc2::runtime::message_receiver::msg_send_primitive::send::<(usize,), *mut objc2::runtime::nsobject::NSObject>",
        #[cfg(debug_assertions)]
        "<*mut objc2::runtime::anyobject::AnyObject as objc2::runtime::message_receiver::MessageReceiver>::send_message::<(usize,), *mut objc2::runtime::nsobject::NSObject>",
        #[cfg(debug_assertions)]
        "<objc2::__macros::method_family::MethodFamily<6u8> as objc2::__macros::msg_send::retained::MsgSend<&objc2_foundation::generated::__NSArray::NSArray<objc2::runtime::nsobject::NSObject>, objc2::rc::retained::Retained<objc2::runtime::nsobject::NSObject>>>::send_message::<(usize,)>",
        #[cfg(debug_assertions)]
        "<objc2_foundation::generated::__NSArray::NSArray<objc2::runtime::nsobject::NSObject>>::objectAtIndex",
        "array_exception_via_extern_methods",
    ];

    for (fnptr, expected) in [
        (
            array_exception_via_msg_send as fn(),
            &expected_msg_send as &[_],
        ),
        (array_exception_via_extern_methods, &expected_extern_methods),
    ] {
        let res = objc2::exception::catch(fnptr);
        let exc = res.unwrap_err().unwrap();
        let exc = exc.downcast::<NSException>().unwrap();

        let symbols = extract_nsexception_symbols(exc.callStackSymbols());

        // No debug info available, such as when using `--release`.
        // if symbols[3] == "__mh_execute_header" {
        //     continue;
        // }

        for (expected, actual) in expected.iter().zip(&symbols) {
            let actual = remove_crate_id(actual);
            assert_eq!(actual, *expected, "{symbols:#?}");
        }

        if symbols.len() < expected.len() {
            panic!("did not find enough symbols: {symbols:#?}");
        }
    }
}

define_class!(
    #[unsafe(super = NSObject)]
    struct Thrower;

    impl Thrower {
        #[unsafe(method(backtrace))]
        fn __backtrace() -> *mut c_void {
            let backtrace = backtrace::Backtrace::new();
            Box::into_raw(Box::new(backtrace)).cast()
        }
    }
);

impl Thrower {
    extern_methods!(
        #[unsafe(method(backtrace))]
        fn backtrace() -> *mut c_void;
    );
}

/// Test an exception unwinding from Rust through Objective-C and into Rust.
#[test]
#[cfg_attr(feature = "catch-all", ignore = "catch-all changes the backtrace")]
fn capture_backtrace() {
    #[no_mangle]
    #[inline(never)]
    fn rust_backtrace_via_msg_send() -> backtrace::Backtrace {
        let ptr: *mut c_void = unsafe { msg_send![Thrower::class(), backtrace] };
        *unsafe { Box::from_raw(ptr.cast()) }
    }
    let expected_msg_send: &[_] = &[
        "<tests::backtrace::Thrower>::__backtrace",
        #[cfg(debug_assertions)]
        "objc2::__macros::define_class::thunk::_::thunk::<<tests::backtrace::Thrower as objc2::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2::runtime::anyclass::AnyClass, *mut core::ffi::c_void, objc2::__macros::method_family::MethodFamily<6u8>, tests::backtrace::Thrower>",
        #[cfg(debug_assertions)]
        "<() as objc2::encode::EncodeArguments>::__invoke::<*mut core::ffi::c_void>",
        #[cfg(debug_assertions)]
        "objc2::runtime::message_receiver::msg_send_primitive::send::<(), *mut core::ffi::c_void>",
        #[cfg(debug_assertions)]
        "<*mut objc2::runtime::anyobject::AnyObject as objc2::runtime::message_receiver::MessageReceiver>::send_message::<(), *mut core::ffi::c_void>",
        #[cfg(debug_assertions)]
        "<objc2::__macros::method_family::MethodFamily<6u8> as objc2::__macros::msg_send::retained::MsgSend<&objc2::runtime::anyclass::AnyClass, *mut core::ffi::c_void>>::send_message::<()>",
        "rust_backtrace_via_msg_send",
    ];

    #[no_mangle]
    #[inline(never)]
    fn rust_backtrace_via_extern_methods() -> backtrace::Backtrace {
        let ptr = Thrower::backtrace();
        *unsafe { Box::from_raw(ptr.cast()) }
    }
    let expected_extern_methods: &[_] = &[
        "<tests::backtrace::Thrower>::__backtrace",
        #[cfg(debug_assertions)]
        "objc2::__macros::define_class::thunk::_::thunk::<<tests::backtrace::Thrower as objc2::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2::runtime::anyclass::AnyClass, *mut core::ffi::c_void, objc2::__macros::method_family::MethodFamily<6u8>, tests::backtrace::Thrower>",
        #[cfg(debug_assertions)]
        "<() as objc2::encode::EncodeArguments>::__invoke::<*mut core::ffi::c_void>",
        #[cfg(debug_assertions)]
        "objc2::runtime::message_receiver::msg_send_primitive::send::<(), *mut core::ffi::c_void>",
        #[cfg(debug_assertions)]
        "<*mut objc2::runtime::anyobject::AnyObject as objc2::runtime::message_receiver::MessageReceiver>::send_message::<(), *mut core::ffi::c_void>",
        #[cfg(debug_assertions)]
        "<objc2::__macros::method_family::MethodFamily<6u8> as objc2::__macros::msg_send::retained::MsgSend<&objc2::runtime::anyclass::AnyClass, *mut core::ffi::c_void>>::send_message::<()>",
        #[cfg(debug_assertions)]
        "<tests::backtrace::Thrower>::backtrace",
        "rust_backtrace_via_extern_methods",
    ];

    for (backtrace, expected) in [
        (rust_backtrace_via_msg_send(), expected_msg_send),
        (rust_backtrace_via_extern_methods(), expected_extern_methods),
    ] {
        let symbols: Vec<_> = backtrace
            .frames()
            .iter()
            .flat_map(|frame| frame.symbols())
            .map(|symbol| {
                symbol
                    .name()
                    .map(|name| name.to_string())
                    .unwrap_or_default()
            })
            .skip_while(|name| !name.contains("__backtrace"))
            .collect();

        // No debug info available, such as when using `--release`.
        if backtrace.frames()[0].symbols()[0]
            .name()
            .unwrap()
            .to_string()
            == "__mh_execute_header"
        {
            continue;
        }

        for (expected, actual) in expected.iter().zip(&symbols) {
            let actual = remove_crate_id(actual);
            // FIXME: `backtrace` includes an extra `_` in symbols when
            // figuring out names from symbol names instead of debug info.
            let actual = actual.strip_prefix("_").unwrap_or(&actual);
            assert_eq!(actual, *expected, "{symbols:#?}");
        }

        if symbols.len() < expected.len() {
            panic!("did not find enough symbols: {backtrace:?}\n{symbols:?}");
        }
    }
}

fn extract_nsexception_symbols<T: std::string::ToString>(
    data: impl IntoIterator<Item = T>,
) -> Vec<String> {
    data.into_iter()
        .map(|item| {
            // `callStackSymbols` contains more data than just symbols, it
            // also contains the address and such. Here, we extract just the
            // symbol name.
            let item = item.to_string();
            let item = item
                .rsplit_once(" + ")
                .map(|(symbol, _offset)| symbol)
                .unwrap_or(&item);
            let symbol = item.rsplit_once(' ').unwrap().1;

            // Objective-C and Rust have different mangling schemes, and while
            // some of Apple's software (mostly the ones based on LLVM, such
            // as Xcode or `c++-filt`) understands Rust's v0 mangling scheme,
            // the built-in demangler doesn't seem to.
            //
            // So we demangle all symbols manually.
            rustc_demangle::demangle(symbol).to_string()
        })
        .collect()
}

#[test]
fn test_extract_nsexception_symbols() {
    let inp = [
        "0   CoreFoundation                      0x0000000184d5fae0 __exceptionPreprocess + 176",
        "1   libobjc.A.dylib                     0x0000000184822b90 objc_exception_throw + 88",
        "18  tests-64165e8a00f54cd5              0x00000001023be98c _RNCNvCsRhuSZWqZya_4test8run_test0B3_ + 908",
    ];
    let result = [
        "__exceptionPreprocess",
        "objc_exception_throw",
        "test[a0285ec8061feec]::run_test::{closure#0}",
    ];
    assert_eq!(extract_nsexception_symbols(inp), result);
}

/// Remove the crate ID present in demangled symbols, like `xyz` in `foo[xyz]`.
///
/// These are present because there can be multiple crates with the same name,
/// but we don't want to worry about them in these tests.
fn remove_crate_id(symbol: &str) -> String {
    let mut symbol = symbol.to_string();
    while let Some(start_idx) = symbol.find('[') {
        if let Some(end_idx) = symbol[start_idx..].find(']') {
            symbol.replace_range(start_idx..(start_idx + end_idx + 1), "");
        } else {
            break;
        }
    }
    symbol
}

#[test]
fn test_remove_crate_id() {
    assert_eq!(
        remove_crate_id("test[a0285ec8061feec]::run_test::{closure#0}"),
        "test::run_test::{closure#0}"
    );
}
