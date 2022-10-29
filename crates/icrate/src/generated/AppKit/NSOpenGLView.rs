#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSOpenGLView;
    unsafe impl ClassType for NSOpenGLView {
        type Super = NSView;
    }
);
extern_methods!(
    unsafe impl NSOpenGLView {
        #[method_id(defaultPixelFormat)]
        pub unsafe fn defaultPixelFormat() -> Id<NSOpenGLPixelFormat, Shared>;
        #[method_id(initWithFrame:pixelFormat:)]
        pub unsafe fn initWithFrame_pixelFormat(
            &self,
            frameRect: NSRect,
            format: Option<&NSOpenGLPixelFormat>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(openGLContext)]
        pub unsafe fn openGLContext(&self) -> Option<Id<NSOpenGLContext, Shared>>;
        #[method(setOpenGLContext:)]
        pub unsafe fn setOpenGLContext(&self, openGLContext: Option<&NSOpenGLContext>);
        #[method(clearGLContext)]
        pub unsafe fn clearGLContext(&self);
        #[method(update)]
        pub unsafe fn update(&self);
        #[method(reshape)]
        pub unsafe fn reshape(&self);
        #[method_id(pixelFormat)]
        pub unsafe fn pixelFormat(&self) -> Option<Id<NSOpenGLPixelFormat, Shared>>;
        #[method(setPixelFormat:)]
        pub unsafe fn setPixelFormat(&self, pixelFormat: Option<&NSOpenGLPixelFormat>);
        #[method(prepareOpenGL)]
        pub unsafe fn prepareOpenGL(&self);
        #[method(wantsBestResolutionOpenGLSurface)]
        pub unsafe fn wantsBestResolutionOpenGLSurface(&self) -> bool;
        #[method(setWantsBestResolutionOpenGLSurface:)]
        pub unsafe fn setWantsBestResolutionOpenGLSurface(
            &self,
            wantsBestResolutionOpenGLSurface: bool,
        );
        #[method(wantsExtendedDynamicRangeOpenGLSurface)]
        pub unsafe fn wantsExtendedDynamicRangeOpenGLSurface(&self) -> bool;
        #[method(setWantsExtendedDynamicRangeOpenGLSurface:)]
        pub unsafe fn setWantsExtendedDynamicRangeOpenGLSurface(
            &self,
            wantsExtendedDynamicRangeOpenGLSurface: bool,
        );
    }
);
extern_methods!(
    #[doc = "NSOpenGLSurfaceResolution"]
    unsafe impl NSView {
        #[method(wantsBestResolutionOpenGLSurface)]
        pub unsafe fn wantsBestResolutionOpenGLSurface(&self) -> bool;
        #[method(setWantsBestResolutionOpenGLSurface:)]
        pub unsafe fn setWantsBestResolutionOpenGLSurface(
            &self,
            wantsBestResolutionOpenGLSurface: bool,
        );
    }
);
extern_methods!(
    #[doc = "NSExtendedDynamicRange"]
    unsafe impl NSView {
        #[method(wantsExtendedDynamicRangeOpenGLSurface)]
        pub unsafe fn wantsExtendedDynamicRangeOpenGLSurface(&self) -> bool;
        #[method(setWantsExtendedDynamicRangeOpenGLSurface:)]
        pub unsafe fn setWantsExtendedDynamicRangeOpenGLSurface(
            &self,
            wantsExtendedDynamicRangeOpenGLSurface: bool,
        );
    }
);
