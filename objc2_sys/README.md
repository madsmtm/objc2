# `objc_sys`

[![Latest version](https://badgen.net/crates/v/objc_sys)](https://crates.io/crates/objc_sys)
[![License](https://badgen.net/badge/license/MIT/blue)](../LICENSE.txt)
[![Documentation](https://docs.rs/objc_sys/badge.svg)](https://docs.rs/objc_sys/)
[![CI Status](https://github.com/madsmtm/objc/workflows/CI/badge.svg)](https://github.com/madsmtm/objc/actions)

Raw Rust bindings to core Objective-C runtimes and ABIs.

## Runtime Support

`objc_sys` currently supports two runtimes (support for [`ObjFW`] and
[`WinObjC`] may be added):
- Apple's [`objc4`] on `cfg(target_vendor = "apple")` targets.
- GNUStep's [`libobjc2`] on all other targets. See their [Objective-C Compiler
  and Runtime FAQ][gnustep-faq].

This library will probably only ever support ["Modern"][modern] Objective-C
runtimes, since support for reference-counting primitives like `objc_retain`
and `objc_autoreleasePoolPop` is a vital requirement for most applications.

Just so we're being clear, this rules out the GCC [`libobjc`][gcc-libobjc]
runtime (see [this][gcc-objc-support]), and the [`mulle-objc`] runtime.

[`ObjFW`]: https://github.com/ObjFW/ObjFW
[`WinObjC`]: https://github.com/microsoft/WinObjC
[`objc4`]: https://opensource.apple.com/source/objc4/
[`libobjc2`]: https://github.com/gnustep/libobjc2
[gnustep-faq]: http://wiki.gnustep.org/index.php/Objective-C_Compiler_and_Runtime_FAQ
[modern]: https://en.wikipedia.org/wiki/Objective-C#Modern_Objective-C
[gcc-libobjc]: https://github.com/gcc-mirror/gcc/tree/master/libobjc
[gcc-objc-support]: https://gcc.gnu.org/onlinedocs/gcc/Standards.html#Objective-C-and-Objective-C_002b_002b-Languages
[`mulle-objc`]: https://github.com/mulle-objc/mulle-objc-runtime


## Required Versions

At least `libobjc2` [version 1.7][libobjc2-1.7] or `objc4`
[version 493.9][objc4-493.9] is required.

`objc4` version 493.9 is available with:
- **macOS 10.7**
- **iOS 5.0**
- **tvOS 9.0**
- **watchOS 1.0**
- **bridgeOS 2.0**

So those are the **minimum supported Apple versions**. Functionality that was
added after these versions are not (yet?) available in `objc_sys`.

[libobjc2-1.7]: https://github.com/gnustep/libobjc2/tree/1.7
[objc4-493.9]: https://opensource.apple.com/source/objc4/


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
