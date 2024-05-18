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
	mov	eax, dword ptr [eax + L_OBJC_CLASS_REFERENCES_831fece26e45cd9e-L0$pb]
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
	mov	eax, dword ptr [eax + L_OBJC_CLASS_REFERENCES_723df0da59ee573a-L1$pb]
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
	mov	eax, dword ptr [eax + L_OBJC_CLASS_REFERENCES_a1c78af2bef71f32-L2$pb]
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
	mov	edx, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_831fece26e45cd9e-L4$pb]
	mov	esi, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_723df0da59ee573a-L4$pb]
	mov	edi, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_a1c78af2bef71f32-L4$pb]
	mov	ecx, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_ae7bef6061eca0c4-L4$pb]
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
	mov	ecx, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_831fece26e45cd9e-L5$pb]
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
	.globl	L_OBJC_CLASS_NAME_831fece26e45cd9e
L_OBJC_CLASS_NAME_831fece26e45cd9e:
	.ascii	"NSObject"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_831fece26e45cd9e
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_831fece26e45cd9e:
	.long	L_OBJC_CLASS_NAME_831fece26e45cd9e

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_831fece26e45cd9e
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_831fece26e45cd9e:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_831fece26e45cd9e_MODULE_INFO
L_OBJC_CLASS_NAME_831fece26e45cd9e_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_831fece26e45cd9e
	.p2align	2, 0x0
L_OBJC_MODULES_831fece26e45cd9e:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_831fece26e45cd9e_MODULE_INFO
	.space	4

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_723df0da59ee573a
L_OBJC_CLASS_NAME_723df0da59ee573a:
	.ascii	"NSObject"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_723df0da59ee573a
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_723df0da59ee573a:
	.long	L_OBJC_CLASS_NAME_723df0da59ee573a

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_723df0da59ee573a
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_723df0da59ee573a:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_723df0da59ee573a_MODULE_INFO
L_OBJC_CLASS_NAME_723df0da59ee573a_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_723df0da59ee573a
	.p2align	2, 0x0
L_OBJC_MODULES_723df0da59ee573a:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_723df0da59ee573a_MODULE_INFO
	.space	4

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_a1c78af2bef71f32
L_OBJC_CLASS_NAME_a1c78af2bef71f32:
	.ascii	"NSString"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_a1c78af2bef71f32
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_a1c78af2bef71f32:
	.long	L_OBJC_CLASS_NAME_a1c78af2bef71f32

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_a1c78af2bef71f32
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a1c78af2bef71f32:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_a1c78af2bef71f32_MODULE_INFO
L_OBJC_CLASS_NAME_a1c78af2bef71f32_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_a1c78af2bef71f32
	.p2align	2, 0x0
L_OBJC_MODULES_a1c78af2bef71f32:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_a1c78af2bef71f32_MODULE_INFO
	.space	4

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_ce452f03dca2d1c0
L_OBJC_CLASS_NAME_ce452f03dca2d1c0:
	.ascii	"NSData"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_ce452f03dca2d1c0
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_ce452f03dca2d1c0:
	.long	L_OBJC_CLASS_NAME_ce452f03dca2d1c0

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_ce452f03dca2d1c0
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ce452f03dca2d1c0:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_ce452f03dca2d1c0_MODULE_INFO
L_OBJC_CLASS_NAME_ce452f03dca2d1c0_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_ce452f03dca2d1c0
	.p2align	2, 0x0
L_OBJC_MODULES_ce452f03dca2d1c0:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_ce452f03dca2d1c0_MODULE_INFO
	.space	4

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_ae7bef6061eca0c4
L_OBJC_CLASS_NAME_ae7bef6061eca0c4:
	.ascii	"NSException"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_ae7bef6061eca0c4
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_ae7bef6061eca0c4:
	.long	L_OBJC_CLASS_NAME_ae7bef6061eca0c4

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_ae7bef6061eca0c4
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ae7bef6061eca0c4:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_ae7bef6061eca0c4_MODULE_INFO
L_OBJC_CLASS_NAME_ae7bef6061eca0c4_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_ae7bef6061eca0c4
	.p2align	2, 0x0
L_OBJC_MODULES_ae7bef6061eca0c4:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_ae7bef6061eca0c4_MODULE_INFO
	.space	4

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_bedb39edf06e7f45
L_OBJC_CLASS_NAME_bedb39edf06e7f45:
	.ascii	"NSLock"

	.section	__OBJC,__cls_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_CLASS_REFERENCES_bedb39edf06e7f45
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_bedb39edf06e7f45:
	.long	L_OBJC_CLASS_NAME_bedb39edf06e7f45

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_bedb39edf06e7f45
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_bedb39edf06e7f45:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_bedb39edf06e7f45_MODULE_INFO
L_OBJC_CLASS_NAME_bedb39edf06e7f45_MODULE_INFO:
	.space	1

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_bedb39edf06e7f45
	.p2align	2, 0x0
L_OBJC_MODULES_bedb39edf06e7f45:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_bedb39edf06e7f45_MODULE_INFO
	.space	4

.subsections_via_symbols
