	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_class
	.p2align	4, 0x90
_get_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_831fece26e45cd9e]
	pop	rbp
	ret

	.globl	_get_same_class
	.p2align	4, 0x90
_get_same_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_723df0da59ee573a]
	pop	rbp
	ret

	.globl	_get_different_class
	.p2align	4, 0x90
_get_different_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_a1c78af2bef71f32]
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
	mov	rcx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_831fece26e45cd9e]
	mov	rdx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_723df0da59ee573a]
	mov	rsi, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_a1c78af2bef71f32]
	mov	rdi, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_ae7bef6061eca0c4]
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
	mov	rcx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_831fece26e45cd9e]
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
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_831fece26e45cd9e
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_831fece26e45cd9e:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_831fece26e45cd9e
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_831fece26e45cd9e:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_723df0da59ee573a
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_723df0da59ee573a:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_723df0da59ee573a
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_723df0da59ee573a:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_a1c78af2bef71f32
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_a1c78af2bef71f32:
	.quad	_OBJC_CLASS_$_NSString

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_a1c78af2bef71f32
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a1c78af2bef71f32:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_ce452f03dca2d1c0
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_ce452f03dca2d1c0:
	.quad	_OBJC_CLASS_$_NSData

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_ce452f03dca2d1c0
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ce452f03dca2d1c0:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_ae7bef6061eca0c4
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_ae7bef6061eca0c4:
	.quad	_OBJC_CLASS_$_NSException

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_ae7bef6061eca0c4
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ae7bef6061eca0c4:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_bedb39edf06e7f45
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_bedb39edf06e7f45:
	.quad	_OBJC_CLASS_$_NSLock

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_bedb39edf06e7f45
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_bedb39edf06e7f45:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
