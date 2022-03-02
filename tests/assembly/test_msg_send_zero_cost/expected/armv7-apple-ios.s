	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle
	.p2align	2
	.code	32
_handle:
	b	_objc_msgSend

.subsections_via_symbols
