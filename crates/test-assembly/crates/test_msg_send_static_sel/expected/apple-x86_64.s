	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_with_sel
	.p2align	4, 0x90
_handle_with_sel:
	push	rbp
	mov	rbp, rsp
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_ad1b815073641351]
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
	mov	rbx, qword ptr [rax]
	call	_objc_alloc
	mov	r14, rax
	mov	rdi, rax
	mov	rsi, rbx
	call	_objc_msgSend
	test	rax, rax
	je	LBB1_2
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB1_2:
	lea	rdx, [rip + l_anon.[ID].1]
	mov	rdi, r14
	mov	rsi, rbx
	call	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<3_u8> as objc2::__macro_helpers::msg_send_id::MsgSendIdFailed>::failed::GENERATED_ID, 0)

	.globl	_use_generic
	.p2align	4, 0x90
_use_generic:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdi
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_1f1c7bd8029c3138]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05]
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_eb5b4d2de37744da]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05]
	mov	rdi, rbx
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_c76827c00227cd8a]
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
	.p2align	3, 0x0
l_anon.[ID].1:
	.quad	l_anon.[ID].0
	.asciz	";\000\000\000\000\000\000\000\016\000\000\000\005\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_ad1b815073641351
L_OBJC_METH_VAR_NAME_ad1b815073641351:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_ad1b815073641351
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_ad1b815073641351:
	.quad	L_OBJC_METH_VAR_NAME_ad1b815073641351

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_ad1b815073641351
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ad1b815073641351:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_5ace898e385eba05
L_OBJC_METH_VAR_NAME_5ace898e385eba05:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05:
	.quad	L_OBJC_METH_VAR_NAME_5ace898e385eba05

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_5ace898e385eba05
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_5ace898e385eba05:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_1f1c7bd8029c3138
L_OBJC_METH_VAR_NAME_1f1c7bd8029c3138:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_1f1c7bd8029c3138
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_1f1c7bd8029c3138:
	.quad	L_OBJC_METH_VAR_NAME_1f1c7bd8029c3138

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_1f1c7bd8029c3138
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_1f1c7bd8029c3138:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_eb5b4d2de37744da
L_OBJC_METH_VAR_NAME_eb5b4d2de37744da:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_eb5b4d2de37744da
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_eb5b4d2de37744da:
	.quad	L_OBJC_METH_VAR_NAME_eb5b4d2de37744da

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_eb5b4d2de37744da
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_eb5b4d2de37744da:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_c76827c00227cd8a
L_OBJC_METH_VAR_NAME_c76827c00227cd8a:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_c76827c00227cd8a
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_c76827c00227cd8a:
	.quad	L_OBJC_METH_VAR_NAME_c76827c00227cd8a

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_c76827c00227cd8a
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_c76827c00227cd8a:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
