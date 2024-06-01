# Changelog

Notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Unreleased - YYYY-MM-DD


## 0.1.3 - 2024-05-21

### Fixed
* Fixed an issue with publishing using an older version of Cargo that didn't
  handle the `lints.workspace = true` Cargo setup properly.


## 0.1.2 - 2024-05-21 (Yanked)

### Deprecated
* Deprecated the `apple` Cargo feature flag, it is assumed by default on Apple
  platforms.


## 0.1.1 - 2023-02-07

### Fixed
* Allow all types of tokens in internal macro.


## 0.1.0 - 2022-07-19

### Added
* Added private macro used for the `unstable-static-sel` feature in `objc2`.


## 0.0.0 - 2022-06-16

Initial empty release.
