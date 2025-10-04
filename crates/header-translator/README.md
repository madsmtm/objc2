# Objective-C header translator

For use in making framework crates. Run using:

```console
cargo run --bin header-translator --release all
```

Or to only generate for a single crate, do something like:

```console
cargo run --bin header-translator --release objc2-foundation
cargo run --bin header-translator --release CoreFoundation
cargo run --bin header-translator --release MY_FRAMEWORK
```


## SDKs

Make sure you have the same Xcode version installed as the one documented in [`crates/objc2/src/lib.rs`](../objc2/src/lib.rs).

If you use a different operating system than macOS, or have multiple SDKs installed, you can specify the developer directory with `DEVELOPER_DIR`.

Note that in this case, you will likely need to use a newer `libclang`, such as [the one distributed with Swift `5.7.2`](https://github.com/apple/llvm-project/tree/swift-5.7.2-RELEASE). You can use a different version as follows (details might vary between operating systems):

```console
export DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer
export LIBCLANG_PATH=/path/to/custom/installation/usr/lib/libclang.dylib
export CPATH=/path/to/custom/installation/usr/lib/clang/14.0.0/include/
cargo run --bin header-translator --release all
```

We do not redistribute SDKs, to hopefully avoid a license violation. You should download Xcode (which contain the SDKs) yourself from [Apple's website](https://developer.apple.com/download/all/?q=xcode) (requires an Apple ID).


## Test feature setup

`header-translator` emits a bunch of features to conditionally enable classes.

If you're working on improving this, you should run the `check_framework_features` tool to (somewhat) ensure that your changes still compile.

```console
cargo run --bin check_framework_features
```


## Upgrading Xcode

The support policy for Xcode versions is [documented in `objc2/src/lib.rs`](../objc2/src/lib.rs). Upgrading Xcode is usually done by `@madsmtm`, in a PR for visibility. The process is basically to run with developer directory pointing to the new Xcode (as discussed above), fix issues that arise from this, and document changed APIs (see also [#338]).

For frameworks that are marked as automatically safe, extra vigilance is required to ensure that no new unsafe APIs are introduced.

[#338]: https://github.com/madsmtm/objc2/issues/338


## Adding a new framework crate

To add a new framework crate, create a new empty crate in [`framework-crates`](../../framework-crates/), along with a bare-bones `translation-config.toml`. You can use this as a template:
```toml
framework = "XXX"
crate = "objc2-xxx"
required-crates = ["objc2", "objc2-foundation"]
macos = "XXX"
maccatalyst = "XXX"
ios = "XXX"
tvos = "XXX"
watchos = "XXX"
visionos = "XXX"
```

The version numbers should be filled in from the front page of Apple's documentation for the given framework. Once this is done, you should be able to run `header-translator`, and have it generate the framework.

You will need to further modify this `translation-config.toml` with `skipped` items until `header-translator` produces no more `ERROR` logging messages, and it compiles successfully.

Feel free to open a half-finished PR if you need assistance.


## Data enrichment

The `translation-config.toml` file describes various tweaks that we need to do because our header translation is incomplete in some areas. This includes skipping items, changing the nullability and renaming items, read through [src/config.rs](./src/config.rs) for the full functionality.

A common pattern here is to skip an item in the translation config, and then add the manual definition in a file that you simply include in `lib.rs` with `mod my_file;`. If you do this, please specify `custom-lib-rs = true` in the translation config, otherwise your changes will be overwritten the next time you run the translator.

Manual modifications to `Cargo.toml` can be done by adding a `Cargo.modified.toml` file, and including the keys that you want modified.

See [objc2-foundation](../../framework-crates/objc2-foundation) for a good example of these files at work.


### Safety

Even if our header translation was perfect, we'd still need a way to enrich the generated data, since C headers have no way to describe which methods are safe and which are not! See [this document](../objc2/src/topics/frameworks_soundness.md) for details on this.

Methods/functions/properties can be marked safe with:

```toml
# Mark specific function/method as safe
fn.my_method.unsafe = false
class.MyClass.methods.myProperty.unsafe = false
protocol.MyProtocol.methods."myMethodWithArg:".unsafe = false # Uses selector name
# Mark all methods as safe
class.MyClass.unsafe = false
protocol.MyClass.unsafe = false
```

Everything is unsafe by default, but the default for the entire framework can be changed with:

```toml
# Automatically mark properties, methods and free functions as safe.
unsafe-default-safety.documentation-is-reviewed = true
```

The safety is conservative, and is determined by inspecting the signature of the method/function, so you probably still have to change the safety of some methods that fall outside common patterns.

You can opt-in to some stricter heuristics with the following:

```toml
# Heuristic, don't mark as safe if method contains a name like `index` or `offset`.
unsafe-default-safety.bounds-checked-internally = false
```

Note: It is _not_ considered a breaking change for a method to be marked safe, so marking things as safe can be done in a minor/patch version!
