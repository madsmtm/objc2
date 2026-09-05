//! # Bindings to the `FSKit` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/fskit/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-fs-kit/0.3.2")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

/// A tool to detect whether the directory contents changed since the last call to enumerate a directory.
///
/// Your implementation of ``FSVolume/Handler/enumerateDirectory(_:startingAt:verifier:attributes:packer:replyHandler:)`` defines the semantics of this value; it's opaque to FSKit.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/fskit/fsdirectoryverifier?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "FSVolume")]
pub type FSDirectoryVerifier = u64;

#[cfg(all(feature = "FSVolume", feature = "FSVolumeHandlerResult"))]
#[allow(non_snake_case)]
impl FSEnumerateDirectoryResult {
    objc2::extern_methods!(
        /// Creates a result instance with all required properties populated.
        ///
        /// - Parameters:
        /// - currentVerifier: An ``FSDirectoryVerifier`` value that reflects the directory's current version. This value is used to detect whether the directory contents changed since the last enumeration call.
        /// - Returns: A populated result instance, or `nil` if validation fails.
        #[unsafe(method(initWithVerifier:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithVerifier(
            this: objc2::rc::Allocated<Self>,
            current_verifier: FSDirectoryVerifier,
        ) -> Option<objc2::rc::Retained<Self>>;
    );
}
