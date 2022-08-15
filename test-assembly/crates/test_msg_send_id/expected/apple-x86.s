	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_new
	.p2align	4, 0x90
_handle_new:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	_objc_msgSend

	.globl	_handle_new_fallible
	.p2align	4, 0x90
_handle_new_fallible:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	mov	edi, dword ptr [ebp + 12]
	mov	esi, dword ptr [ebp + 8]
	sub	esp, 8
	push	edi
	push	esi
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB1_2
	pop	esi
	pop	edi
	pop	ebp
	ret
LBB1_2:
	sub	esp, 8
	push	edi
	push	esi
	call	SYM(objc2::__macro_helpers::new_failed::GENERATED_ID, 0)

	.globl	_handle_alloc
	.p2align	4, 0x90
_handle_alloc:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	_objc_msgSend

	.globl	_handle_alloc_fallible
	.p2align	4, 0x90
_handle_alloc_fallible:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	mov	edi, dword ptr [ebp + 12]
	mov	esi, dword ptr [ebp + 8]
	sub	esp, 8
	push	edi
	push	esi
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB3_2
	pop	esi
	pop	edi
	pop	ebp
	ret
LBB3_2:
	sub	esp, 8
	push	edi
	push	esi
	call	SYM(objc2::__macro_helpers::alloc_failed::GENERATED_ID, 0)

	.globl	_handle_init
	.p2align	4, 0x90
_handle_init:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	_objc_msgSend

	.globl	_handle_init_fallible
	.p2align	4, 0x90
_handle_init_fallible:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	mov	edi, dword ptr [ebp + 12]
	mov	esi, dword ptr [ebp + 8]
	sub	esp, 8
	push	edi
	push	esi
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB5_2
	pop	esi
	pop	edi
	pop	ebp
	ret
LBB5_2:
	sub	esp, 8
	push	edi
	push	esi
	call	SYM(objc2::__macro_helpers::init_failed::GENERATED_ID, 0)

	.globl	_handle_alloc_init
	.p2align	4, 0x90
_handle_alloc_init:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	mov	esi, dword ptr [ebp + 16]
	sub	esp, 8
	push	dword ptr [ebp + 12]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 8
	push	esi
	push	eax
	call	_objc_msgSend
	add	esp, 20
	pop	esi
	pop	ebp
	ret

	.globl	_handle_alloc_release
	.p2align	4, 0x90
_handle_alloc_release:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	mov	dword ptr [esp], eax
	call	_objc_release
	add	esp, 8
	pop	ebp
	ret

	.globl	_handle_alloc_init_release
	.p2align	4, 0x90
_handle_alloc_init_release:
	push	ebp
	mov	ebp, esp
	push	esi
	sub	esp, 20
	mov	esi, dword ptr [ebp + 16]
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	mov	dword ptr [esp], eax
	call	_objc_release
	add	esp, 20
	pop	esi
	pop	ebp
	ret

	.globl	_handle_copy
	.p2align	4, 0x90
_handle_copy:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	_objc_msgSend

	.globl	_handle_copy_fallible
	.p2align	4, 0x90
_handle_copy_fallible:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	sub	esp, 8
	push	dword ptr [ebp + 12]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB10_2
	add	esp, 8
	pop	ebp
	ret
LBB10_2:
	call	SYM(objc2::__macro_helpers::copy_failed::GENERATED_ID, 0)

	.globl	_handle_autoreleased
	.p2align	4, 0x90
_handle_autoreleased:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	## InlineAsm Start

	mov	ebp, ebp

	## InlineAsm End
	mov	dword ptr [esp], eax
	call	_objc_retainAutoreleasedReturnValue
	add	esp, 8
	pop	ebp
	ret

	.globl	_handle_autoreleased_fallible
	.p2align	4, 0x90
_handle_autoreleased_fallible:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	sub	esp, 16
	mov	edi, dword ptr [ebp + 12]
	mov	esi, dword ptr [ebp + 8]
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	_objc_msgSend
	## InlineAsm Start

	mov	ebp, ebp

	## InlineAsm End
	mov	dword ptr [esp], eax
	call	_objc_retainAutoreleasedReturnValue
	test	eax, eax
	je	LBB12_2
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret
LBB12_2:
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	SYM(objc2::__macro_helpers::normal_failed::GENERATED_ID, 0)

.subsections_via_symbols
