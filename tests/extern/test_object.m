#include <Foundation/NSObject.h>

@interface MyTestObject: NSObject <NSObject> {
    int var1;
    BOOL var2;
}

+ (instancetype) getAutoreleasedInstance;
+ (int) add: (int) a and: (int) b;

- (int) var1;
- (void) addToVar1: (int) number;

- (BOOL) var2;
@end


@implementation MyTestObject
- (id)init {
    self = [super init];
    if (self) {
        var1 = 42;
        var2 = YES;
    }
    return self;
}

+ (instancetype) getAutoreleasedInstance {
    return [[MyTestObject alloc] init];
}

+ (int) add: (int) a and: (int) b {
    return a + b;
}

- (int) var1 {
    return var1;
}

- (void) addToVar1: (int) number {
    var1 += number;
}

- (BOOL) var2 {
    return var2;
}
@end
