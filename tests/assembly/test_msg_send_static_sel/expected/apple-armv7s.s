	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle_with_sel
	.p2align	2
	.code	32
_handle_with_sel:
	push	{r7, lr}
	mov	r7, sp
	movw	r1, :lower16:(__ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17h5684820907e02a7dE-(LPC0_0+8))
	movt	r1, :upper16:(__ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17h5684820907e02a7dE-(LPC0_0+8))
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
__ZN24test_msg_send_static_sel15handle_with_sel5do_it4NAME17h1b045ce2d125ff39E:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17h5684820907e02a7dE:
	.long	__ZN24test_msg_send_static_sel15handle_with_sel5do_it4NAME17h1b045ce2d125ff39E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it4NAME17h4e0852fc05b66326E:
	.asciz	"init"

__ZN24test_msg_send_static_sel17handle_alloc_init5do_it4NAME17ha3a756441bd0acebE:
	.asciz	"alloc"

__ZN24test_msg_send_static_sel7generic5do_it4NAME17h4c3b369edbd3aadcE:
	.asciz	"generic:selector:"

__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17h639b2c9b06a336e7E:
	.asciz	"performSelector:"

__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17hd38879191654969cE:
	.asciz	"performSelector:"

__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17h558dcd606e4b144fE:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__MergedGlobals:
	.long	__ZN24test_msg_send_static_sel17handle_alloc_init5do_it4NAME17h4e0852fc05b66326E
	.long	__ZN24test_msg_send_static_sel17handle_alloc_init5do_it4NAME17ha3a756441bd0acebE
	.long	__ZN24test_msg_send_static_sel7generic5do_it4NAME17h4c3b369edbd3aadcE
	.long	__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17h639b2c9b06a336e7E
	.long	__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17hd38879191654969cE
	.long	__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17h558dcd606e4b144fE

.subsections_via_symbols
