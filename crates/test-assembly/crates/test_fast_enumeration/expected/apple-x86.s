	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_iter_create
	.p2align	4, 0x90
_iter_create:
	push	ebp
	mov	ebp, esp
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [eax + 84], 0
	mov	dword ptr [eax + 80], 0
	mov	dword ptr [eax + 92], 0
	mov	dword ptr [eax + 88], 0
	mov	dword ptr [eax + 96], 0
	mov	dword ptr [eax], ecx
	mov	dword ptr [eax + 8], 0
	mov	dword ptr [eax + 4], 0
	mov	dword ptr [eax + 16], 0
	mov	dword ptr [eax + 12], 0
	mov	dword ptr [eax + 24], 0
	mov	dword ptr [eax + 20], 0
	mov	dword ptr [eax + 32], 0
	mov	dword ptr [eax + 28], 0
	mov	dword ptr [eax + 40], 0
	mov	dword ptr [eax + 36], 0
	mov	dword ptr [eax + 48], 0
	mov	dword ptr [eax + 44], 0
	mov	dword ptr [eax + 56], 0
	mov	dword ptr [eax + 52], 0
	mov	dword ptr [eax + 64], 0
	mov	dword ptr [eax + 60], 0
	mov	dword ptr [eax + 72], 0
	mov	dword ptr [eax + 68], 0
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
	jb	LBB1_1
	lea	eax, [esi + 4]
	mov	dword ptr [ebp - 20], eax
	mov	edx, dword ptr [esi]
	lea	edi, [esi + 68]
	mov	ebx, dword ptr [ecx + L__ZN6icrate10Foundation9generated14__NSEnumerator17NSFastEnumeration41countByEnumeratingWithState_objects_count10CACHED_SEL17h230d5b6091d37064E$non_lazy_ptr-L1$pb]
	mov	eax, dword ptr [ebx]
	test	eax, eax
	jne	LBB1_4
	sub	esp, 12
	lea	eax, [ecx + l_anon.[ID].0-L1$pb]
	push	eax
	mov	dword ptr [ebp - 16], edx
	call	SYM(objc2::runtime::Sel::register_unchecked::GENERATED_ID, 0)
	mov	edx, dword ptr [ebp - 16]
	add	esp, 16
	mov	dword ptr [ebx], eax
LBB1_4:
	sub	esp, 12
	push	16
	push	dword ptr [ebp - 20]
	push	edi
	push	eax
	push	edx
	call	_objc_msgSend
	add	esp, 32
	mov	ecx, eax
	mov	dword ptr [esi + 104], eax
	mov	dword ptr [esi + 100], 0
	xor	eax, eax
	test	ecx, ecx
	je	LBB1_5
LBB1_1:
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
	mov	dword ptr [ebp - 44], 0
	mov	dword ptr [ebp - 48], 0
	mov	dword ptr [ebp - 36], 0
	mov	dword ptr [ebp - 40], 0
	mov	dword ptr [ebp - 32], 0
	mov	dword ptr [ebp - 128], ebx
	lea	edi, [ebp - 60]
	mov	dword ptr [ebp - 120], 0
	mov	dword ptr [ebp - 124], 0
	mov	dword ptr [ebp - 112], 0
	mov	dword ptr [ebp - 116], 0
	mov	dword ptr [ebp - 104], 0
	mov	dword ptr [ebp - 108], 0
	mov	dword ptr [ebp - 96], 0
	mov	dword ptr [ebp - 100], 0
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
	mov	dword ptr [ebp - 52], 0
	mov	dword ptr [ebp - 28], 0
	mov	dword ptr [ebp - 24], 0
	mov	esi, dword ptr [eax + L__ZN6icrate10Foundation9generated14__NSEnumerator17NSFastEnumeration41countByEnumeratingWithState_objects_count10CACHED_SEL17h230d5b6091d37064E$non_lazy_ptr-L3$pb]
	lea	eax, [eax + l_anon.[ID].0-L3$pb]
	mov	dword ptr [ebp - 16], eax
	xor	eax, eax
	xor	ecx, ecx
	jmp	LBB3_1
	.p2align	4, 0x90
LBB3_4:
	sub	esp, 12
	push	16
	lea	ecx, [ebp - 124]
	push	ecx
	push	edi
	push	eax
	push	ebx
	call	_objc_msgSend
	add	esp, 32
	mov	dword ptr [ebp - 24], eax
	xor	ecx, ecx
	test	eax, eax
	je	LBB3_5
LBB3_6:
	mov	eax, dword ptr [ebp - 56]
	lea	edx, [ecx + 1]
	mov	dword ptr [ebp - 28], edx
	sub	esp, 12
	push	dword ptr [eax + 4*ecx]
	call	_use_obj
	add	esp, 16
	mov	ebx, dword ptr [ebp - 128]
	mov	ecx, dword ptr [ebp - 28]
	mov	eax, dword ptr [ebp - 24]
LBB3_1:
	cmp	ecx, eax
	jb	LBB3_6
	mov	eax, dword ptr [esi]
	test	eax, eax
	jne	LBB3_4
	sub	esp, 12
	push	dword ptr [ebp - 16]
	call	SYM(objc2::runtime::Sel::register_unchecked::GENERATED_ID, 0)
	add	esp, 16
	mov	dword ptr [esi], eax
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
	mov	dword ptr [ebp - 44], 0
	mov	dword ptr [ebp - 48], 0
	mov	dword ptr [ebp - 36], 0
	mov	dword ptr [ebp - 40], 0
	mov	dword ptr [ebp - 32], 0
	mov	dword ptr [ebp - 128], ebx
	lea	edi, [ebp - 60]
	mov	dword ptr [ebp - 120], 0
	mov	dword ptr [ebp - 124], 0
	mov	dword ptr [ebp - 112], 0
	mov	dword ptr [ebp - 116], 0
	mov	dword ptr [ebp - 104], 0
	mov	dword ptr [ebp - 108], 0
	mov	dword ptr [ebp - 96], 0
	mov	dword ptr [ebp - 100], 0
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
	mov	dword ptr [ebp - 52], 0
	mov	dword ptr [ebp - 28], 0
	mov	dword ptr [ebp - 24], 0
	mov	esi, dword ptr [eax + L__ZN6icrate10Foundation9generated14__NSEnumerator17NSFastEnumeration41countByEnumeratingWithState_objects_count10CACHED_SEL17h230d5b6091d37064E$non_lazy_ptr-L4$pb]
	lea	eax, [eax + l_anon.[ID].0-L4$pb]
	mov	dword ptr [ebp - 16], eax
	xor	eax, eax
	xor	ecx, ecx
	jmp	LBB4_1
	.p2align	4, 0x90
LBB4_6:
	inc	ecx
	mov	dword ptr [ebp - 28], ecx
LBB4_1:
	cmp	ecx, eax
	jb	LBB4_6
	mov	eax, dword ptr [esi]
	test	eax, eax
	jne	LBB4_4
	sub	esp, 12
	push	dword ptr [ebp - 16]
	call	SYM(objc2::runtime::Sel::register_unchecked::GENERATED_ID, 0)
	add	esp, 16
	mov	dword ptr [esi], eax
LBB4_4:
	sub	esp, 12
	push	16
	lea	ecx, [ebp - 124]
	push	ecx
	push	edi
	push	eax
	push	ebx
	call	_objc_msgSend
	add	esp, 32
	mov	dword ptr [ebp - 24], eax
	test	eax, eax
	je	LBB4_7
	xor	ecx, ecx
	mov	ebx, dword ptr [ebp - 128]
	jmp	LBB4_6
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
	sub	esp, 140
	call	L5$pb
L5$pb:
	pop	eax
	mov	esi, dword ptr [ebp + 8]
	mov	dword ptr [ebp - 44], 0
	mov	dword ptr [ebp - 48], 0
	mov	dword ptr [ebp - 36], 0
	mov	dword ptr [ebp - 40], 0
	mov	dword ptr [ebp - 32], 0
	mov	dword ptr [ebp - 128], esi
	lea	ebx, [ebp - 60]
	mov	dword ptr [ebp - 120], 0
	mov	dword ptr [ebp - 124], 0
	mov	dword ptr [ebp - 112], 0
	mov	dword ptr [ebp - 116], 0
	mov	dword ptr [ebp - 104], 0
	mov	dword ptr [ebp - 108], 0
	mov	dword ptr [ebp - 96], 0
	mov	dword ptr [ebp - 100], 0
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
	mov	dword ptr [ebp - 52], 0
	mov	dword ptr [ebp - 28], 0
	mov	dword ptr [ebp - 24], 0
	mov	edi, dword ptr [eax + L__ZN6icrate10Foundation9generated14__NSEnumerator17NSFastEnumeration41countByEnumeratingWithState_objects_count10CACHED_SEL17h230d5b6091d37064E$non_lazy_ptr-L5$pb]
	lea	eax, [eax + l_anon.[ID].0-L5$pb]
	mov	dword ptr [ebp - 16], eax
	xor	eax, eax
	xor	ecx, ecx
	jmp	LBB5_1
	.p2align	4, 0x90
LBB5_4:
	lea	ecx, [ebp - 124]
	mov	dword ptr [esp + 12], ecx
	mov	dword ptr [esp + 8], ebx
	mov	dword ptr [esp + 4], eax
	mov	dword ptr [esp], esi
	mov	dword ptr [esp + 16], 16
	call	_objc_msgSend
	mov	dword ptr [ebp - 24], eax
	xor	ecx, ecx
	test	eax, eax
	je	LBB5_5
LBB5_6:
	mov	eax, dword ptr [ebp - 56]
	lea	edx, [ecx + 1]
	mov	dword ptr [ebp - 28], edx
	mov	eax, dword ptr [eax + 4*ecx]
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	esi, eax
	mov	dword ptr [esp], eax
	call	_use_obj
	mov	dword ptr [esp], esi
	call	_objc_release
	mov	esi, dword ptr [ebp - 128]
	mov	ecx, dword ptr [ebp - 28]
	mov	eax, dword ptr [ebp - 24]
LBB5_1:
	cmp	ecx, eax
	jb	LBB5_6
	mov	eax, dword ptr [edi]
	test	eax, eax
	jne	LBB5_4
	mov	eax, dword ptr [ebp - 16]
	mov	dword ptr [esp], eax
	call	SYM(objc2::runtime::Sel::register_unchecked::GENERATED_ID, 0)
	mov	dword ptr [edi], eax
	jmp	LBB5_4
LBB5_5:
	add	esp, 140
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.section	__TEXT,__const
l_anon.[ID].0:
	.asciz	"countByEnumeratingWithState:objects:count:"

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
L__ZN6icrate10Foundation9generated14__NSEnumerator17NSFastEnumeration41countByEnumeratingWithState_objects_count10CACHED_SEL17h230d5b6091d37064E$non_lazy_ptr:
	.indirect_symbol	SYM(icrate::Foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)
	.long	0

.subsections_via_symbols
