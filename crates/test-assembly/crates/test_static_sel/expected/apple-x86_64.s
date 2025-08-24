	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_fn1_get_sel
	.p2align	4
_fn1_get_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb]
	pop	rbp
	ret

	.globl	_fn2_get_same_sel
	.p2align	4
_fn2_get_same_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f]
	pop	rbp
	ret

	.globl	_fn3_get_common_twice
	.p2align	4
_fn3_get_common_twice:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_alloc@GOTPCREL]
	mov	rax, qword ptr [rax]
	mov	rdx, rax
	pop	rbp
	ret

	.globl	_fn4_get_different_sel
	.p2align	4
_fn4_get_different_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85]
	pop	rbp
	ret

	.globl	_fn5_unused_sel
	.p2align	4
_fn5_unused_sel:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.globl	_fn6_use_fns
	.p2align	4
_fn6_use_fns:
	push	rbp
	mov	rbp, rsp
	mov	rax, rdi
	mov	rcx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f]
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85]
	mov	rdi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_e91a5376ddae3cfc]
	mov	qword ptr [rax], rcx
	mov	qword ptr [rax + 8], rdx
	mov	qword ptr [rax + 16], rsi
	mov	qword ptr [rax + 24], rdi
	pop	rbp
	ret

	.globl	_fn7_use_same_twice
	.p2align	4
_fn7_use_same_twice:
	push	rbp
	mov	rbp, rsp
	mov	rax, rdi
	mov	rcx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb]
	mov	qword ptr [rdi], rcx
	mov	qword ptr [rdi + 8], rcx
	pop	rbp
	ret

	.globl	_fn8_use_in_loop
	.p2align	4
_fn8_use_in_loop:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2668dedcc69bf8fb
L_OBJC_METH_VAR_NAME_2668dedcc69bf8fb:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb:
	.quad	L_OBJC_METH_VAR_NAME_2668dedcc69bf8fb

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_9303807037ba4f9f
L_OBJC_METH_VAR_NAME_9303807037ba4f9f:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f:
	.quad	L_OBJC_METH_VAR_NAME_9303807037ba4f9f

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_ff60dae1998e0c85
L_OBJC_METH_VAR_NAME_ff60dae1998e0c85:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85:
	.quad	L_OBJC_METH_VAR_NAME_ff60dae1998e0c85

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_29d0234b9445d447
L_OBJC_METH_VAR_NAME_29d0234b9445d447:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_29d0234b9445d447
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_29d0234b9445d447:
	.quad	L_OBJC_METH_VAR_NAME_29d0234b9445d447

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e91a5376ddae3cfc
L_OBJC_METH_VAR_NAME_e91a5376ddae3cfc:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_e91a5376ddae3cfc
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_e91a5376ddae3cfc:
	.quad	L_OBJC_METH_VAR_NAME_e91a5376ddae3cfc

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_a23daf114eba1518
L_OBJC_METH_VAR_NAME_a23daf114eba1518:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_a23daf114eba1518
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_a23daf114eba1518:
	.quad	L_OBJC_METH_VAR_NAME_a23daf114eba1518

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2668dedcc69bf8fb
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2668dedcc69bf8fb:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_9303807037ba4f9f
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_9303807037ba4f9f:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_ff60dae1998e0c85
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ff60dae1998e0c85:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_29d0234b9445d447
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_29d0234b9445d447:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_e91a5376ddae3cfc
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_e91a5376ddae3cfc:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_a23daf114eba1518
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a23daf114eba1518:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
