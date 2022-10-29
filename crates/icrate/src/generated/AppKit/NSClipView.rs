#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSClipView;
    unsafe impl ClassType for NSClipView {
        type Super = NSView;
    }
);
extern_methods!(
    unsafe impl NSClipView {
        #[method_id(backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor, Shared>;
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: &NSColor);
        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;
        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, drawsBackground: bool);
        #[method_id(documentView)]
        pub unsafe fn documentView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setDocumentView:)]
        pub unsafe fn setDocumentView(&self, documentView: Option<&NSView>);
        #[method(documentRect)]
        pub unsafe fn documentRect(&self) -> NSRect;
        #[method_id(documentCursor)]
        pub unsafe fn documentCursor(&self) -> Option<Id<NSCursor, Shared>>;
        #[method(setDocumentCursor:)]
        pub unsafe fn setDocumentCursor(&self, documentCursor: Option<&NSCursor>);
        #[method(documentVisibleRect)]
        pub unsafe fn documentVisibleRect(&self) -> NSRect;
        #[method(viewFrameChanged:)]
        pub unsafe fn viewFrameChanged(&self, notification: &NSNotification);
        #[method(viewBoundsChanged:)]
        pub unsafe fn viewBoundsChanged(&self, notification: &NSNotification);
        #[method(autoscroll:)]
        pub unsafe fn autoscroll(&self, event: &NSEvent) -> bool;
        #[method(scrollToPoint:)]
        pub unsafe fn scrollToPoint(&self, newOrigin: NSPoint);
        #[method(constrainBoundsRect:)]
        pub unsafe fn constrainBoundsRect(&self, proposedBounds: NSRect) -> NSRect;
        #[method(contentInsets)]
        pub unsafe fn contentInsets(&self) -> NSEdgeInsets;
        #[method(setContentInsets:)]
        pub unsafe fn setContentInsets(&self, contentInsets: NSEdgeInsets);
        #[method(automaticallyAdjustsContentInsets)]
        pub unsafe fn automaticallyAdjustsContentInsets(&self) -> bool;
        #[method(setAutomaticallyAdjustsContentInsets:)]
        pub unsafe fn setAutomaticallyAdjustsContentInsets(
            &self,
            automaticallyAdjustsContentInsets: bool,
        );
    }
);
extern_methods!(
    #[doc = "NSClipViewSuperview"]
    unsafe impl NSView {
        #[method(reflectScrolledClipView:)]
        pub unsafe fn reflectScrolledClipView(&self, clipView: &NSClipView);
        #[method(scrollClipView:toPoint:)]
        pub unsafe fn scrollClipView_toPoint(&self, clipView: &NSClipView, point: NSPoint);
    }
);
extern_methods!(
    unsafe impl NSClipView {
        #[method(constrainScrollPoint:)]
        pub unsafe fn constrainScrollPoint(&self, newOrigin: NSPoint) -> NSPoint;
        #[method(copiesOnScroll)]
        pub unsafe fn copiesOnScroll(&self) -> bool;
        #[method(setCopiesOnScroll:)]
        pub unsafe fn setCopiesOnScroll(&self, copiesOnScroll: bool);
    }
);
