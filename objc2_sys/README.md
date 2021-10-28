# `objc2_sys`

[![Latest version](https://badgen.net/crates/v/objc2_sys)](https://crates.io/crates/objc2_sys)
[![License](https://badgen.net/badge/license/MIT/blue)](../LICENSE.txt)
[![Documentation](https://docs.rs/objc2_sys/badge.svg)](https://docs.rs/objc2_sys/)
[![Apple CI](https://github.com/madsmtm/objc2/actions/workflows/apple.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/apple.yml)
[![GNUStep CI](https://github.com/madsmtm/objc2/actions/workflows/gnustep.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/gnustep.yml)

Raw Rust bindings to core Objective-C runtimes and ABIs.

## Runtime Support

### Apple's [`objc4`](https://opensource.apple.com/source/objc4/)

This is the default on Apple platforms:
```rust , ignore
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos",
))]
```

The supported runtime version (higher versions lets the compiler enable newer
optimizations, at the cost of not supporting older operating systems) can be
chosen using the standard `X_DEPLOYMENT_TARGET` environment variables:
- macOS: `MACOSX_DEPLOYMENT_TARGET`
  - Default: `10.7` ([same as Rust](https://github.com/rust-lang/rust/blob/1.56.0/compiler/rustc_target/src/spec/apple_base.rs#L67))
  - Minimum: `10.7`
- iOS: `IPHONEOS_DEPLOYMENT_TARGET`
  - Default: `7.0` ([same as Rust](https://github.com/rust-lang/rust/blob/1.56.0/compiler/rustc_target/src/spec/apple_base.rs#L92))
  - Minimum: `5.0` (theoretically)
- tvOS: `TVOS_DEPLOYMENT_TARGET`
  - Default: TODO
  - Minimum: `9.0` (theoretically)
- watchOS: `WATCHOS_DEPLOYMENT_TARGET`
  - Default: TODO
  - Minimum: `1.0` (theoretically)


### Window's [`WinObjC`](https://github.com/microsoft/WinObjC)

This is the default on `#[cfg(target_os = "windows")]`.

This is essentially just [a fork](https://github.com/microsoft/libobjc2) based
on GNUStep's `libobjc2` version 1.8, with very few user-facing changes.


### GNUStep's [`libobjc2`](https://github.com/gnustep/libobjc2)

This is the default on all other systems.

The version can be chosen by setting the standard (used by GNUStep already)
`RUNTIME_VERSION` environment variable to one of the following:
- `gnustep-1.7`
- `gnustep-1.8` (default)
- `gnustep-1.9`
- `gnustep-2.0`
- `gnustep-2.1`

If you wish to force using the GNUStep runtime on Apple or Windows systems,
set the `RUNTIME_VERSION` environment variable to one of the values above.


### Other runtimes

This library will probably only ever support ["Modern"][modern] Objective-C
runtimes, since support for reference-counting primitives like `objc_retain`
and `objc_autoreleasePoolPop` is a vital requirement for most applications.

Just so we're being clear, this rules out the GCC [`libobjc`][gcc-libobjc]
runtime (see [this][gcc-objc-support]), and the [`mulle-objc`] runtime. (But
support for [`ObjFW`] may be added). More information on different runtimes
can be found in GNUStep's [Objective-C Compiler and Runtime FAQ][gnustep-faq].

[modern]: https://en.wikipedia.org/wiki/Objective-C#Modern_Objective-C
[gcc-libobjc]: https://github.com/gcc-mirror/gcc/tree/master/libobjc
[gcc-objc-support]: https://gcc.gnu.org/onlinedocs/gcc/Standards.html#Objective-C-and-Objective-C_002b_002b-Languages
[`mulle-objc`]: https://github.com/mulle-objc/mulle-objc-runtime
[`ObjFW`]: https://github.com/ObjFW/ObjFW
[gnustep-faq]: http://wiki.gnustep.org/index.php/Objective-C_Compiler_and_Runtime_FAQ


## Configuring linking

This crate defines the `links` key in `Cargo.toml` so it's possible to
change the linking to `libobjc`, see [the relevant cargo docs][overriding].

In the future, this crate may vendor the required source code to automatically
build and link to the runtimes. Choosing static vs. dynamic linking here may
also become an option.

[overriding]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#overriding-build-scripts


## Objective-C compiler ABI configuration

Objective-C compilers like `clang` and `gcc` requires configuring the calling
ABI to the runtime you're using:
- `clang` uses the [`-fobjc-runtime`] flag, of which there are a few different
  [options][clang-objc-kinds].
- `gcc` uses the [`-fgnu-runtime` or `-fnext-runtime`][gcc-flags] options.
  Note that Modern Objective-C features are ill supported.

This is relevant if you're building and linking to custom Objective-C sources
in a build script. In the future, this crate may expose build script metadata
to help with selecting these (and other required) flags.

[`-fobjc-runtime`]: https://clang.llvm.org/docs/ClangCommandLineReference.html#cmdoption-clang-fobjc-runtime
[clang-objc-kinds]: https://clang.llvm.org/doxygen/classclang_1_1ObjCRuntime.html#af19fe070a7073df4ecc666b44137c4e5
[gcc-flags]: https://gcc.gnu.org/onlinedocs/gcc/Objective-C-and-Objective-C_002b_002b-Dialect-Options.html


## Design choices

It is recognized that the most primary consumer of this library will be macOS
and secondly iOS applications. Therefore it was chosen not to use `bindgen` in
our build script to not add compilation cost to those targets.<sup>1</sup>

Deprecated functions are also not included for future compability, since they
could be removed in any macOS release, and then our code would break. If you
have a need for these, please open an issue and we can discuss it!

Some items (in particular the `objc_msgSend_X` family) have `cfg`s that prevent
their usage on different platforms; these are **semver-stable** in the sense
that they will only get less restrictive, never more.

<sup>1</sup> That said, most of this is created with the help of `bindgen`'s
commandline interface, so huge thanks to them!
