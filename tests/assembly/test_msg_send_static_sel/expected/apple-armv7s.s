	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle_with_sel
	.p2align	2
	.code	32
_handle_with_sel:
	push	{r7, lr}
	mov	r7, sp
	movw	r1, :lower16:(__ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17h9a8ef11721428e5cE-(LPC0_0+8))
	movt	r1, :upper16:(__ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17h9a8ef11721428e5cE-(LPC0_0+8))
LPC0_0:
	ldr	r1, [pc, r1]
	bl	_objc_msgSend
	pop	{r7, pc}

	.globl	_handle_alloc_init
	.p2align	2
	.code	32
_handle_alloc_init:
	push	{r4, r7, lr}
	add	r7, sp, #4
	movw	r1, :lower16:(__MergedGlobals-(LPC1_0+8))
	movt	r1, :upper16:(__MergedGlobals-(LPC1_0+8))
	movw	r4, :lower16:(__MergedGlobals-(LPC1_1+8))
	movt	r4, :upper16:(__MergedGlobals-(LPC1_1+8))
LPC1_0:
	add	r1, pc, r1
LPC1_1:
	ldr	r4, [pc, r4]
	ldr	r1, [r1, #4]
	bl	_objc_msgSend
	mov	r1, r4
	bl	_objc_msgSend
	pop	{r4, r7, pc}

	.globl	_use_generic
	.p2align	2
	.code	32
_use_generic:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	movw	r5, :lower16:(__MergedGlobals-(LPC2_0+8))
	mov	r4, r0
	movt	r5, :upper16:(__MergedGlobals-(LPC2_0+8))
LPC2_0:
	add	r5, pc, r5
	ldr	r1, [r5, #12]
	ldr	r2, [r5, #8]
	bl	_objc_msgSend
	ldr	r1, [r5, #16]
	mov	r0, r4
	ldr	r2, [r5, #8]
	bl	_objc_msgSend
	ldr	r1, [r5, #20]
	mov	r0, r4
	ldr	r2, [r5, #8]
	bl	_objc_msgSend
	pop	{r4, r5, r7, pc}

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel15handle_with_sel5do_it5VALUE17h5efd63825ac22048E:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17h9a8ef11721428e5cE:
	.long	__ZN24test_msg_send_static_sel15handle_with_sel5do_it5VALUE17h5efd63825ac22048E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h441437e082275571E:
	.asciz	"init"

__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h511d9a3abe34514fE:
	.asciz	"alloc"

__ZN24test_msg_send_static_sel7generic5do_it5VALUE17h085bf46fbd37778eE:
	.asciz	"generic:selector:"

__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17hd2f614f4541f1fb6E:
	.asciz	"performSelector:"

__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h4daddbacc43585e3E:
	.asciz	"performSelector:"

__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17hce3635c78de3d40cE:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__MergedGlobals:
	.long	__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h441437e082275571E
	.long	__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h511d9a3abe34514fE
	.long	__ZN24test_msg_send_static_sel7generic5do_it5VALUE17h085bf46fbd37778eE
	.long	__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17hd2f614f4541f1fb6E
	.long	__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h4daddbacc43585e3E
	.long	__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17hce3635c78de3d40cE

.subsections_via_symbols
