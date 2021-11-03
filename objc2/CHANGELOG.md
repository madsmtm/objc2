# Changelog

Notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Unreleased - YYYY-MM-DD

### Added
* Added a lot more documentation of safety requirements.
* Added more documentation in general.
* Added newtype `Bool` to fix soundness issues with using `BOOL` or `bool`.
* **BREAKING**: GNUStep users must specify the appropriate feature flag on
  `objc-sys` for the version they're using.
* Moved `objc_exception` crate into `exception` module (under feature flag).
* Moved `objc_id` crate into `rc` module. Most significant changes are
  `T: Message` bounds on `rc::Id`, `rc::Id` not having a default `Ownership` (
  so you must specify it everywhere with `rc::Id<T, Owned / Shared>`) and a
  lot of forwarding implementations for easier use.
* Added `rc::SliceId`, `rc::SliceIdMut` and `rc::DefaultId` helper traits for
  extra functionality on `rc::Id`.
* Added `rc::AutoreleasePool` and `rc::AutoreleaseSafe` to make accessing
  autoreleased objects safe.
* Added (sealed for now) trait `MessageReceiver` to specify types that can
  be used as the receiver of a message (instead of only allowing pointer
  types).
* Added support for `_Complex` types.

### Changed
* **BREAKING**: Updated encoding utilities to use `objc2-encode`. See that for
  how to use the new type `Encoding` and the new traits `Encode` and
  `RefEncode`. Most likely you will need to implement `RefEncode` for any
  custom types you may have.
* **BREAKING**: Require that the receiver, arguments and return types of
  messages always implement `Encode`. This helps ensuring that only types made
  to go across the FFI boundary (`repr(C)`, ...) may.
* **BREAKING**: The closure in `rc::autoreleasepool` now takes an argument
  `&rc::AutoreleasePool`. This reference can be given to functions like
  `INSString::as_str` so that it knows which lifetime to bound the `str` with.
* **BREAKING**: Most types are now `!UnwindSafe`, to discourage trying to use
  them after an unwind. This restriction may be lifted in the future.
* **BREAKING**: Updated `malloc_buf` to version `1.0`.
* **BREAKING**: `Ivar::type_encoding` now returns a `&str` instead of
  `Encoding` for performance reasons.
* **BREAKING**: `Method::return_type` and `Method::argument_type` now return
  `Malloc<str>` instead of `Encoding` for performance reasons.
* **BREAKING**: Dynamic message sending with `Message::send_message` is moved
  to `MessageReceiver`.
* **BREAKING**: The `exception` feature now just enables the `exception`
  module for general use. Use the new `catch_all` feature to wrap all message
  sends in a `@try/@catch`.
* **BREAKING**: Most types are now `!Send` and `!Sync`. TODO: Reevaluate this!
* A lot of smaller things.

### Fixed
* **BREAKING**: Stop unsafely dereferencing `msg_send!`s first argument. The
  `MessageReceiver` trait was introduced to avoid most breakage from this
  change.
* Sealed the `rc::Ownership` trait.
* Various smaller soundness issues.
* Small performance tweaks (probably improvements) in message sending.
* Documentation links.

### Removed
* Removed hidden `sel_impl!` macro.
* **BREAKING**: Removed `rc::StrongPtr`. Use `Option<rc::Id<Object, Shared>>`
  instead (beware: This has stronger safety invariants!).
* **BREAKING**: Removed `rc::WeakPtr`. Use `rc::WeakId<Object>` instead.


## [0.2.7] - 2019-10-19

### Fixed
* Uses of `msg_send!` will now correctly fail to compile if no return type
  can be inferred, instead of relying on an edge case of the compiler
  that will soon change and silently cause undefined behavior.


## [0.2.6] - 2019-03-25

### Fixed
* Suppressed a deprecation warning in `sel!`, `msg_send!`, and `class!`.


## [0.2.5] - 2018-07-24

### Added
* `autoreleasepool` returns the value returned by its body closure.


## [0.2.4] - 2018-07-22

### Added
* Added an `rc` module with reference counting utilities:
  `StrongPtr`, `WeakPtr`, and `autoreleasepool`.
* Added some reference counting ABI foreign functions to the `runtime` module.

### Fixed
* Messaging nil under GNUstep now correctly returns zeroed results for all
  return types.


## [0.2.3] - 2018-07-07

### Added
* Added a `class!` macro for getting statically-known classes. The result is
  non-optional (avoiding a need to unwrap) and cached so each usage will only
  look up the class once.
* Added caching to the `sel!` macro so that each usage will only register the
  selector once.

### Fixed
* Implementation of `objc2::runtime` structs so there can't be unsound
  references to uninhabited types.


## [0.2.2] - 2016-10-30

### Added
* Implemented `Sync` and `Send` for `Sel`.


## [0.2.1] - 2016-04-23

### Added
* Added support for working with protocols with the `Protocol` struct.
  The protocols a class conforms to can be examined with the new
  `Class::adopted_protocols` and `Class::conforms_to` methods.
* Protocols can be declared using the new `ProtocolDecl` struct.


## [0.2.0] - 2016-03-20

### Added
* Added verification for the types used when sending messages.
  This can be enabled for all messages with the `"verify_message"` feature,
  or you can test before sending specific messages with the
  `Message::verify_message` method. Verification errors are reported using the
  new `MessageError` struct.
* Added support for the GNUstep runtime!
  Operating systems besides OSX and iOS will fall back to the GNUstep runtime.
* Root classes can be declared by using the `ClassDecl::root` constructor.

### Changed
* **BREAKING**: C types are now used from `std::os::raw` rather than `libc`.
  This means `Encode` may not be implemented for `libc` types; switch them to
  the `std::os::raw` equivalents instead. This avoids an issue that would
  arise from simultaneously using different versions of the libc crate.
* **BREAKING**: Dynamic messaging was moved into the `Message` trait; instead
  of `().send(obj, sel!(description))`, use
  `obj.send_message(sel!(description), ())`.
* **BREAKING**: Rearranged the parameters to `ClassDecl::new` for consistency;
  instead of `ClassDecl::new(superclass, "MyObject")`, use
  `ClassDecl::new("MyObject", superclass)`.
* **BREAKING**: Overhauled the `MethodImplementation` trait. Encodings are now
  accessed through the `MethodImplementation::Args` associated type. The
  `imp_for` method was replaced with `imp` and no longer takes a selector or
  returns an `UnequalArgsError`, although `ClassDecl::add_method` still
  validates the number of arguments.
* **BREAKING**: Updated the definition of `Imp` to not use the old dispatch
  prototypes. To invoke an `Imp`, it must first be transmuted to the correct
  type.

### Removed
  **BREAKING**: `objc_msgSend` functions from the `runtime` module; the
  availability of these functions varies and they shouldn't be called without
  trasmuting, so they are now hidden as an implementation detail of messaging.

### Fixed
* Corrected alignment of ivars in `ClassDecl`; declared classes may now have a
  smaller size.
* With the `"exception"` or `"verify_message"` feature enabled, panics from
  `msg_send!` will now be triggered from the line and file where the macro is
  used, rather than from within the implementation of messaging.


[0.2.7]: https://github.com/madsmtm/objc2/compare/objc-0.2.6...objc-0.2.7
[0.2.6]: https://github.com/madsmtm/objc2/compare/objc-0.2.5...objc-0.2.6
[0.2.5]: https://github.com/madsmtm/objc2/compare/objc-0.2.4...objc-0.2.5
[0.2.4]: https://github.com/madsmtm/objc2/compare/objc-0.2.3...objc-0.2.4
[0.2.3]: https://github.com/madsmtm/objc2/compare/objc-0.2.2...objc-0.2.3
[0.2.2]: https://github.com/madsmtm/objc2/compare/objc-0.2.1...objc-0.2.2
[0.2.1]: https://github.com/madsmtm/objc2/compare/objc-0.2.0...objc-0.2.1
[0.2.0]: https://github.com/madsmtm/objc2/compare/objc-0.1.8...objc-0.2.0
