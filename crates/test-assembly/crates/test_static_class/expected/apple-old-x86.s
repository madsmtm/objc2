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
	mov	eax, dword ptr [eax + L_OBJC_CLASS_REFERENCES_928cf03fcc497777-L0$pb]
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
	mov	eax, dword ptr [eax + L_OBJC_CLASS_REFERENCES_2fe1990982915f07-L1$pb]
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
	mov	eax, dword ptr [eax + L_OBJC_CLASS_REFERENCES_dfff3a06c0bf722b-L2$pb]
	pop	ebp
	ret

	.globl	_unused_class
	.p2align	4, 0x90
_unused_class:
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
	mov	edx, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_928cf03fcc497777-L4$pb]
	mov	esi, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_2fe1990982915f07-L4$pb]
	mov	edi, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_dfff3a06c0bf722b-L4$pb]
	mov	ecx, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_97e6a8c6ed5db063-L4$pb]
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
	mov	ecx, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_928cf03fcc497777-L5$pb]
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

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_928cf03fcc497777
L_OBJC_CLASS_NAME_928cf03fcc497777:
	.ascii	"NSObject"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_928cf03fcc497777
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_928cf03fcc497777:
	.long	L_OBJC_CLASS_NAME_928cf03fcc497777

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_928cf03fcc497777
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_928cf03fcc497777:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_928cf03fcc497777_MODULE_INFO
L_OBJC_CLASS_NAME_928cf03fcc497777_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_928cf03fcc497777
	.p2align	2, 0x0
L_OBJC_MODULES_928cf03fcc497777:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_928cf03fcc497777_MODULE_INFO
	.space	4

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_2fe1990982915f07
L_OBJC_CLASS_NAME_2fe1990982915f07:
	.ascii	"NSObject"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_2fe1990982915f07
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_2fe1990982915f07:
	.long	L_OBJC_CLASS_NAME_2fe1990982915f07

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_2fe1990982915f07
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2fe1990982915f07:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_2fe1990982915f07_MODULE_INFO
L_OBJC_CLASS_NAME_2fe1990982915f07_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_2fe1990982915f07
	.p2align	2, 0x0
L_OBJC_MODULES_2fe1990982915f07:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_2fe1990982915f07_MODULE_INFO
	.space	4

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_dfff3a06c0bf722b
L_OBJC_CLASS_NAME_dfff3a06c0bf722b:
	.ascii	"NSString"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_dfff3a06c0bf722b
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_dfff3a06c0bf722b:
	.long	L_OBJC_CLASS_NAME_dfff3a06c0bf722b

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_dfff3a06c0bf722b
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_dfff3a06c0bf722b:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_dfff3a06c0bf722b_MODULE_INFO
L_OBJC_CLASS_NAME_dfff3a06c0bf722b_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_dfff3a06c0bf722b
	.p2align	2, 0x0
L_OBJC_MODULES_dfff3a06c0bf722b:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_dfff3a06c0bf722b_MODULE_INFO
	.space	4

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_ea6fbcf172f7f513
L_OBJC_CLASS_NAME_ea6fbcf172f7f513:
	.ascii	"NSData"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_ea6fbcf172f7f513
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_ea6fbcf172f7f513:
	.long	L_OBJC_CLASS_NAME_ea6fbcf172f7f513

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_ea6fbcf172f7f513
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ea6fbcf172f7f513:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_ea6fbcf172f7f513_MODULE_INFO
L_OBJC_CLASS_NAME_ea6fbcf172f7f513_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_ea6fbcf172f7f513
	.p2align	2, 0x0
L_OBJC_MODULES_ea6fbcf172f7f513:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_ea6fbcf172f7f513_MODULE_INFO
	.space	4

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_97e6a8c6ed5db063
L_OBJC_CLASS_NAME_97e6a8c6ed5db063:
	.ascii	"NSException"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_97e6a8c6ed5db063
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_97e6a8c6ed5db063:
	.long	L_OBJC_CLASS_NAME_97e6a8c6ed5db063

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_97e6a8c6ed5db063
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_97e6a8c6ed5db063:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_97e6a8c6ed5db063_MODULE_INFO
L_OBJC_CLASS_NAME_97e6a8c6ed5db063_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_97e6a8c6ed5db063
	.p2align	2, 0x0
L_OBJC_MODULES_97e6a8c6ed5db063:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_97e6a8c6ed5db063_MODULE_INFO
	.space	4

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_bb5b616899716c0d
L_OBJC_CLASS_NAME_bb5b616899716c0d:
	.ascii	"NSLock"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_bb5b616899716c0d
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_bb5b616899716c0d:
	.long	L_OBJC_CLASS_NAME_bb5b616899716c0d

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_bb5b616899716c0d
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_bb5b616899716c0d:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_bb5b616899716c0d_MODULE_INFO
L_OBJC_CLASS_NAME_bb5b616899716c0d_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_bb5b616899716c0d
	.p2align	2, 0x0
L_OBJC_MODULES_bb5b616899716c0d:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_bb5b616899716c0d_MODULE_INFO
	.space	4

.subsections_via_symbols
