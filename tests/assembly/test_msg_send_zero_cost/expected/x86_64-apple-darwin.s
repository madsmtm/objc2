	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 7
	.globl	_handle
	.p2align	4, 0x90
_handle:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	popq	%rbp
	jmp	_objc_msgSend
	.cfi_endproc

.subsections_via_symbols
