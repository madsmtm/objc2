	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_fn1_get_class
	.p2align	4
_fn1_get_class:
	push	ebp
	mov	ebp, esp
	call	L0$pb
L0$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34-L0$pb]
	pop	ebp
	ret

	.globl	_fn1_get_same_class
	.p2align	4
_fn1_get_same_class:
	push	ebp
	mov	ebp, esp
	call	L1$pb
L1$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a-L1$pb]
	pop	ebp
	ret

	.globl	_fn3_get_different_class
	.p2align	4
_fn3_get_different_class:
	push	ebp
	mov	ebp, esp
	call	L2$pb
L2$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1-L2$pb]
	pop	ebp
	ret

	.globl	_fn4_unused_class
	.p2align	4
_fn4_unused_class:
	push	ebp
	mov	ebp, esp
	pop	ebp
	ret

	.globl	_fn5_use_fns
	.p2align	4
_fn5_use_fns:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	call	L4$pb
L4$pb:
	pop	ecx
	mov	eax, dword ptr [ebp + 8]
	mov	edx, dword ptr [ecx + L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34-L4$pb]
	mov	esi, dword ptr [ecx + L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a-L4$pb]
	mov	edi, dword ptr [ecx + L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1-L4$pb]
	mov	ecx, dword ptr [ecx + L_OBJC_CLASSLIST_REFERENCES_$_a92f01d3b55d29c5-L4$pb]
	mov	dword ptr [eax], edx
	mov	dword ptr [eax + 4], esi
	mov	dword ptr [eax + 8], edi
	mov	dword ptr [eax + 12], ecx
	pop	esi
	pop	edi
	pop	ebp
	ret	4

	.globl	_fn6_use_same_twice
	.p2align	4
_fn6_use_same_twice:
	push	ebp
	mov	ebp, esp
	call	L5$pb
L5$pb:
	pop	ecx
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ecx + L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34-L5$pb]
	mov	dword ptr [eax], ecx
	mov	dword ptr [eax + 4], ecx
	pop	ebp
	ret	4

	.globl	_fn7_use_in_loop
	.p2align	4
_fn7_use_in_loop:
	push	ebp
	mov	ebp, esp
	pop	ebp
	ret

	.section	__DATA,__objc_classrefs
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34:
	.long	_OBJC_CLASS_$_NSObject

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a:
	.long	_OBJC_CLASS_$_NSObject

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1:
	.long	_OBJC_CLASS_$_NSString

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_9f503c7582f87b48
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_9f503c7582f87b48:
	.long	_OBJC_CLASS_$_NSData

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_a92f01d3b55d29c5
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_a92f01d3b55d29c5:
	.long	_OBJC_CLASS_$_NSException

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_d4ef9efb3ee49ab7
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_d4ef9efb3ee49ab7:
	.long	_OBJC_CLASS_$_NSLock

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_1f36dafa1e0a7b34
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_1f36dafa1e0a7b34:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_e1a6d3426ab3be5a
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_e1a6d3426ab3be5a:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_5a6ce274a9f949e1
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_5a6ce274a9f949e1:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_9f503c7582f87b48
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_9f503c7582f87b48:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_a92f01d3b55d29c5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a92f01d3b55d29c5:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_d4ef9efb3ee49ab7
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_d4ef9efb3ee49ab7:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
