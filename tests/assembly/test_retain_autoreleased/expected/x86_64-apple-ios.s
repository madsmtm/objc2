	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle
	.p2align	4, 0x90
_handle:
	push	rbp
	mov	rbp, rsp
	call	_objc_msgSend
	mov	rdi, rax
	pop	rbp
	jmp	_objc_retainAutoreleasedReturnValue

.subsections_via_symbols
