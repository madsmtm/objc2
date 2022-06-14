	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_alloc
	.p2align	4, 0x90
_handle_alloc:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L0$pb
L0$pb:
	pop	esi
	sub	esp, 8
	push	dword ptr [ebp + 12]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB0_2
	add	esp, 4
	pop	esi
	pop	ebp
	ret
LBB0_2:
	sub	esp, 4
	lea	eax, [esi + l___unnamed_1-L0$pb]
	lea	ecx, [esi + l___unnamed_2-L0$pb]
	push	eax
	push	17
	push	ecx
	call	__ZN4core6option13expect_failed17hc70f05bfe703751eE

	.globl	_handle_init
	.p2align	4, 0x90
_handle_init:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	_objc_msgSend

	.globl	_handle_alloc_init
	.p2align	4, 0x90
_handle_alloc_init:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L2$pb
L2$pb:
	pop	esi
	sub	esp, 8
	push	dword ptr [ebp + 12]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB2_2
	sub	esp, 8
	push	dword ptr [ebp + 16]
	push	eax
	call	_objc_msgSend
	add	esp, 20
	pop	esi
	pop	ebp
	ret
LBB2_2:
	sub	esp, 4
	lea	eax, [esi + l___unnamed_1-L2$pb]
	lea	ecx, [esi + l___unnamed_2-L2$pb]
	push	eax
	push	17
	push	ecx
	call	__ZN4core6option13expect_failed17hc70f05bfe703751eE

	.globl	_handle_alloc_release
	.p2align	4, 0x90
_handle_alloc_release:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L3$pb
L3$pb:
	pop	esi
	sub	esp, 8
	push	dword ptr [ebp + 12]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB3_2
	sub	esp, 12
	push	eax
	call	_objc_release
	add	esp, 20
	pop	esi
	pop	ebp
	ret
LBB3_2:
	sub	esp, 4
	lea	eax, [esi + l___unnamed_1-L3$pb]
	lea	ecx, [esi + l___unnamed_2-L3$pb]
	push	eax
	push	17
	push	ecx
	call	__ZN4core6option13expect_failed17hc70f05bfe703751eE

	.globl	_handle_alloc_init_release
	.p2align	4, 0x90
_handle_alloc_init_release:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L4$pb
L4$pb:
	pop	esi
	sub	esp, 8
	push	dword ptr [ebp + 12]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB4_2
	sub	esp, 8
	push	dword ptr [ebp + 16]
	push	eax
	call	_objc_msgSend
	add	esp, 4
	push	eax
	call	_objc_release
	add	esp, 20
	pop	esi
	pop	ebp
	ret
LBB4_2:
	sub	esp, 4
	lea	eax, [esi + l___unnamed_1-L4$pb]
	lea	ecx, [esi + l___unnamed_2-L4$pb]
	push	eax
	push	17
	push	ecx
	call	__ZN4core6option13expect_failed17hc70f05bfe703751eE

	.globl	_handle_copy
	.p2align	4, 0x90
_handle_copy:
	push	ebp
	mov	ebp, esp
	pop	ebp
	jmp	_objc_msgSend

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

	.section	__TEXT,__const
l___unnamed_2:
	.ascii	"Failed allocating"

l___unnamed_3:
	.ascii	"$WORKSPACE/objc2/src/__macro_helpers.rs"

	.section	__DATA,__const
	.p2align	2
l___unnamed_1:
	.long	l___unnamed_3
	.asciz	"B\000\000\000\037\000\000\000%\000\000"

.subsections_via_symbols
