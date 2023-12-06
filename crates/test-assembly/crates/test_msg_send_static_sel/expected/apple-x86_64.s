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
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_124acdf5c403f4b7]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_f0e0414fd9875b81]
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_2949aa14a0c1ed09]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_f0e0414fd9875b81]
	mov	rdi, rbx
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_cfad9aae979f14dd]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_f0e0414fd9875b81]
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
	.globl	L_OBJC_METH_VAR_NAME_f0e0414fd9875b81
L_OBJC_METH_VAR_NAME_f0e0414fd9875b81:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_f0e0414fd9875b81
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_f0e0414fd9875b81:
	.quad	L_OBJC_METH_VAR_NAME_f0e0414fd9875b81

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f0e0414fd9875b81
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f0e0414fd9875b81:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_124acdf5c403f4b7
L_OBJC_METH_VAR_NAME_124acdf5c403f4b7:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_124acdf5c403f4b7
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_124acdf5c403f4b7:
	.quad	L_OBJC_METH_VAR_NAME_124acdf5c403f4b7

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_124acdf5c403f4b7
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_124acdf5c403f4b7:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2949aa14a0c1ed09
L_OBJC_METH_VAR_NAME_2949aa14a0c1ed09:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2949aa14a0c1ed09
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_2949aa14a0c1ed09:
	.quad	L_OBJC_METH_VAR_NAME_2949aa14a0c1ed09

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2949aa14a0c1ed09
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2949aa14a0c1ed09:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_cfad9aae979f14dd
L_OBJC_METH_VAR_NAME_cfad9aae979f14dd:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_cfad9aae979f14dd
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_cfad9aae979f14dd:
	.quad	L_OBJC_METH_VAR_NAME_cfad9aae979f14dd

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_cfad9aae979f14dd
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_cfad9aae979f14dd:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
