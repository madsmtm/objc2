	.section	__TEXT,__text,regular,pure_instructions
	.globl	_simple
	.p2align	2
_simple:
	b	_objc_autoreleaseReturnValue

	.globl	_with_body
	.p2align	2
_with_body:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_objc_msgSend
	ldp	x29, x30, [sp], #16
	b	_objc_autoreleaseReturnValue

.subsections_via_symbols
