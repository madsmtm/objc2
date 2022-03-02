	.section	__TEXT,__text,regular,pure_instructions
	.globl	_handle
	.p2align	2
_handle:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_objc_msgSend
	ldp	x29, x30, [sp], #16
	b	_objc_retainAutoreleasedReturnValue

.subsections_via_symbols
