use super::__exported::NSClipView;
use super::__exported::NSColor;
use super::__exported::NSRulerView;
use super::__exported::NSScroller;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSScroller::*;
use crate::AppKit::generated::NSTextFinder::*;
use crate::AppKit::generated::NSView::*;
use crate::Foundation::generated::NSDate::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSScrollView;
    unsafe impl ClassType for NSScrollView {
        type Super = NSView;
    }
);
extern_methods!(
    unsafe impl NSScrollView {
        #[method_id(initWithFrame:)]
        pub unsafe fn initWithFrame(&self, frameRect: NSRect) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method(frameSizeForContentSize:horizontalScrollerClass:verticalScrollerClass:borderType:controlSize:scrollerStyle:)]
        pub unsafe fn frameSizeForContentSize_horizontalScrollerClass_verticalScrollerClass_borderType_controlSize_scrollerStyle(
            cSize: NSSize,
            horizontalScrollerClass: Option<&Class>,
            verticalScrollerClass: Option<&Class>,
            type_: NSBorderType,
            controlSize: NSControlSize,
            scrollerStyle: NSScrollerStyle,
        ) -> NSSize;
        #[method(contentSizeForFrameSize:horizontalScrollerClass:verticalScrollerClass:borderType:controlSize:scrollerStyle:)]
        pub unsafe fn contentSizeForFrameSize_horizontalScrollerClass_verticalScrollerClass_borderType_controlSize_scrollerStyle(
            fSize: NSSize,
            horizontalScrollerClass: Option<&Class>,
            verticalScrollerClass: Option<&Class>,
            type_: NSBorderType,
            controlSize: NSControlSize,
            scrollerStyle: NSScrollerStyle,
        ) -> NSSize;
        #[method(frameSizeForContentSize:hasHorizontalScroller:hasVerticalScroller:borderType:)]
        pub unsafe fn frameSizeForContentSize_hasHorizontalScroller_hasVerticalScroller_borderType(
            cSize: NSSize,
            hFlag: bool,
            vFlag: bool,
            type_: NSBorderType,
        ) -> NSSize;
        #[method(contentSizeForFrameSize:hasHorizontalScroller:hasVerticalScroller:borderType:)]
        pub unsafe fn contentSizeForFrameSize_hasHorizontalScroller_hasVerticalScroller_borderType(
            fSize: NSSize,
            hFlag: bool,
            vFlag: bool,
            type_: NSBorderType,
        ) -> NSSize;
        #[method(documentVisibleRect)]
        pub unsafe fn documentVisibleRect(&self) -> NSRect;
        #[method(contentSize)]
        pub unsafe fn contentSize(&self) -> NSSize;
        #[method_id(documentView)]
        pub unsafe fn documentView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setDocumentView:)]
        pub unsafe fn setDocumentView(&self, documentView: Option<&NSView>);
        #[method_id(contentView)]
        pub unsafe fn contentView(&self) -> Id<NSClipView, Shared>;
        #[method(setContentView:)]
        pub unsafe fn setContentView(&self, contentView: &NSClipView);
        #[method_id(documentCursor)]
        pub unsafe fn documentCursor(&self) -> Option<Id<NSCursor, Shared>>;
        #[method(setDocumentCursor:)]
        pub unsafe fn setDocumentCursor(&self, documentCursor: Option<&NSCursor>);
        #[method(borderType)]
        pub unsafe fn borderType(&self) -> NSBorderType;
        #[method(setBorderType:)]
        pub unsafe fn setBorderType(&self, borderType: NSBorderType);
        #[method_id(backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor, Shared>;
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: &NSColor);
        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;
        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, drawsBackground: bool);
        #[method(hasVerticalScroller)]
        pub unsafe fn hasVerticalScroller(&self) -> bool;
        #[method(setHasVerticalScroller:)]
        pub unsafe fn setHasVerticalScroller(&self, hasVerticalScroller: bool);
        #[method(hasHorizontalScroller)]
        pub unsafe fn hasHorizontalScroller(&self) -> bool;
        #[method(setHasHorizontalScroller:)]
        pub unsafe fn setHasHorizontalScroller(&self, hasHorizontalScroller: bool);
        #[method_id(verticalScroller)]
        pub unsafe fn verticalScroller(&self) -> Option<Id<NSScroller, Shared>>;
        #[method(setVerticalScroller:)]
        pub unsafe fn setVerticalScroller(&self, verticalScroller: Option<&NSScroller>);
        #[method_id(horizontalScroller)]
        pub unsafe fn horizontalScroller(&self) -> Option<Id<NSScroller, Shared>>;
        #[method(setHorizontalScroller:)]
        pub unsafe fn setHorizontalScroller(&self, horizontalScroller: Option<&NSScroller>);
        #[method(autohidesScrollers)]
        pub unsafe fn autohidesScrollers(&self) -> bool;
        #[method(setAutohidesScrollers:)]
        pub unsafe fn setAutohidesScrollers(&self, autohidesScrollers: bool);
        #[method(horizontalLineScroll)]
        pub unsafe fn horizontalLineScroll(&self) -> CGFloat;
        #[method(setHorizontalLineScroll:)]
        pub unsafe fn setHorizontalLineScroll(&self, horizontalLineScroll: CGFloat);
        #[method(verticalLineScroll)]
        pub unsafe fn verticalLineScroll(&self) -> CGFloat;
        #[method(setVerticalLineScroll:)]
        pub unsafe fn setVerticalLineScroll(&self, verticalLineScroll: CGFloat);
        #[method(lineScroll)]
        pub unsafe fn lineScroll(&self) -> CGFloat;
        #[method(setLineScroll:)]
        pub unsafe fn setLineScroll(&self, lineScroll: CGFloat);
        #[method(horizontalPageScroll)]
        pub unsafe fn horizontalPageScroll(&self) -> CGFloat;
        #[method(setHorizontalPageScroll:)]
        pub unsafe fn setHorizontalPageScroll(&self, horizontalPageScroll: CGFloat);
        #[method(verticalPageScroll)]
        pub unsafe fn verticalPageScroll(&self) -> CGFloat;
        #[method(setVerticalPageScroll:)]
        pub unsafe fn setVerticalPageScroll(&self, verticalPageScroll: CGFloat);
        #[method(pageScroll)]
        pub unsafe fn pageScroll(&self) -> CGFloat;
        #[method(setPageScroll:)]
        pub unsafe fn setPageScroll(&self, pageScroll: CGFloat);
        #[method(scrollsDynamically)]
        pub unsafe fn scrollsDynamically(&self) -> bool;
        #[method(setScrollsDynamically:)]
        pub unsafe fn setScrollsDynamically(&self, scrollsDynamically: bool);
        #[method(tile)]
        pub unsafe fn tile(&self);
        #[method(reflectScrolledClipView:)]
        pub unsafe fn reflectScrolledClipView(&self, cView: &NSClipView);
        #[method(scrollWheel:)]
        pub unsafe fn scrollWheel(&self, event: &NSEvent);
        #[method(scrollerStyle)]
        pub unsafe fn scrollerStyle(&self) -> NSScrollerStyle;
        #[method(setScrollerStyle:)]
        pub unsafe fn setScrollerStyle(&self, scrollerStyle: NSScrollerStyle);
        #[method(scrollerKnobStyle)]
        pub unsafe fn scrollerKnobStyle(&self) -> NSScrollerKnobStyle;
        #[method(setScrollerKnobStyle:)]
        pub unsafe fn setScrollerKnobStyle(&self, scrollerKnobStyle: NSScrollerKnobStyle);
        #[method(flashScrollers)]
        pub unsafe fn flashScrollers(&self);
        #[method(horizontalScrollElasticity)]
        pub unsafe fn horizontalScrollElasticity(&self) -> NSScrollElasticity;
        #[method(setHorizontalScrollElasticity:)]
        pub unsafe fn setHorizontalScrollElasticity(
            &self,
            horizontalScrollElasticity: NSScrollElasticity,
        );
        #[method(verticalScrollElasticity)]
        pub unsafe fn verticalScrollElasticity(&self) -> NSScrollElasticity;
        #[method(setVerticalScrollElasticity:)]
        pub unsafe fn setVerticalScrollElasticity(
            &self,
            verticalScrollElasticity: NSScrollElasticity,
        );
        #[method(usesPredominantAxisScrolling)]
        pub unsafe fn usesPredominantAxisScrolling(&self) -> bool;
        #[method(setUsesPredominantAxisScrolling:)]
        pub unsafe fn setUsesPredominantAxisScrolling(&self, usesPredominantAxisScrolling: bool);
        #[method(allowsMagnification)]
        pub unsafe fn allowsMagnification(&self) -> bool;
        #[method(setAllowsMagnification:)]
        pub unsafe fn setAllowsMagnification(&self, allowsMagnification: bool);
        #[method(magnification)]
        pub unsafe fn magnification(&self) -> CGFloat;
        #[method(setMagnification:)]
        pub unsafe fn setMagnification(&self, magnification: CGFloat);
        #[method(maxMagnification)]
        pub unsafe fn maxMagnification(&self) -> CGFloat;
        #[method(setMaxMagnification:)]
        pub unsafe fn setMaxMagnification(&self, maxMagnification: CGFloat);
        #[method(minMagnification)]
        pub unsafe fn minMagnification(&self) -> CGFloat;
        #[method(setMinMagnification:)]
        pub unsafe fn setMinMagnification(&self, minMagnification: CGFloat);
        #[method(magnifyToFitRect:)]
        pub unsafe fn magnifyToFitRect(&self, rect: NSRect);
        #[method(setMagnification:centeredAtPoint:)]
        pub unsafe fn setMagnification_centeredAtPoint(
            &self,
            magnification: CGFloat,
            point: NSPoint,
        );
        #[method(addFloatingSubview:forAxis:)]
        pub unsafe fn addFloatingSubview_forAxis(&self, view: &NSView, axis: NSEventGestureAxis);
        #[method(automaticallyAdjustsContentInsets)]
        pub unsafe fn automaticallyAdjustsContentInsets(&self) -> bool;
        #[method(setAutomaticallyAdjustsContentInsets:)]
        pub unsafe fn setAutomaticallyAdjustsContentInsets(
            &self,
            automaticallyAdjustsContentInsets: bool,
        );
        #[method(contentInsets)]
        pub unsafe fn contentInsets(&self) -> NSEdgeInsets;
        #[method(setContentInsets:)]
        pub unsafe fn setContentInsets(&self, contentInsets: NSEdgeInsets);
        #[method(scrollerInsets)]
        pub unsafe fn scrollerInsets(&self) -> NSEdgeInsets;
        #[method(setScrollerInsets:)]
        pub unsafe fn setScrollerInsets(&self, scrollerInsets: NSEdgeInsets);
    }
);
extern_methods!(
    #[doc = "NSRulerSupport"]
    unsafe impl NSScrollView {
        #[method(rulerViewClass)]
        pub unsafe fn rulerViewClass() -> Option<&Class>;
        #[method(setRulerViewClass:)]
        pub unsafe fn setRulerViewClass(rulerViewClass: Option<&Class>);
        #[method(rulersVisible)]
        pub unsafe fn rulersVisible(&self) -> bool;
        #[method(setRulersVisible:)]
        pub unsafe fn setRulersVisible(&self, rulersVisible: bool);
        #[method(hasHorizontalRuler)]
        pub unsafe fn hasHorizontalRuler(&self) -> bool;
        #[method(setHasHorizontalRuler:)]
        pub unsafe fn setHasHorizontalRuler(&self, hasHorizontalRuler: bool);
        #[method(hasVerticalRuler)]
        pub unsafe fn hasVerticalRuler(&self) -> bool;
        #[method(setHasVerticalRuler:)]
        pub unsafe fn setHasVerticalRuler(&self, hasVerticalRuler: bool);
        #[method_id(horizontalRulerView)]
        pub unsafe fn horizontalRulerView(&self) -> Option<Id<NSRulerView, Shared>>;
        #[method(setHorizontalRulerView:)]
        pub unsafe fn setHorizontalRulerView(&self, horizontalRulerView: Option<&NSRulerView>);
        #[method_id(verticalRulerView)]
        pub unsafe fn verticalRulerView(&self) -> Option<Id<NSRulerView, Shared>>;
        #[method(setVerticalRulerView:)]
        pub unsafe fn setVerticalRulerView(&self, verticalRulerView: Option<&NSRulerView>);
    }
);
extern_methods!(
    #[doc = "NSFindBarSupport"]
    unsafe impl NSScrollView {
        #[method(findBarPosition)]
        pub unsafe fn findBarPosition(&self) -> NSScrollViewFindBarPosition;
        #[method(setFindBarPosition:)]
        pub unsafe fn setFindBarPosition(&self, findBarPosition: NSScrollViewFindBarPosition);
    }
);
