	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_fn1_iter_create
	.p2align	4
_fn1_iter_create:
	push	ebp
	mov	ebp, esp
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
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
	pop	ebp
	ret	4

	.globl	_fn2_iter_once
	.p2align	4
_fn2_iter_once:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	call	L1$pb
L1$pb:
	pop	ecx
	mov	esi, dword ptr [ebp + 8]
	mov	eax, dword ptr [esi + 100]
	cmp	eax, dword ptr [esi + 104]
	jb	LBB1_4
	lea	ebx, [esi + 4]
	mov	eax, dword ptr [esi]
	mov	dword ptr [ebp - 16], eax
	lea	edi, [esi + 68]
	mov	edx, dword ptr [ecx + LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-L1$pb]
	mov	eax, dword ptr [edx]
	test	eax, eax
	je	LBB1_2
LBB1_3:
	sub	esp, 12
	push	16
	push	ebx
	push	edi
	push	eax
	push	dword ptr [ebp - 16]
	call	_objc_msgSend
	add	esp, 32
	mov	ecx, eax
	mov	dword ptr [esi + 104], eax
	mov	dword ptr [esi + 100], 0
	xor	eax, eax
	test	ecx, ecx
	je	LBB1_5
LBB1_4:
	mov	ecx, dword ptr [esi + 72]
	lea	edx, [eax + 1]
	mov	dword ptr [esi + 100], edx
	mov	eax, dword ptr [ecx + 4*eax]
LBB1_5:
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB1_2:
	sub	esp, 8
	lea	eax, [ecx + L_anon.[ID].0-L1$pb]
	push	eax
	push	edx
	call	SYM(objc2::__macros::sel::CachedSel::fetch::GENERATED_ID, 0)
	add	esp, 16
	jmp	LBB1_3

	.globl	_fn3_use_obj
	.p2align	4
_fn3_use_obj:
	push	ebp
	mov	ebp, esp
	push	eax
	mov	eax, dword ptr [ebp + 8]
	mov	dword ptr [ebp - 4], eax
	lea	eax, [ebp - 4]
	## InlineAsm Start
	## InlineAsm End
	add	esp, 4
	pop	ebp
	ret

	.globl	_fn4_iter
	.p2align	4
_fn4_iter:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 124
	call	L3$pb
L3$pb:
	pop	eax
	mov	ebx, dword ptr [ebp + 8]
	xorps	xmm0, xmm0
	movsd	qword ptr [ebp - 36], xmm0
	movsd	qword ptr [ebp - 44], xmm0
	mov	dword ptr [ebp - 28], 0
	movsd	qword ptr [ebp - 120], xmm0
	movsd	qword ptr [ebp - 112], xmm0
	movsd	qword ptr [ebp - 104], xmm0
	movsd	qword ptr [ebp - 96], xmm0
	movsd	qword ptr [ebp - 88], xmm0
	movsd	qword ptr [ebp - 80], xmm0
	movsd	qword ptr [ebp - 72], xmm0
	movsd	qword ptr [ebp - 64], xmm0
	mov	dword ptr [ebp - 124], ebx
	lea	edi, [ebp - 56]
	mov	dword ptr [ebp - 56], 0
	mov	dword ptr [ebp - 52], 0
	mov	dword ptr [ebp - 48], 0
	mov	dword ptr [ebp - 24], 0
	mov	dword ptr [ebp - 20], 0
	mov	esi, dword ptr [eax + LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-L3$pb]
	lea	eax, [eax + L_anon.[ID].0-L3$pb]
	mov	dword ptr [ebp - 16], eax
	xor	eax, eax
	xor	ecx, ecx
	jmp	LBB3_1
	.p2align	4
LBB3_5:
	mov	eax, dword ptr [ebp - 52]
	lea	edx, [ecx + 1]
	mov	dword ptr [ebp - 24], edx
	sub	esp, 12
	push	dword ptr [eax + 4*ecx]
	call	_fn3_use_obj
	add	esp, 16
	mov	ebx, dword ptr [ebp - 124]
	mov	ecx, dword ptr [ebp - 24]
	mov	eax, dword ptr [ebp - 20]
LBB3_1:
	cmp	ecx, eax
	jb	LBB3_5
	mov	eax, dword ptr [esi]
	test	eax, eax
	je	LBB3_3
LBB3_4:
	sub	esp, 12
	push	16
	lea	ecx, [ebp - 120]
	push	ecx
	push	edi
	push	eax
	push	ebx
	call	_objc_msgSend
	add	esp, 32
	mov	dword ptr [ebp - 20], eax
	xor	ecx, ecx
	test	eax, eax
	jne	LBB3_5
	jmp	LBB3_6
LBB3_3:
	sub	esp, 8
	push	dword ptr [ebp - 16]
	push	esi
	call	SYM(objc2::__macros::sel::CachedSel::fetch::GENERATED_ID, 0)
	add	esp, 16
	jmp	LBB3_4
LBB3_6:
	add	esp, 124
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.globl	_fn5_iter_noop
	.p2align	4
_fn5_iter_noop:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 124
	call	L4$pb
L4$pb:
	pop	eax
	mov	edi, dword ptr [ebp + 8]
	xorps	xmm0, xmm0
	movsd	qword ptr [ebp - 36], xmm0
	movsd	qword ptr [ebp - 44], xmm0
	mov	dword ptr [ebp - 28], 0
	movsd	qword ptr [ebp - 120], xmm0
	movsd	qword ptr [ebp - 112], xmm0
	movsd	qword ptr [ebp - 104], xmm0
	movsd	qword ptr [ebp - 96], xmm0
	movsd	qword ptr [ebp - 88], xmm0
	movsd	qword ptr [ebp - 80], xmm0
	movsd	qword ptr [ebp - 72], xmm0
	movsd	qword ptr [ebp - 64], xmm0
	mov	dword ptr [ebp - 124], edi
	lea	ebx, [ebp - 56]
	mov	dword ptr [ebp - 56], 0
	mov	dword ptr [ebp - 52], 0
	mov	dword ptr [ebp - 48], 0
	mov	dword ptr [ebp - 24], 0
	mov	dword ptr [ebp - 20], 0
	mov	esi, dword ptr [eax + LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-L4$pb]
	lea	eax, [eax + L_anon.[ID].0-L4$pb]
	mov	dword ptr [ebp - 16], eax
	xor	eax, eax
	xor	ecx, ecx
	jmp	LBB4_1
	.p2align	4
LBB4_6:
	inc	ecx
	mov	dword ptr [ebp - 24], ecx
LBB4_1:
	cmp	ecx, eax
	jb	LBB4_6
	mov	eax, dword ptr [esi]
	test	eax, eax
	je	LBB4_3
LBB4_4:
	sub	esp, 12
	push	16
	lea	ecx, [ebp - 120]
	push	ecx
	push	ebx
	push	eax
	push	edi
	call	_objc_msgSend
	add	esp, 32
	mov	dword ptr [ebp - 20], eax
	test	eax, eax
	je	LBB4_7
	xor	ecx, ecx
	mov	edi, dword ptr [ebp - 124]
	jmp	LBB4_6
LBB4_3:
	sub	esp, 8
	push	dword ptr [ebp - 16]
	push	esi
	call	SYM(objc2::__macros::sel::CachedSel::fetch::GENERATED_ID, 0)
	add	esp, 16
	jmp	LBB4_4
LBB4_7:
	add	esp, 124
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.globl	_fn6_iter_retained
	.p2align	4
_fn6_iter_retained:
Lfunc_begin0:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 140
	call	L5$pb
L5$pb:
	pop	eax
	mov	esi, dword ptr [ebp + 8]
	xorps	xmm0, xmm0
	movsd	qword ptr [ebp - 36], xmm0
	movsd	qword ptr [ebp - 44], xmm0
	mov	dword ptr [ebp - 28], 0
	movsd	qword ptr [ebp - 120], xmm0
	movsd	qword ptr [ebp - 112], xmm0
	movsd	qword ptr [ebp - 104], xmm0
	movsd	qword ptr [ebp - 96], xmm0
	movsd	qword ptr [ebp - 88], xmm0
	movsd	qword ptr [ebp - 80], xmm0
	movsd	qword ptr [ebp - 72], xmm0
	movsd	qword ptr [ebp - 64], xmm0
	mov	dword ptr [ebp - 132], 0
	mov	dword ptr [ebp - 124], esi
	lea	ebx, [ebp - 56]
	mov	dword ptr [ebp - 56], 0
	mov	dword ptr [ebp - 52], 0
	mov	dword ptr [ebp - 48], 0
	mov	dword ptr [ebp - 24], 0
	mov	dword ptr [ebp - 20], 0
	mov	edi, dword ptr [eax + LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-L5$pb]
	lea	eax, [eax + L_anon.[ID].0-L5$pb]
	mov	dword ptr [ebp - 16], eax
	xor	ecx, ecx
	xor	eax, eax
	cmp	eax, ecx
	jb	LBB5_10
	.p2align	4
LBB5_2:
	mov	eax, dword ptr [edi]
	test	eax, eax
	je	LBB5_3
LBB5_4:
	lea	ecx, [ebp - 120]
	mov	dword ptr [esp + 12], ecx
	mov	dword ptr [esp + 8], ebx
	mov	dword ptr [esp + 4], eax
	mov	dword ptr [esp], esi
	mov	dword ptr [esp + 16], 16
	call	_objc_msgSend
	mov	dword ptr [ebp - 20], eax
	mov	dword ptr [ebp - 24], 0
	test	eax, eax
	je	LBB5_13
	xor	eax, eax
	cmp	dword ptr [ebp - 52], 0
	je	LBB5_6
LBB5_10:
	mov	ecx, dword ptr [ebp - 48]
	test	ecx, ecx
	je	LBB5_11
	mov	ecx, dword ptr [ecx]
	cmp	dword ptr [ebp - 132], 1
	jne	LBB5_17
	cmp	dword ptr [ebp - 128], ecx
	je	LBB5_11
	jmp	LBB5_9
	.p2align	4
LBB5_17:
	mov	dword ptr [ebp - 132], 1
	mov	dword ptr [ebp - 128], ecx
LBB5_11:
	mov	ecx, dword ptr [ebp - 52]
	lea	edx, [eax + 1]
	mov	dword ptr [ebp - 24], edx
	mov	eax, dword ptr [ecx + 4*eax]
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	esi, eax
Ltmp0:
	mov	dword ptr [esp], eax
	call	_fn3_use_obj
Ltmp1:
	mov	dword ptr [esp], esi
	call	_objc_release
	mov	esi, dword ptr [ebp - 124]
	mov	eax, dword ptr [ebp - 24]
	mov	ecx, dword ptr [ebp - 20]
	cmp	eax, ecx
	jae	LBB5_2
	jmp	LBB5_10
LBB5_3:
	mov	eax, dword ptr [ebp - 16]
	mov	dword ptr [esp + 4], eax
	mov	dword ptr [esp], edi
	call	SYM(objc2::__macros::sel::CachedSel::fetch::GENERATED_ID, 0)
	jmp	LBB5_4
LBB5_13:
	add	esp, 140
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB5_6:
	call	SYM(objc2_foundation::iter::items_ptr_null::GENERATED_ID, 0)
LBB5_9:
	call	SYM(objc2_foundation::iter::mutation_detected::GENERATED_ID, 0)
LBB5_15:
Ltmp2:
	mov	edi, eax
Ltmp3:
	mov	dword ptr [esp], esi
	call	_objc_release
Ltmp4:
	mov	dword ptr [esp], edi
	call	__Unwind_Resume
LBB5_14:
Ltmp5:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table5:
Lexception0:
	.byte	255
	.byte	155
	.uleb128 Lttbase0-Lttbaseref0
Lttbaseref0:
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Lfunc_begin0-Lfunc_begin0
	.uleb128 Ltmp0-Lfunc_begin0
	.byte	0
	.byte	0
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp1-Ltmp0
	.uleb128 Ltmp2-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp1-Lfunc_begin0
	.uleb128 Ltmp3-Ltmp1
	.byte	0
	.byte	0
	.uleb128 Ltmp3-Lfunc_begin0
	.uleb128 Ltmp4-Ltmp3
	.uleb128 Ltmp5-Lfunc_begin0
	.byte	1
	.uleb128 Ltmp4-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp4
	.byte	0
	.byte	0
Lcst_end0:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].0:
	.asciz	"countByEnumeratingWithState:objects:count:"

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr:
	.indirect_symbol	SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)
	.long	0
L_rust_eh_personality$non_lazy_ptr:
	.indirect_symbol	_rust_eh_personality
	.long	0

.subsections_via_symbols
