	.text
	.intel_syntax noprefix
	.section	.text.nonnull_nonnull,"ax",@progbits
	.globl	nonnull_nonnull
	.p2align	4, 0x90
	.type	nonnull_nonnull,@function
nonnull_nonnull:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	eax, dword ptr [esp + 32]
	mov	esi, dword ptr [esp + 36]
	mov	edi, dword ptr [esp + 40]
	call	.L0$pb
.L0$pb:
	pop	ebx
.Ltmp0:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp0-.L0$pb)
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], eax
	call	objc_msg_lookup@PLT
	mov	ecx, dword ptr [esp + 32]
	mov	ebp, dword ptr [edi]
	mov	dword ptr [esp + 8], edi
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], ecx
	call	eax
	mov	esi, eax
	mov	eax, dword ptr [edi]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	mov	dword ptr [esp], ebp
	call	objc_release@PLT
	mov	eax, esi
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end0:
	.size	nonnull_nonnull, .Lfunc_end0-nonnull_nonnull

	.section	.text.null_nonnull,"ax",@progbits
	.globl	null_nonnull
	.p2align	4, 0x90
	.type	null_nonnull,@function
null_nonnull:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	edi, dword ptr [esp + 40]
	mov	ebp, dword ptr [esp + 36]
	mov	esi, dword ptr [esp + 32]
	call	.L1$pb
.L1$pb:
	pop	ebx
.Ltmp1:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp1-.L1$pb)
	sub	esp, 8
	push	ebp
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 16
	test	edi, edi
	je	.LBB1_1
	mov	ecx, dword ptr [edi]
	mov	dword ptr [esp + 8], ecx
	sub	esp, 4
	push	edi
	push	ebp
	push	esi
	call	eax
	add	esp, 16
	mov	esi, eax
	sub	esp, 12
	push	dword ptr [edi]
	call	objc_retain@PLT
	add	esp, 4
	push	dword ptr [esp + 20]
	call	objc_release@PLT
	add	esp, 16
	mov	eax, esi
	add	esp, 12
	jmp	.LBB1_2
.LBB1_1:
	sub	esp, 4
	push	0
	push	ebp
	push	esi
	call	eax
	add	esp, 28
.LBB1_2:
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end1:
	.size	null_nonnull, .Lfunc_end1-null_nonnull

	.section	.text.nonnull_null,"ax",@progbits
	.globl	nonnull_null
	.p2align	4, 0x90
	.type	nonnull_null,@function
nonnull_null:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	eax, dword ptr [esp + 32]
	mov	esi, dword ptr [esp + 36]
	mov	ebp, dword ptr [esp + 40]
	call	.L2$pb
.L2$pb:
	pop	ebx
.Ltmp2:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp2-.L2$pb)
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], eax
	call	objc_msg_lookup@PLT
	mov	ecx, dword ptr [esp + 32]
	mov	edi, dword ptr [ebp]
	mov	dword ptr [esp + 8], ebp
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], ecx
	call	eax
	mov	esi, eax
	mov	eax, dword ptr [ebp]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	test	edi, edi
	je	.LBB2_2
	mov	dword ptr [esp], edi
	call	objc_release@PLT
.LBB2_2:
	mov	eax, esi
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end2:
	.size	nonnull_null, .Lfunc_end2-nonnull_null

	.section	.text.null_null,"ax",@progbits
	.globl	null_null
	.p2align	4, 0x90
	.type	null_null,@function
null_null:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	edi, dword ptr [esp + 40]
	mov	ebp, dword ptr [esp + 36]
	mov	esi, dword ptr [esp + 32]
	call	.L3$pb
.L3$pb:
	pop	ebx
.Ltmp3:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp3-.L3$pb)
	sub	esp, 8
	push	ebp
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 16
	test	edi, edi
	je	.LBB3_1
	mov	ecx, esi
	mov	esi, dword ptr [edi]
	sub	esp, 4
	push	edi
	push	ebp
	push	ecx
	call	eax
	add	esp, 16
	mov	ebp, eax
	sub	esp, 12
	push	dword ptr [edi]
	call	objc_retain@PLT
	add	esp, 16
	test	esi, esi
	je	.LBB3_4
	sub	esp, 12
	push	esi
	call	objc_release@PLT
	add	esp, 16
.LBB3_4:
	mov	eax, ebp
	add	esp, 12
	jmp	.LBB3_5
.LBB3_1:
	sub	esp, 4
	push	0
	push	ebp
	push	esi
	call	eax
	add	esp, 28
.LBB3_5:
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end3:
	.size	null_null, .Lfunc_end3-null_null

	.section	.text.two_nonnull_nonnull,"ax",@progbits
	.globl	two_nonnull_nonnull
	.p2align	4, 0x90
	.type	two_nonnull_nonnull,@function
two_nonnull_nonnull:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 28
	mov	eax, dword ptr [esp + 48]
	mov	esi, dword ptr [esp + 52]
	mov	edi, dword ptr [esp + 56]
	call	.L4$pb
.L4$pb:
	pop	ebx
.Ltmp4:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp4-.L4$pb)
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], eax
	call	objc_msg_lookup@PLT
	mov	ecx, dword ptr [esp + 60]
	mov	ebp, dword ptr [edi]
	mov	dword ptr [esp + 8], edi
	mov	dword ptr [esp + 4], esi
	mov	edx, dword ptr [ecx]
	mov	dword ptr [esp + 12], ecx
	mov	ecx, dword ptr [esp + 48]
	mov	dword ptr [esp + 24], edx
	mov	dword ptr [esp], ecx
	call	eax
	mov	esi, eax
	mov	eax, dword ptr [edi]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	mov	dword ptr [esp], ebp
	call	objc_release@PLT
	mov	eax, dword ptr [esp + 60]
	mov	eax, dword ptr [eax]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	mov	eax, dword ptr [esp + 24]
	mov	dword ptr [esp], eax
	call	objc_release@PLT
	mov	eax, esi
	add	esp, 28
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end4:
	.size	two_nonnull_nonnull, .Lfunc_end4-two_nonnull_nonnull

	.section	.text.call_with_none1,"ax",@progbits
	.globl	call_with_none1
	.p2align	4, 0x90
	.type	call_with_none1,@function
call_with_none1:
	push	ebx
	push	edi
	push	esi
	mov	esi, dword ptr [esp + 16]
	mov	edi, dword ptr [esp + 20]
	call	.L5$pb
.L5$pb:
	pop	ebx
.Ltmp5:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp5-.L5$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 12
	push	0
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end5:
	.size	call_with_none1, .Lfunc_end5-call_with_none1

	.section	.text.call_with_none2,"ax",@progbits
	.globl	call_with_none2
	.p2align	4, 0x90
	.type	call_with_none2,@function
call_with_none2:
	push	ebx
	push	edi
	push	esi
	mov	esi, dword ptr [esp + 16]
	mov	edi, dword ptr [esp + 20]
	call	.L6$pb
.L6$pb:
	pop	ebx
.Ltmp6:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp6-.L6$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 12
	push	0
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end6:
	.size	call_with_none2, .Lfunc_end6-call_with_none2

	.section	.text.call_with_none3,"ax",@progbits
	.globl	call_with_none3
	.p2align	4, 0x90
	.type	call_with_none3,@function
call_with_none3:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L7$pb
.L7$pb:
	pop	ebx
	mov	dword ptr [esp + 12], 0
.Ltmp7:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp7-.L7$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 12
	lea	ecx, [esp + 16]
	push	ecx
	push	edi
	push	esi
	call	eax
	add	esp, 16
	mov	esi, eax
	sub	esp, 12
	push	dword ptr [esp + 24]
	call	objc_retain@PLT
	add	esp, 16
	mov	edx, dword ptr [esp + 12]
	mov	eax, esi
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end7:
	.size	call_with_none3, .Lfunc_end7-call_with_none3

	.section	.text.call_with_none4,"ax",@progbits
	.globl	call_with_none4
	.p2align	4, 0x90
	.type	call_with_none4,@function
call_with_none4:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L8$pb
.L8$pb:
	pop	ebx
	mov	dword ptr [esp + 12], 0
.Ltmp8:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp8-.L8$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 12
	lea	ecx, [esp + 16]
	push	ecx
	push	edi
	push	esi
	call	eax
	add	esp, 16
	mov	esi, eax
	sub	esp, 12
	push	dword ptr [esp + 24]
	call	objc_retain@PLT
	add	esp, 16
	mov	edx, dword ptr [esp + 12]
	mov	eax, esi
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end8:
	.size	call_with_none4, .Lfunc_end8-call_with_none4

	.section	.text.call_with_some1,"ax",@progbits
	.globl	call_with_some1
	.p2align	4, 0x90
	.type	call_with_some1,@function
call_with_some1:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	esi, dword ptr [esp + 32]
	mov	ebp, dword ptr [esp + 36]
	mov	edi, dword ptr [esp + 40]
	call	.L9$pb
.L9$pb:
	pop	ebx
.Ltmp9:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp9-.L9$pb)
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
	lea	ecx, [esp + 40]
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	mov	dword ptr [esp + 8], ecx
	call	eax
	mov	esi, eax
	mov	eax, dword ptr [esp + 40]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	mov	dword ptr [esp], edi
	call	objc_release@PLT
	mov	edx, dword ptr [esp + 40]
	mov	eax, esi
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end9:
	.size	call_with_some1, .Lfunc_end9-call_with_some1

	.section	.text.call_with_some2,"ax",@progbits
	.globl	call_with_some2
	.p2align	4, 0x90
	.type	call_with_some2,@function
call_with_some2:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 28
	mov	esi, dword ptr [esp + 48]
	mov	ebp, dword ptr [esp + 52]
	mov	edi, dword ptr [esp + 56]
	call	.L10$pb
.L10$pb:
	pop	ebx
.Ltmp10:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp10-.L10$pb)
	mov	dword ptr [esp + 24], edi
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
	lea	ecx, [esp + 24]
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	mov	dword ptr [esp + 8], ecx
	call	eax
	mov	esi, eax
	mov	eax, dword ptr [esp + 24]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	mov	dword ptr [esp], edi
	call	objc_release@PLT
	mov	edx, dword ptr [esp + 24]
	mov	eax, esi
	add	esp, 28
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end10:
	.size	call_with_some2, .Lfunc_end10-call_with_some2

	.section	.text.call_with_some3,"ax",@progbits
	.globl	call_with_some3
	.p2align	4, 0x90
	.type	call_with_some3,@function
call_with_some3:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 28
	mov	esi, dword ptr [esp + 48]
	mov	ebp, dword ptr [esp + 52]
	mov	edi, dword ptr [esp + 56]
	call	.L11$pb
.L11$pb:
	pop	ebx
.Ltmp11:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp11-.L11$pb)
	mov	dword ptr [esp + 24], edi
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
	lea	ecx, [esp + 24]
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	mov	dword ptr [esp + 8], ecx
	call	eax
	mov	esi, eax
	mov	eax, dword ptr [esp + 24]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	mov	dword ptr [esp], edi
	call	objc_release@PLT
	mov	edx, dword ptr [esp + 24]
	mov	eax, esi
	add	esp, 28
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end11:
	.size	call_with_some3, .Lfunc_end11-call_with_some3

	.section	".note.GNU-stack","",@progbits
