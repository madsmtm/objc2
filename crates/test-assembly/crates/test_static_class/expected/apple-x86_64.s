	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_fn1_get_class
	.p2align	4
_fn1_get_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34]
	pop	rbp
	ret

	.globl	_fn1_get_same_class
	.p2align	4
_fn1_get_same_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a]
	pop	rbp
	ret

	.globl	_fn3_get_different_class
	.p2align	4
_fn3_get_different_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1]
	pop	rbp
	ret

	.globl	_fn4_unused_class
	.p2align	4
_fn4_unused_class:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.globl	_fn5_use_fns
	.p2align	4
_fn5_use_fns:
	push	rbp
	mov	rbp, rsp
	mov	rax, rdi
	mov	rcx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34]
	mov	rdx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a]
	mov	rsi, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1]
	mov	rdi, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_a92f01d3b55d29c5]
	mov	qword ptr [rax], rcx
	mov	qword ptr [rax + 8], rdx
	mov	qword ptr [rax + 16], rsi
	mov	qword ptr [rax + 24], rdi
	pop	rbp
	ret

	.globl	_fn6_use_same_twice
	.p2align	4
_fn6_use_same_twice:
	push	rbp
	mov	rbp, rsp
	mov	rax, rdi
	mov	rcx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34]
	mov	qword ptr [rdi], rcx
	mov	qword ptr [rdi + 8], rcx
	pop	rbp
	ret

	.globl	_fn7_use_in_loop
	.p2align	4
_fn7_use_in_loop:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.section	__DATA,__objc_classrefs
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34:
	.quad	_OBJC_CLASS_$_NSObject

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1:
	.quad	_OBJC_CLASS_$_NSString

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_9f503c7582f87b48
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_9f503c7582f87b48:
	.quad	_OBJC_CLASS_$_NSData

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_a92f01d3b55d29c5
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_a92f01d3b55d29c5:
	.quad	_OBJC_CLASS_$_NSException

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_d4ef9efb3ee49ab7
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_d4ef9efb3ee49ab7:
	.quad	_OBJC_CLASS_$_NSLock

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_1f36dafa1e0a7b34
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_1f36dafa1e0a7b34:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_5a6ce274a9f949e1
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_5a6ce274a9f949e1:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_9f503c7582f87b48
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_9f503c7582f87b48:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_a92f01d3b55d29c5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a92f01d3b55d29c5:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_d4ef9efb3ee49ab7
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_d4ef9efb3ee49ab7:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_e1a6d3426ab3be5a
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_e1a6d3426ab3be5a:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
