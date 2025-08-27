	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn1_error_bool
	.p2align	2
_fn1_error_bool:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	str	xzr, [sp, #8]
	add	x3, sp, #8
	bl	_objc_msgSend
	mov	x8, x0
	mov	x0, #0
	tbz	w8, #0, LBB0_2
LBB0_1:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB0_2:
	ldr	x0, [sp, #8]
	bl	_objc_retain
	cbnz	x0, LBB0_1
	bl	SYM(objc2::__macros::msg_send::null_error::null_error::GENERATED_ID, 0)
	b	LBB0_1

	.globl	_fn2_error_new
	.p2align	2
_fn2_error_new:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	str	xzr, [sp, #8]
	add	x2, sp, #8
	bl	_objc_msgSend
	cbz	x0, LBB1_3
	mov	x1, x0
	mov	x0, #0
LBB1_2:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB1_3:
	ldr	x0, [sp, #8]
	bl	_objc_retain
	cbz	x0, LBB1_5
LBB1_4:
	mov	x1, x0
	mov	w0, #1
	b	LBB1_2
LBB1_5:
	bl	SYM(objc2::__macros::msg_send::null_error::null_error::GENERATED_ID, 0)
	b	LBB1_4

	.globl	_fn3_error_init
	.p2align	2
_fn3_error_init:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	str	xzr, [sp, #8]
	add	x2, sp, #8
	bl	_objc_msgSend
	cbz	x0, LBB2_3
	mov	x1, x0
	mov	x0, #0
LBB2_2:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB2_3:
	ldr	x0, [sp, #8]
	bl	_objc_retain
	cbz	x0, LBB2_5
LBB2_4:
	mov	x1, x0
	mov	w0, #1
	b	LBB2_2
LBB2_5:
	bl	SYM(objc2::__macros::msg_send::null_error::null_error::GENERATED_ID, 0)
	b	LBB2_4

	.globl	_fn4_error_copy
	.p2align	2
_fn4_error_copy:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	str	xzr, [sp, #8]
	add	x2, sp, #8
	bl	_objc_msgSend
	cbz	x0, LBB3_3
	mov	x1, x0
	mov	x0, #0
LBB3_2:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB3_3:
	ldr	x0, [sp, #8]
	bl	_objc_retain
	cbz	x0, LBB3_5
LBB3_4:
	mov	x1, x0
	mov	w0, #1
	b	LBB3_2
LBB3_5:
	bl	SYM(objc2::__macros::msg_send::null_error::null_error::GENERATED_ID, 0)
	b	LBB3_4

	.globl	_fn5_error_mutable_copy
	.p2align	2
_fn5_error_mutable_copy:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	str	xzr, [sp, #8]
	add	x2, sp, #8
	bl	_objc_msgSend
	cbz	x0, LBB4_3
	mov	x1, x0
	mov	x0, #0
LBB4_2:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB4_3:
	ldr	x0, [sp, #8]
	bl	_objc_retain
	cbz	x0, LBB4_5
LBB4_4:
	mov	x1, x0
	mov	w0, #1
	b	LBB4_2
LBB4_5:
	bl	SYM(objc2::__macros::msg_send::null_error::null_error::GENERATED_ID, 0)
	b	LBB4_4

	.globl	_fn6_error_autoreleased
	.p2align	2
_fn6_error_autoreleased:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	str	xzr, [sp, #8]
	add	x2, sp, #8
	bl	_objc_msgSend
	; InlineAsm Start
	mov	x29, x29
	; InlineAsm End
	bl	_objc_retainAutoreleasedReturnValue
	cbz	x0, LBB5_3
	mov	x1, x0
	mov	x0, #0
LBB5_2:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB5_3:
	ldr	x0, [sp, #8]
	bl	_objc_retain
	cbz	x0, LBB5_5
LBB5_4:
	mov	x1, x0
	mov	w0, #1
	b	LBB5_2
LBB5_5:
	bl	SYM(objc2::__macros::msg_send::null_error::null_error::GENERATED_ID, 0)
	b	LBB5_4

.subsections_via_symbols
