	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_simple
	.p2align	2
	.code	32
_simple:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_autoreleaseReturnValue
	pop	{r7, pc}

	.globl	_with_body
	.p2align	2
	.code	32
_with_body:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	bl	_objc_autoreleaseReturnValue
	pop	{r7, pc}

.subsections_via_symbols
