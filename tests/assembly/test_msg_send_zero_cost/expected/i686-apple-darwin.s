	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 7
	.intel_syntax noprefix
	.globl	_handle
	.p2align	4, 0x90
_handle:
	.cfi_startproc
	push	ebp
	.cfi_def_cfa_offset 8
	.cfi_offset ebp, -8
	mov	ebp, esp
	.cfi_def_cfa_register ebp
	pop	ebp
	jmp	_objc_msgSend
	.cfi_endproc

.subsections_via_symbols
