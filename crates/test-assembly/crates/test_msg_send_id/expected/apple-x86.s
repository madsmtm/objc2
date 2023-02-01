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
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	call	L1$pb
L1$pb:
	pop	ebx
	mov	edi, dword ptr [ebp + 12]
	mov	esi, dword ptr [ebp + 8]
	sub	esp, 8
	push	edi
	push	esi
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB1_2
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB1_2:
	sub	esp, 4
	lea	eax, [ebx + l_anon.[ID].1-L1$pb]
	push	eax
	push	edi
	push	esi
	call	SYM(<objc2::__macro_helpers::RetainSemantics<1_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)

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
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	call	L3$pb
L3$pb:
	pop	ebx
	mov	edi, dword ptr [ebp + 12]
	mov	esi, dword ptr [ebp + 8]
	sub	esp, 8
	push	edi
	push	esi
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB3_2
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB3_2:
	sub	esp, 4
	lea	eax, [ebx + l_anon.[ID].2-L3$pb]
	push	eax
	push	edi
	push	esi
	call	SYM(<objc2::__macro_helpers::RetainSemantics<2_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)

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
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	call	L5$pb
L5$pb:
	pop	ebx
	mov	edi, dword ptr [ebp + 12]
	mov	esi, dword ptr [ebp + 8]
	sub	esp, 8
	push	edi
	push	esi
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB5_2
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB5_2:
	sub	esp, 4
	lea	eax, [ebx + l_anon.[ID].3-L5$pb]
	push	eax
	push	edi
	push	esi
	call	SYM(<objc2::__macro_helpers::RetainSemantics<3_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)

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
	push	esi
	sub	esp, 20
	call	L10$pb
L10$pb:
	pop	esi
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	test	eax, eax
	je	LBB10_2
	add	esp, 20
	pop	esi
	pop	ebp
	ret
LBB10_2:
	lea	eax, [esi + l_anon.[ID].4-L10$pb]
	mov	dword ptr [esp], eax
	call	SYM(<objc2::__macro_helpers::RetainSemantics<4_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)

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
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	call	L12$pb
L12$pb:
	pop	ebx
	mov	edi, dword ptr [ebp + 12]
	mov	esi, dword ptr [ebp + 8]
	sub	esp, 8
	push	edi
	push	esi
	call	_objc_msgSend
	add	esp, 16
	## InlineAsm Start

	mov	ebp, ebp

	## InlineAsm End
	sub	esp, 12
	push	eax
	call	_objc_retainAutoreleasedReturnValue
	add	esp, 16
	test	eax, eax
	je	LBB12_2
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB12_2:
	sub	esp, 4
	lea	eax, [ebx + l_anon.[ID].5-L12$pb]
	push	eax
	push	edi
	push	esi
	call	SYM(<objc2::__macro_helpers::RetainSemantics<5_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)

	.globl	_handle_with_out_param
	.p2align	4, 0x90
_handle_with_out_param:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	ebx, dword ptr [ebp + 16]
	mov	edi, dword ptr [ebx]
	mov	dword ptr [esp + 8], ebx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	mov	esi, eax
	mov	eax, dword ptr [ebx]
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	dword ptr [esp], edi
	call	_objc_release
	## InlineAsm Start

	mov	ebp, ebp

	## InlineAsm End
	mov	dword ptr [esp], esi
	call	_objc_retainAutoreleasedReturnValue
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	2
l_anon.[ID].1:
	.long	l_anon.[ID].0
	.asciz	"3\000\000\000\r\000\000\000\005\000\000"

	.p2align	2
l_anon.[ID].2:
	.long	l_anon.[ID].0
	.asciz	"3\000\000\000\027\000\000\000\005\000\000"

	.p2align	2
l_anon.[ID].3:
	.long	l_anon.[ID].0
	.asciz	"3\000\000\000!\000\000\000\005\000\000"

	.p2align	2
l_anon.[ID].4:
	.long	l_anon.[ID].0
	.asciz	"3\000\000\000>\000\000\000\005\000\000"

	.p2align	2
l_anon.[ID].5:
	.long	l_anon.[ID].0
	.asciz	"3\000\000\000H\000\000\000\005\000\000"

.subsections_via_symbols
