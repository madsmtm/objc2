#include <stdint.h>
#include <Block.h>

typedef struct {
    float x;
    uint8_t y[100];
} LargeStruct;

typedef int32_t (^IntBlock)();
typedef int32_t (^AddBlock)(int32_t);
typedef LargeStruct (^LargeStructBlock)(LargeStruct);

IntBlock get_int_block() {
    return ^{ return (int32_t)7; };
}

IntBlock get_int_block_with(int32_t i) {
    return Block_copy(^{ return i; });
}

AddBlock get_add_block() {
    return ^(int32_t a) { return a + 7; };
}

AddBlock get_add_block_with(int32_t i) {
    return Block_copy(^(int32_t a) { return a + i; });
}

int32_t invoke_int_block(IntBlock block) {
    return block();
}

int32_t invoke_add_block(AddBlock block, int32_t a) {
    return block(a);
}

LargeStructBlock get_large_struct_block() {
    return ^(LargeStruct s) {
        s.x -= 1.0;
        s.y[12] += 1;
        s.y[99] = 123;
        return s;
    };
}

LargeStructBlock get_large_struct_block_with(LargeStruct a) {
    return Block_copy(^(LargeStruct s) {
        (void)s; // Unused
        return a;
    });
}

LargeStruct invoke_large_struct_block(LargeStructBlock block, LargeStruct s) {
    return block(s);
}
