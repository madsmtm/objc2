	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_class
	.p2align	4, 0x90
_get_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f]
	pop	rbp
	ret

	.globl	_get_same_class
	.p2align	4, 0x90
_get_same_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79]
	pop	rbp
	ret

	.globl	_get_different_class
	.p2align	4, 0x90
_get_different_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4]
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
	mov	rcx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f]
	mov	rdx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79]
	mov	rsi, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4]
	mov	rdi, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_2f45d8445f72bd9b]
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
	mov	rcx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f]
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
	.globl	L_OBJC_IMAGE_INFO_9657804a2a54ab6f
	.p2align	2
L_OBJC_IMAGE_INFO_9657804a2a54ab6f:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_e897a41b218dcf79
	.p2align	2
L_OBJC_IMAGE_INFO_e897a41b218dcf79:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_3b7780b4dcfcb9d4
	.p2align	2
L_OBJC_IMAGE_INFO_3b7780b4dcfcb9d4:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4:
	.quad	_OBJC_CLASS_$_NSString

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f2fb7c579d3c0a74
	.p2align	2
L_OBJC_IMAGE_INFO_f2fb7c579d3c0a74:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_f2fb7c579d3c0a74
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_f2fb7c579d3c0a74:
	.quad	_OBJC_CLASS_$_NSData

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2f45d8445f72bd9b
	.p2align	2
L_OBJC_IMAGE_INFO_2f45d8445f72bd9b:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_2f45d8445f72bd9b
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_2f45d8445f72bd9b:
	.quad	_OBJC_CLASS_$_NSException

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_3bf610c78df2b6bb
	.p2align	2
L_OBJC_IMAGE_INFO_3bf610c78df2b6bb:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_3bf610c78df2b6bb
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_3bf610c78df2b6bb:
	.quad	_OBJC_CLASS_$_NSLock

.subsections_via_symbols
