	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_get_protocol
	.p2align	2
	.code	32
_get_protocol:
	movw	r0, :lower16:(l_anon.[ID].0-(LPC0_0+8))
	movt	r0, :upper16:(l_anon.[ID].0-(LPC0_0+8))
LPC0_0:
	add	r0, pc, r0
	mov	r1, #10
	b	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)

	.globl	_dyn_call
	.p2align	2
	.code	32
_dyn_call:
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_b79c3c5185d5ed67-(LPC1_0+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_b79c3c5185d5ed67-(LPC1_0+8))
LPC1_0:
	ldr	r1, [pc, r1]
	b	_objc_msgSend

	.globl	_dyn_consume
	.p2align	2
	.code	32
_dyn_consume:
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r0
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_b79c3c5185d5ed67-(LPC2_0+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_b79c3c5185d5ed67-(LPC2_0+8))
LPC2_0:
	ldr	r1, [pc, r1]
	bl	_objc_msgSend
	mov	r0, r4
	pop	{r4, r7, lr}
	b	_objc_release

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"MyProtocol"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_b79c3c5185d5ed67
L_OBJC_METH_VAR_NAME_b79c3c5185d5ed67:
	.asciz	"aMethod"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_b79c3c5185d5ed67
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_b79c3c5185d5ed67:
	.long	L_OBJC_METH_VAR_NAME_b79c3c5185d5ed67

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_b79c3c5185d5ed67
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_b79c3c5185d5ed67:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
