	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle_with_sel
	.p2align	2
	.code	32
_handle_with_sel:
	movw	r1, :lower16:(__ZN24test_msg_send_static_sel15handle_with_sel5do_it5VALUE17hc7c6bad48a0601f0E-(LPC0_0+8))
	movt	r1, :upper16:(__ZN24test_msg_send_static_sel15handle_with_sel5do_it5VALUE17hc7c6bad48a0601f0E-(LPC0_0+8))
LPC0_0:
	add	r1, pc, r1
	b	_objc_msgSend

	.globl	_handle_alloc_init
	.p2align	2
	.code	32
_handle_alloc_init:
	push	{r7, lr}
	mov	r7, sp
	movw	r1, :lower16:(__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h580fd8fa27b5ffcbE-(LPC1_0+8))
	movt	r1, :upper16:(__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h580fd8fa27b5ffcbE-(LPC1_0+8))
LPC1_0:
	add	r1, pc, r1
	bl	_objc_msgSend
	movw	r1, :lower16:(__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h0cbd3103e48b1841E-(LPC1_1+8))
	movt	r1, :upper16:(__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h0cbd3103e48b1841E-(LPC1_1+8))
LPC1_1:
	add	r1, pc, r1
	pop	{r7, lr}
	b	_objc_msgSend

	.globl	_use_generic
	.p2align	2
	.code	32
_use_generic:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	movw	r1, :lower16:(__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17hea370ab904c591dcE-(LPC2_0+8))
	mov	r4, r0
	movt	r1, :upper16:(__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17hea370ab904c591dcE-(LPC2_0+8))
	movw	r5, :lower16:(__ZN24test_msg_send_static_sel7generic5do_it5VALUE17hdb7b18c98f78677aE-(LPC2_1+8))
	movt	r5, :upper16:(__ZN24test_msg_send_static_sel7generic5do_it5VALUE17hdb7b18c98f78677aE-(LPC2_1+8))
LPC2_0:
	add	r1, pc, r1
LPC2_1:
	add	r5, pc, r5
	mov	r2, r5
	bl	_objc_msgSend
	movw	r1, :lower16:(__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17hf0637c53b24a70f1E-(LPC2_2+8))
	mov	r0, r4
	movt	r1, :upper16:(__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17hf0637c53b24a70f1E-(LPC2_2+8))
	mov	r2, r5
LPC2_2:
	add	r1, pc, r1
	bl	_objc_msgSend
	movw	r1, :lower16:(__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17hb63aece1e25549bdE-(LPC2_3+8))
	mov	r0, r4
	movt	r1, :upper16:(__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17hb63aece1e25549bdE-(LPC2_3+8))
	mov	r2, r5
LPC2_3:
	add	r1, pc, r1
	pop	{r4, r5, r7, lr}
	b	_objc_msgSend

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel15handle_with_sel5do_it5VALUE17hc7c6bad48a0601f0E:
	.asciz	"someSelector"

__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h0cbd3103e48b1841E:
	.asciz	"init"

__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h580fd8fa27b5ffcbE:
	.asciz	"alloc"

__ZN24test_msg_send_static_sel7generic5do_it5VALUE17hdb7b18c98f78677aE:
	.asciz	"generic:selector:"

__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17hea370ab904c591dcE:
	.asciz	"performSelector:"

__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17hf0637c53b24a70f1E:
	.asciz	"performSelector:"

__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17hb63aece1e25549bdE:
	.asciz	"performSelector:"

.subsections_via_symbols
