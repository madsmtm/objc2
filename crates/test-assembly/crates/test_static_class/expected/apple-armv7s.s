	.syntax	unified
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn1_get_class
	.p2align	2
	.code	32
_fn1_get_class:
	push	{r7, lr}
	mov	r7, sp
	movw	r0, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34-(LPC0_0+8))
	movt	r0, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34-(LPC0_0+8))
LPC0_0:
	ldr	r0, [pc, r0]
	pop	{r7, pc}

	.globl	_fn1_get_same_class
	.p2align	2
	.code	32
_fn1_get_same_class:
	push	{r7, lr}
	mov	r7, sp
	movw	r0, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a-(LPC1_0+8))
	movt	r0, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a-(LPC1_0+8))
LPC1_0:
	ldr	r0, [pc, r0]
	pop	{r7, pc}

	.globl	_fn3_get_different_class
	.p2align	2
	.code	32
_fn3_get_different_class:
	push	{r7, lr}
	mov	r7, sp
	movw	r0, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1-(LPC2_0+8))
	movt	r0, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1-(LPC2_0+8))
LPC2_0:
	ldr	r0, [pc, r0]
	pop	{r7, pc}

	.globl	_fn4_unused_class
	.p2align	2
	.code	32
_fn4_unused_class:
	push	{r7, lr}
	mov	r7, sp
	pop	{r7, pc}

	.globl	_fn5_use_fns
	.p2align	2
	.code	32
_fn5_use_fns:
	push	{r7, lr}
	mov	r7, sp
	movw	r1, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34-(LPC4_0+8))
	movt	r1, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34-(LPC4_0+8))
LPC4_0:
	ldr	r1, [pc, r1]
	movw	r2, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a-(LPC4_1+8))
	movt	r2, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a-(LPC4_1+8))
LPC4_1:
	ldr	r2, [pc, r2]
	movw	r3, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1-(LPC4_2+8))
	movt	r3, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1-(LPC4_2+8))
LPC4_2:
	ldr	r3, [pc, r3]
	movw	r9, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_a92f01d3b55d29c5-(LPC4_3+8))
	movt	r9, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_a92f01d3b55d29c5-(LPC4_3+8))
LPC4_3:
	ldr	r9, [pc, r9]
	stm	r0, {r1, r2, r3, r9}
	pop	{r7, pc}

	.globl	_fn6_use_same_twice
	.p2align	2
	.code	32
_fn6_use_same_twice:
	push	{r7, lr}
	mov	r7, sp
	movw	r1, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34-(LPC5_0+8))
	movt	r1, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34-(LPC5_0+8))
LPC5_0:
	ldr	r1, [pc, r1]
	str	r1, [r0]
	str	r1, [r0, #4]
	pop	{r7, pc}

	.globl	_fn7_use_in_loop
	.p2align	2
	.code	32
_fn7_use_in_loop:
	push	{r7, lr}
	mov	r7, sp
	pop	{r7, pc}

	.section	__DATA,__objc_classrefs
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34:
	.long	_OBJC_CLASS_$_NSObject

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1:
	.long	_OBJC_CLASS_$_NSString

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_9f503c7582f87b48
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_9f503c7582f87b48:
	.long	_OBJC_CLASS_$_NSData

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_a92f01d3b55d29c5
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_a92f01d3b55d29c5:
	.long	_OBJC_CLASS_$_NSException

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_d4ef9efb3ee49ab7
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_d4ef9efb3ee49ab7:
	.long	_OBJC_CLASS_$_NSLock

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a:
	.long	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_1f36dafa1e0a7b34
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_1f36dafa1e0a7b34:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_5a6ce274a9f949e1
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_5a6ce274a9f949e1:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_9f503c7582f87b48
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_9f503c7582f87b48:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_a92f01d3b55d29c5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a92f01d3b55d29c5:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_d4ef9efb3ee49ab7
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_d4ef9efb3ee49ab7:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_e1a6d3426ab3be5a
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_e1a6d3426ab3be5a:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
