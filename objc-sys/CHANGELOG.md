# Changelog

Notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Unreleased - YYYY-MM-DD

## Added
* `NSInteger` and `NSUInteger` (type aliases of `isize`/`usize`).
* `NSIntegerMax`, `NSIntegerMin` and `NSUIntegerMax`.

### Changed
* **BREAKING**: `cfg`-guarded `class_getImageName` to only appear on Apple
  platforms.

  This is a breaking change, but it will be allowed in a semver-compatible
  release, since it is unlikely that anyone is using it, and the symbol is not
  present on other platforms anyhow, so users will just get an error at
  link-time.

### Fixed
* **BREAKING**: Opaque types are now also `!UnwindSafe`.


## 0.1.0 - 2021-11-22

### Changed
* **BREAKING**: Use feature flags `apple`, `gnustep-X-Y` or `winobjc` to
  specify the runtime you're using, instead of the `RUNTIME_VERSION`
  environment variable.
* **BREAKING**: `DEP_OBJC_RUNTIME` now returns `gnustep` on WinObjC.


## 0.0.1 - 2021-10-28

Initial release.
