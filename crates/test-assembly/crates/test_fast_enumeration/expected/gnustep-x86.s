	.intel_syntax noprefix
	.section	.text.fn1_iter_create,"ax",@progbits
	.globl	fn1_iter_create
	.p2align	4
	.type	fn1_iter_create,@function
fn1_iter_create:
	mov	eax, dword ptr [esp + 4]
	mov	ecx, dword ptr [esp + 8]
	xorps	xmm0, xmm0
	movsd	qword ptr [eax + 88], xmm0
	movsd	qword ptr [eax + 80], xmm0
	mov	dword ptr [eax + 96], 0
	movsd	qword ptr [eax + 4], xmm0
	movsd	qword ptr [eax + 12], xmm0
	movsd	qword ptr [eax + 20], xmm0
	movsd	qword ptr [eax + 28], xmm0
	movsd	qword ptr [eax + 36], xmm0
	movsd	qword ptr [eax + 44], xmm0
	movsd	qword ptr [eax + 52], xmm0
	movsd	qword ptr [eax + 60], xmm0
	mov	dword ptr [eax], ecx
	mov	dword ptr [eax + 68], 0
	mov	dword ptr [eax + 72], 0
	mov	dword ptr [eax + 76], 0
	mov	dword ptr [eax + 100], 0
	mov	dword ptr [eax + 104], 0
	ret	4
.Lfunc_end0:
	.size	fn1_iter_create, .Lfunc_end0-fn1_iter_create

	.section	.text.fn2_iter_once,"ax",@progbits
	.globl	fn2_iter_once
	.p2align	4
	.type	fn2_iter_once,@function
fn2_iter_once:
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
	mov	eax, dword ptr [edi + 100]
	cmp	eax, dword ptr [edi + 104]
	jb	.LBB1_4
	lea	eax, [edi + 4]
	mov	ebp, dword ptr [edi]
	mov	dword ptr [esp + 8], eax
	lea	eax, [edi + 68]
	mov	dword ptr [esp + 4], eax
	mov	eax, dword ptr [ebx + SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOT]
	mov	esi, dword ptr [eax]
	test	esi, esi
	je	.LBB1_2
.LBB1_3:
	sub	esp, 8
	push	esi
	push	ebp
	call	objc_msg_lookup@PLT
	add	esp, 4
	push	16
	push	dword ptr [esp + 24]
	push	dword ptr [esp + 24]
	push	esi
	push	ebp
	call	eax
	add	esp, 32
	mov	ecx, eax
	mov	dword ptr [edi + 104], eax
	xor	eax, eax
	mov	dword ptr [edi + 100], 0
	test	ecx, ecx
	je	.LBB1_5
.LBB1_4:
	mov	ecx, dword ptr [edi + 72]
	lea	edx, [eax + 1]
	mov	dword ptr [edi + 100], edx
	mov	eax, dword ptr [ecx + 4*eax]
.LBB1_5:
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB1_2:
	sub	esp, 8
	lea	ecx, [ebx + .Lanon.[ID].0@GOTOFF]
	push	ecx
	push	eax
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)@PLT
	add	esp, 16
	mov	esi, eax
	jmp	.LBB1_3
.Lfunc_end1:
	.size	fn2_iter_once, .Lfunc_end1-fn2_iter_once

	.section	.text.fn3_use_obj,"ax",@progbits
	.globl	fn3_use_obj
	.p2align	4
	.type	fn3_use_obj,@function
fn3_use_obj:
	push	eax
	mov	eax, dword ptr [esp + 8]
	mov	dword ptr [esp], eax
	mov	eax, esp
	#APP
	#NO_APP
	pop	eax
	ret
.Lfunc_end2:
	.size	fn3_use_obj, .Lfunc_end2-fn3_use_obj

	.section	.text.fn4_iter,"ax",@progbits
	.globl	fn4_iter
	.p2align	4
	.type	fn4_iter,@function
fn4_iter:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 124
	call	.L3$pb
.L3$pb:
	pop	ebx
	mov	ebp, dword ptr [esp + 144]
	xorps	xmm0, xmm0
	xor	eax, eax
	mov	dword ptr [esp + 112], 0
.Ltmp1:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp1-.L3$pb)
	movsd	qword ptr [esp + 104], xmm0
	movsd	qword ptr [esp + 96], xmm0
	movsd	qword ptr [esp + 20], xmm0
	movsd	qword ptr [esp + 28], xmm0
	movsd	qword ptr [esp + 36], xmm0
	movsd	qword ptr [esp + 44], xmm0
	movsd	qword ptr [esp + 52], xmm0
	movsd	qword ptr [esp + 60], xmm0
	movsd	qword ptr [esp + 68], xmm0
	movsd	qword ptr [esp + 76], xmm0
	mov	edi, dword ptr [ebx + SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOT]
	lea	ecx, [ebx + .Lanon.[ID].0@GOTOFF]
	mov	dword ptr [esp + 12], ecx
	xor	ecx, ecx
	mov	dword ptr [esp + 16], ebp
	mov	dword ptr [esp + 84], 0
	mov	dword ptr [esp + 88], 0
	mov	dword ptr [esp + 92], 0
	mov	dword ptr [esp + 116], 0
	mov	dword ptr [esp + 120], 0
	jmp	.LBB3_1
	.p2align	4
.LBB3_5:
	mov	eax, dword ptr [esp + 88]
	lea	edx, [ecx + 1]
	mov	dword ptr [esp + 116], edx
	sub	esp, 12
	push	dword ptr [eax + 4*ecx]
	call	fn3_use_obj@PLT
	add	esp, 16
	mov	ebp, dword ptr [esp + 16]
	mov	ecx, dword ptr [esp + 116]
	mov	eax, dword ptr [esp + 120]
.LBB3_1:
	cmp	ecx, eax
	jb	.LBB3_5
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
	lea	ecx, [esp + 36]
	push	ecx
	lea	ecx, [esp + 104]
	push	ecx
	push	esi
	push	ebp
	call	eax
	add	esp, 32
	xor	ecx, ecx
	test	eax, eax
	mov	dword ptr [esp + 120], eax
	jne	.LBB3_5
	jmp	.LBB3_6
.LBB3_3:
	sub	esp, 8
	push	dword ptr [esp + 20]
	push	edi
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)@PLT
	add	esp, 16
	mov	esi, eax
	jmp	.LBB3_4
.LBB3_6:
	add	esp, 124
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end3:
	.size	fn4_iter, .Lfunc_end3-fn4_iter

	.section	.text.fn5_iter_noop,"ax",@progbits
	.globl	fn5_iter_noop
	.p2align	4
	.type	fn5_iter_noop,@function
fn5_iter_noop:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 124
	call	.L4$pb
.L4$pb:
	pop	ebx
	mov	ebp, dword ptr [esp + 144]
	xorps	xmm0, xmm0
	xor	eax, eax
	mov	dword ptr [esp + 112], 0
.Ltmp2:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp2-.L4$pb)
	movsd	qword ptr [esp + 104], xmm0
	movsd	qword ptr [esp + 96], xmm0
	movsd	qword ptr [esp + 20], xmm0
	movsd	qword ptr [esp + 28], xmm0
	movsd	qword ptr [esp + 36], xmm0
	movsd	qword ptr [esp + 44], xmm0
	movsd	qword ptr [esp + 52], xmm0
	movsd	qword ptr [esp + 60], xmm0
	movsd	qword ptr [esp + 68], xmm0
	movsd	qword ptr [esp + 76], xmm0
	mov	edi, dword ptr [ebx + SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOT]
	lea	ecx, [ebx + .Lanon.[ID].0@GOTOFF]
	mov	dword ptr [esp + 12], ecx
	xor	ecx, ecx
	mov	dword ptr [esp + 16], ebp
	mov	dword ptr [esp + 84], 0
	mov	dword ptr [esp + 88], 0
	mov	dword ptr [esp + 92], 0
	mov	dword ptr [esp + 116], 0
	mov	dword ptr [esp + 120], 0
	jmp	.LBB4_1
	.p2align	4
.LBB4_6:
	inc	ecx
	mov	dword ptr [esp + 116], ecx
.LBB4_1:
	cmp	ecx, eax
	jb	.LBB4_6
	mov	esi, dword ptr [edi]
	test	esi, esi
	je	.LBB4_3
.LBB4_4:
	sub	esp, 8
	push	esi
	push	ebp
	call	objc_msg_lookup@PLT
	add	esp, 4
	push	16
	lea	ecx, [esp + 36]
	push	ecx
	lea	ecx, [esp + 104]
	push	ecx
	push	esi
	push	ebp
	call	eax
	add	esp, 32
	test	eax, eax
	mov	dword ptr [esp + 120], eax
	je	.LBB4_7
	mov	ebp, dword ptr [esp + 16]
	xor	ecx, ecx
	jmp	.LBB4_6
.LBB4_3:
	sub	esp, 8
	push	dword ptr [esp + 20]
	push	edi
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)@PLT
	add	esp, 16
	mov	esi, eax
	jmp	.LBB4_4
.LBB4_7:
	add	esp, 124
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end4:
	.size	fn5_iter_noop, .Lfunc_end4-fn5_iter_noop

	.section	.text.fn6_iter_retained,"ax",@progbits
	.globl	fn6_iter_retained
	.p2align	4
	.type	fn6_iter_retained,@function
fn6_iter_retained:
.Lfunc_begin0:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 140
	call	.L5$pb
.L5$pb:
	pop	ebx
	mov	ebp, dword ptr [esp + 160]
	xorps	xmm0, xmm0
	xor	ecx, ecx
	mov	dword ptr [esp + 128], 0
.Ltmp9:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp9-.L5$pb)
	movsd	qword ptr [esp + 120], xmm0
	movsd	qword ptr [esp + 112], xmm0
	movsd	qword ptr [esp + 36], xmm0
	movsd	qword ptr [esp + 44], xmm0
	movsd	qword ptr [esp + 52], xmm0
	movsd	qword ptr [esp + 60], xmm0
	movsd	qword ptr [esp + 68], xmm0
	movsd	qword ptr [esp + 76], xmm0
	movsd	qword ptr [esp + 84], xmm0
	movsd	qword ptr [esp + 92], xmm0
	mov	dword ptr [esp + 24], 0
	mov	edi, dword ptr [ebx + SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOT]
	lea	eax, [ebx + .Lanon.[ID].0@GOTOFF]
	mov	dword ptr [esp + 20], eax
	xor	eax, eax
	mov	dword ptr [esp + 32], ebp
	mov	dword ptr [esp + 100], 0
	mov	dword ptr [esp + 104], 0
	mov	dword ptr [esp + 108], 0
	mov	dword ptr [esp + 132], 0
	mov	dword ptr [esp + 136], 0
	cmp	eax, ecx
	jb	.LBB5_10
	.p2align	4
.LBB5_2:
	mov	esi, dword ptr [edi]
	test	esi, esi
	je	.LBB5_3
.LBB5_4:
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], ebp
	call	objc_msg_lookup@PLT
	lea	ecx, [esp + 36]
	lea	edx, [esp + 100]
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], ebp
	mov	dword ptr [esp + 16], 16
	mov	dword ptr [esp + 12], ecx
	mov	dword ptr [esp + 8], edx
	call	eax
	test	eax, eax
	mov	dword ptr [esp + 136], eax
	mov	dword ptr [esp + 132], 0
	je	.LBB5_13
	xor	eax, eax
	cmp	dword ptr [esp + 104], 0
	je	.LBB5_6
.LBB5_10:
	mov	ecx, dword ptr [esp + 108]
	test	ecx, ecx
	je	.LBB5_11
	mov	ecx, dword ptr [ecx]
	cmp	dword ptr [esp + 24], 1
	jne	.LBB5_17
	cmp	dword ptr [esp + 28], ecx
	je	.LBB5_11
	jmp	.LBB5_9
	.p2align	4
.LBB5_17:
	mov	dword ptr [esp + 24], 1
	mov	dword ptr [esp + 28], ecx
.LBB5_11:
	mov	ecx, dword ptr [esp + 104]
	lea	edx, [eax + 1]
	mov	dword ptr [esp + 132], edx
	mov	eax, dword ptr [ecx + 4*eax]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	mov	esi, eax
.Ltmp3:
	mov	dword ptr [esp], eax
	call	fn3_use_obj@PLT
.Ltmp4:
	mov	dword ptr [esp], esi
	call	objc_release@PLT
	mov	ebp, dword ptr [esp + 32]
	mov	eax, dword ptr [esp + 132]
	mov	ecx, dword ptr [esp + 136]
	cmp	eax, ecx
	jae	.LBB5_2
	jmp	.LBB5_10
.LBB5_3:
	mov	eax, dword ptr [esp + 20]
	mov	dword ptr [esp], edi
	mov	dword ptr [esp + 4], eax
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)@PLT
	mov	esi, eax
	jmp	.LBB5_4
.LBB5_13:
	add	esp, 140
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB5_6:
	call	SYM(objc2_foundation::iter::items_ptr_null::GENERATED_ID, 0)@PLT
.LBB5_9:
	call	SYM(objc2_foundation::iter::mutation_detected::GENERATED_ID, 0)@PLT
.LBB5_15:
.Ltmp5:
	mov	edi, eax
.Ltmp6:
	mov	dword ptr [esp], esi
	call	objc_release@PLT
.Ltmp7:
	mov	dword ptr [esp], edi
	call	_Unwind_Resume@PLT
.LBB5_14:
.Ltmp8:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
.Lfunc_end5:
	.size	fn6_iter_retained, .Lfunc_end5-fn6_iter_retained
	.section	.gcc_except_table.fn6_iter_retained,"a",@progbits
	.p2align	2, 0x0
GCC_except_table5:
.Lexception0:
	.byte	255
	.byte	155
	.uleb128 .Lttbase0-.Lttbaseref0
.Lttbaseref0:
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Lfunc_begin0-.Lfunc_begin0
	.uleb128 .Ltmp3-.Lfunc_begin0
	.byte	0
	.byte	0
	.uleb128 .Ltmp3-.Lfunc_begin0
	.uleb128 .Ltmp4-.Ltmp3
	.uleb128 .Ltmp5-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp4-.Lfunc_begin0
	.uleb128 .Ltmp6-.Ltmp4
	.byte	0
	.byte	0
	.uleb128 .Ltmp6-.Lfunc_begin0
	.uleb128 .Ltmp7-.Ltmp6
	.uleb128 .Ltmp8-.Lfunc_begin0
	.byte	1
	.uleb128 .Ltmp7-.Lfunc_begin0
	.uleb128 .Lfunc_end5-.Ltmp7
	.byte	0
	.byte	0
.Lcst_end0:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.type	.Lanon.[ID].0,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lanon.[ID].0:
	.asciz	"countByEnumeratingWithState:objects:count:"
	.size	.Lanon.[ID].0, 43

	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"awG",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	2, 0x0
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 4
DW.ref.rust_eh_personality:
	.long	rust_eh_personality
	.section	".note.GNU-stack","",@progbits
