#include <BrowserEngineCore/BEMemory.h>

#pragma clang diagnostic ignored "-Wunguarded-availability-new"

void rw() {
    be_memory_inline_jit_restrict_rwx_to_rw_with_witness();
}

void rx() {
    be_memory_inline_jit_restrict_rwx_to_rx_with_witness();
}
