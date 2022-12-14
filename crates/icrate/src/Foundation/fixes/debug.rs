use core::fmt;

use objc2::rc::Ownership;
use objc2::Message;

use crate::Foundation::{
    NSAttributedString, NSBundle, NSMutableArray, NSMutableAttributedString, NSMutableData,
    NSMutableDictionary, NSMutableSet, NSMutableString, NSNumber, NSObject, NSThread,
};

impl fmt::Debug for NSAttributedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use -[NSAttributedString description] since it is pretty good
        let obj: &NSObject = self;
        fmt::Debug::fmt(obj, f)
    }
}

impl fmt::Debug for NSBundle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to NSObject
        (**self).fmt(f)
    }
}

impl<T: fmt::Debug + Message, O: Ownership> fmt::Debug for NSMutableArray<T, O> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl fmt::Debug for NSMutableAttributedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl fmt::Debug for NSMutableData {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<K: fmt::Debug + Message, V: fmt::Debug + Message> fmt::Debug for NSMutableDictionary<K, V> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: fmt::Debug + Message, O: Ownership> fmt::Debug for NSMutableSet<T, O> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl fmt::Debug for NSMutableString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl fmt::Debug for NSThread {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use -[NSThread description] since that includes the thread number
        let obj: &NSObject = self;
        fmt::Debug::fmt(obj, f)
    }
}

impl fmt::Debug for NSNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to -[NSObject description]
        // (happens to return the same as -[NSNumber stringValue])
        fmt::Debug::fmt(&***self, f)
    }
}
