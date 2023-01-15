# Objective-C header translator

For use in making `icrate`. Run using:

```console
cargo run --bin header-translator
```


## SDKs

Make sure you have the same XCode version installed as the one documented in [`crates/icrate/README.md`](../icrate/README.md).

If you have the SDK stored in a different directory, you can specify that as the first argument:

```console
cargo run --bin header-translator -- /Applications/Xcode_new.app/Contents/Developer
```

We do not redistribute SDKs, to hopefully avoid a license violation. You should download XCode (which contain the SDKs) yourself from [Apple's website](https://developer.apple.com/download/all/?q=xcode) (requires an Apple ID).


## Test `icrate`'s feature setup

`header-translator` emits a bunch of features to conditionally enable classes.

If you're working on improving this, you should run the `check_icrate_features` tool to (somewhat) ensure that your changes still compile.

```console
cargo run --bin=check_icrate_features
```
