	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_class
	.p2align	4, 0x90
_get_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777]
	pop	rbp
	ret

	.globl	_get_same_class
	.p2align	4, 0x90
_get_same_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07]
	pop	rbp
	ret

	.globl	_get_different_class
	.p2align	4, 0x90
_get_different_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b]
	pop	rbp
	ret

	.globl	_unused_class
	.p2align	4, 0x90
_unused_class:
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
	mov	rcx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777]
	mov	rdx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07]
	mov	rsi, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b]
	mov	rdi, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_97e6a8c6ed5db063]
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
	mov	rcx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777]
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

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_928cf03fcc497777
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_928cf03fcc497777:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2fe1990982915f07
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2fe1990982915f07:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b:
	.quad	_OBJC_CLASS_$_NSString

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_dfff3a06c0bf722b
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_dfff3a06c0bf722b:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_ea6fbcf172f7f513
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_ea6fbcf172f7f513:
	.quad	_OBJC_CLASS_$_NSData

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_ea6fbcf172f7f513
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ea6fbcf172f7f513:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_97e6a8c6ed5db063
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_97e6a8c6ed5db063:
	.quad	_OBJC_CLASS_$_NSException

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_97e6a8c6ed5db063
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_97e6a8c6ed5db063:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_bb5b616899716c0d
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_bb5b616899716c0d:
	.quad	_OBJC_CLASS_$_NSLock

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_bb5b616899716c0d
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_bb5b616899716c0d:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
