#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSGarbageCollector;
    unsafe impl ClassType for NSGarbageCollector {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSGarbageCollector {
        #[method_id(defaultCollector)]
        pub unsafe fn defaultCollector() -> Id<Object, Shared>;
        #[method(isCollecting)]
        pub unsafe fn isCollecting(&self) -> bool;
        #[method(disable)]
        pub unsafe fn disable(&self);
        #[method(enable)]
        pub unsafe fn enable(&self);
        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;
        #[method(collectIfNeeded)]
        pub unsafe fn collectIfNeeded(&self);
        #[method(collectExhaustively)]
        pub unsafe fn collectExhaustively(&self);
        #[method(disableCollectorForPointer:)]
        pub unsafe fn disableCollectorForPointer(&self, ptr: NonNull<c_void>);
        #[method(enableCollectorForPointer:)]
        pub unsafe fn enableCollectorForPointer(&self, ptr: NonNull<c_void>);
        #[method(zone)]
        pub unsafe fn zone(&self) -> NonNull<NSZone>;
    }
);
