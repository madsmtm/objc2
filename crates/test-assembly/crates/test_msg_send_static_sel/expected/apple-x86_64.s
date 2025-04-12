	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_with_sel
	.p2align	4
_handle_with_sel:
	push	rbp
	mov	rbp, rsp
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_80f1580ed33ec51b]
	pop	rbp
	jmp	_objc_msgSend

	.globl	_handle_alloc_init
	.p2align	4
_handle_alloc_init:
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
	call	SYM(objc2::__macro_helpers::retain_semantics::init_fail::GENERATED_ID, 0)

	.globl	_use_generic
	.p2align	4
_use_generic:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdi
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_67bf3e41c7e639a3]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5]
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_2c2c9a8191012941]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5]
	mov	rdi, rbx
	call	_objc_msgSend
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_993d94b40d47ed52]
	mov	rdx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5]
	mov	rdi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_msgSend

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_80f1580ed33ec51b
L_OBJC_METH_VAR_NAME_80f1580ed33ec51b:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_80f1580ed33ec51b
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_80f1580ed33ec51b:
	.quad	L_OBJC_METH_VAR_NAME_80f1580ed33ec51b

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_91c006d97540f4b5
L_OBJC_METH_VAR_NAME_91c006d97540f4b5:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5:
	.quad	L_OBJC_METH_VAR_NAME_91c006d97540f4b5

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_67bf3e41c7e639a3
L_OBJC_METH_VAR_NAME_67bf3e41c7e639a3:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_67bf3e41c7e639a3
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_67bf3e41c7e639a3:
	.quad	L_OBJC_METH_VAR_NAME_67bf3e41c7e639a3

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2c2c9a8191012941
L_OBJC_METH_VAR_NAME_2c2c9a8191012941:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_2c2c9a8191012941
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_2c2c9a8191012941:
	.quad	L_OBJC_METH_VAR_NAME_2c2c9a8191012941

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_993d94b40d47ed52
L_OBJC_METH_VAR_NAME_993d94b40d47ed52:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_993d94b40d47ed52
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_993d94b40d47ed52:
	.quad	L_OBJC_METH_VAR_NAME_993d94b40d47ed52

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].1:
	.quad	l_anon.[ID].0
	.asciz	";\000\000\000\000\000\000\000\016\000\000\000\005\000\000"

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_80f1580ed33ec51b
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_80f1580ed33ec51b:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_91c006d97540f4b5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_91c006d97540f4b5:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_67bf3e41c7e639a3
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_67bf3e41c7e639a3:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_2c2c9a8191012941
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2c2c9a8191012941:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_993d94b40d47ed52
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_993d94b40d47ed52:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
