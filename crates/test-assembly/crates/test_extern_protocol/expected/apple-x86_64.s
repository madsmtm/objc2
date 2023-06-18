	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_protocol
	.p2align	4, 0x90
_get_protocol:
	push	rbp
	mov	rbp, rsp
	lea	rdi, [rip + l_anon.[ID].0]
	mov	esi, 10
	pop	rbp
	jmp	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)

	.globl	_dyn_call
	.p2align	4, 0x90
_dyn_call:
	push	rbp
	mov	rbp, rsp
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_17aa92881c42487f]
	pop	rbp
	jmp	_objc_msgSend

	.globl	_dyn_consume
	.p2align	4, 0x90
_dyn_consume:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdi
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_17aa92881c42487f]
	call	_objc_msgSend
	mov	rdi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_release

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"MyProtocol"

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_17aa92881c42487f
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_17aa92881c42487f:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_17aa92881c42487f
L_OBJC_METH_VAR_NAME_17aa92881c42487f:
	.asciz	"aMethod"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_17aa92881c42487f
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_17aa92881c42487f:
	.quad	L_OBJC_METH_VAR_NAME_17aa92881c42487f

.subsections_via_symbols
