	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle
	.p2align	2
	.code	32
_handle:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	pop	{r7, pc}

.subsections_via_symbols
