# [![Rust + \[Obj-C\]](assets/logo-small.png)](https://github.com/madsmtm/objc2) <br> Objective-C in Rust

[![License](https://badgen.net/badge/license/MIT/blue)](../LICENSE.txt)
[![Apple CI](https://github.com/madsmtm/objc2/actions/workflows/apple.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/apple.yml)
[![GNUStep CI](https://github.com/madsmtm/objc2/actions/workflows/gnustep.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/gnustep.yml)

# DISCLAIMER! These crates are work in progress, and should not be used in production environments. Use the battle-tested `objc` family instead!


## Migrating from original crates

It is recommended that you upgrade in small steps, to decrease the chance of
making a mistake somewhere. This way, you can under each release review the
relevant changelogs and run your test suite, to ensure you haven't missed
something.

As an example, if you want to migrate to `objc2`, you'd start by using it
instead of `objc` in your `Cargo.toml`:
```toml
[dependencies]
objc = { package = "objc2", version = "0.2.7" }
```

Afterwards, you can upgrade to the next release, in this case
`v0.3.0-alpha.0`, and make the required changes to your code (the
[changelog](objc/CHANGELOG.m) contains recommendations for this). And so on,
with every following release.


## License

This project is licensed under the MIT license, see [`LICENSE.txt`].

Work is in progress to make it dual-licensed under the Apache License
(Version 2.0) as well, see [this][#23].

[`LICENSE.txt`]: https://github.com/madsmtm/objc2/blob/master/LICENSE.txt
[#23]: https://github.com/madsmtm/objc2/issues/23


## Acknowledgements

This repository is originally a fork of [`objc`], with the following
projects merged into it (see reasoning for the fork [here][origin-issue-101]):
- [`objc-encode`](https://github.com/SSheldon/rust-objc-encode)
- [`objc_exception`](https://github.com/SSheldon/rust-objc-exception)
- [`objc_id`](https://github.com/SSheldon/rust-objc-id)
- [`objc-foundation`](https://github.com/SSheldon/rust-objc-foundation)
- [`block`](https://github.com/SSheldon/rust-block)

These were created almost solely by [@SSheldon](https://github.com/SSheldon),
so a huge thanks for their fantastic work on these crates!

This project also draws heavy inspiration from [`fruity`] and [`objrs`].

[`objc`]: https://github.com/SSheldon/rust-objc
[origin-issue-101]: https://github.com/SSheldon/rust-objc/issues/101
[`fruity`]: https://github.com/nvzqz/fruity
[`objrs`]: https://gitlab.com/objrs/objrs
