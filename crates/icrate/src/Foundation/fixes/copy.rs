use alloc::borrow::ToOwned;

use crate::common::*;
use crate::Foundation::{self, NSCopying, NSMutableCopying};

#[cfg(feature = "Foundation_NSArray")]
impl<T: IsIdCloneable> ToOwned for Foundation::NSArray<T> {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "Foundation_NSMutableArray")]
impl<T: IsIdCloneable> ToOwned for Foundation::NSMutableArray<T> {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.mutableCopy()
    }
}

#[cfg(feature = "Foundation_NSData")]
impl ToOwned for Foundation::NSData {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "Foundation_NSMutableData")]
impl ToOwned for Foundation::NSMutableData {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.mutableCopy()
    }
}

#[cfg(feature = "Foundation_NSException")]
impl ToOwned for Foundation::NSException {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "Foundation_NSSet")]
impl<T: IsIdCloneable> ToOwned for Foundation::NSSet<T> {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "Foundation_NSMutableSet")]
impl<T: IsIdCloneable> ToOwned for Foundation::NSMutableSet<T> {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.mutableCopy()
    }
}

#[cfg(feature = "Foundation_NSString")]
impl ToOwned for Foundation::NSString {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "Foundation_NSMutableString")]
impl ToOwned for Foundation::NSMutableString {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.mutableCopy()
    }
}

#[cfg(feature = "Foundation_NSAttributedString")]
impl ToOwned for Foundation::NSAttributedString {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "Foundation_NSMutableAttributedString")]
impl ToOwned for Foundation::NSMutableAttributedString {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.mutableCopy()
    }
}

#[cfg(feature = "Foundation_NSUUID")]
impl ToOwned for Foundation::NSUUID {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "Foundation_NSValue")]
impl ToOwned for Foundation::NSValue {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "Foundation_NSNumber")]
impl ToOwned for Foundation::NSNumber {
    type Owned = Id<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}
