#include <Foundation/NSObject.h>

@interface Writeback: NSObject
@end

@implementation Writeback
+(void) write: (NSObject*) obj toParam: (NSObject**) param {
    if (param) {
        *param = obj;
    }
}

+(void) writeNullParam: (NSObject**) param {
    if (param) {
        *param = nil;
    }
}

+(void) writeNewParam: (Writeback**) param {
    if (param) {
        *param = [Writeback new];
    }
}

// This is unsound in Clang's ARC, don't attempt to call this!
+(void) writeNewParamAutoreleased: (Writeback**) param {
    @autoreleasepool {
        if (param) {
            *param = [Writeback new];
        }
    }
}
@end
