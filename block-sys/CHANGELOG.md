# Changelog

Notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Unreleased - YYYY-MM-DD

### Changed
* **BREAKING**: Changed `links` key from `block` to `block_0_0` for better
  future compatibility, until we reach 1.0 (so `DEP_BLOCK_X` in build scripts
  becomes `DEP_BLOCK_0_0_X`).


## 0.0.3 - 2022-01-03

### Changed
* **BREAKING**: Updated `objc-sys` to `v0.2.0-alpha.1`.


## 0.0.2 - 2021-12-22

### Changed
* **BREAKING**: Updated `objc-sys` to `v0.2.0-alpha.0`.

### Fixed
* **BREAKING**: `Class` is now `!UnwindSafe`.


## 0.0.1 - 2021-11-22

Initial release.
