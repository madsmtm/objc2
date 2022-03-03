	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle_with_sel
	.p2align	2
	.code	32
_handle_with_sel:
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r0
	bl	__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround17h8840e188dc899306E
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
	bl	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h6885ad1b2e8dc9baE
	mov	r5, r0
	bl	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h58e80aa8a9adf276E
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
	bl	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h2bd60fb737aa3a73E
	mov	r5, r0
	bl	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h099d48e890343afaE
	mov	r2, r0
	mov	r0, r4
	mov	r1, r5
	bl	_objc_msgSend
	bl	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17ha61194771669e257E
	mov	r5, r0
	bl	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h099d48e890343afaE
	mov	r2, r0
	mov	r0, r4
	mov	r1, r5
	bl	_objc_msgSend
	bl	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h0981e22b69330471E
	mov	r5, r0
	bl	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h099d48e890343afaE
	mov	r2, r0
	mov	r0, r4
	mov	r1, r5
	pop	{r4, r5, r7, lr}
	b	_objc_msgSend

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround17h8840e188dc899306E:
	movw	r0, :lower16:(__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround3REF17h3880a6b55dd508fdE-(LPC3_0+8))
	movt	r0, :upper16:(__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround3REF17h3880a6b55dd508fdE-(LPC3_0+8))
LPC3_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h6885ad1b2e8dc9baE:
	movw	r0, :lower16:(__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17hb403c2227a4556eaE-(LPC4_0+8))
	movt	r0, :upper16:(__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17hb403c2227a4556eaE-(LPC4_0+8))
LPC4_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h58e80aa8a9adf276E:
	movw	r0, :lower16:(__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h9a421e5e16879b72E-(LPC5_0+8))
	movt	r0, :upper16:(__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h9a421e5e16879b72E-(LPC5_0+8))
LPC5_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h099d48e890343afaE:
	movw	r0, :lower16:(__ZN24test_msg_send_static_sel7generic22objc_static_workaround3REF17h04ee696a1f722b85E-(LPC6_0+8))
	movt	r0, :upper16:(__ZN24test_msg_send_static_sel7generic22objc_static_workaround3REF17h04ee696a1f722b85E-(LPC6_0+8))
LPC6_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h2bd60fb737aa3a73E:
	movw	r0, :lower16:(__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h67da2d48b5d85e71E-(LPC7_0+8))
	movt	r0, :upper16:(__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h67da2d48b5d85e71E-(LPC7_0+8))
LPC7_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17ha61194771669e257E:
	movw	r0, :lower16:(__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17hec3e445d56420b81E-(LPC8_0+8))
	movt	r0, :upper16:(__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17hec3e445d56420b81E-(LPC8_0+8))
LPC8_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h0981e22b69330471E:
	movw	r0, :lower16:(__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h8288fa628cf38b02E-(LPC9_0+8))
	movt	r0, :upper16:(__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h8288fa628cf38b02E-(LPC9_0+8))
LPC9_0:
	ldr	r0, [pc, r0]
	bx	lr

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround4NAME17h374ccf94d0633775E:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround3REF17h3880a6b55dd508fdE:
	.long	__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround4NAME17h374ccf94d0633775E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17h4549b59da4b18fa1E:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17hb403c2227a4556eaE:
	.long	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17h4549b59da4b18fa1E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17h43004b1b9153fea2E:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h9a421e5e16879b72E:
	.long	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17h43004b1b9153fea2E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel7generic22objc_static_workaround4NAME17h76925b222cc28afaE:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel7generic22objc_static_workaround3REF17h04ee696a1f722b85E:
	.long	__ZN24test_msg_send_static_sel7generic22objc_static_workaround4NAME17h76925b222cc28afaE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17hb9c72b88c79dd4b6E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h67da2d48b5d85e71E:
	.long	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17hb9c72b88c79dd4b6E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17hf7d8172547f73868E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17hec3e445d56420b81E:
	.long	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17hf7d8172547f73868E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17h4c7e1e7c7657946eE:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h8288fa628cf38b02E:
	.long	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17h4c7e1e7c7657946eE

.subsections_via_symbols
