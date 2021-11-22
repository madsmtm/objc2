# Changelog

Notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Unreleased - YYYY-MM-DD

### Added
* **BREAKING**: Add `Encoding::LongDouble`, `Encoding::FloatComplex`,
  `Encoding::DoubleComplex` and `Encoding::LongDoubleComplex`.
* Improved documentation.
* **BREAKING**: Add `RefEncode` trait, which represents types whoose pointers
  has an encoding. This means you now only have to implement `RefEncode`, and
  not both `&Encode` and `&mut Encode`.
* **BREAKING**: Implement `Encode` and `RefEncode` for all basic types (where
  `T` is properly bound to implement it as well):
  - Number types (`f32`, `u32`, ...)
  - `NonZero` integer types
  - `[T; LEN]` for all `LEN`s
  - `ManuallyDrop<T>`
  - `Pin<T>`
  - `NonNull<T>`
  - `Option<NonNull<T>>`
  - `Wrapping<T>`
  - `*const c_void`
  - `*mut c_void`
  - Some `extern "C" fn` pointers
  - Pointers to pointers
  - Pointers to pointers to pointers
  - And so on
* Add `EncodeArguments` from `objc` crate.

### Changed
* **BREAKING**: Forked the project, so it is now available under the name
  `objc2-encode`.
* **BREAKING**: Made `Encoding` `#[non_exhaustive]`. This will help us in
  evolving the API while minimizing further breaking changes.
* Loosen `'static` bounds on references implementing `Encode`.
* Discourage using `bool::ENCODING`; use `objc2::Bool::ENCODING` instead.

### Removed
* **BREAKING**: Automatic `*const T: Encode` and `*mut T: Encode` impls when
  when `&T: Encode` and `&mut T: Encode` was implemented. Implement
  `T: RefEncode` instead!


## [1.1.0] (`objc-encode` crate) - 2019-10-16

### Added
* Implement `Encode` for arrays with up to 32 elements.

### Changed
* Simplify internal encoding comparison.


## [1.0.0] (`objc-encode` crate) - 2019-03-25

### Added
* Implement `PartialEq` between `Encoding` and `&str`.

### Changed
* **BREAKING**: Make `Encoding` an enum instead of a trait, yielding a vastly
  different design. This makes use of associated constants.
* **BREAKING**: Rename `Encode::CODE` to `Encode::ENCODING`.
* Update to Rust 2018.

### Removed
* `libc` dependency.


## [0.0.3] (`objc-encode` crate) - 2017-04-30

### Fixed
* Compilation on versions prior to Rust `1.15`.


## [0.0.2] (`objc-encode` crate) - 2017-02-20

### Added
* **BREAKING**: `Display` requirement for encodings.
* Implement `PartialEq` for encodings.
* Implement `Encode` for pointers when references do.

### Fixed
* `IndexEncodingsComparator`.
* Compilation with older Rust versions.


## [0.0.1] (`objc-encode` crate) - 2017-02-19

Initial version.


[1.1.0]: https://github.com/madsmtm/objc2/compare/objc-encode-1.0.0...objc-encode-1.1.0
[1.0.0]: https://github.com/madsmtm/objc2/compare/objc-encode-0.0.3...objc-encode-1.0.0
[0.0.3]: https://github.com/madsmtm/objc2/compare/objc-encode-0.0.2...objc-encode-0.0.3
[0.0.2]: https://github.com/madsmtm/objc2/compare/objc-encode-0.0.1...objc-encode-0.0.2
[0.0.1]: https://github.com/madsmtm/objc2/releases/tag/objc-encode-0.0.1
