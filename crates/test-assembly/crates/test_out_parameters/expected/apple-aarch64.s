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
Lfunc_begin0:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	str	xzr, [sp, #8]
Ltmp0:
	add	x2, sp, #8
	bl	_objc_msgSend
Ltmp1:
	mov	x19, x0
	ldr	x0, [sp, #8]
Ltmp2:
	bl	_objc_retain
Ltmp3:
	ldr	x1, [sp, #8]
	mov	x0, x19
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB7_3:
Ltmp4:
	mov	x19, x0
	ldr	x0, [sp, #8]
	cbz	x0, LBB7_5
Ltmp5:
	bl	_objc_release
Ltmp6:
LBB7_5:
	mov	x0, x19
	bl	__Unwind_Resume
LBB7_6:
Ltmp7:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table7:
Lexception0:
	.byte	255
	.byte	155
	.uleb128 Lttbase0-Lttbaseref0
Lttbaseref0:
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp3-Ltmp0
	.uleb128 Ltmp4-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp5-Lfunc_begin0
	.uleb128 Ltmp6-Ltmp5
	.uleb128 Ltmp7-Lfunc_begin0
	.byte	1
	.uleb128 Ltmp6-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp6
	.byte	0
	.byte	0
Lcst_end0:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_call_with_none4
	.p2align	2
_call_with_none4:
Lfunc_begin1:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	str	xzr, [sp, #8]
Ltmp8:
	add	x2, sp, #8
	bl	_objc_msgSend
Ltmp9:
	mov	x19, x0
	ldr	x0, [sp, #8]
Ltmp10:
	bl	_objc_retain
Ltmp11:
	ldr	x1, [sp, #8]
	mov	x0, x19
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB8_3:
Ltmp12:
	mov	x19, x0
	ldr	x0, [sp, #8]
	cbz	x0, LBB8_5
Ltmp13:
	bl	_objc_release
Ltmp14:
LBB8_5:
	mov	x0, x19
	bl	__Unwind_Resume
LBB8_6:
Ltmp15:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end1:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table8:
Lexception1:
	.byte	255
	.byte	155
	.uleb128 Lttbase1-Lttbaseref1
Lttbaseref1:
	.byte	1
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
	.uleb128 Ltmp8-Lfunc_begin1
	.uleb128 Ltmp11-Ltmp8
	.uleb128 Ltmp12-Lfunc_begin1
	.byte	0
	.uleb128 Ltmp13-Lfunc_begin1
	.uleb128 Ltmp14-Ltmp13
	.uleb128 Ltmp15-Lfunc_begin1
	.byte	1
	.uleb128 Ltmp14-Lfunc_begin1
	.uleb128 Lfunc_end1-Ltmp14
	.byte	0
	.byte	0
Lcst_end1:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase1:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_call_with_some1
	.p2align	2
_call_with_some1:
Lfunc_begin2:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x2
	str	x2, [sp, #8]
Ltmp16:
	add	x2, sp, #8
	bl	_objc_msgSend
Ltmp17:
	mov	x20, x0
	ldr	x0, [sp, #8]
Ltmp18:
	bl	_objc_retain
Ltmp19:
Ltmp20:
	mov	x0, x19
	bl	_objc_release
Ltmp21:
	ldr	x1, [sp, #8]
	mov	x0, x20
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB9_4:
Ltmp22:
	mov	x19, x0
	ldr	x0, [sp, #8]
Ltmp23:
	bl	_objc_release
Ltmp24:
	mov	x0, x19
	bl	__Unwind_Resume
LBB9_6:
Ltmp25:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end2:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table9:
Lexception2:
	.byte	255
	.byte	155
	.uleb128 Lttbase2-Lttbaseref2
Lttbaseref2:
	.byte	1
	.uleb128 Lcst_end2-Lcst_begin2
Lcst_begin2:
	.uleb128 Ltmp16-Lfunc_begin2
	.uleb128 Ltmp21-Ltmp16
	.uleb128 Ltmp22-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp23-Lfunc_begin2
	.uleb128 Ltmp24-Ltmp23
	.uleb128 Ltmp25-Lfunc_begin2
	.byte	1
	.uleb128 Ltmp24-Lfunc_begin2
	.uleb128 Lfunc_end2-Ltmp24
	.byte	0
	.byte	0
Lcst_end2:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase2:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_call_with_some2
	.p2align	2
_call_with_some2:
Lfunc_begin3:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x2
	str	x2, [sp, #8]
Ltmp26:
	add	x2, sp, #8
	bl	_objc_msgSend
Ltmp27:
	mov	x20, x0
	ldr	x0, [sp, #8]
Ltmp28:
	bl	_objc_retain
Ltmp29:
Ltmp30:
	mov	x0, x19
	bl	_objc_release
Ltmp31:
	ldr	x1, [sp, #8]
	mov	x0, x20
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB10_4:
Ltmp32:
	mov	x19, x0
	ldr	x0, [sp, #8]
	cbz	x0, LBB10_6
Ltmp33:
	bl	_objc_release
Ltmp34:
LBB10_6:
	mov	x0, x19
	bl	__Unwind_Resume
LBB10_7:
Ltmp35:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end3:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table10:
Lexception3:
	.byte	255
	.byte	155
	.uleb128 Lttbase3-Lttbaseref3
Lttbaseref3:
	.byte	1
	.uleb128 Lcst_end3-Lcst_begin3
Lcst_begin3:
	.uleb128 Ltmp26-Lfunc_begin3
	.uleb128 Ltmp31-Ltmp26
	.uleb128 Ltmp32-Lfunc_begin3
	.byte	0
	.uleb128 Ltmp33-Lfunc_begin3
	.uleb128 Ltmp34-Ltmp33
	.uleb128 Ltmp35-Lfunc_begin3
	.byte	1
	.uleb128 Ltmp34-Lfunc_begin3
	.uleb128 Lfunc_end3-Ltmp34
	.byte	0
	.byte	0
Lcst_end3:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase3:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_call_with_some3
	.p2align	2
_call_with_some3:
Lfunc_begin4:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x2
	str	x2, [sp, #8]
Ltmp36:
	add	x2, sp, #8
	bl	_objc_msgSend
Ltmp37:
	mov	x20, x0
	ldr	x0, [sp, #8]
Ltmp38:
	bl	_objc_retain
Ltmp39:
Ltmp40:
	mov	x0, x19
	bl	_objc_release
Ltmp41:
	ldr	x1, [sp, #8]
	mov	x0, x20
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB11_4:
Ltmp42:
	mov	x19, x0
	ldr	x0, [sp, #8]
	cbz	x0, LBB11_6
Ltmp43:
	bl	_objc_release
Ltmp44:
LBB11_6:
	mov	x0, x19
	bl	__Unwind_Resume
LBB11_7:
Ltmp45:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end4:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table11:
Lexception4:
	.byte	255
	.byte	155
	.uleb128 Lttbase4-Lttbaseref4
Lttbaseref4:
	.byte	1
	.uleb128 Lcst_end4-Lcst_begin4
Lcst_begin4:
	.uleb128 Ltmp36-Lfunc_begin4
	.uleb128 Ltmp41-Ltmp36
	.uleb128 Ltmp42-Lfunc_begin4
	.byte	0
	.uleb128 Ltmp43-Lfunc_begin4
	.uleb128 Ltmp44-Ltmp43
	.uleb128 Ltmp45-Lfunc_begin4
	.byte	1
	.uleb128 Ltmp44-Lfunc_begin4
	.uleb128 Lfunc_end4-Ltmp44
	.byte	0
	.byte	0
Lcst_end4:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase4:
	.byte	0
	.p2align	2, 0x0

.subsections_via_symbols
