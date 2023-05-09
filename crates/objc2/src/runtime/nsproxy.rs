use core::fmt;
use core::hash;

use crate::mutability::Root;
use crate::runtime::{Class, NSObject, NSObjectProtocol, Object, ProtocolObject};
use crate::ClassType;

crate::__emit_struct! {
    (
        /// An abstract superclass defining an API for objects that act as
        /// stand-ins for other objects or for objects that don’t exist yet.
        ///
        /// See [Apple's documentation][apple-doc] for more information.
        ///
        /// [apple-doc]: https://developer.apple.com/documentation/foundation/nsproxy?language=objc
    )
    (pub)
    (NSProxy)
    (
        __inner: Object,
    )
}

crate::__extern_class_impl_traits! {
    unsafe impl () for NSProxy {
        INHERITS = [Object];

        fn as_super(&self) {
            &self.__inner
        }

        fn as_super_mut(&mut self) {
            &mut self.__inner
        }
    }
}

unsafe impl ClassType for NSProxy {
    type Super = Object;
    type Mutability = Root;
    const NAME: &'static str = "NSProxy";

    #[inline]
    fn class() -> &'static Class {
        #[cfg(feature = "apple")]
        {
            crate::class!(NSProxy)
        }
        #[cfg(feature = "gnustep-1-7")]
        {
            extern "C" {
                // The linking changed in libobjc2 v2.0
                #[cfg_attr(feature = "gnustep-2-0", link_name = "._OBJC_CLASS_NSProxy")]
                #[cfg_attr(not(feature = "gnustep-2-0"), link_name = "_OBJC_CLASS_NSProxy")]
                static OBJC_CLASS_NSProxy: Class;
                // Others:
                // __objc_class_name_NSProxy
                // _OBJC_CLASS_REF_NSProxy
            }

            unsafe { &OBJC_CLASS_NSProxy }
        }
    }

    #[inline]
    fn as_super(&self) -> &Self::Super {
        &self.__inner
    }

    #[inline]
    fn as_super_mut(&mut self) -> &mut Self::Super {
        &mut self.__inner
    }
}

unsafe impl NSObjectProtocol for NSProxy {}

impl PartialEq for NSProxy {
    #[inline]
    #[doc(alias = "isEqual:")]
    fn eq(&self, other: &Self) -> bool {
        self.__isEqual(other)
    }
}

impl Eq for NSProxy {}

impl hash::Hash for NSProxy {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.__hash().hash(state);
    }
}

impl fmt::Debug for NSProxy {
    #[inline]
    #[doc(alias = "description")]
    #[doc(alias = "debugDescription")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let obj: &ProtocolObject<NSObject> = ProtocolObject::from_ref(self);
        obj.fmt(f)
    }
}
