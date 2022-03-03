	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle_with_sel
	.p2align	2
	.code	32
_handle_with_sel:
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r0
	bl	__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround17hf2801f55a2385362E
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
	bl	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h7c6d883b994711aeE
	mov	r5, r0
	bl	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17hac26443520976ad4E
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
	bl	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h945fbaf0c124eab9E
	mov	r5, r0
	bl	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h376904f2996327c9E
	mov	r2, r0
	mov	r0, r4
	mov	r1, r5
	bl	_objc_msgSend
	bl	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h03112b6cac679e4bE
	mov	r5, r0
	bl	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h376904f2996327c9E
	mov	r2, r0
	mov	r0, r4
	mov	r1, r5
	bl	_objc_msgSend
	bl	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h17015ecab548b281E
	mov	r5, r0
	bl	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h376904f2996327c9E
	mov	r2, r0
	mov	r0, r4
	mov	r1, r5
	bl	_objc_msgSend
	pop	{r4, r5, r7, pc}

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround17hf2801f55a2385362E:
	movw	r0, :lower16:(__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround3REF17hce6b3d95b6bcbb85E-(LPC3_0+8))
	movt	r0, :upper16:(__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround3REF17hce6b3d95b6bcbb85E-(LPC3_0+8))
LPC3_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h7c6d883b994711aeE:
	movw	r0, :lower16:(__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h3f5f4ba3789d7134E-(LPC4_0+8))
	movt	r0, :upper16:(__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h3f5f4ba3789d7134E-(LPC4_0+8))
LPC4_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17hac26443520976ad4E:
	movw	r0, :lower16:(__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h2fb4ecd06397a741E-(LPC5_0+8))
	movt	r0, :upper16:(__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h2fb4ecd06397a741E-(LPC5_0+8))
LPC5_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h376904f2996327c9E:
	movw	r0, :lower16:(__ZN24test_msg_send_static_sel7generic22objc_static_workaround3REF17h54efb4374be3eaadE-(LPC6_0+8))
	movt	r0, :upper16:(__ZN24test_msg_send_static_sel7generic22objc_static_workaround3REF17h54efb4374be3eaadE-(LPC6_0+8))
LPC6_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h945fbaf0c124eab9E:
	movw	r0, :lower16:(__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17hd4fed20bec515f63E-(LPC7_0+8))
	movt	r0, :upper16:(__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17hd4fed20bec515f63E-(LPC7_0+8))
LPC7_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h03112b6cac679e4bE:
	movw	r0, :lower16:(__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h45fc54a9172d9f80E-(LPC8_0+8))
	movt	r0, :upper16:(__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h45fc54a9172d9f80E-(LPC8_0+8))
LPC8_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h17015ecab548b281E:
	movw	r0, :lower16:(__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17hf335b3712f9d9cf1E-(LPC9_0+8))
	movt	r0, :upper16:(__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17hf335b3712f9d9cf1E-(LPC9_0+8))
LPC9_0:
	ldr	r0, [pc, r0]
	bx	lr

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround4NAME17h60ee841e6e2b2881E:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround3REF17hce6b3d95b6bcbb85E:
	.long	__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround4NAME17h60ee841e6e2b2881E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17hb6f6b554e76f0d73E:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h3f5f4ba3789d7134E:
	.long	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17hb6f6b554e76f0d73E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17ha71a9b8e90260cc1E:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h2fb4ecd06397a741E:
	.long	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17ha71a9b8e90260cc1E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel7generic22objc_static_workaround4NAME17hd2b41be12cc743f1E:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel7generic22objc_static_workaround3REF17h54efb4374be3eaadE:
	.long	__ZN24test_msg_send_static_sel7generic22objc_static_workaround4NAME17hd2b41be12cc743f1E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17hd419b68b3cb425bbE:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17hd4fed20bec515f63E:
	.long	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17hd419b68b3cb425bbE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17he119ab22173d8451E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h45fc54a9172d9f80E:
	.long	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17he119ab22173d8451E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17h77c600195720ccb0E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17hf335b3712f9d9cf1E:
	.long	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17h77c600195720ccb0E

.subsections_via_symbols
