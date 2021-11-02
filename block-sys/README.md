# `block-sys`

[![Latest version](https://badgen.net/crates/v/block-sys)](https://crates.io/crates/block-sys)
[![License](https://badgen.net/badge/license/MIT/blue)](../LICENSE.txt)
[![Documentation](https://docs.rs/block-sys/badge.svg)](https://docs.rs/block-sys/)
[![Apple CI](https://github.com/madsmtm/objc2/actions/workflows/apple.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/apple.yml)
[![GNUStep CI](https://github.com/madsmtm/objc2/actions/workflows/gnustep.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/gnustep.yml)

Raw Rust bindings to Apple's C language extension of blocks.

## Runtime Support

This library is basically just a raw interface to the aptly specified [Blocks
ABI](https://clang.llvm.org/docs/Block-ABI-Apple.html). However, different
runtime implementations exist and act in slightly different ways (and have
several different helper functions), the most important aspect being that the
libraries are named differently, so the linking must take that into account.

The user can choose the desired runtime by using the relevant cargo feature
flags, see the following sections:


### Apple's [`libclosure`](https://opensource.apple.com/source/libclosure/)

- Feature flag: `apple`.

This is naturally the most sophisticated runtime, and it has quite a lot more
features than the specification mandates. This is used by default on Apple
platforms when no feature flags are specified.

The minimum required operating system versions are as follows:
- macOS: `10.6`
- iOS: `3.2`
- tvOS: Unknown
- watchOS: Unknown

Though in practice Rust itself requires higher versions than this.

A git mirror of the sources is available [here](https://github.com/madsmtm/libclosure).


### LLVM `compiler-rt`'s [`libBlocksRuntime`](https://github.com/llvm/llvm-project/tree/release/13.x/compiler-rt/lib/BlocksRuntime)

- Feature flag: `compiler-rt`.

This is the default runtime on all non-Apple platforms when no feature flags
are specified.

This is effectively just a copy of Apple's older (around macOS 10.6) runtime,
and is now used in [Swift's `libdispatch`] and [Swift's Foundation] as well.

This can be easily used on many Linux systems with the `libblocksruntime-dev`
package.

[Swift's `libdispatch`]: https://github.com/apple/swift-corelibs-libdispatch/tree/swift-5.5.1-RELEASE/src/BlocksRuntime
[Swift's Foundation]: https://github.com/apple/swift-corelibs-foundation/tree/swift-5.5.1-RELEASE/Sources/BlocksRuntime


### GNUStep's [`libobjc2`](https://github.com/gnustep/libobjc2)

- Feature flag: `gnustep-1-7`, `gnustep-1-8`, `gnustep-1-9`, `gnustep-2-0` and
  `gnustep-2-1` depending on the version you're using.

GNUStep is a bit odd, because it bundles blocks support into its Objective-C
runtime. This means we have to link to `libobjc`, and this is done by
depending on the `objc-sys` crate. A bit unorthodox, yes, but it works.

Sources:
- [`Block.h`](https://github.com/gnustep/libobjc2/blob/v2.1/objc/blocks_runtime.h)
- [`Block_private.h`](https://github.com/gnustep/libobjc2/blob/v2.1/objc/blocks_private.h)


### Microsoft's [`WinObjC`](https://github.com/microsoft/WinObjC)

- Feature flag: `winobjc`.

Essentially just [a fork](https://github.com/microsoft/libobjc2) based on
GNUStep's `libobjc2` version 1.8.


### [`ObjFW`](https://github.com/ObjFW/ObjFW) (WIP)

- Feature flag: `objfw`.

TODO.


## C Compiler configuration

To our knowledge, currently only `clang` supports the [Language Specification
for Blocks][block-lang]. To assist in compiling C (or Objective-C) sources
using these features, this crate's build script expose the `DEP_BLOCK_CC_ARGS`
environment variable to downstream build scripts.

Example usage in your `build.rs` (using the `cc` crate) would be as follows:

```rust , ignore
fn main() {
    let mut builder = cc::Build::new();
    builder.compiler("clang");
    builder.file("my_script_using_blocks.c");

    for flag in std::env::var("DEP_BLOCK_CC_ARGS").unwrap().split(' ') {
        builder.flag(flag);
    }

    builder.compile("libmy_script_using_blocks.a");
}
```

[block-lang]: https://clang.llvm.org/docs/BlockLanguageSpec.html
