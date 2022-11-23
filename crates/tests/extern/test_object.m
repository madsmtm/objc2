#include <Foundation/NSObject.h>
#include <Foundation/NSValue.h>

@protocol MyTestProtocol <NSObject>
- (int) a;
+ (int) b;
- (id) c;
+ (id) d;
@optional
- (int) e;
+ (int) f;
- (id) g;
+ (id) h;
@end

@protocol MyTestProtocol2
@end

@protocol MyTestProtocol3 <MyTestProtocol>
@end

@protocol MyTestProtocol4 <MyTestProtocol3, MyTestProtocol2>
@end

@interface MyTestObject: NSObject <MyTestProtocol> {
    int var1;
    BOOL var2;
    id var3;
}

+ (instancetype) getAutoreleasedInstance;
+ (int) add: (int) a and: (int) b;

- (int) var1;
- (void) addToVar1: (int) number;

- (BOOL) var2;

- (id) var3;
- (void) setVar3: (id) obj;
@end


@implementation MyTestObject
- (id)init {
    self = [super init];
    if (self) {
        var1 = 42;
        var2 = YES;
        // var3 = nil;
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

- (id) var3 {
    return var3;
}

- (void) setVar3: (id) obj {
    var3 = obj;
}

- (int) a {
    return 1;
}
+ (int) b {
    return 2;
}
- (id) c {
    return @3;
}
+ (id) d {
    return @4;
}
- (int) e {
    return 5;
}
+ (int) f {
    return 6;
}
- (id) g {
    return @7;
}
+ (id) h {
    return @8;
}
@end
