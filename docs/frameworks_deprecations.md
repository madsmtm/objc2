# Deprecations in framework crates

A somewhat common confusion in the framework crates are that items are marked
as deprecated; this is usually done because Apple marks them as deprecated,
not because the `objc2` project intends to remove them!

This has implications for your minimum version support; for example, if you
want to support all the way down to macOS 10.12 (the minimum version that Rust
itself currently supports), you will sometimes run into deprecation warnings.

In these cases, `#[allow(deprecated)]` is perfectly acceptable! Again, we
expose these deprecation attributes because that's what the headers have, but
usually you don't need to worry too much about them.[^apple-slow-to-remove]

Example: [#824](https://github.com/madsmtm/objc2/issues/824).

[^apple-slow-to-remove]: It usually takes fairly long (think years) between
Apple marking an API as deprecated before it's removed from the SDK, and even
longer before it's finally removed from the OS (if ever).
