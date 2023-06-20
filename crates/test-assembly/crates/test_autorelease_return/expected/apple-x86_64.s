	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_simple
	.p2align	4, 0x90
_simple:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	_objc_autoreleaseReturnValue

	.globl	_with_body
	.p2align	4, 0x90
_with_body:
	push	rbp
	mov	rbp, rsp
	call	_objc_msgSend
	mov	rdi, rax
	pop	rbp
	jmp	_objc_autoreleaseReturnValue

.subsections_via_symbols
