#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSStepperTouchBarItem;
    unsafe impl ClassType for NSStepperTouchBarItem {
        type Super = NSTouchBarItem;
    }
);
extern_methods!(
    unsafe impl NSStepperTouchBarItem {
        #[method_id(stepperTouchBarItemWithIdentifier:formatter:)]
        pub unsafe fn stepperTouchBarItemWithIdentifier_formatter(
            identifier: &NSTouchBarItemIdentifier,
            formatter: &NSFormatter,
        ) -> Id<Self, Shared>;
        #[method_id(stepperTouchBarItemWithIdentifier:drawingHandler:)]
        pub unsafe fn stepperTouchBarItemWithIdentifier_drawingHandler(
            identifier: &NSTouchBarItemIdentifier,
            drawingHandler: TodoBlock,
        ) -> Id<Self, Shared>;
        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;
        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, maxValue: c_double);
        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;
        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, minValue: c_double);
        #[method(increment)]
        pub unsafe fn increment(&self) -> c_double;
        #[method(setIncrement:)]
        pub unsafe fn setIncrement(&self, increment: c_double);
        #[method(value)]
        pub unsafe fn value(&self) -> c_double;
        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: c_double);
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
