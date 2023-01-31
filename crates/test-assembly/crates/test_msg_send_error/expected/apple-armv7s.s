	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.p2align	2
	.code	32
SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::Object>, 0):
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r1
	bl	_objc_retain
	cmp	r0, #0
	popne	{r4, r7, pc}
LBB0_1:
	movw	r0, :lower16:(l_anon.[ID].0-(LPC0_0+8))
	mov	r1, #56
	movt	r0, :upper16:(l_anon.[ID].0-(LPC0_0+8))
	mov	r2, r4
LPC0_0:
	add	r0, pc, r0
	mov	lr, pc
	b	SYM(core::option::expect_failed::GENERATED_ID, 0)

	.p2align	2
	.code	32
SYM(objc2[CRATE_ID]::message::encountered_error::<objc2[CRATE_ID]::runtime::Object>, 0):
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_retain
	cmp	r0, #0
	popne	{r7, pc}
LBB1_1:
	movw	r0, :lower16:(l_anon.[ID].1-(LPC1_0+8))
	mov	r1, #54
	movt	r0, :upper16:(l_anon.[ID].1-(LPC1_0+8))
	movw	r2, :lower16:(l_anon.[ID].3-(LPC1_1+8))
	movt	r2, :upper16:(l_anon.[ID].3-(LPC1_1+8))
LPC1_0:
	add	r0, pc, r0
LPC1_1:
	add	r2, pc, r2
	mov	lr, pc
	b	SYM(core::option::expect_failed::GENERATED_ID, 0)

	.globl	_error_bool
	.p2align	2
	.code	32
_error_bool:
	push	{r4, r7, lr}
	add	r7, sp, #4
	sub	sp, sp, #4
	mov	r4, #0
	mov	r3, sp
	str	r4, [sp]
	bl	_objc_msgSend
	cmp	r0, #0
	beq	LBB2_2
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}
LBB2_2:
	ldr	r0, [sp]
	bl	SYM(objc2[CRATE_ID]::message::encountered_error::<objc2[CRATE_ID]::runtime::Object>, 0)
	mov	r4, r0
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}

	.globl	_error_new
	.p2align	2
	.code	32
_error_new:
	push	{r4, r7, lr}
	add	r7, sp, #4
	sub	sp, sp, #4
	mov	r4, #0
	mov	r2, sp
	str	r4, [sp]
	bl	_objc_msgSend
	mov	r1, r0
	cmp	r0, #0
	beq	LBB3_2
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}
LBB3_2:
	movw	r1, :lower16:(l_anon.[ID].4-(LPC3_0+8))
	movt	r1, :upper16:(l_anon.[ID].4-(LPC3_0+8))
	ldr	r0, [sp]
LPC3_0:
	add	r1, pc, r1
	bl	SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::Object>, 0)
	mov	r1, r0
	mov	r4, #1
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}

	.globl	_error_alloc
	.p2align	2
	.code	32
_error_alloc:
	push	{r4, r7, lr}
	add	r7, sp, #4
	sub	sp, sp, #4
	mov	r4, #0
	mov	r2, sp
	str	r4, [sp]
	bl	_objc_msgSend
	mov	r1, r0
	cmp	r0, #0
	beq	LBB4_2
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}
LBB4_2:
	movw	r1, :lower16:(l_anon.[ID].5-(LPC4_0+8))
	movt	r1, :upper16:(l_anon.[ID].5-(LPC4_0+8))
	ldr	r0, [sp]
LPC4_0:
	add	r1, pc, r1
	bl	SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::Object>, 0)
	mov	r1, r0
	mov	r4, #1
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}

	.globl	_error_init
	.p2align	2
	.code	32
_error_init:
	push	{r4, r7, lr}
	add	r7, sp, #4
	sub	sp, sp, #4
	mov	r4, #0
	mov	r2, sp
	str	r4, [sp]
	bl	_objc_msgSend
	mov	r1, r0
	cmp	r0, #0
	beq	LBB5_2
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}
LBB5_2:
	movw	r1, :lower16:(l_anon.[ID].6-(LPC5_0+8))
	movt	r1, :upper16:(l_anon.[ID].6-(LPC5_0+8))
	ldr	r0, [sp]
LPC5_0:
	add	r1, pc, r1
	bl	SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::Object>, 0)
	mov	r1, r0
	mov	r4, #1
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}

	.globl	_error_copy
	.p2align	2
	.code	32
_error_copy:
	push	{r4, r7, lr}
	add	r7, sp, #4
	sub	sp, sp, #4
	mov	r4, #0
	mov	r2, sp
	str	r4, [sp]
	bl	_objc_msgSend
	mov	r1, r0
	cmp	r0, #0
	beq	LBB6_2
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}
LBB6_2:
	movw	r1, :lower16:(l_anon.[ID].7-(LPC6_0+8))
	movt	r1, :upper16:(l_anon.[ID].7-(LPC6_0+8))
	ldr	r0, [sp]
LPC6_0:
	add	r1, pc, r1
	bl	SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::Object>, 0)
	mov	r1, r0
	mov	r4, #1
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}

	.globl	_error_autoreleased
	.p2align	2
	.code	32
_error_autoreleased:
	push	{r4, r7, lr}
	add	r7, sp, #4
	sub	sp, sp, #4
	mov	r4, #0
	mov	r2, sp
	str	r4, [sp]
	bl	_objc_msgSend
	@ InlineAsm Start
	mov	r7, r7
	@ InlineAsm End
	bl	_objc_retainAutoreleasedReturnValue
	mov	r1, r0
	cmp	r0, #0
	beq	LBB7_2
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}
LBB7_2:
	movw	r1, :lower16:(l_anon.[ID].8-(LPC7_0+8))
	movt	r1, :upper16:(l_anon.[ID].8-(LPC7_0+8))
	ldr	r0, [sp]
LPC7_0:
	add	r1, pc, r1
	bl	SYM(objc2[CRATE_ID]::__macro_helpers::encountered_error::<objc2[CRATE_ID]::runtime::Object>, 0)
	mov	r1, r0
	mov	r4, #1
	mov	r0, r4
	sub	sp, r7, #4
	pop	{r4, r7, pc}

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"error parameter should be set if the method returns NULL"

l_anon.[ID].1:
	.ascii	"error parameter should be set if the method returns NO"

l_anon.[ID].2:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	2
l_anon.[ID].3:
	.long	l_anon.[ID].2
	.asciz	"6\000\000\000\013\000\000\000\005\000\000"

	.p2align	2
l_anon.[ID].4:
	.long	l_anon.[ID].2
	.asciz	"6\000\000\000\020\000\000\000\005\000\000"

	.p2align	2
l_anon.[ID].5:
	.long	l_anon.[ID].2
	.asciz	"6\000\000\000\025\000\000\000\005\000\000"

	.p2align	2
l_anon.[ID].6:
	.long	l_anon.[ID].2
	.asciz	"6\000\000\000\032\000\000\000\005\000\000"

	.p2align	2
l_anon.[ID].7:
	.long	l_anon.[ID].2
	.asciz	"6\000\000\000\037\000\000\000\005\000\000"

	.p2align	2
l_anon.[ID].8:
	.long	l_anon.[ID].2
	.asciz	"6\000\000\000$\000\000\000\005\000\000"

.subsections_via_symbols
