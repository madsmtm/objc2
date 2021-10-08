# `gnustep_libobjc2_src`

Source of GNUStep's libobjc2 and logic to build it for Rust.

You probably want to use `objc2_sys`, but this is kept separate from that so
that users don't download the entire GNUStep source when they don't need to.

Using `clang` with at least version `8.0.0` is recommended. You can specify
your desired compiler using the `CC` and `CXX` environment variables.
