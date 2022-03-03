	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle_with_sel
	.p2align	2
	.code	32
_handle_with_sel:
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r0
	bl	__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround17h8e8912e05611baf8E
	mov	r1, r0
	mov	r0, r4
	bl	_objc_msgSend
	pop	{r4, r7, pc}

	.globl	_handle_alloc_init
	.p2align	2
	.code	32
_handle_alloc_init:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	mov	r4, r0
	bl	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h98b7256dded064d1E
	mov	r5, r0
	bl	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h8bd68890a8be4b94E
	mov	r1, r0
	mov	r0, r4
	bl	_objc_msgSend
	mov	r1, r5
	bl	_objc_msgSend
	pop	{r4, r5, r7, pc}

	.globl	_use_generic
	.p2align	2
	.code	32
_use_generic:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	mov	r4, r0
	bl	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h4db106eab2ec1470E
	mov	r5, r0
	bl	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h137ede0f8414f244E
	mov	r2, r0
	mov	r0, r4
	mov	r1, r5
	bl	_objc_msgSend
	bl	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h68391fe30382cbbeE
	mov	r5, r0
	bl	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h137ede0f8414f244E
	mov	r2, r0
	mov	r0, r4
	mov	r1, r5
	bl	_objc_msgSend
	bl	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17hd7ee6447198a8703E
	mov	r5, r0
	bl	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h137ede0f8414f244E
	mov	r2, r0
	mov	r0, r4
	mov	r1, r5
	bl	_objc_msgSend
	pop	{r4, r5, r7, pc}

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround17h8e8912e05611baf8E:
	push	{r7, lr}
	mov	r7, sp
	bl	__ZN5objc210image_info17h43072185f7d30955E
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_40f5b12005284286-(LPC3_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_40f5b12005284286-(LPC3_0+8))
LPC3_0:
	ldr	r0, [pc, r0]
	pop	{r7, pc}

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h98b7256dded064d1E:
	push	{r7, lr}
	mov	r7, sp
	bl	__ZN5objc210image_info17h43072185f7d30955E
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9-(LPC4_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9-(LPC4_0+8))
LPC4_0:
	ldr	r0, [pc, r0]
	pop	{r7, pc}

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h8bd68890a8be4b94E:
	push	{r7, lr}
	mov	r7, sp
	bl	__ZN5objc210image_info17h43072185f7d30955E
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9-(LPC5_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9-(LPC5_0+8))
LPC5_0:
	ldr	r0, [pc, r0]
	pop	{r7, pc}

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h137ede0f8414f244E:
	push	{r7, lr}
	mov	r7, sp
	bl	__ZN5objc210image_info17h43072185f7d30955E
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_31f63858e271db32-(LPC6_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_31f63858e271db32-(LPC6_0+8))
LPC6_0:
	ldr	r0, [pc, r0]
	pop	{r7, pc}

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h4db106eab2ec1470E:
	push	{r7, lr}
	mov	r7, sp
	bl	__ZN5objc210image_info17h43072185f7d30955E
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4-(LPC7_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4-(LPC7_0+8))
LPC7_0:
	ldr	r0, [pc, r0]
	pop	{r7, pc}

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h68391fe30382cbbeE:
	push	{r7, lr}
	mov	r7, sp
	bl	__ZN5objc210image_info17h43072185f7d30955E
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1-(LPC8_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1-(LPC8_0+8))
LPC8_0:
	ldr	r0, [pc, r0]
	pop	{r7, pc}

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17hd7ee6447198a8703E:
	push	{r7, lr}
	mov	r7, sp
	bl	__ZN5objc210image_info17h43072185f7d30955E
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720-(LPC9_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720-(LPC9_0+8))
LPC9_0:
	ldr	r0, [pc, r0]
	pop	{r7, pc}

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_40f5b12005284286
L_OBJC_METH_VAR_NAME_40f5b12005284286:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_40f5b12005284286
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_40f5b12005284286:
	.long	L_OBJC_METH_VAR_NAME_40f5b12005284286

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_904c14aa63c4eec9
L_OBJC_METH_VAR_NAME_904c14aa63c4eec9:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9:
	.long	L_OBJC_METH_VAR_NAME_904c14aa63c4eec9

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_b1ab35d3713395f9
L_OBJC_METH_VAR_NAME_b1ab35d3713395f9:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9:
	.long	L_OBJC_METH_VAR_NAME_b1ab35d3713395f9

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_31f63858e271db32
L_OBJC_METH_VAR_NAME_31f63858e271db32:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_31f63858e271db32
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_31f63858e271db32:
	.long	L_OBJC_METH_VAR_NAME_31f63858e271db32

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4
L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4:
	.long	L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1
L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1:
	.long	L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1

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
