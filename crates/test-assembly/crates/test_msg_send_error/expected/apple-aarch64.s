	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_retained::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0):
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x1
	bl	_objc_retain
	cbz	x0, LBB0_2
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB0_2:
Lloh0:
	adrp	x0, l_anon.[ID].0@PAGE
Lloh1:
	add	x0, x0, l_anon.[ID].0@PAGEOFF
	mov	w1, #56
	mov	x2, x19
	bl	SYM(core::option::expect_failed::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh0, Lloh1

	.p2align	2
SYM(objc2[CRATE_ID]::__macro_helpers::msg_send::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0):
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_objc_retain
	cbz	x0, LBB1_2
	ldp	x29, x30, [sp], #16
	ret
LBB1_2:
Lloh2:
	adrp	x0, l_anon.[ID].1@PAGE
Lloh3:
	add	x0, x0, l_anon.[ID].1@PAGEOFF
Lloh4:
	adrp	x2, l_anon.[ID].3@PAGE
Lloh5:
	add	x2, x2, l_anon.[ID].3@PAGEOFF
	mov	w1, #54
	bl	SYM(core::option::expect_failed::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh2, Lloh3

	.globl	_error_bool
	.p2align	2
_error_bool:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	str	xzr, [sp, #8]
	add	x3, sp, #8
	bl	_objc_msgSend
	mov	x8, x0
	mov	x0, #0
	tbz	w8, #0, LBB2_2
LBB2_1:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB2_2:
	ldr	x0, [sp, #8]
	bl	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	b	LBB2_1

	.globl	_error_new
	.p2align	2
_error_new:
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
Lloh6:
	adrp	x1, l_anon.[ID].4@PAGE
Lloh7:
	add	x1, x1, l_anon.[ID].4@PAGEOFF
	bl	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_retained::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	x1, x0
	mov	w0, #1
	b	LBB3_2
	.loh AdrpAdd	Lloh6, Lloh7

	.globl	_error_init
	.p2align	2
_error_init:
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
Lloh8:
	adrp	x1, l_anon.[ID].5@PAGE
Lloh9:
	add	x1, x1, l_anon.[ID].5@PAGEOFF
	bl	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_retained::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	x1, x0
	mov	w0, #1
	b	LBB4_2
	.loh AdrpAdd	Lloh8, Lloh9

	.globl	_error_copy
	.p2align	2
_error_copy:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	str	xzr, [sp, #8]
	add	x2, sp, #8
	bl	_objc_msgSend
	cbz	x0, LBB5_3
	mov	x1, x0
	mov	x0, #0
LBB5_2:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB5_3:
	ldr	x0, [sp, #8]
Lloh10:
	adrp	x1, l_anon.[ID].6@PAGE
Lloh11:
	add	x1, x1, l_anon.[ID].6@PAGEOFF
	bl	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_retained::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	x1, x0
	mov	w0, #1
	b	LBB5_2
	.loh AdrpAdd	Lloh10, Lloh11

	.globl	_error_mutable_copy
	.p2align	2
_error_mutable_copy:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	str	xzr, [sp, #8]
	add	x2, sp, #8
	bl	_objc_msgSend
	cbz	x0, LBB6_3
	mov	x1, x0
	mov	x0, #0
LBB6_2:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB6_3:
	ldr	x0, [sp, #8]
Lloh12:
	adrp	x1, l_anon.[ID].7@PAGE
Lloh13:
	add	x1, x1, l_anon.[ID].7@PAGEOFF
	bl	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_retained::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	x1, x0
	mov	w0, #1
	b	LBB6_2
	.loh AdrpAdd	Lloh12, Lloh13

	.globl	_error_autoreleased
	.p2align	2
_error_autoreleased:
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
	cbz	x0, LBB7_3
	mov	x1, x0
	mov	x0, #0
LBB7_2:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB7_3:
	ldr	x0, [sp, #8]
Lloh14:
	adrp	x1, l_anon.[ID].8@PAGE
Lloh15:
	add	x1, x1, l_anon.[ID].8@PAGEOFF
	bl	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_retained::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	x1, x0
	mov	w0, #1
	b	LBB7_2
	.loh AdrpAdd	Lloh14, Lloh15

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"error parameter should be set if the method returns NULL"

l_anon.[ID].1:
	.ascii	"error parameter should be set if the method returns NO"

l_anon.[ID].2:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].3:
	.quad	l_anon.[ID].2
	.asciz	"6\000\000\000\000\000\000\000\n\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].4:
	.quad	l_anon.[ID].2
	.asciz	"6\000\000\000\000\000\000\000\017\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].5:
	.quad	l_anon.[ID].2
	.asciz	"6\000\000\000\000\000\000\000\026\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].6:
	.quad	l_anon.[ID].2
	.asciz	"6\000\000\000\000\000\000\000\033\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].7:
	.quad	l_anon.[ID].2
	.asciz	"6\000\000\000\000\000\000\000 \000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].8:
	.quad	l_anon.[ID].2
	.asciz	"6\000\000\000\000\000\000\000%\000\000\000\005\000\000"

.subsections_via_symbols
