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
	push	edi
	push	esi
	call	L1$pb
L1$pb:
	pop	eax
	mov	ecx, dword ptr [eax + LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-L1$pb]
	mov	edi, dword ptr [ecx]
	mov	eax, dword ptr [eax + LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr-L1$pb]
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
	pop	esi
	pop	edi
	pop	ebp
	ret
LBB1_2:
	sub	esp, 8
	push	edi
	push	esi
	call	SYM(objc2::__macro_helpers::init_failed::GENERATED_ID, 0)

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

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_044375a4329d08dc
	.p2align	2
L_OBJC_IMAGE_INFO_044375a4329d08dc:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_044375a4329d08dc
L_OBJC_METH_VAR_NAME_044375a4329d08dc:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc:
	.long	L_OBJC_METH_VAR_NAME_044375a4329d08dc

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_5ace898e385eba05
	.p2align	2
L_OBJC_IMAGE_INFO_5ace898e385eba05:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_5ace898e385eba05
L_OBJC_METH_VAR_NAME_5ace898e385eba05:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05:
	.long	L_OBJC_METH_VAR_NAME_5ace898e385eba05

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_36a6e334f5aeb023
	.p2align	2
L_OBJC_IMAGE_INFO_36a6e334f5aeb023:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_36a6e334f5aeb023
L_OBJC_METH_VAR_NAME_36a6e334f5aeb023:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_36a6e334f5aeb023
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_36a6e334f5aeb023:
	.long	L_OBJC_METH_VAR_NAME_36a6e334f5aeb023

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2c3c38f6ea036343
	.p2align	2
L_OBJC_IMAGE_INFO_2c3c38f6ea036343:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2c3c38f6ea036343
L_OBJC_METH_VAR_NAME_2c3c38f6ea036343:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2c3c38f6ea036343
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_2c3c38f6ea036343:
	.long	L_OBJC_METH_VAR_NAME_2c3c38f6ea036343

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_e1e97023e8bcf6a4
	.p2align	2
L_OBJC_IMAGE_INFO_e1e97023e8bcf6a4:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e1e97023e8bcf6a4
L_OBJC_METH_VAR_NAME_e1e97023e8bcf6a4:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
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
