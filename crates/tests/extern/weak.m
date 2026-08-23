#include <Foundation/NSObject.h>

@interface HasWeakIvar : NSObject {
    __weak NSObject* _object;
    __weak NSObject** _objPtr;
}
@end

@implementation HasWeakIvar
- (instancetype)initWithObject:(NSObject*)object {
    self = [super init];
    if (self) {
        _object = object;
        _objPtr = &_object;
    }
    return self;
}

// Use `copy` methods to avoid dealing with autorelease optimization.
- (NSObject*)copyObject {
    return _object;
}

// NOTE: Returning `__weak NSObject*` doesn't return a `Weak<NSObject>`
// ABI-wise (because what would it even mean?), but instead
// `Retained<NSObject>`, and then it probably encourages the caller to
// use `__weak` as their storage.
//
// A `__weak NSObject**` does become a `&Weak<NSObject>` as expected.
- (__weak NSObject**)rawObject {
    return &_object;
}
@end
