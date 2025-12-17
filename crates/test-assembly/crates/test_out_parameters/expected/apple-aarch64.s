	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macros::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0):
	cbz	x0, LBB0_2
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x1
	ldr	x0, [x0]
	bl	_objc_retain
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_release
LBB0_2:
	ret

	.p2align	2
SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macros::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0):
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	cbz	x0, LBB1_3
	mov	x19, x1
	ldr	x0, [x0]
	bl	_objc_retain
	cbz	x19, LBB1_3
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_release
LBB1_3:
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret

	.p2align	2
SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macros::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macros::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0):
Lfunc_begin0:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x0
	ldp	x8, x20, [x0]
	ldr	x0, [x8]
Ltmp0:
	bl	_objc_retain
Ltmp1:
Ltmp2:
	mov	x0, x20
	bl	_objc_release
Ltmp3:
	ldp	x8, x19, [x19, #16]
	ldr	x0, [x8]
	bl	_objc_retain
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_release
LBB2_3:
Ltmp4:
	mov	x20, x0
	ldp	x8, x19, [x19, #16]
	ldr	x0, [x8]
Ltmp5:
	bl	_objc_retain
Ltmp6:
Ltmp7:
	mov	x0, x19
	bl	_objc_release
Ltmp8:
	mov	x0, x20
	bl	__Unwind_Resume
LBB2_6:
Ltmp9:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table2:
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
	.uleb128 Ltmp3-Lfunc_begin0
	.uleb128 Ltmp5-Ltmp3
	.byte	0
	.byte	0
	.uleb128 Ltmp5-Lfunc_begin0
	.uleb128 Ltmp8-Ltmp5
	.uleb128 Ltmp9-Lfunc_begin0
	.byte	1
	.uleb128 Ltmp8-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp8
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
	.p2align	2
SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macros::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>,)>, 0):
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x1
	ldr	x0, [x0]
	bl	_objc_retain
	cbz	x19, LBB3_2
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_release
LBB3_2:
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret

	.globl	_fn1_nonnull_nonnull
	.p2align	2
_fn1_nonnull_nonnull:
Lfunc_begin1:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x20, x2
	ldr	x19, [x2]
Ltmp10:
	bl	_objc_msgSend
Ltmp11:
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
LBB4_2:
Ltmp12:
	mov	x21, x0
	ldr	x0, [x20]
Ltmp13:
	bl	_objc_retain
Ltmp14:
Ltmp15:
	mov	x0, x19
	bl	_objc_release
Ltmp16:
	mov	x0, x21
	bl	__Unwind_Resume
LBB4_5:
Ltmp17:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end1:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table4:
Lexception1:
	.byte	255
	.byte	155
	.uleb128 Lttbase1-Lttbaseref1
Lttbaseref1:
	.byte	1
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
	.uleb128 Ltmp10-Lfunc_begin1
	.uleb128 Ltmp11-Ltmp10
	.uleb128 Ltmp12-Lfunc_begin1
	.byte	0
	.uleb128 Ltmp11-Lfunc_begin1
	.uleb128 Ltmp13-Ltmp11
	.byte	0
	.byte	0
	.uleb128 Ltmp13-Lfunc_begin1
	.uleb128 Ltmp16-Ltmp13
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

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn2_null_nonnull
	.p2align	2
_fn2_null_nonnull:
Lfunc_begin2:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x20, x2
	cbz	x2, LBB5_2
	ldr	x19, [x20]
	b	LBB5_3
LBB5_2:
LBB5_3:
Ltmp18:
	mov	x2, x20
	bl	_objc_msgSend
Ltmp19:
	cbz	x20, LBB5_6
	ldr	x8, [x20]
	mov	x20, x0
	mov	x0, x8
	bl	_objc_retain
	mov	x0, x19
	bl	_objc_release
	mov	x0, x20
LBB5_6:
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret
LBB5_7:
Ltmp20:
	mov	x21, x0
Ltmp21:
	mov	x0, x20
	mov	x1, x19
	bl	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macros::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0)
Ltmp22:
	mov	x0, x21
	bl	__Unwind_Resume
LBB5_9:
Ltmp23:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end2:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table5:
Lexception2:
	.byte	255
	.byte	155
	.uleb128 Lttbase2-Lttbaseref2
Lttbaseref2:
	.byte	1
	.uleb128 Lcst_end2-Lcst_begin2
Lcst_begin2:
	.uleb128 Ltmp18-Lfunc_begin2
	.uleb128 Ltmp19-Ltmp18
	.uleb128 Ltmp20-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp19-Lfunc_begin2
	.uleb128 Ltmp21-Ltmp19
	.byte	0
	.byte	0
	.uleb128 Ltmp21-Lfunc_begin2
	.uleb128 Ltmp22-Ltmp21
	.uleb128 Ltmp23-Lfunc_begin2
	.byte	1
	.uleb128 Ltmp22-Lfunc_begin2
	.uleb128 Lfunc_end2-Ltmp22
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
	.globl	_fn3_nonnull_null
	.p2align	2
_fn3_nonnull_null:
Lfunc_begin3:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x20, x2
	ldr	x19, [x2]
Ltmp24:
	bl	_objc_msgSend
Ltmp25:
	mov	x21, x0
	ldr	x0, [x20]
	bl	_objc_retain
	cbz	x19, LBB6_3
	mov	x0, x19
	bl	_objc_release
LBB6_3:
	mov	x0, x21
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret
LBB6_4:
Ltmp26:
	mov	x21, x0
Ltmp27:
	mov	x0, x20
	mov	x1, x19
	bl	SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macros::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>,)>, 0)
Ltmp28:
	mov	x0, x21
	bl	__Unwind_Resume
LBB6_6:
Ltmp29:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end3:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table6:
Lexception3:
	.byte	255
	.byte	155
	.uleb128 Lttbase3-Lttbaseref3
Lttbaseref3:
	.byte	1
	.uleb128 Lcst_end3-Lcst_begin3
Lcst_begin3:
	.uleb128 Ltmp24-Lfunc_begin3
	.uleb128 Ltmp25-Ltmp24
	.uleb128 Ltmp26-Lfunc_begin3
	.byte	0
	.uleb128 Ltmp25-Lfunc_begin3
	.uleb128 Ltmp27-Ltmp25
	.byte	0
	.byte	0
	.uleb128 Ltmp27-Lfunc_begin3
	.uleb128 Ltmp28-Ltmp27
	.uleb128 Ltmp29-Lfunc_begin3
	.byte	1
	.uleb128 Ltmp28-Lfunc_begin3
	.uleb128 Lfunc_end3-Ltmp28
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
	.globl	_fn4_null_null
	.p2align	2
_fn4_null_null:
Lfunc_begin4:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x20, x2
	cbz	x2, LBB7_2
	ldr	x19, [x20]
	b	LBB7_3
LBB7_2:
LBB7_3:
Ltmp30:
	mov	x2, x20
	bl	_objc_msgSend
Ltmp31:
	mov	x21, x0
	cbz	x20, LBB7_7
	ldr	x0, [x20]
	bl	_objc_retain
	cbz	x19, LBB7_7
	mov	x0, x19
	bl	_objc_release
LBB7_7:
	mov	x0, x21
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret
LBB7_8:
Ltmp32:
	mov	x21, x0
Ltmp33:
	mov	x0, x20
	mov	x1, x19
	bl	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macros::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0)
Ltmp34:
	mov	x0, x21
	bl	__Unwind_Resume
LBB7_10:
Ltmp35:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end4:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table7:
Lexception4:
	.byte	255
	.byte	155
	.uleb128 Lttbase4-Lttbaseref4
Lttbaseref4:
	.byte	1
	.uleb128 Lcst_end4-Lcst_begin4
Lcst_begin4:
	.uleb128 Ltmp30-Lfunc_begin4
	.uleb128 Ltmp31-Ltmp30
	.uleb128 Ltmp32-Lfunc_begin4
	.byte	0
	.uleb128 Ltmp31-Lfunc_begin4
	.uleb128 Ltmp33-Ltmp31
	.byte	0
	.byte	0
	.uleb128 Ltmp33-Lfunc_begin4
	.uleb128 Ltmp34-Ltmp33
	.uleb128 Ltmp35-Lfunc_begin4
	.byte	1
	.uleb128 Ltmp34-Lfunc_begin4
	.uleb128 Lfunc_end4-Ltmp34
	.byte	0
	.byte	0
Lcst_end4:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase4:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn5_two_nonnull_nonnull
	.p2align	2
_fn5_two_nonnull_nonnull:
Lfunc_begin5:
	sub	sp, sp, #96
	stp	x24, x23, [sp, #32]
	stp	x22, x21, [sp, #48]
	stp	x20, x19, [sp, #64]
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	mov	x20, x3
	mov	x23, x2
	ldr	x22, [x2]
	ldr	x19, [x3]
	stp	x2, x22, [sp]
	stp	x3, x19, [sp, #16]
Ltmp36:
	bl	_objc_msgSend
Ltmp37:
	mov	x21, x0
	ldr	x0, [x23]
Ltmp42:
	bl	_objc_retain
Ltmp43:
Ltmp44:
	mov	x0, x22
	bl	_objc_release
Ltmp45:
	ldr	x0, [x20]
	bl	_objc_retain
	mov	x0, x19
	bl	_objc_release
	mov	x0, x21
	ldp	x29, x30, [sp, #80]
	ldp	x20, x19, [sp, #64]
	ldp	x22, x21, [sp, #48]
	ldp	x24, x23, [sp, #32]
	add	sp, sp, #96
	ret
LBB8_4:
Ltmp38:
	mov	x21, x0
Ltmp39:
	mov	x0, sp
	bl	SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macros::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macros::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0)
Ltmp40:
	b	LBB8_8
LBB8_5:
Ltmp41:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
LBB8_6:
Ltmp46:
	mov	x21, x0
	ldr	x0, [x20]
Ltmp47:
	bl	_objc_retain
Ltmp48:
Ltmp49:
	mov	x0, x19
	bl	_objc_release
Ltmp50:
LBB8_8:
	mov	x0, x21
	bl	__Unwind_Resume
LBB8_9:
Ltmp51:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end5:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table8:
Lexception5:
	.byte	255
	.byte	155
	.uleb128 Lttbase5-Lttbaseref5
Lttbaseref5:
	.byte	1
	.uleb128 Lcst_end5-Lcst_begin5
Lcst_begin5:
	.uleb128 Ltmp36-Lfunc_begin5
	.uleb128 Ltmp37-Ltmp36
	.uleb128 Ltmp38-Lfunc_begin5
	.byte	0
	.uleb128 Ltmp42-Lfunc_begin5
	.uleb128 Ltmp45-Ltmp42
	.uleb128 Ltmp46-Lfunc_begin5
	.byte	0
	.uleb128 Ltmp45-Lfunc_begin5
	.uleb128 Ltmp39-Ltmp45
	.byte	0
	.byte	0
	.uleb128 Ltmp39-Lfunc_begin5
	.uleb128 Ltmp40-Ltmp39
	.uleb128 Ltmp41-Lfunc_begin5
	.byte	1
	.uleb128 Ltmp47-Lfunc_begin5
	.uleb128 Ltmp50-Ltmp47
	.uleb128 Ltmp51-Lfunc_begin5
	.byte	1
	.uleb128 Ltmp50-Lfunc_begin5
	.uleb128 Lfunc_end5-Ltmp50
	.byte	0
	.byte	0
Lcst_end5:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase5:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn6_call_with_none1
	.p2align	2
_fn6_call_with_none1:
	mov	x2, #0
	b	_objc_msgSend

	.globl	_fn6_call_with_none2
	.p2align	2
_fn6_call_with_none2:
	mov	x2, #0
	b	_objc_msgSend

	.globl	_fn6_call_with_none3
	.p2align	2
_fn6_call_with_none3:
Lfunc_begin6:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	str	xzr, [sp, #8]
Ltmp52:
	add	x2, sp, #8
	bl	_objc_msgSend
Ltmp53:
	mov	x19, x0
	ldr	x0, [sp, #8]
Ltmp58:
	bl	_objc_retain
Ltmp59:
	ldr	x1, [sp, #8]
	mov	x0, x19
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB11_3:
Ltmp60:
	mov	x19, x0
	b	LBB11_5
LBB11_4:
Ltmp54:
	mov	x19, x0
	ldr	x0, [sp, #8]
Ltmp55:
	bl	_objc_retain
Ltmp56:
LBB11_5:
	ldr	x0, [sp, #8]
	cbz	x0, LBB11_7
Ltmp61:
	bl	_objc_release
Ltmp62:
LBB11_7:
	mov	x0, x19
	bl	__Unwind_Resume
LBB11_8:
Ltmp63:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
LBB11_9:
Ltmp57:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end6:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table11:
Lexception6:
	.byte	255
	.byte	155
	.uleb128 Lttbase6-Lttbaseref6
Lttbaseref6:
	.byte	1
	.uleb128 Lcst_end6-Lcst_begin6
Lcst_begin6:
	.uleb128 Ltmp52-Lfunc_begin6
	.uleb128 Ltmp53-Ltmp52
	.uleb128 Ltmp54-Lfunc_begin6
	.byte	0
	.uleb128 Ltmp58-Lfunc_begin6
	.uleb128 Ltmp59-Ltmp58
	.uleb128 Ltmp60-Lfunc_begin6
	.byte	0
	.uleb128 Ltmp55-Lfunc_begin6
	.uleb128 Ltmp56-Ltmp55
	.uleb128 Ltmp57-Lfunc_begin6
	.byte	1
	.uleb128 Ltmp61-Lfunc_begin6
	.uleb128 Ltmp62-Ltmp61
	.uleb128 Ltmp63-Lfunc_begin6
	.byte	1
	.uleb128 Ltmp62-Lfunc_begin6
	.uleb128 Lfunc_end6-Ltmp62
	.byte	0
	.byte	0
Lcst_end6:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase6:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn6_call_with_none4
	.p2align	2
_fn6_call_with_none4:
Lfunc_begin7:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	str	xzr, [sp, #8]
Ltmp64:
	add	x2, sp, #8
	bl	_objc_msgSend
Ltmp65:
	mov	x19, x0
	ldr	x0, [sp, #8]
Ltmp70:
	bl	_objc_retain
Ltmp71:
	ldr	x1, [sp, #8]
	mov	x0, x19
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB12_3:
Ltmp72:
	mov	x19, x0
	b	LBB12_5
LBB12_4:
Ltmp66:
	mov	x19, x0
	ldr	x0, [sp, #8]
Ltmp67:
	bl	_objc_retain
Ltmp68:
LBB12_5:
	ldr	x0, [sp, #8]
	cbz	x0, LBB12_7
Ltmp73:
	bl	_objc_release
Ltmp74:
LBB12_7:
	mov	x0, x19
	bl	__Unwind_Resume
LBB12_8:
Ltmp75:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
LBB12_9:
Ltmp69:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end7:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table12:
Lexception7:
	.byte	255
	.byte	155
	.uleb128 Lttbase7-Lttbaseref7
Lttbaseref7:
	.byte	1
	.uleb128 Lcst_end7-Lcst_begin7
Lcst_begin7:
	.uleb128 Ltmp64-Lfunc_begin7
	.uleb128 Ltmp65-Ltmp64
	.uleb128 Ltmp66-Lfunc_begin7
	.byte	0
	.uleb128 Ltmp70-Lfunc_begin7
	.uleb128 Ltmp71-Ltmp70
	.uleb128 Ltmp72-Lfunc_begin7
	.byte	0
	.uleb128 Ltmp67-Lfunc_begin7
	.uleb128 Ltmp68-Ltmp67
	.uleb128 Ltmp69-Lfunc_begin7
	.byte	1
	.uleb128 Ltmp73-Lfunc_begin7
	.uleb128 Ltmp74-Ltmp73
	.uleb128 Ltmp75-Lfunc_begin7
	.byte	1
	.uleb128 Ltmp74-Lfunc_begin7
	.uleb128 Lfunc_end7-Ltmp74
	.byte	0
	.byte	0
Lcst_end7:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase7:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn7_call_with_some1
	.p2align	2
_fn7_call_with_some1:
Lfunc_begin8:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x2
	str	x2, [sp, #8]
Ltmp76:
	add	x2, sp, #8
	bl	_objc_msgSend
Ltmp77:
	mov	x20, x0
	ldr	x0, [sp, #8]
Ltmp84:
	bl	_objc_retain
Ltmp85:
Ltmp86:
	mov	x0, x19
	bl	_objc_release
Ltmp87:
	ldr	x1, [sp, #8]
	mov	x0, x20
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB13_4:
Ltmp78:
	mov	x20, x0
	ldr	x0, [sp, #8]
Ltmp79:
	bl	_objc_retain
Ltmp80:
Ltmp81:
	mov	x0, x19
	bl	_objc_release
Ltmp82:
	b	LBB13_8
LBB13_6:
Ltmp83:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
LBB13_7:
Ltmp88:
	mov	x20, x0
LBB13_8:
	ldr	x0, [sp, #8]
Ltmp89:
	bl	_objc_release
Ltmp90:
	mov	x0, x20
	bl	__Unwind_Resume
LBB13_10:
Ltmp91:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end8:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table13:
Lexception8:
	.byte	255
	.byte	155
	.uleb128 Lttbase8-Lttbaseref8
Lttbaseref8:
	.byte	1
	.uleb128 Lcst_end8-Lcst_begin8
Lcst_begin8:
	.uleb128 Ltmp76-Lfunc_begin8
	.uleb128 Ltmp77-Ltmp76
	.uleb128 Ltmp78-Lfunc_begin8
	.byte	0
	.uleb128 Ltmp84-Lfunc_begin8
	.uleb128 Ltmp87-Ltmp84
	.uleb128 Ltmp88-Lfunc_begin8
	.byte	0
	.uleb128 Ltmp79-Lfunc_begin8
	.uleb128 Ltmp82-Ltmp79
	.uleb128 Ltmp83-Lfunc_begin8
	.byte	1
	.uleb128 Ltmp89-Lfunc_begin8
	.uleb128 Ltmp90-Ltmp89
	.uleb128 Ltmp91-Lfunc_begin8
	.byte	1
	.uleb128 Ltmp90-Lfunc_begin8
	.uleb128 Lfunc_end8-Ltmp90
	.byte	0
	.byte	0
Lcst_end8:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase8:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn7_call_with_some2
	.p2align	2
_fn7_call_with_some2:
Lfunc_begin9:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x2
	str	x2, [sp, #8]
Ltmp92:
	add	x2, sp, #8
	bl	_objc_msgSend
Ltmp93:
	mov	x20, x0
	ldr	x0, [sp, #8]
Ltmp100:
	bl	_objc_retain
Ltmp101:
Ltmp102:
	mov	x0, x19
	bl	_objc_release
Ltmp103:
	ldr	x1, [sp, #8]
	mov	x0, x20
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB14_4:
Ltmp94:
	mov	x20, x0
	ldr	x0, [sp, #8]
Ltmp95:
	bl	_objc_retain
Ltmp96:
Ltmp97:
	mov	x0, x19
	bl	_objc_release
Ltmp98:
	b	LBB14_8
LBB14_6:
Ltmp99:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
LBB14_7:
Ltmp104:
	mov	x20, x0
LBB14_8:
	ldr	x0, [sp, #8]
	cbz	x0, LBB14_10
Ltmp105:
	bl	_objc_release
Ltmp106:
LBB14_10:
	mov	x0, x20
	bl	__Unwind_Resume
LBB14_11:
Ltmp107:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end9:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table14:
Lexception9:
	.byte	255
	.byte	155
	.uleb128 Lttbase9-Lttbaseref9
Lttbaseref9:
	.byte	1
	.uleb128 Lcst_end9-Lcst_begin9
Lcst_begin9:
	.uleb128 Ltmp92-Lfunc_begin9
	.uleb128 Ltmp93-Ltmp92
	.uleb128 Ltmp94-Lfunc_begin9
	.byte	0
	.uleb128 Ltmp100-Lfunc_begin9
	.uleb128 Ltmp103-Ltmp100
	.uleb128 Ltmp104-Lfunc_begin9
	.byte	0
	.uleb128 Ltmp95-Lfunc_begin9
	.uleb128 Ltmp98-Ltmp95
	.uleb128 Ltmp99-Lfunc_begin9
	.byte	1
	.uleb128 Ltmp105-Lfunc_begin9
	.uleb128 Ltmp106-Ltmp105
	.uleb128 Ltmp107-Lfunc_begin9
	.byte	1
	.uleb128 Ltmp106-Lfunc_begin9
	.uleb128 Lfunc_end9-Ltmp106
	.byte	0
	.byte	0
Lcst_end9:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase9:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn7_call_with_some3
	.p2align	2
_fn7_call_with_some3:
Lfunc_begin10:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x2
	str	x2, [sp, #8]
Ltmp108:
	add	x2, sp, #8
	bl	_objc_msgSend
Ltmp109:
	mov	x20, x0
	ldr	x0, [sp, #8]
Ltmp116:
	bl	_objc_retain
Ltmp117:
Ltmp118:
	mov	x0, x19
	bl	_objc_release
Ltmp119:
	ldr	x1, [sp, #8]
	mov	x0, x20
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB15_4:
Ltmp110:
	mov	x20, x0
	ldr	x0, [sp, #8]
Ltmp111:
	bl	_objc_retain
Ltmp112:
Ltmp113:
	mov	x0, x19
	bl	_objc_release
Ltmp114:
	b	LBB15_8
LBB15_6:
Ltmp115:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
LBB15_7:
Ltmp120:
	mov	x20, x0
LBB15_8:
	ldr	x0, [sp, #8]
	cbz	x0, LBB15_10
Ltmp121:
	bl	_objc_release
Ltmp122:
LBB15_10:
	mov	x0, x20
	bl	__Unwind_Resume
LBB15_11:
Ltmp123:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end10:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table15:
Lexception10:
	.byte	255
	.byte	155
	.uleb128 Lttbase10-Lttbaseref10
Lttbaseref10:
	.byte	1
	.uleb128 Lcst_end10-Lcst_begin10
Lcst_begin10:
	.uleb128 Ltmp108-Lfunc_begin10
	.uleb128 Ltmp109-Ltmp108
	.uleb128 Ltmp110-Lfunc_begin10
	.byte	0
	.uleb128 Ltmp116-Lfunc_begin10
	.uleb128 Ltmp119-Ltmp116
	.uleb128 Ltmp120-Lfunc_begin10
	.byte	0
	.uleb128 Ltmp111-Lfunc_begin10
	.uleb128 Ltmp114-Ltmp111
	.uleb128 Ltmp115-Lfunc_begin10
	.byte	1
	.uleb128 Ltmp121-Lfunc_begin10
	.uleb128 Ltmp122-Ltmp121
	.uleb128 Ltmp123-Lfunc_begin10
	.byte	1
	.uleb128 Ltmp122-Lfunc_begin10
	.uleb128 Lfunc_end10-Ltmp122
	.byte	0
	.byte	0
Lcst_end10:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase10:
	.byte	0
	.p2align	2, 0x0

.subsections_via_symbols
