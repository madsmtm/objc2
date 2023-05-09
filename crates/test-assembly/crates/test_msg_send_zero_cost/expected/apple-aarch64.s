	.section	__TEXT,__text,regular,pure_instructions
	.globl	_handle
	.p2align	2
_handle:
	b	_objc_msgSend

	.globl	_handle_with_sel
	.p2align	2
_handle_with_sel:
Lloh0:
	adrp	x8, _SEL_REF@PAGE
Lloh1:
	ldr	x1, [x8, _SEL_REF@PAGEOFF]
	b	_objc_msgSend
	.loh AdrpLdr	Lloh0, Lloh1

	.section	__TEXT,__const
	.globl	_SEL
_SEL:
	.asciz	"someSelector"

	.section	__DATA,__const
	.globl	_SEL_REF
	.p2align	3, 0x0
_SEL_REF:
	.quad	_SEL

.subsections_via_symbols
