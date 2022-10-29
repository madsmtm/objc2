#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSOpenGLLayer;
    unsafe impl ClassType for NSOpenGLLayer {
        type Super = CAOpenGLLayer;
    }
);
extern_methods!(
    unsafe impl NSOpenGLLayer {
        #[method_id(view)]
        pub unsafe fn view(&self) -> Option<Id<NSView, Shared>>;
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);
        #[method_id(openGLPixelFormat)]
        pub unsafe fn openGLPixelFormat(&self) -> Option<Id<NSOpenGLPixelFormat, Shared>>;
        #[method(setOpenGLPixelFormat:)]
        pub unsafe fn setOpenGLPixelFormat(&self, openGLPixelFormat: Option<&NSOpenGLPixelFormat>);
        #[method_id(openGLContext)]
        pub unsafe fn openGLContext(&self) -> Option<Id<NSOpenGLContext, Shared>>;
        #[method(setOpenGLContext:)]
        pub unsafe fn setOpenGLContext(&self, openGLContext: Option<&NSOpenGLContext>);
        #[method_id(openGLPixelFormatForDisplayMask:)]
        pub unsafe fn openGLPixelFormatForDisplayMask(
            &self,
            mask: u32,
        ) -> Id<NSOpenGLPixelFormat, Shared>;
        #[method_id(openGLContextForPixelFormat:)]
        pub unsafe fn openGLContextForPixelFormat(
            &self,
            pixelFormat: &NSOpenGLPixelFormat,
        ) -> Id<NSOpenGLContext, Shared>;
        #[method(canDrawInOpenGLContext:pixelFormat:forLayerTime:displayTime:)]
        pub unsafe fn canDrawInOpenGLContext_pixelFormat_forLayerTime_displayTime(
            &self,
            context: &NSOpenGLContext,
            pixelFormat: &NSOpenGLPixelFormat,
            t: CFTimeInterval,
            ts: NonNull<CVTimeStamp>,
        ) -> bool;
        #[method(drawInOpenGLContext:pixelFormat:forLayerTime:displayTime:)]
        pub unsafe fn drawInOpenGLContext_pixelFormat_forLayerTime_displayTime(
            &self,
            context: &NSOpenGLContext,
            pixelFormat: &NSOpenGLPixelFormat,
            t: CFTimeInterval,
            ts: NonNull<CVTimeStamp>,
        );
    }
);
