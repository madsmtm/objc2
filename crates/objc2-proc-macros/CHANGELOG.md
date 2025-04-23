# Changelog

Notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Unreleased - YYYY-MM-DD


## [0.2.0] - 2025-01-22
[0.2.0]: https://github.com/madsmtm/objc2/compare/objc2-proc-macros-0.1.3...objc2-proc-macros-0.2.0

### Removed
* **BREAKING**: Removed the deprecated `"apple"` Cargo feature flag.


## [0.1.3] - 2024-05-21
[0.1.3]: https://github.com/madsmtm/objc2/compare/objc2-proc-macros-0.1.2...objc2-proc-macros-0.1.3

### Fixed
* Fixed an issue with publishing using an older version of Cargo that didn't
  handle the `lints.workspace = true` Cargo setup properly.


## [0.1.2] - 2024-05-21 (Yanked)
[0.1.2]: https://github.com/madsmtm/objc2/compare/objc2-proc-macros-0.1.1...objc2-proc-macros-0.1.2

### Deprecated
* Deprecated the `"apple"` Cargo feature flag, it is assumed by default on
  Apple platforms.


## [0.1.1] - 2023-02-07
[0.1.1]: https://github.com/madsmtm/objc2/compare/objc2-proc-macros-0.1.0...objc2-proc-macros-0.1.1

### Fixed
* Allow all types of tokens in internal macro.


## [0.1.0] - 2022-07-19
[0.1.0]: https://github.com/madsmtm/objc2/compare/objc2-proc-macros-0.0.0...objc2-proc-macros-0.1.0

### Added
* Added private macro used for the `unstable-static-sel` feature in `objc2`.


## [0.0.0] - 2022-06-16
[0.0.0]: https://github.com/madsmtm/objc2/releases/tag/objc2-proc-macros-0.0.0

Initial empty release.
