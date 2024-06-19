	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_iter_create
	.p2align	2
	.code	32
_iter_create:
	add	r2, r0, #80
	vmov.i32	q8, #0x0
	vst1.32	{d16, d17}, [r2]!
	mov	r3, #0
	str	r3, [r2]
	add	r2, r0, #4
	vst1.32	{d16, d17}, [r2]!
	vst1.32	{d16, d17}, [r2]!
	vst1.32	{d16, d17}, [r2]!
	vst1.32	{d16, d17}, [r2]!
	str	r1, [r0]
	str	r3, [r2]
	str	r3, [r0, #72]
	str	r3, [r0, #76]
	str	r3, [r0, #100]
	str	r3, [r0, #104]
	bx	lr

	.globl	_iter_once
	.p2align	2
	.code	32
_iter_once:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8}
	sub	sp, sp, #4
	mov	r4, r0
	ldrd	r0, r1, [r0, #100]
	cmp	r0, r1
	blo	LBB1_3
	add	r3, r4, #4
	ldr	r0, [r4]
	movw	r1, :lower16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC1_0+8))
	movt	r1, :upper16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC1_0+8))
LPC1_0:
	ldr	r1, [pc, r1]
	add	r2, r4, #68
	ldr	r1, [r1]
	cmp	r1, #0
	beq	LBB1_5
LBB1_2:
	mov	r6, #16
	str	r6, [sp]
	bl	_objc_msgSend
	mov	r1, r0
	mov	r0, #0
	strd	r0, r1, [r4, #100]
	cmp	r1, #0
	beq	LBB1_4
LBB1_3:
	ldr	r1, [r4, #72]
	add	r2, r0, #1
	str	r2, [r4, #100]
	ldr	r0, [r1, r0, lsl #2]
LBB1_4:
	sub	sp, r7, #16
	pop	{r8}
	pop	{r4, r5, r6, r7, pc}
LBB1_5:
	movw	r9, :lower16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC1_1+8))
	movt	r9, :upper16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC1_1+8))
LPC1_1:
	ldr	r9, [pc, r9]
	movw	r1, :lower16:(l_anon.[ID].0-(LPC1_2+8))
	movt	r1, :upper16:(l_anon.[ID].0-(LPC1_2+8))
LPC1_2:
	add	r1, pc, r1
	mov	r8, r0
	mov	r0, r9
	mov	r6, r3
	mov	r5, r2
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r2, r5
	mov	r3, r6
	mov	r1, r0
	mov	r0, r8
	b	LBB1_2

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
	sub	sp, sp, #120
	bfc	sp, #0, #3
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
	movw	r6, :lower16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC3_0+8))
	movt	r6, :upper16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC3_0+8))
LPC3_0:
	ldr	r6, [pc, r6]
	movw	r8, :lower16:(l_anon.[ID].0-(LPC3_1+8))
	movt	r8, :upper16:(l_anon.[ID].0-(LPC3_1+8))
LPC3_1:
	add	r8, pc, r8
	str	r1, [sp, #112]
	mov	r10, #16
	mov	r2, #0
	cmp	r2, r1
	blo	LBB3_3
LBB3_1:
	ldr	r1, [r6]
	cmp	r1, #0
	beq	LBB3_5
LBB3_2:
	str	r10, [sp]
	mov	r2, r5
	mov	r3, r4
	bl	_objc_msgSend
	str	r0, [sp, #112]
	mov	r2, #0
	cmp	r0, #0
	beq	LBB3_6
LBB3_3:
	ldr	r0, [sp, #80]
	add	r1, r2, #1
	str	r1, [sp, #108]
	ldr	r0, [r0, r2, lsl #2]
	cmp	r0, #0
	beq	LBB3_6
	bl	_use_obj
	ldr	r0, [sp, #8]
	ldr	r2, [sp, #108]
	ldr	r1, [sp, #112]
	cmp	r2, r1
	bhs	LBB3_1
	b	LBB3_3
LBB3_5:
	mov	r11, r0
	mov	r0, r6
	mov	r1, r8
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r1, r0
	mov	r0, r11
	b	LBB3_2
LBB3_6:
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}

	.globl	_iter_noop
	.p2align	2
	.code	32
_iter_noop:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	sp, sp, #120
	bfc	sp, #0, #3
	add	r2, sp, #8
	add	r3, r2, #80
	vmov.i32	q8, #0x0
	vst1.64	{d16, d17}, [r3]!
	mov	r1, #0
	str	r1, [r3]
	orr	r4, r2, #4
	mov	r5, r4
	vst1.32	{d16, d17}, [r5]!
	vst1.32	{d16, d17}, [r5]!
	vst1.32	{d16, d17}, [r5]!
	vst1.32	{d16, d17}, [r5]!
	str	r0, [sp, #8]
	str	r1, [r5]
	str	r1, [sp, #80]
	str	r1, [sp, #84]
	str	r1, [sp, #108]
	str	r1, [sp, #112]
	movw	r6, :lower16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC4_0+8))
	movt	r6, :upper16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC4_0+8))
LPC4_0:
	ldr	r6, [pc, r6]
	movw	r8, :lower16:(l_anon.[ID].0-(LPC4_1+8))
	movt	r8, :upper16:(l_anon.[ID].0-(LPC4_1+8))
LPC4_1:
	add	r8, pc, r8
	mov	r10, #16
	mov	r0, #0
	mov	r2, #0
	b	LBB4_2
LBB4_1:
	add	r3, r2, #1
	str	r3, [sp, #108]
	ldr	r2, [r1, r2, lsl #2]
	cmp	r2, #0
	mov	r2, r3
	beq	LBB4_7
LBB4_2:
	cmp	r2, r0
	blo	LBB4_1
	ldr	r0, [sp, #8]
	ldr	r1, [r6]
	cmp	r1, #0
	beq	LBB4_6
LBB4_4:
	str	r10, [sp]
	mov	r2, r5
	mov	r3, r4
	bl	_objc_msgSend
	str	r0, [sp, #112]
	cmp	r0, #0
	beq	LBB4_7
	mov	r2, #0
	ldr	r1, [sp, #80]
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
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}

	.globl	_iter_retained
	.p2align	2
	.code	32
_iter_retained:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	sp, sp, #128
	bfc	sp, #0, #3
	add	r1, sp, #8
	add	r2, r1, #88
	vmov.i32	q8, #0x0
	vst1.64	{d16, d17}, [r2]!
	mov	r11, #0
	add	r4, r1, #12
	mov	r5, r4
	vst1.32	{d16, d17}, [r5]!
	vst1.32	{d16, d17}, [r5]!
	vst1.32	{d16, d17}, [r5]!
	vst1.32	{d16, d17}, [r5]!
	str	r11, [r2]
	str	r11, [sp, #8]
	str	r0, [sp, #16]
	str	r11, [r5]
	str	r11, [sp, #88]
	str	r11, [sp, #92]
	str	r11, [sp, #116]
	str	r11, [sp, #120]
	movw	r10, :lower16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC5_0+8))
	movt	r10, :upper16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC5_0+8))
LPC5_0:
	ldr	r10, [pc, r10]
	mov	r8, #16
	mov	r2, #0
	mov	r1, #0
	cmp	r1, r2
	blo	LBB5_4
LBB5_1:
	ldr	r1, [r10]
	cmp	r1, #0
	beq	LBB5_10
LBB5_2:
	str	r8, [sp]
	mov	r2, r5
	mov	r3, r4
	bl	_objc_msgSend
	str	r0, [sp, #120]
	str	r11, [sp, #116]
	cmp	r0, #0
	beq	LBB5_11
	ldr	r0, [sp, #88]
	mov	r1, #0
	cmp	r0, #0
	beq	LBB5_12
LBB5_4:
	ldr	r0, [sp, #92]
	cmp	r0, #0
	beq	LBB5_8
	ldr	r0, [r0]
	ldr	r2, [sp, #8]
	cmp	r2, #0
	beq	LBB5_7
	ldr	r2, [sp, #12]
	cmp	r2, r0
	beq	LBB5_8
	b	LBB5_13
LBB5_7:
	mov	r2, #1
	str	r2, [sp, #8]
	str	r0, [sp, #12]
LBB5_8:
	ldr	r0, [sp, #88]
	add	r2, r1, #1
	str	r2, [sp, #116]
	ldr	r0, [r0, r1, lsl #2]
	cmp	r0, #0
	beq	LBB5_11
	bl	_objc_retain
	mov	r6, r0
	bl	_use_obj
	mov	r0, r6
	bl	_objc_release
	ldr	r0, [sp, #16]
	ldr	r1, [sp, #116]
	ldr	r2, [sp, #120]
	cmp	r1, r2
	bhs	LBB5_1
	b	LBB5_4
LBB5_10:
	mov	r6, r0
	mov	r0, r10
	movw	r1, :lower16:(l_anon.[ID].0-(LPC5_1+8))
	movt	r1, :upper16:(l_anon.[ID].0-(LPC5_1+8))
LPC5_1:
	add	r1, pc, r1
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r1, r0
	mov	r0, r6
	b	LBB5_2
LBB5_11:
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB5_12:
	mov	lr, pc
	b	SYM(objc2_foundation::iter::items_ptr_null::GENERATED_ID, 0)
LBB5_13:
	mov	lr, pc
	b	SYM(objc2_foundation::iter::mutation_detected::GENERATED_ID, 0)

	.section	__TEXT,__const
l_anon.[ID].0:
	.asciz	"countByEnumeratingWithState:objects:count:"

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2, 0x0
LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr:
	.indirect_symbol	SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)
	.long	0

.subsections_via_symbols
