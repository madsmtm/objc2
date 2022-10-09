use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSCell::*;
use crate::AppKit::generated::NSImage::*;
use crate::AppKit::generated::NSView::*;
use crate::AppKit::generated::NSWindow::*;
use crate::Foundation::generated::NSObjCRuntime::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSVisualEffectView;
    unsafe impl ClassType for NSVisualEffectView {
        type Super = NSView;
    }
);
extern_methods!(
    unsafe impl NSVisualEffectView {
        #[method(material)]
        pub unsafe fn material(&self) -> NSVisualEffectMaterial;
        #[method(setMaterial:)]
        pub unsafe fn setMaterial(&self, material: NSVisualEffectMaterial);
        #[method(interiorBackgroundStyle)]
        pub unsafe fn interiorBackgroundStyle(&self) -> NSBackgroundStyle;
        #[method(blendingMode)]
        pub unsafe fn blendingMode(&self) -> NSVisualEffectBlendingMode;
        #[method(setBlendingMode:)]
        pub unsafe fn setBlendingMode(&self, blendingMode: NSVisualEffectBlendingMode);
        #[method(state)]
        pub unsafe fn state(&self) -> NSVisualEffectState;
        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSVisualEffectState);
        #[method_id(maskImage)]
        pub unsafe fn maskImage(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setMaskImage:)]
        pub unsafe fn setMaskImage(&self, maskImage: Option<&NSImage>);
        #[method(isEmphasized)]
        pub unsafe fn isEmphasized(&self) -> bool;
        #[method(setEmphasized:)]
        pub unsafe fn setEmphasized(&self, emphasized: bool);
        #[method(viewDidMoveToWindow)]
        pub unsafe fn viewDidMoveToWindow(&self);
        #[method(viewWillMoveToWindow:)]
        pub unsafe fn viewWillMoveToWindow(&self, newWindow: Option<&NSWindow>);
    }
);
