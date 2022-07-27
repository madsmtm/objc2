	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_with_sel
	.p2align	4, 0x90
_handle_with_sel:
	push	rbp
	mov	rbp, rsp
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc]
	pop	rbp
	jmp	_objc_msgSend

	.globl	_handle_alloc_init
	.p2align	4, 0x90
_handle_alloc_init:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_init@GOTPCREL]
	mov	rbx, qword ptr [rax]
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_alloc@GOTPCREL]
	mov	rsi, qword ptr [rax]
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
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_f16064a6f68ca673]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_457d234345d46cbe]
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_9f134b97cb598446]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_457d234345d46cbe]
	mov	rdi, rbx
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_e76e01e8b2327e5d]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_457d234345d46cbe]
	mov	rdi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_msgSend

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_044375a4329d08dc
	.p2align	2
L_OBJC_IMAGE_INFO_044375a4329d08dc:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_044375a4329d08dc
L_OBJC_METH_VAR_NAME_044375a4329d08dc:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc:
	.quad	L_OBJC_METH_VAR_NAME_044375a4329d08dc

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_457d234345d46cbe
	.p2align	2
L_OBJC_IMAGE_INFO_457d234345d46cbe:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_457d234345d46cbe
L_OBJC_METH_VAR_NAME_457d234345d46cbe:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_457d234345d46cbe
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_457d234345d46cbe:
	.quad	L_OBJC_METH_VAR_NAME_457d234345d46cbe

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f16064a6f68ca673
	.p2align	2
L_OBJC_IMAGE_INFO_f16064a6f68ca673:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_f16064a6f68ca673
L_OBJC_METH_VAR_NAME_f16064a6f68ca673:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_f16064a6f68ca673
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_f16064a6f68ca673:
	.quad	L_OBJC_METH_VAR_NAME_f16064a6f68ca673

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_9f134b97cb598446
	.p2align	2
L_OBJC_IMAGE_INFO_9f134b97cb598446:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_9f134b97cb598446
L_OBJC_METH_VAR_NAME_9f134b97cb598446:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_9f134b97cb598446
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_9f134b97cb598446:
	.quad	L_OBJC_METH_VAR_NAME_9f134b97cb598446

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_e76e01e8b2327e5d
	.p2align	2
L_OBJC_IMAGE_INFO_e76e01e8b2327e5d:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e76e01e8b2327e5d
L_OBJC_METH_VAR_NAME_e76e01e8b2327e5d:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_e76e01e8b2327e5d
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_e76e01e8b2327e5d:
	.quad	L_OBJC_METH_VAR_NAME_e76e01e8b2327e5d

.subsections_via_symbols
