	.text
	.intel_syntax noprefix
	.section	.text.error_bool,"ax",@progbits
	.globl	error_bool
	.p2align	4, 0x90
	.type	error_bool,@function
error_bool:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	edi, dword ptr [esp + 40]
	mov	esi, dword ptr [esp + 32]
	mov	ebp, dword ptr [esp + 36]
	call	.L0$pb
.L0$pb:
	pop	ebx
	mov	dword ptr [esp + 8], 0
.Ltmp0:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp0-.L0$pb)
	sub	esp, 8
	push	ebp
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 16
	lea	ecx, [esp + 8]
	push	ecx
	push	edi
	push	ebp
	push	esi
	call	eax
	add	esp, 16
	mov	ecx, eax
	xor	eax, eax
	test	cl, cl
	je	.LBB0_1
.LBB0_3:
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB0_1:
	sub	esp, 12
	push	dword ptr [esp + 20]
	call	objc_retain@PLT
	add	esp, 16
	test	eax, eax
	jne	.LBB0_3
	call	SYM(objc2::__macro_helpers::null_error::null_error::GENERATED_ID, 0)@PLT
	jmp	.LBB0_3
.Lfunc_end0:
	.size	error_bool, .Lfunc_end0-error_bool

	.section	.text.error_new,"ax",@progbits
	.globl	error_new
	.p2align	4, 0x90
	.type	error_new,@function
error_new:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L1$pb
.L1$pb:
	pop	ebx
	mov	dword ptr [esp + 12], 0
.Ltmp1:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp1-.L1$pb)
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
	test	eax, eax
	je	.LBB1_2
	mov	edx, eax
	xor	eax, eax
	jmp	.LBB1_4
.LBB1_2:
	sub	esp, 12
	push	dword ptr [esp + 24]
	call	objc_retain@PLT
	add	esp, 16
	mov	edx, eax
	mov	eax, 1
	test	edx, edx
	je	.LBB1_3
.LBB1_4:
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB1_3:
	call	SYM(objc2::__macro_helpers::null_error::null_error::GENERATED_ID, 0)@PLT
	mov	edx, eax
	mov	eax, 1
	jmp	.LBB1_4
.Lfunc_end1:
	.size	error_new, .Lfunc_end1-error_new

	.section	.text.error_init,"ax",@progbits
	.globl	error_init
	.p2align	4, 0x90
	.type	error_init,@function
error_init:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	call	.L2$pb
.L2$pb:
	pop	ebx
	mov	dword ptr [esp + 12], 0
.Ltmp2:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp2-.L2$pb)
	test	esi, esi
	je	.LBB2_1
	mov	edi, dword ptr [esp + 36]
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
	test	eax, eax
	je	.LBB2_4
	mov	edx, eax
	xor	eax, eax
	jmp	.LBB2_7
.LBB2_1:
	xor	eax, eax
	jmp	.LBB2_5
.LBB2_4:
	mov	eax, dword ptr [esp + 12]
.LBB2_5:
	sub	esp, 12
	push	eax
	call	objc_retain@PLT
	add	esp, 16
	mov	edx, eax
	mov	eax, 1
	test	edx, edx
	je	.LBB2_6
.LBB2_7:
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB2_6:
	call	SYM(objc2::__macro_helpers::null_error::null_error::GENERATED_ID, 0)@PLT
	mov	edx, eax
	mov	eax, 1
	jmp	.LBB2_7
.Lfunc_end2:
	.size	error_init, .Lfunc_end2-error_init

	.section	.text.error_copy,"ax",@progbits
	.globl	error_copy
	.p2align	4, 0x90
	.type	error_copy,@function
error_copy:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L3$pb
.L3$pb:
	pop	ebx
	mov	dword ptr [esp + 12], 0
.Ltmp3:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp3-.L3$pb)
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
	test	eax, eax
	je	.LBB3_2
	mov	edx, eax
	xor	eax, eax
	jmp	.LBB3_4
.LBB3_2:
	sub	esp, 12
	push	dword ptr [esp + 24]
	call	objc_retain@PLT
	add	esp, 16
	mov	edx, eax
	mov	eax, 1
	test	edx, edx
	je	.LBB3_3
.LBB3_4:
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB3_3:
	call	SYM(objc2::__macro_helpers::null_error::null_error::GENERATED_ID, 0)@PLT
	mov	edx, eax
	mov	eax, 1
	jmp	.LBB3_4
.Lfunc_end3:
	.size	error_copy, .Lfunc_end3-error_copy

	.section	.text.error_mutable_copy,"ax",@progbits
	.globl	error_mutable_copy
	.p2align	4, 0x90
	.type	error_mutable_copy,@function
error_mutable_copy:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L4$pb
.L4$pb:
	pop	ebx
	mov	dword ptr [esp + 12], 0
.Ltmp4:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp4-.L4$pb)
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
	test	eax, eax
	je	.LBB4_2
	mov	edx, eax
	xor	eax, eax
	jmp	.LBB4_4
.LBB4_2:
	sub	esp, 12
	push	dword ptr [esp + 24]
	call	objc_retain@PLT
	add	esp, 16
	mov	edx, eax
	mov	eax, 1
	test	edx, edx
	je	.LBB4_3
.LBB4_4:
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB4_3:
	call	SYM(objc2::__macro_helpers::null_error::null_error::GENERATED_ID, 0)@PLT
	mov	edx, eax
	mov	eax, 1
	jmp	.LBB4_4
.Lfunc_end4:
	.size	error_mutable_copy, .Lfunc_end4-error_mutable_copy

	.section	.text.error_autoreleased,"ax",@progbits
	.globl	error_autoreleased
	.p2align	4, 0x90
	.type	error_autoreleased,@function
error_autoreleased:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L5$pb
.L5$pb:
	pop	ebx
	mov	dword ptr [esp + 12], 0
.Ltmp5:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp5-.L5$pb)
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
	lea	ecx, [esp + 12]
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	mov	dword ptr [esp + 8], ecx
	call	eax
	mov	dword ptr [esp], eax
	call	objc_retainAutoreleasedReturnValue@PLT
	test	eax, eax
	je	.LBB5_2
	mov	edx, eax
	xor	eax, eax
	jmp	.LBB5_4
.LBB5_2:
	mov	eax, dword ptr [esp + 12]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	mov	edx, eax
	mov	eax, 1
	test	edx, edx
	je	.LBB5_3
.LBB5_4:
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB5_3:
	call	SYM(objc2::__macro_helpers::null_error::null_error::GENERATED_ID, 0)@PLT
	mov	edx, eax
	mov	eax, 1
	jmp	.LBB5_4
.Lfunc_end5:
	.size	error_autoreleased, .Lfunc_end5-error_autoreleased

	.section	".note.GNU-stack","",@progbits
