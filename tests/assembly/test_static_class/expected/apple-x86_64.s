	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_class
	.p2align	4, 0x90
_get_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_e0796f2f86586929]
	pop	rbp
	ret

	.globl	_get_same_class
	.p2align	4, 0x90
_get_same_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_1121b6b8519ae283]
	pop	rbp
	ret

	.globl	_get_different_class
	.p2align	4, 0x90
_get_different_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_7314246f93b118b1]
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
	mov	rcx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_e0796f2f86586929]
	mov	rdx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_1121b6b8519ae283]
	mov	rsi, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_7314246f93b118b1]
	mov	rdi, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_16e72be3e810c687]
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
	mov	rcx, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_e0796f2f86586929]
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
	.globl	L_OBJC_IMAGE_INFO_e0796f2f86586929
	.p2align	2
L_OBJC_IMAGE_INFO_e0796f2f86586929:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_e0796f2f86586929
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_e0796f2f86586929:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_1121b6b8519ae283
	.p2align	2
L_OBJC_IMAGE_INFO_1121b6b8519ae283:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_1121b6b8519ae283
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_1121b6b8519ae283:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_7314246f93b118b1
	.p2align	2
L_OBJC_IMAGE_INFO_7314246f93b118b1:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_7314246f93b118b1
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_7314246f93b118b1:
	.quad	_OBJC_CLASS_$_NSString

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f5b45e093b02d6c9
	.p2align	2
L_OBJC_IMAGE_INFO_f5b45e093b02d6c9:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_f5b45e093b02d6c9
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_f5b45e093b02d6c9:
	.quad	_OBJC_CLASS_$_NSData

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_16e72be3e810c687
	.p2align	2
L_OBJC_IMAGE_INFO_16e72be3e810c687:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_16e72be3e810c687
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_16e72be3e810c687:
	.quad	_OBJC_CLASS_$_NSException

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_4465b4c2c67ae674
	.p2align	2
L_OBJC_IMAGE_INFO_4465b4c2c67ae674:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_4465b4c2c67ae674
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_4465b4c2c67ae674:
	.quad	_OBJC_CLASS_$_NSLock

.subsections_via_symbols
