	.intel_syntax noprefix
	.section	.text.handle,"ax",@progbits
	.globl	handle
	.p2align	4
	.type	handle,@function
handle:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L0$pb
.L0$pb:
	pop	ebx
.Ltmp0:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp0-.L0$pb)
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	eax
	mov	dword ptr [esp], eax
	call	objc_retainAutoreleasedReturnValue@PLT
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end0:
	.size	handle, .Lfunc_end0-handle

	.section	".note.GNU-stack","",@progbits
