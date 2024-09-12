	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_nonnull_nonnull
	.p2align	2
	.code	32
_nonnull_nonnull:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	mov	r4, r2
	ldr	r5, [r2]
	bl	_objc_msgSend
	mov	r6, r0
	ldr	r0, [r4]
	bl	_objc_retain
	mov	r0, r5
	bl	_objc_release
	mov	r0, r6
	pop	{r4, r5, r6, r7, pc}

	.globl	_null_nonnull
	.p2align	2
	.code	32
_null_nonnull:
	cmp	r2, #0
	beq	LBB1_2
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	ldr	r4, [r2]
	mov	r5, r2
	bl	_objc_msgSend
	mov	r6, r0
	ldr	r0, [r5]
	bl	_objc_retain
	mov	r0, r4
	bl	_objc_release
	mov	r0, r6
	pop	{r4, r5, r6, r7, pc}
LBB1_2:
	mov	r2, #0
	b	_objc_msgSend

	.globl	_nonnull_null
	.p2align	2
	.code	32
_nonnull_null:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	mov	r6, r2
	ldr	r4, [r2]
	bl	_objc_msgSend
	mov	r5, r0
	ldr	r0, [r6]
	bl	_objc_retain
	cmp	r4, #0
	beq	LBB2_2
	mov	r0, r4
	bl	_objc_release
LBB2_2:
	mov	r0, r5
	pop	{r4, r5, r6, r7, pc}

	.globl	_null_null
	.p2align	2
	.code	32
_null_null:
	cmp	r2, #0
	beq	LBB3_4
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	ldr	r4, [r2]
	mov	r6, r2
	bl	_objc_msgSend
	mov	r5, r0
	ldr	r0, [r6]
	bl	_objc_retain
	cmp	r4, #0
	beq	LBB3_3
	mov	r0, r4
	bl	_objc_release
LBB3_3:
	mov	r0, r5
	pop	{r4, r5, r6, r7, pc}
LBB3_4:
	mov	r2, #0
	b	_objc_msgSend

	.globl	_two_nonnull_nonnull
	.p2align	2
	.code	32
_two_nonnull_nonnull:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10}
	mov	r4, r3
	mov	r5, r2
	ldr	r6, [r2]
	ldr	r8, [r3]
	bl	_objc_msgSend
	mov	r10, r0
	ldr	r0, [r5]
	bl	_objc_retain
	mov	r0, r6
	bl	_objc_release
	ldr	r0, [r4]
	bl	_objc_retain
	mov	r0, r8
	bl	_objc_release
	mov	r0, r10
	pop	{r8, r10}
	pop	{r4, r5, r6, r7, pc}

	.globl	_call_with_none1
	.p2align	2
	.code	32
_call_with_none1:
	mov	r2, #0
	b	_objc_msgSend

	.globl	_call_with_none2
	.p2align	2
	.code	32
_call_with_none2:
	mov	r2, #0
	b	_objc_msgSend

	.globl	_call_with_none3
	.p2align	2
	.code	32
_call_with_none3:
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
	mov	r4, r1
	mov	r5, r0
	mov	r0, #0
	movw	r1, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC7_2+8))
	movt	r1, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC7_2+8))
LPC7_2:
	ldr	r1, [pc, r1]
	str	r0, [sp, #8]
	str	r1, [sp, #36]
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
	mov	r0, #1
	str	r0, [sp, #16]
	add	r0, sp, #12
	bl	__Unwind_SjLj_Register
Ltmp0:
	add	r2, sp, #8
	mov	r0, r5
	mov	r1, r4
	bl	_objc_msgSend
	str	r0, [sp, #4]
Ltmp1:
	ldr	r0, [sp, #8]
	mov	r1, #2
	str	r1, [sp, #16]
Ltmp2:
	bl	_objc_retain
Ltmp3:
	ldr	r4, [sp, #8]
	add	r0, sp, #12
	bl	__Unwind_SjLj_Unregister
	ldr	r0, [sp, #4]
	mov	r1, r4
	add	r4, sp, #64
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB7_3:
	lsl	r0, r0, #2
	adr	r1, LJTI7_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI7_0:
	.data_region jt32
	.long	LBB7_5-LJTI7_0
	.long	LBB7_5-LJTI7_0
	.long	LBB7_10-LJTI7_0
	.end_data_region
LBB7_5:
Ltmp4:
	ldr	r0, [sp, #20]
	str	r0, [sp, #4]
	ldr	r0, [sp, #8]
	cmp	r0, #0
	beq	LBB7_9
	mov	r1, #3
	str	r1, [sp, #16]
Ltmp5:
	bl	_objc_release
Ltmp6:
	b	LBB7_9
LBB7_7:
	ldr	r0, [sp, #16]
	cmp	r0, #3
	bls	LBB7_3
	trap
LBB7_9:
	mvn	r0, #0
	str	r0, [sp, #16]
	ldr	r0, [sp, #4]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
LBB7_10:
Ltmp7:
	ldr	r0, [sp, #20]
	ldr	r0, [sp, #24]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.p2align	2
	.data_region
LCPI7_0:
	.long	Lexception0-(LPC7_0+8)
LCPI7_1:
	.long	LBB7_7-(LPC7_1+8)
	.end_data_region
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table7:
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
	.code	32
_call_with_none4:
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
	mov	r0, #0
	movw	r1, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC8_2+8))
	movt	r1, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC8_2+8))
LPC8_2:
	ldr	r1, [pc, r1]
	str	r0, [sp, #8]
	str	r1, [sp, #36]
	ldr	r0, LCPI8_0
LPC8_0:
	add	r0, pc, r0
	str	r0, [sp, #40]
	str	r7, [sp, #44]
	str	sp, [sp, #52]
	ldr	r0, LCPI8_1
LPC8_1:
	add	r0, pc, r0
	str	r0, [sp, #48]
	mov	r0, #1
	str	r0, [sp, #16]
	add	r0, sp, #12
	bl	__Unwind_SjLj_Register
Ltmp8:
	add	r2, sp, #8
	mov	r0, r5
	mov	r1, r4
	bl	_objc_msgSend
	str	r0, [sp, #4]
Ltmp9:
	ldr	r0, [sp, #8]
	mov	r1, #2
	str	r1, [sp, #16]
Ltmp10:
	bl	_objc_retain
Ltmp11:
	ldr	r4, [sp, #8]
	add	r0, sp, #12
	bl	__Unwind_SjLj_Unregister
	ldr	r0, [sp, #4]
	mov	r1, r4
	add	r4, sp, #64
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB8_3:
	lsl	r0, r0, #2
	adr	r1, LJTI8_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI8_0:
	.data_region jt32
	.long	LBB8_5-LJTI8_0
	.long	LBB8_5-LJTI8_0
	.long	LBB8_10-LJTI8_0
	.end_data_region
LBB8_5:
Ltmp12:
	ldr	r0, [sp, #20]
	str	r0, [sp, #4]
	ldr	r0, [sp, #8]
	cmp	r0, #0
	beq	LBB8_9
	mov	r1, #3
	str	r1, [sp, #16]
Ltmp13:
	bl	_objc_release
Ltmp14:
	b	LBB8_9
LBB8_7:
	ldr	r0, [sp, #16]
	cmp	r0, #3
	bls	LBB8_3
	trap
LBB8_9:
	mvn	r0, #0
	str	r0, [sp, #16]
	ldr	r0, [sp, #4]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
LBB8_10:
Ltmp15:
	ldr	r0, [sp, #20]
	ldr	r0, [sp, #24]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.p2align	2
	.data_region
LCPI8_0:
	.long	Lexception1-(LPC8_0+8)
LCPI8_1:
	.long	LBB8_7-(LPC8_1+8)
	.end_data_region
Lfunc_end1:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table8:
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
	.byte	0
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
	.globl	_call_with_some1
	.p2align	2
	.code	32
_call_with_some1:
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
	movw	r0, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC9_2+8))
	movt	r0, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC9_2+8))
LPC9_2:
	ldr	r0, [pc, r0]
	str	r2, [sp, #4]
	str	r2, [sp, #8]
	str	r0, [sp, #36]
	ldr	r0, LCPI9_0
LPC9_0:
	add	r0, pc, r0
	str	r0, [sp, #40]
	str	r7, [sp, #44]
	str	sp, [sp, #52]
	ldr	r0, LCPI9_1
LPC9_1:
	add	r0, pc, r0
	str	r0, [sp, #48]
	mov	r0, #1
	str	r0, [sp, #16]
	add	r0, sp, #12
	bl	__Unwind_SjLj_Register
Ltmp16:
	add	r2, sp, #8
	mov	r0, r5
	mov	r1, r4
	bl	_objc_msgSend
	str	r0, [sp]
Ltmp17:
	ldr	r0, [sp, #8]
	mov	r1, #2
	str	r1, [sp, #16]
Ltmp18:
	bl	_objc_retain
Ltmp19:
	mov	r0, #3
	str	r0, [sp, #16]
Ltmp20:
	ldr	r0, [sp, #4]
	bl	_objc_release
Ltmp21:
	ldr	r4, [sp, #8]
	add	r0, sp, #12
	bl	__Unwind_SjLj_Unregister
	ldr	r0, [sp]
	mov	r1, r4
	add	r4, sp, #64
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB9_4:
	lsl	r0, r0, #2
	adr	r1, LJTI9_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI9_0:
	.data_region jt32
	.long	LBB9_6-LJTI9_0
	.long	LBB9_6-LJTI9_0
	.long	LBB9_6-LJTI9_0
	.long	LBB9_10-LJTI9_0
	.end_data_region
LBB9_6:
Ltmp22:
	ldr	r0, [sp, #20]
	str	r0, [sp, #4]
	ldr	r0, [sp, #8]
	mov	r1, #4
	str	r1, [sp, #16]
Ltmp23:
	bl	_objc_release
Ltmp24:
	b	LBB9_9
LBB9_7:
	ldr	r0, [sp, #16]
	cmp	r0, #4
	bls	LBB9_4
	trap
LBB9_9:
	mvn	r0, #0
	str	r0, [sp, #16]
	ldr	r0, [sp, #4]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
LBB9_10:
Ltmp25:
	ldr	r0, [sp, #20]
	ldr	r0, [sp, #24]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.p2align	2
	.data_region
LCPI9_0:
	.long	Lexception2-(LPC9_0+8)
LCPI9_1:
	.long	LBB9_7-(LPC9_1+8)
	.end_data_region
Lfunc_end2:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table9:
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
	.byte	0
	.byte	2
	.byte	0
	.byte	3
	.byte	1
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
	.code	32
_call_with_some2:
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
	movw	r0, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC10_2+8))
	movt	r0, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC10_2+8))
LPC10_2:
	ldr	r0, [pc, r0]
	str	r2, [sp, #4]
	str	r2, [sp, #8]
	str	r0, [sp, #36]
	ldr	r0, LCPI10_0
LPC10_0:
	add	r0, pc, r0
	str	r0, [sp, #40]
	str	r7, [sp, #44]
	str	sp, [sp, #52]
	ldr	r0, LCPI10_1
LPC10_1:
	add	r0, pc, r0
	str	r0, [sp, #48]
	mov	r0, #1
	str	r0, [sp, #16]
	add	r0, sp, #12
	bl	__Unwind_SjLj_Register
Ltmp26:
	add	r2, sp, #8
	mov	r0, r5
	mov	r1, r4
	bl	_objc_msgSend
	str	r0, [sp]
Ltmp27:
	ldr	r0, [sp, #8]
	mov	r1, #2
	str	r1, [sp, #16]
Ltmp28:
	bl	_objc_retain
Ltmp29:
	mov	r0, #3
	str	r0, [sp, #16]
Ltmp30:
	ldr	r0, [sp, #4]
	bl	_objc_release
Ltmp31:
	ldr	r4, [sp, #8]
	add	r0, sp, #12
	bl	__Unwind_SjLj_Unregister
	ldr	r0, [sp]
	mov	r1, r4
	add	r4, sp, #64
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB10_4:
	lsl	r0, r0, #2
	adr	r1, LJTI10_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI10_0:
	.data_region jt32
	.long	LBB10_6-LJTI10_0
	.long	LBB10_6-LJTI10_0
	.long	LBB10_6-LJTI10_0
	.long	LBB10_11-LJTI10_0
	.end_data_region
LBB10_6:
Ltmp32:
	ldr	r0, [sp, #20]
	str	r0, [sp, #4]
	ldr	r0, [sp, #8]
	cmp	r0, #0
	beq	LBB10_10
	mov	r1, #4
	str	r1, [sp, #16]
Ltmp33:
	bl	_objc_release
Ltmp34:
	b	LBB10_10
LBB10_8:
	ldr	r0, [sp, #16]
	cmp	r0, #4
	bls	LBB10_4
	trap
LBB10_10:
	mvn	r0, #0
	str	r0, [sp, #16]
	ldr	r0, [sp, #4]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
LBB10_11:
Ltmp35:
	ldr	r0, [sp, #20]
	ldr	r0, [sp, #24]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.p2align	2
	.data_region
LCPI10_0:
	.long	Lexception3-(LPC10_0+8)
LCPI10_1:
	.long	LBB10_8-(LPC10_1+8)
	.end_data_region
Lfunc_end3:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table10:
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
	.byte	0
	.byte	2
	.byte	0
	.byte	3
	.byte	1
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
	.code	32
_call_with_some3:
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
	movw	r0, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC11_2+8))
	movt	r0, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC11_2+8))
LPC11_2:
	ldr	r0, [pc, r0]
	str	r2, [sp, #4]
	str	r2, [sp, #8]
	str	r0, [sp, #36]
	ldr	r0, LCPI11_0
LPC11_0:
	add	r0, pc, r0
	str	r0, [sp, #40]
	str	r7, [sp, #44]
	str	sp, [sp, #52]
	ldr	r0, LCPI11_1
LPC11_1:
	add	r0, pc, r0
	str	r0, [sp, #48]
	mov	r0, #1
	str	r0, [sp, #16]
	add	r0, sp, #12
	bl	__Unwind_SjLj_Register
Ltmp36:
	add	r2, sp, #8
	mov	r0, r5
	mov	r1, r4
	bl	_objc_msgSend
	str	r0, [sp]
Ltmp37:
	ldr	r0, [sp, #8]
	mov	r1, #2
	str	r1, [sp, #16]
Ltmp38:
	bl	_objc_retain
Ltmp39:
	mov	r0, #3
	str	r0, [sp, #16]
Ltmp40:
	ldr	r0, [sp, #4]
	bl	_objc_release
Ltmp41:
	ldr	r4, [sp, #8]
	add	r0, sp, #12
	bl	__Unwind_SjLj_Unregister
	ldr	r0, [sp]
	mov	r1, r4
	add	r4, sp, #64
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB11_4:
	lsl	r0, r0, #2
	adr	r1, LJTI11_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI11_0:
	.data_region jt32
	.long	LBB11_6-LJTI11_0
	.long	LBB11_6-LJTI11_0
	.long	LBB11_6-LJTI11_0
	.long	LBB11_11-LJTI11_0
	.end_data_region
LBB11_6:
Ltmp42:
	ldr	r0, [sp, #20]
	str	r0, [sp, #4]
	ldr	r0, [sp, #8]
	cmp	r0, #0
	beq	LBB11_10
	mov	r1, #4
	str	r1, [sp, #16]
Ltmp43:
	bl	_objc_release
Ltmp44:
	b	LBB11_10
LBB11_8:
	ldr	r0, [sp, #16]
	cmp	r0, #4
	bls	LBB11_4
	trap
LBB11_10:
	mvn	r0, #0
	str	r0, [sp, #16]
	ldr	r0, [sp, #4]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
LBB11_11:
Ltmp45:
	ldr	r0, [sp, #20]
	ldr	r0, [sp, #24]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.p2align	2
	.data_region
LCPI11_0:
	.long	Lexception4-(LPC11_0+8)
LCPI11_1:
	.long	LBB11_8-(LPC11_1+8)
	.end_data_region
Lfunc_end4:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table11:
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
	.byte	0
	.byte	2
	.byte	0
	.byte	3
	.byte	1
Lcst_end4:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase4:
	.byte	0
	.p2align	2, 0x0

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2, 0x0
L_rust_eh_personality$non_lazy_ptr:
	.indirect_symbol	_rust_eh_personality
	.long	0

.subsections_via_symbols
