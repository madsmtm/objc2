//! # Bindings to Apple's frameworks
//!
//! `icrate` is an interface to Apple's Objective-C frameworks like
//! AppKit, Foundation, Metal, WebKit, and so on.
//!
//! Quick links:
//! - [Tutorial][self::__tutorial]
//!
//! The bindings are mostly automatically generated, and currently contain
//! very little documentation, you should view [Apple's developer
//! documentation][apple-doc-index] for detailed information about each API.
//! (There are [plans][#309] for importing that documentation here).
//!
//! [#309]: https://github.com/madsmtm/objc2/issues/309
//! [apple-doc-index]: https://developer.apple.com/documentation/technologies
//!
//!
//! ## Use of `Deref`
//!
//! `icrate` uses the [`Deref`] trait in a bit special way: All objects deref
//! to their superclasses. For example, `NSMutableArray` derefs to `NSArray`,
//! which in turn derefs to `NSObject`.
//!
//! Note that this is explicitly recommended against in [the
//! documentation][`Deref`] and [the Rust Design patterns
//! book][anti-pattern-deref] (see those links for details).
//!
//! Due to Objective-C objects only ever being accessible behind pointers in
//! the first place, the problems stated there are less severe, and having the
//! implementation just means that everything is much nicer when you actually
//! want to use the objects!
//!
//! All objects also implement [`AsRef`] and [`AsMut`] to their superclass,
//! and can be used in [`Id::into_super`], so if you favour explicit
//! conversion, that is a possibility too.
//!
//! [`Deref`]: std::ops::Deref
//! [`ClassType`]: crate::objc2::ClassType
//! [anti-pattern-deref]: https://rust-unofficial.github.io/patterns/anti_patterns/deref.html
//! [`Id::into_super`]: objc2::rc::Id::into_super
//!
//!
//! ## Rust vs. Objective-C types
//!
//! A quick overview of some types you will encounter often in Objective-C,
//! and their approximate Rust equivalent.
//!
//! | Objective-C | (approximately) equivalent Rust |
//! | --- | --- |
//! | `NSData*` | `Arc<[u8]>` |
//! | `NSMutableData*` | `Vec<u8>` |
//! | `NSString*` | `Arc<str>` |
//! | `NSMutableString*` | `String` |
//! | `NSValue*` | `Arc<dyn Any>` |
//! | `NSNumber*` | `Arc<enum { I8(i8), U8(u8), I16(i16), U16(u16), I32(i32), U32(u32), I64(i64), U64(u64), F32(f32), F64(f64), CLong(ffi::c_long), CULong(ffi::c_ulong) }>` |
//! | `NSError*` | `Arc<dyn Error + Send + Sync>` |
//! | `NSException*` | `Arc<dyn Error + Send + Sync>` |
//! | `NSRange` | `ops::Range<usize>` |
//! | `NSComparisonResult` | `cmp::Ordering` |
//! | `NSArray<T>*` | `Arc<[T]>` |
//! | `NSMutableArray<T>*` | `Vec<T>` |
//! | `NSDictionary<K, V>*` | `Arc<HashMap<K, V>>` |
//! | `NSMutableDictionary<K, V>*` | `HashMap<K, V>` |
//! | `NSEnumerator<T>*` | `Box<dyn Iterator<T>>` |
//! | `NSCopying*` | `Box<dyn Clone>` |
//!
//!
//! ## Example
//!
//! ```console
//! $ cargo add icrate --features=Foundation,Foundation_all
//! ```
//!
#![cfg_attr(
    all(
        feature = "Foundation",
        feature = "Foundation_NSArray",
        feature = "Foundation_NSString"
    ),
    doc = "```"
)]
#![cfg_attr(
    not(all(
        feature = "Foundation",
        feature = "Foundation_NSArray",
        feature = "Foundation_NSString"
    )),
    doc = "```ignore"
)]
//! use icrate::Foundation::{ns_string, NSCopying, NSArray};
//!
//! let string = ns_string!("world");
//! println!("hello {string}");
//!
//! let array = NSArray::from_id_slice(&[string.copy()]);
//! println!("{array:?}");
//! ```
#![no_std]
#![cfg_attr(feature = "unstable-docsrs", feature(doc_auto_cfg))]
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![deny(non_ascii_idents)]
#![warn(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::cargo)]
#![warn(clippy::ptr_as_ptr)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/icrate/0.0.4")]
#![recursion_limit = "512"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
extern "C" {}

#[cfg(feature = "objective-c")]
pub extern crate objc2;

#[cfg(feature = "block")]
pub extern crate block2;

mod common;
#[macro_use]
mod macros;
#[cfg(any(doc, doctest))]
pub mod __tutorial;
#[allow(unreachable_pub)]
#[allow(unused_imports)]
#[allow(deprecated)]
mod generated;

#[allow(unreachable_pub)]
pub use self::generated::*;

/// Deprecated alias of [`Foundation::ns_string`].
#[macro_export]
#[deprecated = "use icrate::Foundation::ns_string instead"]
#[cfg(feature = "Foundation_NSString")]
macro_rules! ns_string {
    ($s:expr) => {
        $crate::Foundation::ns_string!($s)
    };
}
