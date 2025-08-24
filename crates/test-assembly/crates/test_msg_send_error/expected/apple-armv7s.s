	.syntax	unified
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn1_error_bool
	.p2align	2
	.code	32
_fn1_error_bool:
	push	{r4, r7, lr}
	add	r7, sp, #4
	sub	sp, sp, #4
	mov	r4, #0
	str	r4, [sp]
	mov	r3, sp
	bl	_objc_msgSend
	cmp	r0, #0
	beq	LBB0_2
LBB0_1:
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}
LBB0_2:
	ldr	r0, [sp]
	bl	_objc_retain
	mov	r4, r0
	cmp	r0, #0
	bne	LBB0_1
	bl	SYM(objc2::__macro_helpers::null_error::null_error::GENERATED_ID, 0)
	mov	r4, r0
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}

	.globl	_fn2_error_new
	.p2align	2
	.code	32
_fn2_error_new:
	push	{r4, r7, lr}
	add	r7, sp, #4
	sub	sp, sp, #4
	mov	r4, #0
	str	r4, [sp]
	mov	r2, sp
	bl	_objc_msgSend
	cmp	r0, #0
	beq	LBB1_2
LBB1_1:
	mov	r1, r0
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}
LBB1_2:
	ldr	r0, [sp]
	bl	_objc_retain
	mov	r4, #1
	cmp	r0, #0
	bne	LBB1_1
	bl	SYM(objc2::__macro_helpers::null_error::null_error::GENERATED_ID, 0)
	mov	r1, r0
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}

	.globl	_fn3_error_init
	.p2align	2
	.code	32
_fn3_error_init:
	push	{r4, r7, lr}
	add	r7, sp, #4
	sub	sp, sp, #4
	mov	r4, #0
	str	r4, [sp]
	mov	r2, sp
	bl	_objc_msgSend
	cmp	r0, #0
	beq	LBB2_2
LBB2_1:
	mov	r1, r0
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}
LBB2_2:
	ldr	r0, [sp]
	bl	_objc_retain
	mov	r4, #1
	cmp	r0, #0
	bne	LBB2_1
	bl	SYM(objc2::__macro_helpers::null_error::null_error::GENERATED_ID, 0)
	mov	r1, r0
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}

	.globl	_fn4_error_copy
	.p2align	2
	.code	32
_fn4_error_copy:
	push	{r4, r7, lr}
	add	r7, sp, #4
	sub	sp, sp, #4
	mov	r4, #0
	str	r4, [sp]
	mov	r2, sp
	bl	_objc_msgSend
	cmp	r0, #0
	beq	LBB3_2
LBB3_1:
	mov	r1, r0
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}
LBB3_2:
	ldr	r0, [sp]
	bl	_objc_retain
	mov	r4, #1
	cmp	r0, #0
	bne	LBB3_1
	bl	SYM(objc2::__macro_helpers::null_error::null_error::GENERATED_ID, 0)
	mov	r1, r0
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}

	.globl	_fn5_error_mutable_copy
	.p2align	2
	.code	32
_fn5_error_mutable_copy:
	push	{r4, r7, lr}
	add	r7, sp, #4
	sub	sp, sp, #4
	mov	r4, #0
	str	r4, [sp]
	mov	r2, sp
	bl	_objc_msgSend
	cmp	r0, #0
	beq	LBB4_2
LBB4_1:
	mov	r1, r0
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}
LBB4_2:
	ldr	r0, [sp]
	bl	_objc_retain
	mov	r4, #1
	cmp	r0, #0
	bne	LBB4_1
	bl	SYM(objc2::__macro_helpers::null_error::null_error::GENERATED_ID, 0)
	mov	r1, r0
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}

	.globl	_fn6_error_autoreleased
	.p2align	2
	.code	32
_fn6_error_autoreleased:
	push	{r4, r7, lr}
	add	r7, sp, #4
	sub	sp, sp, #4
	mov	r4, #0
	str	r4, [sp]
	mov	r2, sp
	bl	_objc_msgSend
	@ InlineAsm Start
	mov	r7, r7
	@ InlineAsm End
	bl	_objc_retainAutoreleasedReturnValue
	cmp	r0, #0
	beq	LBB5_2
LBB5_1:
	mov	r1, r0
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}
LBB5_2:
	ldr	r0, [sp]
	bl	_objc_retain
	mov	r4, #1
	cmp	r0, #0
	bne	LBB5_1
	bl	SYM(objc2::__macro_helpers::null_error::null_error::GENERATED_ID, 0)
	mov	r1, r0
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}

.subsections_via_symbols
