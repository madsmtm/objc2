	.syntax	unified
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_simple
	.p2align	2
	.code	32
_simple:
	push	{r7, lr}
	mov	r7, sp
	pop	{r7, lr}
	b	_objc_autoreleaseReturnValue

	.globl	_with_body
	.p2align	2
	.code	32
_with_body:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	pop	{r7, lr}
	b	_objc_autoreleaseReturnValue

.subsections_via_symbols
