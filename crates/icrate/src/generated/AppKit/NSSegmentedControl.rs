use super::__exported::NSImage;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSCell::*;
use crate::AppKit::generated::NSControl::*;
use crate::AppKit::generated::NSUserInterfaceCompression::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSegmentedControl;
    unsafe impl ClassType for NSSegmentedControl {
        type Super = NSControl;
    }
);
extern_methods!(
    unsafe impl NSSegmentedControl {
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
        #[method(setMenu:forSegment:)]
        pub unsafe fn setMenu_forSegment(&self, menu: Option<&NSMenu>, segment: NSInteger);
        #[method_id(menuForSegment:)]
        pub unsafe fn menuForSegment(&self, segment: NSInteger) -> Option<Id<NSMenu, Shared>>;
        #[method(setSelected:forSegment:)]
        pub unsafe fn setSelected_forSegment(&self, selected: bool, segment: NSInteger);
        #[method(isSelectedForSegment:)]
        pub unsafe fn isSelectedForSegment(&self, segment: NSInteger) -> bool;
        #[method(setEnabled:forSegment:)]
        pub unsafe fn setEnabled_forSegment(&self, enabled: bool, segment: NSInteger);
        #[method(isEnabledForSegment:)]
        pub unsafe fn isEnabledForSegment(&self, segment: NSInteger) -> bool;
        #[method(setToolTip:forSegment:)]
        pub unsafe fn setToolTip_forSegment(&self, toolTip: Option<&NSString>, segment: NSInteger);
        #[method_id(toolTipForSegment:)]
        pub unsafe fn toolTipForSegment(&self, segment: NSInteger) -> Option<Id<NSString, Shared>>;
        #[method(setTag:forSegment:)]
        pub unsafe fn setTag_forSegment(&self, tag: NSInteger, segment: NSInteger);
        #[method(tagForSegment:)]
        pub unsafe fn tagForSegment(&self, segment: NSInteger) -> NSInteger;
        #[method(setShowsMenuIndicator:forSegment:)]
        pub unsafe fn setShowsMenuIndicator_forSegment(
            &self,
            showsMenuIndicator: bool,
            segment: NSInteger,
        );
        #[method(showsMenuIndicatorForSegment:)]
        pub unsafe fn showsMenuIndicatorForSegment(&self, segment: NSInteger) -> bool;
        #[method(segmentStyle)]
        pub unsafe fn segmentStyle(&self) -> NSSegmentStyle;
        #[method(setSegmentStyle:)]
        pub unsafe fn setSegmentStyle(&self, segmentStyle: NSSegmentStyle);
        #[method(isSpringLoaded)]
        pub unsafe fn isSpringLoaded(&self) -> bool;
        #[method(setSpringLoaded:)]
        pub unsafe fn setSpringLoaded(&self, springLoaded: bool);
        #[method(trackingMode)]
        pub unsafe fn trackingMode(&self) -> NSSegmentSwitchTracking;
        #[method(setTrackingMode:)]
        pub unsafe fn setTrackingMode(&self, trackingMode: NSSegmentSwitchTracking);
        #[method(doubleValueForSelectedSegment)]
        pub unsafe fn doubleValueForSelectedSegment(&self) -> c_double;
        #[method_id(selectedSegmentBezelColor)]
        pub unsafe fn selectedSegmentBezelColor(&self) -> Option<Id<NSColor, Shared>>;
        #[method(setSelectedSegmentBezelColor:)]
        pub unsafe fn setSelectedSegmentBezelColor(
            &self,
            selectedSegmentBezelColor: Option<&NSColor>,
        );
        #[method(indexOfSelectedItem)]
        pub unsafe fn indexOfSelectedItem(&self) -> NSInteger;
        #[method(setAlignment:forSegment:)]
        pub unsafe fn setAlignment_forSegment(
            &self,
            alignment: NSTextAlignment,
            segment: NSInteger,
        );
        #[method(alignmentForSegment:)]
        pub unsafe fn alignmentForSegment(&self, segment: NSInteger) -> NSTextAlignment;
        #[method(segmentDistribution)]
        pub unsafe fn segmentDistribution(&self) -> NSSegmentDistribution;
        #[method(setSegmentDistribution:)]
        pub unsafe fn setSegmentDistribution(&self, segmentDistribution: NSSegmentDistribution);
        #[method(compressWithPrioritizedCompressionOptions:)]
        pub unsafe fn compressWithPrioritizedCompressionOptions(
            &self,
            prioritizedOptions: &NSArray<NSUserInterfaceCompressionOptions>,
        );
        #[method(minimumSizeWithPrioritizedCompressionOptions:)]
        pub unsafe fn minimumSizeWithPrioritizedCompressionOptions(
            &self,
            prioritizedOptions: &NSArray<NSUserInterfaceCompressionOptions>,
        ) -> NSSize;
        #[method_id(activeCompressionOptions)]
        pub unsafe fn activeCompressionOptions(
            &self,
        ) -> Id<NSUserInterfaceCompressionOptions, Shared>;
    }
);
extern_methods!(
    #[doc = "NSSegmentedControlConvenience"]
    unsafe impl NSSegmentedControl {
        #[method_id(segmentedControlWithLabels:trackingMode:target:action:)]
        pub unsafe fn segmentedControlWithLabels_trackingMode_target_action(
            labels: &NSArray<NSString>,
            trackingMode: NSSegmentSwitchTracking,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
        #[method_id(segmentedControlWithImages:trackingMode:target:action:)]
        pub unsafe fn segmentedControlWithImages_trackingMode_target_action(
            images: &NSArray<NSImage>,
            trackingMode: NSSegmentSwitchTracking,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
    }
);
