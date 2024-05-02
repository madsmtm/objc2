# Objective-C header translator

For use in making framework crates. Run using:

```console
cargo run --bin header-translator
```


## SDKs

Make sure you have the same XCode version installed as the one documented in [`crates/objc2/src/topics/about_generated/README.md`](../objc2/src/topics/about_generated/README.md).

If you use a different operating system than macOS, or have multiple SDKs installed, you can specify the directory as the first argument:

```console
cargo run --bin header-translator -- /Applications/Xcode.app/Contents/Developer
```

Note that you will likely need to use a newer `libclang`, such as [the one distributed with Swift `5.7.2`](https://github.com/apple/llvm-project/tree/swift-5.7.2-RELEASE). You can use a different version as follows (details might vary between operating systems):

```console
export LIBCLANG_PATH=/path/to/custom/installation/usr/lib/libclang.dylib
export CPATH=/path/to/custom/installation/usr/lib/clang/14.0.0/include/
cargo run --bin header-translator -- /Applications/Xcode_new.app/Contents/Developer
```

We do not redistribute SDKs, to hopefully avoid a license violation. You should download XCode (which contain the SDKs) yourself from [Apple's website](https://developer.apple.com/download/all/?q=xcode) (requires an Apple ID).


## Test feature setup

`header-translator` emits a bunch of features to conditionally enable classes.

If you're working on improving this, you should run the `check_framework_features` tool to (somewhat) ensure that your changes still compile.

```console
cargo run --bin check_framework_features
```


## Data enrichment

The `translation-config.toml` file describes various tweaks that we need to do because our header translation is incomplete in some areas.

However, even if our header translation was perfect, we still need a way to enrich the generated data, since C headers have no way to describe which methods are safe, or mutable, and which are not!


### What is required for a method to be safe?

This is a longer discussion, but the following guidelines will get you far. Do
not be afraid of opening an issue or PR that discusses the safety of a
specific API!

1. The method must not take a raw pointer; one could trivially pass
    `ptr::invalid()` and cause UB with that.
2. Any extra requirements that the method states in its documentation must be
    upheld. For example, a method may declare in its documentation that some
    property must be something specific in relation to another property. Since
    we don't know whether this is upheld, the method is not safe.

    Note: This is the hardest part; determining for sure if a given method is
    safe or not!
3. If the method can throw an exception if provided with invalid inputs, it is
    not safe. Consider declaring a helper method that checks the preconditions
    first!
4. Beware of `Mutable` classes (e.g. `NSMutableString`); these usually need to
    be passed as `&mut T`, or operate on `&mut self`.

Note: It is _not_ considered a breaking change for a method to be marked safe,
so such an improvement can be made in a minor version!
