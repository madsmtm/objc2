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
	mov	eax, dword ptr [eax + L_OBJC_CLASS_REFERENCES_49a0cd2a35b9a474-L0$pb]
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
	mov	eax, dword ptr [eax + L_OBJC_CLASS_REFERENCES_60860b498061fbc6-L1$pb]
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
	mov	eax, dword ptr [eax + L_OBJC_CLASS_REFERENCES_089cee9fe04089a4-L2$pb]
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
	mov	edx, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_49a0cd2a35b9a474-L4$pb]
	mov	esi, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_60860b498061fbc6-L4$pb]
	mov	edi, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_089cee9fe04089a4-L4$pb]
	mov	ecx, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_15b3f8b356e4fdb3-L4$pb]
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
	mov	ecx, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_49a0cd2a35b9a474-L5$pb]
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

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_49a0cd2a35b9a474
	.p2align	2
L_OBJC_IMAGE_INFO_49a0cd2a35b9a474:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_49a0cd2a35b9a474
L_OBJC_CLASS_NAME_49a0cd2a35b9a474:
	.ascii	"NSObject"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_49a0cd2a35b9a474
	.p2align	2
L_OBJC_CLASS_REFERENCES_49a0cd2a35b9a474:
	.long	L_OBJC_CLASS_NAME_49a0cd2a35b9a474

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_49a0cd2a35b9a474_MODULE_INFO
L_OBJC_CLASS_NAME_49a0cd2a35b9a474_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_49a0cd2a35b9a474
	.p2align	2
L_OBJC_MODULES_49a0cd2a35b9a474:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_49a0cd2a35b9a474_MODULE_INFO
	.space	4

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_60860b498061fbc6
	.p2align	2
L_OBJC_IMAGE_INFO_60860b498061fbc6:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_60860b498061fbc6
L_OBJC_CLASS_NAME_60860b498061fbc6:
	.ascii	"NSObject"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_60860b498061fbc6
	.p2align	2
L_OBJC_CLASS_REFERENCES_60860b498061fbc6:
	.long	L_OBJC_CLASS_NAME_60860b498061fbc6

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_60860b498061fbc6_MODULE_INFO
L_OBJC_CLASS_NAME_60860b498061fbc6_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_60860b498061fbc6
	.p2align	2
L_OBJC_MODULES_60860b498061fbc6:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_60860b498061fbc6_MODULE_INFO
	.space	4

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_089cee9fe04089a4
	.p2align	2
L_OBJC_IMAGE_INFO_089cee9fe04089a4:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_089cee9fe04089a4
L_OBJC_CLASS_NAME_089cee9fe04089a4:
	.ascii	"NSString"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_089cee9fe04089a4
	.p2align	2
L_OBJC_CLASS_REFERENCES_089cee9fe04089a4:
	.long	L_OBJC_CLASS_NAME_089cee9fe04089a4

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_089cee9fe04089a4_MODULE_INFO
L_OBJC_CLASS_NAME_089cee9fe04089a4_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_089cee9fe04089a4
	.p2align	2
L_OBJC_MODULES_089cee9fe04089a4:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_089cee9fe04089a4_MODULE_INFO
	.space	4

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_5331bb309754c706
	.p2align	2
L_OBJC_IMAGE_INFO_5331bb309754c706:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_5331bb309754c706
L_OBJC_CLASS_NAME_5331bb309754c706:
	.ascii	"NSData"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_5331bb309754c706
	.p2align	2
L_OBJC_CLASS_REFERENCES_5331bb309754c706:
	.long	L_OBJC_CLASS_NAME_5331bb309754c706

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_5331bb309754c706_MODULE_INFO
L_OBJC_CLASS_NAME_5331bb309754c706_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_5331bb309754c706
	.p2align	2
L_OBJC_MODULES_5331bb309754c706:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_5331bb309754c706_MODULE_INFO
	.space	4

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_15b3f8b356e4fdb3
	.p2align	2
L_OBJC_IMAGE_INFO_15b3f8b356e4fdb3:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_15b3f8b356e4fdb3
L_OBJC_CLASS_NAME_15b3f8b356e4fdb3:
	.ascii	"NSException"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_15b3f8b356e4fdb3
	.p2align	2
L_OBJC_CLASS_REFERENCES_15b3f8b356e4fdb3:
	.long	L_OBJC_CLASS_NAME_15b3f8b356e4fdb3

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_15b3f8b356e4fdb3_MODULE_INFO
L_OBJC_CLASS_NAME_15b3f8b356e4fdb3_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_15b3f8b356e4fdb3
	.p2align	2
L_OBJC_MODULES_15b3f8b356e4fdb3:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_15b3f8b356e4fdb3_MODULE_INFO
	.space	4

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_4fe8a7873c5b5bcf
	.p2align	2
L_OBJC_IMAGE_INFO_4fe8a7873c5b5bcf:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_4fe8a7873c5b5bcf
L_OBJC_CLASS_NAME_4fe8a7873c5b5bcf:
	.ascii	"NSLock"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_4fe8a7873c5b5bcf
	.p2align	2
L_OBJC_CLASS_REFERENCES_4fe8a7873c5b5bcf:
	.long	L_OBJC_CLASS_NAME_4fe8a7873c5b5bcf

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_4fe8a7873c5b5bcf_MODULE_INFO
L_OBJC_CLASS_NAME_4fe8a7873c5b5bcf_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_4fe8a7873c5b5bcf
	.p2align	2
L_OBJC_MODULES_4fe8a7873c5b5bcf:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_4fe8a7873c5b5bcf_MODULE_INFO
	.space	4

.subsections_via_symbols
