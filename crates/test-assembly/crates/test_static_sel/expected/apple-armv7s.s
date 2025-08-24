	.syntax	unified
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn1_get_sel
	.p2align	2
	.code	32
_fn1_get_sel:
	push	{r7, lr}
	mov	r7, sp
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb-(LPC0_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb-(LPC0_0+8))
LPC0_0:
	ldr	r0, [pc, r0]
	pop	{r7, pc}

	.globl	_fn2_get_same_sel
	.p2align	2
	.code	32
_fn2_get_same_sel:
	push	{r7, lr}
	mov	r7, sp
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f-(LPC1_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f-(LPC1_0+8))
LPC1_0:
	ldr	r0, [pc, r0]
	pop	{r7, pc}

	.globl	_fn3_get_common_twice
	.p2align	2
	.code	32
_fn3_get_common_twice:
	push	{r7, lr}
	mov	r7, sp
	movw	r0, :lower16:(LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr-(LPC2_0+8))
	movt	r0, :upper16:(LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr-(LPC2_0+8))
LPC2_0:
	ldr	r0, [pc, r0]
	ldr	r0, [r0]
	mov	r1, r0
	pop	{r7, pc}

	.globl	_fn4_get_different_sel
	.p2align	2
	.code	32
_fn4_get_different_sel:
	push	{r7, lr}
	mov	r7, sp
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85-(LPC3_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85-(LPC3_0+8))
LPC3_0:
	ldr	r0, [pc, r0]
	pop	{r7, pc}

	.globl	_fn5_unused_sel
	.p2align	2
	.code	32
_fn5_unused_sel:
	push	{r7, lr}
	mov	r7, sp
	pop	{r7, pc}

	.globl	_fn6_use_fns
	.p2align	2
	.code	32
_fn6_use_fns:
	push	{r7, lr}
	mov	r7, sp
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb-(LPC5_0+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb-(LPC5_0+8))
LPC5_0:
	ldr	r1, [pc, r1]
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f-(LPC5_1+8))
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f-(LPC5_1+8))
LPC5_1:
	ldr	r2, [pc, r2]
	movw	r3, :lower16:(L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85-(LPC5_2+8))
	movt	r3, :upper16:(L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85-(LPC5_2+8))
LPC5_2:
	ldr	r3, [pc, r3]
	movw	r9, :lower16:(L_OBJC_SELECTOR_REFERENCES_e91a5376ddae3cfc-(LPC5_3+8))
	movt	r9, :upper16:(L_OBJC_SELECTOR_REFERENCES_e91a5376ddae3cfc-(LPC5_3+8))
LPC5_3:
	ldr	r9, [pc, r9]
	stm	r0, {r1, r2, r3, r9}
	pop	{r7, pc}

	.globl	_fn7_use_same_twice
	.p2align	2
	.code	32
_fn7_use_same_twice:
	push	{r7, lr}
	mov	r7, sp
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb-(LPC6_0+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb-(LPC6_0+8))
LPC6_0:
	ldr	r1, [pc, r1]
	str	r1, [r0]
	str	r1, [r0, #4]
	pop	{r7, pc}

	.globl	_fn8_use_in_loop
	.p2align	2
	.code	32
_fn8_use_in_loop:
	push	{r7, lr}
	mov	r7, sp
	pop	{r7, pc}

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2668dedcc69bf8fb
L_OBJC_METH_VAR_NAME_2668dedcc69bf8fb:
	.asciz	"simple"

	.globl	L_OBJC_METH_VAR_NAME_29d0234b9445d447
L_OBJC_METH_VAR_NAME_29d0234b9445d447:
	.asciz	"unused"

	.globl	L_OBJC_METH_VAR_NAME_9303807037ba4f9f
L_OBJC_METH_VAR_NAME_9303807037ba4f9f:
	.asciz	"simple"

	.globl	L_OBJC_METH_VAR_NAME_a23daf114eba1518
L_OBJC_METH_VAR_NAME_a23daf114eba1518:
	.asciz	"loopedSelector"

	.globl	L_OBJC_METH_VAR_NAME_e91a5376ddae3cfc
L_OBJC_METH_VAR_NAME_e91a5376ddae3cfc:
	.asciz	"fourthSel"

	.globl	L_OBJC_METH_VAR_NAME_ff60dae1998e0c85
L_OBJC_METH_VAR_NAME_ff60dae1998e0c85:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb:
	.long	L_OBJC_METH_VAR_NAME_2668dedcc69bf8fb

	.globl	L_OBJC_SELECTOR_REFERENCES_29d0234b9445d447
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_29d0234b9445d447:
	.long	L_OBJC_METH_VAR_NAME_29d0234b9445d447

	.globl	L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f:
	.long	L_OBJC_METH_VAR_NAME_9303807037ba4f9f

	.globl	L_OBJC_SELECTOR_REFERENCES_a23daf114eba1518
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_a23daf114eba1518:
	.long	L_OBJC_METH_VAR_NAME_a23daf114eba1518

	.globl	L_OBJC_SELECTOR_REFERENCES_e91a5376ddae3cfc
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_e91a5376ddae3cfc:
	.long	L_OBJC_METH_VAR_NAME_e91a5376ddae3cfc

	.globl	L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85:
	.long	L_OBJC_METH_VAR_NAME_ff60dae1998e0c85

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2668dedcc69bf8fb
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2668dedcc69bf8fb:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_29d0234b9445d447
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_29d0234b9445d447:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_9303807037ba4f9f
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_9303807037ba4f9f:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_a23daf114eba1518
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a23daf114eba1518:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_e91a5376ddae3cfc
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_e91a5376ddae3cfc:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_ff60dae1998e0c85
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ff60dae1998e0c85:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2, 0x0
LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_alloc
	.long	0

.subsections_via_symbols
