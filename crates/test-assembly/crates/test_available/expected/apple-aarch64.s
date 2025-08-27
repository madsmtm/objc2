	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn1_always
	.p2align	2
_fn1_always:
	mov	w0, #1
	ret

	.globl	_fn2_never
	.p2align	2
_fn2_never:
	mov	w0, #0
	ret

	.globl	_fn3_low
	.p2align	2
_fn3_low:
	mov	w0, #1
	ret

	.globl	_fn4_high
	.p2align	2
_fn4_high:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh0:
	adrp	x19, SYM(objc2::__macros::available::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)@GOTPAGE
Lloh1:
	ldr	x19, [x19, SYM(objc2::__macros::available::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)@GOTPAGEOFF]
	ldr	w0, [x19]
	cbz	w0, LBB3_2
LBB3_1:
	lsr	w8, w0, #16
	cmp	w8, #14
	cset	w0, hi
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB3_2:
	bl	SYM(objc2::__macros::available::apple::lookup_version::GENERATED_ID, 0)
	str	w0, [x19]
	b	LBB3_1
	.loh AdrpLdrGot	Lloh0, Lloh1

	.globl	_fn5_only_ios
	.p2align	2
_fn5_only_ios:
	mov	w0, #0
	ret

	.globl	_fn6_two_checks
	.p2align	2
_fn6_two_checks:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh2:
	adrp	x20, SYM(objc2::__macros::available::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)@GOTPAGE
Lloh3:
	ldr	x20, [x20, SYM(objc2::__macros::available::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)@GOTPAGEOFF]
	ldr	w19, [x20]
	cbz	w19, LBB5_3
	ldr	w0, [x20]
	cbz	w0, LBB5_4
LBB5_2:
	lsr	w8, w19, #17
	cmp	w8, #6
	mov	w8, #917503
	add	w8, w8, #16, lsl #12
	ccmp	w0, w8, #0, hi
	cset	w0, hi
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB5_3:
	bl	SYM(objc2::__macros::available::apple::lookup_version::GENERATED_ID, 0)
	mov	x19, x0
	str	w0, [x20]
	ldr	w0, [x20]
	cbnz	w0, LBB5_2
LBB5_4:
	bl	SYM(objc2::__macros::available::apple::lookup_version::GENERATED_ID, 0)
	str	w0, [x20]
	b	LBB5_2
	.loh AdrpLdrGot	Lloh2, Lloh3

.subsections_via_symbols
