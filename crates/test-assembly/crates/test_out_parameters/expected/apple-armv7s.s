	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_nonnull_nonnull
	.p2align	2
	.code	32
_nonnull_nonnull:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	mov	r4, r2
	ldr	r5, [r2]
	bl	_objc_msgSend
	mov	r6, r0
	ldr	r0, [r4]
	bl	_objc_retain
	mov	r0, r5
	bl	_objc_release
	mov	r0, r6
	pop	{r4, r5, r6, r7, pc}

	.globl	_null_nonnull
	.p2align	2
	.code	32
_null_nonnull:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	cmp	r2, #0
	beq	LBB1_2
	mov	r5, r2
	ldr	r6, [r2]
	bl	_objc_msgSend
	mov	r4, r0
	ldr	r0, [r5]
	bl	_objc_retain
	mov	r0, r6
	bl	_objc_release
	mov	r0, r4
	pop	{r4, r5, r6, r7, pc}
LBB1_2:
	mov	r2, #0
	bl	_objc_msgSend
	pop	{r4, r5, r6, r7, pc}

	.globl	_nonnull_null
	.p2align	2
	.code	32
_nonnull_null:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	mov	r6, r2
	ldr	r4, [r2]
	bl	_objc_msgSend
	mov	r5, r0
	ldr	r0, [r6]
	bl	_objc_retain
	cmp	r4, #0
	beq	LBB2_2
	mov	r0, r4
	bl	_objc_release
LBB2_2:
	mov	r0, r5
	pop	{r4, r5, r6, r7, pc}

	.globl	_null_null
	.p2align	2
	.code	32
_null_null:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	cmp	r2, #0
	beq	LBB3_4
	mov	r6, r2
	ldr	r5, [r2]
	bl	_objc_msgSend
	mov	r4, r0
	ldr	r0, [r6]
	bl	_objc_retain
	cmp	r5, #0
	beq	LBB3_3
	mov	r0, r5
	bl	_objc_release
LBB3_3:
	mov	r0, r4
	pop	{r4, r5, r6, r7, pc}
LBB3_4:
	mov	r2, #0
	bl	_objc_msgSend
	pop	{r4, r5, r6, r7, pc}

	.globl	_two_nonnull_nonnull
	.p2align	2
	.code	32
_two_nonnull_nonnull:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10}
	mov	r4, r3
	mov	r5, r2
	ldr	r8, [r3]
	ldr	r6, [r2]
	bl	_objc_msgSend
	mov	r10, r0
	ldr	r0, [r5]
	bl	_objc_retain
	mov	r0, r6
	bl	_objc_release
	ldr	r0, [r4]
	bl	_objc_retain
	mov	r0, r8
	bl	_objc_release
	mov	r0, r10
	pop	{r8, r10}
	pop	{r4, r5, r6, r7, pc}

	.globl	_call_with_none1
	.p2align	2
	.code	32
_call_with_none1:
	push	{r7, lr}
	mov	r7, sp
	mov	r2, #0
	bl	_objc_msgSend
	pop	{r7, pc}

	.globl	_call_with_none2
	.p2align	2
	.code	32
_call_with_none2:
	push	{r7, lr}
	mov	r7, sp
	mov	r2, #0
	bl	_objc_msgSend
	pop	{r7, pc}

	.globl	_call_with_none3
	.p2align	2
	.code	32
_call_with_none3:
	push	{r4, r7, lr}
	add	r7, sp, #4
	sub	sp, sp, #4
	mov	r2, #0
	str	r2, [sp]
	mov	r2, sp
	bl	_objc_msgSend
	mov	r4, r0
	ldr	r0, [sp]
	bl	_objc_retain
	ldr	r1, [sp]
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}

	.globl	_call_with_none4
	.p2align	2
	.code	32
_call_with_none4:
	push	{r4, r7, lr}
	add	r7, sp, #4
	sub	sp, sp, #4
	mov	r2, #0
	str	r2, [sp]
	mov	r2, sp
	bl	_objc_msgSend
	mov	r4, r0
	ldr	r0, [sp]
	bl	_objc_retain
	ldr	r1, [sp]
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}

	.globl	_call_with_some1
	.p2align	2
	.code	32
_call_with_some1:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	sub	sp, sp, #4
	mov	r4, r2
	str	r2, [sp]
	mov	r2, sp
	bl	_objc_msgSend
	mov	r5, r0
	ldr	r0, [sp]
	bl	_objc_retain
	mov	r0, r4
	bl	_objc_release
	ldr	r1, [sp]
	mov	r0, r5
	sub	sp, r7, #8
	pop	{r4, r5, r7, pc}

	.globl	_call_with_some2
	.p2align	2
	.code	32
_call_with_some2:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	sub	sp, sp, #4
	mov	r4, r2
	str	r2, [sp]
	mov	r2, sp
	bl	_objc_msgSend
	mov	r5, r0
	ldr	r0, [sp]
	bl	_objc_retain
	mov	r0, r4
	bl	_objc_release
	ldr	r1, [sp]
	mov	r0, r5
	sub	sp, r7, #8
	pop	{r4, r5, r7, pc}

	.globl	_call_with_some3
	.p2align	2
	.code	32
_call_with_some3:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	sub	sp, sp, #4
	mov	r4, r2
	str	r2, [sp]
	mov	r2, sp
	bl	_objc_msgSend
	mov	r5, r0
	ldr	r0, [sp]
	bl	_objc_retain
	mov	r0, r4
	bl	_objc_release
	ldr	r1, [sp]
	mov	r0, r5
	sub	sp, r7, #8
	pop	{r4, r5, r7, pc}

.subsections_via_symbols
