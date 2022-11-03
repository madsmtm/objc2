//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTouchPhase {
        NSTouchPhaseBegan = 1 << 0,
        NSTouchPhaseMoved = 1 << 1,
        NSTouchPhaseStationary = 1 << 2,
        NSTouchPhaseEnded = 1 << 3,
        NSTouchPhaseCancelled = 1 << 4,
        NSTouchPhaseTouching = NSTouchPhaseBegan | NSTouchPhaseMoved | NSTouchPhaseStationary,
        NSTouchPhaseAny = 18446744073709551615,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTouchType {
        NSTouchTypeDirect = 0,
        NSTouchTypeIndirect = 1,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTouchTypeMask {
        NSTouchTypeMaskDirect = 1 << NSTouchTypeDirect,
        NSTouchTypeMaskIndirect = 1 << NSTouchTypeIndirect,
    }
);

inline_fn!(
    pub unsafe fn NSTouchTypeMaskFromType(type_: NSTouchType) -> NSTouchTypeMask {
        todo!()
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSTouch;

    unsafe impl ClassType for NSTouch {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSTouch {
        #[method_id(@__retain_semantics Other identity)]
        pub unsafe fn identity(&self) -> Id<TodoProtocols, Shared>;

        #[method(phase)]
        pub unsafe fn phase(&self) -> NSTouchPhase;

        #[method(normalizedPosition)]
        pub unsafe fn normalizedPosition(&self) -> NSPoint;

        #[method(isResting)]
        pub unsafe fn isResting(&self) -> bool;

        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Option<Id<Object, Shared>>;

        #[method(deviceSize)]
        pub unsafe fn deviceSize(&self) -> NSSize;
    }
);

extern_methods!(
    /// NSTouchBar
    unsafe impl NSTouch {
        #[method(type)]
        pub unsafe fn type_(&self) -> NSTouchType;

        #[method(locationInView:)]
        pub unsafe fn locationInView(&self, view: Option<&NSView>) -> NSPoint;

        #[method(previousLocationInView:)]
        pub unsafe fn previousLocationInView(&self, view: Option<&NSView>) -> NSPoint;
    }
);
