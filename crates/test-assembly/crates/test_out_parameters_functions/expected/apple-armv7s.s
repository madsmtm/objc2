	.syntax	unified
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn1_strong_none
	.p2align	2
	.code	32
_fn1_strong_none:
	push	{r7, lr}
	mov	r7, sp
	mov	r1, #0
	pop	{r7, lr}
	b	_SecTrustEvaluateWithError

	.globl	_fn2_strong_some_none
	.p2align	2
	.code	32
_fn2_strong_some_none:
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
	mov	r4, r0
	mov	r0, #0
	str	r0, [sp, #8]
	movw	r0, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC1_2+8))
	movt	r0, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC1_2+8))
LPC1_2:
	ldr	r0, [pc, r0]
	str	r0, [sp, #36]
	ldr	r0, LCPI1_0
LPC1_0:
	add	r0, pc, r0
	str	r0, [sp, #40]
	str	r7, [sp, #44]
	str	sp, [sp, #52]
	ldr	r0, LCPI1_1
LPC1_1:
	add	r0, pc, r0
	str	r0, [sp, #48]
	add	r0, sp, #12
	bl	__Unwind_SjLj_Register
	mov	r0, #1
	str	r0, [sp, #16]
Ltmp0:
	add	r1, sp, #8
	mov	r0, r4
	bl	_SecTrustEvaluateWithError
Ltmp1:
	ldr	r4, [sp, #8]
	add	r0, sp, #12
	bl	__Unwind_SjLj_Unregister
	mov	r0, r4
	add	r4, sp, #64
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB1_2:
	lsl	r0, r0, #2
	adr	r1, LJTI1_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI1_0:
	.data_region jt32
	.long	LBB1_4-LJTI1_0
	.long	LBB1_8-LJTI1_0
	.end_data_region
LBB1_4:
Ltmp2:
	ldr	r0, [sp, #20]
	str	r0, [sp, #4]
	ldr	r0, [sp, #8]
	cmp	r0, #0
	beq	LBB1_9
	mov	r1, #2
	str	r1, [sp, #16]
Ltmp3:
	bl	_CFRelease
Ltmp4:
	b	LBB1_9
LBB1_6:
	ldr	r0, [sp, #16]
	cmp	r0, #2
	bls	LBB1_2
	trap
LBB1_8:
Ltmp5:
	ldr	r0, [sp, #20]
	ldr	r0, [sp, #24]
	mov	lr, pc
	b	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
LBB1_9:
	mvn	r0, #0
	str	r0, [sp, #16]
	ldr	r0, [sp, #4]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
	.p2align	2
	.data_region
LCPI1_0:
	.long	Lexception0-(LPC1_0+8)
LCPI1_1:
	.long	LBB1_6-(LPC1_1+8)
	.end_data_region
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table1:
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
	.byte	1
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
	.code	32
_fn3_autoreleasing_none:
	push	{r7, lr}
	mov	r7, sp
	sub	sp, sp, #8
	mov	r1, #0
	str	r1, [sp]
	strb	r1, [r7, #-1]
	mov	r2, sp
	sub	r3, r7, #1
	bl	_CMAudioDeviceClockGetAudioDevice
	mov	sp, r7
	pop	{r7, pc}

	.globl	_fn4_autoreleasing_some_none
	.p2align	2
	.code	32
_fn4_autoreleasing_some_none:
Lfunc_begin1:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	r4, sp, #64
	bfc	r4, #0, #4
	mov	sp, r4
	vst1.64	{d8, d9, d10, d11}, [r4:128]!
	vst1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, sp, #96
	mov	r4, r0
	mov	r0, #0
	str	r0, [sp, #20]
	str	r0, [sp, #24]
	strb	r0, [sp, #31]
	movw	r0, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC3_2+8))
	movt	r0, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC3_2+8))
LPC3_2:
	ldr	r0, [pc, r0]
	str	r0, [sp, #68]
	ldr	r0, LCPI3_0
LPC3_0:
	add	r0, pc, r0
	str	r0, [sp, #72]
	str	r7, [sp, #76]
	str	sp, [sp, #84]
	ldr	r0, LCPI3_1
LPC3_1:
	add	r0, pc, r0
	str	r0, [sp, #80]
	add	r0, sp, #44
	bl	__Unwind_SjLj_Register
	mov	r0, #1
	str	r0, [sp, #48]
Ltmp6:
	add	r1, sp, #20
	add	r2, sp, #24
	add	r3, sp, #31
	mov	r0, r4
	bl	_CMAudioDeviceClockGetAudioDevice
Ltmp7:
	ldr	r0, [sp, #20]
	cmp	r0, #0
	mov	r4, #0
	beq	LBB3_4
	mov	r1, #3
	str	r1, [sp, #48]
Ltmp12:
	bl	_CFRetain
Ltmp13:
	ldr	r4, [sp, #20]
LBB3_4:
	add	r0, sp, #44
	bl	__Unwind_SjLj_Unregister
	mov	r0, r4
	add	r4, sp, #96
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB3_5:
	lsl	r0, r0, #2
	adr	r1, LJTI3_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI3_0:
	.data_region jt32
	.long	LBB3_7-LJTI3_0
	.long	LBB3_16-LJTI3_0
	.long	LBB3_9-LJTI3_0
	.long	LBB3_15-LJTI3_0
	.end_data_region
LBB3_7:
Ltmp8:
	ldr	r0, [sp, #52]
	str	r0, [sp, #16]
	ldr	r0, [sp, #56]
	str	r0, [sp, #12]
	ldr	r0, [sp, #20]
	cmp	r0, #0
	beq	LBB3_14
	mov	r1, #2
	str	r1, [sp, #48]
Ltmp9:
	bl	_CFRetain
Ltmp10:
	b	LBB3_10
LBB3_9:
Ltmp14:
	ldr	r0, [sp, #52]
	str	r0, [sp, #16]
	ldr	r0, [sp, #56]
	str	r0, [sp, #12]
LBB3_10:
	ldr	r0, [sp, #12]
	str	r0, [sp, #36]
	ldr	r0, [sp, #16]
	str	r0, [sp, #32]
	ldr	r0, [sp, #20]
	str	r0, [sp, #40]
	ldr	r0, [sp, #40]
	ldr	r1, [sp, #36]
	ldr	r1, [sp, #32]
	str	r1, [sp, #16]
	cmp	r0, #0
	beq	LBB3_14
	ldr	r0, [sp, #32]
	str	r0, [sp, #16]
	ldr	r0, [sp, #36]
	ldr	r0, [sp, #40]
	mov	r1, #4
	str	r1, [sp, #48]
Ltmp15:
	bl	_CFRelease
Ltmp16:
	b	LBB3_14
LBB3_12:
	ldr	r0, [sp, #48]
	cmp	r0, #4
	bls	LBB3_5
	trap
LBB3_14:
	mvn	r0, #0
	str	r0, [sp, #48]
	ldr	r0, [sp, #16]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
LBB3_15:
Ltmp17:
	ldr	r0, [sp, #52]
	ldr	r0, [sp, #56]
	mov	lr, pc
	b	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
LBB3_16:
Ltmp11:
	ldr	r0, [sp, #52]
	ldr	r0, [sp, #56]
	mov	lr, pc
	b	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
	.p2align	2
	.data_region
LCPI3_0:
	.long	Lexception1-(LPC3_0+8)
LCPI3_1:
	.long	LBB3_12-(LPC3_1+8)
	.end_data_region
Lfunc_end1:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table3:
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
	.byte	0
	.byte	3
	.byte	1
Lcst_end1:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase1:
	.byte	0
	.p2align	2, 0x0

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2, 0x0
L_rust_eh_personality$non_lazy_ptr:
	.indirect_symbol	_rust_eh_personality
	.long	0

.subsections_via_symbols
