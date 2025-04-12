	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_simple
	.p2align	4
_simple:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	_objc_autoreleaseReturnValue

	.globl	_with_body
	.p2align	4
_with_body:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	mov	dword ptr [esp], eax
	call	_objc_autoreleaseReturnValue
	add	esp, 8
	pop	ebp
	ret

.subsections_via_symbols
