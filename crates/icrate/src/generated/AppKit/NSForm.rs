#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSForm;
    unsafe impl ClassType for NSForm {
        type Super = NSMatrix;
    }
);
extern_methods!(
    unsafe impl NSForm {
        #[method(indexOfSelectedItem)]
        pub unsafe fn indexOfSelectedItem(&self) -> NSInteger;
        #[method(setEntryWidth:)]
        pub unsafe fn setEntryWidth(&self, width: CGFloat);
        #[method(setInterlineSpacing:)]
        pub unsafe fn setInterlineSpacing(&self, spacing: CGFloat);
        #[method(setBordered:)]
        pub unsafe fn setBordered(&self, flag: bool);
        #[method(setBezeled:)]
        pub unsafe fn setBezeled(&self, flag: bool);
        #[method(setTitleAlignment:)]
        pub unsafe fn setTitleAlignment(&self, mode: NSTextAlignment);
        #[method(setTextAlignment:)]
        pub unsafe fn setTextAlignment(&self, mode: NSTextAlignment);
        #[method(setTitleFont:)]
        pub unsafe fn setTitleFont(&self, fontObj: &NSFont);
        #[method(setTextFont:)]
        pub unsafe fn setTextFont(&self, fontObj: &NSFont);
        #[method_id(cellAtIndex:)]
        pub unsafe fn cellAtIndex(&self, index: NSInteger) -> Option<Id<Object, Shared>>;
        #[method(drawCellAtIndex:)]
        pub unsafe fn drawCellAtIndex(&self, index: NSInteger);
        #[method_id(addEntry:)]
        pub unsafe fn addEntry(&self, title: &NSString) -> Id<NSFormCell, Shared>;
        #[method_id(insertEntry:atIndex:)]
        pub unsafe fn insertEntry_atIndex(
            &self,
            title: &NSString,
            index: NSInteger,
        ) -> Option<Id<NSFormCell, Shared>>;
        #[method(removeEntryAtIndex:)]
        pub unsafe fn removeEntryAtIndex(&self, index: NSInteger);
        #[method(indexOfCellWithTag:)]
        pub unsafe fn indexOfCellWithTag(&self, tag: NSInteger) -> NSInteger;
        #[method(selectTextAtIndex:)]
        pub unsafe fn selectTextAtIndex(&self, index: NSInteger);
        #[method(setFrameSize:)]
        pub unsafe fn setFrameSize(&self, newSize: NSSize);
        #[method(setTitleBaseWritingDirection:)]
        pub unsafe fn setTitleBaseWritingDirection(&self, writingDirection: NSWritingDirection);
        #[method(setTextBaseWritingDirection:)]
        pub unsafe fn setTextBaseWritingDirection(&self, writingDirection: NSWritingDirection);
        #[method(setPreferredTextFieldWidth:)]
        pub unsafe fn setPreferredTextFieldWidth(&self, preferredWidth: CGFloat);
        #[method(preferredTextFieldWidth)]
        pub unsafe fn preferredTextFieldWidth(&self) -> CGFloat;
    }
);
