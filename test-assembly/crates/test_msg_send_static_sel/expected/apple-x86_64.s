	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_with_sel
	.p2align	4, 0x90
_handle_with_sel:
	push	rbp
	mov	rbp, rsp
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_40f5b12005284286]
	pop	rbp
	jmp	_objc_msgSend

	.globl	_handle_alloc_init
	.p2align	4, 0x90
_handle_alloc_init:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9]
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9]
	call	_objc_msgSend
	mov	rdi, rax
	mov	rsi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_msgSend

	.globl	_use_generic
	.p2align	4, 0x90
_use_generic:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdi
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_31f63858e271db32]
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_31f63858e271db32]
	mov	rdi, rbx
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_31f63858e271db32]
	mov	rdi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_msgSend

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_40f5b12005284286
	.p2align	2
L_OBJC_IMAGE_INFO_40f5b12005284286:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_40f5b12005284286
L_OBJC_METH_VAR_NAME_40f5b12005284286:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_40f5b12005284286
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_40f5b12005284286:
	.quad	L_OBJC_METH_VAR_NAME_40f5b12005284286

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_904c14aa63c4eec9
	.p2align	2
L_OBJC_IMAGE_INFO_904c14aa63c4eec9:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_904c14aa63c4eec9
L_OBJC_METH_VAR_NAME_904c14aa63c4eec9:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9:
	.quad	L_OBJC_METH_VAR_NAME_904c14aa63c4eec9

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_b1ab35d3713395f9
	.p2align	2
L_OBJC_IMAGE_INFO_b1ab35d3713395f9:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_b1ab35d3713395f9
L_OBJC_METH_VAR_NAME_b1ab35d3713395f9:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9:
	.quad	L_OBJC_METH_VAR_NAME_b1ab35d3713395f9

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_31f63858e271db32
	.p2align	2
L_OBJC_IMAGE_INFO_31f63858e271db32:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_31f63858e271db32
L_OBJC_METH_VAR_NAME_31f63858e271db32:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_31f63858e271db32
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_31f63858e271db32:
	.quad	L_OBJC_METH_VAR_NAME_31f63858e271db32

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_cdfe92d39025fdf4
	.p2align	2
L_OBJC_IMAGE_INFO_cdfe92d39025fdf4:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4
L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4:
	.quad	L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_79bd65c86d46fbf1
	.p2align	2
L_OBJC_IMAGE_INFO_79bd65c86d46fbf1:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1
L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1:
	.quad	L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_8e0840c6b39b7720
	.p2align	2
L_OBJC_IMAGE_INFO_8e0840c6b39b7720:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_8e0840c6b39b7720
L_OBJC_METH_VAR_NAME_8e0840c6b39b7720:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720:
	.quad	L_OBJC_METH_VAR_NAME_8e0840c6b39b7720

.subsections_via_symbols
