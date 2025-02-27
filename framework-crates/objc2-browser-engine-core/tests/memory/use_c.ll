; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define void @rw() #0 {
  call void asm sideeffect "adr x0, ${:uid}f\0Amovz x1, #((0x4a4954l >> 0) & 0xFFFF), lsl #0 \0Amovk x1, #((0x4a4954l >> 16) & 0xFFFF), lsl #16\0Abl _be_memory_inline_jit_restrict_rwx_to_rw_with_witness_impl\0A${:uid}:\0Anop\0A", "~{x0},~{x1},~{x2},~{x3},~{x4},~{x5},~{x6},~{x7},~{x8},~{x16},~{x17},~{lr},~{memory},~{cc}"() #1, !srcloc !6
  ret void
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define void @rx() #0 {
  call void asm sideeffect "bl _be_memory_inline_jit_restrict_rwx_to_rx_with_witness_impl\0A${:uid}:\0Anop\0A", "~{x0},~{x1},~{x2},~{x3},~{x4},~{x5},~{x6},~{x7},~{x8},~{x16},~{x17},~{lr},~{memory},~{cc}"() #1, !srcloc !7
  ret void
}
