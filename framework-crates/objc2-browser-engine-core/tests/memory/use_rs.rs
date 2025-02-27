use objc2_browser_engine_core::{
    be_memory_inline_jit_restrict_rwx_to_rw_with_witness,
    be_memory_inline_jit_restrict_rwx_to_rx_with_witness,
};

#[no_mangle]
extern "C" fn rw() {
    be_memory_inline_jit_restrict_rwx_to_rw_with_witness();
}

#[no_mangle]
extern "C" fn rx() {
    be_memory_inline_jit_restrict_rwx_to_rx_with_witness();
}
