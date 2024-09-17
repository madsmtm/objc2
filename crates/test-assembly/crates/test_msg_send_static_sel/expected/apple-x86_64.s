	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_with_sel
	.p2align	4, 0x90
_handle_with_sel:
	push	rbp
	mov	rbp, rsp
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_664c1e40eb8cd76e]
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
	call	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<3_u8> as objc2::__macro_helpers::msg_send_retained::MsgSendIdFailed>::failed::GENERATED_ID, 0)

	.globl	_use_generic
	.p2align	4, 0x90
_use_generic:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdi
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_e4e4edcd2d17efb8]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_80036160fc60677b]
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_bf9373a91792acd9]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_80036160fc60677b]
	mov	rdi, rbx
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_65f663aa0a6ddc1d]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_80036160fc60677b]
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
	.globl	L_OBJC_METH_VAR_NAME_664c1e40eb8cd76e
L_OBJC_METH_VAR_NAME_664c1e40eb8cd76e:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_664c1e40eb8cd76e
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_664c1e40eb8cd76e:
	.quad	L_OBJC_METH_VAR_NAME_664c1e40eb8cd76e

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_664c1e40eb8cd76e
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_664c1e40eb8cd76e:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_80036160fc60677b
L_OBJC_METH_VAR_NAME_80036160fc60677b:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_80036160fc60677b
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_80036160fc60677b:
	.quad	L_OBJC_METH_VAR_NAME_80036160fc60677b

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_80036160fc60677b
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_80036160fc60677b:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e4e4edcd2d17efb8
L_OBJC_METH_VAR_NAME_e4e4edcd2d17efb8:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_e4e4edcd2d17efb8
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_e4e4edcd2d17efb8:
	.quad	L_OBJC_METH_VAR_NAME_e4e4edcd2d17efb8

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_e4e4edcd2d17efb8
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_e4e4edcd2d17efb8:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_bf9373a91792acd9
L_OBJC_METH_VAR_NAME_bf9373a91792acd9:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_bf9373a91792acd9
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_bf9373a91792acd9:
	.quad	L_OBJC_METH_VAR_NAME_bf9373a91792acd9

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_bf9373a91792acd9
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_bf9373a91792acd9:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_65f663aa0a6ddc1d
L_OBJC_METH_VAR_NAME_65f663aa0a6ddc1d:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_65f663aa0a6ddc1d
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_65f663aa0a6ddc1d:
	.quad	L_OBJC_METH_VAR_NAME_65f663aa0a6ddc1d

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_65f663aa0a6ddc1d
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_65f663aa0a6ddc1d:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
