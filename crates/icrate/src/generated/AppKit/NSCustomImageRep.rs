#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSCustomImageRep;
    unsafe impl ClassType for NSCustomImageRep {
        type Super = NSImageRep;
    }
);
extern_methods!(
    unsafe impl NSCustomImageRep {
        #[method_id(initWithSize:flipped:drawingHandler:)]
        pub unsafe fn initWithSize_flipped_drawingHandler(
            &self,
            size: NSSize,
            drawingHandlerShouldBeCalledWithFlippedContext: bool,
            drawingHandler: TodoBlock,
        ) -> Id<Self, Shared>;
        #[method(drawingHandler)]
        pub unsafe fn drawingHandler(&self) -> TodoBlock;
        #[method_id(initWithDrawSelector:delegate:)]
        pub unsafe fn initWithDrawSelector_delegate(
            &self,
            selector: Sel,
            delegate: &Object,
        ) -> Id<Self, Shared>;
        #[method(drawSelector)]
        pub unsafe fn drawSelector(&self) -> Option<Sel>;
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<Object, Shared>>;
    }
);
