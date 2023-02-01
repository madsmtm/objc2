	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_nonnull_nonnull
	.p2align	4, 0x90
_nonnull_nonnull:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	ebx, dword ptr [ebp + 16]
	mov	edi, dword ptr [ebx]
	mov	dword ptr [esp + 8], ebx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	mov	esi, eax
	mov	eax, dword ptr [ebx]
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	dword ptr [esp], edi
	call	_objc_release
	mov	eax, esi
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.globl	_null_nonnull
	.p2align	4, 0x90
_null_nonnull:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	edi, dword ptr [ebp + 16]
	mov	ecx, dword ptr [ebp + 12]
	mov	eax, dword ptr [ebp + 8]
	test	edi, edi
	je	LBB1_1
	mov	ebx, dword ptr [edi]
	sub	esp, 4
	push	edi
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 16
	mov	esi, eax
	sub	esp, 12
	push	dword ptr [edi]
	call	_objc_retain
	add	esp, 4
	push	ebx
	call	_objc_release
	add	esp, 16
	mov	eax, esi
	add	esp, 12
	jmp	LBB1_2
LBB1_1:
	sub	esp, 4
	push	0
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 28
LBB1_2:
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.globl	_nonnull_null
	.p2align	4, 0x90
_nonnull_null:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	ebx, dword ptr [ebp + 16]
	mov	edi, dword ptr [ebx]
	mov	dword ptr [esp + 8], ebx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	mov	esi, eax
	mov	eax, dword ptr [ebx]
	mov	dword ptr [esp], eax
	call	_objc_retain
	test	edi, edi
	je	LBB2_2
	mov	dword ptr [esp], edi
	call	_objc_release
LBB2_2:
	mov	eax, esi
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.globl	_null_null
	.p2align	4, 0x90
_null_null:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	ebx, dword ptr [ebp + 16]
	mov	ecx, dword ptr [ebp + 12]
	mov	eax, dword ptr [ebp + 8]
	test	ebx, ebx
	je	LBB3_1
	mov	edi, dword ptr [ebx]
	sub	esp, 4
	push	ebx
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 16
	mov	esi, eax
	sub	esp, 12
	push	dword ptr [ebx]
	call	_objc_retain
	add	esp, 16
	test	edi, edi
	je	LBB3_4
	sub	esp, 12
	push	edi
	call	_objc_release
	add	esp, 16
LBB3_4:
	mov	eax, esi
	add	esp, 12
	jmp	LBB3_5
LBB3_1:
	sub	esp, 4
	push	0
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 28
LBB3_5:
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.globl	_two_nonnull_nonnull
	.p2align	4, 0x90
_two_nonnull_nonnull:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 28
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	esi, dword ptr [ebp + 20]
	mov	ebx, dword ptr [ebp + 16]
	mov	edi, dword ptr [ebx]
	mov	edx, dword ptr [esi]
	mov	dword ptr [ebp - 16], edx
	mov	dword ptr [esp + 12], esi
	mov	dword ptr [esp + 8], ebx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	mov	esi, eax
	mov	eax, dword ptr [ebx]
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	dword ptr [esp], edi
	call	_objc_release
	mov	eax, dword ptr [ebp + 20]
	mov	eax, dword ptr [eax]
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	eax, dword ptr [ebp - 16]
	mov	dword ptr [esp], eax
	call	_objc_release
	mov	eax, esi
	add	esp, 28
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.globl	_call_with_none1
	.p2align	4, 0x90
_call_with_none1:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	sub	esp, 4
	push	0
	push	dword ptr [ebp + 12]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 24
	pop	ebp
	ret

	.globl	_call_with_none2
	.p2align	4, 0x90
_call_with_none2:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	sub	esp, 4
	push	0
	push	dword ptr [ebp + 12]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 24
	pop	ebp
	ret

	.globl	_call_with_none3
	.p2align	4, 0x90
_call_with_none3:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 8], 0
	sub	esp, 4
	lea	edx, [ebp - 8]
	push	edx
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 16
	mov	esi, eax
	sub	esp, 12
	push	dword ptr [ebp - 8]
	call	_objc_retain
	add	esp, 16
	mov	edx, dword ptr [ebp - 8]
	mov	eax, esi
	add	esp, 4
	pop	esi
	pop	ebp
	ret

	.globl	_call_with_none4
	.p2align	4, 0x90
_call_with_none4:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 8], 0
	sub	esp, 4
	lea	edx, [ebp - 8]
	push	edx
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 16
	mov	esi, eax
	sub	esp, 12
	push	dword ptr [ebp - 8]
	call	_objc_retain
	add	esp, 16
	mov	edx, dword ptr [ebp - 8]
	mov	eax, esi
	add	esp, 4
	pop	esi
	pop	ebp
	ret

	.globl	_call_with_some1
	.p2align	4, 0x90
_call_with_some1:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	sub	esp, 16
	mov	edi, dword ptr [ebp + 16]
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	lea	edx, [ebp + 16]
	mov	dword ptr [esp + 8], edx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	mov	esi, eax
	mov	eax, dword ptr [ebp + 16]
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	dword ptr [esp], edi
	call	_objc_release
	mov	edx, dword ptr [ebp + 16]
	mov	eax, esi
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret

	.globl	_call_with_some2
	.p2align	4, 0x90
_call_with_some2:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	sub	esp, 16
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	edi, dword ptr [ebp + 16]
	mov	dword ptr [ebp - 12], edi
	lea	edx, [ebp - 12]
	mov	dword ptr [esp + 8], edx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	mov	esi, eax
	mov	eax, dword ptr [ebp - 12]
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	dword ptr [esp], edi
	call	_objc_release
	mov	edx, dword ptr [ebp - 12]
	mov	eax, esi
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret

	.globl	_call_with_some3
	.p2align	4, 0x90
_call_with_some3:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	sub	esp, 16
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	edi, dword ptr [ebp + 16]
	mov	dword ptr [ebp - 12], edi
	lea	edx, [ebp - 12]
	mov	dword ptr [esp + 8], edx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	mov	esi, eax
	mov	eax, dword ptr [ebp - 12]
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	dword ptr [esp], edi
	call	_objc_release
	mov	edx, dword ptr [ebp - 12]
	mov	eax, esi
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret

.subsections_via_symbols
