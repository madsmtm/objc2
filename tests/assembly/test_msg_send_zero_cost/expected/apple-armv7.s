	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle
	.p2align	2
	.code	32
_handle:
	b	_objc_msgSend

	.globl	_handle_with_sel
	.p2align	2
	.code	32
_handle_with_sel:
	movw	r1, :lower16:(_SEL_REF-(LPC1_0+8))
	movt	r1, :upper16:(_SEL_REF-(LPC1_0+8))
LPC1_0:
	ldr	r1, [pc, r1]
	b	_objc_msgSend

	.section	__TEXT,__const
	.globl	_SEL
_SEL:
	.asciz	"someSelector"

	.section	__DATA,__const
	.globl	_SEL_REF
	.p2align	2
_SEL_REF:
	.long	_SEL

.subsections_via_symbols
