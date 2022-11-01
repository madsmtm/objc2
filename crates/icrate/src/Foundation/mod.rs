#[path = "../generated/Foundation/mod.rs"]
pub(crate) mod generated;

pub use objc2::ffi::NSIntegerMax;
pub use objc2::foundation::{CGFloat, CGPoint, CGRect, CGSize, NSRange, NSTimeInterval, NSZone};
pub use objc2::ns_string;

objc2::__inner_extern_class! {
    @__inner
    pub struct (NSObject) {}

    unsafe impl () for NSObject {
        INHERITS = [objc2::runtime::Object];
    }
}

unsafe impl objc2::ClassType for NSObject {
    type Super = objc2::runtime::Object;
    const NAME: &'static str = "NSObject";

    #[inline]
    fn class() -> &'static objc2::runtime::Class {
        objc2::class!(NSObject)
    }

    fn as_super(&self) -> &Self::Super {
        &self.__inner
    }

    fn as_super_mut(&mut self) -> &mut Self::Super {
        &mut self.__inner
    }
}
impl PartialEq for NSObject {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
impl Eq for NSObject {}
impl std::hash::Hash for NSObject {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        todo!()
    }
}
impl std::fmt::Debug for NSObject {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// TODO
pub type NSRangePointer = *const NSRange;

pub use self::generated::__exported::*;
