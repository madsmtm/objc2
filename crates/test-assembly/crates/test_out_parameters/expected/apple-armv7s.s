	.syntax unified
	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
	.code	32
SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0):
	cmp	r0, #0
	bxeq	lr
LBB0_1:
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r1
	ldr	r0, [r0]
	bl	_objc_retain
	mov	r0, r4
	pop	{r4, r7, lr}
	b	_objc_release

	.p2align	2
	.code	32
SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0):
	push	{r4, r7, lr}
	add	r7, sp, #4
	cmp	r0, #0
	beq	LBB1_3
	mov	r4, r1
	ldr	r0, [r0]
	bl	_objc_retain
	cmp	r4, #0
	beq	LBB1_3
	mov	r0, r4
	pop	{r4, r7, lr}
	b	_objc_release
LBB1_3:
	pop	{r4, r7, pc}

	.p2align	2
	.code	32
SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0):
Lfunc_begin0:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	r4, sp, #64
	bfc	r4, #0, #4
	mov	sp, r4
	vst1.64	{d8, d9, d10, d11}, [r4:128]!
	vst1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, sp, #64
	str	r0, [sp, #8]
	ldr	r0, [sp, #8]
	ldr	r0, [r0]
	ldr	r1, [sp, #8]
	ldr	r1, [r1, #4]
	str	r1, [sp, #4]
	ldr	r4, [r0]
	movw	r0, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC2_2+8))
	movt	r0, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC2_2+8))
LPC2_2:
	ldr	r0, [pc, r0]
	str	r0, [sp, #36]
	ldr	r0, LCPI2_0
LPC2_0:
	add	r0, pc, r0
	str	r0, [sp, #40]
	str	r7, [sp, #44]
	str	sp, [sp, #52]
	ldr	r0, LCPI2_1
LPC2_1:
	add	r0, pc, r0
	str	r0, [sp, #48]
	add	r0, sp, #12
	bl	__Unwind_SjLj_Register
	mov	r0, #1
	str	r0, [sp, #16]
Ltmp0:
	mov	r0, r4
	bl	_objc_retain
Ltmp1:
	mov	r0, #2
	str	r0, [sp, #16]
Ltmp2:
	ldr	r0, [sp, #4]
	bl	_objc_release
Ltmp3:
	ldr	r0, [sp, #8]
	ldr	r0, [r0, #8]
	ldr	r1, [sp, #8]
	ldr	r4, [r1, #12]
	ldr	r0, [r0]
	mvn	r5, #0
	str	r5, [sp, #16]
	bl	_objc_retain
	str	r5, [sp, #16]
	mov	r0, r4
	bl	_objc_release
	add	r0, sp, #12
	bl	__Unwind_SjLj_Unregister
	add	r4, sp, #64
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB2_3:
	lsl	r0, r0, #2
	adr	r1, LJTI2_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI2_0:
	.data_region jt32
	.long	LBB2_5-LJTI2_0
	.long	LBB2_5-LJTI2_0
	.long	LBB2_10-LJTI2_0
	.long	LBB2_10-LJTI2_0
	.end_data_region
LBB2_5:
Ltmp4:
	ldr	r0, [sp, #20]
	str	r0, [sp, #4]
	ldr	r0, [sp, #8]
	ldr	r0, [r0, #8]
	ldr	r1, [sp, #8]
	ldr	r1, [r1, #12]
	str	r1, [sp]
	ldr	r0, [r0]
	mov	r1, #3
	str	r1, [sp, #16]
Ltmp5:
	bl	_objc_retain
Ltmp6:
	mov	r0, #4
	str	r0, [sp, #16]
Ltmp7:
	ldr	r0, [sp]
	bl	_objc_release
Ltmp8:
	b	LBB2_9
LBB2_7:
	ldr	r0, [sp, #16]
	cmp	r0, #4
	bls	LBB2_3
	trap
LBB2_9:
	mvn	r0, #0
	str	r0, [sp, #16]
	ldr	r0, [sp, #4]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
LBB2_10:
Ltmp9:
	ldr	r0, [sp, #20]
	ldr	r0, [sp, #24]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.p2align	2
	.data_region
LCPI2_0:
	.long	Lexception0-(LPC2_0+8)
LCPI2_1:
	.long	LBB2_7-(LPC2_1+8)
	.end_data_region
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table2:
Lexception0:
	.byte	255
	.byte	155
	.uleb128 Lttbase0-Lttbaseref0
Lttbaseref0:
	.byte	3
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.byte	0
	.byte	0
	.byte	1
	.byte	0
	.byte	2
	.byte	1
	.byte	3
	.byte	1
Lcst_end0:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
	.code	32
SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>,)>, 0):
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r1
	ldr	r0, [r0]
	bl	_objc_retain
	cmp	r4, #0
	beq	LBB3_2
	mov	r0, r4
	pop	{r4, r7, lr}
	b	_objc_release
LBB3_2:
	pop	{r4, r7, pc}

	.globl	_nonnull_nonnull
	.p2align	2
	.code	32
_nonnull_nonnull:
Lfunc_begin1:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	r4, sp, #64
	bfc	r4, #0, #4
	mov	sp, r4
	vst1.64	{d8, d9, d10, d11}, [r4:128]!
	vst1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, sp, #64
	mov	r4, r1
	mov	r5, r0
	str	r2, [sp, #4]
	ldr	r0, [sp, #4]
	ldr	r0, [r0]
	str	r0, [sp, #8]
	ldr	r6, [sp, #4]
	movw	r0, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC4_2+8))
	movt	r0, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC4_2+8))
LPC4_2:
	ldr	r0, [pc, r0]
	str	r0, [sp, #36]
	ldr	r0, LCPI4_0
LPC4_0:
	add	r0, pc, r0
	str	r0, [sp, #40]
	str	r7, [sp, #44]
	str	sp, [sp, #52]
	ldr	r0, LCPI4_1
LPC4_1:
	add	r0, pc, r0
	str	r0, [sp, #48]
	add	r0, sp, #12
	bl	__Unwind_SjLj_Register
	mov	r0, #1
	str	r0, [sp, #16]
Ltmp10:
	mov	r0, r5
	mov	r1, r4
	mov	r2, r6
	bl	_objc_msgSend
Ltmp11:
	mov	r4, r0
	ldr	r0, [sp, #4]
	ldr	r0, [r0]
	mvn	r5, #0
	str	r5, [sp, #16]
	bl	_objc_retain
	ldr	r0, [sp, #8]
	str	r5, [sp, #16]
	bl	_objc_release
	add	r0, sp, #12
	bl	__Unwind_SjLj_Unregister
	mov	r0, r4
	add	r4, sp, #64
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB4_2:
	lsl	r0, r0, #2
	adr	r1, LJTI4_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI4_0:
	.data_region jt32
	.long	LBB4_4-LJTI4_0
	.long	LBB4_8-LJTI4_0
	.long	LBB4_8-LJTI4_0
	.end_data_region
LBB4_4:
Ltmp12:
	ldr	r0, [sp, #20]
	str	r0, [sp]
	ldr	r0, [sp, #4]
	ldr	r0, [r0]
	mov	r1, #2
	str	r1, [sp, #16]
Ltmp13:
	bl	_objc_retain
Ltmp14:
	ldr	r0, [sp, #8]
	mov	r1, #3
	str	r1, [sp, #16]
Ltmp15:
	bl	_objc_release
Ltmp16:
	b	LBB4_9
LBB4_6:
	ldr	r0, [sp, #16]
	cmp	r0, #3
	bls	LBB4_2
	trap
LBB4_8:
Ltmp17:
	ldr	r0, [sp, #20]
	ldr	r0, [sp, #24]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB4_9:
	mvn	r0, #0
	str	r0, [sp, #16]
	ldr	r0, [sp]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
	.p2align	2
	.data_region
LCPI4_0:
	.long	Lexception1-(LPC4_0+8)
LCPI4_1:
	.long	LBB4_6-(LPC4_1+8)
	.end_data_region
Lfunc_end1:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table4:
Lexception1:
	.byte	255
	.byte	155
	.uleb128 Lttbase1-Lttbaseref1
Lttbaseref1:
	.byte	3
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
	.byte	0
	.byte	0
	.byte	1
	.byte	1
	.byte	2
	.byte	1
Lcst_end1:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase1:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_null_nonnull
	.p2align	2
	.code	32
_null_nonnull:
Lfunc_begin2:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	r4, sp, #64
	bfc	r4, #0, #4
	mov	sp, r4
	vst1.64	{d8, d9, d10, d11}, [r4:128]!
	vst1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, sp, #64
	mov	r4, r1
	mov	r5, r0
	str	r2, [sp, #4]
	ldr	r6, [sp, #4]
	movw	r0, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC5_2+8))
	movt	r0, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC5_2+8))
LPC5_2:
	ldr	r0, [pc, r0]
	str	r0, [sp, #36]
	ldr	r0, LCPI5_0
LPC5_0:
	add	r0, pc, r0
	str	r0, [sp, #40]
	str	r7, [sp, #44]
	str	sp, [sp, #52]
	ldr	r0, LCPI5_1
LPC5_1:
	add	r0, pc, r0
	str	r0, [sp, #48]
	add	r0, sp, #12
	bl	__Unwind_SjLj_Register
	cmp	r6, #0
	beq	LBB5_2
	ldr	r0, [sp, #4]
	ldr	r0, [r0]
	b	LBB5_3
LBB5_2:
LBB5_3:
	str	r0, [sp, #8]
	ldr	r2, [sp, #4]
	mov	r0, #1
	str	r0, [sp, #16]
Ltmp18:
	mov	r0, r5
	mov	r1, r4
	bl	_objc_msgSend
Ltmp19:
	mov	r4, r0
	ldr	r0, [sp, #4]
	cmp	r0, #0
	beq	LBB5_6
	ldr	r0, [sp, #4]
	ldr	r0, [r0]
	mvn	r5, #0
	str	r5, [sp, #16]
	bl	_objc_retain
	ldr	r0, [sp, #8]
	str	r5, [sp, #16]
	bl	_objc_release
LBB5_6:
	add	r0, sp, #12
	bl	__Unwind_SjLj_Unregister
	mov	r0, r4
	add	r4, sp, #64
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB5_7:
	lsl	r0, r0, #2
	adr	r1, LJTI5_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI5_0:
	.data_region jt32
	.long	LBB5_9-LJTI5_0
	.long	LBB5_12-LJTI5_0
	.end_data_region
LBB5_9:
Ltmp20:
	ldr	r0, [sp, #20]
	str	r0, [sp]
	ldr	r0, [sp, #4]
	ldr	r1, [sp, #8]
	mov	r2, #2
	str	r2, [sp, #16]
Ltmp21:
	bl	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0)
Ltmp22:
	b	LBB5_13
LBB5_10:
	ldr	r0, [sp, #16]
	cmp	r0, #2
	bls	LBB5_7
	trap
LBB5_12:
Ltmp23:
	ldr	r0, [sp, #20]
	ldr	r0, [sp, #24]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB5_13:
	mvn	r0, #0
	str	r0, [sp, #16]
	ldr	r0, [sp]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
	.p2align	2
	.data_region
LCPI5_0:
	.long	Lexception2-(LPC5_0+8)
LCPI5_1:
	.long	LBB5_10-(LPC5_1+8)
	.end_data_region
Lfunc_end2:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table5:
Lexception2:
	.byte	255
	.byte	155
	.uleb128 Lttbase2-Lttbaseref2
Lttbaseref2:
	.byte	3
	.uleb128 Lcst_end2-Lcst_begin2
Lcst_begin2:
	.byte	0
	.byte	0
	.byte	1
	.byte	1
Lcst_end2:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase2:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_nonnull_null
	.p2align	2
	.code	32
_nonnull_null:
Lfunc_begin3:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	r4, sp, #64
	bfc	r4, #0, #4
	mov	sp, r4
	vst1.64	{d8, d9, d10, d11}, [r4:128]!
	vst1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, sp, #64
	mov	r4, r1
	mov	r5, r0
	str	r2, [sp, #4]
	ldr	r0, [sp, #4]
	ldr	r0, [r0]
	str	r0, [sp, #8]
	ldr	r6, [sp, #4]
	movw	r0, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC6_2+8))
	movt	r0, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC6_2+8))
LPC6_2:
	ldr	r0, [pc, r0]
	str	r0, [sp, #36]
	ldr	r0, LCPI6_0
LPC6_0:
	add	r0, pc, r0
	str	r0, [sp, #40]
	str	r7, [sp, #44]
	str	sp, [sp, #52]
	ldr	r0, LCPI6_1
LPC6_1:
	add	r0, pc, r0
	str	r0, [sp, #48]
	add	r0, sp, #12
	bl	__Unwind_SjLj_Register
	mov	r0, #1
	str	r0, [sp, #16]
Ltmp24:
	mov	r0, r5
	mov	r1, r4
	mov	r2, r6
	bl	_objc_msgSend
Ltmp25:
	mov	r4, r0
	ldr	r0, [sp, #4]
	ldr	r0, [r0]
	mvn	r5, #0
	str	r5, [sp, #16]
	bl	_objc_retain
	ldr	r0, [sp, #8]
	cmp	r0, #0
	beq	LBB6_3
	ldr	r0, [sp, #8]
	str	r5, [sp, #16]
	bl	_objc_release
LBB6_3:
	add	r0, sp, #12
	bl	__Unwind_SjLj_Unregister
	mov	r0, r4
	add	r4, sp, #64
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB6_4:
	lsl	r0, r0, #2
	adr	r1, LJTI6_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI6_0:
	.data_region jt32
	.long	LBB6_6-LJTI6_0
	.long	LBB6_9-LJTI6_0
	.end_data_region
LBB6_6:
Ltmp26:
	ldr	r0, [sp, #20]
	str	r0, [sp]
	ldr	r0, [sp, #4]
	ldr	r1, [sp, #8]
	mov	r2, #2
	str	r2, [sp, #16]
Ltmp27:
	bl	SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>,)>, 0)
Ltmp28:
	b	LBB6_10
LBB6_7:
	ldr	r0, [sp, #16]
	cmp	r0, #2
	bls	LBB6_4
	trap
LBB6_9:
Ltmp29:
	ldr	r0, [sp, #20]
	ldr	r0, [sp, #24]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB6_10:
	mvn	r0, #0
	str	r0, [sp, #16]
	ldr	r0, [sp]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
	.p2align	2
	.data_region
LCPI6_0:
	.long	Lexception3-(LPC6_0+8)
LCPI6_1:
	.long	LBB6_7-(LPC6_1+8)
	.end_data_region
Lfunc_end3:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table6:
Lexception3:
	.byte	255
	.byte	155
	.uleb128 Lttbase3-Lttbaseref3
Lttbaseref3:
	.byte	3
	.uleb128 Lcst_end3-Lcst_begin3
Lcst_begin3:
	.byte	0
	.byte	0
	.byte	1
	.byte	1
Lcst_end3:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase3:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_null_null
	.p2align	2
	.code	32
_null_null:
Lfunc_begin4:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	r4, sp, #64
	bfc	r4, #0, #4
	mov	sp, r4
	vst1.64	{d8, d9, d10, d11}, [r4:128]!
	vst1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, sp, #64
	mov	r4, r1
	mov	r5, r0
	str	r2, [sp, #4]
	ldr	r6, [sp, #4]
	movw	r0, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC7_2+8))
	movt	r0, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC7_2+8))
LPC7_2:
	ldr	r0, [pc, r0]
	str	r0, [sp, #36]
	ldr	r0, LCPI7_0
LPC7_0:
	add	r0, pc, r0
	str	r0, [sp, #40]
	str	r7, [sp, #44]
	str	sp, [sp, #52]
	ldr	r0, LCPI7_1
LPC7_1:
	add	r0, pc, r0
	str	r0, [sp, #48]
	add	r0, sp, #12
	bl	__Unwind_SjLj_Register
	cmp	r6, #0
	beq	LBB7_2
	ldr	r0, [sp, #4]
	ldr	r0, [r0]
	b	LBB7_3
LBB7_2:
LBB7_3:
	str	r0, [sp, #8]
	ldr	r2, [sp, #4]
	mov	r0, #1
	str	r0, [sp, #16]
Ltmp30:
	mov	r0, r5
	mov	r1, r4
	bl	_objc_msgSend
Ltmp31:
	mov	r4, r0
	ldr	r0, [sp, #4]
	cmp	r0, #0
	beq	LBB7_7
	ldr	r0, [sp, #4]
	ldr	r0, [r0]
	mvn	r5, #0
	str	r5, [sp, #16]
	bl	_objc_retain
	ldr	r0, [sp, #8]
	cmp	r0, #0
	beq	LBB7_7
	ldr	r0, [sp, #8]
	str	r5, [sp, #16]
	bl	_objc_release
LBB7_7:
	add	r0, sp, #12
	bl	__Unwind_SjLj_Unregister
	mov	r0, r4
	add	r4, sp, #64
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB7_8:
	lsl	r0, r0, #2
	adr	r1, LJTI7_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI7_0:
	.data_region jt32
	.long	LBB7_10-LJTI7_0
	.long	LBB7_13-LJTI7_0
	.end_data_region
LBB7_10:
Ltmp32:
	ldr	r0, [sp, #20]
	str	r0, [sp]
	ldr	r0, [sp, #4]
	ldr	r1, [sp, #8]
	mov	r2, #2
	str	r2, [sp, #16]
Ltmp33:
	bl	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0)
Ltmp34:
	b	LBB7_14
LBB7_11:
	ldr	r0, [sp, #16]
	cmp	r0, #2
	bls	LBB7_8
	trap
LBB7_13:
Ltmp35:
	ldr	r0, [sp, #20]
	ldr	r0, [sp, #24]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB7_14:
	mvn	r0, #0
	str	r0, [sp, #16]
	ldr	r0, [sp]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
	.p2align	2
	.data_region
LCPI7_0:
	.long	Lexception4-(LPC7_0+8)
LCPI7_1:
	.long	LBB7_11-(LPC7_1+8)
	.end_data_region
Lfunc_end4:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table7:
Lexception4:
	.byte	255
	.byte	155
	.uleb128 Lttbase4-Lttbaseref4
Lttbaseref4:
	.byte	3
	.uleb128 Lcst_end4-Lcst_begin4
Lcst_begin4:
	.byte	0
	.byte	0
	.byte	1
	.byte	1
Lcst_end4:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase4:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_two_nonnull_nonnull
	.p2align	2
	.code	32
_two_nonnull_nonnull:
Lfunc_begin5:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	r4, sp, #64
	bfc	r4, #0, #4
	mov	sp, r4
	vst1.64	{d8, d9, d10, d11}, [r4:128]!
	vst1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, sp, #96
	mov	r4, r2
	mov	r8, r1
	mov	r5, r0
	str	r3, [sp, #36]
	ldr	r1, [r2]
	ldr	r0, [sp, #36]
	ldr	r0, [r0]
	str	r0, [sp, #40]
	str	r2, [sp, #20]
	str	r1, [sp, #16]
	str	r1, [sp, #24]
	ldr	r0, [sp, #36]
	str	r0, [sp, #28]
	ldr	r0, [sp, #40]
	str	r0, [sp, #32]
	ldr	r6, [sp, #36]
	movw	r0, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC8_2+8))
	movt	r0, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC8_2+8))
LPC8_2:
	ldr	r0, [pc, r0]
	str	r0, [sp, #68]
	ldr	r0, LCPI8_0
LPC8_0:
	add	r0, pc, r0
	str	r0, [sp, #72]
	str	r7, [sp, #76]
	str	sp, [sp, #84]
	ldr	r0, LCPI8_1
LPC8_1:
	add	r0, pc, r0
	str	r0, [sp, #80]
	add	r0, sp, #44
	bl	__Unwind_SjLj_Register
	mov	r0, #1
	str	r0, [sp, #48]
Ltmp36:
	mov	r0, r5
	mov	r1, r8
	str	r4, [sp, #12]
	mov	r2, r4
	mov	r3, r6
	bl	_objc_msgSend
	str	r0, [sp, #8]
Ltmp37:
	ldr	r0, [sp, #12]
	ldr	r0, [r0]
	mov	r1, #3
	str	r1, [sp, #48]
Ltmp42:
	bl	_objc_retain
Ltmp43:
	mov	r0, #4
	str	r0, [sp, #48]
Ltmp44:
	ldr	r0, [sp, #16]
	bl	_objc_release
Ltmp45:
	ldr	r0, [sp, #36]
	ldr	r0, [r0]
	mvn	r4, #0
	str	r4, [sp, #48]
	bl	_objc_retain
	ldr	r0, [sp, #40]
	str	r4, [sp, #48]
	bl	_objc_release
	add	r0, sp, #44
	bl	__Unwind_SjLj_Unregister
	ldr	r0, [sp, #8]
	add	r4, sp, #96
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB8_4:
	lsl	r0, r0, #2
	adr	r1, LJTI8_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI8_0:
	.data_region jt32
	.long	LBB8_6-LJTI8_0
	.long	LBB8_12-LJTI8_0
	.long	LBB8_7-LJTI8_0
	.long	LBB8_7-LJTI8_0
	.long	LBB8_13-LJTI8_0
	.long	LBB8_13-LJTI8_0
	.end_data_region
LBB8_6:
Ltmp38:
	ldr	r0, [sp, #52]
	str	r0, [sp, #16]
	ldr	r0, [sp, #56]
	mov	r0, #2
	str	r0, [sp, #48]
Ltmp39:
	add	r0, sp, #20
	bl	SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0)
Ltmp40:
	b	LBB8_11
LBB8_7:
Ltmp46:
	ldr	r0, [sp, #52]
	str	r0, [sp, #16]
	ldr	r0, [sp, #56]
	ldr	r0, [sp, #36]
	ldr	r0, [r0]
	mov	r1, #5
	str	r1, [sp, #48]
Ltmp47:
	bl	_objc_retain
Ltmp48:
	ldr	r0, [sp, #40]
	mov	r1, #6
	str	r1, [sp, #48]
Ltmp49:
	bl	_objc_release
Ltmp50:
	b	LBB8_11
LBB8_9:
	ldr	r0, [sp, #48]
	cmp	r0, #6
	bls	LBB8_4
	trap
LBB8_11:
	mvn	r0, #0
	str	r0, [sp, #48]
	ldr	r0, [sp, #16]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
LBB8_12:
Ltmp41:
	ldr	r0, [sp, #52]
	ldr	r0, [sp, #56]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB8_13:
Ltmp51:
	ldr	r0, [sp, #52]
	ldr	r0, [sp, #56]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.p2align	2
	.data_region
LCPI8_0:
	.long	Lexception5-(LPC8_0+8)
LCPI8_1:
	.long	LBB8_9-(LPC8_1+8)
	.end_data_region
Lfunc_end5:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table8:
Lexception5:
	.byte	255
	.byte	155
	.uleb128 Lttbase5-Lttbaseref5
Lttbaseref5:
	.byte	3
	.uleb128 Lcst_end5-Lcst_begin5
Lcst_begin5:
	.byte	0
	.byte	0
	.byte	1
	.byte	1
	.byte	2
	.byte	0
	.byte	3
	.byte	0
	.byte	4
	.byte	1
	.byte	5
	.byte	1
Lcst_end5:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase5:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_call_with_none1
	.p2align	2
	.code	32
_call_with_none1:
	push	{r7, lr}
	mov	r7, sp
	mov	r2, #0
	pop	{r7, lr}
	b	_objc_msgSend

	.globl	_call_with_none2
	.p2align	2
	.code	32
_call_with_none2:
	push	{r7, lr}
	mov	r7, sp
	mov	r2, #0
	pop	{r7, lr}
	b	_objc_msgSend

	.globl	_call_with_none3
	.p2align	2
	.code	32
_call_with_none3:
Lfunc_begin6:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	r4, sp, #64
	bfc	r4, #0, #4
	mov	sp, r4
	vst1.64	{d8, d9, d10, d11}, [r4:128]!
	vst1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, sp, #80
	mov	r4, r1
	mov	r5, r0
	mov	r0, #0
	str	r0, [sp, #12]
	movw	r0, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC11_2+8))
	movt	r0, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC11_2+8))
LPC11_2:
	ldr	r0, [pc, r0]
	str	r0, [sp, #52]
	ldr	r0, LCPI11_0
LPC11_0:
	add	r0, pc, r0
	str	r0, [sp, #56]
	str	r7, [sp, #60]
	str	sp, [sp, #68]
	ldr	r0, LCPI11_1
LPC11_1:
	add	r0, pc, r0
	str	r0, [sp, #64]
	add	r0, sp, #28
	bl	__Unwind_SjLj_Register
	mov	r0, #1
	str	r0, [sp, #32]
Ltmp52:
	add	r2, sp, #12
	mov	r0, r5
	mov	r1, r4
	bl	_objc_msgSend
	str	r0, [sp, #8]
Ltmp53:
	ldr	r0, [sp, #12]
	mov	r1, #3
	str	r1, [sp, #32]
Ltmp58:
	bl	_objc_retain
Ltmp59:
	ldr	r4, [sp, #12]
	add	r0, sp, #28
	bl	__Unwind_SjLj_Unregister
	ldr	r0, [sp, #8]
	mov	r1, r4
	add	r4, sp, #80
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB11_3:
	lsl	r0, r0, #2
	adr	r1, LJTI11_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI11_0:
	.data_region jt32
	.long	LBB11_5-LJTI11_0
	.long	LBB11_13-LJTI11_0
	.long	LBB11_6-LJTI11_0
	.long	LBB11_12-LJTI11_0
	.end_data_region
LBB11_5:
Ltmp54:
	ldr	r0, [sp, #36]
	str	r0, [sp, #8]
	ldr	r0, [sp, #40]
	str	r0, [sp, #4]
	ldr	r0, [sp, #12]
	mov	r1, #2
	str	r1, [sp, #32]
Ltmp55:
	bl	_objc_retain
Ltmp56:
	b	LBB11_7
LBB11_6:
Ltmp60:
	ldr	r0, [sp, #36]
	str	r0, [sp, #8]
	ldr	r0, [sp, #40]
	str	r0, [sp, #4]
LBB11_7:
	ldr	r0, [sp, #8]
	str	r0, [sp, #16]
	ldr	r0, [sp, #4]
	str	r0, [sp, #20]
	ldr	r0, [sp, #12]
	str	r0, [sp, #24]
	ldr	r0, [sp, #24]
	cmp	r0, #0
	beq	LBB11_11
	ldr	r0, [sp, #24]
	mov	r1, #4
	str	r1, [sp, #32]
Ltmp61:
	bl	_objc_release
Ltmp62:
	b	LBB11_11
LBB11_9:
	ldr	r0, [sp, #32]
	cmp	r0, #4
	bls	LBB11_3
	trap
LBB11_11:
	ldr	r0, [sp, #16]
	ldr	r1, [sp, #20]
	mvn	r1, #0
	str	r1, [sp, #32]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
LBB11_12:
Ltmp63:
	ldr	r0, [sp, #36]
	ldr	r0, [sp, #40]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB11_13:
Ltmp57:
	ldr	r0, [sp, #36]
	ldr	r0, [sp, #40]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.p2align	2
	.data_region
LCPI11_0:
	.long	Lexception6-(LPC11_0+8)
LCPI11_1:
	.long	LBB11_9-(LPC11_1+8)
	.end_data_region
Lfunc_end6:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table11:
Lexception6:
	.byte	255
	.byte	155
	.uleb128 Lttbase6-Lttbaseref6
Lttbaseref6:
	.byte	3
	.uleb128 Lcst_end6-Lcst_begin6
Lcst_begin6:
	.byte	0
	.byte	0
	.byte	1
	.byte	1
	.byte	2
	.byte	0
	.byte	3
	.byte	1
Lcst_end6:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase6:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_call_with_none4
	.p2align	2
	.code	32
_call_with_none4:
Lfunc_begin7:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	r4, sp, #64
	bfc	r4, #0, #4
	mov	sp, r4
	vst1.64	{d8, d9, d10, d11}, [r4:128]!
	vst1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, sp, #80
	mov	r4, r1
	mov	r5, r0
	mov	r0, #0
	str	r0, [sp, #12]
	movw	r0, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC12_2+8))
	movt	r0, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC12_2+8))
LPC12_2:
	ldr	r0, [pc, r0]
	str	r0, [sp, #52]
	ldr	r0, LCPI12_0
LPC12_0:
	add	r0, pc, r0
	str	r0, [sp, #56]
	str	r7, [sp, #60]
	str	sp, [sp, #68]
	ldr	r0, LCPI12_1
LPC12_1:
	add	r0, pc, r0
	str	r0, [sp, #64]
	add	r0, sp, #28
	bl	__Unwind_SjLj_Register
	mov	r0, #1
	str	r0, [sp, #32]
Ltmp64:
	add	r2, sp, #12
	mov	r0, r5
	mov	r1, r4
	bl	_objc_msgSend
	str	r0, [sp, #8]
Ltmp65:
	ldr	r0, [sp, #12]
	mov	r1, #3
	str	r1, [sp, #32]
Ltmp70:
	bl	_objc_retain
Ltmp71:
	ldr	r4, [sp, #12]
	add	r0, sp, #28
	bl	__Unwind_SjLj_Unregister
	ldr	r0, [sp, #8]
	mov	r1, r4
	add	r4, sp, #80
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB12_3:
	lsl	r0, r0, #2
	adr	r1, LJTI12_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI12_0:
	.data_region jt32
	.long	LBB12_5-LJTI12_0
	.long	LBB12_13-LJTI12_0
	.long	LBB12_6-LJTI12_0
	.long	LBB12_12-LJTI12_0
	.end_data_region
LBB12_5:
Ltmp66:
	ldr	r0, [sp, #36]
	str	r0, [sp, #8]
	ldr	r0, [sp, #40]
	str	r0, [sp, #4]
	ldr	r0, [sp, #12]
	mov	r1, #2
	str	r1, [sp, #32]
Ltmp67:
	bl	_objc_retain
Ltmp68:
	b	LBB12_7
LBB12_6:
Ltmp72:
	ldr	r0, [sp, #36]
	str	r0, [sp, #8]
	ldr	r0, [sp, #40]
	str	r0, [sp, #4]
LBB12_7:
	ldr	r0, [sp, #8]
	str	r0, [sp, #16]
	ldr	r0, [sp, #4]
	str	r0, [sp, #20]
	ldr	r0, [sp, #12]
	str	r0, [sp, #24]
	ldr	r0, [sp, #24]
	cmp	r0, #0
	beq	LBB12_11
	ldr	r0, [sp, #24]
	mov	r1, #4
	str	r1, [sp, #32]
Ltmp73:
	bl	_objc_release
Ltmp74:
	b	LBB12_11
LBB12_9:
	ldr	r0, [sp, #32]
	cmp	r0, #4
	bls	LBB12_3
	trap
LBB12_11:
	ldr	r0, [sp, #16]
	ldr	r1, [sp, #20]
	mvn	r1, #0
	str	r1, [sp, #32]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
LBB12_12:
Ltmp75:
	ldr	r0, [sp, #36]
	ldr	r0, [sp, #40]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB12_13:
Ltmp69:
	ldr	r0, [sp, #36]
	ldr	r0, [sp, #40]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.p2align	2
	.data_region
LCPI12_0:
	.long	Lexception7-(LPC12_0+8)
LCPI12_1:
	.long	LBB12_9-(LPC12_1+8)
	.end_data_region
Lfunc_end7:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table12:
Lexception7:
	.byte	255
	.byte	155
	.uleb128 Lttbase7-Lttbaseref7
Lttbaseref7:
	.byte	3
	.uleb128 Lcst_end7-Lcst_begin7
Lcst_begin7:
	.byte	0
	.byte	0
	.byte	1
	.byte	1
	.byte	2
	.byte	0
	.byte	3
	.byte	1
Lcst_end7:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase7:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_call_with_some1
	.p2align	2
	.code	32
_call_with_some1:
Lfunc_begin8:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	r4, sp, #64
	bfc	r4, #0, #4
	mov	sp, r4
	vst1.64	{d8, d9, d10, d11}, [r4:128]!
	vst1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, sp, #80
	mov	r4, r1
	mov	r5, r0
	str	r2, [sp, #16]
	ldr	r0, [sp, #16]
	str	r0, [sp, #12]
	movw	r0, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC13_2+8))
	movt	r0, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC13_2+8))
LPC13_2:
	ldr	r0, [pc, r0]
	str	r0, [sp, #52]
	ldr	r0, LCPI13_0
LPC13_0:
	add	r0, pc, r0
	str	r0, [sp, #56]
	str	r7, [sp, #60]
	str	sp, [sp, #68]
	ldr	r0, LCPI13_1
LPC13_1:
	add	r0, pc, r0
	str	r0, [sp, #64]
	add	r0, sp, #28
	bl	__Unwind_SjLj_Register
	mov	r0, #1
	str	r0, [sp, #32]
Ltmp76:
	add	r2, sp, #12
	mov	r0, r5
	mov	r1, r4
	bl	_objc_msgSend
	str	r0, [sp, #8]
Ltmp77:
	ldr	r0, [sp, #12]
	mov	r1, #4
	str	r1, [sp, #32]
Ltmp84:
	bl	_objc_retain
Ltmp85:
	ldr	r0, [sp, #16]
	mov	r1, #5
	str	r1, [sp, #32]
Ltmp86:
	bl	_objc_release
Ltmp87:
	ldr	r4, [sp, #12]
	add	r0, sp, #28
	bl	__Unwind_SjLj_Unregister
	ldr	r0, [sp, #8]
	mov	r1, r4
	add	r4, sp, #80
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB13_4:
	lsl	r0, r0, #2
	adr	r1, LJTI13_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI13_0:
	.data_region jt32
	.long	LBB13_6-LJTI13_0
	.long	LBB13_13-LJTI13_0
	.long	LBB13_13-LJTI13_0
	.long	LBB13_8-LJTI13_0
	.long	LBB13_8-LJTI13_0
	.long	LBB13_14-LJTI13_0
	.end_data_region
LBB13_6:
Ltmp78:
	ldr	r0, [sp, #36]
	str	r0, [sp, #8]
	ldr	r0, [sp, #40]
	str	r0, [sp, #4]
	ldr	r0, [sp, #12]
	mov	r1, #2
	str	r1, [sp, #32]
Ltmp79:
	bl	_objc_retain
Ltmp80:
	ldr	r0, [sp, #16]
	mov	r1, #3
	str	r1, [sp, #32]
Ltmp81:
	bl	_objc_release
Ltmp82:
	b	LBB13_9
LBB13_8:
Ltmp88:
	ldr	r0, [sp, #36]
	str	r0, [sp, #8]
	ldr	r0, [sp, #40]
	str	r0, [sp, #4]
LBB13_9:
	ldr	r0, [sp, #8]
	str	r0, [sp, #20]
	ldr	r0, [sp, #4]
	str	r0, [sp, #24]
	ldr	r0, [sp, #12]
	mov	r1, #6
	str	r1, [sp, #32]
Ltmp89:
	bl	_objc_release
Ltmp90:
	b	LBB13_12
LBB13_10:
	ldr	r0, [sp, #32]
	cmp	r0, #6
	bls	LBB13_4
	trap
LBB13_12:
	ldr	r0, [sp, #20]
	ldr	r1, [sp, #24]
	mvn	r1, #0
	str	r1, [sp, #32]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
LBB13_13:
Ltmp83:
	ldr	r0, [sp, #36]
	ldr	r0, [sp, #40]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB13_14:
Ltmp91:
	ldr	r0, [sp, #36]
	ldr	r0, [sp, #40]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.p2align	2
	.data_region
LCPI13_0:
	.long	Lexception8-(LPC13_0+8)
LCPI13_1:
	.long	LBB13_10-(LPC13_1+8)
	.end_data_region
Lfunc_end8:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table13:
Lexception8:
	.byte	255
	.byte	155
	.uleb128 Lttbase8-Lttbaseref8
Lttbaseref8:
	.byte	3
	.uleb128 Lcst_end8-Lcst_begin8
Lcst_begin8:
	.byte	0
	.byte	0
	.byte	1
	.byte	1
	.byte	2
	.byte	1
	.byte	3
	.byte	0
	.byte	4
	.byte	0
	.byte	5
	.byte	1
Lcst_end8:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase8:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_call_with_some2
	.p2align	2
	.code	32
_call_with_some2:
Lfunc_begin9:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	r4, sp, #64
	bfc	r4, #0, #4
	mov	sp, r4
	vst1.64	{d8, d9, d10, d11}, [r4:128]!
	vst1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, sp, #80
	mov	r4, r1
	mov	r5, r0
	str	r2, [sp, #12]
	ldr	r0, [sp, #12]
	str	r0, [sp, #8]
	movw	r0, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC14_2+8))
	movt	r0, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC14_2+8))
LPC14_2:
	ldr	r0, [pc, r0]
	str	r0, [sp, #52]
	ldr	r0, LCPI14_0
LPC14_0:
	add	r0, pc, r0
	str	r0, [sp, #56]
	str	r7, [sp, #60]
	str	sp, [sp, #68]
	ldr	r0, LCPI14_1
LPC14_1:
	add	r0, pc, r0
	str	r0, [sp, #64]
	add	r0, sp, #28
	bl	__Unwind_SjLj_Register
	mov	r0, #1
	str	r0, [sp, #32]
Ltmp92:
	add	r2, sp, #8
	mov	r0, r5
	mov	r1, r4
	bl	_objc_msgSend
	str	r0, [sp, #4]
Ltmp93:
	ldr	r0, [sp, #8]
	mov	r1, #4
	str	r1, [sp, #32]
Ltmp100:
	bl	_objc_retain
Ltmp101:
	ldr	r0, [sp, #12]
	mov	r1, #5
	str	r1, [sp, #32]
Ltmp102:
	bl	_objc_release
Ltmp103:
	ldr	r4, [sp, #8]
	add	r0, sp, #28
	bl	__Unwind_SjLj_Unregister
	ldr	r0, [sp, #4]
	mov	r1, r4
	add	r4, sp, #80
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB14_4:
	lsl	r0, r0, #2
	adr	r1, LJTI14_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI14_0:
	.data_region jt32
	.long	LBB14_6-LJTI14_0
	.long	LBB14_14-LJTI14_0
	.long	LBB14_14-LJTI14_0
	.long	LBB14_8-LJTI14_0
	.long	LBB14_8-LJTI14_0
	.long	LBB14_15-LJTI14_0
	.end_data_region
LBB14_6:
Ltmp94:
	ldr	r0, [sp, #36]
	str	r0, [sp, #4]
	ldr	r0, [sp, #40]
	str	r0, [sp]
	ldr	r0, [sp, #8]
	mov	r1, #2
	str	r1, [sp, #32]
Ltmp95:
	bl	_objc_retain
Ltmp96:
	ldr	r0, [sp, #12]
	mov	r1, #3
	str	r1, [sp, #32]
Ltmp97:
	bl	_objc_release
Ltmp98:
	b	LBB14_9
LBB14_8:
Ltmp104:
	ldr	r0, [sp, #36]
	str	r0, [sp, #4]
	ldr	r0, [sp, #40]
	str	r0, [sp]
LBB14_9:
	ldr	r0, [sp, #4]
	str	r0, [sp, #16]
	ldr	r0, [sp]
	str	r0, [sp, #20]
	ldr	r0, [sp, #8]
	str	r0, [sp, #24]
	ldr	r0, [sp, #24]
	cmp	r0, #0
	beq	LBB14_13
	ldr	r0, [sp, #24]
	mov	r1, #6
	str	r1, [sp, #32]
Ltmp105:
	bl	_objc_release
Ltmp106:
	b	LBB14_13
LBB14_11:
	ldr	r0, [sp, #32]
	cmp	r0, #6
	bls	LBB14_4
	trap
LBB14_13:
	ldr	r0, [sp, #16]
	ldr	r1, [sp, #20]
	mvn	r1, #0
	str	r1, [sp, #32]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
LBB14_14:
Ltmp99:
	ldr	r0, [sp, #36]
	ldr	r0, [sp, #40]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB14_15:
Ltmp107:
	ldr	r0, [sp, #36]
	ldr	r0, [sp, #40]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.p2align	2
	.data_region
LCPI14_0:
	.long	Lexception9-(LPC14_0+8)
LCPI14_1:
	.long	LBB14_11-(LPC14_1+8)
	.end_data_region
Lfunc_end9:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table14:
Lexception9:
	.byte	255
	.byte	155
	.uleb128 Lttbase9-Lttbaseref9
Lttbaseref9:
	.byte	3
	.uleb128 Lcst_end9-Lcst_begin9
Lcst_begin9:
	.byte	0
	.byte	0
	.byte	1
	.byte	1
	.byte	2
	.byte	1
	.byte	3
	.byte	0
	.byte	4
	.byte	0
	.byte	5
	.byte	1
Lcst_end9:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase9:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_call_with_some3
	.p2align	2
	.code	32
_call_with_some3:
Lfunc_begin10:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	r4, sp, #64
	bfc	r4, #0, #4
	mov	sp, r4
	vst1.64	{d8, d9, d10, d11}, [r4:128]!
	vst1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, sp, #80
	mov	r4, r1
	mov	r5, r0
	str	r2, [sp, #12]
	ldr	r0, [sp, #12]
	str	r0, [sp, #8]
	movw	r0, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC15_2+8))
	movt	r0, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC15_2+8))
LPC15_2:
	ldr	r0, [pc, r0]
	str	r0, [sp, #52]
	ldr	r0, LCPI15_0
LPC15_0:
	add	r0, pc, r0
	str	r0, [sp, #56]
	str	r7, [sp, #60]
	str	sp, [sp, #68]
	ldr	r0, LCPI15_1
LPC15_1:
	add	r0, pc, r0
	str	r0, [sp, #64]
	add	r0, sp, #28
	bl	__Unwind_SjLj_Register
	mov	r0, #1
	str	r0, [sp, #32]
Ltmp108:
	add	r2, sp, #8
	mov	r0, r5
	mov	r1, r4
	bl	_objc_msgSend
	str	r0, [sp, #4]
Ltmp109:
	ldr	r0, [sp, #8]
	mov	r1, #4
	str	r1, [sp, #32]
Ltmp116:
	bl	_objc_retain
Ltmp117:
	ldr	r0, [sp, #12]
	mov	r1, #5
	str	r1, [sp, #32]
Ltmp118:
	bl	_objc_release
Ltmp119:
	ldr	r4, [sp, #8]
	add	r0, sp, #28
	bl	__Unwind_SjLj_Unregister
	ldr	r0, [sp, #4]
	mov	r1, r4
	add	r4, sp, #80
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB15_4:
	lsl	r0, r0, #2
	adr	r1, LJTI15_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI15_0:
	.data_region jt32
	.long	LBB15_6-LJTI15_0
	.long	LBB15_14-LJTI15_0
	.long	LBB15_14-LJTI15_0
	.long	LBB15_8-LJTI15_0
	.long	LBB15_8-LJTI15_0
	.long	LBB15_15-LJTI15_0
	.end_data_region
LBB15_6:
Ltmp110:
	ldr	r0, [sp, #36]
	str	r0, [sp, #4]
	ldr	r0, [sp, #40]
	str	r0, [sp]
	ldr	r0, [sp, #8]
	mov	r1, #2
	str	r1, [sp, #32]
Ltmp111:
	bl	_objc_retain
Ltmp112:
	ldr	r0, [sp, #12]
	mov	r1, #3
	str	r1, [sp, #32]
Ltmp113:
	bl	_objc_release
Ltmp114:
	b	LBB15_9
LBB15_8:
Ltmp120:
	ldr	r0, [sp, #36]
	str	r0, [sp, #4]
	ldr	r0, [sp, #40]
	str	r0, [sp]
LBB15_9:
	ldr	r0, [sp, #4]
	str	r0, [sp, #16]
	ldr	r0, [sp]
	str	r0, [sp, #20]
	ldr	r0, [sp, #8]
	str	r0, [sp, #24]
	ldr	r0, [sp, #24]
	cmp	r0, #0
	beq	LBB15_13
	ldr	r0, [sp, #24]
	mov	r1, #6
	str	r1, [sp, #32]
Ltmp121:
	bl	_objc_release
Ltmp122:
	b	LBB15_13
LBB15_11:
	ldr	r0, [sp, #32]
	cmp	r0, #6
	bls	LBB15_4
	trap
LBB15_13:
	ldr	r0, [sp, #16]
	ldr	r1, [sp, #20]
	mvn	r1, #0
	str	r1, [sp, #32]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
LBB15_14:
Ltmp115:
	ldr	r0, [sp, #36]
	ldr	r0, [sp, #40]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB15_15:
Ltmp123:
	ldr	r0, [sp, #36]
	ldr	r0, [sp, #40]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.p2align	2
	.data_region
LCPI15_0:
	.long	Lexception10-(LPC15_0+8)
LCPI15_1:
	.long	LBB15_11-(LPC15_1+8)
	.end_data_region
Lfunc_end10:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table15:
Lexception10:
	.byte	255
	.byte	155
	.uleb128 Lttbase10-Lttbaseref10
Lttbaseref10:
	.byte	3
	.uleb128 Lcst_end10-Lcst_begin10
Lcst_begin10:
	.byte	0
	.byte	0
	.byte	1
	.byte	1
	.byte	2
	.byte	1
	.byte	3
	.byte	0
	.byte	4
	.byte	0
	.byte	5
	.byte	1
Lcst_end10:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase10:
	.byte	0
	.p2align	2, 0x0

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2, 0x0
L_rust_eh_personality$non_lazy_ptr:
	.indirect_symbol	_rust_eh_personality
	.long	0

.subsections_via_symbols
