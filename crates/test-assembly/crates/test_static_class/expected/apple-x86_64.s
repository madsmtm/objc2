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
	mov	rcx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777]
	mov	rdx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07]
	mov	rsi, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b]
	mov	rdi, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_5ab5a81fcf2763fb]
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

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_928cf03fcc497777
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_928cf03fcc497777:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2fe1990982915f07
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2fe1990982915f07:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_dfff3a06c0bf722b
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_dfff3a06c0bf722b:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b:
	.quad	_OBJC_CLASS_$_NSString

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f6e054106fdbe219
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f6e054106fdbe219:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_f6e054106fdbe219
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_f6e054106fdbe219:
	.quad	_OBJC_CLASS_$_NSData

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_5ab5a81fcf2763fb
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_5ab5a81fcf2763fb:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_5ab5a81fcf2763fb
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_5ab5a81fcf2763fb:
	.quad	_OBJC_CLASS_$_NSException

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_54ecac6d305d112a
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_54ecac6d305d112a:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_54ecac6d305d112a
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_54ecac6d305d112a:
	.quad	_OBJC_CLASS_$_NSLock

.subsections_via_symbols
