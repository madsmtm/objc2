	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 7
	.intel_syntax noprefix
	.globl	_handle
	.p2align	4, 0x90
_handle:
	.cfi_startproc
	push	rbp
	.cfi_def_cfa_offset 16
	.cfi_offset rbp, -16
	mov	rbp, rsp
	.cfi_def_cfa_register rbp
	pop	rbp
	jmp	_objc_msgSend
	.cfi_endproc

.subsections_via_symbols
