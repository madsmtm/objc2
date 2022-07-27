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
	mov	eax, dword ptr [eax + L_OBJC_CLASSLIST_REFERENCES_$_e0796f2f86586929-L0$pb]
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
	mov	eax, dword ptr [eax + L_OBJC_CLASSLIST_REFERENCES_$_1121b6b8519ae283-L1$pb]
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
	mov	eax, dword ptr [eax + L_OBJC_CLASSLIST_REFERENCES_$_7314246f93b118b1-L2$pb]
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
	mov	edx, dword ptr [ecx + L_OBJC_CLASSLIST_REFERENCES_$_e0796f2f86586929-L4$pb]
	mov	esi, dword ptr [ecx + L_OBJC_CLASSLIST_REFERENCES_$_1121b6b8519ae283-L4$pb]
	mov	edi, dword ptr [ecx + L_OBJC_CLASSLIST_REFERENCES_$_7314246f93b118b1-L4$pb]
	mov	ecx, dword ptr [ecx + L_OBJC_CLASSLIST_REFERENCES_$_16e72be3e810c687-L4$pb]
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
	mov	ecx, dword ptr [ecx + L_OBJC_CLASSLIST_REFERENCES_$_e0796f2f86586929-L5$pb]
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
	.globl	L_OBJC_IMAGE_INFO_e0796f2f86586929
	.p2align	2
L_OBJC_IMAGE_INFO_e0796f2f86586929:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_e0796f2f86586929
	.p2align	2
L_OBJC_CLASSLIST_REFERENCES_$_e0796f2f86586929:
	.long	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_1121b6b8519ae283
	.p2align	2
L_OBJC_IMAGE_INFO_1121b6b8519ae283:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_1121b6b8519ae283
	.p2align	2
L_OBJC_CLASSLIST_REFERENCES_$_1121b6b8519ae283:
	.long	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_7314246f93b118b1
	.p2align	2
L_OBJC_IMAGE_INFO_7314246f93b118b1:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_7314246f93b118b1
	.p2align	2
L_OBJC_CLASSLIST_REFERENCES_$_7314246f93b118b1:
	.long	_OBJC_CLASS_$_NSString

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f5b45e093b02d6c9
	.p2align	2
L_OBJC_IMAGE_INFO_f5b45e093b02d6c9:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_f5b45e093b02d6c9
	.p2align	2
L_OBJC_CLASSLIST_REFERENCES_$_f5b45e093b02d6c9:
	.long	_OBJC_CLASS_$_NSData

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_16e72be3e810c687
	.p2align	2
L_OBJC_IMAGE_INFO_16e72be3e810c687:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_16e72be3e810c687
	.p2align	2
L_OBJC_CLASSLIST_REFERENCES_$_16e72be3e810c687:
	.long	_OBJC_CLASS_$_NSException

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_4465b4c2c67ae674
	.p2align	2
L_OBJC_IMAGE_INFO_4465b4c2c67ae674:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_4465b4c2c67ae674
	.p2align	2
L_OBJC_CLASSLIST_REFERENCES_$_4465b4c2c67ae674:
	.long	_OBJC_CLASS_$_NSLock

.subsections_via_symbols
