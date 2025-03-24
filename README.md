# [![Rust + \[Obj-C\]](assets/logo-small.png)](https://github.com/madsmtm/objc2) <br> Objective-C in Rust

[![License](https://badgen.net/badge/license/MIT/blue)](./LICENSE.txt)
[![CI](https://github.com/madsmtm/objc2/actions/workflows/ci.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/ci.yml)

**See [`objc2`] for top-level documentation**, including its [documentation on framework crates]. Also check out [`block2`] and [`dispatch2`].

[`objc2`]: https://docs.rs/objc2/
[documentation on framework crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
[`block2`]: https://docs.rs/block2/
[`dispatch2`]: https://docs.rs/dispatch2/


## Contact Us

Always feel free to [open an issue on GitHub](https://github.com/madsmtm/objc2/issues/new/choose) if you find a problem or have any questions.

If you prefer to have a more synchronous and less "formal" discussion, we have [a Matrix workspace](https://matrix.to/#/#objc2:matrix.org), feel free to ask any questions in the "Users" room.


## License

The licensing of the project is a bit complicated, see
[`LICENSE.md`](./LICENSE.md) for details.


## Acknowledgements / Prior art

This repository is a merge of the following projects, see reasoning for the
fork [here](https://github.com/SSheldon/rust-objc/issues/101):
- [`objc`](https://github.com/SSheldon/rust-objc), renamed to `objc2`.
- [`objc-encode`](https://github.com/SSheldon/rust-objc-encode), renamed to `objc2-encode`.
- [`objc_exception`](https://github.com/SSheldon/rust-objc-exception), moved to `objc2::exception`.
- [`objc_id`](https://github.com/SSheldon/rust-objc-id), moved to `objc2::rc`.
- [`objc-foundation`](https://github.com/SSheldon/rust-objc-foundation), renamed to `objc2-foundation`.
- [`block`](https://github.com/SSheldon/rust-block), renamed to `block2`.

These were created almost solely by [@SSheldon](https://github.com/SSheldon),
so a huge thanks for their fantastic work on these crates!

Additionally, the `dispatch2` crate originally lived [here](https://github.com/marysaka/dispatch2).

This project also draws inspiration from:
- [`apple-sys`](https://github.com/youknowone/apple-sys)
- [`cacao`](https://github.com/ryanmcgrath/cacao)
- [the `core-foundation-rs` project](https://github.com/servo/core-foundation-rs)
- [`fruity`](https://github.com/nvzqz/fruity)
- [`metal`](https://github.com/gfx-rs/metal-rs)
- [`objrs`](https://gitlab.com/objrs/objrs)
- [`objr` and family](https://github.com/drewcrawford/objr#objr-expanded-universe)
- [`rust-macios`](https://github.com/a-isaiahharvey/rust-macios)
- [`uikit-sys`](https://github.com/simlay/uikit-sys) and `@simlay`'s [Objective-C work on `bindgen`](https://rust-lang.github.io/rust-bindgen/objc.html)
- [`cidre`](https://github.com/yury/cidre)
- [the `apple-media` project](https://github.com/rust-media/apple-media-rs)
- [`dispatch`](https://github.com/SSheldon/rust-dispatch)

Finally, this is by far not the only project that ever tried to interoperate with Objective-C; other languages have done so as well (to varying degrees of success):
- Swift: Built from the beginning for Objective-C interop, and is what `objc2` aspires to have feature-parity with (though will probably never reach). Truly beautifully designed language!
- C#: Xamarin, [Xamarin.Mac](https://www.mono-project.com/docs/tools+libraries/libraries/monomac/), a good source of inspiration for what "should" work.
- Python: [PyObjC](https://pypi.org/project/pyobjc/) (previously?) official Apple project that worked with "BridgeSupport", nowadays they also [generate metadata by invoking Clang](https://github.com/ronaldoussoren/objective.metadata). Others include [`objp`](https://pypi.org/project/objp/) and [rubicon.objc](https://rubicon-objc.readthedocs.io/en/latest/index.html)
- Ruby: [MacRuby](http://macruby.org/), RubyCocoa
- Dart: [`ffigen`](https://github.com/dart-lang/ffigen/tree/master/example/objective_c)
- Kotlin: [somewhat built-in support](https://kotlinlang.org/docs/native-objc-interop.html)
- Nim: [somewhat built-in support](https://nim-lang.org/docs/backends.html), [`darwin`](https://github.com/yglukhov/darwin), [`objc`](https://github.com/jangko/objc)
- D: [somewhat built-in support](https://dlang.org/spec/objc_interface.html), [`derelict`](https://github.com/AuburnSounds/Dplug/tree/v12.8.0/macos/derelict/cocoa)
- Java: [Java-Objective-C-Bridge](https://github.com/shannah/Java-Objective-C-Bridge), [Multi-OS Engine: Nat/J](https://github.com/multi-os-engine/moe-natj) (also has a [generator](https://github.com/multi-os-engine/moe-natjgen)), Apple also has a very old official project.
- Node.js: [NodObjC](https://github.com/TooTallNate/NodObjC), [`objc`](https://github.com/lukaskollmer/objc)
- Zig: [zig-objcrt](https://github.com/hazeycode/zig-objcrt)
- V: Not really existing, they just write and compile Objective-C code, and use manual C-bindings.
- Go: [MacDriver](https://github.com/progrium/macdriver)
