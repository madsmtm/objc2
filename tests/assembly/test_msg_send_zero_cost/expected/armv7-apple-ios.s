	.section	__TEXT,__text,regular,pure_instructions
	.ios_version_min 7, 0
	.syntax unified
	.globl	_handle
	.p2align	2
	.code	32
_handle:
	b	_objc_msgSend

.subsections_via_symbols
