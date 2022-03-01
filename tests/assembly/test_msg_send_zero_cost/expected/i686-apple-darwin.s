	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 7
	.globl	_handle
	.p2align	4, 0x90
_handle:
	.cfi_startproc
	pushl	%ebp
	.cfi_def_cfa_offset 8
	.cfi_offset %ebp, -8
	movl	%esp, %ebp
	.cfi_def_cfa_register %ebp
	popl	%ebp
	jmp	_objc_msgSend
	.cfi_endproc

.subsections_via_symbols
