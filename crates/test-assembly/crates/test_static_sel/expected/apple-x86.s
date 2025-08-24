	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_fn1_get_sel
	.p2align	4
_fn1_get_sel:
	push	ebp
	mov	ebp, esp
	call	L0$pb
L0$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb-L0$pb]
	pop	ebp
	ret

	.globl	_fn2_get_same_sel
	.p2align	4
_fn2_get_same_sel:
	push	ebp
	mov	ebp, esp
	call	L1$pb
L1$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f-L1$pb]
	pop	ebp
	ret

	.globl	_fn3_get_common_twice
	.p2align	4
_fn3_get_common_twice:
	push	ebp
	mov	ebp, esp
	call	L2$pb
L2$pb:
	pop	eax
	mov	eax, dword ptr [eax + LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr-L2$pb]
	mov	eax, dword ptr [eax]
	mov	edx, eax
	pop	ebp
	ret

	.globl	_fn4_get_different_sel
	.p2align	4
_fn4_get_different_sel:
	push	ebp
	mov	ebp, esp
	call	L3$pb
L3$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85-L3$pb]
	pop	ebp
	ret

	.globl	_fn5_unused_sel
	.p2align	4
_fn5_unused_sel:
	push	ebp
	mov	ebp, esp
	pop	ebp
	ret

	.globl	_fn6_use_fns
	.p2align	4
_fn6_use_fns:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	call	L5$pb
L5$pb:
	pop	ecx
	mov	eax, dword ptr [ebp + 8]
	mov	edx, dword ptr [ecx + L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb-L5$pb]
	mov	esi, dword ptr [ecx + L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f-L5$pb]
	mov	edi, dword ptr [ecx + L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85-L5$pb]
	mov	ecx, dword ptr [ecx + L_OBJC_SELECTOR_REFERENCES_e91a5376ddae3cfc-L5$pb]
	mov	dword ptr [eax], edx
	mov	dword ptr [eax + 4], esi
	mov	dword ptr [eax + 8], edi
	mov	dword ptr [eax + 12], ecx
	pop	esi
	pop	edi
	pop	ebp
	ret	4

	.globl	_fn7_use_same_twice
	.p2align	4
_fn7_use_same_twice:
	push	ebp
	mov	ebp, esp
	call	L6$pb
L6$pb:
	pop	ecx
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ecx + L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb-L6$pb]
	mov	dword ptr [eax], ecx
	mov	dword ptr [eax + 4], ecx
	pop	ebp
	ret	4

	.globl	_fn8_use_in_loop
	.p2align	4
_fn8_use_in_loop:
	push	ebp
	mov	ebp, esp
	pop	ebp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2668dedcc69bf8fb
L_OBJC_METH_VAR_NAME_2668dedcc69bf8fb:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb:
	.long	L_OBJC_METH_VAR_NAME_2668dedcc69bf8fb

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_9303807037ba4f9f
L_OBJC_METH_VAR_NAME_9303807037ba4f9f:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f:
	.long	L_OBJC_METH_VAR_NAME_9303807037ba4f9f

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_ff60dae1998e0c85
L_OBJC_METH_VAR_NAME_ff60dae1998e0c85:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85:
	.long	L_OBJC_METH_VAR_NAME_ff60dae1998e0c85

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_29d0234b9445d447
L_OBJC_METH_VAR_NAME_29d0234b9445d447:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_29d0234b9445d447
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_29d0234b9445d447:
	.long	L_OBJC_METH_VAR_NAME_29d0234b9445d447

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e91a5376ddae3cfc
L_OBJC_METH_VAR_NAME_e91a5376ddae3cfc:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_e91a5376ddae3cfc
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_e91a5376ddae3cfc:
	.long	L_OBJC_METH_VAR_NAME_e91a5376ddae3cfc

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_a23daf114eba1518
L_OBJC_METH_VAR_NAME_a23daf114eba1518:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_a23daf114eba1518
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_a23daf114eba1518:
	.long	L_OBJC_METH_VAR_NAME_a23daf114eba1518

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2668dedcc69bf8fb
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2668dedcc69bf8fb:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_9303807037ba4f9f
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_9303807037ba4f9f:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_ff60dae1998e0c85
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ff60dae1998e0c85:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_29d0234b9445d447
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_29d0234b9445d447:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_e91a5376ddae3cfc
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_e91a5376ddae3cfc:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_a23daf114eba1518
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a23daf114eba1518:
	.asciz	"\000\000\000\000@\000\000"

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_alloc
	.long	0

.subsections_via_symbols
