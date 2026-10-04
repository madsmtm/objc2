# How to add custom Objective-C bindings

Sometimes, the framework crates don't have specific API that you might need.
This could be for many reasons, such as if the API was introduced recently in
a newer Xcode version than supported, you're interfacing with a third-party
Objective-C library, or the API is simply not yet supported for technical
reasons.

There might also be cases where introducing a dependency on the framework
crate is undesirable, such [as in the `webbrowser` crate][browser-no-dep].

In those cases, `objc2` can still be used, you just have to drop down into
lower-level APIs.

For example, `objc2-screen-capture-kit` used to not expose the
[`-[SCStream addStreamOutput:type:sampleHandlerQueue:error:]`][add-stream]
method. To use this regardless, you could define a helper method that calls
[`msg_send!`][crate::msg_send]:

```rust,ignore
unsafe fn addStreamOutput_type_sampleHandlerQueue_error(
    stream: &SCStream,
    output: &ProtocolObject<dyn SCStreamOutput>,
    r#type: SCStreamOutputType,
    sample_handler_queue: Option<&DispatchQueue>,
) -> Result<(), Retained<NSError>> {
    // SAFETY: The argument and return types are correct.
    unsafe { msg_send![stream, addStreamOutput: output, type: r#type, sampleHandlerQueue: sample_handler_queue, error: _] }
}
```

Note that now you're responsible for ensuring that the signature is correct,
e.g. the macro wouldn't be able to know that writing `output: &NSString` would
be wrong (hence this is more `unsafe` than using the framework crates).

[browser-no-dep]: https://github.com/amodm/webbrowser-rs/blob/13fdff33e967dd160581ce8ac45a8c3588e8eb59/src/ios.rs#L8-L35
[add-stream]: https://developer.apple.com/documentation/screencapturekit/scstream/addstreamoutput(_:type:samplehandlerqueue:)?language=objc
