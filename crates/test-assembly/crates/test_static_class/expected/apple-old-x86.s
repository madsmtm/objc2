	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_class
	.p2align	4
_get_class:
	push	ebp
	mov	ebp, esp
	call	L0$pb
L0$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_CLASS_REFERENCES_7443a74fb2d1e4c6-L0$pb]
	pop	ebp
	ret

	.globl	_get_same_class
	.p2align	4
_get_same_class:
	push	ebp
	mov	ebp, esp
	call	L1$pb
L1$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_CLASS_REFERENCES_8f52a951012bf702-L1$pb]
	pop	ebp
	ret

	.globl	_get_different_class
	.p2align	4
_get_different_class:
	push	ebp
	mov	ebp, esp
	call	L2$pb
L2$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_CLASS_REFERENCES_4882212c6ef400ba-L2$pb]
	pop	ebp
	ret

	.globl	_unused_class
	.p2align	4
_unused_class:
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
	call	L4$pb
L4$pb:
	pop	ecx
	mov	eax, dword ptr [ebp + 8]
	mov	edx, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_7443a74fb2d1e4c6-L4$pb]
	mov	esi, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_8f52a951012bf702-L4$pb]
	mov	edi, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_4882212c6ef400ba-L4$pb]
	mov	ecx, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_5ca3eecf631727de-L4$pb]
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
	call	L5$pb
L5$pb:
	pop	ecx
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ecx + L_OBJC_CLASS_REFERENCES_7443a74fb2d1e4c6-L5$pb]
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

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_7443a74fb2d1e4c6
L_OBJC_CLASS_NAME_7443a74fb2d1e4c6:
	.ascii	"NSObject"

	.section	__OBJC,__cls_refs,literal_pointers
	.globl	L_OBJC_CLASS_REFERENCES_7443a74fb2d1e4c6
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_7443a74fb2d1e4c6:
	.long	L_OBJC_CLASS_NAME_7443a74fb2d1e4c6

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_7443a74fb2d1e4c6_MODULE_INFO
L_OBJC_CLASS_NAME_7443a74fb2d1e4c6_MODULE_INFO:
	.space	1

	.globl	L_OBJC_CLASS_NAME_8f52a951012bf702
L_OBJC_CLASS_NAME_8f52a951012bf702:
	.ascii	"NSObject"

	.section	__OBJC,__cls_refs,literal_pointers
	.globl	L_OBJC_CLASS_REFERENCES_8f52a951012bf702
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_8f52a951012bf702:
	.long	L_OBJC_CLASS_NAME_8f52a951012bf702

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_8f52a951012bf702_MODULE_INFO
L_OBJC_CLASS_NAME_8f52a951012bf702_MODULE_INFO:
	.space	1

	.globl	L_OBJC_CLASS_NAME_4882212c6ef400ba
L_OBJC_CLASS_NAME_4882212c6ef400ba:
	.ascii	"NSString"

	.section	__OBJC,__cls_refs,literal_pointers
	.globl	L_OBJC_CLASS_REFERENCES_4882212c6ef400ba
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_4882212c6ef400ba:
	.long	L_OBJC_CLASS_NAME_4882212c6ef400ba

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_4882212c6ef400ba_MODULE_INFO
L_OBJC_CLASS_NAME_4882212c6ef400ba_MODULE_INFO:
	.space	1

	.globl	L_OBJC_CLASS_NAME_9c6ceff32d4e4b8b
L_OBJC_CLASS_NAME_9c6ceff32d4e4b8b:
	.ascii	"NSData"

	.section	__OBJC,__cls_refs,literal_pointers
	.globl	L_OBJC_CLASS_REFERENCES_9c6ceff32d4e4b8b
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_9c6ceff32d4e4b8b:
	.long	L_OBJC_CLASS_NAME_9c6ceff32d4e4b8b

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_9c6ceff32d4e4b8b_MODULE_INFO
L_OBJC_CLASS_NAME_9c6ceff32d4e4b8b_MODULE_INFO:
	.space	1

	.globl	L_OBJC_CLASS_NAME_5ca3eecf631727de
L_OBJC_CLASS_NAME_5ca3eecf631727de:
	.ascii	"NSException"

	.section	__OBJC,__cls_refs,literal_pointers
	.globl	L_OBJC_CLASS_REFERENCES_5ca3eecf631727de
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_5ca3eecf631727de:
	.long	L_OBJC_CLASS_NAME_5ca3eecf631727de

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_5ca3eecf631727de_MODULE_INFO
L_OBJC_CLASS_NAME_5ca3eecf631727de_MODULE_INFO:
	.space	1

	.globl	L_OBJC_CLASS_NAME_76a360f1704b1e39
L_OBJC_CLASS_NAME_76a360f1704b1e39:
	.ascii	"NSLock"

	.section	__OBJC,__cls_refs,literal_pointers
	.globl	L_OBJC_CLASS_REFERENCES_76a360f1704b1e39
	.p2align	2, 0x0
L_OBJC_CLASS_REFERENCES_76a360f1704b1e39:
	.long	L_OBJC_CLASS_NAME_76a360f1704b1e39

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_CLASS_NAME_76a360f1704b1e39_MODULE_INFO
L_OBJC_CLASS_NAME_76a360f1704b1e39_MODULE_INFO:
	.space	1

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_7443a74fb2d1e4c6
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_7443a74fb2d1e4c6:
	.asciz	"\000\000\000\000@\000\000"

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_7443a74fb2d1e4c6
	.p2align	2, 0x0
L_OBJC_MODULES_7443a74fb2d1e4c6:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_7443a74fb2d1e4c6_MODULE_INFO
	.space	4

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_8f52a951012bf702
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_8f52a951012bf702:
	.asciz	"\000\000\000\000@\000\000"

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_8f52a951012bf702
	.p2align	2, 0x0
L_OBJC_MODULES_8f52a951012bf702:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_8f52a951012bf702_MODULE_INFO
	.space	4

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_4882212c6ef400ba
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_4882212c6ef400ba:
	.asciz	"\000\000\000\000@\000\000"

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_4882212c6ef400ba
	.p2align	2, 0x0
L_OBJC_MODULES_4882212c6ef400ba:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_4882212c6ef400ba_MODULE_INFO
	.space	4

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_9c6ceff32d4e4b8b
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_9c6ceff32d4e4b8b:
	.asciz	"\000\000\000\000@\000\000"

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_9c6ceff32d4e4b8b
	.p2align	2, 0x0
L_OBJC_MODULES_9c6ceff32d4e4b8b:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_9c6ceff32d4e4b8b_MODULE_INFO
	.space	4

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_5ca3eecf631727de
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_5ca3eecf631727de:
	.asciz	"\000\000\000\000@\000\000"

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_5ca3eecf631727de
	.p2align	2, 0x0
L_OBJC_MODULES_5ca3eecf631727de:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_5ca3eecf631727de_MODULE_INFO
	.space	4

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_76a360f1704b1e39
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_76a360f1704b1e39:
	.asciz	"\000\000\000\000@\000\000"

	.section	__OBJC,__module_info,regular,no_dead_strip
	.globl	L_OBJC_MODULES_76a360f1704b1e39
	.p2align	2, 0x0
L_OBJC_MODULES_76a360f1704b1e39:
	.asciz	"\007\000\000\000\020\000\000"
	.long	L_OBJC_CLASS_NAME_76a360f1704b1e39_MODULE_INFO
	.space	4

.subsections_via_symbols
