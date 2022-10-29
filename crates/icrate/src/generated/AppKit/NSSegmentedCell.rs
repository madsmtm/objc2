#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSegmentedCell;
    unsafe impl ClassType for NSSegmentedCell {
        type Super = NSActionCell;
    }
);
extern_methods!(
    unsafe impl NSSegmentedCell {
        #[method(segmentCount)]
        pub unsafe fn segmentCount(&self) -> NSInteger;
        #[method(setSegmentCount:)]
        pub unsafe fn setSegmentCount(&self, segmentCount: NSInteger);
        #[method(selectedSegment)]
        pub unsafe fn selectedSegment(&self) -> NSInteger;
        #[method(setSelectedSegment:)]
        pub unsafe fn setSelectedSegment(&self, selectedSegment: NSInteger);
        #[method(selectSegmentWithTag:)]
        pub unsafe fn selectSegmentWithTag(&self, tag: NSInteger) -> bool;
        #[method(makeNextSegmentKey)]
        pub unsafe fn makeNextSegmentKey(&self);
        #[method(makePreviousSegmentKey)]
        pub unsafe fn makePreviousSegmentKey(&self);
        #[method(trackingMode)]
        pub unsafe fn trackingMode(&self) -> NSSegmentSwitchTracking;
        #[method(setTrackingMode:)]
        pub unsafe fn setTrackingMode(&self, trackingMode: NSSegmentSwitchTracking);
        #[method(setWidth:forSegment:)]
        pub unsafe fn setWidth_forSegment(&self, width: CGFloat, segment: NSInteger);
        #[method(widthForSegment:)]
        pub unsafe fn widthForSegment(&self, segment: NSInteger) -> CGFloat;
        #[method(setImage:forSegment:)]
        pub unsafe fn setImage_forSegment(&self, image: Option<&NSImage>, segment: NSInteger);
        #[method_id(imageForSegment:)]
        pub unsafe fn imageForSegment(&self, segment: NSInteger) -> Option<Id<NSImage, Shared>>;
        #[method(setImageScaling:forSegment:)]
        pub unsafe fn setImageScaling_forSegment(
            &self,
            scaling: NSImageScaling,
            segment: NSInteger,
        );
        #[method(imageScalingForSegment:)]
        pub unsafe fn imageScalingForSegment(&self, segment: NSInteger) -> NSImageScaling;
        #[method(setLabel:forSegment:)]
        pub unsafe fn setLabel_forSegment(&self, label: &NSString, segment: NSInteger);
        #[method_id(labelForSegment:)]
        pub unsafe fn labelForSegment(&self, segment: NSInteger) -> Option<Id<NSString, Shared>>;
        #[method(setSelected:forSegment:)]
        pub unsafe fn setSelected_forSegment(&self, selected: bool, segment: NSInteger);
        #[method(isSelectedForSegment:)]
        pub unsafe fn isSelectedForSegment(&self, segment: NSInteger) -> bool;
        #[method(setEnabled:forSegment:)]
        pub unsafe fn setEnabled_forSegment(&self, enabled: bool, segment: NSInteger);
        #[method(isEnabledForSegment:)]
        pub unsafe fn isEnabledForSegment(&self, segment: NSInteger) -> bool;
        #[method(setMenu:forSegment:)]
        pub unsafe fn setMenu_forSegment(&self, menu: Option<&NSMenu>, segment: NSInteger);
        #[method_id(menuForSegment:)]
        pub unsafe fn menuForSegment(&self, segment: NSInteger) -> Option<Id<NSMenu, Shared>>;
        #[method(setToolTip:forSegment:)]
        pub unsafe fn setToolTip_forSegment(&self, toolTip: Option<&NSString>, segment: NSInteger);
        #[method_id(toolTipForSegment:)]
        pub unsafe fn toolTipForSegment(&self, segment: NSInteger) -> Option<Id<NSString, Shared>>;
        #[method(setTag:forSegment:)]
        pub unsafe fn setTag_forSegment(&self, tag: NSInteger, segment: NSInteger);
        #[method(tagForSegment:)]
        pub unsafe fn tagForSegment(&self, segment: NSInteger) -> NSInteger;
        #[method(segmentStyle)]
        pub unsafe fn segmentStyle(&self) -> NSSegmentStyle;
        #[method(setSegmentStyle:)]
        pub unsafe fn setSegmentStyle(&self, segmentStyle: NSSegmentStyle);
        #[method(drawSegment:inFrame:withView:)]
        pub unsafe fn drawSegment_inFrame_withView(
            &self,
            segment: NSInteger,
            frame: NSRect,
            controlView: &NSView,
        );
    }
);
extern_methods!(
    #[doc = "NSSegmentBackgroundStyle"]
    unsafe impl NSSegmentedCell {
        #[method(interiorBackgroundStyleForSegment:)]
        pub unsafe fn interiorBackgroundStyleForSegment(
            &self,
            segment: NSInteger,
        ) -> NSBackgroundStyle;
    }
);
