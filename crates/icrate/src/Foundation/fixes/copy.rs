use alloc::borrow::ToOwned;

use crate::common::*;
use crate::Foundation::{self, NSCopying, NSMutableCopying};

// Arrays

/// This is implemented as a shallow copy.
///
/// As such, it is only possible when the array's contents are `Shared`.
#[cfg(feature = "Foundation_NSArray")]
unsafe impl<T: Message> NSCopying for Foundation::NSArray<T, Shared> {
    type Ownership = Shared;
    type Output = Foundation::NSArray<T, Shared>;
}

/// This is implemented as a shallow copy.
#[cfg(feature = "Foundation_NSArray")]
#[cfg(feature = "Foundation_NSMutableArray")]
unsafe impl<T: Message> NSMutableCopying for Foundation::NSArray<T, Shared> {
    type Output = Foundation::NSMutableArray<T, Shared>;
}

#[cfg(feature = "Foundation_NSArray")]
impl<T: Message> ToOwned for Foundation::NSArray<T, Shared> {
    type Owned = Id<Foundation::NSArray<T, Shared>, Shared>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

// Copying only possible when ItemOwnership = Shared

/// This is implemented as a shallow copy.
#[cfg(feature = "Foundation_NSMutableArray")]
unsafe impl<T: Message> NSCopying for Foundation::NSMutableArray<T, Shared> {
    type Ownership = Shared;
    type Output = Foundation::NSArray<T, Shared>;
}

/// This is implemented as a shallow copy.
#[cfg(feature = "Foundation_NSMutableArray")]
unsafe impl<T: Message> NSMutableCopying for Foundation::NSMutableArray<T, Shared> {
    type Output = Foundation::NSMutableArray<T, Shared>;
}

#[cfg(feature = "Foundation_NSMutableArray")]
impl<T: Message> ToOwned for Foundation::NSMutableArray<T, Shared> {
    type Owned = Id<Foundation::NSMutableArray<T, Shared>, Owned>;
    fn to_owned(&self) -> Self::Owned {
        self.mutable_copy()
    }
}

// Data

#[cfg(feature = "Foundation_NSData")]
unsafe impl NSCopying for Foundation::NSData {
    type Ownership = Shared;
    type Output = Foundation::NSData;
}

#[cfg(feature = "Foundation_NSData")]
#[cfg(feature = "Foundation_NSMutableData")]
unsafe impl NSMutableCopying for Foundation::NSData {
    type Output = Foundation::NSMutableData;
}

#[cfg(feature = "Foundation_NSData")]
impl ToOwned for Foundation::NSData {
    type Owned = Id<Foundation::NSData, Shared>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "Foundation_NSMutableData")]
unsafe impl NSCopying for Foundation::NSMutableData {
    type Ownership = Shared;
    type Output = Foundation::NSData;
}

#[cfg(feature = "Foundation_NSMutableData")]
unsafe impl NSMutableCopying for Foundation::NSMutableData {
    type Output = Foundation::NSMutableData;
}

#[cfg(feature = "Foundation_NSMutableData")]
impl ToOwned for Foundation::NSMutableData {
    type Owned = Id<Foundation::NSMutableData, Owned>;
    fn to_owned(&self) -> Self::Owned {
        self.mutable_copy()
    }
}

// Errors

#[cfg(feature = "Foundation_NSError")]
unsafe impl NSCopying for Foundation::NSError {
    type Ownership = Shared;
    type Output = Self;
}

#[cfg(feature = "Foundation_NSException")]
unsafe impl NSCopying for Foundation::NSException {
    type Ownership = Shared;
    type Output = Foundation::NSException;
}

#[cfg(feature = "Foundation_NSException")]
impl ToOwned for Foundation::NSException {
    type Owned = Id<Foundation::NSException, Shared>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

// Sets

#[cfg(feature = "Foundation_NSSet")]
unsafe impl<T: Message> NSCopying for Foundation::NSSet<T, Shared> {
    type Ownership = Shared;
    type Output = Foundation::NSSet<T, Shared>;
}

#[cfg(feature = "Foundation_NSSet")]
#[cfg(feature = "Foundation_NSMutableSet")]
unsafe impl<T: Message> NSMutableCopying for Foundation::NSSet<T, Shared> {
    type Output = Foundation::NSMutableSet<T, Shared>;
}

#[cfg(feature = "Foundation_NSSet")]
impl<T: Message> ToOwned for Foundation::NSSet<T, Shared> {
    type Owned = Id<Foundation::NSSet<T, Shared>, Shared>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "Foundation_NSMutableSet")]
unsafe impl<T: Message> NSCopying for Foundation::NSMutableSet<T, Shared> {
    type Ownership = Shared;
    type Output = Foundation::NSSet<T, Shared>;
}

#[cfg(feature = "Foundation_NSMutableSet")]
unsafe impl<T: Message> NSMutableCopying for Foundation::NSMutableSet<T, Shared> {
    type Output = Foundation::NSMutableSet<T, Shared>;
}

#[cfg(feature = "Foundation_NSMutableSet")]
impl<T: Message> ToOwned for Foundation::NSMutableSet<T, Shared> {
    type Owned = Id<Foundation::NSMutableSet<T, Shared>, Owned>;
    fn to_owned(&self) -> Self::Owned {
        self.mutable_copy()
    }
}

// Strings

#[cfg(feature = "Foundation_NSString")]
unsafe impl NSCopying for Foundation::NSString {
    type Ownership = Shared;
    type Output = Foundation::NSString;
}

#[cfg(feature = "Foundation_NSString")]
#[cfg(feature = "Foundation_NSMutableString")]
unsafe impl NSMutableCopying for Foundation::NSString {
    type Output = Foundation::NSMutableString;
}

#[cfg(feature = "Foundation_NSString")]
impl ToOwned for Foundation::NSString {
    type Owned = Id<Foundation::NSString, Shared>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "Foundation_NSMutableString")]
unsafe impl NSCopying for Foundation::NSMutableString {
    type Ownership = Shared;
    type Output = Foundation::NSString;
}

#[cfg(feature = "Foundation_NSMutableString")]
unsafe impl NSMutableCopying for Foundation::NSMutableString {
    type Output = Foundation::NSMutableString;
}

#[cfg(feature = "Foundation_NSMutableString")]
impl ToOwned for Foundation::NSMutableString {
    type Owned = Id<Foundation::NSMutableString, Owned>;
    fn to_owned(&self) -> Self::Owned {
        self.mutable_copy()
    }
}

#[cfg(feature = "Foundation_NSAttributedString")]
unsafe impl NSCopying for Foundation::NSAttributedString {
    type Ownership = Shared;
    type Output = Foundation::NSAttributedString;
}

#[cfg(feature = "Foundation_NSAttributedString")]
#[cfg(feature = "Foundation_NSMutableAttributedString")]
unsafe impl NSMutableCopying for Foundation::NSAttributedString {
    type Output = Foundation::NSMutableAttributedString;
}

#[cfg(feature = "Foundation_NSAttributedString")]
impl ToOwned for Foundation::NSAttributedString {
    type Owned = Id<Foundation::NSAttributedString, Shared>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "Foundation_NSMutableAttributedString")]
unsafe impl NSCopying for Foundation::NSMutableAttributedString {
    type Ownership = Shared;
    type Output = Foundation::NSAttributedString;
}

#[cfg(feature = "Foundation_NSMutableAttributedString")]
unsafe impl NSMutableCopying for Foundation::NSMutableAttributedString {
    type Output = Foundation::NSMutableAttributedString;
}

#[cfg(feature = "Foundation_NSMutableAttributedString")]
impl ToOwned for Foundation::NSMutableAttributedString {
    type Owned = Id<Foundation::NSMutableAttributedString, Owned>;
    fn to_owned(&self) -> Self::Owned {
        self.mutable_copy()
    }
}

// UUID

#[cfg(feature = "Foundation_NSUUID")]
unsafe impl NSCopying for Foundation::NSUUID {
    type Ownership = Shared;
    type Output = Foundation::NSUUID;
}

#[cfg(feature = "Foundation_NSUUID")]
impl ToOwned for Foundation::NSUUID {
    type Owned = Id<Foundation::NSUUID, Shared>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

// Value

#[cfg(feature = "Foundation_NSValue")]
unsafe impl NSCopying for Foundation::NSValue {
    type Ownership = Shared;
    type Output = Foundation::NSValue;
}

#[cfg(feature = "Foundation_NSValue")]
impl ToOwned for Foundation::NSValue {
    type Owned = Id<Foundation::NSValue, Shared>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "Foundation_NSNumber")]
unsafe impl NSCopying for Foundation::NSNumber {
    type Ownership = Shared;
    type Output = Foundation::NSNumber;
}

#[cfg(feature = "Foundation_NSNumber")]
impl ToOwned for Foundation::NSNumber {
    type Owned = Id<Foundation::NSNumber, Shared>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}
