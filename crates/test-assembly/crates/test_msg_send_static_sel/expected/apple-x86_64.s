	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_with_sel
	.p2align	4, 0x90
_handle_with_sel:
	push	rbp
	mov	rbp, rsp
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_77d2b75bddfbef7c]
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
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_alloc@GOTPCREL]
	mov	rsi, qword ptr [rax]
	call	_objc_msgSend
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
	call	SYM(<objc2::__macro_helpers::RetainSemantics<3_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)

	.globl	_use_generic
	.p2align	4, 0x90
_use_generic:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdi
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_038d21a6277de1da]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_379095321e06c060]
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_573c1e9c42ae1ea1]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_379095321e06c060]
	mov	rdi, rbx
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_9885c1be4d03110d]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_379095321e06c060]
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

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_77d2b75bddfbef7c
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_77d2b75bddfbef7c:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_77d2b75bddfbef7c
L_OBJC_METH_VAR_NAME_77d2b75bddfbef7c:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_77d2b75bddfbef7c
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_77d2b75bddfbef7c:
	.quad	L_OBJC_METH_VAR_NAME_77d2b75bddfbef7c

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_379095321e06c060
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_379095321e06c060:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_379095321e06c060
L_OBJC_METH_VAR_NAME_379095321e06c060:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_379095321e06c060
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_379095321e06c060:
	.quad	L_OBJC_METH_VAR_NAME_379095321e06c060

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_038d21a6277de1da
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_038d21a6277de1da:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_038d21a6277de1da
L_OBJC_METH_VAR_NAME_038d21a6277de1da:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_038d21a6277de1da
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_038d21a6277de1da:
	.quad	L_OBJC_METH_VAR_NAME_038d21a6277de1da

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_573c1e9c42ae1ea1
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_573c1e9c42ae1ea1:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_573c1e9c42ae1ea1
L_OBJC_METH_VAR_NAME_573c1e9c42ae1ea1:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_573c1e9c42ae1ea1
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_573c1e9c42ae1ea1:
	.quad	L_OBJC_METH_VAR_NAME_573c1e9c42ae1ea1

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_9885c1be4d03110d
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_9885c1be4d03110d:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_9885c1be4d03110d
L_OBJC_METH_VAR_NAME_9885c1be4d03110d:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_9885c1be4d03110d
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_9885c1be4d03110d:
	.quad	L_OBJC_METH_VAR_NAME_9885c1be4d03110d

.subsections_via_symbols
