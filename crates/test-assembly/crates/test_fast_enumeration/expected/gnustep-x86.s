	.text
	.intel_syntax noprefix
	.section	.text.iter_create,"ax",@progbits
	.globl	iter_create
	.p2align	4, 0x90
	.type	iter_create,@function
iter_create:
	mov	eax, dword ptr [esp + 4]
	mov	ecx, dword ptr [esp + 8]
<<<<<<< HEAD
	xorps	xmm0, xmm0
	mov	dword ptr [eax], ecx
	movsd	qword ptr [eax + 4], xmm0
	movsd	qword ptr [eax + 12], xmm0
	movsd	qword ptr [eax + 20], xmm0
	movsd	qword ptr [eax + 28], xmm0
	movsd	qword ptr [eax + 36], xmm0
	movsd	qword ptr [eax + 44], xmm0
	movsd	qword ptr [eax + 52], xmm0
	movsd	qword ptr [eax + 60], xmm0
	movsd	qword ptr [eax + 68], xmm0
	movsd	qword ptr [eax + 76], xmm0
	movsd	qword ptr [eax + 84], xmm0
	movsd	qword ptr [eax + 92], xmm0
	movsd	qword ptr [eax + 100], xmm0
=======
	mov	dword ptr [eax + 24], 0
	mov	dword ptr [eax + 20], 0
	mov	dword ptr [eax + 32], 0
	mov	dword ptr [eax + 28], 0
	mov	dword ptr [eax + 36], 0
	mov	dword ptr [eax], 0
	mov	dword ptr [eax + 8], 0
	mov	dword ptr [eax + 12], 0
	mov	dword ptr [eax + 16], 0
	mov	dword ptr [eax + 44], 0
	mov	dword ptr [eax + 40], 0
	mov	dword ptr [eax + 52], 0
	mov	dword ptr [eax + 48], 0
	mov	dword ptr [eax + 60], 0
	mov	dword ptr [eax + 56], 0
	mov	dword ptr [eax + 68], 0
	mov	dword ptr [eax + 64], 0
	mov	dword ptr [eax + 76], 0
	mov	dword ptr [eax + 72], 0
	mov	dword ptr [eax + 84], 0
	mov	dword ptr [eax + 80], 0
	mov	dword ptr [eax + 92], 0
	mov	dword ptr [eax + 88], 0
	mov	dword ptr [eax + 100], 0
	mov	dword ptr [eax + 96], 0
	mov	dword ptr [eax + 108], 0
	mov	dword ptr [eax + 104], 0
	mov	dword ptr [eax + 112], ecx
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	ret	4
.Lfunc_end0:
	.size	iter_create, .Lfunc_end0-iter_create

	.section	.text.iter_once,"ax",@progbits
	.globl	iter_once
	.p2align	4, 0x90
	.type	iter_once,@function
iter_once:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	edi, dword ptr [esp + 32]
	call	.L1$pb
.L1$pb:
	pop	ebx
.Ltmp0:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp0-.L1$pb)
<<<<<<< HEAD
	mov	eax, dword ptr [edi + 100]
	cmp	eax, dword ptr [edi + 104]
	jb	.LBB1_4
	lea	eax, [edi + 4]
	mov	ebp, dword ptr [edi]
=======
	mov	eax, dword ptr [edi + 104]
	cmp	eax, dword ptr [edi + 108]
	jb	.LBB1_6
	lea	eax, [edi + 40]
	mov	ebp, dword ptr [edi + 112]
	mov	dword ptr [esp + 4], eax
	lea	eax, [edi + 8]
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	mov	dword ptr [esp + 8], eax
	mov	eax, dword ptr [ebx + SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOT]
	mov	esi, dword ptr [eax]
	test	esi, esi
	je	.LBB1_2
.LBB1_3:
	lea	eax, [edi + 68]
	mov	dword ptr [esp + 4], eax
	sub	esp, 8
	push	esi
	push	ebp
	call	objc_msg_lookup@PLT
	add	esp, 4
	push	16
	push	dword ptr [esp + 20]
	push	dword ptr [esp + 28]
	push	esi
	push	ebp
	call	eax
	add	esp, 32
	test	eax, eax
	mov	dword ptr [edi + 108], eax
	mov	dword ptr [edi + 104], 0
	je	.LBB1_4
	xor	eax, eax
	cmp	dword ptr [edi + 12], 0
	je	.LBB1_13
.LBB1_6:
	mov	ecx, dword ptr [edi + 16]
	test	ecx, ecx
	je	.LBB1_7
	mov	ecx, dword ptr [ecx]
	cmp	dword ptr [edi], 0
	je	.LBB1_9
	cmp	dword ptr [edi + 4], ecx
	je	.LBB1_7
	call	SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)@PLT
.LBB1_9:
	mov	dword ptr [edi], 1
	mov	dword ptr [edi + 4], ecx
.LBB1_7:
	mov	ecx, dword ptr [edi + 12]
	lea	edx, [eax + 1]
	mov	dword ptr [edi + 104], edx
	mov	eax, dword ptr [ecx + 4*eax]
.LBB1_8:
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB1_4:
	xor	eax, eax
	jmp	.LBB1_8
.LBB1_2:
	sub	esp, 8
	lea	ecx, [ebx + .Lanon.[ID].0@GOTOFF]
	push	ecx
	push	eax
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)@PLT
	add	esp, 16
	mov	esi, eax
	jmp	.LBB1_3
.LBB1_13:
	call	SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)@PLT
.Lfunc_end1:
	.size	iter_once, .Lfunc_end1-iter_once

	.section	.text.use_obj,"ax",@progbits
	.globl	use_obj
	.p2align	4, 0x90
	.type	use_obj,@function
use_obj:
	push	eax
	mov	eax, dword ptr [esp + 8]
	mov	dword ptr [esp], eax
	mov	eax, esp
	#APP
	#NO_APP
	pop	eax
	ret
.Lfunc_end2:
	.size	use_obj, .Lfunc_end2-use_obj

	.section	.text.iter,"ax",@progbits
	.globl	iter
	.p2align	4, 0x90
	.type	iter,@function
iter:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 124
	call	.L3$pb
.L3$pb:
	pop	ebx
	mov	ebp, dword ptr [esp + 144]
<<<<<<< HEAD
	xorps	xmm0, xmm0
	xor	eax, eax
	mov	dword ptr [esp + 112], 0
.Ltmp1:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp1-.L3$pb)
	movsd	qword ptr [esp + 104], xmm0
	movsd	qword ptr [esp + 96], xmm0
	mov	edi, dword ptr [ebx + SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOT]
	lea	ecx, [ebx + .Lanon.[ID].0@GOTOFF]
	mov	dword ptr [esp + 12], ecx
	xor	ecx, ecx
	mov	dword ptr [esp + 16], ebp
	movsd	qword ptr [esp + 20], xmm0
	movsd	qword ptr [esp + 28], xmm0
	movsd	qword ptr [esp + 36], xmm0
	movsd	qword ptr [esp + 44], xmm0
	movsd	qword ptr [esp + 52], xmm0
	movsd	qword ptr [esp + 60], xmm0
	movsd	qword ptr [esp + 68], xmm0
	movsd	qword ptr [esp + 76], xmm0
	movsd	qword ptr [esp + 84], xmm0
=======
	xor	ecx, ecx
	xor	eax, eax
	mov	dword ptr [esp + 32], 0
	mov	dword ptr [esp + 28], 0
	mov	dword ptr [esp + 40], 0
	mov	dword ptr [esp + 36], 0
	mov	dword ptr [esp + 44], 0
	mov	dword ptr [esp + 8], 0
	mov	dword ptr [esp + 16], 0
	mov	dword ptr [esp + 20], 0
	mov	dword ptr [esp + 24], 0
	mov	dword ptr [esp + 52], 0
	mov	dword ptr [esp + 48], 0
	mov	dword ptr [esp + 60], 0
	mov	dword ptr [esp + 56], 0
	mov	dword ptr [esp + 68], 0
	mov	dword ptr [esp + 64], 0
	mov	dword ptr [esp + 76], 0
	mov	dword ptr [esp + 72], 0
	mov	dword ptr [esp + 84], 0
	mov	dword ptr [esp + 80], 0
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	mov	dword ptr [esp + 92], 0
	mov	dword ptr [esp + 88], 0
	mov	dword ptr [esp + 100], 0
	mov	dword ptr [esp + 96], 0
	mov	dword ptr [esp + 108], 0
	mov	dword ptr [esp + 104], 0
	mov	dword ptr [esp + 116], 0
	mov	dword ptr [esp + 112], 0
.Ltmp1:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp1-.L3$pb)
	mov	edi, dword ptr [ebx + SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOT]
	mov	dword ptr [esp + 120], ebp
	jmp	.LBB3_1
	.p2align	4, 0x90
.LBB3_8:
	mov	dword ptr [esp + 8], 1
	mov	dword ptr [esp + 12], ecx
.LBB3_13:
	mov	ecx, dword ptr [esp + 20]
	lea	edx, [eax + 1]
	mov	dword ptr [esp + 112], edx
	sub	esp, 12
	push	dword ptr [ecx + 4*eax]
	call	use_obj@PLT
	add	esp, 16
	mov	ebp, dword ptr [esp + 120]
	mov	eax, dword ptr [esp + 112]
	mov	ecx, dword ptr [esp + 116]
.LBB3_1:
	cmp	eax, ecx
	jb	.LBB3_12
	mov	esi, dword ptr [edi]
	test	esi, esi
	je	.LBB3_3
.LBB3_4:
	sub	esp, 8
	push	esi
	push	ebp
	call	objc_msg_lookup@PLT
	add	esp, 4
	push	16
	lea	ecx, [esp + 64]
	push	ecx
	lea	ecx, [esp + 36]
	push	ecx
	push	esi
	push	ebp
	call	eax
	add	esp, 32
	test	eax, eax
<<<<<<< HEAD
	mov	dword ptr [esp + 120], eax
	je	.LBB3_5
.LBB3_6:
	mov	eax, dword ptr [esp + 88]
	lea	edx, [ecx + 1]
	mov	dword ptr [esp + 116], edx
	sub	esp, 12
	push	dword ptr [eax + 4*ecx]
	call	use_obj@PLT
	add	esp, 16
	mov	ebp, dword ptr [esp + 16]
	mov	ecx, dword ptr [esp + 116]
	mov	eax, dword ptr [esp + 120]
.LBB3_1:
	cmp	ecx, eax
	jb	.LBB3_6
	mov	esi, dword ptr [edi]
	test	esi, esi
	jne	.LBB3_4
	sub	esp, 8
	push	dword ptr [esp + 20]
=======
	mov	dword ptr [esp + 116], eax
	mov	dword ptr [esp + 112], 0
	je	.LBB3_11
	xor	eax, eax
	cmp	dword ptr [esp + 20], 0
	je	.LBB3_6
.LBB3_12:
	mov	ecx, dword ptr [esp + 24]
	test	ecx, ecx
	je	.LBB3_13
	mov	ecx, dword ptr [ecx]
	cmp	dword ptr [esp + 8], 0
	je	.LBB3_8
	cmp	dword ptr [esp + 12], ecx
	je	.LBB3_13
	jmp	.LBB3_10
.LBB3_3:
	sub	esp, 8
	lea	eax, [ebx + .Lanon.[ID].0@GOTOFF]
	push	eax
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	push	edi
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)@PLT
	add	esp, 16
	mov	esi, eax
	jmp	.LBB3_4
.LBB3_11:
	add	esp, 124
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB3_6:
	call	SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)@PLT
.LBB3_10:
	call	SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)@PLT
.Lfunc_end3:
	.size	iter, .Lfunc_end3-iter

	.section	.text.iter_noop,"ax",@progbits
	.globl	iter_noop
	.p2align	4, 0x90
	.type	iter_noop,@function
iter_noop:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 124
	mov	edi, dword ptr [esp + 144]
	call	.L4$pb
.L4$pb:
	pop	ebx
<<<<<<< HEAD
	mov	edi, dword ptr [esp + 144]
	xorps	xmm0, xmm0
	xor	eax, eax
	mov	dword ptr [esp + 112], 0
.Ltmp2:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp2-.L4$pb)
	movsd	qword ptr [esp + 104], xmm0
	movsd	qword ptr [esp + 96], xmm0
	mov	ebp, dword ptr [ebx + SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOT]
	lea	ecx, [ebx + .Lanon.[ID].0@GOTOFF]
	mov	dword ptr [esp + 12], ecx
	xor	ecx, ecx
	mov	dword ptr [esp + 16], edi
	movsd	qword ptr [esp + 20], xmm0
	movsd	qword ptr [esp + 28], xmm0
	movsd	qword ptr [esp + 36], xmm0
	movsd	qword ptr [esp + 44], xmm0
	movsd	qword ptr [esp + 52], xmm0
	movsd	qword ptr [esp + 60], xmm0
	movsd	qword ptr [esp + 68], xmm0
	movsd	qword ptr [esp + 76], xmm0
	movsd	qword ptr [esp + 84], xmm0
=======
	lea	ebp, [esp + 48]
	xor	ecx, ecx
	xor	eax, eax
	xor	edx, edx
	mov	dword ptr [esp + 32], 0
	mov	dword ptr [esp + 28], 0
	mov	dword ptr [esp + 40], 0
	mov	dword ptr [esp + 36], 0
	mov	dword ptr [esp + 44], 0
	mov	dword ptr [esp + 8], 0
	mov	dword ptr [esp + 16], 0
	mov	dword ptr [esp + 20], 0
	mov	dword ptr [esp + 24], 0
	mov	dword ptr [esp + 52], 0
	mov	dword ptr [esp + 48], 0
	mov	dword ptr [esp + 60], 0
	mov	dword ptr [esp + 56], 0
	mov	dword ptr [esp + 68], 0
	mov	dword ptr [esp + 64], 0
	mov	dword ptr [esp + 76], 0
	mov	dword ptr [esp + 72], 0
	mov	dword ptr [esp + 84], 0
	mov	dword ptr [esp + 80], 0
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	mov	dword ptr [esp + 92], 0
	mov	dword ptr [esp + 88], 0
	mov	dword ptr [esp + 100], 0
	mov	dword ptr [esp + 96], 0
	mov	dword ptr [esp + 108], 0
	mov	dword ptr [esp + 104], 0
	mov	dword ptr [esp + 116], 0
	mov	dword ptr [esp + 112], 0
.Ltmp2:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp2-.L4$pb)
	mov	dword ptr [esp + 120], edi
	jmp	.LBB4_1
	.p2align	4, 0x90
.LBB4_9:
	mov	dword ptr [esp + 8], 1
	mov	dword ptr [esp + 12], esi
.LBB4_11:
	inc	edx
	mov	dword ptr [esp + 112], edx
	mov	edi, dword ptr [esp + 120]
.LBB4_1:
	cmp	edx, eax
	jb	.LBB4_7
	mov	eax, dword ptr [ebx + SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOT]
	mov	esi, dword ptr [eax]
	test	esi, esi
	je	.LBB4_3
.LBB4_4:
	sub	esp, 8
	push	esi
	push	edi
	call	objc_msg_lookup@PLT
	add	esp, 4
	push	16
	push	ebp
	lea	ecx, [esp + 36]
	push	ecx
	push	esi
	push	edi
	call	eax
	add	esp, 32
	test	eax, eax
	mov	dword ptr [esp + 116], eax
	mov	dword ptr [esp + 112], 0
	je	.LBB4_12
	cmp	dword ptr [esp + 20], 0
	je	.LBB4_13
	mov	ecx, dword ptr [esp + 24]
	xor	edx, edx
.LBB4_7:
	test	ecx, ecx
	je	.LBB4_11
	mov	esi, dword ptr [ecx]
	cmp	dword ptr [esp + 8], 0
	je	.LBB4_9
	cmp	dword ptr [esp + 12], esi
	je	.LBB4_11
	jmp	.LBB4_14
.LBB4_3:
	sub	esp, 8
	lea	ecx, [ebx + .Lanon.[ID].0@GOTOFF]
	push	ecx
	push	eax
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)@PLT
	add	esp, 16
	mov	esi, eax
	jmp	.LBB4_4
.LBB4_12:
	add	esp, 124
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB4_13:
	call	SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)@PLT
.LBB4_14:
	call	SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)@PLT
.Lfunc_end4:
	.size	iter_noop, .Lfunc_end4-iter_noop

	.section	.text.iter_retained,"ax",@progbits
	.globl	iter_retained
	.p2align	4, 0x90
	.type	iter_retained,@function
iter_retained:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 124
	mov	edi, dword ptr [esp + 144]
	call	.L5$pb
.L5$pb:
	pop	ebx
<<<<<<< HEAD
	mov	ebp, dword ptr [esp + 144]
	xorps	xmm0, xmm0
	xor	eax, eax
	mov	dword ptr [esp + 112], 0
.Ltmp3:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp3-.L5$pb)
	movsd	qword ptr [esp + 104], xmm0
	movsd	qword ptr [esp + 96], xmm0
	mov	edi, dword ptr [ebx + SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOT]
	lea	ecx, [ebx + .Lanon.[ID].0@GOTOFF]
	mov	dword ptr [esp + 12], ecx
	xor	ecx, ecx
	mov	dword ptr [esp + 16], ebp
	movsd	qword ptr [esp + 20], xmm0
	movsd	qword ptr [esp + 28], xmm0
	movsd	qword ptr [esp + 36], xmm0
	movsd	qword ptr [esp + 44], xmm0
	movsd	qword ptr [esp + 52], xmm0
	movsd	qword ptr [esp + 60], xmm0
	movsd	qword ptr [esp + 68], xmm0
	movsd	qword ptr [esp + 76], xmm0
	movsd	qword ptr [esp + 84], xmm0
=======
	lea	ebp, [esp + 48]
	xor	ecx, ecx
	xor	eax, eax
	mov	dword ptr [esp + 32], 0
	mov	dword ptr [esp + 28], 0
	mov	dword ptr [esp + 40], 0
	mov	dword ptr [esp + 36], 0
	mov	dword ptr [esp + 44], 0
	mov	dword ptr [esp + 8], 0
	mov	dword ptr [esp + 16], 0
	mov	dword ptr [esp + 20], 0
	mov	dword ptr [esp + 24], 0
	mov	dword ptr [esp + 52], 0
	mov	dword ptr [esp + 48], 0
	mov	dword ptr [esp + 60], 0
	mov	dword ptr [esp + 56], 0
	mov	dword ptr [esp + 68], 0
	mov	dword ptr [esp + 64], 0
	mov	dword ptr [esp + 76], 0
	mov	dword ptr [esp + 72], 0
	mov	dword ptr [esp + 84], 0
	mov	dword ptr [esp + 80], 0
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	mov	dword ptr [esp + 92], 0
	mov	dword ptr [esp + 88], 0
	mov	dword ptr [esp + 100], 0
	mov	dword ptr [esp + 96], 0
	mov	dword ptr [esp + 108], 0
	mov	dword ptr [esp + 104], 0
	mov	dword ptr [esp + 116], 0
	mov	dword ptr [esp + 112], 0
.Ltmp3:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp3-.L5$pb)
	mov	dword ptr [esp + 120], edi
	jmp	.LBB5_1
	.p2align	4, 0x90
.LBB5_8:
	mov	dword ptr [esp + 8], 1
	mov	dword ptr [esp + 12], ecx
.LBB5_13:
	mov	ecx, dword ptr [esp + 20]
	lea	edx, [eax + 1]
	mov	dword ptr [esp + 112], edx
	sub	esp, 12
	push	dword ptr [ecx + 4*eax]
	call	objc_retain@PLT
	add	esp, 16
	mov	esi, eax
	sub	esp, 12
	push	eax
	call	use_obj@PLT
	add	esp, 4
	push	esi
	call	objc_release@PLT
	add	esp, 16
	mov	edi, dword ptr [esp + 120]
	mov	eax, dword ptr [esp + 112]
	mov	ecx, dword ptr [esp + 116]
.LBB5_1:
	cmp	eax, ecx
	jb	.LBB5_12
	mov	eax, dword ptr [ebx + SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOT]
	mov	esi, dword ptr [eax]
	test	esi, esi
	je	.LBB5_3
.LBB5_4:
	sub	esp, 8
	push	esi
	push	edi
	call	objc_msg_lookup@PLT
	add	esp, 4
	push	16
	push	ebp
	lea	ecx, [esp + 36]
	push	ecx
	push	esi
	push	edi
	call	eax
	add	esp, 32
	test	eax, eax
	mov	dword ptr [esp + 116], eax
	mov	dword ptr [esp + 112], 0
	je	.LBB5_11
	xor	eax, eax
	cmp	dword ptr [esp + 20], 0
	je	.LBB5_6
.LBB5_12:
	mov	ecx, dword ptr [esp + 24]
	test	ecx, ecx
	je	.LBB5_13
	mov	ecx, dword ptr [ecx]
	cmp	dword ptr [esp + 8], 0
	je	.LBB5_8
	cmp	dword ptr [esp + 12], ecx
	je	.LBB5_13
	jmp	.LBB5_10
.LBB5_3:
	sub	esp, 8
	lea	ecx, [ebx + .Lanon.[ID].0@GOTOFF]
	push	ecx
	push	eax
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)@PLT
	add	esp, 16
	mov	esi, eax
	jmp	.LBB5_4
.LBB5_11:
	add	esp, 124
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB5_6:
	call	SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)@PLT
.LBB5_10:
	call	SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)@PLT
.Lfunc_end5:
	.size	iter_retained, .Lfunc_end5-iter_retained

	.type	.Lanon.[ID].0,@object
	.section	.rodata..Lanon.[ID].0,"a",@progbits
.Lanon.[ID].0:
	.asciz	"countByEnumeratingWithState:objects:count:"
	.size	.Lanon.[ID].0, 43

	.section	".note.GNU-stack","",@progbits
