	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_class
	.p2align	4, 0x90
_get_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_49a0cd2a35b9a474]
	pop	rbp
	ret

	.globl	_get_same_class
	.p2align	4, 0x90
_get_same_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_60860b498061fbc6]
	pop	rbp
	ret

	.globl	_get_different_class
	.p2align	4, 0x90
_get_different_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_089cee9fe04089a4]
	pop	rbp
	ret

	.globl	_unused_sel
	.p2align	4, 0x90
_unused_sel:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.globl	_use_fns
	.p2align	4, 0x90
_use_fns:
	push	rbp
	mov	rbp, rsp
	mov	rax, rdi
	mov	rcx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_49a0cd2a35b9a474]
	mov	rdx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_60860b498061fbc6]
	mov	rsi, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_089cee9fe04089a4]
	mov	rdi, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_15b3f8b356e4fdb3]
	mov	qword ptr [rax], rcx
	mov	qword ptr [rax + 8], rdx
	mov	qword ptr [rax + 16], rsi
	mov	qword ptr [rax + 24], rdi
	pop	rbp
	ret

	.globl	_use_same_twice
	.p2align	4, 0x90
_use_same_twice:
	push	rbp
	mov	rbp, rsp
	mov	rax, rdi
	mov	rcx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_49a0cd2a35b9a474]
	mov	qword ptr [rdi], rcx
	mov	qword ptr [rdi + 8], rcx
	pop	rbp
	ret

	.globl	_use_in_loop
	.p2align	4, 0x90
_use_in_loop:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_49a0cd2a35b9a474
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_49a0cd2a35b9a474:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_49a0cd2a35b9a474
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_49a0cd2a35b9a474:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_60860b498061fbc6
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_60860b498061fbc6:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_60860b498061fbc6
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_60860b498061fbc6:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_089cee9fe04089a4
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_089cee9fe04089a4:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_089cee9fe04089a4
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_089cee9fe04089a4:
	.quad	_OBJC_CLASS_$_NSString

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_5331bb309754c706
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_5331bb309754c706:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_5331bb309754c706
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_5331bb309754c706:
	.quad	_OBJC_CLASS_$_NSData

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_15b3f8b356e4fdb3
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_15b3f8b356e4fdb3:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_15b3f8b356e4fdb3
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_15b3f8b356e4fdb3:
	.quad	_OBJC_CLASS_$_NSException

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_4fe8a7873c5b5bcf
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_4fe8a7873c5b5bcf:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_4fe8a7873c5b5bcf
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_4fe8a7873c5b5bcf:
	.quad	_OBJC_CLASS_$_NSLock

.subsections_via_symbols
