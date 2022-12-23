use core::fmt;
use core::hash;

use objc2::runtime::{Class, Object};
use objc2::{ClassType, __inner_extern_class};

__inner_extern_class! {
    @__inner
    pub struct (NSProxy) {}

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

impl PartialEq for NSProxy {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}

impl Eq for NSProxy {}

impl hash::Hash for NSProxy {
    #[inline]
    fn hash<H: hash::Hasher>(&self, _state: &mut H) {
        todo!()
    }
}

impl fmt::Debug for NSProxy {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
