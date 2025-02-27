	.section	__TEXT,__text,regular,pure_instructions
	.globl	_rw
	.p2align	2
_rw:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	; InlineAsm Start
	adr	x0, Ltmp0
	mov	x1, #18772
	movk	x1, #74, lsl #16
	bl	_be_memory_inline_jit_restrict_rwx_to_rw_with_witness_impl
Ltmp0:
	nop
	; InlineAsm End
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.globl	_rx
	.p2align	2
_rx:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	; InlineAsm Start
	bl	_be_memory_inline_jit_restrict_rwx_to_rx_with_witness_impl
Ltmp1:
	nop
	; InlineAsm End
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

.subsections_via_symbols
