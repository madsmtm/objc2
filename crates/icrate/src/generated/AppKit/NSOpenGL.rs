//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSOpenGLGlobalOption = u32;
pub const NSOpenGLGOFormatCacheSize: NSOpenGLGlobalOption = 501;
pub const NSOpenGLGOClearFormatCache: NSOpenGLGlobalOption = 502;
pub const NSOpenGLGORetainRenderers: NSOpenGLGlobalOption = 503;
pub const NSOpenGLGOUseBuildCache: NSOpenGLGlobalOption = 506;
pub const NSOpenGLGOResetLibrary: NSOpenGLGlobalOption = 504;

pub const NSOpenGLPFAAllRenderers: c_uint = 1;
pub const NSOpenGLPFATripleBuffer: c_uint = 3;
pub const NSOpenGLPFADoubleBuffer: c_uint = 5;
pub const NSOpenGLPFAAuxBuffers: c_uint = 7;
pub const NSOpenGLPFAColorSize: c_uint = 8;
pub const NSOpenGLPFAAlphaSize: c_uint = 11;
pub const NSOpenGLPFADepthSize: c_uint = 12;
pub const NSOpenGLPFAStencilSize: c_uint = 13;
pub const NSOpenGLPFAAccumSize: c_uint = 14;
pub const NSOpenGLPFAMinimumPolicy: c_uint = 51;
pub const NSOpenGLPFAMaximumPolicy: c_uint = 52;
pub const NSOpenGLPFASampleBuffers: c_uint = 55;
pub const NSOpenGLPFASamples: c_uint = 56;
pub const NSOpenGLPFAAuxDepthStencil: c_uint = 57;
pub const NSOpenGLPFAColorFloat: c_uint = 58;
pub const NSOpenGLPFAMultisample: c_uint = 59;
pub const NSOpenGLPFASupersample: c_uint = 60;
pub const NSOpenGLPFASampleAlpha: c_uint = 61;
pub const NSOpenGLPFARendererID: c_uint = 70;
pub const NSOpenGLPFANoRecovery: c_uint = 72;
pub const NSOpenGLPFAAccelerated: c_uint = 73;
pub const NSOpenGLPFAClosestPolicy: c_uint = 74;
pub const NSOpenGLPFABackingStore: c_uint = 76;
pub const NSOpenGLPFAScreenMask: c_uint = 84;
pub const NSOpenGLPFAAllowOfflineRenderers: c_uint = 96;
pub const NSOpenGLPFAAcceleratedCompute: c_uint = 97;
pub const NSOpenGLPFAOpenGLProfile: c_uint = 99;
pub const NSOpenGLPFAVirtualScreenCount: c_uint = 128;
pub const NSOpenGLPFAStereo: c_uint = 6;
pub const NSOpenGLPFAOffScreen: c_uint = 53;
pub const NSOpenGLPFAFullScreen: c_uint = 54;
pub const NSOpenGLPFASingleRenderer: c_uint = 71;
pub const NSOpenGLPFARobust: c_uint = 75;
pub const NSOpenGLPFAMPSafe: c_uint = 78;
pub const NSOpenGLPFAWindow: c_uint = 80;
pub const NSOpenGLPFAMultiScreen: c_uint = 81;
pub const NSOpenGLPFACompliant: c_uint = 83;
pub const NSOpenGLPFAPixelBuffer: c_uint = 90;
pub const NSOpenGLPFARemotePixelBuffer: c_uint = 91;

pub type NSOpenGLPixelFormatAttribute = u32;

pub const NSOpenGLProfileVersionLegacy: c_uint = 0x1000;
pub const NSOpenGLProfileVersion3_2Core: c_uint = 0x3200;
pub const NSOpenGLProfileVersion4_1Core: c_uint = 0x4100;

extern_class!(
    #[derive(Debug)]
    pub struct NSOpenGLPixelFormat;

    unsafe impl ClassType for NSOpenGLPixelFormat {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSOpenGLPixelFormat {
        #[method_id(@__retain_semantics Init initWithCGLPixelFormatObj:)]
        pub unsafe fn initWithCGLPixelFormatObj(
            this: Option<Allocated<Self>>,
            format: CGLPixelFormatObj,
        ) -> Option<Id<NSOpenGLPixelFormat, Shared>>;

        #[method_id(@__retain_semantics Init initWithAttributes:)]
        pub unsafe fn initWithAttributes(
            this: Option<Allocated<Self>>,
            attribs: NonNull<NSOpenGLPixelFormatAttribute>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(
            this: Option<Allocated<Self>>,
            attribs: Option<&NSData>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other attributes)]
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
        #[method_id(@__retain_semantics Init initWithTextureTarget:textureInternalFormat:textureMaxMipMapLevel:pixelsWide:pixelsHigh:)]
        pub unsafe fn initWithTextureTarget_textureInternalFormat_textureMaxMipMapLevel_pixelsWide_pixelsHigh(
            this: Option<Allocated<Self>>,
            target: GLenum,
            format: GLenum,
            maxLevel: GLint,
            pixelsWide: GLsizei,
            pixelsHigh: GLsizei,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithCGLPBufferObj:)]
        pub unsafe fn initWithCGLPBufferObj(
            this: Option<Allocated<Self>>,
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

pub type NSOpenGLContextParameter = NSInteger;
pub const NSOpenGLContextParameterSwapInterval: NSOpenGLContextParameter = 222;
pub const NSOpenGLContextParameterSurfaceOrder: NSOpenGLContextParameter = 235;
pub const NSOpenGLContextParameterSurfaceOpacity: NSOpenGLContextParameter = 236;
pub const NSOpenGLContextParameterSurfaceBackingSize: NSOpenGLContextParameter = 304;
pub const NSOpenGLContextParameterReclaimResources: NSOpenGLContextParameter = 308;
pub const NSOpenGLContextParameterCurrentRendererID: NSOpenGLContextParameter = 309;
pub const NSOpenGLContextParameterGPUVertexProcessing: NSOpenGLContextParameter = 310;
pub const NSOpenGLContextParameterGPUFragmentProcessing: NSOpenGLContextParameter = 311;
pub const NSOpenGLContextParameterHasDrawable: NSOpenGLContextParameter = 314;
pub const NSOpenGLContextParameterMPSwapsInFlight: NSOpenGLContextParameter = 315;
pub const NSOpenGLContextParameterSwapRectangle: NSOpenGLContextParameter = 200;
pub const NSOpenGLContextParameterSwapRectangleEnable: NSOpenGLContextParameter = 201;
pub const NSOpenGLContextParameterRasterizationEnable: NSOpenGLContextParameter = 221;
pub const NSOpenGLContextParameterStateValidation: NSOpenGLContextParameter = 301;
pub const NSOpenGLContextParameterSurfaceSurfaceVolatile: NSOpenGLContextParameter = 306;

extern_class!(
    #[derive(Debug)]
    pub struct NSOpenGLContext;

    unsafe impl ClassType for NSOpenGLContext {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSOpenGLContext {
        #[method_id(@__retain_semantics Init initWithFormat:shareContext:)]
        pub unsafe fn initWithFormat_shareContext(
            this: Option<Allocated<Self>>,
            format: &NSOpenGLPixelFormat,
            share: Option<&NSOpenGLContext>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithCGLContextObj:)]
        pub unsafe fn initWithCGLContextObj(
            this: Option<Allocated<Self>>,
            context: CGLContextObj,
        ) -> Option<Id<NSOpenGLContext, Shared>>;

        #[method_id(@__retain_semantics Other pixelFormat)]
        pub unsafe fn pixelFormat(&self) -> Id<NSOpenGLPixelFormat, Shared>;

        #[method_id(@__retain_semantics Other view)]
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

        #[method_id(@__retain_semantics Other currentContext)]
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
    /// NSOpenGLPixelBuffer
    unsafe impl NSOpenGLContext {
        #[method(setPixelBuffer:cubeMapFace:mipMapLevel:currentVirtualScreen:)]
        pub unsafe fn setPixelBuffer_cubeMapFace_mipMapLevel_currentVirtualScreen(
            &self,
            pixelBuffer: &NSOpenGLPixelBuffer,
            face: GLenum,
            level: GLint,
            screen: GLint,
        );

        #[method_id(@__retain_semantics Other pixelBuffer)]
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

pub static NSOpenGLCPSwapInterval: NSOpenGLContextParameter = NSOpenGLContextParameterSwapInterval;

pub static NSOpenGLCPSurfaceOrder: NSOpenGLContextParameter = NSOpenGLContextParameterSurfaceOrder;

pub static NSOpenGLCPSurfaceOpacity: NSOpenGLContextParameter =
    NSOpenGLContextParameterSurfaceOpacity;

pub static NSOpenGLCPSurfaceBackingSize: NSOpenGLContextParameter =
    NSOpenGLContextParameterSurfaceBackingSize;

pub static NSOpenGLCPReclaimResources: NSOpenGLContextParameter =
    NSOpenGLContextParameterReclaimResources;

pub static NSOpenGLCPCurrentRendererID: NSOpenGLContextParameter =
    NSOpenGLContextParameterCurrentRendererID;

pub static NSOpenGLCPGPUVertexProcessing: NSOpenGLContextParameter =
    NSOpenGLContextParameterGPUVertexProcessing;

pub static NSOpenGLCPGPUFragmentProcessing: NSOpenGLContextParameter =
    NSOpenGLContextParameterGPUFragmentProcessing;

pub static NSOpenGLCPHasDrawable: NSOpenGLContextParameter = NSOpenGLContextParameterHasDrawable;

pub static NSOpenGLCPMPSwapsInFlight: NSOpenGLContextParameter =
    NSOpenGLContextParameterMPSwapsInFlight;

pub static NSOpenGLCPSwapRectangle: NSOpenGLContextParameter =
    NSOpenGLContextParameterSwapRectangle;

pub static NSOpenGLCPSwapRectangleEnable: NSOpenGLContextParameter =
    NSOpenGLContextParameterSwapRectangleEnable;

pub static NSOpenGLCPRasterizationEnable: NSOpenGLContextParameter =
    NSOpenGLContextParameterRasterizationEnable;

pub static NSOpenGLCPStateValidation: NSOpenGLContextParameter =
    NSOpenGLContextParameterStateValidation;

pub static NSOpenGLCPSurfaceSurfaceVolatile: NSOpenGLContextParameter =
    NSOpenGLContextParameterSurfaceSurfaceVolatile;
