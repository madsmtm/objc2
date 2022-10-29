#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSSliderAccessoryWidth = CGFloat;
extern_class!(
    #[derive(Debug)]
    pub struct NSSliderTouchBarItem;
    unsafe impl ClassType for NSSliderTouchBarItem {
        type Super = NSTouchBarItem;
    }
);
extern_methods!(
    unsafe impl NSSliderTouchBarItem {
        #[method_id(view)]
        pub unsafe fn view(&self) -> Id<TodoProtocols, Shared>;
        #[method_id(slider)]
        pub unsafe fn slider(&self) -> Id<NSSlider, Shared>;
        #[method(setSlider:)]
        pub unsafe fn setSlider(&self, slider: &NSSlider);
        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;
        #[method(setDoubleValue:)]
        pub unsafe fn setDoubleValue(&self, doubleValue: c_double);
        #[method(minimumSliderWidth)]
        pub unsafe fn minimumSliderWidth(&self) -> CGFloat;
        #[method(setMinimumSliderWidth:)]
        pub unsafe fn setMinimumSliderWidth(&self, minimumSliderWidth: CGFloat);
        #[method(maximumSliderWidth)]
        pub unsafe fn maximumSliderWidth(&self) -> CGFloat;
        #[method(setMaximumSliderWidth:)]
        pub unsafe fn setMaximumSliderWidth(&self, maximumSliderWidth: CGFloat);
        #[method_id(label)]
        pub unsafe fn label(&self) -> Option<Id<NSString, Shared>>;
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: Option<&NSString>);
        #[method_id(minimumValueAccessory)]
        pub unsafe fn minimumValueAccessory(&self) -> Option<Id<NSSliderAccessory, Shared>>;
        #[method(setMinimumValueAccessory:)]
        pub unsafe fn setMinimumValueAccessory(
            &self,
            minimumValueAccessory: Option<&NSSliderAccessory>,
        );
        #[method_id(maximumValueAccessory)]
        pub unsafe fn maximumValueAccessory(&self) -> Option<Id<NSSliderAccessory, Shared>>;
        #[method(setMaximumValueAccessory:)]
        pub unsafe fn setMaximumValueAccessory(
            &self,
            maximumValueAccessory: Option<&NSSliderAccessory>,
        );
        #[method(valueAccessoryWidth)]
        pub unsafe fn valueAccessoryWidth(&self) -> NSSliderAccessoryWidth;
        #[method(setValueAccessoryWidth:)]
        pub unsafe fn setValueAccessoryWidth(&self, valueAccessoryWidth: NSSliderAccessoryWidth);
        #[method_id(target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);
        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;
        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);
        #[method_id(customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString, Shared>;
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customizationLabel: Option<&NSString>);
    }
);