	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle
	.p2align	4, 0x90
_handle:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	_objc_msgSend

	.globl	_handle_with_sel
	.p2align	4, 0x90
_handle_with_sel:
	push	rbp
	mov	rbp, rsp
	mov	rsi, qword ptr [rip + _SEL_REF]
	pop	rbp
	jmp	_objc_msgSend

	.section	__TEXT,__const
	.globl	_SEL
_SEL:
	.asciz	"someSelector"

	.section	__DATA,__const
	.globl	_SEL_REF
	.p2align	3
_SEL_REF:
	.quad	_SEL

.subsections_via_symbols
