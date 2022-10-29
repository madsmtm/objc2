#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSOpenGLPixelFormatAttribute = u32;
extern_class!(
    #[derive(Debug)]
    pub struct NSOpenGLPixelFormat;
    unsafe impl ClassType for NSOpenGLPixelFormat {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSOpenGLPixelFormat {
        #[method_id(initWithCGLPixelFormatObj:)]
        pub unsafe fn initWithCGLPixelFormatObj(
            &self,
            format: CGLPixelFormatObj,
        ) -> Option<Id<NSOpenGLPixelFormat, Shared>>;
        #[method_id(initWithAttributes:)]
        pub unsafe fn initWithAttributes(
            &self,
            attribs: NonNull<NSOpenGLPixelFormatAttribute>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithData:)]
        pub unsafe fn initWithData(&self, attribs: Option<&NSData>) -> Option<Id<Object, Shared>>;
        #[method_id(attributes)]
        pub unsafe fn attributes(&self) -> Option<Id<NSData, Shared>>;
        #[method(setAttributes:)]
        pub unsafe fn setAttributes(&self, attribs: Option<&NSData>);
        #[method(getValues:forAttribute:forVirtualScreen:)]
        pub unsafe fn getValues_forAttribute_forVirtualScreen(
            &self,
            vals: NonNull<GLint>,
            attrib: NSOpenGLPixelFormatAttribute,
            screen: GLint,
        );
        #[method(numberOfVirtualScreens)]
        pub unsafe fn numberOfVirtualScreens(&self) -> GLint;
        #[method(CGLPixelFormatObj)]
        pub unsafe fn CGLPixelFormatObj(&self) -> CGLPixelFormatObj;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSOpenGLPixelBuffer;
    unsafe impl ClassType for NSOpenGLPixelBuffer {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSOpenGLPixelBuffer {
        #[method_id(initWithTextureTarget:textureInternalFormat:textureMaxMipMapLevel:pixelsWide:pixelsHigh:)]
        pub unsafe fn initWithTextureTarget_textureInternalFormat_textureMaxMipMapLevel_pixelsWide_pixelsHigh(
            &self,
            target: GLenum,
            format: GLenum,
            maxLevel: GLint,
            pixelsWide: GLsizei,
            pixelsHigh: GLsizei,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithCGLPBufferObj:)]
        pub unsafe fn initWithCGLPBufferObj(
            &self,
            pbuffer: CGLPBufferObj,
        ) -> Option<Id<NSOpenGLPixelBuffer, Shared>>;
        #[method(CGLPBufferObj)]
        pub unsafe fn CGLPBufferObj(&self) -> CGLPBufferObj;
        #[method(pixelsWide)]
        pub unsafe fn pixelsWide(&self) -> GLsizei;
        #[method(pixelsHigh)]
        pub unsafe fn pixelsHigh(&self) -> GLsizei;
        #[method(textureTarget)]
        pub unsafe fn textureTarget(&self) -> GLenum;
        #[method(textureInternalFormat)]
        pub unsafe fn textureInternalFormat(&self) -> GLenum;
        #[method(textureMaxMipMapLevel)]
        pub unsafe fn textureMaxMipMapLevel(&self) -> GLint;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSOpenGLContext;
    unsafe impl ClassType for NSOpenGLContext {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSOpenGLContext {
        #[method_id(initWithFormat:shareContext:)]
        pub unsafe fn initWithFormat_shareContext(
            &self,
            format: &NSOpenGLPixelFormat,
            share: Option<&NSOpenGLContext>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithCGLContextObj:)]
        pub unsafe fn initWithCGLContextObj(
            &self,
            context: CGLContextObj,
        ) -> Option<Id<NSOpenGLContext, Shared>>;
        #[method_id(pixelFormat)]
        pub unsafe fn pixelFormat(&self) -> Id<NSOpenGLPixelFormat, Shared>;
        #[method_id(view)]
        pub unsafe fn view(&self) -> Option<Id<NSView, Shared>>;
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);
        #[method(setFullScreen)]
        pub unsafe fn setFullScreen(&self);
        #[method(setOffScreen:width:height:rowbytes:)]
        pub unsafe fn setOffScreen_width_height_rowbytes(
            &self,
            baseaddr: NonNull<c_void>,
            width: GLsizei,
            height: GLsizei,
            rowbytes: GLint,
        );
        #[method(clearDrawable)]
        pub unsafe fn clearDrawable(&self);
        #[method(update)]
        pub unsafe fn update(&self);
        #[method(flushBuffer)]
        pub unsafe fn flushBuffer(&self);
        #[method(makeCurrentContext)]
        pub unsafe fn makeCurrentContext(&self);
        #[method(clearCurrentContext)]
        pub unsafe fn clearCurrentContext();
        #[method_id(currentContext)]
        pub unsafe fn currentContext() -> Option<Id<NSOpenGLContext, Shared>>;
        #[method(copyAttributesFromContext:withMask:)]
        pub unsafe fn copyAttributesFromContext_withMask(
            &self,
            context: &NSOpenGLContext,
            mask: GLbitfield,
        );
        #[method(setValues:forParameter:)]
        pub unsafe fn setValues_forParameter(
            &self,
            vals: NonNull<GLint>,
            param: NSOpenGLContextParameter,
        );
        #[method(getValues:forParameter:)]
        pub unsafe fn getValues_forParameter(
            &self,
            vals: NonNull<GLint>,
            param: NSOpenGLContextParameter,
        );
        #[method(currentVirtualScreen)]
        pub unsafe fn currentVirtualScreen(&self) -> GLint;
        #[method(setCurrentVirtualScreen:)]
        pub unsafe fn setCurrentVirtualScreen(&self, currentVirtualScreen: GLint);
        #[method(createTexture:fromView:internalFormat:)]
        pub unsafe fn createTexture_fromView_internalFormat(
            &self,
            target: GLenum,
            view: &NSView,
            format: GLenum,
        );
        #[method(CGLContextObj)]
        pub unsafe fn CGLContextObj(&self) -> CGLContextObj;
    }
);
extern_methods!(
    #[doc = "NSOpenGLPixelBuffer"]
    unsafe impl NSOpenGLContext {
        #[method(setPixelBuffer:cubeMapFace:mipMapLevel:currentVirtualScreen:)]
        pub unsafe fn setPixelBuffer_cubeMapFace_mipMapLevel_currentVirtualScreen(
            &self,
            pixelBuffer: &NSOpenGLPixelBuffer,
            face: GLenum,
            level: GLint,
            screen: GLint,
        );
        #[method_id(pixelBuffer)]
        pub unsafe fn pixelBuffer(&self) -> Option<Id<NSOpenGLPixelBuffer, Shared>>;
        #[method(pixelBufferCubeMapFace)]
        pub unsafe fn pixelBufferCubeMapFace(&self) -> GLenum;
        #[method(pixelBufferMipMapLevel)]
        pub unsafe fn pixelBufferMipMapLevel(&self) -> GLint;
        #[method(setTextureImageToPixelBuffer:colorBuffer:)]
        pub unsafe fn setTextureImageToPixelBuffer_colorBuffer(
            &self,
            pixelBuffer: &NSOpenGLPixelBuffer,
            source: GLenum,
        );
    }
);
