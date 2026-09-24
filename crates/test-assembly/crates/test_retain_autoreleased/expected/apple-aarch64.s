	.section	__TEXT,__text,regular,pure_instructions
	.globl	_handle
	.p2align	2
_handle:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_objc_msgSend
	; InlineAsm Start
	mov	x29, x29
	; InlineAsm End
	bl	_objc_retainAutoreleasedReturnValue
	; InlineAsm Start
	nop
	; InlineAsm End
	ldp	x29, x30, [sp], #16
	ret

.subsections_via_symbols
