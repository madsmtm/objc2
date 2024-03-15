	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_iter_create
	.p2align	4, 0x90
_iter_create:
	push	ebp
	mov	ebp, esp
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
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
	mov	edi, dword ptr [esi]
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
	mov	dword ptr [ebp - 48], 0
	mov	dword ptr [ebp - 24], 0
	mov	dword ptr [ebp - 20], 0
	mov	esi, dword ptr [eax + LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-L3$pb]
	lea	eax, [eax + l_anon.[ID].0-L3$pb]
	mov	dword ptr [ebp - 16], eax
	xor	eax, eax
	xor	ecx, ecx
	jmp	LBB3_1
	.p2align	4, 0x90
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
	je	LBB3_5
LBB3_6:
	mov	eax, dword ptr [ebp - 52]
	lea	edx, [ecx + 1]
	mov	dword ptr [ebp - 24], edx
	sub	esp, 12
	push	dword ptr [eax + 4*ecx]
	call	_use_obj
	add	esp, 16
	mov	ebx, dword ptr [ebp - 124]
	mov	ecx, dword ptr [ebp - 24]
	mov	eax, dword ptr [ebp - 20]
LBB3_1:
	cmp	ecx, eax
	jb	LBB3_6
	mov	eax, dword ptr [esi]
	test	eax, eax
	jne	LBB3_4
	sub	esp, 8
	push	dword ptr [ebp - 16]
	push	esi
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	add	esp, 16
	jmp	LBB3_4
LBB3_5:
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
	mov	dword ptr [ebp - 48], 0
	mov	dword ptr [ebp - 24], 0
	mov	dword ptr [ebp - 20], 0
	mov	esi, dword ptr [eax + LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-L4$pb]
	lea	eax, [eax + l_anon.[ID].0-L4$pb]
	mov	dword ptr [ebp - 16], eax
	xor	eax, eax
	xor	ecx, ecx
	jmp	LBB4_1
	.p2align	4, 0x90
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
	push	edi
	push	eax
	push	ebx
	call	_objc_msgSend
	add	esp, 32
	mov	dword ptr [ebp - 20], eax
	test	eax, eax
	je	LBB4_7
	xor	ecx, ecx
	mov	ebx, dword ptr [ebp - 124]
	jmp	LBB4_6
LBB4_3:
	sub	esp, 8
	push	dword ptr [ebp - 16]
	push	esi
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
	mov	dword ptr [ebp - 48], 0
	mov	dword ptr [ebp - 24], 0
	mov	dword ptr [ebp - 20], 0
	mov	edi, dword ptr [eax + LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-L5$pb]
	lea	eax, [eax + l_anon.[ID].0-L5$pb]
	mov	dword ptr [ebp - 16], eax
	xor	eax, eax
	xor	ecx, ecx
	jmp	LBB5_1
	.p2align	4, 0x90
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
	xor	ecx, ecx
	test	eax, eax
	je	LBB5_5
LBB5_6:
	mov	eax, dword ptr [ebp - 52]
	lea	edx, [ecx + 1]
	mov	dword ptr [ebp - 24], edx
	sub	esp, 12
	push	dword ptr [eax + 4*ecx]
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
	mov	ecx, dword ptr [ebp - 24]
	mov	eax, dword ptr [ebp - 20]
LBB5_1:
	cmp	ecx, eax
	jb	LBB5_6
	mov	eax, dword ptr [edi]
	test	eax, eax
	jne	LBB5_4
	sub	esp, 8
	push	dword ptr [ebp - 16]
	push	edi
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	add	esp, 16
	jmp	LBB5_4
LBB5_5:
	add	esp, 124
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.section	__TEXT,__const
l_anon.[ID].0:
	.asciz	"countByEnumeratingWithState:objects:count:"

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr:
	.indirect_symbol	SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)
	.long	0

.subsections_via_symbols
