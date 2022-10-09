use super::__exported::NSData;
use crate::CoreFoundation::generated::CFCharacterSet::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRange::*;
use crate::Foundation::generated::NSString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSCharacterSet;
    unsafe impl ClassType for NSCharacterSet {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSCharacterSet {
        #[method_id(controlCharacterSet)]
        pub unsafe fn controlCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(whitespaceCharacterSet)]
        pub unsafe fn whitespaceCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(whitespaceAndNewlineCharacterSet)]
        pub unsafe fn whitespaceAndNewlineCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(decimalDigitCharacterSet)]
        pub unsafe fn decimalDigitCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(letterCharacterSet)]
        pub unsafe fn letterCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(lowercaseLetterCharacterSet)]
        pub unsafe fn lowercaseLetterCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(uppercaseLetterCharacterSet)]
        pub unsafe fn uppercaseLetterCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(nonBaseCharacterSet)]
        pub unsafe fn nonBaseCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(alphanumericCharacterSet)]
        pub unsafe fn alphanumericCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(decomposableCharacterSet)]
        pub unsafe fn decomposableCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(illegalCharacterSet)]
        pub unsafe fn illegalCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(punctuationCharacterSet)]
        pub unsafe fn punctuationCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(capitalizedLetterCharacterSet)]
        pub unsafe fn capitalizedLetterCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(symbolCharacterSet)]
        pub unsafe fn symbolCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(newlineCharacterSet)]
        pub unsafe fn newlineCharacterSet() -> Id<NSCharacterSet, Shared>;
        #[method_id(characterSetWithRange:)]
        pub unsafe fn characterSetWithRange(aRange: NSRange) -> Id<NSCharacterSet, Shared>;
        #[method_id(characterSetWithCharactersInString:)]
        pub unsafe fn characterSetWithCharactersInString(
            aString: &NSString,
        ) -> Id<NSCharacterSet, Shared>;
        #[method_id(characterSetWithBitmapRepresentation:)]
        pub unsafe fn characterSetWithBitmapRepresentation(
            data: &NSData,
        ) -> Id<NSCharacterSet, Shared>;
        #[method_id(characterSetWithContentsOfFile:)]
        pub unsafe fn characterSetWithContentsOfFile(
            fName: &NSString,
        ) -> Option<Id<NSCharacterSet, Shared>>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method(characterIsMember:)]
        pub unsafe fn characterIsMember(&self, aCharacter: unichar) -> bool;
        #[method_id(bitmapRepresentation)]
        pub unsafe fn bitmapRepresentation(&self) -> Id<NSData, Shared>;
        #[method_id(invertedSet)]
        pub unsafe fn invertedSet(&self) -> Id<NSCharacterSet, Shared>;
        #[method(longCharacterIsMember:)]
        pub unsafe fn longCharacterIsMember(&self, theLongChar: UTF32Char) -> bool;
        #[method(isSupersetOfSet:)]
        pub unsafe fn isSupersetOfSet(&self, theOtherSet: &NSCharacterSet) -> bool;
        #[method(hasMemberInPlane:)]
        pub unsafe fn hasMemberInPlane(&self, thePlane: u8) -> bool;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSMutableCharacterSet;
    unsafe impl ClassType for NSMutableCharacterSet {
        type Super = NSCharacterSet;
    }
);
extern_methods!(
    unsafe impl NSMutableCharacterSet {
        #[method(addCharactersInRange:)]
        pub unsafe fn addCharactersInRange(&self, aRange: NSRange);
        #[method(removeCharactersInRange:)]
        pub unsafe fn removeCharactersInRange(&self, aRange: NSRange);
        #[method(addCharactersInString:)]
        pub unsafe fn addCharactersInString(&self, aString: &NSString);
        #[method(removeCharactersInString:)]
        pub unsafe fn removeCharactersInString(&self, aString: &NSString);
        #[method(formUnionWithCharacterSet:)]
        pub unsafe fn formUnionWithCharacterSet(&self, otherSet: &NSCharacterSet);
        #[method(formIntersectionWithCharacterSet:)]
        pub unsafe fn formIntersectionWithCharacterSet(&self, otherSet: &NSCharacterSet);
        #[method(invert)]
        pub unsafe fn invert(&self);
        #[method_id(controlCharacterSet)]
        pub unsafe fn controlCharacterSet() -> Id<NSMutableCharacterSet, Shared>;
        #[method_id(whitespaceCharacterSet)]
        pub unsafe fn whitespaceCharacterSet() -> Id<NSMutableCharacterSet, Shared>;
        #[method_id(whitespaceAndNewlineCharacterSet)]
        pub unsafe fn whitespaceAndNewlineCharacterSet() -> Id<NSMutableCharacterSet, Shared>;
        #[method_id(decimalDigitCharacterSet)]
        pub unsafe fn decimalDigitCharacterSet() -> Id<NSMutableCharacterSet, Shared>;
        #[method_id(letterCharacterSet)]
        pub unsafe fn letterCharacterSet() -> Id<NSMutableCharacterSet, Shared>;
        #[method_id(lowercaseLetterCharacterSet)]
        pub unsafe fn lowercaseLetterCharacterSet() -> Id<NSMutableCharacterSet, Shared>;
        #[method_id(uppercaseLetterCharacterSet)]
        pub unsafe fn uppercaseLetterCharacterSet() -> Id<NSMutableCharacterSet, Shared>;
        #[method_id(nonBaseCharacterSet)]
        pub unsafe fn nonBaseCharacterSet() -> Id<NSMutableCharacterSet, Shared>;
        #[method_id(alphanumericCharacterSet)]
        pub unsafe fn alphanumericCharacterSet() -> Id<NSMutableCharacterSet, Shared>;
        #[method_id(decomposableCharacterSet)]
        pub unsafe fn decomposableCharacterSet() -> Id<NSMutableCharacterSet, Shared>;
        #[method_id(illegalCharacterSet)]
        pub unsafe fn illegalCharacterSet() -> Id<NSMutableCharacterSet, Shared>;
        #[method_id(punctuationCharacterSet)]
        pub unsafe fn punctuationCharacterSet() -> Id<NSMutableCharacterSet, Shared>;
        #[method_id(capitalizedLetterCharacterSet)]
        pub unsafe fn capitalizedLetterCharacterSet() -> Id<NSMutableCharacterSet, Shared>;
        #[method_id(symbolCharacterSet)]
        pub unsafe fn symbolCharacterSet() -> Id<NSMutableCharacterSet, Shared>;
        #[method_id(newlineCharacterSet)]
        pub unsafe fn newlineCharacterSet() -> Id<NSMutableCharacterSet, Shared>;
        #[method_id(characterSetWithRange:)]
        pub unsafe fn characterSetWithRange(aRange: NSRange) -> Id<NSMutableCharacterSet, Shared>;
        #[method_id(characterSetWithCharactersInString:)]
        pub unsafe fn characterSetWithCharactersInString(
            aString: &NSString,
        ) -> Id<NSMutableCharacterSet, Shared>;
        #[method_id(characterSetWithBitmapRepresentation:)]
        pub unsafe fn characterSetWithBitmapRepresentation(
            data: &NSData,
        ) -> Id<NSMutableCharacterSet, Shared>;
        #[method_id(characterSetWithContentsOfFile:)]
        pub unsafe fn characterSetWithContentsOfFile(
            fName: &NSString,
        ) -> Option<Id<NSMutableCharacterSet, Shared>>;
    }
);
