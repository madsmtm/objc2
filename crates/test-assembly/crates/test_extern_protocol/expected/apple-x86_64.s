	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_fn1_get_protocol
	.p2align	4
_fn1_get_protocol:
	push	rbp
	mov	rbp, rsp
	lea	rdi, [rip + l_anon.[ID].0]
	mov	esi, 10
	pop	rbp
	jmp	SYM(objc2::top_level_traits::get_protocol::GENERATED_ID, 0)

	.globl	_fn2_dyn_call
	.p2align	4
_fn2_dyn_call:
	push	rbp
	mov	rbp, rsp
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_a3f3690bc9f113ac]
	pop	rbp
	jmp	_objc_msgSend

	.globl	_fn3_dyn_consume
	.p2align	4
_fn3_dyn_consume:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdi
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_a3f3690bc9f113ac]
	call	_objc_msgSend
	mov	rdi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_release

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_a3f3690bc9f113ac
L_OBJC_METH_VAR_NAME_a3f3690bc9f113ac:
	.asciz	"aMethod"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_a3f3690bc9f113ac
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_a3f3690bc9f113ac:
	.quad	L_OBJC_METH_VAR_NAME_a3f3690bc9f113ac

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_a3f3690bc9f113ac
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a3f3690bc9f113ac:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"MyProtocol"

.subsections_via_symbols
