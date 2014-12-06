#import <Foundation/Foundation.h>

typedef NSInteger (^IntBlock)();
typedef NSInteger (^AddBlock)(NSInteger);

IntBlock get_int_block() {
    return ^{ return (NSInteger)7; };
}

IntBlock get_int_block_with(NSInteger i) {
    return [^{ return i; } copy];
}

AddBlock get_add_block() {
    return ^(NSInteger a) { return a + 7; };
}

AddBlock get_add_block_with(NSInteger i) {
    return [^(NSInteger a) { return a + i; } copy];
}

NSInteger invoke_int_block(IntBlock block) {
    return block();
}

NSInteger invoke_add_block(AddBlock block, NSInteger a) {
    return block(a);
}
