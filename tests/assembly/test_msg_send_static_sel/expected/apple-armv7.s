	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle_with_sel
	.p2align	2
	.code	32
_handle_with_sel:
	movw	r1, :lower16:(__ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17ha3faf09d3de71191E-(LPC0_0+8))
	movt	r1, :upper16:(__ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17ha3faf09d3de71191E-(LPC0_0+8))
LPC0_0:
	ldr	r1, [pc, r1]
	b	_objc_msgSend

	.globl	_handle_alloc_init
	.p2align	2
	.code	32
_handle_alloc_init:
	push	{r4, r7, lr}
	add	r7, sp, #4
	movw	r1, :lower16:(__MergedGlobals-(LPC1_0+8))
	movt	r1, :upper16:(__MergedGlobals-(LPC1_0+8))
	movw	r4, :lower16:(__MergedGlobals-(LPC1_1+8))
LPC1_0:
	add	r1, pc, r1
	movt	r4, :upper16:(__MergedGlobals-(LPC1_1+8))
LPC1_1:
	ldr	r4, [pc, r4]
	ldr	r1, [r1, #4]
	bl	_objc_msgSend
	mov	r1, r4
	pop	{r4, r7, lr}
	b	_objc_msgSend

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
	pop	{r4, r5, r7, lr}
	b	_objc_msgSend

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel15handle_with_sel5do_it4NAME17h4be0477fe73c107cE:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17ha3faf09d3de71191E:
	.long	__ZN24test_msg_send_static_sel15handle_with_sel5do_it4NAME17h4be0477fe73c107cE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init5do_it4NAME17hfb54835af2ec43c7E:
	.asciz	"init"

__ZN24test_msg_send_static_sel17handle_alloc_init5do_it4NAME17h8f551a75058466b2E:
	.asciz	"alloc"

__ZN24test_msg_send_static_sel7generic5do_it4NAME17h3eb023dd38e4a0fcE:
	.asciz	"generic:selector:"

__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17h68ef93208bbbc446E:
	.asciz	"performSelector:"

__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17h52746d9e41762e25E:
	.asciz	"performSelector:"

__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17hc6eea12cf00dc3ecE:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	2
__MergedGlobals:
	.long	__ZN24test_msg_send_static_sel17handle_alloc_init5do_it4NAME17hfb54835af2ec43c7E
	.long	__ZN24test_msg_send_static_sel17handle_alloc_init5do_it4NAME17h8f551a75058466b2E
	.long	__ZN24test_msg_send_static_sel7generic5do_it4NAME17h3eb023dd38e4a0fcE
	.long	__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17h68ef93208bbbc446E
	.long	__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17h52746d9e41762e25E
	.long	__ZN24test_msg_send_static_sel11use_generic5do_it4NAME17hc6eea12cf00dc3ecE

.subsections_via_symbols
