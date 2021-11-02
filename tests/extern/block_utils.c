#include <stdint.h>
#include <Block.h>

typedef int32_t (^IntBlock)();
typedef int32_t (^AddBlock)(int32_t);

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
