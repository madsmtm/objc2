# Changelog

Notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Unreleased - YYYY-MM-DD

### Added
* **BREAKING**: Added associated `Ownership` type to `NSCopying`.
* **BREAKING**: Added associated `Ownership` type to `INSData`.
* **BREAKING**: Added associated `Ownership` type to `INSArray`.
* Added common trait impls (`PartialEq`, `Eq`, `Hash` and `Debug`) to
  `NSValue`, `NSDictionary`, `NSArray` and `NSMutableArray`.

### Changed
* **BREAKING**: Made some creation methods a bit less generic (e.g.
  `INSDictionary::from_keys_and_objects` now always returns `Id<_, Shared>`).

### Removed
* **BREAKING**: Removed associated `Ownership` type from `INSObject`; instead,
  it is present on the types that actually need it (for example `NSCopying`).

### Fixed
* Soundness issue with `NSValue`, `NSDictionary`, `NSArray` and
  `NSMutableArray` not being `#[repr(C)]`.

## 0.2.0-alpha.2 - 2021-11-22

### Added
* **BREAKING**: Added associated `Ownership` type to `INSObject` to specify
  whether the type can be mutated or not. `NSString` is a prime example of a
  type that you may never get a `Owned/&mut` reference to, since it is very
  easy to create two `NSString`s with the same underlying allocation.
* Added helper `is_empty` methods.
* Added `INSArray::first_mut` and `INSArray::last_mut`.

### Changed
* **BREAKING**: Renamed a lot of methods to better match Rusts naming scheme:
  - `INSArray`
    - `count` -> `len`
    - `object_at` -> `get`
    - `mut_object_at` -> `get_mut`
    - `shared_object_at` -> `get_retained`
    - `first_object` -> `first`
    - `last_object` -> `last`
    - `object_enumerator` -> `iter`
  - `INSMutableArray`
    - `add_object` -> `push`
    - `insert_object_at` -> `insert`
    - `replace_object_at` -> `replace`
    - `remove_object_at` -> `remove`
    - `remove_last_object` -> `pop`
    - `remove_all_objects` -> `clear`
  - `INSDictionary`
    - `count` -> `len`
    - `object_for` -> `get`
    - `key_enumerator` -> `iter_keys`
    - `object_enumerator` -> `iter_values`
  - `INSValue`
    - `value` -> `get`
    - `from_value` -> `new`
  - `NSComparisonResult`
    - `from_ordering` -> `from`
    - `as_ordering` -> `into`
  - `NSRange`
    - `from_range` -> `from`
    - `as_range` -> `into`
* Use `SliceId` for better performance when creating arrays and dictionaries.

### Removed
* **BREAKING**: Removed the `object_struct!` macro. It may be re-added in
  another form in the future.
* **BREAKING**: Removed `NSMutableSharedArray<T>` and `NSSharedArray<T>` type
  aliases. Use `NSMutableArray<T, Shared>` and `NSArray<T, Shared>` instead.
* **BREAKING**: Removed `Any / 'static` bound on `INSObject`. This allows
  implementing it for objects that contain lifetimes from the outer scope.

### Fixed
* **BREAKING**: Marked `INS...` traits as `unsafe` to implement.
* **BREAKING**: Removed `new` method from `INSObject` since some classes don't
  want this called. It has been re-added to other `INS...` traits on a case by
  case basis (in particular not `NSValue`).
* **BREAKING**: `INSString::as_str` now takes an a reference to
  `objc2::rc::AutoreleasePool`. This ensure that the returned `&str` is only
  used while the current autorelease pool is valid.
* Fixed `NSData::from_vec` on GNUStep.


## 0.2.0-alpha.1 - 2021-10-28

### Added
* Implement new `RefEncode` trait for objects.
* Implement `Encode` for `NSComparisonResult` and `NSFastEnumerationState`.
* Implement `RefEncode` for objects and `NSFastEnumerationState`.

### Changed
* **BREAKING**: Uses `Id` from `objc2::rc` module instead of `objc_id` crate.
* **BREAKING**: `INSValue::encoding` now returns `&str` instead of `Encoding`.

### Fixed
* Use proper `#[repr(C)]` structs to represent Objective-C objects.
* `INSString::from_str` on GNUStep (`UTF8_ENCODING` was the wrong type).


## 0.2.0-alpha.0 - 2021-08-29

Note: This is the version that is, as of this writing, available on the
`master` branch in the original `objc-foundation` project.

### Added
* Implement `Display` for `NSString`.
* Make `INSObject::class` faster using the `objc::class!` macro.

### Changed
* **BREAKING**: Forked the project, the crate name is now `objc2-foundation`.

### Fixed
* Fixed types in various calls to `objc::msg_send!` for better verification.


## [0.1.1] - 2016-06-19

### Fixed
* An issue with passing functions (instead of function pointers) in
  `INSMutableArray::sort_by`.


## [0.1.0] - 2016-03-20

### Changed
* Update `objc` to `v0.2`.
* Update `objc_id` to `v0.1`.


## [0.0.4] - 2015-12-09

### Removed
* `libc` dependency.


## [0.0.3] - 2015-11-07

### Added
* `object_struct!` macro.

### Changed
* `libc` version can both be `0.1` and `0.2`.


## [0.0.2] - 2015-09-03

### Added
* `Any` bound on `INSObject`, because of a change in `objc` `v0.1.6`.


## [0.0.1] - 2015-06-13

Initial release.


[0.1.1]: https://github.com/madsmtm/objc2/compare/objc-foundation-0.1.0...objc-foundation-0.1.1
[0.1.0]: https://github.com/madsmtm/objc2/compare/objc-foundation-0.0.4...objc-foundation-0.1.0
[0.0.4]: https://github.com/madsmtm/objc2/compare/objc-foundation-0.0.3...objc-foundation-0.0.4
[0.0.3]: https://github.com/madsmtm/objc2/compare/objc-foundation-0.0.2...objc-foundation-0.0.3
[0.0.2]: https://github.com/madsmtm/objc2/compare/objc-foundation-0.0.1...objc-foundation-0.0.2
[0.0.1]: https://github.com/madsmtm/objc2/releases/tag/objc-foundation-0.0.1
