	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_sel
	.p2align	4, 0x90
_get_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0]
	pop	rbp
	ret

	.globl	_get_same_sel
	.p2align	4, 0x90
_get_same_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83]
	pop	rbp
	ret

	.globl	_get_common_twice
	.p2align	4, 0x90
_get_common_twice:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_b3892a38c2415013]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_9a8b70db451c67b1]
	pop	rbp
	ret

	.globl	_get_different_sel
	.p2align	4, 0x90
_get_different_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_25911857653c680c]
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
	mov	rcx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83]
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_25911857653c680c]
	mov	rdi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_acb291d82e56f534]
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
	mov	rcx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0]
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
	.globl	L_OBJC_IMAGE_INFO_2ff5c2d33acc98c0
	.p2align	2
L_OBJC_IMAGE_INFO_2ff5c2d33acc98c0:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2ff5c2d33acc98c0
L_OBJC_METH_VAR_NAME_2ff5c2d33acc98c0:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0:
	.quad	L_OBJC_METH_VAR_NAME_2ff5c2d33acc98c0

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_6e17eb9d3fa7fa83
	.p2align	2
L_OBJC_IMAGE_INFO_6e17eb9d3fa7fa83:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6e17eb9d3fa7fa83
L_OBJC_METH_VAR_NAME_6e17eb9d3fa7fa83:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83:
	.quad	L_OBJC_METH_VAR_NAME_6e17eb9d3fa7fa83

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_b3892a38c2415013
	.p2align	2
L_OBJC_IMAGE_INFO_b3892a38c2415013:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_b3892a38c2415013
L_OBJC_METH_VAR_NAME_b3892a38c2415013:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_b3892a38c2415013
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_b3892a38c2415013:
	.quad	L_OBJC_METH_VAR_NAME_b3892a38c2415013

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_9a8b70db451c67b1
	.p2align	2
L_OBJC_IMAGE_INFO_9a8b70db451c67b1:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_9a8b70db451c67b1
L_OBJC_METH_VAR_NAME_9a8b70db451c67b1:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_9a8b70db451c67b1
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_9a8b70db451c67b1:
	.quad	L_OBJC_METH_VAR_NAME_9a8b70db451c67b1

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_25911857653c680c
	.p2align	2
L_OBJC_IMAGE_INFO_25911857653c680c:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_25911857653c680c
L_OBJC_METH_VAR_NAME_25911857653c680c:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_25911857653c680c
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_25911857653c680c:
	.quad	L_OBJC_METH_VAR_NAME_25911857653c680c

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_baa3c09478169afc
	.p2align	2
L_OBJC_IMAGE_INFO_baa3c09478169afc:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_baa3c09478169afc
L_OBJC_METH_VAR_NAME_baa3c09478169afc:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_baa3c09478169afc
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_baa3c09478169afc:
	.quad	L_OBJC_METH_VAR_NAME_baa3c09478169afc

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_acb291d82e56f534
	.p2align	2
L_OBJC_IMAGE_INFO_acb291d82e56f534:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_acb291d82e56f534
L_OBJC_METH_VAR_NAME_acb291d82e56f534:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_acb291d82e56f534
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_acb291d82e56f534:
	.quad	L_OBJC_METH_VAR_NAME_acb291d82e56f534

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_c831c01ba82dcc2e
	.p2align	2
L_OBJC_IMAGE_INFO_c831c01ba82dcc2e:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_c831c01ba82dcc2e
L_OBJC_METH_VAR_NAME_c831c01ba82dcc2e:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_c831c01ba82dcc2e
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_c831c01ba82dcc2e:
	.quad	L_OBJC_METH_VAR_NAME_c831c01ba82dcc2e

.subsections_via_symbols
