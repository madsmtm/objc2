	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 11, 0
	.globl	_handle
	.p2align	2
_handle:
	.cfi_startproc
	b	_objc_msgSend
	.cfi_endproc

.subsections_via_symbols
