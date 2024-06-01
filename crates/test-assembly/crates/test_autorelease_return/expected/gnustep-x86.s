	.text
	.intel_syntax noprefix
	.section	.text.simple,"ax",@progbits
	.globl	simple
	.p2align	4, 0x90
	.type	simple,@function
simple:
	push	ebx
	sub	esp, 8
	mov	eax, dword ptr [esp + 16]
	call	.L0$pb
.L0$pb:
	pop	ebx
.Ltmp0:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp0-.L0$pb)
	mov	dword ptr [esp], eax
	call	objc_autoreleaseReturnValue@PLT
	add	esp, 8
	pop	ebx
	ret
.Lfunc_end0:
	.size	simple, .Lfunc_end0-simple

	.section	.text.with_body,"ax",@progbits
	.globl	with_body
	.p2align	4, 0x90
	.type	with_body,@function
with_body:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L1$pb
.L1$pb:
	pop	ebx
.Ltmp1:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp1-.L1$pb)
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	eax
	mov	dword ptr [esp], eax
	call	objc_autoreleaseReturnValue@PLT
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end1:
	.size	with_body, .Lfunc_end1-with_body

	.section	".note.GNU-stack","",@progbits
