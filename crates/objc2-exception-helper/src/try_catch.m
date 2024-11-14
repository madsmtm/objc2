// Don't include any headers, cross compilation is difficult to set up
// properly in such situations.

/// Users are linking to `libobjc`, so this should be available.
///
/// See <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#arc-runtime-objc-retain>.
id objc_retain(id value);

/// C has no name resolution and will conflict if multiple versions of this
/// crate is present, so we make sure to version this symbol.
///
/// Return `unsigned char` since it is guaranteed to be `u8` on all platforms.
unsigned char objc2_exception_helper_0_1_try_catch(void (*f)(void *), void *context, id *error) {
    @try {
        f(context);
        return 0;
    } @catch (id exception) {
        // The exception is retained while inside the `@catch` block, but is
        // not guaranteed to be so outside of it; so hence we must do it here!
        //
        // Code throwing an exception know that they don't hold sole access to
        // that object any more, so it's certainly safe to retain.
        *error = objc_retain(exception);
        // NOTE: The above `retain` can throw, so an extra landing pad will be
        // inserted here for that case.
        return 1;
    }
    // NOTE: @catch(...) exists, but it only works reliably in 64-bit. On
    // 32-bit, an exception may continue past that frame (I think).
    // https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Exceptions/Articles/Exceptions64Bit.html
    //
    // Given that there is no way for us to reliably catch all Rust/C++/...
    // exceptions here, the best choice is to consistently let them continue
    // unwinding up the stack frame.
    //
    // Besides, not re-throwing Rust exceptions is not currently supported,
    // so we'd just get an `abort` if we _did_ try to handle it:
    // https://github.com/rust-lang/rust/blob/80d0d927d5069b67cc08c0c65b48e7b6e0cdeeb5/library/std/src/panicking.rs#L51-L58
    //
    // In any case, this _is_ the behaviour we want, to just catch Objective-C
    // exceptions, C++/Rust/... exceptions are better handled with
    // `std::panic::catch_unwind`.
}
