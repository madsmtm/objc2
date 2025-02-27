; Function Attrs: nounwind uwtable
define void @rw() unnamed_addr #0 {
start:
  %0 = call { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } asm sideeffect "adr x0, 2f\0Amovz x1, #((0x4a4954l >>  0) & 0xFFFF), lsl #0\0Amovk x1, #((0x4a4954l >> 16) & 0xFFFF), lsl #16\0Abl _be_memory_inline_jit_restrict_rwx_to_rw_with_witness_impl\0A2:\0Anop", "=&{w0},=&{w1},=&{w2},=&{w3},=&{w4},=&{w5},=&{w6},=&{w7},=&{w8},=&{w16},=&{w17},=&{w30},~{cc},~{memory}"() #1, !srcloc !2
  ret void
}

; Function Attrs: nounwind uwtable
define void @rx() unnamed_addr #0 {
start:
  %0 = call { i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 } asm sideeffect "bl _be_memory_inline_jit_restrict_rwx_to_rx_with_witness_impl\0A2:\0Anop", "=&{w0},=&{w1},=&{w2},=&{w3},=&{w4},=&{w5},=&{w6},=&{w7},=&{w8},=&{w16},=&{w17},=&{w30},~{cc},~{memory}"() #1, !srcloc !3
  ret void
}
