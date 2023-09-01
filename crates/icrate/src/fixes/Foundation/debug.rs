use core::fmt;

use crate::common::*;
use crate::Foundation;

#[cfg(feature = "Foundation_NSAttributedString")]
impl fmt::Debug for Foundation::NSAttributedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use -[NSAttributedString description] since it is pretty good
        let obj: &Foundation::NSObject = self;
        fmt::Debug::fmt(obj, f)
    }
}

#[cfg(feature = "Foundation_NSBundle")]
impl fmt::Debug for Foundation::NSBundle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to NSObject
        (**self).fmt(f)
    }
}

#[cfg(feature = "Foundation_NSMutableArray")]
impl<T: fmt::Debug + Message> fmt::Debug for Foundation::NSMutableArray<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

#[cfg(feature = "Foundation_NSMutableAttributedString")]
impl fmt::Debug for Foundation::NSMutableAttributedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

#[cfg(feature = "Foundation_NSMutableData")]
impl fmt::Debug for Foundation::NSMutableData {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

#[cfg(feature = "Foundation_NSMutableDictionary")]
impl<K: fmt::Debug + Message, V: fmt::Debug + Message> fmt::Debug
    for Foundation::NSMutableDictionary<K, V>
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

#[cfg(feature = "Foundation_NSMutableSet")]
impl<T: fmt::Debug + Message> fmt::Debug for Foundation::NSMutableSet<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

#[cfg(feature = "Foundation_NSMutableString")]
impl fmt::Debug for Foundation::NSMutableString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

#[cfg(feature = "Foundation_NSThread")]
impl fmt::Debug for Foundation::NSThread {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use -[NSThread description] since that includes the thread number
        let obj: &Foundation::NSObject = self;
        fmt::Debug::fmt(obj, f)
    }
}

#[cfg(feature = "Foundation_NSNumber")]
impl fmt::Debug for Foundation::NSNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to -[NSObject description]
        // (happens to return the same as -[NSNumber stringValue])
        fmt::Debug::fmt(&***self, f)
    }
}
