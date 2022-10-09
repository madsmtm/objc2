use super::__exported::NSRulerMarker;
use super::__exported::NSScrollView;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSView::*;
use crate::Foundation::generated::NSArray::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSRulerViewUnitName = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSRulerView;
    unsafe impl ClassType for NSRulerView {
        type Super = NSView;
    }
);
extern_methods!(
    unsafe impl NSRulerView {
        #[method(registerUnitWithName:abbreviation:unitToPointsConversionFactor:stepUpCycle:stepDownCycle:)]
        pub unsafe fn registerUnitWithName_abbreviation_unitToPointsConversionFactor_stepUpCycle_stepDownCycle(
            unitName: &NSRulerViewUnitName,
            abbreviation: &NSString,
            conversionFactor: CGFloat,
            stepUpCycle: &NSArray<NSNumber>,
            stepDownCycle: &NSArray<NSNumber>,
        );
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method_id(initWithScrollView:orientation:)]
        pub unsafe fn initWithScrollView_orientation(
            &self,
            scrollView: Option<&NSScrollView>,
            orientation: NSRulerOrientation,
        ) -> Id<Self, Shared>;
        #[method_id(scrollView)]
        pub unsafe fn scrollView(&self) -> Option<Id<NSScrollView, Shared>>;
        #[method(setScrollView:)]
        pub unsafe fn setScrollView(&self, scrollView: Option<&NSScrollView>);
        #[method(orientation)]
        pub unsafe fn orientation(&self) -> NSRulerOrientation;
        #[method(setOrientation:)]
        pub unsafe fn setOrientation(&self, orientation: NSRulerOrientation);
        #[method(baselineLocation)]
        pub unsafe fn baselineLocation(&self) -> CGFloat;
        #[method(requiredThickness)]
        pub unsafe fn requiredThickness(&self) -> CGFloat;
        #[method(ruleThickness)]
        pub unsafe fn ruleThickness(&self) -> CGFloat;
        #[method(setRuleThickness:)]
        pub unsafe fn setRuleThickness(&self, ruleThickness: CGFloat);
        #[method(reservedThicknessForMarkers)]
        pub unsafe fn reservedThicknessForMarkers(&self) -> CGFloat;
        #[method(setReservedThicknessForMarkers:)]
        pub unsafe fn setReservedThicknessForMarkers(&self, reservedThicknessForMarkers: CGFloat);
        #[method(reservedThicknessForAccessoryView)]
        pub unsafe fn reservedThicknessForAccessoryView(&self) -> CGFloat;
        #[method(setReservedThicknessForAccessoryView:)]
        pub unsafe fn setReservedThicknessForAccessoryView(
            &self,
            reservedThicknessForAccessoryView: CGFloat,
        );
        #[method_id(measurementUnits)]
        pub unsafe fn measurementUnits(&self) -> Id<NSRulerViewUnitName, Shared>;
        #[method(setMeasurementUnits:)]
        pub unsafe fn setMeasurementUnits(&self, measurementUnits: &NSRulerViewUnitName);
        #[method(originOffset)]
        pub unsafe fn originOffset(&self) -> CGFloat;
        #[method(setOriginOffset:)]
        pub unsafe fn setOriginOffset(&self, originOffset: CGFloat);
        #[method_id(clientView)]
        pub unsafe fn clientView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setClientView:)]
        pub unsafe fn setClientView(&self, clientView: Option<&NSView>);
        #[method(addMarker:)]
        pub unsafe fn addMarker(&self, marker: &NSRulerMarker);
        #[method(removeMarker:)]
        pub unsafe fn removeMarker(&self, marker: &NSRulerMarker);
        #[method_id(markers)]
        pub unsafe fn markers(&self) -> Option<Id<NSArray<NSRulerMarker>, Shared>>;
        #[method(setMarkers:)]
        pub unsafe fn setMarkers(&self, markers: Option<&NSArray<NSRulerMarker>>);
        #[method(trackMarker:withMouseEvent:)]
        pub unsafe fn trackMarker_withMouseEvent(
            &self,
            marker: &NSRulerMarker,
            event: &NSEvent,
        ) -> bool;
        #[method_id(accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessoryView: Option<&NSView>);
        #[method(moveRulerlineFromLocation:toLocation:)]
        pub unsafe fn moveRulerlineFromLocation_toLocation(
            &self,
            oldLocation: CGFloat,
            newLocation: CGFloat,
        );
        #[method(invalidateHashMarks)]
        pub unsafe fn invalidateHashMarks(&self);
        #[method(drawHashMarksAndLabelsInRect:)]
        pub unsafe fn drawHashMarksAndLabelsInRect(&self, rect: NSRect);
        #[method(drawMarkersInRect:)]
        pub unsafe fn drawMarkersInRect(&self, rect: NSRect);
        #[method(isFlipped)]
        pub unsafe fn isFlipped(&self) -> bool;
    }
);
extern_methods!(
    #[doc = "NSRulerMarkerClientViewDelegation"]
    unsafe impl NSView {
        #[method(rulerView:shouldMoveMarker:)]
        pub unsafe fn rulerView_shouldMoveMarker(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
        ) -> bool;
        #[method(rulerView:willMoveMarker:toLocation:)]
        pub unsafe fn rulerView_willMoveMarker_toLocation(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
            location: CGFloat,
        ) -> CGFloat;
        #[method(rulerView:didMoveMarker:)]
        pub unsafe fn rulerView_didMoveMarker(&self, ruler: &NSRulerView, marker: &NSRulerMarker);
        #[method(rulerView:shouldRemoveMarker:)]
        pub unsafe fn rulerView_shouldRemoveMarker(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
        ) -> bool;
        #[method(rulerView:didRemoveMarker:)]
        pub unsafe fn rulerView_didRemoveMarker(&self, ruler: &NSRulerView, marker: &NSRulerMarker);
        #[method(rulerView:shouldAddMarker:)]
        pub unsafe fn rulerView_shouldAddMarker(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
        ) -> bool;
        #[method(rulerView:willAddMarker:atLocation:)]
        pub unsafe fn rulerView_willAddMarker_atLocation(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
            location: CGFloat,
        ) -> CGFloat;
        #[method(rulerView:didAddMarker:)]
        pub unsafe fn rulerView_didAddMarker(&self, ruler: &NSRulerView, marker: &NSRulerMarker);
        #[method(rulerView:handleMouseDown:)]
        pub unsafe fn rulerView_handleMouseDown(&self, ruler: &NSRulerView, event: &NSEvent);
        #[method(rulerView:willSetClientView:)]
        pub unsafe fn rulerView_willSetClientView(&self, ruler: &NSRulerView, newClient: &NSView);
        #[method(rulerView:locationForPoint:)]
        pub unsafe fn rulerView_locationForPoint(
            &self,
            ruler: &NSRulerView,
            point: NSPoint,
        ) -> CGFloat;
        #[method(rulerView:pointForLocation:)]
        pub unsafe fn rulerView_pointForLocation(
            &self,
            ruler: &NSRulerView,
            point: CGFloat,
        ) -> NSPoint;
    }
);
