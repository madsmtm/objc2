	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle
	.p2align	4, 0x90
_handle:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	_objc_msgSend

	.globl	_handle_with_sel
	.p2align	4, 0x90
_handle_with_sel:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	call	L1$pb
L1$pb:
	pop	eax
	sub	esp, 8
	push	dword ptr [eax + _SEL_REF-L1$pb]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 24
	pop	ebp
	ret

	.section	__TEXT,__const
	.globl	_SEL
_SEL:
	.asciz	"someSelector"

	.section	__DATA,__const
	.globl	_SEL_REF
	.p2align	2
_SEL_REF:
	.long	_SEL

.subsections_via_symbols
