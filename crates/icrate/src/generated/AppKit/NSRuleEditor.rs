//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSRuleEditorPredicatePartKey = NSString;

extern "C" {
    static NSRuleEditorPredicateLeftExpression: &'static NSRuleEditorPredicatePartKey;
}

extern "C" {
    static NSRuleEditorPredicateRightExpression: &'static NSRuleEditorPredicatePartKey;
}

extern "C" {
    static NSRuleEditorPredicateComparisonModifier: &'static NSRuleEditorPredicatePartKey;
}

extern "C" {
    static NSRuleEditorPredicateOptions: &'static NSRuleEditorPredicatePartKey;
}

extern "C" {
    static NSRuleEditorPredicateOperatorType: &'static NSRuleEditorPredicatePartKey;
}

extern "C" {
    static NSRuleEditorPredicateCustomSelector: &'static NSRuleEditorPredicatePartKey;
}

extern "C" {
    static NSRuleEditorPredicateCompoundType: &'static NSRuleEditorPredicatePartKey;
}

pub type NSRuleEditorNestingMode = NSUInteger;
pub const NSRuleEditorNestingModeSingle: NSRuleEditorNestingMode = 0;
pub const NSRuleEditorNestingModeList: NSRuleEditorNestingMode = 1;
pub const NSRuleEditorNestingModeCompound: NSRuleEditorNestingMode = 2;
pub const NSRuleEditorNestingModeSimple: NSRuleEditorNestingMode = 3;

pub type NSRuleEditorRowType = NSUInteger;
pub const NSRuleEditorRowTypeSimple: NSRuleEditorRowType = 0;
pub const NSRuleEditorRowTypeCompound: NSRuleEditorRowType = 1;

extern_class!(
    #[derive(Debug)]
    pub struct NSRuleEditor;

    unsafe impl ClassType for NSRuleEditor {
        type Super = NSControl;
    }
);

extern_methods!(
    unsafe impl NSRuleEditor {
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSRuleEditorDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSRuleEditorDelegate>);

        #[method_id(formattingStringsFilename)]
        pub unsafe fn formattingStringsFilename(&self) -> Option<Id<NSString, Shared>>;

        #[method(setFormattingStringsFilename:)]
        pub unsafe fn setFormattingStringsFilename(
            &self,
            formattingStringsFilename: Option<&NSString>,
        );

        #[method_id(formattingDictionary)]
        pub unsafe fn formattingDictionary(
            &self,
        ) -> Option<Id<NSDictionary<NSString, NSString>, Shared>>;

        #[method(setFormattingDictionary:)]
        pub unsafe fn setFormattingDictionary(
            &self,
            formattingDictionary: Option<&NSDictionary<NSString, NSString>>,
        );

        #[method(reloadCriteria)]
        pub unsafe fn reloadCriteria(&self);

        #[method(nestingMode)]
        pub unsafe fn nestingMode(&self) -> NSRuleEditorNestingMode;

        #[method(setNestingMode:)]
        pub unsafe fn setNestingMode(&self, nestingMode: NSRuleEditorNestingMode);

        #[method(rowHeight)]
        pub unsafe fn rowHeight(&self) -> CGFloat;

        #[method(setRowHeight:)]
        pub unsafe fn setRowHeight(&self, rowHeight: CGFloat);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method(canRemoveAllRows)]
        pub unsafe fn canRemoveAllRows(&self) -> bool;

        #[method(setCanRemoveAllRows:)]
        pub unsafe fn setCanRemoveAllRows(&self, canRemoveAllRows: bool);

        #[method_id(predicate)]
        pub unsafe fn predicate(&self) -> Option<Id<NSPredicate, Shared>>;

        #[method(reloadPredicate)]
        pub unsafe fn reloadPredicate(&self);

        #[method_id(predicateForRow:)]
        pub unsafe fn predicateForRow(&self, row: NSInteger) -> Option<Id<NSPredicate, Shared>>;

        #[method(numberOfRows)]
        pub unsafe fn numberOfRows(&self) -> NSInteger;

        #[method_id(subrowIndexesForRow:)]
        pub unsafe fn subrowIndexesForRow(&self, rowIndex: NSInteger) -> Id<NSIndexSet, Shared>;

        #[method_id(criteriaForRow:)]
        pub unsafe fn criteriaForRow(&self, row: NSInteger) -> Id<NSArray, Shared>;

        #[method_id(displayValuesForRow:)]
        pub unsafe fn displayValuesForRow(&self, row: NSInteger) -> Id<NSArray, Shared>;

        #[method(rowForDisplayValue:)]
        pub unsafe fn rowForDisplayValue(&self, displayValue: &Object) -> NSInteger;

        #[method(rowTypeForRow:)]
        pub unsafe fn rowTypeForRow(&self, rowIndex: NSInteger) -> NSRuleEditorRowType;

        #[method(parentRowForRow:)]
        pub unsafe fn parentRowForRow(&self, rowIndex: NSInteger) -> NSInteger;

        #[method(addRow:)]
        pub unsafe fn addRow(&self, sender: Option<&Object>);

        #[method(insertRowAtIndex:withType:asSubrowOfRow:animate:)]
        pub unsafe fn insertRowAtIndex_withType_asSubrowOfRow_animate(
            &self,
            rowIndex: NSInteger,
            rowType: NSRuleEditorRowType,
            parentRow: NSInteger,
            shouldAnimate: bool,
        );

        #[method(setCriteria:andDisplayValues:forRowAtIndex:)]
        pub unsafe fn setCriteria_andDisplayValues_forRowAtIndex(
            &self,
            criteria: &NSArray,
            values: &NSArray,
            rowIndex: NSInteger,
        );

        #[method(removeRowAtIndex:)]
        pub unsafe fn removeRowAtIndex(&self, rowIndex: NSInteger);

        #[method(removeRowsAtIndexes:includeSubrows:)]
        pub unsafe fn removeRowsAtIndexes_includeSubrows(
            &self,
            rowIndexes: &NSIndexSet,
            includeSubrows: bool,
        );

        #[method_id(selectedRowIndexes)]
        pub unsafe fn selectedRowIndexes(&self) -> Id<NSIndexSet, Shared>;

        #[method(selectRowIndexes:byExtendingSelection:)]
        pub unsafe fn selectRowIndexes_byExtendingSelection(
            &self,
            indexes: &NSIndexSet,
            extend: bool,
        );

        #[method(rowClass)]
        pub unsafe fn rowClass(&self) -> &'static Class;

        #[method(setRowClass:)]
        pub unsafe fn setRowClass(&self, rowClass: &Class);

        #[method_id(rowTypeKeyPath)]
        pub unsafe fn rowTypeKeyPath(&self) -> Id<NSString, Shared>;

        #[method(setRowTypeKeyPath:)]
        pub unsafe fn setRowTypeKeyPath(&self, rowTypeKeyPath: &NSString);

        #[method_id(subrowsKeyPath)]
        pub unsafe fn subrowsKeyPath(&self) -> Id<NSString, Shared>;

        #[method(setSubrowsKeyPath:)]
        pub unsafe fn setSubrowsKeyPath(&self, subrowsKeyPath: &NSString);

        #[method_id(criteriaKeyPath)]
        pub unsafe fn criteriaKeyPath(&self) -> Id<NSString, Shared>;

        #[method(setCriteriaKeyPath:)]
        pub unsafe fn setCriteriaKeyPath(&self, criteriaKeyPath: &NSString);

        #[method_id(displayValuesKeyPath)]
        pub unsafe fn displayValuesKeyPath(&self) -> Id<NSString, Shared>;

        #[method(setDisplayValuesKeyPath:)]
        pub unsafe fn setDisplayValuesKeyPath(&self, displayValuesKeyPath: &NSString);
    }
);

pub type NSRuleEditorDelegate = NSObject;

extern "C" {
    static NSRuleEditorRowsDidChangeNotification: &'static NSNotificationName;
}
