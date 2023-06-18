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
	mov	edx, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_928cf03fcc497777-L4$pb]
	mov	esi, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_2fe1990982915f07-L4$pb]
	mov	edi, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_dfff3a06c0bf722b-L4$pb]
	mov	ecx, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_5ab5a81fcf2763fb-L4$pb]
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

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_928cf03fcc497777
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_928cf03fcc497777:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_928cf03fcc497777
L_OBJC_CLASS_NAME_928cf03fcc497777:
	.ascii	"NSObject"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_928cf03fcc497777
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_928cf03fcc497777:
	.long	L_OBJC_CLASS_NAME_928cf03fcc497777

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

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_2fe1990982915f07
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2fe1990982915f07:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_2fe1990982915f07
L_OBJC_CLASS_NAME_2fe1990982915f07:
	.ascii	"NSObject"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_2fe1990982915f07
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_2fe1990982915f07:
	.long	L_OBJC_CLASS_NAME_2fe1990982915f07

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

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_dfff3a06c0bf722b
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_dfff3a06c0bf722b:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_dfff3a06c0bf722b
L_OBJC_CLASS_NAME_dfff3a06c0bf722b:
	.ascii	"NSString"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_dfff3a06c0bf722b
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_dfff3a06c0bf722b:
	.long	L_OBJC_CLASS_NAME_dfff3a06c0bf722b

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

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_f6e054106fdbe219
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f6e054106fdbe219:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_f6e054106fdbe219
L_OBJC_CLASS_NAME_f6e054106fdbe219:
	.ascii	"NSData"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_f6e054106fdbe219
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_f6e054106fdbe219:
	.long	L_OBJC_CLASS_NAME_f6e054106fdbe219

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_f6e054106fdbe219_MODULE_INFO
L_OBJC_CLASS_NAME_f6e054106fdbe219_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_f6e054106fdbe219
	.p2align	2, 0x0
L_OBJC_MODULES_f6e054106fdbe219:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_f6e054106fdbe219_MODULE_INFO
	.space	4

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_5ab5a81fcf2763fb
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_5ab5a81fcf2763fb:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_5ab5a81fcf2763fb
L_OBJC_CLASS_NAME_5ab5a81fcf2763fb:
	.ascii	"NSException"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_5ab5a81fcf2763fb
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_5ab5a81fcf2763fb:
	.long	L_OBJC_CLASS_NAME_5ab5a81fcf2763fb

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_5ab5a81fcf2763fb_MODULE_INFO
L_OBJC_CLASS_NAME_5ab5a81fcf2763fb_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_5ab5a81fcf2763fb
	.p2align	2, 0x0
L_OBJC_MODULES_5ab5a81fcf2763fb:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_5ab5a81fcf2763fb_MODULE_INFO
	.space	4

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_54ecac6d305d112a
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_54ecac6d305d112a:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_54ecac6d305d112a
L_OBJC_CLASS_NAME_54ecac6d305d112a:
	.ascii	"NSLock"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_54ecac6d305d112a
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_54ecac6d305d112a:
	.long	L_OBJC_CLASS_NAME_54ecac6d305d112a

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_54ecac6d305d112a_MODULE_INFO
L_OBJC_CLASS_NAME_54ecac6d305d112a_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_54ecac6d305d112a
	.p2align	2, 0x0
L_OBJC_MODULES_54ecac6d305d112a:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_54ecac6d305d112a_MODULE_INFO
	.space	4

.subsections_via_symbols
