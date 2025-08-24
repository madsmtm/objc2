	.intel_syntax noprefix
	.section	.text.fn1_error_bool,"ax",@progbits
	.globl	fn1_error_bool
	.p2align	4
	.type	fn1_error_bool,@function
fn1_error_bool:
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
	.size	fn1_error_bool, .Lfunc_end0-fn1_error_bool

	.section	.text.fn2_error_new,"ax",@progbits
	.globl	fn2_error_new
	.p2align	4
	.type	fn2_error_new,@function
fn2_error_new:
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
.LBB1_4:
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB1_2:
	sub	esp, 12
	push	dword ptr [esp + 24]
	call	objc_retain@PLT
	add	esp, 16
	mov	edx, eax
	mov	eax, 1
	test	edx, edx
	jne	.LBB1_4
	call	SYM(objc2::__macro_helpers::null_error::null_error::GENERATED_ID, 0)@PLT
	mov	edx, eax
	mov	eax, 1
	jmp	.LBB1_4
.Lfunc_end1:
	.size	fn2_error_new, .Lfunc_end1-fn2_error_new

	.section	.text.fn3_error_init,"ax",@progbits
	.globl	fn3_error_init
	.p2align	4
	.type	fn3_error_init,@function
fn3_error_init:
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
.LBB2_4:
	mov	eax, dword ptr [esp + 12]
	jmp	.LBB2_5
.Lfunc_end2:
	.size	fn3_error_init, .Lfunc_end2-fn3_error_init

	.section	.text.fn4_error_copy,"ax",@progbits
	.globl	fn4_error_copy
	.p2align	4
	.type	fn4_error_copy,@function
fn4_error_copy:
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
.LBB3_4:
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB3_2:
	sub	esp, 12
	push	dword ptr [esp + 24]
	call	objc_retain@PLT
	add	esp, 16
	mov	edx, eax
	mov	eax, 1
	test	edx, edx
	jne	.LBB3_4
	call	SYM(objc2::__macro_helpers::null_error::null_error::GENERATED_ID, 0)@PLT
	mov	edx, eax
	mov	eax, 1
	jmp	.LBB3_4
.Lfunc_end3:
	.size	fn4_error_copy, .Lfunc_end3-fn4_error_copy

	.section	.text.fn5_error_mutable_copy,"ax",@progbits
	.globl	fn5_error_mutable_copy
	.p2align	4
	.type	fn5_error_mutable_copy,@function
fn5_error_mutable_copy:
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
.LBB4_4:
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB4_2:
	sub	esp, 12
	push	dword ptr [esp + 24]
	call	objc_retain@PLT
	add	esp, 16
	mov	edx, eax
	mov	eax, 1
	test	edx, edx
	jne	.LBB4_4
	call	SYM(objc2::__macro_helpers::null_error::null_error::GENERATED_ID, 0)@PLT
	mov	edx, eax
	mov	eax, 1
	jmp	.LBB4_4
.Lfunc_end4:
	.size	fn5_error_mutable_copy, .Lfunc_end4-fn5_error_mutable_copy

	.section	.text.fn6_error_autoreleased,"ax",@progbits
	.globl	fn6_error_autoreleased
	.p2align	4
	.type	fn6_error_autoreleased,@function
fn6_error_autoreleased:
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
.LBB5_4:
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB5_2:
	mov	eax, dword ptr [esp + 12]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	mov	edx, eax
	mov	eax, 1
	test	edx, edx
	jne	.LBB5_4
	call	SYM(objc2::__macro_helpers::null_error::null_error::GENERATED_ID, 0)@PLT
	mov	edx, eax
	mov	eax, 1
	jmp	.LBB5_4
.Lfunc_end5:
	.size	fn6_error_autoreleased, .Lfunc_end5-fn6_error_autoreleased

	.section	".note.GNU-stack","",@progbits
