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
	push	dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_80f1580ed33ec51b-L0$pb]
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
	pop	edi
	mov	eax, dword ptr [edi + LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr-L1$pb]
	sub	esp, 8
	push	dword ptr [eax]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 16
	mov	esi, eax
	mov	eax, dword ptr [edi + LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-L1$pb]
	mov	ebx, dword ptr [eax]
	sub	esp, 8
	push	ebx
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
	lea	eax, [edi + l_anon.[ID].1-L1$pb]
	push	eax
	push	ebx
	push	esi
	call	SYM(objc2::__macro_helpers::retain_semantics::init_fail::GENERATED_ID, 0)

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
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5-L2$pb]
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_67bf3e41c7e639a3-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 12
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5-L2$pb]
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_2c2c9a8191012941-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 12
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5-L2$pb]
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_993d94b40d47ed52-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].1:
	.long	l_anon.[ID].0
	.asciz	";\000\000\000\016\000\000\000\005\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_80f1580ed33ec51b
L_OBJC_METH_VAR_NAME_80f1580ed33ec51b:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_80f1580ed33ec51b
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_80f1580ed33ec51b:
	.long	L_OBJC_METH_VAR_NAME_80f1580ed33ec51b

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_80f1580ed33ec51b
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_80f1580ed33ec51b:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_91c006d97540f4b5
L_OBJC_METH_VAR_NAME_91c006d97540f4b5:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5:
	.long	L_OBJC_METH_VAR_NAME_91c006d97540f4b5

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_91c006d97540f4b5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_91c006d97540f4b5:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_67bf3e41c7e639a3
L_OBJC_METH_VAR_NAME_67bf3e41c7e639a3:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_67bf3e41c7e639a3
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_67bf3e41c7e639a3:
	.long	L_OBJC_METH_VAR_NAME_67bf3e41c7e639a3

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_67bf3e41c7e639a3
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_67bf3e41c7e639a3:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2c2c9a8191012941
L_OBJC_METH_VAR_NAME_2c2c9a8191012941:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_2c2c9a8191012941
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_2c2c9a8191012941:
	.long	L_OBJC_METH_VAR_NAME_2c2c9a8191012941

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2c2c9a8191012941
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2c2c9a8191012941:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_993d94b40d47ed52
L_OBJC_METH_VAR_NAME_993d94b40d47ed52:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_993d94b40d47ed52
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_993d94b40d47ed52:
	.long	L_OBJC_METH_VAR_NAME_993d94b40d47ed52

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_993d94b40d47ed52
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_993d94b40d47ed52:
	.asciz	"\000\000\000\000@\000\000"

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_alloc
	.long	0
LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_init
	.long	0

.subsections_via_symbols
