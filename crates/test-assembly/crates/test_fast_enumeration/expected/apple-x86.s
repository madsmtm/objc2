	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_iter_create
	.p2align	4, 0x90
_iter_create:
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
	lea	eax, [ecx + l_anon.[ID].0-L1$pb]
	push	eax
	push	edx
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	add	esp, 16
	jmp	LBB1_3

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
	mov	dword ptr [ebp - 124], esi
	lea	edi, [ebp - 56]
	mov	dword ptr [ebp - 56], 0
	mov	dword ptr [ebp - 52], 0
	mov	dword ptr [ebp - 48], 0
	mov	dword ptr [ebp - 24], 0
	mov	dword ptr [ebp - 20], 0
	mov	ebx, dword ptr [eax + LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-L3$pb]
	lea	eax, [eax + l_anon.[ID].0-L3$pb]
	mov	dword ptr [ebp - 16], eax
	xor	eax, eax
	xor	ecx, ecx
	cmp	ecx, eax
	jb	LBB3_5
	.p2align	4, 0x90
LBB3_2:
	mov	eax, dword ptr [ebx]
	test	eax, eax
	je	LBB3_3
LBB3_4:
	sub	esp, 12
	push	16
	lea	ecx, [ebp - 120]
	push	ecx
	push	edi
	push	eax
	push	esi
	call	_objc_msgSend
	add	esp, 32
	mov	dword ptr [ebp - 20], eax
	xor	ecx, ecx
	test	eax, eax
	je	LBB3_6
LBB3_5:
	mov	eax, dword ptr [ebp - 52]
	lea	edx, [ecx + 1]
	mov	dword ptr [ebp - 24], edx
	mov	eax, dword ptr [eax + 4*ecx]
	test	eax, eax
	je	LBB3_6
	sub	esp, 12
	push	eax
	call	_use_obj
	add	esp, 16
	mov	esi, dword ptr [ebp - 124]
	mov	ecx, dword ptr [ebp - 24]
	mov	eax, dword ptr [ebp - 20]
	cmp	ecx, eax
	jae	LBB3_2
	jmp	LBB3_5
LBB3_3:
	sub	esp, 8
	push	dword ptr [ebp - 16]
	push	ebx
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	add	esp, 16
	jmp	LBB3_4
LBB3_6:
	add	esp, 124
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

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
	pop	eax
	mov	ecx, dword ptr [ebp + 8]
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
	mov	dword ptr [ebp - 124], ecx
	lea	edi, [ebp - 56]
	mov	dword ptr [ebp - 56], 0
	mov	dword ptr [ebp - 52], 0
	mov	dword ptr [ebp - 48], 0
	mov	dword ptr [ebp - 24], 0
	mov	dword ptr [ebp - 20], 0
	xor	ecx, ecx
	mov	ebx, dword ptr [eax + LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-L4$pb]
	lea	eax, [eax + l_anon.[ID].0-L4$pb]
	mov	dword ptr [ebp - 16], eax
	xor	eax, eax
	xor	edx, edx
	jmp	LBB4_1
	.p2align	4, 0x90
LBB4_6:
	lea	esi, [edx + 1]
	mov	dword ptr [ebp - 24], esi
	cmp	dword ptr [ecx + 4*edx], 0
	mov	edx, esi
	je	LBB4_7
LBB4_1:
	cmp	edx, eax
	jb	LBB4_6
	mov	esi, dword ptr [ebp - 124]
	mov	eax, dword ptr [ebx]
	test	eax, eax
	je	LBB4_3
LBB4_4:
	sub	esp, 12
	push	16
	lea	ecx, [ebp - 120]
	push	ecx
	push	edi
	push	eax
	push	esi
	call	_objc_msgSend
	add	esp, 32
	mov	dword ptr [ebp - 20], eax
	test	eax, eax
	je	LBB4_7
	xor	edx, edx
	mov	ecx, dword ptr [ebp - 52]
	jmp	LBB4_6
LBB4_3:
	sub	esp, 8
	push	dword ptr [ebp - 16]
	push	ebx
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	add	esp, 16
	jmp	LBB4_4
LBB4_7:
	add	esp, 124
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

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
	lea	eax, [eax + l_anon.[ID].0-L5$pb]
	mov	dword ptr [ebp - 16], eax
	xor	ecx, ecx
	xor	eax, eax
	cmp	eax, ecx
	jb	LBB5_11
	.p2align	4, 0x90
LBB5_2:
	mov	eax, dword ptr [edi]
	test	eax, eax
	je	LBB5_3
LBB5_4:
	sub	esp, 12
	push	16
	lea	ecx, [ebp - 120]
	push	ecx
	push	ebx
	push	eax
	push	esi
	call	_objc_msgSend
	add	esp, 32
	mov	dword ptr [ebp - 20], eax
	mov	dword ptr [ebp - 24], 0
	test	eax, eax
	je	LBB5_13
	xor	eax, eax
	cmp	dword ptr [ebp - 52], 0
	je	LBB5_6
LBB5_11:
	mov	ecx, dword ptr [ebp - 48]
	test	ecx, ecx
	je	LBB5_12
	mov	ecx, dword ptr [ecx]
	test	byte ptr [ebp - 132], 1
	je	LBB5_8
	cmp	dword ptr [ebp - 128], ecx
	je	LBB5_12
	jmp	LBB5_10
	.p2align	4, 0x90
LBB5_8:
	mov	dword ptr [ebp - 132], 1
	mov	dword ptr [ebp - 128], ecx
LBB5_12:
	mov	ecx, dword ptr [ebp - 52]
	lea	edx, [eax + 1]
	mov	dword ptr [ebp - 24], edx
	mov	eax, dword ptr [ecx + 4*eax]
	test	eax, eax
	je	LBB5_13
	sub	esp, 12
	push	eax
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
	mov	esi, dword ptr [ebp - 124]
	mov	eax, dword ptr [ebp - 24]
	mov	ecx, dword ptr [ebp - 20]
	cmp	eax, ecx
	jae	LBB5_2
	jmp	LBB5_11
LBB5_3:
	sub	esp, 8
	push	dword ptr [ebp - 16]
	push	edi
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	add	esp, 16
	jmp	LBB5_4
LBB5_13:
	add	esp, 124
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB5_6:
	call	SYM(objc2_foundation::iter::items_ptr_null::GENERATED_ID, 0)
LBB5_10:
	call	SYM(objc2_foundation::iter::mutation_detected::GENERATED_ID, 0)

	.section	__TEXT,__const
l_anon.[ID].0:
	.asciz	"countByEnumeratingWithState:objects:count:"

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr:
	.indirect_symbol	SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)
	.long	0

.subsections_via_symbols
