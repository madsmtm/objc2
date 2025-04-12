	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_sel
	.p2align	4
_get_sel:
	push	ebp
	mov	ebp, esp
	call	L0$pb
L0$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7-L0$pb]
	pop	ebp
	ret

	.globl	_get_same_sel
	.p2align	4
_get_same_sel:
	push	ebp
	mov	ebp, esp
	call	L1$pb
L1$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513-L1$pb]
	pop	ebp
	ret

	.globl	_get_common_twice
	.p2align	4
_get_common_twice:
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

	.globl	_get_different_sel
	.p2align	4
_get_different_sel:
	push	ebp
	mov	ebp, esp
	call	L3$pb
L3$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864-L3$pb]
	pop	ebp
	ret

	.globl	_unused_sel
	.p2align	4
_unused_sel:
	push	ebp
	mov	ebp, esp
	pop	ebp
	ret

	.globl	_use_fns
	.p2align	4
_use_fns:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	call	L5$pb
L5$pb:
	pop	ecx
	mov	eax, dword ptr [ebp + 8]
	mov	edx, dword ptr [ecx + L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7-L5$pb]
	mov	esi, dword ptr [ecx + L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513-L5$pb]
	mov	edi, dword ptr [ecx + L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864-L5$pb]
	mov	ecx, dword ptr [ecx + L_OBJC_SELECTOR_REFERENCES_408f5be8f4fd2627-L5$pb]
	mov	dword ptr [eax], edx
	mov	dword ptr [eax + 4], esi
	mov	dword ptr [eax + 8], edi
	mov	dword ptr [eax + 12], ecx
	pop	esi
	pop	edi
	pop	ebp
	ret	4

	.globl	_use_same_twice
	.p2align	4
_use_same_twice:
	push	ebp
	mov	ebp, esp
	call	L6$pb
L6$pb:
	pop	ecx
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ecx + L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7-L6$pb]
	mov	dword ptr [eax], ecx
	mov	dword ptr [eax + 4], ecx
	pop	ebp
	ret	4

	.globl	_use_in_loop
	.p2align	4
_use_in_loop:
	push	ebp
	mov	ebp, esp
	pop	ebp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_caedaca3f40015a7
L_OBJC_METH_VAR_NAME_caedaca3f40015a7:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7:
	.long	L_OBJC_METH_VAR_NAME_caedaca3f40015a7

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_a7c7f3067f40b513
L_OBJC_METH_VAR_NAME_a7c7f3067f40b513:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513:
	.long	L_OBJC_METH_VAR_NAME_a7c7f3067f40b513

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_bae8570d40d73864
L_OBJC_METH_VAR_NAME_bae8570d40d73864:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864:
	.long	L_OBJC_METH_VAR_NAME_bae8570d40d73864

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_9c1b77e8cf40622d
L_OBJC_METH_VAR_NAME_9c1b77e8cf40622d:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_9c1b77e8cf40622d
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_9c1b77e8cf40622d:
	.long	L_OBJC_METH_VAR_NAME_9c1b77e8cf40622d

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_408f5be8f4fd2627
L_OBJC_METH_VAR_NAME_408f5be8f4fd2627:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_408f5be8f4fd2627
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_408f5be8f4fd2627:
	.long	L_OBJC_METH_VAR_NAME_408f5be8f4fd2627

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_82483a8131827890
L_OBJC_METH_VAR_NAME_82483a8131827890:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_82483a8131827890
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_82483a8131827890:
	.long	L_OBJC_METH_VAR_NAME_82483a8131827890

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_caedaca3f40015a7
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_caedaca3f40015a7:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_a7c7f3067f40b513
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a7c7f3067f40b513:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_bae8570d40d73864
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_bae8570d40d73864:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_9c1b77e8cf40622d
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_9c1b77e8cf40622d:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_408f5be8f4fd2627
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_408f5be8f4fd2627:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_82483a8131827890
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_82483a8131827890:
	.asciz	"\000\000\000\000@\000\000"

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_alloc
	.long	0

.subsections_via_symbols
