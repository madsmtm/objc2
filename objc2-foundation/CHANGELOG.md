# Changelog

Notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Unreleased - YYYY-MM-DD

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

### Fixed
* **BREAKING**: Marked `INS...` traits as `unsafe` to implement.
* **BREAKING**: Removed `new` method from `INSObject` since some classes don't
  want this called. It has been re-added to other `INS...` traits on a case by
  case basis (in particular not `NSValue`).
* **BREAKING**: `INSString::as_str` now takes an a reference to
  `objc2::rc::AutoreleasePool`. This ensure that the returned `&str` is only
  used while the current autorelease pool is valid.

### Removed
* **BREAKING**: Removed the `object_struct!` macro. It may be re-added in
  another form.
* **BREAKING**: `NSMutableSharedArray<T>` and `NSSharedArray<T>` type
  definitions. Use `NSMutableArray<T, Shared>` and `NSArray<T, Shared>`
  instead.
* **BREAKING**: Removed `Any / 'static` bound on `INSObject`. This allows
  implementing it for objects that contain lifetimes from the outer scope.
