	.section	__TEXT,__text,regular,pure_instructions
	.globl	_nonnull_nonnull
	.p2align	2
_nonnull_nonnull:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x2
	ldr	x20, [x2]
	bl	_objc_msgSend
	mov	x21, x0
	ldr	x0, [x19]
	bl	_objc_retain
	mov	x0, x20
	bl	_objc_release
	mov	x0, x21
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret

	.globl	_null_nonnull
	.p2align	2
_null_nonnull:
	cbz	x2, LBB1_2
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	ldr	x19, [x2]
	mov	x20, x2
	bl	_objc_msgSend
	mov	x21, x0
	ldr	x0, [x20]
	bl	_objc_retain
	mov	x0, x19
	bl	_objc_release
	mov	x0, x21
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret
LBB1_2:
	b	_objc_msgSend

	.globl	_nonnull_null
	.p2align	2
_nonnull_null:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x21, x2
	ldr	x19, [x2]
	bl	_objc_msgSend
	mov	x20, x0
	ldr	x0, [x21]
	bl	_objc_retain
	cbz	x19, LBB2_2
	mov	x0, x19
	bl	_objc_release
LBB2_2:
	mov	x0, x20
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret

	.globl	_null_null
	.p2align	2
_null_null:
	cbz	x2, LBB3_4
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	ldr	x19, [x2]
	mov	x21, x2
	bl	_objc_msgSend
	mov	x20, x0
	ldr	x0, [x21]
	bl	_objc_retain
	cbz	x19, LBB3_3
	mov	x0, x19
	bl	_objc_release
LBB3_3:
	mov	x0, x20
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret
LBB3_4:
	b	_objc_msgSend

	.globl	_two_nonnull_nonnull
	.p2align	2
_two_nonnull_nonnull:
	stp	x24, x23, [sp, #-64]!
	stp	x22, x21, [sp, #16]
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	mov	x19, x3
	mov	x20, x2
	ldr	x21, [x2]
	ldr	x22, [x3]
	bl	_objc_msgSend
	mov	x23, x0
	ldr	x0, [x20]
	bl	_objc_retain
	mov	x0, x21
	bl	_objc_release
	ldr	x0, [x19]
	bl	_objc_retain
	mov	x0, x22
	bl	_objc_release
	mov	x0, x23
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	ldp	x22, x21, [sp, #16]
	ldp	x24, x23, [sp], #64
	ret

	.globl	_call_with_none1
	.p2align	2
_call_with_none1:
	mov	x2, #0
	b	_objc_msgSend

	.globl	_call_with_none2
	.p2align	2
_call_with_none2:
	mov	x2, #0
	b	_objc_msgSend

	.globl	_call_with_none3
	.p2align	2
_call_with_none3:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	str	xzr, [sp, #8]
	add	x2, sp, #8
	bl	_objc_msgSend
	mov	x19, x0
	ldr	x0, [sp, #8]
	bl	_objc_retain
	ldr	x1, [sp, #8]
	mov	x0, x19
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret

	.globl	_call_with_none4
	.p2align	2
_call_with_none4:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	str	xzr, [sp, #8]
	add	x2, sp, #8
	bl	_objc_msgSend
	mov	x19, x0
	ldr	x0, [sp, #8]
	bl	_objc_retain
	ldr	x1, [sp, #8]
	mov	x0, x19
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret

	.globl	_call_with_some1
	.p2align	2
_call_with_some1:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x2
	str	x2, [sp, #8]
	add	x2, sp, #8
	bl	_objc_msgSend
	mov	x20, x0
	ldr	x0, [sp, #8]
	bl	_objc_retain
	mov	x0, x19
	bl	_objc_release
	ldr	x1, [sp, #8]
	mov	x0, x20
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret

	.globl	_call_with_some2
	.p2align	2
_call_with_some2:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x2
	str	x2, [sp, #8]
	add	x2, sp, #8
	bl	_objc_msgSend
	mov	x20, x0
	ldr	x0, [sp, #8]
	bl	_objc_retain
	mov	x0, x19
	bl	_objc_release
	ldr	x1, [sp, #8]
	mov	x0, x20
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret

	.globl	_call_with_some3
	.p2align	2
_call_with_some3:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x2
	str	x2, [sp, #8]
	add	x2, sp, #8
	bl	_objc_msgSend
	mov	x20, x0
	ldr	x0, [sp, #8]
	bl	_objc_retain
	mov	x0, x19
	bl	_objc_release
	ldr	x1, [sp, #8]
	mov	x0, x20
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret

.subsections_via_symbols
