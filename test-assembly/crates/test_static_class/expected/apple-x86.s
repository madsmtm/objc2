	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_class
	.p2align	4, 0x90
_get_class:
	push	ebp
	mov	ebp, esp
	call	L0$pb
L0$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f-L0$pb]
	pop	ebp
	ret

	.globl	_get_same_class
	.p2align	4, 0x90
_get_same_class:
	push	ebp
	mov	ebp, esp
	call	L1$pb
L1$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79-L1$pb]
	pop	ebp
	ret

	.globl	_get_different_class
	.p2align	4, 0x90
_get_different_class:
	push	ebp
	mov	ebp, esp
	call	L2$pb
L2$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4-L2$pb]
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
	call	L4$pb
L4$pb:
	pop	ecx
	mov	eax, dword ptr [ebp + 8]
	mov	edx, dword ptr [ecx + L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f-L4$pb]
	mov	esi, dword ptr [ecx + L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79-L4$pb]
	mov	edi, dword ptr [ecx + L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4-L4$pb]
	mov	ecx, dword ptr [ecx + L_OBJC_CLASSLIST_REFERENCES_$_2f45d8445f72bd9b-L4$pb]
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
	call	L5$pb
L5$pb:
	pop	ecx
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ecx + L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f-L5$pb]
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
	.globl	L_OBJC_IMAGE_INFO_9657804a2a54ab6f
	.p2align	2
L_OBJC_IMAGE_INFO_9657804a2a54ab6f:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f
	.p2align	2
L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f:
	.long	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_e897a41b218dcf79
	.p2align	2
L_OBJC_IMAGE_INFO_e897a41b218dcf79:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79
	.p2align	2
L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79:
	.long	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_3b7780b4dcfcb9d4
	.p2align	2
L_OBJC_IMAGE_INFO_3b7780b4dcfcb9d4:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4
	.p2align	2
L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4:
	.long	_OBJC_CLASS_$_NSString

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f2fb7c579d3c0a74
	.p2align	2
L_OBJC_IMAGE_INFO_f2fb7c579d3c0a74:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_f2fb7c579d3c0a74
	.p2align	2
L_OBJC_CLASSLIST_REFERENCES_$_f2fb7c579d3c0a74:
	.long	_OBJC_CLASS_$_NSData

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2f45d8445f72bd9b
	.p2align	2
L_OBJC_IMAGE_INFO_2f45d8445f72bd9b:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_2f45d8445f72bd9b
	.p2align	2
L_OBJC_CLASSLIST_REFERENCES_$_2f45d8445f72bd9b:
	.long	_OBJC_CLASS_$_NSException

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_3bf610c78df2b6bb
	.p2align	2
L_OBJC_IMAGE_INFO_3bf610c78df2b6bb:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_3bf610c78df2b6bb
	.p2align	2
L_OBJC_CLASSLIST_REFERENCES_$_3bf610c78df2b6bb:
	.long	_OBJC_CLASS_$_NSLock

.subsections_via_symbols