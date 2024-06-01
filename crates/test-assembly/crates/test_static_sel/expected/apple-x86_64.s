	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_sel
	.p2align	4, 0x90
_get_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7]
	pop	rbp
	ret

	.globl	_get_same_sel
	.p2align	4, 0x90
_get_same_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513]
	pop	rbp
	ret

	.globl	_get_common_twice
	.p2align	4, 0x90
_get_common_twice:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_alloc@GOTPCREL]
	mov	rax, qword ptr [rax]
	mov	rdx, rax
	pop	rbp
	ret

	.globl	_get_different_sel
	.p2align	4, 0x90
_get_different_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864]
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
	mov	rcx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513]
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864]
	mov	rdi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_408f5be8f4fd2627]
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
	mov	rcx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7]
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

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_caedaca3f40015a7
L_OBJC_METH_VAR_NAME_caedaca3f40015a7:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7:
	.quad	L_OBJC_METH_VAR_NAME_caedaca3f40015a7

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_caedaca3f40015a7
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_caedaca3f40015a7:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_a7c7f3067f40b513
L_OBJC_METH_VAR_NAME_a7c7f3067f40b513:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513:
	.quad	L_OBJC_METH_VAR_NAME_a7c7f3067f40b513

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_a7c7f3067f40b513
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a7c7f3067f40b513:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_bae8570d40d73864
L_OBJC_METH_VAR_NAME_bae8570d40d73864:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864:
	.quad	L_OBJC_METH_VAR_NAME_bae8570d40d73864

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_bae8570d40d73864
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_bae8570d40d73864:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_9c1b77e8cf40622d
L_OBJC_METH_VAR_NAME_9c1b77e8cf40622d:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_9c1b77e8cf40622d
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_9c1b77e8cf40622d:
	.quad	L_OBJC_METH_VAR_NAME_9c1b77e8cf40622d

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_9c1b77e8cf40622d
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_9c1b77e8cf40622d:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_408f5be8f4fd2627
L_OBJC_METH_VAR_NAME_408f5be8f4fd2627:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_408f5be8f4fd2627
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_408f5be8f4fd2627:
	.quad	L_OBJC_METH_VAR_NAME_408f5be8f4fd2627

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_408f5be8f4fd2627
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_408f5be8f4fd2627:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_82483a8131827890
L_OBJC_METH_VAR_NAME_82483a8131827890:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_82483a8131827890
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_82483a8131827890:
	.quad	L_OBJC_METH_VAR_NAME_82483a8131827890

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_82483a8131827890
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_82483a8131827890:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
