	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_with_sel
	.p2align	4, 0x90
_handle_with_sel:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	call	L0$pb
L0$pb:
	pop	eax
	sub	esp, 8
	push	dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc-L0$pb]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 24
	pop	ebp
	ret

	.globl	_handle_alloc_init
	.p2align	4, 0x90
_handle_alloc_init:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	call	L1$pb
L1$pb:
	pop	ebx
	mov	eax, dword ptr [ebx + LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-L1$pb]
	mov	edi, dword ptr [eax]
	mov	eax, dword ptr [ebx + LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr-L1$pb]
	sub	esp, 8
	push	dword ptr [eax]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 16
	mov	esi, eax
	sub	esp, 8
	push	edi
	push	eax
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
	call	SYM(<objc2::__macro_helpers::RetainSemantics<3_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)

	.globl	_use_generic
	.p2align	4, 0x90
_use_generic:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	call	L2$pb
L2$pb:
	pop	esi
	mov	edi, dword ptr [ebp + 8]
	sub	esp, 4
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05-L2$pb]
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_36a6e334f5aeb023-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 12
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05-L2$pb]
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_2c3c38f6ea036343-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 12
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05-L2$pb]
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_e1e97023e8bcf6a4-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	2
l_anon.[ID].1:
	.long	l_anon.[ID].0
	.asciz	"4\000\000\000\016\000\000\000\005\000\000"

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_044375a4329d08dc
	.p2align	2
L_OBJC_IMAGE_INFO_044375a4329d08dc:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_044375a4329d08dc
L_OBJC_METH_VAR_NAME_044375a4329d08dc:
	.asciz	"someSelector"

	.section	__OBJC,__message_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc:
	.long	L_OBJC_METH_VAR_NAME_044375a4329d08dc

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_5ace898e385eba05
	.p2align	2
L_OBJC_IMAGE_INFO_5ace898e385eba05:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_5ace898e385eba05
L_OBJC_METH_VAR_NAME_5ace898e385eba05:
	.asciz	"generic:selector:"

	.section	__OBJC,__message_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05:
	.long	L_OBJC_METH_VAR_NAME_5ace898e385eba05

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_36a6e334f5aeb023
	.p2align	2
L_OBJC_IMAGE_INFO_36a6e334f5aeb023:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_36a6e334f5aeb023
L_OBJC_METH_VAR_NAME_36a6e334f5aeb023:
	.asciz	"performSelector:"

	.section	__OBJC,__message_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_36a6e334f5aeb023
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_36a6e334f5aeb023:
	.long	L_OBJC_METH_VAR_NAME_36a6e334f5aeb023

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_2c3c38f6ea036343
	.p2align	2
L_OBJC_IMAGE_INFO_2c3c38f6ea036343:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2c3c38f6ea036343
L_OBJC_METH_VAR_NAME_2c3c38f6ea036343:
	.asciz	"performSelector:"

	.section	__OBJC,__message_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2c3c38f6ea036343
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_2c3c38f6ea036343:
	.long	L_OBJC_METH_VAR_NAME_2c3c38f6ea036343

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_e1e97023e8bcf6a4
	.p2align	2
L_OBJC_IMAGE_INFO_e1e97023e8bcf6a4:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e1e97023e8bcf6a4
L_OBJC_METH_VAR_NAME_e1e97023e8bcf6a4:
	.asciz	"performSelector:"

	.section	__OBJC,__message_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_e1e97023e8bcf6a4
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_e1e97023e8bcf6a4:
	.long	L_OBJC_METH_VAR_NAME_e1e97023e8bcf6a4

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_alloc
	.long	0
LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_init
	.long	0

.subsections_via_symbols
