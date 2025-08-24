	.syntax	unified
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn1_iter_create
	.p2align	2
	.code	32
_fn1_iter_create:
	push	{r7, lr}
	mov	r7, sp
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
	pop	{r7, pc}

	.globl	_fn2_iter_once
	.p2align	2
	.code	32
_fn2_iter_once:
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
	movw	r1, :lower16:(L_anon.[ID].0-(LPC1_2+8))
	movt	r1, :upper16:(L_anon.[ID].0-(LPC1_2+8))
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

	.globl	_fn3_use_obj
	.p2align	2
	.code	32
_fn3_use_obj:
	push	{r7, lr}
	mov	r7, sp
	push	{r0}
	mov	r0, sp
	@ InlineAsm Start
	@ InlineAsm End
	mov	sp, r7
	pop	{r7, pc}

	.globl	_fn4_iter
	.p2align	2
	.code	32
_fn4_iter:
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
	movw	r10, :lower16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC3_0+8))
	movt	r10, :upper16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC3_0+8))
LPC3_0:
	ldr	r10, [pc, r10]
	movw	r8, :lower16:(L_anon.[ID].0-(LPC3_1+8))
	movt	r8, :upper16:(L_anon.[ID].0-(LPC3_1+8))
LPC3_1:
	add	r8, pc, r8
	str	r1, [sp, #112]
	mov	r6, #16
	mov	r2, #0
	b	LBB3_2
LBB3_1:
	ldr	r0, [sp, #80]
	add	r1, r2, #1
	str	r1, [sp, #108]
	ldr	r0, [r0, r2, lsl #2]
	bl	_fn3_use_obj
	ldr	r0, [sp, #8]
	ldr	r2, [sp, #108]
	ldr	r1, [sp, #112]
LBB3_2:
	cmp	r2, r1
	blo	LBB3_1
	ldr	r1, [r10]
	cmp	r1, #0
	beq	LBB3_5
LBB3_4:
	str	r6, [sp]
	mov	r2, r5
	mov	r3, r4
	bl	_objc_msgSend
	str	r0, [sp, #112]
	mov	r2, #0
	cmp	r0, #0
	bne	LBB3_1
	b	LBB3_6
LBB3_5:
	mov	r11, r0
	mov	r0, r10
	mov	r1, r8
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r1, r0
	mov	r0, r11
	b	LBB3_4
LBB3_6:
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}

	.globl	_fn5_iter_noop
	.p2align	2
	.code	32
_fn5_iter_noop:
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
	movw	r10, :lower16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC4_0+8))
	movt	r10, :upper16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC4_0+8))
LPC4_0:
	ldr	r10, [pc, r10]
	movw	r8, :lower16:(L_anon.[ID].0-(LPC4_1+8))
	movt	r8, :upper16:(L_anon.[ID].0-(LPC4_1+8))
LPC4_1:
	add	r8, pc, r8
	str	r1, [sp, #112]
	mov	r6, #16
	mov	r2, #0
	b	LBB4_2
LBB4_1:
	add	r2, r2, #1
	str	r2, [sp, #108]
LBB4_2:
	cmp	r2, r1
	blo	LBB4_1
	ldr	r1, [r10]
	cmp	r1, #0
	beq	LBB4_6
LBB4_4:
	str	r6, [sp]
	mov	r2, r5
	mov	r3, r4
	bl	_objc_msgSend
	str	r0, [sp, #112]
	cmp	r0, #0
	beq	LBB4_7
	mov	r1, r0
	mov	r2, #0
	ldr	r0, [sp, #8]
	b	LBB4_1
LBB4_6:
	mov	r11, r0
	mov	r0, r10
	mov	r1, r8
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r1, r0
	mov	r0, r11
	b	LBB4_4
LBB4_7:
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}

	.globl	_fn6_iter_retained
	.p2align	2
	.code	32
_fn6_iter_retained:
Lfunc_begin0:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10, r11}
	sub	r4, sp, #64
	bfc	r4, #0, #4
	mov	sp, r4
	vst1.64	{d8, d9, d10, d11}, [r4:128]!
	vst1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, sp, #192
	add	r1, sp, #16
	add	r2, r1, #88
	vmov.i32	q8, #0x0
	vst1.64	{d16, d17}, [r2]!
	mov	r4, r0
	add	r0, r1, #12
	str	r0, [sp, #12]
	vst1.32	{d16, d17}, [r0]!
	vst1.32	{d16, d17}, [r0]!
	vst1.32	{d16, d17}, [r0]!
	vst1.32	{d16, d17}, [r0]!
	mov	r1, #0
	str	r1, [r2]
	str	r1, [sp, #16]
	str	r4, [sp, #24]
	str	r0, [sp, #8]
	str	r1, [r0]
	str	r1, [sp, #96]
	str	r1, [sp, #100]
	str	r1, [sp, #124]
	str	r1, [sp, #128]
	movw	r0, :lower16:(L_rust_eh_personality$non_lazy_ptr-(LPC5_2+8))
	movt	r0, :upper16:(L_rust_eh_personality$non_lazy_ptr-(LPC5_2+8))
LPC5_2:
	ldr	r0, [pc, r0]
	str	r0, [sp, #164]
	ldr	r0, LCPI5_0
LPC5_0:
	add	r0, pc, r0
	str	r0, [sp, #168]
	str	r7, [sp, #172]
	str	sp, [sp, #180]
	ldr	r0, LCPI5_1
LPC5_1:
	add	r0, pc, r0
	str	r0, [sp, #176]
	add	r0, sp, #140
	bl	__Unwind_SjLj_Register
	mov	r0, r4
	mvn	r5, #0
	mov	r2, #0
	mov	r1, #0
	cmp	r1, r2
	blo	LBB5_4
LBB5_1:
	movw	r1, :lower16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC5_3+8))
	movt	r1, :upper16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC5_3+8))
LPC5_3:
	ldr	r1, [pc, r1]
	ldr	r1, [r1]
	cmp	r1, #0
	beq	LBB5_10
LBB5_2:
	str	r5, [sp, #144]
	mov	r2, #16
	str	r2, [sp]
	ldr	r2, [sp, #8]
	ldr	r3, [sp, #12]
	bl	_objc_msgSend
	str	r0, [sp, #128]
	mov	r1, #0
	str	r1, [sp, #124]
	cmp	r0, #0
	beq	LBB5_11
	ldr	r0, [sp, #96]
	cmp	r0, #0
	beq	LBB5_12
LBB5_4:
	ldr	r0, [sp, #100]
	cmp	r0, #0
	beq	LBB5_8
	ldr	r0, [r0]
	ldr	r2, [sp, #16]
	tst	r2, #1
	beq	LBB5_7
	ldr	r2, [sp, #20]
	cmp	r2, r0
	beq	LBB5_8
	b	LBB5_13
LBB5_7:
	mov	r2, #1
	str	r2, [sp, #16]
	str	r0, [sp, #20]
LBB5_8:
	ldr	r0, [sp, #96]
	add	r2, r1, #1
	str	r2, [sp, #124]
	ldr	r0, [r0, r1, lsl #2]
	str	r5, [sp, #144]
	bl	_objc_retain
	str	r0, [sp, #136]
	ldr	r0, [sp, #136]
	mov	r1, #1
	str	r1, [sp, #144]
Ltmp0:
	bl	_fn3_use_obj
Ltmp1:
	ldr	r0, [sp, #136]
	mvn	r1, #0
	str	r1, [sp, #144]
	bl	_objc_release
	ldr	r0, [sp, #24]
	ldr	r1, [sp, #124]
	ldr	r2, [sp, #128]
	mvn	r5, #0
	cmp	r1, r2
	bhs	LBB5_1
	b	LBB5_4
LBB5_10:
	str	r5, [sp, #144]
	mov	r4, r0
	movw	r0, :lower16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC5_4+8))
	movt	r0, :upper16:(LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC5_4+8))
LPC5_4:
	ldr	r0, [pc, r0]
	movw	r1, :lower16:(L_anon.[ID].0-(LPC5_5+8))
	movt	r1, :upper16:(L_anon.[ID].0-(LPC5_5+8))
LPC5_5:
	add	r1, pc, r1
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r1, r0
	mov	r0, r4
	b	LBB5_2
LBB5_11:
	add	r0, sp, #140
	bl	__Unwind_SjLj_Unregister
	add	r4, sp, #192
	vld1.64	{d8, d9, d10, d11}, [r4:128]!
	vld1.64	{d12, d13, d14, d15}, [r4:128]
	sub	sp, r7, #24
	pop	{r8, r10, r11}
	pop	{r4, r5, r6, r7, pc}
LBB5_12:
	mvn	r0, #0
	str	r0, [sp, #144]
	mov	lr, pc
	b	SYM(objc2_foundation::iter::items_ptr_null::GENERATED_ID, 0)
LBB5_13:
	mvn	r0, #0
	str	r0, [sp, #144]
	mov	lr, pc
	b	SYM(objc2_foundation::iter::mutation_detected::GENERATED_ID, 0)
LBB5_14:
	lsl	r0, r0, #2
	adr	r1, LJTI5_0
	ldr	r0, [r0, r1]
	add	pc, r0, r1
	.p2align	2
LJTI5_0:
	.data_region jt32
	.long	LBB5_16-LJTI5_0
	.long	LBB5_19-LJTI5_0
	.end_data_region
LBB5_16:
Ltmp2:
	ldr	r0, [sp, #148]
	str	r0, [sp, #12]
	ldr	r0, [sp, #136]
	mov	r1, #2
	str	r1, [sp, #144]
Ltmp3:
	bl	_objc_release
Ltmp4:
	b	LBB5_20
LBB5_17:
	ldr	r0, [sp, #144]
	cmp	r0, #2
	bls	LBB5_14
	trap
LBB5_19:
Ltmp5:
	ldr	r0, [sp, #148]
	ldr	r0, [sp, #152]
	mov	lr, pc
	b	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB5_20:
	mvn	r0, #0
	str	r0, [sp, #144]
	ldr	r0, [sp, #12]
	mov	lr, pc
	b	__Unwind_SjLj_Resume
	.p2align	2
	.data_region
LCPI5_0:
	.long	Lexception0-(LPC5_0+8)
LCPI5_1:
	.long	LBB5_17-(LPC5_1+8)
	.end_data_region
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table5:
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

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].0:
	.asciz	"countByEnumeratingWithState:objects:count:"

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2, 0x0
LSYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr:
	.indirect_symbol	SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)
	.long	0
L_rust_eh_personality$non_lazy_ptr:
	.indirect_symbol	_rust_eh_personality
	.long	0

.subsections_via_symbols
