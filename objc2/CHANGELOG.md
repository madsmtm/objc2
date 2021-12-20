# Changelog

Notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Unreleased - YYYY-MM-DD

### Added
* Export `objc-sys` as `ffi` module.
* Added common trait impls on `rc::Owned` and `rc::Shared` (useful in generic
  contexts).

### Changed
* Deprecated `runtime::BOOL`, `runtime::YES` and `runtime::NO`. Use the
  newtype `Bool` instead, or low-level `ffi::BOOL`, `ffi::YES` and `ffi::NO`.

### Removed
* **BREAKING**: Removed the raw FFI functions from the `runtime` module. These
  are available in the new `ffi` module instead.

### Fixed
* An issue with inlining various `rc` methods.


## 0.3.0-alpha.4 - 2021-11-22

### Added
* **BREAKING**: GNUStep users must depend on, and specify the appropriate
  feature flag on `objc-sys` for the version they're using.
* Moved `objc_exception` crate into `exception` module (under feature flag).
* Added support for `_Complex` types.
* Added `rc::SliceId`, `rc::SliceIdMut` and `rc::DefaultId` helper traits for
  extra functionality on `rc::Id`.

### Changed
* **BREAKING**: The `exception` feature now just enables the `exception`
  module for general use. Use the new `catch_all` feature to wrap all message
  sends in a `@try/@catch`.


## 0.3.0-alpha.3 - 2021-09-05

### Added
* Now uses the `objc-sys` crate for possibly better interoperability with
  other crates that link to `libobjc`.
* Added newtype `runtime::Bool` to fix soundness issues with using
  `runtime::BOOL` or `bool`.
* Moved `objc_id` crate into `rc` module. Notable changes:
  - Vastly improved documentation
  - Added `Id::autorelease`
  - Added `Id::from_shared`
  - Added a lot of forwarding implementations on `Id` for easier use
  - `Id` and `WeakId` are now able to use the null-pointer optimization
  - **BREAKING**: Added `T: Message` bounds on `Id`
  - **BREAKING**: Remove `ShareId` type alias
  - **BREAKING**: `Id` no longer have a default `Ownership`, you must specify
    it everywhere as either `Id<T, Shared>` or `Id<T, Owned>`
  - **BREAKING**: Sealed the `Ownership` trait
  - **BREAKING**: Renamed `Id::from_ptr` to `Id::retain`
  - **BREAKING**: Renamed `Id::from_retained_ptr` to `Id::new`
  - **BREAKING**: Changed `Id::share` to a `From` implementation (usage of
    `obj.share()` can be changed to `obj.into()`)
  - **BREAKING**: Fixed soundness issues with missing `Send` and `Sync` bounds
    on `Id` and `WeakId`
* Added sealed (for now) trait `MessageReceiver` to specify types that can
  be used as the receiver of a message (instead of only allowing pointer
  types).
* Add `MessageReceiver::send_super_message` method for dynamic selectors.

### Changed
* **BREAKING**: Change types of parameters to FFI functions exported in the
  `runtime` module.
* **BREAKING**: Most types are now `!UnwindSafe`, to discourage trying to use
  them after an unwind. This restriction may be lifted in the future.
* **BREAKING**: Most types are now `!Send` and `!Sync`. **TODO**: Reevaluate this!
* A lot of smaller things.
* **BREAKING**: Dynamic message sending with `Message::send_message` is moved
  to `MessageReceiver`.
* **BREAKING** Make `MessageArguments` a subtrait of `EncodeArguments`.
* Allow an optional comma after each argument to `msg_send!`.

### Removed
* **BREAKING**: Removed `rc::StrongPtr`. Use `Option<rc::Id<Object, Shared>>`
  instead (beware: This has stronger safety invariants!).
* **BREAKING**: Removed `rc::WeakPtr`. Use `rc::WeakId<Object>` instead.

### Fixed
* **BREAKING**: Stop unsafely dereferencing `msg_send!`s first argument. The
  `MessageReceiver` trait was introduced to avoid most breakage from this
  change.


## 0.3.0-alpha.2 - 2021-09-05

### Added
* Added `rc::AutoreleasePool` and `rc::AutoreleaseSafe` to make accessing
  autoreleased objects safe, by binding references to it using the
  `ptr_as_ref` and `ptr_as_mut` methods.

### Changed
* **BREAKING**: The closure in `rc::autoreleasepool` now takes an argument
  `&rc::AutoreleasePool`. This reference can be given to functions like
  `INSString::as_str` so that it knows which lifetime to bound the returned
  `&str` with.

  Simple migration:
  ```rust
  // Change
  autoreleasepool(|| {
      // Some code that autoreleases objects
  });
  // To
  autoreleasepool(|_pool| {
      // Some code that autoreleases objects
  });
  ```

### Fixed
* The encoding of `BOOL` on `GNUStep`.


## 0.3.0-alpha.1 - 2021-09-02

### Added
* More documentation of safety requirements, and in general.

### Changed
* **BREAKING**: Change `objc-encode` dependency to `objc2-encode` version
  `2.0.0-alpha.1`, and re-export the new `RefEncode` trait from that.
* **BREAKING**: Require that the receiver, arguments and return types of
  messages always implement `Encode`. This helps ensuring that only types made
  to go across the FFI boundary (`repr(C)`, ...) may. These requirements were
  already present when the `verify_message` feature was enabled.

  This is a very _disruptive change_, since libraries are now required to
  implement `Encode` and `RefEncode` for all types intended to go across the
  FFI-boundary to Objective-C. The change is justified because it helps
  ensuring that users only pass valid types to `msg_send!` (for example, this
  prevents users from accidentally passing `Drop` types to `msg_send`).

  See the following examples for how to implement these traits, and otherwise
  refer to the documentation of `objc2-encode` (`v2.0.0-alpha.1` or above).
  ```rust
  use objc2::{Encode, Encoding, RefEncode};

  /// Example struct.
  #[repr(C)]
  pub struct NSRange {
      pub location: usize,
      pub length: usize,
  }
  unsafe impl Encode for NSRange {
      const ENCODING: Encoding<'static> = Encoding::Struct(
          "_NSRange", // This is how the struct is defined in C header files
          &[usize::ENCODING, usize::ENCODING]
      );
  }
  unsafe impl RefEncode for NSRange {
      const ENCODING_REF: Encoding<'static> = Encoding::Pointer(&Self::ENCODING);
  }

  /// Example object.
  #[repr(C)]
  pub struct __CFString(c_void);

  pub type CFStringRef = *const __CFString;

  unsafe impl RefEncode for __CFString {
      const ENCODING_REF: Encoding<'static> = Encoding::Object;
  }
  ```
* Temporarily disabled iOS tests.

### Fixed
* Statically find the correct `objc_msgSend[_X]` function to use based on the
  `Encode` implementation of the return type. This fixes using functions that
  return e.g. `type CGFloat = f32 / f64;`.
* Documentation links.


## 0.3.0-alpha.0 - 2021-08-29

Note: This is the version that is, as of this writing, available on the
`master` branch in the original `objc` project.

### Added
* Improve macro hygiene.
  ```rust
  // You can now do
  use objc2::{sel, class, msg_send};
  // Instead of
  #[macro_use]
  extern crate objc2;
  ```
* Update to Rust 2018.
* Other internal improvements.

### Changed
* **BREAKING**: Forked the project, so the crate name is now `objc2`.
* **BREAKING**: Updated encoding utilities to use `objc-encode`. See that for
  how to use the updated type `Encoding` and trait `Encode`.

  In short, you will likely need to change your implementations of `Encode`
  like this:
  ```rust
  use objc2::{Encode, Encoding};

  pub type CGFloat = ...; // Varies based on target_pointer_width

  #[repr(C)]
  pub struct NSPoint {
      pub x: CGFloat,
      pub y: CGFloat,
  }

  // Before
  unsafe impl Encode for NSPoint {
      fn encode() -> Encoding {
          let encoding = format!(
              "{{CGPoint={}{}}}",
              CGFloat::encode().as_str(),
              CGFloat::encode().as_str(),
          );
          unsafe { Encoding::from_str(&encoding) }
      }
  }

  // After
  unsafe impl Encode for NSPoint {
      const ENCODING: Encoding<'static> = Encoding::Struct(
          "CGPoint",
          &[CGFloat::ENCODING, CGFloat::ENCODING]
      );
  }
  ```
* **BREAKING**: Updated public dependency `malloc_buf` to `1.0`.
* **BREAKING**: `Method::return_type` and `Method::argument_type` now return
  `Malloc<str>` instead of `Encoding`.
* **BREAKING**: `Ivar::type_encoding` now return `&str` instead of `Encoding`.

### Removed
* **BREAKING**: Removed hidden `sel_impl!` macro.


## [0.2.7] (`objc` crate) - 2019-10-19

### Fixed
* **BREAKING**: Uses of `msg_send!` will now correctly fail to compile if no
  return type can be inferred, instead of relying on an edge case of the
  compiler that will soon change and silently cause undefined behavior.


## [0.2.6] (`objc` crate) - 2019-03-25

### Fixed
* Suppressed a deprecation warning in `sel!`, `msg_send!`, and `class!`.


## [0.2.5] (`objc` crate) - 2018-07-24

### Added
* **BREAKING**: `autoreleasepool` returns the value returned by its body
  closure.


## [0.2.4] (`objc` crate) - 2018-07-22

### Added
* Added an `rc` module with reference counting utilities:
  `StrongPtr`, `WeakPtr`, and `autoreleasepool`.
* Added some reference counting ABI foreign functions to the `runtime` module.

### Fixed
* Messaging nil under GNUstep now correctly returns zeroed results for all
  return types.


## [0.2.3] (`objc` crate) - 2018-07-07

### Added
* Added a `class!` macro for getting statically-known classes. The result is
  non-optional (avoiding a need to unwrap) and cached so each usage will only
  look up the class once.
* Added caching to the `sel!` macro so that each usage will only register the
  selector once.

### Fixed
* Implementation of `objc2::runtime` structs so there can't be unsound
  references to uninhabited types.


## [0.2.2] (`objc` crate) - 2016-10-30

### Added
* Implemented `Sync` and `Send` for `Sel`.


## [0.2.1] (`objc` crate) - 2016-04-23

### Added
* Added support for working with protocols with the `Protocol` struct.
  The protocols a class conforms to can be examined with the new
  `Class::adopted_protocols` and `Class::conforms_to` methods.
* Protocols can be declared using the new `ProtocolDecl` struct.


## [0.2.0] (`objc` crate) - 2016-03-20

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

## [0.1.8] (`objc` crate) - 2015-11-06

### Changed
* Updated `libc` dependency.


## [0.1.7] (`objc` crate) - 2015-09-23

### Fixed
* `improper_ctypes` warning.


## [0.1.6] (`objc` crate) - 2015-08-08

### Added
* Added `"exception"` feature which catches Objective-C exceptions and turns
  them into Rust panics.
* Added support for `ARM`, `ARM64` and `x86` architectures.
* **BREAKING**: Added `Any` bound on message return types. In practice this
  probably won't break anything.
* Start testing on iOS.


## [0.1.5] (`objc` crate) - 2015-05-02

### Changed
* **BREAKING**: Renamed `IntoMethodImp` to `MethodImplementation`.
* **BREAKING**: Renamed `MethodImplementation::into_imp` to `::imp_for`.
* **BREAKING**: Relaxed `Sized` bounds on `Encode` and `Message`. In practice
  this probably won't break anything.

### Removed
* **BREAKING**: Removed `Id`, `Owned`, `Ownership`, `Shared`, `ShareId` and
  `WeakId`. Use them from the `objc_id` crate instead.
* **BREAKING**: Removed `Method::set_implementation` and
  `Method::exchange_implementation`.


## [0.1.4] (`objc` crate) - 2015-04-17

### Removed
* **BREAKING**: Removed `block` module. Use them from the `block` crate
  instead.


## [0.1.3] (`objc` crate) - 2015-04-11

### Added
* Implement `fmt::Pointer` for `Id`.

### Fixed
* Odd lifetime bug.


## [0.1.2] (`objc` crate) - 2015-04-04

### Fixed
* **BREAKING**: Replace uses of `PhantomFn` with `Sized`.


## [0.1.1] (`objc` crate) - 2015-03-27

### Added
* Implement `Error` for `UnequalArgsError`.

### Removed
* **BREAKING**: Move `objc::foundation` into new crate `objc_foundation`.


[0.2.7]: https://github.com/madsmtm/objc2/compare/objc-0.2.6...objc-0.2.7
[0.2.6]: https://github.com/madsmtm/objc2/compare/objc-0.2.5...objc-0.2.6
[0.2.5]: https://github.com/madsmtm/objc2/compare/objc-0.2.4...objc-0.2.5
[0.2.4]: https://github.com/madsmtm/objc2/compare/objc-0.2.3...objc-0.2.4
[0.2.3]: https://github.com/madsmtm/objc2/compare/objc-0.2.2...objc-0.2.3
[0.2.2]: https://github.com/madsmtm/objc2/compare/objc-0.2.1...objc-0.2.2
[0.2.1]: https://github.com/madsmtm/objc2/compare/objc-0.2.0...objc-0.2.1
[0.2.0]: https://github.com/madsmtm/objc2/compare/objc-0.1.8...objc-0.2.0
[0.1.8]: https://github.com/madsmtm/objc2/compare/objc-0.1.7...objc-0.1.8
[0.1.7]: https://github.com/madsmtm/objc2/compare/objc-0.1.6...objc-0.1.7
[0.1.6]: https://github.com/madsmtm/objc2/compare/objc-0.1.5...objc-0.1.6
[0.1.5]: https://github.com/madsmtm/objc2/compare/objc-0.1.4...objc-0.1.5
[0.1.4]: https://github.com/madsmtm/objc2/compare/objc-0.1.3...objc-0.1.4
[0.1.3]: https://github.com/madsmtm/objc2/compare/objc-0.1.2...objc-0.1.3
[0.1.2]: https://github.com/madsmtm/objc2/compare/objc-0.1.1...objc-0.1.2
[0.1.1]: https://github.com/madsmtm/objc2/compare/objc-0.1.0...objc-0.1.1
