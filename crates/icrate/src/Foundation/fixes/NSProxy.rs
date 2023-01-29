#![cfg(feature = "Foundation_NSProxy")]
use core::fmt;
use core::hash;

use crate::common::*;

objc2::__emit_struct! {
    ()
    (pub)
    (NSProxy)
    (
        __inner: Object,
    )
}

objc2::__extern_class_impl_traits! {
    unsafe impl () for NSProxy {
        INHERITS = [Object];
    }
}

unsafe impl ClassType for NSProxy {
    type Super = Object;
    const NAME: &'static str = "NSProxy";

    #[inline]
    fn class() -> &'static Class {
        objc2::class!(NSProxy)
    }

    fn as_super(&self) -> &Self::Super {
        &self.__inner
    }

    fn as_super_mut(&mut self) -> &mut Self::Super {
        &mut self.__inner
    }
}

unsafe impl NSObjectProtocol for NSProxy {}

impl PartialEq for NSProxy {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let obj: &ProtocolObject<NSObject> = ProtocolObject::from_ref(self);
        obj.fmt(f)
    }
}
