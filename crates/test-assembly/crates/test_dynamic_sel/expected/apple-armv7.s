	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_get_sel
	.p2align	2
	.code	32
_get_sel:
	movw	r0, :lower16:(__MergedGlobals-(LPC0_0+8))
	movt	r0, :upper16:(__MergedGlobals-(LPC0_0+8))
LPC0_0:
	add	r0, pc, r0
	ldr	r0, [r0]
	cmp	r0, #0
	bxne	lr
LBB0_1:
	movw	r0, :lower16:(__MergedGlobals-(LPC0_1+8))
	movt	r0, :upper16:(__MergedGlobals-(LPC0_1+8))
	movw	r1, :lower16:(l_anon.[ID].0-(LPC0_2+8))
	movt	r1, :upper16:(l_anon.[ID].0-(LPC0_2+8))
LPC0_1:
	add	r0, pc, r0
LPC0_2:
	add	r1, pc, r1
	b	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)

	.globl	_get_same_sel
	.p2align	2
	.code	32
_get_same_sel:
	movw	r2, :lower16:(__MergedGlobals-(LPC1_0+8))
	movt	r2, :upper16:(__MergedGlobals-(LPC1_0+8))
LPC1_0:
	add	r2, pc, r2
	ldr	r0, [r2, #4]
	cmp	r0, #0
	bxne	lr
LBB1_1:
	movw	r1, :lower16:(l_anon.[ID].0-(LPC1_1+8))
	add	r0, r2, #4
	movt	r1, :upper16:(l_anon.[ID].0-(LPC1_1+8))
LPC1_1:
	add	r1, pc, r1
	b	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)

	.globl	_get_common_twice
	.p2align	2
	.code	32
_get_common_twice:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	movw	r5, :lower16:(LSYM(objc2::__macro_helpers::common_selectors::alloc_sel::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC2_0+8))
	movt	r5, :upper16:(LSYM(objc2::__macro_helpers::common_selectors::alloc_sel::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC2_0+8))
LPC2_0:
	ldr	r5, [pc, r5]
	ldr	r4, [r5]
	cmp	r4, #0
	beq	LBB2_3
	ldr	r1, [r5]
	cmp	r1, #0
	beq	LBB2_4
LBB2_2:
	mov	r0, r4
	pop	{r4, r5, r7, pc}
LBB2_3:
	movw	r0, :lower16:(LSYM(objc2::__macro_helpers::common_selectors::alloc_sel::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC2_1+8))
	movt	r0, :upper16:(LSYM(objc2::__macro_helpers::common_selectors::alloc_sel::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC2_1+8))
	movw	r1, :lower16:(l_anon.[ID].1-(LPC2_2+8))
	movt	r1, :upper16:(l_anon.[ID].1-(LPC2_2+8))
LPC2_1:
	ldr	r0, [pc, r0]
LPC2_2:
	add	r1, pc, r1
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r4, r0
	ldr	r1, [r5]
	cmp	r1, #0
	bne	LBB2_2
LBB2_4:
	movw	r0, :lower16:(LSYM(objc2::__macro_helpers::common_selectors::alloc_sel::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC2_3+8))
	movt	r0, :upper16:(LSYM(objc2::__macro_helpers::common_selectors::alloc_sel::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr-(LPC2_3+8))
	movw	r1, :lower16:(l_anon.[ID].1-(LPC2_4+8))
	movt	r1, :upper16:(l_anon.[ID].1-(LPC2_4+8))
LPC2_3:
	ldr	r0, [pc, r0]
LPC2_4:
	add	r1, pc, r1
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r1, r0
	mov	r0, r4
	pop	{r4, r5, r7, pc}

	.globl	_get_different_sel
	.p2align	2
	.code	32
_get_different_sel:
	movw	r2, :lower16:(__MergedGlobals-(LPC3_0+8))
	movt	r2, :upper16:(__MergedGlobals-(LPC3_0+8))
LPC3_0:
	add	r2, pc, r2
	ldr	r0, [r2, #8]
	cmp	r0, #0
	bxne	lr
LBB3_1:
	movw	r1, :lower16:(L_anon.[ID].2-(LPC3_1+8))
	add	r0, r2, #8
	movt	r1, :upper16:(L_anon.[ID].2-(LPC3_1+8))
LPC3_1:
	add	r1, pc, r1
	b	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)

	.globl	_unused_sel
	.p2align	2
	.code	32
_unused_sel:
	movw	r0, :lower16:(SYM(test_dynamic_sel[CRATE_ID]::unused_sel::CACHED_SEL, 0)-(LPC4_0+8))
	movt	r0, :upper16:(SYM(test_dynamic_sel[CRATE_ID]::unused_sel::CACHED_SEL, 0)-(LPC4_0+8))
LPC4_0:
	add	r0, pc, r0
	ldr	r0, [r0]
	cmp	r0, #0
	bxne	lr
LBB4_1:
	movw	r0, :lower16:(SYM(test_dynamic_sel[CRATE_ID]::unused_sel::CACHED_SEL, 0)-(LPC4_1+8))
	movt	r0, :upper16:(SYM(test_dynamic_sel[CRATE_ID]::unused_sel::CACHED_SEL, 0)-(LPC4_1+8))
	movw	r1, :lower16:(l_anon.[ID].3-(LPC4_2+8))
	movt	r1, :upper16:(l_anon.[ID].3-(LPC4_2+8))
LPC4_1:
	add	r0, pc, r0
LPC4_2:
	add	r1, pc, r1
	b	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)

	.globl	_use_fns
	.p2align	2
	.code	32
_use_fns:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10}
	movw	r10, :lower16:(__MergedGlobals-(LPC5_0+8))
	mov	r4, r0
	movt	r10, :upper16:(__MergedGlobals-(LPC5_0+8))
LPC5_0:
	add	r10, pc, r10
	ldr	r8, [r10]
	cmp	r8, #0
	beq	LBB5_5
	ldr	r6, [r10, #4]
	cmp	r6, #0
	beq	LBB5_6
LBB5_2:
	ldr	r5, [r10, #8]
	cmp	r5, #0
	beq	LBB5_7
LBB5_3:
	ldr	r0, [r10, #12]
	cmp	r0, #0
	beq	LBB5_8
LBB5_4:
	str	r8, [r4]
	str	r6, [r4, #4]
	str	r5, [r4, #8]
	str	r0, [r4, #12]
	pop	{r8, r10}
	pop	{r4, r5, r6, r7, pc}
LBB5_5:
	movw	r0, :lower16:(__MergedGlobals-(LPC5_1+8))
	movt	r0, :upper16:(__MergedGlobals-(LPC5_1+8))
	movw	r1, :lower16:(l_anon.[ID].0-(LPC5_2+8))
	movt	r1, :upper16:(l_anon.[ID].0-(LPC5_2+8))
LPC5_1:
	add	r0, pc, r0
LPC5_2:
	add	r1, pc, r1
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r8, r0
	ldr	r6, [r10, #4]
	cmp	r6, #0
	bne	LBB5_2
LBB5_6:
	movw	r1, :lower16:(l_anon.[ID].0-(LPC5_3+8))
	add	r0, r10, #4
	movt	r1, :upper16:(l_anon.[ID].0-(LPC5_3+8))
LPC5_3:
	add	r1, pc, r1
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r6, r0
	ldr	r5, [r10, #8]
	cmp	r5, #0
	bne	LBB5_3
LBB5_7:
	movw	r1, :lower16:(L_anon.[ID].2-(LPC5_4+8))
	add	r0, r10, #8
	movt	r1, :upper16:(L_anon.[ID].2-(LPC5_4+8))
LPC5_4:
	add	r1, pc, r1
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r5, r0
	ldr	r0, [r10, #12]
	cmp	r0, #0
	bne	LBB5_4
LBB5_8:
	movw	r1, :lower16:(l_anon.[ID].4-(LPC5_5+8))
	add	r0, r10, #12
	movt	r1, :upper16:(l_anon.[ID].4-(LPC5_5+8))
LPC5_5:
	add	r1, pc, r1
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	b	LBB5_4

	.globl	_use_same_twice
	.p2align	2
	.code	32
_use_same_twice:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	push	{r8}
	movw	r5, :lower16:(__MergedGlobals-(LPC6_0+8))
	mov	r4, r0
	movt	r5, :upper16:(__MergedGlobals-(LPC6_0+8))
LPC6_0:
	add	r5, pc, r5
	ldr	r8, [r5]
	cmp	r8, #0
	beq	LBB6_3
	ldr	r9, [r5]
	cmp	r9, #0
	beq	LBB6_4
LBB6_2:
	strd	r8, r9, [r4]
	pop	{r8}
	pop	{r4, r5, r7, pc}
LBB6_3:
	movw	r0, :lower16:(__MergedGlobals-(LPC6_1+8))
	movt	r0, :upper16:(__MergedGlobals-(LPC6_1+8))
	movw	r1, :lower16:(l_anon.[ID].0-(LPC6_2+8))
	movt	r1, :upper16:(l_anon.[ID].0-(LPC6_2+8))
LPC6_1:
	add	r0, pc, r0
LPC6_2:
	add	r1, pc, r1
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r8, r0
	ldr	r9, [r5]
	cmp	r9, #0
	bne	LBB6_2
LBB6_4:
	movw	r0, :lower16:(__MergedGlobals-(LPC6_3+8))
	movt	r0, :upper16:(__MergedGlobals-(LPC6_3+8))
	movw	r1, :lower16:(l_anon.[ID].0-(LPC6_4+8))
	movt	r1, :upper16:(l_anon.[ID].0-(LPC6_4+8))
LPC6_3:
	add	r0, pc, r0
LPC6_4:
	add	r1, pc, r1
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r9, r0
	strd	r8, r9, [r4]
	pop	{r8}
	pop	{r4, r5, r7, pc}

	.globl	_use_in_loop
	.p2align	2
	.code	32
_use_in_loop:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	cmp	r0, #0
	popeq	{r4, r5, r6, r7, pc}
LBB7_1:
	movw	r5, :lower16:(SYM(test_dynamic_sel[CRATE_ID]::use_in_loop::CACHED_SEL, 0)-(LPC7_0+8))
	mov	r4, r0
	movt	r5, :upper16:(SYM(test_dynamic_sel[CRATE_ID]::use_in_loop::CACHED_SEL, 0)-(LPC7_0+8))
	movw	r6, :lower16:(l_anon.[ID].5-(LPC7_1+8))
	movt	r6, :upper16:(l_anon.[ID].5-(LPC7_1+8))
LPC7_0:
	add	r5, pc, r5
LPC7_1:
	add	r6, pc, r6
	b	LBB7_3
LBB7_2:
	subs	r4, r4, #1
	beq	LBB7_5
LBB7_3:
	ldr	r0, [r5]
	cmp	r0, #0
	bne	LBB7_2
	mov	r0, r5
	mov	r1, r6
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	b	LBB7_2
LBB7_5:
	pop	{r4, r5, r6, r7, pc}

	.section	__TEXT,__const
l_anon.[ID].0:
	.asciz	"simple"

l_anon.[ID].1:
	.asciz	"alloc"

	.section	__TEXT,__literal16,16byte_literals
L_anon.[ID].2:
	.asciz	"i:am:different:"

	.section	__TEXT,__const
l_anon.[ID].3:
	.asciz	"unused"

l_anon.[ID].4:
	.asciz	"fourthSel"

l_anon.[ID].5:
	.asciz	"loopedSelector"

.zerofill __DATA,__bss,SYM(test_dynamic_sel[CRATE_ID]::unused_sel::CACHED_SEL, 0),4,2
.zerofill __DATA,__bss,SYM(test_dynamic_sel[CRATE_ID]::use_in_loop::CACHED_SEL, 0),4,2
.zerofill __DATA,__bss,__MergedGlobals,16,2
	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2, 0x0
LSYM(objc2::__macro_helpers::common_selectors::alloc_sel::CACHED_SEL::GENERATED_ID, 0)$non_lazy_ptr:
	.indirect_symbol	SYM(objc2::__macro_helpers::common_selectors::alloc_sel::CACHED_SEL::GENERATED_ID, 0)
	.long	0

.subsections_via_symbols
