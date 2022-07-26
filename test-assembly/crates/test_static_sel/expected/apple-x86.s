	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_sel
	.p2align	4, 0x90
_get_sel:
	push	ebp
	mov	ebp, esp
	call	L0$pb
L0$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0-L0$pb]
	pop	ebp
	ret

	.globl	_get_same_sel
	.p2align	4, 0x90
_get_same_sel:
	push	ebp
	mov	ebp, esp
	call	L1$pb
L1$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83-L1$pb]
	pop	ebp
	ret

	.globl	_get_common_twice
	.p2align	4, 0x90
_get_common_twice:
	push	ebp
	mov	ebp, esp
	call	L2$pb
L2$pb:
	pop	eax
	mov	eax, dword ptr [eax + LL_OBJC_SELECTOR_REFERENCES_1678d2f7468155d2$non_lazy_ptr-L2$pb]
	mov	eax, dword ptr [eax]
	mov	edx, eax
	pop	ebp
	ret

	.globl	_get_different_sel
	.p2align	4, 0x90
_get_different_sel:
	push	ebp
	mov	ebp, esp
	call	L3$pb
L3$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_25911857653c680c-L3$pb]
	pop	ebp
	ret

	.globl	_unused_sel
	.p2align	4, 0x90
_unused_sel:
	push	ebp
	mov	ebp, esp
	pop	ebp
	ret

	.globl	_use_fns
	.p2align	4, 0x90
_use_fns:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	call	L5$pb
L5$pb:
	pop	ecx
	mov	eax, dword ptr [ebp + 8]
	mov	edx, dword ptr [ecx + L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0-L5$pb]
	mov	esi, dword ptr [ecx + L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83-L5$pb]
	mov	edi, dword ptr [ecx + L_OBJC_SELECTOR_REFERENCES_25911857653c680c-L5$pb]
	mov	ecx, dword ptr [ecx + L_OBJC_SELECTOR_REFERENCES_acb291d82e56f534-L5$pb]
	mov	dword ptr [eax], edx
	mov	dword ptr [eax + 4], esi
	mov	dword ptr [eax + 8], edi
	mov	dword ptr [eax + 12], ecx
	pop	esi
	pop	edi
	pop	ebp
	ret	4

	.globl	_use_same_twice
	.p2align	4, 0x90
_use_same_twice:
	push	ebp
	mov	ebp, esp
	call	L6$pb
L6$pb:
	pop	ecx
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ecx + L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0-L6$pb]
	mov	dword ptr [eax], ecx
	mov	dword ptr [eax + 4], ecx
	pop	ebp
	ret	4

	.globl	_use_in_loop
	.p2align	4, 0x90
_use_in_loop:
	push	ebp
	mov	ebp, esp
	pop	ebp
	ret

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2ff5c2d33acc98c0
	.p2align	2
L_OBJC_IMAGE_INFO_2ff5c2d33acc98c0:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2ff5c2d33acc98c0
L_OBJC_METH_VAR_NAME_2ff5c2d33acc98c0:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0:
	.long	L_OBJC_METH_VAR_NAME_2ff5c2d33acc98c0

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_6e17eb9d3fa7fa83
	.p2align	2
L_OBJC_IMAGE_INFO_6e17eb9d3fa7fa83:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6e17eb9d3fa7fa83
L_OBJC_METH_VAR_NAME_6e17eb9d3fa7fa83:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83:
	.long	L_OBJC_METH_VAR_NAME_6e17eb9d3fa7fa83

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_25911857653c680c
	.p2align	2
L_OBJC_IMAGE_INFO_25911857653c680c:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_25911857653c680c
L_OBJC_METH_VAR_NAME_25911857653c680c:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_25911857653c680c
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_25911857653c680c:
	.long	L_OBJC_METH_VAR_NAME_25911857653c680c

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_baa3c09478169afc
	.p2align	2
L_OBJC_IMAGE_INFO_baa3c09478169afc:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_baa3c09478169afc
L_OBJC_METH_VAR_NAME_baa3c09478169afc:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_baa3c09478169afc
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_baa3c09478169afc:
	.long	L_OBJC_METH_VAR_NAME_baa3c09478169afc

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_acb291d82e56f534
	.p2align	2
L_OBJC_IMAGE_INFO_acb291d82e56f534:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_acb291d82e56f534
L_OBJC_METH_VAR_NAME_acb291d82e56f534:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_acb291d82e56f534
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_acb291d82e56f534:
	.long	L_OBJC_METH_VAR_NAME_acb291d82e56f534

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_c831c01ba82dcc2e
	.p2align	2
L_OBJC_IMAGE_INFO_c831c01ba82dcc2e:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_c831c01ba82dcc2e
L_OBJC_METH_VAR_NAME_c831c01ba82dcc2e:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_c831c01ba82dcc2e
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_c831c01ba82dcc2e:
	.long	L_OBJC_METH_VAR_NAME_c831c01ba82dcc2e

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
LL_OBJC_SELECTOR_REFERENCES_1678d2f7468155d2$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_1678d2f7468155d2
	.long	0

.subsections_via_symbols
