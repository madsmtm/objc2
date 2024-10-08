	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_always
	.p2align	4, 0x90
_always:
	push	ebp
	mov	ebp, esp
	mov	al, 1
	pop	ebp
	ret

	.globl	_never
	.p2align	4, 0x90
_never:
	push	ebp
	mov	ebp, esp
	xor	eax, eax
	pop	ebp
	ret

	.globl	_low
	.p2align	4, 0x90
_low:
	push	ebp
	mov	ebp, esp
	mov	al, 1
	pop	ebp
	ret

	.globl	_high
	.p2align	4, 0x90
_high:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L3$pb
L3$pb:
	pop	eax
	mov	esi, dword ptr [eax + LSYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)$non_lazy_ptr-L3$pb]
	mov	eax, dword ptr [esi]
	test	eax, eax
	je	LBB3_1
LBB3_2:
	cmp	eax, 1179648
	setae	al
	add	esp, 4
	pop	esi
	pop	ebp
	ret
LBB3_1:
	call	SYM(objc2::__macro_helpers::os_version::apple::lookup_version::GENERATED_ID, 0)
	mov	dword ptr [esi], eax
	jmp	LBB3_2

	.globl	_only_ios
	.p2align	4, 0x90
_only_ios:
	push	ebp
	mov	ebp, esp
	mov	al, 1
	pop	ebp
	ret

	.globl	_two_checks
	.p2align	4, 0x90
_two_checks:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	call	L5$pb
L5$pb:
	pop	eax
	mov	edi, dword ptr [eax + LSYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)$non_lazy_ptr-L5$pb]
	mov	esi, dword ptr [edi]
	test	esi, esi
	je	LBB5_1
	mov	eax, dword ptr [edi]
	test	eax, eax
	je	LBB5_3
LBB5_4:
	cmp	esi, 1114112
	setae	cl
	cmp	eax, 1179648
	setae	al
	and	al, cl
	pop	esi
	pop	edi
	pop	ebp
	ret
LBB5_1:
	call	SYM(objc2::__macro_helpers::os_version::apple::lookup_version::GENERATED_ID, 0)
	mov	esi, eax
	mov	dword ptr [edi], eax
	mov	eax, dword ptr [edi]
	test	eax, eax
	jne	LBB5_4
LBB5_3:
	call	SYM(objc2::__macro_helpers::os_version::apple::lookup_version::GENERATED_ID, 0)
	mov	dword ptr [edi], eax
	jmp	LBB5_4

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
LSYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)$non_lazy_ptr:
	.indirect_symbol	SYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)
	.long	0

.subsections_via_symbols
