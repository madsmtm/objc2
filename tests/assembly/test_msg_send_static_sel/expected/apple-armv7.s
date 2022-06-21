	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle_with_sel
	.p2align	2
	.code	32
_handle_with_sel:
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r0
	bl	__RNvNvCs7mRPQtePJS0_24test_msg_send_static_sel15handle_with_sel22objc_static_workaround
	mov	r1, r0
	mov	r0, r4
	pop	{r4, r7, lr}
	b	_objc_msgSend

	.globl	_handle_alloc_init
	.p2align	2
	.code	32
_handle_alloc_init:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	mov	r4, r0
	bl	__RNvNvCs7mRPQtePJS0_24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround
	mov	r5, r0
	bl	__RNvNvCs7mRPQtePJS0_24test_msg_send_static_sel17handle_alloc_inits_22objc_static_workaround
	mov	r1, r0
	mov	r0, r4
	bl	_objc_msgSend
	mov	r1, r5
	pop	{r4, r5, r7, lr}
	b	_objc_msgSend

	.globl	_use_generic
	.p2align	2
	.code	32
_use_generic:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	mov	r4, r0
	bl	__RNvNvCs7mRPQtePJS0_24test_msg_send_static_sel11use_generic22objc_static_workaround
	mov	r5, r0
	bl	__RNvNvCs7mRPQtePJS0_24test_msg_send_static_sel7generic22objc_static_workaround
	mov	r2, r0
	mov	r0, r4
	mov	r1, r5
	bl	_objc_msgSend
	bl	__RNvNvCs7mRPQtePJS0_24test_msg_send_static_sel11use_generics_22objc_static_workaround
	mov	r5, r0
	bl	__RNvNvCs7mRPQtePJS0_24test_msg_send_static_sel7generic22objc_static_workaround
	mov	r2, r0
	mov	r0, r4
	mov	r1, r5
	bl	_objc_msgSend
	bl	__RNvNvCs7mRPQtePJS0_24test_msg_send_static_sel11use_generics0_22objc_static_workaround
	mov	r5, r0
	bl	__RNvNvCs7mRPQtePJS0_24test_msg_send_static_sel7generic22objc_static_workaround
	mov	r2, r0
	mov	r0, r4
	mov	r1, r5
	pop	{r4, r5, r7, lr}
	b	_objc_msgSend

	.p2align	2
	.code	32
__RNvNvCs7mRPQtePJS0_24test_msg_send_static_sel15handle_with_sel22objc_static_workaround:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_40f5b12005284286-(LPC3_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_40f5b12005284286-(LPC3_0+8))
LPC3_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__RNvNvCs7mRPQtePJS0_24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9-(LPC4_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9-(LPC4_0+8))
LPC4_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__RNvNvCs7mRPQtePJS0_24test_msg_send_static_sel17handle_alloc_inits_22objc_static_workaround:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9-(LPC5_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9-(LPC5_0+8))
LPC5_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__RNvNvCs7mRPQtePJS0_24test_msg_send_static_sel7generic22objc_static_workaround:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_31f63858e271db32-(LPC6_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_31f63858e271db32-(LPC6_0+8))
LPC6_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__RNvNvCs7mRPQtePJS0_24test_msg_send_static_sel11use_generic22objc_static_workaround:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4-(LPC7_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4-(LPC7_0+8))
LPC7_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__RNvNvCs7mRPQtePJS0_24test_msg_send_static_sel11use_generics_22objc_static_workaround:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1-(LPC8_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1-(LPC8_0+8))
LPC8_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__RNvNvCs7mRPQtePJS0_24test_msg_send_static_sel11use_generics0_22objc_static_workaround:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720-(LPC9_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720-(LPC9_0+8))
LPC9_0:
	ldr	r0, [pc, r0]
	bx	lr

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_40f5b12005284286
	.p2align	2
L_OBJC_IMAGE_INFO_40f5b12005284286:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_40f5b12005284286
L_OBJC_METH_VAR_NAME_40f5b12005284286:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_40f5b12005284286
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_40f5b12005284286:
	.long	L_OBJC_METH_VAR_NAME_40f5b12005284286

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_904c14aa63c4eec9
	.p2align	2
L_OBJC_IMAGE_INFO_904c14aa63c4eec9:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_904c14aa63c4eec9
L_OBJC_METH_VAR_NAME_904c14aa63c4eec9:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9:
	.long	L_OBJC_METH_VAR_NAME_904c14aa63c4eec9

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_b1ab35d3713395f9
	.p2align	2
L_OBJC_IMAGE_INFO_b1ab35d3713395f9:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_b1ab35d3713395f9
L_OBJC_METH_VAR_NAME_b1ab35d3713395f9:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9:
	.long	L_OBJC_METH_VAR_NAME_b1ab35d3713395f9

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_31f63858e271db32
	.p2align	2
L_OBJC_IMAGE_INFO_31f63858e271db32:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_31f63858e271db32
L_OBJC_METH_VAR_NAME_31f63858e271db32:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_31f63858e271db32
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_31f63858e271db32:
	.long	L_OBJC_METH_VAR_NAME_31f63858e271db32

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_cdfe92d39025fdf4
	.p2align	2
L_OBJC_IMAGE_INFO_cdfe92d39025fdf4:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4
L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4:
	.long	L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_79bd65c86d46fbf1
	.p2align	2
L_OBJC_IMAGE_INFO_79bd65c86d46fbf1:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1
L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1:
	.long	L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_8e0840c6b39b7720
	.p2align	2
L_OBJC_IMAGE_INFO_8e0840c6b39b7720:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_8e0840c6b39b7720
L_OBJC_METH_VAR_NAME_8e0840c6b39b7720:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720:
	.long	L_OBJC_METH_VAR_NAME_8e0840c6b39b7720

.subsections_via_symbols
