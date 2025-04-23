# Changelog

Notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Unreleased - YYYY-MM-DD


## [0.1.1] - 2025-01-22
[0.1.1]: https://github.com/madsmtm/objc2/compare/objc2-exception-helper-0.1.0...objc2-exception-helper-0.1.1

### Fixed
* Fixed the symbol name to include the correct SemVer version of the crate.
* Fixed the ABI of `try_catch` to be `extern "C-unwind"`.
* Clarified that `try_catch` does not catch Rust panics.


## [0.1.0] - 2024-06-02
[0.1.0]: https://github.com/madsmtm/objc2/releases/tag/objc2-exception-helper-0.1.0

Initial release.
