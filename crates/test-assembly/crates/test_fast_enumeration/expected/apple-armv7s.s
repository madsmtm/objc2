	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_iter_create
	.p2align	2
	.code	32
_iter_create:
<<<<<<< HEAD
	str	r1, [r0]
	add	r1, r0, #4
	vmov.i32	q8, #0x0
	vst1.32	{d16, d17}, [r1]!
	vst1.32	{d16, d17}, [r1]!
	vst1.32	{d16, d17}, [r1]!
	vst1.32	{d16, d17}, [r1]!
	vst1.32	{d16, d17}, [r1]!
	vst1.32	{d16, d17}, [r1]!
	mov	r2, #0
	str	r2, [r0, #104]
	str	r2, [r1]
	bx	lr
=======
	push	{r4, r7, lr}
	add	r7, sp, #4
	add	r2, r0, #20
	vmov.i32	q8, #0x0
	vst1.32	{d16, d17}, [r2]!
	mov	r4, #0
	add	r3, r0, #40
	vst1.32	{d16, d17}, [r3]!
	vst1.32	{d16, d17}, [r3]!
	vst1.32	{d16, d17}, [r3]!
	vst1.32	{d16, d17}, [r3]!
	str	r4, [r2]
	str	r4, [r0]
	str	r4, [r0, #8]
	str	r4, [r0, #12]
	str	r4, [r0, #16]
	str	r4, [r3]
	str	r4, [r0, #108]
	str	r1, [r0, #112]
	pop	{r4, r7, pc}
>>>>>>> 11a7eeed2 (Fuzz array mutation)

	.globl	_iter_once
	.p2align	2
	.code	32
_iter_once:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	sub	sp, sp, #4
	mov	r4, r0
	ldrd	r0, r1, [r0, #104]
	cmp	r0, r1
<<<<<<< HEAD
	blo	LBB1_3
	add	r3, r4, #4
	movw	r1, :lower16:(LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC1_0+8))
	movt	r1, :upper16:(LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC1_0+8))
LPC1_0:
	ldr	r1, [pc, r1]
	ldr	r0, [r4]
	ldr	r1, [r1]
=======
	blo	LBB1_4
	ldr	r6, [r4, #112]
	add	r8, r4, #40
	movw	r0, :lower16:(LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC1_0+8))
	movt	r0, :upper16:(LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC1_0+8))
LPC1_0:
	ldr	r0, [pc, r0]
	add	r5, r4, #8
	ldr	r1, [r0]
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	cmp	r1, #0
	beq	LBB1_11
LBB1_2:
	add	r2, r4, #68
	mov	r6, #16
	str	r6, [sp]
	bl	_objc_msgSend
	mov	r1, r0
	mov	r0, #0
	strd	r0, r1, [r4, #104]
	cmp	r1, #0
	beq	LBB1_10
	ldr	r1, [r4, #12]
	cmp	r1, #0
	beq	LBB1_12
LBB1_4:
<<<<<<< HEAD
	sub	sp, r7, #12
	pop	{r4, r5, r6, r7, pc}
LBB1_5:
	movw	r2, :lower16:(LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC1_1+8))
	movt	r2, :upper16:(LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC1_1+8))
=======
	ldr	r1, [r4, #16]
	cmp	r1, #0
	beq	LBB1_9
	ldr	r1, [r1]
	ldr	r2, [r4]
	cmp	r2, #0
	beq	LBB1_8
	ldr	r2, [r4, #4]
	cmp	r2, r1
	beq	LBB1_9
	mov	lr, pc
	b	SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)
LBB1_8:
	mov	r2, #1
	str	r2, [r4]
	str	r1, [r4, #4]
LBB1_9:
	ldr	r1, [r4, #12]
	add	r2, r0, #1
	str	r2, [r4, #104]
	ldr	r0, [r1, r0, lsl #2]
LBB1_10:
	sub	sp, r7, #16
	pop	{r8}
	pop	{r4, r5, r6, r7, pc}
LBB1_11:
	movw	r1, :lower16:(l_anon.[ID].0-(LPC1_1+8))
	movt	r1, :upper16:(l_anon.[ID].0-(LPC1_1+8))
>>>>>>> 11a7eeed2 (Fuzz array mutation)
LPC1_1:
	ldr	r2, [pc, r2]
	movw	r1, :lower16:(l_anon.[ID].0-(LPC1_2+8))
	movt	r1, :upper16:(l_anon.[ID].0-(LPC1_2+8))
LPC1_2:
	add	r1, pc, r1
	mov	r5, r0
	mov	r0, r2
	mov	r6, r3
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r3, r6
	mov	r1, r0
	mov	r0, r5
	b	LBB1_2
LBB1_12:
	mov	lr, pc
	b	SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)

	.globl	_use_obj
	.p2align	2
	.code	32
_use_obj:
	sub	sp, sp, #4
	str	r0, [sp]
	mov	r0, sp
	@ InlineAsm Start
	@ InlineAsm End
	add	sp, sp, #4
	bx	lr

	.globl	_iter
	.p2align	2
	.code	32
_iter:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	sp, sp, #128
	bfc	sp, #0, #3
<<<<<<< HEAD
	add	r2, sp, #8
	add	r3, r2, #80
	vmov.i32	q8, #0x0
	vst1.64	{d16, d17}, [r3]!
	mov	r1, #0
	orr	r4, r2, #4
	mov	r5, r4
	vst1.32	{d16, d17}, [r5]!
	vst1.32	{d16, d17}, [r5]!
	vst1.32	{d16, d17}, [r5]!
	vst1.32	{d16, d17}, [r5]!
	str	r1, [r3]
	str	r0, [sp, #8]
	str	r1, [r5]
	str	r1, [sp, #80]
	str	r1, [sp, #84]
	str	r1, [sp, #108]
	movw	r6, :lower16:(LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC3_0+8))
	movt	r6, :upper16:(LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC3_0+8))
LPC3_0:
	ldr	r6, [pc, r6]
	movw	r8, :lower16:(l_anon.[ID].0-(LPC3_1+8))
	movt	r8, :upper16:(l_anon.[ID].0-(LPC3_1+8))
LPC3_1:
	add	r8, pc, r8
	str	r1, [sp, #112]
	mov	r10, #16
	mov	r2, #0
	b	LBB3_3
LBB3_1:
	str	r10, [sp]
	mov	r2, r5
	mov	r3, r4
	bl	_objc_msgSend
	str	r0, [sp, #112]
	mov	r2, #0
	cmp	r0, #0
	beq	LBB3_6
LBB3_2:
	ldr	r0, [sp, #80]
	add	r1, r2, #1
	str	r1, [sp, #108]
	ldr	r0, [r0, r2, lsl #2]
	bl	_use_obj
	ldr	r0, [sp, #8]
	ldr	r2, [sp, #108]
	ldr	r1, [sp, #112]
LBB3_3:
	cmp	r2, r1
	blo	LBB3_2
	ldr	r1, [r6]
	cmp	r1, #0
	bne	LBB3_1
	mov	r11, r0
	mov	r0, r6
	mov	r1, r8
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r1, r0
	mov	r0, r11
	b	LBB3_1
LBB3_6:
=======
	mov	r6, r0
	add	r0, sp, #8
	add	r1, r0, #20
	vmov.i32	q8, #0x0
	vst1.32	{d16, d17}, [r1]!
	mov	r11, #0
	str	r11, [r1]
	add	r4, r0, #40
	mov	r1, r4
	vst1.64	{d16, d17}, [r1]!
	vst1.64	{d16, d17}, [r1]!
	vst1.64	{d16, d17}, [r1]!
	vst1.64	{d16, d17}, [r1]!
	str	r11, [sp, #8]
	add	r5, r0, #8
	str	r11, [sp, #16]
	str	r11, [sp, #20]
	str	r11, [sp, #24]
	str	r11, [r1]
	str	r11, [sp, #116]
	str	r6, [sp, #120]
	movw	r10, :lower16:(LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC3_0+8))
	movt	r10, :upper16:(LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC3_0+8))
LPC3_0:
	ldr	r10, [pc, r10]
	mov	r8, #16
	mov	r1, #0
	mov	r0, #0
	b	LBB3_3
LBB3_1:
	mov	r2, #1
	str	r2, [sp, #8]
	str	r1, [sp, #12]
LBB3_2:
	ldr	r1, [sp, #20]
	add	r2, r0, #1
	str	r2, [sp, #112]
	ldr	r0, [r1, r0, lsl #2]
	bl	_use_obj
	add	r6, sp, #112
	ldm	r6, {r0, r1, r6}
LBB3_3:
	cmp	r0, r1
	blo	LBB3_7
	ldr	r1, [r10]
	cmp	r1, #0
	beq	LBB3_10
LBB3_5:
	str	r8, [sp]
	mov	r0, r6
	mov	r2, r5
	mov	r3, r4
	bl	_objc_msgSend
	str	r0, [sp, #116]
	str	r11, [sp, #112]
	cmp	r0, #0
	beq	LBB3_11
	ldr	r1, [sp, #20]
	mov	r0, #0
	cmp	r1, #0
	beq	LBB3_12
LBB3_7:
	ldr	r1, [sp, #24]
	cmp	r1, #0
	beq	LBB3_2
	ldr	r1, [r1]
	ldr	r2, [sp, #8]
	cmp	r2, #0
	beq	LBB3_1
	ldr	r2, [sp, #12]
	cmp	r2, r1
	beq	LBB3_2
	b	LBB3_13
LBB3_10:
	mov	r0, r10
	movw	r1, :lower16:(l_anon.[ID].0-(LPC3_1+8))
	movt	r1, :upper16:(l_anon.[ID].0-(LPC3_1+8))
LPC3_1:
	add	r1, pc, r1
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r1, r0
	b	LBB3_5
LBB3_11:
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB3_12:
	mov	lr, pc
	b	SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)
LBB3_13:
	mov	lr, pc
	b	SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)

	.globl	_iter_noop
	.p2align	2
	.code	32
_iter_noop:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	sp, sp, #128
	bfc	sp, #0, #3
<<<<<<< HEAD
	add	r2, sp, #8
	add	r3, r2, #80
	vmov.i32	q8, #0x0
	vst1.64	{d16, d17}, [r3]!
	mov	r1, #0
	orr	r4, r2, #4
	mov	r5, r4
	vst1.32	{d16, d17}, [r5]!
	vst1.32	{d16, d17}, [r5]!
	vst1.32	{d16, d17}, [r5]!
	vst1.32	{d16, d17}, [r5]!
	str	r1, [r3]
	str	r0, [sp, #8]
	str	r1, [r5]
	str	r1, [sp, #80]
	str	r1, [sp, #84]
	str	r1, [sp, #108]
	movw	r6, :lower16:(LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC4_0+8))
	movt	r6, :upper16:(LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC4_0+8))
LPC4_0:
	ldr	r6, [pc, r6]
	movw	r8, :lower16:(l_anon.[ID].0-(LPC4_1+8))
	movt	r8, :upper16:(l_anon.[ID].0-(LPC4_1+8))
LPC4_1:
	add	r8, pc, r8
	str	r1, [sp, #112]
	mov	r10, #16
	mov	r2, #0
	b	LBB4_2
LBB4_1:
	add	r2, r2, #1
	str	r2, [sp, #108]
LBB4_2:
	cmp	r2, r1
	blo	LBB4_1
	ldr	r1, [r6]
	cmp	r1, #0
	beq	LBB4_6
LBB4_4:
	str	r10, [sp]
=======
	add	r1, sp, #8
	add	r2, r1, #20
	vmov.i32	q8, #0x0
	vst1.32	{d16, d17}, [r2]!
	mov	r6, r0
	add	r4, r1, #40
	mov	r0, r4
	vst1.64	{d16, d17}, [r0]!
	vst1.64	{d16, d17}, [r0]!
	vst1.64	{d16, d17}, [r0]!
	vst1.64	{d16, d17}, [r0]!
	mov	r11, #0
	str	r11, [r2]
	str	r11, [sp, #8]
	add	r5, r1, #8
	str	r11, [sp, #16]
	str	r11, [sp, #20]
	str	r11, [sp, #24]
	str	r11, [r0]
	str	r11, [sp, #116]
	movw	r10, :lower16:(LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC4_0+8))
	movt	r10, :upper16:(LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC4_0+8))
LPC4_0:
	ldr	r10, [pc, r10]
	str	r6, [sp, #120]
	mov	r8, #16
	mov	r1, #0
	mov	r0, #0
	mov	r2, #0
	b	LBB4_3
LBB4_1:
	mov	r6, #1
	str	r6, [sp, #8]
	str	r3, [sp, #12]
LBB4_2:
	add	r2, r2, #1
	str	r2, [sp, #112]
	ldr	r6, [sp, #120]
LBB4_3:
	cmp	r2, r0
	blo	LBB4_8
	ldr	r1, [r10]
	cmp	r1, #0
	beq	LBB4_11
LBB4_5:
	str	r8, [sp]
	mov	r0, r6
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	mov	r2, r5
	mov	r3, r4
	bl	_objc_msgSend
	str	r0, [sp, #116]
	str	r11, [sp, #112]
	cmp	r0, #0
<<<<<<< HEAD
	beq	LBB4_7
	mov	r1, r0
	mov	r2, #0
	ldr	r0, [sp, #8]
	b	LBB4_1
LBB4_6:
	mov	r11, r0
	mov	r0, r6
	mov	r1, r8
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r1, r0
	mov	r0, r11
	b	LBB4_4
LBB4_7:
=======
	beq	LBB4_12
	ldr	r1, [sp, #20]
	cmp	r1, #0
	beq	LBB4_13
	mov	r2, #0
	ldr	r1, [sp, #24]
LBB4_8:
	cmp	r1, #0
	beq	LBB4_2
	ldr	r3, [r1]
	ldr	r6, [sp, #8]
	cmp	r6, #0
	beq	LBB4_1
	ldr	r6, [sp, #12]
	cmp	r6, r3
	beq	LBB4_2
	b	LBB4_14
LBB4_11:
	mov	r0, r10
	movw	r1, :lower16:(l_anon.[ID].0-(LPC4_1+8))
	movt	r1, :upper16:(l_anon.[ID].0-(LPC4_1+8))
LPC4_1:
	add	r1, pc, r1
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r1, r0
	b	LBB4_5
LBB4_12:
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB4_13:
	mov	lr, pc
	b	SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)
LBB4_14:
	mov	lr, pc
	b	SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)

	.globl	_iter_retained
	.p2align	2
	.code	32
_iter_retained:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	sp, sp, #128
	bfc	sp, #0, #3
<<<<<<< HEAD
	add	r2, sp, #8
	add	r3, r2, #80
	vmov.i32	q8, #0x0
	vst1.64	{d16, d17}, [r3]!
	mov	r1, #0
	orr	r4, r2, #4
	mov	r5, r4
	vst1.32	{d16, d17}, [r5]!
	vst1.32	{d16, d17}, [r5]!
	vst1.32	{d16, d17}, [r5]!
	vst1.32	{d16, d17}, [r5]!
	str	r1, [r3]
	str	r0, [sp, #8]
	str	r1, [r5]
	str	r1, [sp, #80]
	str	r1, [sp, #84]
	str	r1, [sp, #108]
=======
	mov	r6, r0
	add	r0, sp, #8
	add	r1, r0, #20
	vmov.i32	q8, #0x0
	vst1.32	{d16, d17}, [r1]!
	mov	r11, #0
	str	r11, [r1]
	add	r4, r0, #40
	mov	r1, r4
	vst1.64	{d16, d17}, [r1]!
	vst1.64	{d16, d17}, [r1]!
	vst1.64	{d16, d17}, [r1]!
	vst1.64	{d16, d17}, [r1]!
	str	r11, [sp, #8]
	add	r5, r0, #8
	str	r11, [sp, #16]
	str	r11, [sp, #20]
	str	r11, [sp, #24]
	str	r11, [r1]
	str	r11, [sp, #116]
	str	r6, [sp, #120]
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	movw	r10, :lower16:(LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC5_0+8))
	movt	r10, :upper16:(LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC5_0+8))
LPC5_0:
	ldr	r10, [pc, r10]
<<<<<<< HEAD
	movw	r8, :lower16:(l_anon.[ID].0-(LPC5_1+8))
	movt	r8, :upper16:(l_anon.[ID].0-(LPC5_1+8))
LPC5_1:
	add	r8, pc, r8
	str	r1, [sp, #112]
	mov	r11, #16
	mov	r2, #0
	b	LBB5_3
LBB5_1:
	str	r11, [sp]
	mov	r2, r5
	mov	r3, r4
	bl	_objc_msgSend
	str	r0, [sp, #112]
	mov	r2, #0
	cmp	r0, #0
	beq	LBB5_6
LBB5_2:
	ldr	r0, [sp, #80]
	add	r1, r2, #1
	str	r1, [sp, #108]
	ldr	r0, [r0, r2, lsl #2]
=======
	mov	r8, #16
	mov	r1, #0
	mov	r0, #0
	b	LBB5_3
LBB5_1:
	mov	r2, #1
	str	r2, [sp, #8]
	str	r1, [sp, #12]
LBB5_2:
	ldr	r1, [sp, #20]
	add	r2, r0, #1
	str	r2, [sp, #112]
	ldr	r0, [r1, r0, lsl #2]
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	bl	_objc_retain
	mov	r6, r0
	bl	_use_obj
	mov	r0, r6
	bl	_objc_release
<<<<<<< HEAD
	ldr	r0, [sp, #8]
	ldr	r2, [sp, #108]
	ldr	r1, [sp, #112]
LBB5_3:
	cmp	r2, r1
	blo	LBB5_2
	ldr	r1, [r10]
	cmp	r1, #0
	bne	LBB5_1
	mov	r6, r0
=======
	add	r6, sp, #112
	ldm	r6, {r0, r1, r6}
LBB5_3:
	cmp	r0, r1
	blo	LBB5_7
	ldr	r1, [r10]
	cmp	r1, #0
	beq	LBB5_10
LBB5_5:
	str	r8, [sp]
	mov	r0, r6
	mov	r2, r5
	mov	r3, r4
	bl	_objc_msgSend
	str	r0, [sp, #116]
	str	r11, [sp, #112]
	cmp	r0, #0
	beq	LBB5_11
	ldr	r1, [sp, #20]
	mov	r0, #0
	cmp	r1, #0
	beq	LBB5_12
LBB5_7:
	ldr	r1, [sp, #24]
	cmp	r1, #0
	beq	LBB5_2
	ldr	r1, [r1]
	ldr	r2, [sp, #8]
	cmp	r2, #0
	beq	LBB5_1
	ldr	r2, [sp, #12]
	cmp	r2, r1
	beq	LBB5_2
	b	LBB5_13
LBB5_10:
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	mov	r0, r10
	movw	r1, :lower16:(l_anon.[ID].0-(LPC5_1+8))
	movt	r1, :upper16:(l_anon.[ID].0-(LPC5_1+8))
LPC5_1:
	add	r1, pc, r1
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r1, r0
<<<<<<< HEAD
	mov	r0, r6
	b	LBB5_1
LBB5_6:
=======
	b	LBB5_5
LBB5_11:
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB5_12:
	mov	lr, pc
	b	SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)
LBB5_13:
	mov	lr, pc
	b	SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)

	.section	__TEXT,__const
l_anon.[ID].0:
	.asciz	"countByEnumeratingWithState:objects:count:"

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2, 0x0
LSYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr:
	.indirect_symbol	SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)
	.long	0

.subsections_via_symbols
