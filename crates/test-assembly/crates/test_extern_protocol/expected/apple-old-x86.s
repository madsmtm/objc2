	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_fn1_get_protocol
	.p2align	4
_fn1_get_protocol:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	call	L0$pb
L0$pb:
	pop	eax
	sub	esp, 8
	lea	eax, [eax + l_anon.[ID].0-L0$pb]
	push	10
	push	eax
	call	SYM(objc2::top_level_traits::get_protocol::GENERATED_ID, 0)
	add	esp, 24
	pop	ebp
	ret

	.globl	_fn2_dyn_call
	.p2align	4
_fn2_dyn_call:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	call	L1$pb
L1$pb:
	pop	eax
	sub	esp, 8
	push	dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_a3f3690bc9f113ac-L1$pb]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 24
	pop	ebp
	ret

	.globl	_fn3_dyn_consume
	.p2align	4
_fn3_dyn_consume:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	call	L2$pb
L2$pb:
	pop	eax
	sub	esp, 8
	push	dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_a3f3690bc9f113ac-L2$pb]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 24
	pop	ebp
	jmp	_objc_release

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_a3f3690bc9f113ac
L_OBJC_METH_VAR_NAME_a3f3690bc9f113ac:
	.asciz	"aMethod"

	.section	__OBJC,__message_refs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_a3f3690bc9f113ac
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_a3f3690bc9f113ac:
	.long	L_OBJC_METH_VAR_NAME_a3f3690bc9f113ac

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"MyProtocol"

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_a3f3690bc9f113ac
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a3f3690bc9f113ac:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
