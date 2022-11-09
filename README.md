# [![Rust + \[Obj-C\]](assets/logo-small.png)](https://github.com/madsmtm/objc2) <br> Objective-C in Rust

[![License](https://badgen.net/badge/license/MIT/blue)](./LICENSE.txt)
[![CI](https://github.com/madsmtm/objc2/actions/workflows/ci.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/ci.yml)

# DISCLAIMER! These crates are work in progress, and should not be used in production environments. Use the battle-tested `objc` family instead!

Anyway, thanks for being here, any help testing things out is highly
appreciated!


## Crate overview

The core crate is [`objc2`], which contains everything you need to interface
with Objective-C. It also provides safe abstraction over (parts of) the
Foundation Framework, since that is used in almost all Objective-C code.

[`block2`] has a bit of a weird position in all of this: Apple's C language
extension of blocks is _technically_ not limited to being used in Objective-C,
though in practice that's the only place it's used, so it makes sense to
develop these together.

[`objc2-encode`] is really just a part of `objc2`, it mostly exists as a
separate crate to help people cutting down on unneeded dependencies.
[`objc-sys`] and [`block-sys`] contain raw bindings to the underlying C
runtime libraries.

[`objc2`]: ./crates/objc2
[`block2`]: ./crates/block2
[`objc2-encode`]: ./crates/objc2-encode
[`objc-sys`]: ./crates/objc-sys
[`block-sys`]: ./crates/block-sys


## Migrating from original crates

It is recommended that you upgrade in small steps, to decrease the chance of
making a mistake somewhere. This way, you can under each release review the
relevant changelogs and run your test suite, to ensure you haven't missed
something. For the most common errors, the changelogs will include a helpful
example on how to upgrade.

As an example, if you want to migrate to `objc2`, you'd start by using it
instead of `objc` in your `Cargo.toml`:
```toml
[dependencies]
objc = { package = "objc2", version = "0.2.7" }
```

Afterwards, you can upgrade to the next release, in this case
`v0.3.0-alpha.0`, and make the required changes to your code (the
[changelog](crates/objc2/CHANGELOG.md) contains recommendations for this). And so on,
with every following release.


## Goals

- **Complete Soundness.**

  The overarching goal of these crates is to be completely sound, meaning it
  must not be possible for safe Rust to cause undefined behaviour using this
  library!

  This is a really difficult task (!!!), because, as a language interface,
  basically everything it does requires `unsafe`, and because the two
  languages have vastly different semantics.

  Tackling this issue requires vigilance, which is why we're stating it as the
  project's top priority. Internally, all instances of `unsafe` should have an
  accompanying `# SAFETY` comment that explain the reasoning behind their
  safety. At some point we might be able to use the Miri tool to help finding
  instances of undefined behaviour, see [#146].

  If you find, or think you've found, a soundness hole, please report it on
  the [issue tracker].

- **Idiomatic Rust.**

  Soundness is easy to achieve if you just mark every API as `unsafe`.
  However, that just pushes the burden onto the user!

  As such, we'll try to be as safe and idiomatic as possible; using references
  instead of pointers to represent objects and their mutability, `Option`
  instead of `null`, and so on. These abstractions should be zero cost, but
  this is of course a balancing act against being ergonomic.

  Some APIs will still have to remain `unsafe` though, so these should contain
  thorough `# Safety` sections, to let users know _exactly_ which safety
  guarantees they need to uphold.

- **Portability.**

  Apple targets (macOS, iOS, iPadOS, tvOS and watchOS) always have the highest
  priority, but there exist other runtimes for Objective-C, and we would like
  to support them without the user having to worry about portability issues.

  The supported runtimes are tested in CI. This also improves the robustness
  of our implementation, giving os more confidence that things won't break
  when Apple releases a new version of one of their OSes.

- **High quality documentation.**

  This is still fairly lacking, see [#32], help appreciated!

  If you have any questions or need advice and/or review of your Rust crate
  that interfaces with Objective-C, please [open an issue][issue tracker],
  I'll be glad to give you advice!

[#146]: https://github.com/madsmtm/objc2/pull/146
[issue tracker]: https://github.com/madsmtm/objc2/issues/new
[#32]: https://github.com/madsmtm/objc2/issues/32


## License

This project is licensed under the MIT license, see [`LICENSE.txt`].

Work is in progress to make it dual-licensed under the Apache License
(Version 2.0) as well, see [this][#23].

[`LICENSE.txt`]: https://github.com/madsmtm/objc2/blob/master/LICENSE.txt
[#23]: https://github.com/madsmtm/objc2/issues/23


## Acknowledgements

This repository is a merge of the following projects, see reasoning for the
fork [here][origin-issue-101]:
- [`objc`](https://github.com/SSheldon/rust-objc)
  - Renamed to `objc2`.
- [`objc-encode`](https://github.com/SSheldon/rust-objc-encode)
  - Renamed to `objc2-encode`.
- [`objc_exception`](https://github.com/SSheldon/rust-objc-exception)
  - Moved to `objc2::exception`.
- [`objc_id`](https://github.com/SSheldon/rust-objc-id)
  - Moved to `objc2::rc`.
- [`objc-foundation`](https://github.com/SSheldon/rust-objc-foundation)
  - Moved to `objc2::foundation`.
- [`block`](https://github.com/SSheldon/rust-block)
  - Renamed to `block2`.

These were created almost solely by [@SSheldon](https://github.com/SSheldon),
so a huge thanks for their fantastic work on these crates!

This project also draws heavy inspiration from [`fruity`], the [`core-foundation-rs` project] and [`objrs`].

[origin-issue-101]: https://github.com/SSheldon/rust-objc/issues/101
[`fruity`]: https://github.com/nvzqz/fruity
[`core-foundation-rs` project]: https://github.com/servo/core-foundation-rs
[`objrs`]: https://gitlab.com/objrs/objrs
