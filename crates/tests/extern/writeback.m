#include <Foundation/NSObject.h>

@interface Writeback: NSObject
@end

__attribute__((objc_runtime_visible))
@interface WritebackRust
+(void) write: (NSObject*) obj toParamNonNullNonNull: (NSObject**) param;
+(void) write: (NSObject*) obj toParamNonNullNullable: (NSObject**) param;
+(void) write: (NSObject*) obj toParamNullableNonNull: (NSObject**) param;
+(void) write: (NSObject*) obj toParamNullableNullable: (NSObject**) param;
@end

@implementation Writeback
+(void) write: (NSObject*) obj toParam: (NSObject**) param {
    if (param) {
        *param = obj;
    }
}

// This is unsound in Clang's ARC, don't attempt to call this!
+(void) writeAutoreleased: (NSObject*) obj toParam: (NSObject**) param {
    @autoreleasepool {
        if (param) {
            *param = obj;
        }
    }
}

+(void) forward: (NSObject*) obj toParamNonNullNonNull: (NSObject**) param {
    [WritebackRust write: obj toParamNonNullNonNull: param];
}

+(void) forward: (NSObject*) obj toParamNonNullNullable: (NSObject**) param {
    [WritebackRust write: obj toParamNonNullNullable: param];
}

+(void) forward: (NSObject*) obj toParamNullableNonNull: (NSObject**) param {
    [WritebackRust write: obj toParamNullableNonNull: param];
}

+(void) forward: (NSObject*) obj toParamNullableNullable: (NSObject**) param {
    [WritebackRust write: obj toParamNullableNullable: param];
}
@end
