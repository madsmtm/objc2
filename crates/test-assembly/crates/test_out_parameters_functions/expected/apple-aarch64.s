	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn1_strong_none
	.p2align	2
_fn1_strong_none:
	mov	x1, #0
	b	_SecTrustEvaluateWithError

	.globl	_fn2_strong_some_none
	.p2align	2
_fn2_strong_some_none:
Lfunc_begin0:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	str	xzr, [sp, #8]
Ltmp0:
	add	x1, sp, #8
	bl	_SecTrustEvaluateWithError
Ltmp1:
	ldr	x0, [sp, #8]
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB1_2:
Ltmp2:
	mov	x19, x0
	ldr	x0, [sp, #8]
	cbz	x0, LBB1_4
Ltmp3:
	bl	_CFRelease
Ltmp4:
LBB1_4:
	mov	x0, x19
	bl	__Unwind_Resume
LBB1_5:
Ltmp5:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table1:
Lexception0:
	.byte	255
	.byte	155
	.uleb128 Lttbase0-Lttbaseref0
Lttbaseref0:
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp1-Ltmp0
	.uleb128 Ltmp2-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp3-Lfunc_begin0
	.uleb128 Ltmp4-Ltmp3
	.uleb128 Ltmp5-Lfunc_begin0
	.byte	1
	.uleb128 Ltmp4-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp4
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
	.globl	_fn3_autoreleasing_none
	.p2align	2
_fn3_autoreleasing_none:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	str	wzr, [sp, #8]
	sturb	wzr, [x29, #-1]
	add	x2, sp, #8
	sub	x3, x29, #1
	mov	x1, #0
	bl	_CMAudioDeviceClockGetAudioDevice
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.globl	_fn4_autoreleasing_some_none
	.p2align	2
_fn4_autoreleasing_some_none:
Lfunc_begin1:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	str	xzr, [sp]
	str	wzr, [sp, #8]
	strb	wzr, [sp, #15]
Ltmp6:
	mov	x1, sp
	add	x2, sp, #8
	add	x3, sp, #15
	bl	_CMAudioDeviceClockGetAudioDevice
Ltmp7:
	ldr	x0, [sp]
	cbz	x0, LBB3_4
Ltmp12:
	bl	_CFRetain
Ltmp13:
	ldr	x0, [sp]
LBB3_4:
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB3_5:
Ltmp14:
	mov	x19, x0
	b	LBB3_8
LBB3_6:
Ltmp8:
	mov	x19, x0
	ldr	x0, [sp]
	cbz	x0, LBB3_10
Ltmp9:
	bl	_CFRetain
Ltmp10:
LBB3_8:
	ldr	x0, [sp]
	cbz	x0, LBB3_10
Ltmp15:
	bl	_CFRelease
Ltmp16:
LBB3_10:
	mov	x0, x19
	bl	__Unwind_Resume
LBB3_11:
Ltmp17:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
LBB3_12:
Ltmp11:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end1:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table3:
Lexception1:
	.byte	255
	.byte	155
	.uleb128 Lttbase1-Lttbaseref1
Lttbaseref1:
	.byte	1
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
	.uleb128 Ltmp6-Lfunc_begin1
	.uleb128 Ltmp7-Ltmp6
	.uleb128 Ltmp8-Lfunc_begin1
	.byte	0
	.uleb128 Ltmp12-Lfunc_begin1
	.uleb128 Ltmp13-Ltmp12
	.uleb128 Ltmp14-Lfunc_begin1
	.byte	0
	.uleb128 Ltmp9-Lfunc_begin1
	.uleb128 Ltmp10-Ltmp9
	.uleb128 Ltmp11-Lfunc_begin1
	.byte	1
	.uleb128 Ltmp15-Lfunc_begin1
	.uleb128 Ltmp16-Ltmp15
	.uleb128 Ltmp17-Lfunc_begin1
	.byte	1
	.uleb128 Ltmp16-Lfunc_begin1
	.uleb128 Lfunc_end1-Ltmp16
	.byte	0
	.byte	0
Lcst_end1:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase1:
	.byte	0
	.p2align	2, 0x0

.subsections_via_symbols
