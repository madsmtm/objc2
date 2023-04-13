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

	.globl	_handle_with_sel
	.p2align	2
	.code	32
_handle_with_sel:
	push	{r7, lr}
	mov	r7, sp
	movw	r1, :lower16:(_SEL_REF-(LPC1_0+8))
	movt	r1, :upper16:(_SEL_REF-(LPC1_0+8))
LPC1_0:
	ldr	r1, [pc, r1]
	bl	_objc_msgSend
	pop	{r7, pc}

	.section	__TEXT,__const
	.globl	_SEL
_SEL:
	.asciz	"someSelector"

	.section	__DATA,__const
	.globl	_SEL_REF
	.p2align	2, 0x0
_SEL_REF:
	.long	_SEL

.subsections_via_symbols
