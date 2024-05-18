	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_with_sel
	.p2align	4, 0x90
_handle_with_sel:
	push	rbp
	mov	rbp, rsp
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_ac93c21013e05a7d]
	pop	rbp
	jmp	_objc_msgSend

	.globl	_handle_alloc_init
	.p2align	4, 0x90
_handle_alloc_init:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	call	_objc_alloc
	mov	rbx, rax
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_init@GOTPCREL]
	mov	r14, qword ptr [rax]
	mov	rdi, rbx
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
	call	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<3_u8> as objc2::__macro_helpers::msg_send_id::MsgSendIdFailed>::failed::GENERATED_ID, 0)

	.globl	_use_generic
	.p2align	4, 0x90
_use_generic:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdi
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_df2cc6f80edda422]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_8071e784bfc1c0df]
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_e520bf80aade9209]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_8071e784bfc1c0df]
	mov	rdi, rbx
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_bab7ff70ff0c78f0]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_8071e784bfc1c0df]
	mov	rdi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_msgSend

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].1:
	.quad	l_anon.[ID].0
	.asciz	";\000\000\000\000\000\000\000\016\000\000\000\005\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_ac93c21013e05a7d
L_OBJC_METH_VAR_NAME_ac93c21013e05a7d:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_ac93c21013e05a7d
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_ac93c21013e05a7d:
	.quad	L_OBJC_METH_VAR_NAME_ac93c21013e05a7d

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_ac93c21013e05a7d
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ac93c21013e05a7d:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_8071e784bfc1c0df
L_OBJC_METH_VAR_NAME_8071e784bfc1c0df:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_8071e784bfc1c0df
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_8071e784bfc1c0df:
	.quad	L_OBJC_METH_VAR_NAME_8071e784bfc1c0df

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_8071e784bfc1c0df
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_8071e784bfc1c0df:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_df2cc6f80edda422
L_OBJC_METH_VAR_NAME_df2cc6f80edda422:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_df2cc6f80edda422
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_df2cc6f80edda422:
	.quad	L_OBJC_METH_VAR_NAME_df2cc6f80edda422

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_df2cc6f80edda422
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_df2cc6f80edda422:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e520bf80aade9209
L_OBJC_METH_VAR_NAME_e520bf80aade9209:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_e520bf80aade9209
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_e520bf80aade9209:
	.quad	L_OBJC_METH_VAR_NAME_e520bf80aade9209

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_e520bf80aade9209
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_e520bf80aade9209:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_bab7ff70ff0c78f0
L_OBJC_METH_VAR_NAME_bab7ff70ff0c78f0:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_bab7ff70ff0c78f0
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_bab7ff70ff0c78f0:
	.quad	L_OBJC_METH_VAR_NAME_bab7ff70ff0c78f0

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_bab7ff70ff0c78f0
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_bab7ff70ff0c78f0:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
