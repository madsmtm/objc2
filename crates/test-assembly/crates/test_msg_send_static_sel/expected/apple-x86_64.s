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
	push	r14
	push	rbx
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_init@GOTPCREL]
	mov	r14, qword ptr [rax]
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_alloc@GOTPCREL]
	mov	rsi, qword ptr [rax]
	call	_objc_msgSend
	mov	rbx, rax
	mov	rdi, rax
	mov	rsi, r14
	call	_objc_msgSend
	test	rax, rax
	je	LBB1_2
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB1_2:
	lea	rdx, [rip + l_anon.[ID].1]
	mov	rdi, rbx
	mov	rsi, r14
	call	SYM(<objc2::__macro_helpers::RetainSemantics<3_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)

	.globl	_use_generic
	.p2align	4, 0x90
_use_generic:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdi
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_36a6e334f5aeb023]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05]
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_2c3c38f6ea036343]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05]
	mov	rdi, rbx
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_e1e97023e8bcf6a4]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05]
	mov	rdi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_msgSend

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3
l_anon.[ID].1:
	.quad	l_anon.[ID].0
	.asciz	";\000\000\000\000\000\000\000\016\000\000\000\005\000\000"

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
	.globl	L_OBJC_IMAGE_INFO_5ace898e385eba05
	.p2align	2
L_OBJC_IMAGE_INFO_5ace898e385eba05:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_5ace898e385eba05
L_OBJC_METH_VAR_NAME_5ace898e385eba05:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05:
	.quad	L_OBJC_METH_VAR_NAME_5ace898e385eba05

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_36a6e334f5aeb023
	.p2align	2
L_OBJC_IMAGE_INFO_36a6e334f5aeb023:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_36a6e334f5aeb023
L_OBJC_METH_VAR_NAME_36a6e334f5aeb023:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_36a6e334f5aeb023
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_36a6e334f5aeb023:
	.quad	L_OBJC_METH_VAR_NAME_36a6e334f5aeb023

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2c3c38f6ea036343
	.p2align	2
L_OBJC_IMAGE_INFO_2c3c38f6ea036343:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2c3c38f6ea036343
L_OBJC_METH_VAR_NAME_2c3c38f6ea036343:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2c3c38f6ea036343
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_2c3c38f6ea036343:
	.quad	L_OBJC_METH_VAR_NAME_2c3c38f6ea036343

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_e1e97023e8bcf6a4
	.p2align	2
L_OBJC_IMAGE_INFO_e1e97023e8bcf6a4:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e1e97023e8bcf6a4
L_OBJC_METH_VAR_NAME_e1e97023e8bcf6a4:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_e1e97023e8bcf6a4
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_e1e97023e8bcf6a4:
	.quad	L_OBJC_METH_VAR_NAME_e1e97023e8bcf6a4

.subsections_via_symbols
