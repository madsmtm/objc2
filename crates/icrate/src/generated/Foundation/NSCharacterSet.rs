#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSCharacterSet;
    unsafe impl ClassType for NSCharacterSet {
        type Super = NSObject;
    }
);
impl NSCharacterSet {
    pub unsafe fn characterSetWithRange(aRange: NSRange) -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), characterSetWithRange: aRange]
    }
    pub unsafe fn characterSetWithCharactersInString(
        aString: &NSString,
    ) -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), characterSetWithCharactersInString: aString]
    }
    pub unsafe fn characterSetWithBitmapRepresentation(
        data: &NSData,
    ) -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), characterSetWithBitmapRepresentation: data]
    }
    pub unsafe fn characterSetWithContentsOfFile(
        fName: &NSString,
    ) -> Option<Id<NSCharacterSet, Shared>> {
        msg_send_id![Self::class(), characterSetWithContentsOfFile: fName]
    }
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared> {
        msg_send_id![self, initWithCoder: coder]
    }
    pub unsafe fn characterIsMember(&self, aCharacter: unichar) -> bool {
        msg_send![self, characterIsMember: aCharacter]
    }
    pub unsafe fn longCharacterIsMember(&self, theLongChar: UTF32Char) -> bool {
        msg_send![self, longCharacterIsMember: theLongChar]
    }
    pub unsafe fn isSupersetOfSet(&self, theOtherSet: &NSCharacterSet) -> bool {
        msg_send![self, isSupersetOfSet: theOtherSet]
    }
    pub unsafe fn hasMemberInPlane(&self, thePlane: uint8_t) -> bool {
        msg_send![self, hasMemberInPlane: thePlane]
    }
    pub unsafe fn controlCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), controlCharacterSet]
    }
    pub unsafe fn whitespaceCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), whitespaceCharacterSet]
    }
    pub unsafe fn whitespaceAndNewlineCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), whitespaceAndNewlineCharacterSet]
    }
    pub unsafe fn decimalDigitCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), decimalDigitCharacterSet]
    }
    pub unsafe fn letterCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), letterCharacterSet]
    }
    pub unsafe fn lowercaseLetterCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), lowercaseLetterCharacterSet]
    }
    pub unsafe fn uppercaseLetterCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), uppercaseLetterCharacterSet]
    }
    pub unsafe fn nonBaseCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), nonBaseCharacterSet]
    }
    pub unsafe fn alphanumericCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), alphanumericCharacterSet]
    }
    pub unsafe fn decomposableCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), decomposableCharacterSet]
    }
    pub unsafe fn illegalCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), illegalCharacterSet]
    }
    pub unsafe fn punctuationCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), punctuationCharacterSet]
    }
    pub unsafe fn capitalizedLetterCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), capitalizedLetterCharacterSet]
    }
    pub unsafe fn symbolCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), symbolCharacterSet]
    }
    pub unsafe fn newlineCharacterSet() -> Id<NSCharacterSet, Shared> {
        msg_send_id![Self::class(), newlineCharacterSet]
    }
    pub unsafe fn bitmapRepresentation(&self) -> Id<NSData, Shared> {
        msg_send_id![self, bitmapRepresentation]
    }
    pub unsafe fn invertedSet(&self) -> Id<NSCharacterSet, Shared> {
        msg_send_id![self, invertedSet]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSMutableCharacterSet;
    unsafe impl ClassType for NSMutableCharacterSet {
        type Super = NSCharacterSet;
    }
);
impl NSMutableCharacterSet {
    pub unsafe fn addCharactersInRange(&self, aRange: NSRange) {
        msg_send![self, addCharactersInRange: aRange]
    }
    pub unsafe fn removeCharactersInRange(&self, aRange: NSRange) {
        msg_send![self, removeCharactersInRange: aRange]
    }
    pub unsafe fn addCharactersInString(&self, aString: &NSString) {
        msg_send![self, addCharactersInString: aString]
    }
    pub unsafe fn removeCharactersInString(&self, aString: &NSString) {
        msg_send![self, removeCharactersInString: aString]
    }
    pub unsafe fn formUnionWithCharacterSet(&self, otherSet: &NSCharacterSet) {
        msg_send![self, formUnionWithCharacterSet: otherSet]
    }
    pub unsafe fn formIntersectionWithCharacterSet(&self, otherSet: &NSCharacterSet) {
        msg_send![self, formIntersectionWithCharacterSet: otherSet]
    }
    pub unsafe fn invert(&self) {
        msg_send![self, invert]
    }
    pub unsafe fn controlCharacterSet() -> Id<NSMutableCharacterSet, Shared> {
        msg_send_id![Self::class(), controlCharacterSet]
    }
    pub unsafe fn whitespaceCharacterSet() -> Id<NSMutableCharacterSet, Shared> {
        msg_send_id![Self::class(), whitespaceCharacterSet]
    }
    pub unsafe fn whitespaceAndNewlineCharacterSet() -> Id<NSMutableCharacterSet, Shared> {
        msg_send_id![Self::class(), whitespaceAndNewlineCharacterSet]
    }
    pub unsafe fn decimalDigitCharacterSet() -> Id<NSMutableCharacterSet, Shared> {
        msg_send_id![Self::class(), decimalDigitCharacterSet]
    }
    pub unsafe fn letterCharacterSet() -> Id<NSMutableCharacterSet, Shared> {
        msg_send_id![Self::class(), letterCharacterSet]
    }
    pub unsafe fn lowercaseLetterCharacterSet() -> Id<NSMutableCharacterSet, Shared> {
        msg_send_id![Self::class(), lowercaseLetterCharacterSet]
    }
    pub unsafe fn uppercaseLetterCharacterSet() -> Id<NSMutableCharacterSet, Shared> {
        msg_send_id![Self::class(), uppercaseLetterCharacterSet]
    }
    pub unsafe fn nonBaseCharacterSet() -> Id<NSMutableCharacterSet, Shared> {
        msg_send_id![Self::class(), nonBaseCharacterSet]
    }
    pub unsafe fn alphanumericCharacterSet() -> Id<NSMutableCharacterSet, Shared> {
        msg_send_id![Self::class(), alphanumericCharacterSet]
    }
    pub unsafe fn decomposableCharacterSet() -> Id<NSMutableCharacterSet, Shared> {
        msg_send_id![Self::class(), decomposableCharacterSet]
    }
    pub unsafe fn illegalCharacterSet() -> Id<NSMutableCharacterSet, Shared> {
        msg_send_id![Self::class(), illegalCharacterSet]
    }
    pub unsafe fn punctuationCharacterSet() -> Id<NSMutableCharacterSet, Shared> {
        msg_send_id![Self::class(), punctuationCharacterSet]
    }
    pub unsafe fn capitalizedLetterCharacterSet() -> Id<NSMutableCharacterSet, Shared> {
        msg_send_id![Self::class(), capitalizedLetterCharacterSet]
    }
    pub unsafe fn symbolCharacterSet() -> Id<NSMutableCharacterSet, Shared> {
        msg_send_id![Self::class(), symbolCharacterSet]
    }
    pub unsafe fn newlineCharacterSet() -> Id<NSMutableCharacterSet, Shared> {
        msg_send_id![Self::class(), newlineCharacterSet]
    }
    pub unsafe fn characterSetWithRange(aRange: NSRange) -> Id<NSMutableCharacterSet, Shared> {
        msg_send_id![Self::class(), characterSetWithRange: aRange]
    }
    pub unsafe fn characterSetWithCharactersInString(
        aString: &NSString,
    ) -> Id<NSMutableCharacterSet, Shared> {
        msg_send_id![Self::class(), characterSetWithCharactersInString: aString]
    }
    pub unsafe fn characterSetWithBitmapRepresentation(
        data: &NSData,
    ) -> Id<NSMutableCharacterSet, Shared> {
        msg_send_id![Self::class(), characterSetWithBitmapRepresentation: data]
    }
    pub unsafe fn characterSetWithContentsOfFile(
        fName: &NSString,
    ) -> Option<Id<NSMutableCharacterSet, Shared>> {
        msg_send_id![Self::class(), characterSetWithContentsOfFile: fName]
    }
}
