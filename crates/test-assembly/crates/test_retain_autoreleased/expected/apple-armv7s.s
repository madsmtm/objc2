	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle
	.p2align	2
	.code	32
_handle:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	@ InlineAsm Start
	mov	r7, r7
	@ InlineAsm End
	bl	_objc_retainAutoreleasedReturnValue
	pop	{r7, pc}

.subsections_via_symbols
