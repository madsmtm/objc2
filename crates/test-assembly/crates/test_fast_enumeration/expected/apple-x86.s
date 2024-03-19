	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_iter_create
	.p2align	4, 0x90
_iter_create:
	push	ebp
	mov	ebp, esp
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
<<<<<<< HEAD
	mov	dword ptr [eax], ecx
	xorps	xmm0, xmm0
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
	pop	ebp
	ret	4

	.globl	_iter_once
	.p2align	4, 0x90
_iter_once:
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
<<<<<<< HEAD
	mov	eax, dword ptr [esi + 100]
	cmp	eax, dword ptr [esi + 104]
	jb	LBB1_4
	lea	ebx, [esi + 4]
	mov	edi, dword ptr [esi]
=======
	mov	eax, dword ptr [esi + 104]
	cmp	eax, dword ptr [esi + 108]
	jb	LBB1_6
	mov	eax, dword ptr [esi + 112]
	mov	dword ptr [ebp - 16], eax
	lea	ebx, [esi + 40]
	lea	edi, [esi + 8]
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	mov	edx, dword ptr [ecx + LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-L1$pb]
	mov	eax, dword ptr [edx]
	test	eax, eax
	je	LBB1_2
LBB1_3:
	lea	ecx, [esi + 68]
	sub	esp, 12
	push	16
	push	ebx
	push	ecx
	push	eax
	push	edi
	call	_objc_msgSend
	add	esp, 32
	mov	dword ptr [esi + 108], eax
	mov	dword ptr [esi + 104], 0
	test	eax, eax
	je	LBB1_4
	xor	eax, eax
	cmp	dword ptr [esi + 12], 0
	je	LBB1_13
LBB1_6:
	mov	ecx, dword ptr [esi + 16]
	test	ecx, ecx
	je	LBB1_7
	mov	ecx, dword ptr [ecx]
	cmp	dword ptr [esi], 0
	je	LBB1_9
	cmp	dword ptr [esi + 4], ecx
	je	LBB1_7
	call	SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)
LBB1_9:
	mov	dword ptr [esi], 1
	mov	dword ptr [esi + 4], ecx
LBB1_7:
	mov	ecx, dword ptr [esi + 12]
	lea	edx, [eax + 1]
	mov	dword ptr [esi + 104], edx
	mov	eax, dword ptr [ecx + 4*eax]
LBB1_8:
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB1_4:
	xor	eax, eax
	jmp	LBB1_8
LBB1_2:
	sub	esp, 8
	lea	eax, [ecx + l_anon.[ID].0-L1$pb]
	push	eax
	push	edx
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	add	esp, 16
	jmp	LBB1_3
LBB1_13:
	call	SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)

	.globl	_use_obj
	.p2align	4, 0x90
_use_obj:
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

	.globl	_iter
	.p2align	4, 0x90
_iter:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 124
	call	L3$pb
L3$pb:
	pop	eax
<<<<<<< HEAD
	mov	ebx, dword ptr [ebp + 8]
	xorps	xmm0, xmm0
	movsd	qword ptr [ebp - 36], xmm0
	movsd	qword ptr [ebp - 44], xmm0
	mov	dword ptr [ebp - 28], 0
	mov	dword ptr [ebp - 124], ebx
	lea	edi, [ebp - 56]
	movsd	qword ptr [ebp - 120], xmm0
	movsd	qword ptr [ebp - 112], xmm0
	movsd	qword ptr [ebp - 104], xmm0
	movsd	qword ptr [ebp - 96], xmm0
	movsd	qword ptr [ebp - 88], xmm0
	movsd	qword ptr [ebp - 80], xmm0
	movsd	qword ptr [ebp - 72], xmm0
	movsd	qword ptr [ebp - 64], xmm0
	movsd	qword ptr [ebp - 56], xmm0
=======
	mov	edi, dword ptr [ebp + 8]
	mov	dword ptr [ebp - 108], 0
	mov	dword ptr [ebp - 112], 0
	mov	dword ptr [ebp - 100], 0
	mov	dword ptr [ebp - 104], 0
	mov	dword ptr [ebp - 96], 0
	mov	dword ptr [ebp - 132], 0
	mov	dword ptr [ebp - 124], 0
	mov	dword ptr [ebp - 120], 0
	mov	dword ptr [ebp - 116], 0
	lea	ebx, [ebp - 92]
	mov	dword ptr [ebp - 88], 0
	mov	dword ptr [ebp - 92], 0
	mov	dword ptr [ebp - 80], 0
	mov	dword ptr [ebp - 84], 0
	mov	dword ptr [ebp - 72], 0
	mov	dword ptr [ebp - 76], 0
	mov	dword ptr [ebp - 64], 0
	mov	dword ptr [ebp - 68], 0
	mov	dword ptr [ebp - 56], 0
	mov	dword ptr [ebp - 60], 0
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	mov	dword ptr [ebp - 48], 0
	mov	dword ptr [ebp - 52], 0
	mov	dword ptr [ebp - 40], 0
	mov	dword ptr [ebp - 44], 0
	mov	dword ptr [ebp - 32], 0
	mov	dword ptr [ebp - 36], 0
	mov	dword ptr [ebp - 24], 0
	mov	dword ptr [ebp - 28], 0
	mov	dword ptr [ebp - 20], edi
	xor	ecx, ecx
	mov	dword ptr [ebp - 16], eax
	mov	esi, dword ptr [eax + LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-L3$pb]
	xor	eax, eax
	jmp	LBB3_1
	.p2align	4, 0x90
LBB3_8:
	mov	dword ptr [ebp - 132], 1
	mov	dword ptr [ebp - 128], ecx
LBB3_13:
	mov	ecx, dword ptr [ebp - 120]
	lea	edx, [eax + 1]
	mov	dword ptr [ebp - 28], edx
	sub	esp, 12
	push	dword ptr [ecx + 4*eax]
	call	_use_obj
	add	esp, 16
	mov	edi, dword ptr [ebp - 20]
	mov	eax, dword ptr [ebp - 28]
	mov	ecx, dword ptr [ebp - 24]
LBB3_1:
	cmp	eax, ecx
	jb	LBB3_12
	mov	eax, dword ptr [esi]
	test	eax, eax
	je	LBB3_3
LBB3_4:
	sub	esp, 12
	push	16
	push	ebx
	lea	ecx, [ebp - 124]
	push	ecx
	push	eax
	push	edi
	call	_objc_msgSend
	add	esp, 32
	mov	dword ptr [ebp - 24], eax
	mov	dword ptr [ebp - 28], 0
	test	eax, eax
	je	LBB3_11
	xor	eax, eax
	cmp	dword ptr [ebp - 120], 0
	je	LBB3_6
LBB3_12:
	mov	ecx, dword ptr [ebp - 116]
	test	ecx, ecx
	je	LBB3_13
	mov	ecx, dword ptr [ecx]
	cmp	dword ptr [ebp - 132], 0
	je	LBB3_8
	cmp	dword ptr [ebp - 128], ecx
	je	LBB3_13
	jmp	LBB3_10
LBB3_3:
	sub	esp, 8
	mov	eax, dword ptr [ebp - 16]
	lea	eax, [eax + l_anon.[ID].0-L3$pb]
	push	eax
	push	esi
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	add	esp, 16
	jmp	LBB3_4
LBB3_11:
	add	esp, 124
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB3_6:
	call	SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)
LBB3_10:
	call	SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)

	.globl	_iter_noop
	.p2align	4, 0x90
_iter_noop:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 124
	call	L4$pb
L4$pb:
<<<<<<< HEAD
	pop	eax
	mov	ebx, dword ptr [ebp + 8]
	xorps	xmm0, xmm0
	movsd	qword ptr [ebp - 36], xmm0
	movsd	qword ptr [ebp - 44], xmm0
	mov	dword ptr [ebp - 28], 0
	mov	dword ptr [ebp - 124], ebx
	lea	edi, [ebp - 56]
	movsd	qword ptr [ebp - 120], xmm0
	movsd	qword ptr [ebp - 112], xmm0
	movsd	qword ptr [ebp - 104], xmm0
	movsd	qword ptr [ebp - 96], xmm0
	movsd	qword ptr [ebp - 88], xmm0
	movsd	qword ptr [ebp - 80], xmm0
	movsd	qword ptr [ebp - 72], xmm0
	movsd	qword ptr [ebp - 64], xmm0
	movsd	qword ptr [ebp - 56], xmm0
=======
	pop	esi
	mov	edi, dword ptr [ebp + 8]
	mov	dword ptr [ebp - 104], 0
	mov	dword ptr [ebp - 108], 0
	mov	dword ptr [ebp - 96], 0
	mov	dword ptr [ebp - 100], 0
	mov	dword ptr [ebp - 92], 0
	mov	dword ptr [ebp - 128], 0
	mov	dword ptr [ebp - 120], 0
	mov	dword ptr [ebp - 116], 0
	mov	dword ptr [ebp - 112], 0
	lea	ebx, [ebp - 88]
	mov	dword ptr [ebp - 84], 0
	mov	dword ptr [ebp - 88], 0
	mov	dword ptr [ebp - 76], 0
	mov	dword ptr [ebp - 80], 0
	mov	dword ptr [ebp - 68], 0
	mov	dword ptr [ebp - 72], 0
	mov	dword ptr [ebp - 60], 0
	mov	dword ptr [ebp - 64], 0
	mov	dword ptr [ebp - 52], 0
	mov	dword ptr [ebp - 56], 0
	mov	dword ptr [ebp - 44], 0
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	mov	dword ptr [ebp - 48], 0
	mov	dword ptr [ebp - 36], 0
	mov	dword ptr [ebp - 40], 0
	mov	dword ptr [ebp - 28], 0
	mov	dword ptr [ebp - 32], 0
	mov	dword ptr [ebp - 20], 0
	mov	dword ptr [ebp - 24], 0
	mov	dword ptr [ebp - 16], edi
	xor	ecx, ecx
	xor	eax, eax
	xor	edx, edx
	jmp	LBB4_1
	.p2align	4, 0x90
LBB4_9:
	mov	dword ptr [ebp - 128], 1
	mov	dword ptr [ebp - 124], edi
LBB4_11:
	inc	edx
	mov	dword ptr [ebp - 24], edx
	mov	edi, dword ptr [ebp - 16]
LBB4_1:
	cmp	edx, eax
	jb	LBB4_7
	mov	ecx, dword ptr [esi + LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-L4$pb]
	mov	eax, dword ptr [ecx]
	test	eax, eax
	je	LBB4_3
LBB4_4:
	sub	esp, 12
	push	16
	push	ebx
	lea	ecx, [ebp - 120]
	push	ecx
	push	eax
	push	edi
	call	_objc_msgSend
	add	esp, 32
	mov	dword ptr [ebp - 20], eax
	mov	dword ptr [ebp - 24], 0
	test	eax, eax
	je	LBB4_12
	cmp	dword ptr [ebp - 116], 0
	je	LBB4_13
	xor	edx, edx
	mov	ecx, dword ptr [ebp - 112]
LBB4_7:
	test	ecx, ecx
	je	LBB4_11
	mov	edi, dword ptr [ecx]
	cmp	dword ptr [ebp - 128], 0
	je	LBB4_9
	cmp	dword ptr [ebp - 124], edi
	je	LBB4_11
	jmp	LBB4_14
LBB4_3:
	sub	esp, 8
	lea	eax, [esi + l_anon.[ID].0-L4$pb]
	push	eax
	push	ecx
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	add	esp, 16
	jmp	LBB4_4
LBB4_12:
	add	esp, 124
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB4_13:
	call	SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)
LBB4_14:
	call	SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)

	.globl	_iter_retained
	.p2align	4, 0x90
_iter_retained:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 124
	call	L5$pb
L5$pb:
	pop	edi
	mov	esi, dword ptr [ebp + 8]
<<<<<<< HEAD
	xorps	xmm0, xmm0
	movsd	qword ptr [ebp - 36], xmm0
	movsd	qword ptr [ebp - 44], xmm0
	mov	dword ptr [ebp - 28], 0
	mov	dword ptr [ebp - 124], esi
	lea	ebx, [ebp - 56]
	movsd	qword ptr [ebp - 120], xmm0
	movsd	qword ptr [ebp - 112], xmm0
	movsd	qword ptr [ebp - 104], xmm0
	movsd	qword ptr [ebp - 96], xmm0
	movsd	qword ptr [ebp - 88], xmm0
	movsd	qword ptr [ebp - 80], xmm0
	movsd	qword ptr [ebp - 72], xmm0
	movsd	qword ptr [ebp - 64], xmm0
	movsd	qword ptr [ebp - 56], xmm0
=======
	mov	dword ptr [ebp - 104], 0
	mov	dword ptr [ebp - 108], 0
	mov	dword ptr [ebp - 96], 0
	mov	dword ptr [ebp - 100], 0
	mov	dword ptr [ebp - 92], 0
	mov	dword ptr [ebp - 128], 0
	mov	dword ptr [ebp - 120], 0
	mov	dword ptr [ebp - 116], 0
	mov	dword ptr [ebp - 112], 0
	lea	ebx, [ebp - 88]
	mov	dword ptr [ebp - 84], 0
	mov	dword ptr [ebp - 88], 0
	mov	dword ptr [ebp - 76], 0
	mov	dword ptr [ebp - 80], 0
	mov	dword ptr [ebp - 68], 0
	mov	dword ptr [ebp - 72], 0
	mov	dword ptr [ebp - 60], 0
	mov	dword ptr [ebp - 64], 0
	mov	dword ptr [ebp - 52], 0
	mov	dword ptr [ebp - 56], 0
	mov	dword ptr [ebp - 44], 0
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	mov	dword ptr [ebp - 48], 0
	mov	dword ptr [ebp - 36], 0
	mov	dword ptr [ebp - 40], 0
	mov	dword ptr [ebp - 28], 0
	mov	dword ptr [ebp - 32], 0
	mov	dword ptr [ebp - 20], 0
	mov	dword ptr [ebp - 24], 0
	mov	dword ptr [ebp - 16], esi
	xor	ecx, ecx
	xor	eax, eax
	jmp	LBB5_1
	.p2align	4, 0x90
LBB5_8:
	mov	dword ptr [ebp - 128], 1
	mov	dword ptr [ebp - 124], ecx
LBB5_13:
	mov	ecx, dword ptr [ebp - 116]
	lea	edx, [eax + 1]
	mov	dword ptr [ebp - 24], edx
	sub	esp, 12
	push	dword ptr [ecx + 4*eax]
	call	_objc_retain
	add	esp, 16
	mov	esi, eax
	sub	esp, 12
	push	eax
	call	_use_obj
	add	esp, 4
	push	esi
	call	_objc_release
	add	esp, 16
	mov	esi, dword ptr [ebp - 16]
	mov	eax, dword ptr [ebp - 24]
	mov	ecx, dword ptr [ebp - 20]
LBB5_1:
	cmp	eax, ecx
	jb	LBB5_12
	mov	ecx, dword ptr [edi + LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-L5$pb]
	mov	eax, dword ptr [ecx]
	test	eax, eax
	je	LBB5_3
LBB5_4:
	sub	esp, 12
	push	16
	push	ebx
	lea	ecx, [ebp - 120]
	push	ecx
	push	eax
	push	esi
	call	_objc_msgSend
	add	esp, 32
	mov	dword ptr [ebp - 20], eax
	mov	dword ptr [ebp - 24], 0
	test	eax, eax
	je	LBB5_11
	xor	eax, eax
	cmp	dword ptr [ebp - 116], 0
	je	LBB5_6
LBB5_12:
	mov	ecx, dword ptr [ebp - 112]
	test	ecx, ecx
	je	LBB5_13
	mov	ecx, dword ptr [ecx]
	cmp	dword ptr [ebp - 128], 0
	je	LBB5_8
	cmp	dword ptr [ebp - 124], ecx
	je	LBB5_13
	jmp	LBB5_10
LBB5_3:
	sub	esp, 8
	lea	eax, [edi + l_anon.[ID].0-L5$pb]
	push	eax
	push	ecx
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	add	esp, 16
	jmp	LBB5_4
LBB5_11:
	add	esp, 124
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB5_6:
	call	SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)
LBB5_10:
	call	SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)

	.section	__TEXT,__const
l_anon.[ID].0:
	.asciz	"countByEnumeratingWithState:objects:count:"

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr:
	.indirect_symbol	SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)
	.long	0

.subsections_via_symbols
