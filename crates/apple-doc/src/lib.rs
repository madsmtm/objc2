//! # Parse Apple's documentation
//!
//! The data comes from Xcode's Developer Documentation viewer, and resides in
//! `/Applications/Xcode.app/Contents/SharedFrameworks/DNTDocumentationSupport.framework/Versions/A/Resources/external`.
//!
//! This uses, I'm serious, **7** different wrapper formats: Brotli, LDMB,
//! JSON, SQLite, PList and two custom binary formats, along with both SHA1
//! hashing and Base64 encoding for good measure.
//!
//! This crate handles unwrangling the data from all these.
//!
//! See also [Swift-DocC's `Indexing` folder][docc-indexing], it seems to have
//! some similar code. Additionally, Apple has some of the data available
//! online at links [like] [these].
//!
//! [docc-indexing]: https://github.com/swiftlang/swift-docc/tree/main/Sources/SwiftDocC/Indexing
//! [like]: https://developer.apple.com/tutorials/data/index/appkit
//! [these]: https://developer.apple.com/tutorials/data/documentation/appkit/views-and-controls.json?language=objc
//!
//! ## Overview
//!
//! The structure of the `external/` folder on disk looks as follows, with
//! the structure used to parse it annotated next to it.
//! - fs/ ([`BlobStore`])
//!   - 1
//!   - 2
//!   - ...
//! - index/
//!   - availability.index ([`NavigatorTree`])
//!   - data.mdb ([`Lmdb`])
//!   - navigator.index ([`AvailabilityLookup`])
//! - 0.txt ([`TxtItem`])
//! - 1.txt ([`TxtItem`])
//! - 3.txt ([`TxtItem`])
//! - cache.db ([`SqliteDb`])
//! - words.plist (none)
//!
//! To start with, we can get the [`NavigatorTree`]. This contains a tree of
//! [`NavigatorItem`]s, each with a unique `u32` ID, and is used for
//! presenting the sidebar in Xcode's Developer Documentation viewer.
//!
//! [`NavigatorItem::availability_id`] can be turned into a list of
//! [`PlatformAvailabilityId`]s using [`Lmdb::availability`], which can then
//! be used in [`AvailabilityLookup::data`] to give a list of
//! [`PlatformAvailability`]. This is used when filtering the sidebar.
//!
//! You can also take the ID of the [`NavigatorItem`] and use it in
//! [`Lmdb::url_from_navigator_id`] to look up a string identifier for the
//! item.
//!
//! This unique string identifier corresponds to the path component of the URL
//! at which this item can be found on on [Apple's developer documentation
//! website](https://developer.apple.com/), prefixed with the language that we
//! want the documentation for. E.g. the `NSObject` class would have the URL
//! component `"/documentation/objectivec/nsobject-swift.class"`, and either
//! the prefix `"objective-c"` or `"swift"`.
//!
//! The URL can be shortened to an identifier using the [`url_to_uuid`]
//! function. This identifier can be used in [`SqliteDb::get_ref`] to look up
//! a cache entry.
//!
//! The cache entry can be used in [`BlobStore`] to get and decompress the
//! data in `fs`. The data contains either a [`Doc`] or an image (though
//! images are rare, they're usually only stored on Apple's servers).
//!
//! [`TxtItem`] contains the same URL IDs, names and [`Kind`]s of items. It's
//! presumably useful for building a search index.
//!
//! As an ASCII diagram:
//!
//! ```text
//!                 NavigatorTree
//!                 availability_id -------> Lmdb::availability
//!                      ID                          |
//!                       |                  AvailabilityLookup
//!                  Lmdb::index
//!                       |
//!                      URL
//!                       |
//!                    hashing
//!                       |
//!   TxtItem --------> UUID
//!                       |
//!                SQLiteDb::get_ref
//!                       |
//!                   BlobStore
//! ```

mod availability;
mod common;
mod fs;
mod fs_json;
mod lmdb;
mod navigator;
mod sqlite;
mod txt;

pub use self::availability::*;
pub use self::common::*;
pub use self::fs::*;
pub use self::fs_json::*;
pub use self::lmdb::*;
pub use self::navigator::*;
pub use self::sqlite::*;
pub use self::txt::*;
