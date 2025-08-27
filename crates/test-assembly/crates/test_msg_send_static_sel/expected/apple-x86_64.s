	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_fn1_handle_with_sel
	.p2align	4
_fn1_handle_with_sel:
	push	rbp
	mov	rbp, rsp
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_66d4c61f523c7074]
	pop	rbp
	jmp	_objc_msgSend

	.globl	_fn2_handle_alloc_init
	.p2align	4
_fn2_handle_alloc_init:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_alloc@GOTPCREL]
	mov	rsi, qword ptr [rax]
	call	_objc_msgSend
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
	call	SYM(objc2::__macros::retain_semantics::init_fail::GENERATED_ID, 0)

	.globl	_fn3_use_generic
	.p2align	4
_fn3_use_generic:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdi
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_8d4579a56572fa21]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e]
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_d018146ff130bbd9]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e]
	mov	rdi, rbx
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_6659046596384437]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e]
	mov	rdi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_msgSend

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6659046596384437
L_OBJC_METH_VAR_NAME_6659046596384437:
	.asciz	"performSelector:"

	.globl	L_OBJC_METH_VAR_NAME_66d4c61f523c7074
L_OBJC_METH_VAR_NAME_66d4c61f523c7074:
	.asciz	"someSelector"

	.globl	L_OBJC_METH_VAR_NAME_8d4579a56572fa21
L_OBJC_METH_VAR_NAME_8d4579a56572fa21:
	.asciz	"performSelector:"

	.globl	L_OBJC_METH_VAR_NAME_d018146ff130bbd9
L_OBJC_METH_VAR_NAME_d018146ff130bbd9:
	.asciz	"performSelector:"

	.globl	L_OBJC_METH_VAR_NAME_e348e18690a1021e
L_OBJC_METH_VAR_NAME_e348e18690a1021e:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_6659046596384437
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_6659046596384437:
	.quad	L_OBJC_METH_VAR_NAME_6659046596384437

	.globl	L_OBJC_SELECTOR_REFERENCES_66d4c61f523c7074
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_66d4c61f523c7074:
	.quad	L_OBJC_METH_VAR_NAME_66d4c61f523c7074

	.globl	L_OBJC_SELECTOR_REFERENCES_8d4579a56572fa21
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_8d4579a56572fa21:
	.quad	L_OBJC_METH_VAR_NAME_8d4579a56572fa21

	.globl	L_OBJC_SELECTOR_REFERENCES_d018146ff130bbd9
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_d018146ff130bbd9:
	.quad	L_OBJC_METH_VAR_NAME_d018146ff130bbd9

	.globl	L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e:
	.quad	L_OBJC_METH_VAR_NAME_e348e18690a1021e

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_6659046596384437
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_6659046596384437:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_66d4c61f523c7074
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_66d4c61f523c7074:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_8d4579a56572fa21
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_8d4579a56572fa21:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_d018146ff130bbd9
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_d018146ff130bbd9:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_e348e18690a1021e
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_e348e18690a1021e:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].0:
	.asciz	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].1:
	.quad	L_anon.[ID].0
	.asciz	";\000\000\000\000\000\000\000\016\000\000\000\005\000\000"

.subsections_via_symbols
