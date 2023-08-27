	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_get_class
	.p2align	2
	.code	32
_get_class:
	movw	r0, :lower16:(__MergedGlobals-(LPC0_0+8))
	movt	r0, :upper16:(__MergedGlobals-(LPC0_0+8))
LPC0_0:
	add	r0, pc, r0
	ldr	r0, [r0]
	cmp	r0, #0
	bxne	lr
LBB0_1:
	push	{r7, lr}
	mov	r7, sp
	movw	r0, :lower16:(__MergedGlobals-(LPC0_1+8))
	movt	r0, :upper16:(__MergedGlobals-(LPC0_1+8))
	movw	r1, :lower16:(l_anon.[ID].0-(LPC0_2+8))
	movt	r1, :upper16:(l_anon.[ID].0-(LPC0_2+8))
	movw	r2, :lower16:(l_anon.[ID].2-(LPC0_3+8))
	movt	r2, :upper16:(l_anon.[ID].2-(LPC0_3+8))
LPC0_1:
	add	r0, pc, r0
LPC0_2:
	add	r1, pc, r1
LPC0_3:
	add	r2, pc, r2
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	pop	{r7, lr}
	bx	lr

	.globl	_get_same_class
	.p2align	2
	.code	32
_get_same_class:
	movw	r3, :lower16:(__MergedGlobals-(LPC1_0+8))
	movt	r3, :upper16:(__MergedGlobals-(LPC1_0+8))
LPC1_0:
	add	r3, pc, r3
	ldr	r0, [r3, #4]
	cmp	r0, #0
	bxne	lr
LBB1_1:
	push	{r7, lr}
	mov	r7, sp
	movw	r1, :lower16:(l_anon.[ID].0-(LPC1_1+8))
	add	r0, r3, #4
	movt	r1, :upper16:(l_anon.[ID].0-(LPC1_1+8))
	movw	r2, :lower16:(l_anon.[ID].3-(LPC1_2+8))
	movt	r2, :upper16:(l_anon.[ID].3-(LPC1_2+8))
LPC1_1:
	add	r1, pc, r1
LPC1_2:
	add	r2, pc, r2
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	pop	{r7, lr}
	bx	lr

	.globl	_get_different_class
	.p2align	2
	.code	32
_get_different_class:
	movw	r3, :lower16:(__MergedGlobals-(LPC2_0+8))
	movt	r3, :upper16:(__MergedGlobals-(LPC2_0+8))
LPC2_0:
	add	r3, pc, r3
	ldr	r0, [r3, #8]
	cmp	r0, #0
	bxne	lr
LBB2_1:
	push	{r7, lr}
	mov	r7, sp
	movw	r1, :lower16:(l_anon.[ID].4-(LPC2_1+8))
	add	r0, r3, #8
	movt	r1, :upper16:(l_anon.[ID].4-(LPC2_1+8))
	movw	r2, :lower16:(l_anon.[ID].5-(LPC2_2+8))
	movt	r2, :upper16:(l_anon.[ID].5-(LPC2_2+8))
LPC2_1:
	add	r1, pc, r1
LPC2_2:
	add	r2, pc, r2
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	pop	{r7, lr}
	bx	lr

	.globl	_unused_class
	.p2align	2
	.code	32
_unused_class:
	movw	r0, :lower16:(SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0)-(LPC3_0+8))
	movt	r0, :upper16:(SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0)-(LPC3_0+8))
LPC3_0:
	add	r0, pc, r0
	ldr	r0, [r0]
	cmp	r0, #0
	bxne	lr
LBB3_1:
	push	{r7, lr}
	mov	r7, sp
	movw	r0, :lower16:(SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0)-(LPC3_1+8))
	movt	r0, :upper16:(SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0)-(LPC3_1+8))
	movw	r1, :lower16:(l_anon.[ID].6-(LPC3_2+8))
	movt	r1, :upper16:(l_anon.[ID].6-(LPC3_2+8))
	movw	r2, :lower16:(l_anon.[ID].7-(LPC3_3+8))
	movt	r2, :upper16:(l_anon.[ID].7-(LPC3_3+8))
LPC3_1:
	add	r0, pc, r0
LPC3_2:
	add	r1, pc, r1
LPC3_3:
	add	r2, pc, r2
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	pop	{r7, lr}
	bx	lr

	.globl	_use_fns
	.p2align	2
	.code	32
_use_fns:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10}
	movw	r10, :lower16:(__MergedGlobals-(LPC4_0+8))
	mov	r4, r0
	movt	r10, :upper16:(__MergedGlobals-(LPC4_0+8))
LPC4_0:
	add	r10, pc, r10
	ldr	r8, [r10]
	cmp	r8, #0
	beq	LBB4_5
	ldr	r6, [r10, #4]
	cmp	r6, #0
	beq	LBB4_6
LBB4_2:
	ldr	r5, [r10, #8]
	cmp	r5, #0
	beq	LBB4_7
LBB4_3:
	ldr	r0, [r10, #12]
	cmp	r0, #0
	beq	LBB4_8
LBB4_4:
	str	r8, [r4]
	str	r6, [r4, #4]
	str	r5, [r4, #8]
	str	r0, [r4, #12]
	pop	{r8, r10}
	pop	{r4, r5, r6, r7, pc}
LBB4_5:
	movw	r0, :lower16:(__MergedGlobals-(LPC4_1+8))
	movt	r0, :upper16:(__MergedGlobals-(LPC4_1+8))
	movw	r1, :lower16:(l_anon.[ID].0-(LPC4_2+8))
	movt	r1, :upper16:(l_anon.[ID].0-(LPC4_2+8))
	movw	r2, :lower16:(l_anon.[ID].2-(LPC4_3+8))
	movt	r2, :upper16:(l_anon.[ID].2-(LPC4_3+8))
LPC4_1:
	add	r0, pc, r0
LPC4_2:
	add	r1, pc, r1
LPC4_3:
	add	r2, pc, r2
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	r8, r0
	ldr	r6, [r10, #4]
	cmp	r6, #0
	bne	LBB4_2
LBB4_6:
	movw	r1, :lower16:(l_anon.[ID].0-(LPC4_4+8))
	add	r0, r10, #4
	movt	r1, :upper16:(l_anon.[ID].0-(LPC4_4+8))
	movw	r2, :lower16:(l_anon.[ID].3-(LPC4_5+8))
	movt	r2, :upper16:(l_anon.[ID].3-(LPC4_5+8))
LPC4_4:
	add	r1, pc, r1
LPC4_5:
	add	r2, pc, r2
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	r6, r0
	ldr	r5, [r10, #8]
	cmp	r5, #0
	bne	LBB4_3
LBB4_7:
	movw	r1, :lower16:(l_anon.[ID].4-(LPC4_6+8))
	add	r0, r10, #8
	movt	r1, :upper16:(l_anon.[ID].4-(LPC4_6+8))
	movw	r2, :lower16:(l_anon.[ID].5-(LPC4_7+8))
	movt	r2, :upper16:(l_anon.[ID].5-(LPC4_7+8))
LPC4_6:
	add	r1, pc, r1
LPC4_7:
	add	r2, pc, r2
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	r5, r0
	ldr	r0, [r10, #12]
	cmp	r0, #0
	bne	LBB4_4
LBB4_8:
	movw	r1, :lower16:(l_anon.[ID].8-(LPC4_8+8))
	add	r0, r10, #12
	movt	r1, :upper16:(l_anon.[ID].8-(LPC4_8+8))
	movw	r2, :lower16:(l_anon.[ID].9-(LPC4_9+8))
	movt	r2, :upper16:(l_anon.[ID].9-(LPC4_9+8))
LPC4_8:
	add	r1, pc, r1
LPC4_9:
	add	r2, pc, r2
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	b	LBB4_4

	.globl	_use_same_twice
	.p2align	2
	.code	32
_use_same_twice:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	push	{r8}
	movw	r5, :lower16:(__MergedGlobals-(LPC5_0+8))
	mov	r4, r0
	movt	r5, :upper16:(__MergedGlobals-(LPC5_0+8))
LPC5_0:
	add	r5, pc, r5
	ldr	r8, [r5]
	cmp	r8, #0
	beq	LBB5_3
	ldr	r9, [r5]
	cmp	r9, #0
	beq	LBB5_4
LBB5_2:
	strd	r8, r9, [r4]
	pop	{r8}
	pop	{r4, r5, r7, pc}
LBB5_3:
	movw	r0, :lower16:(__MergedGlobals-(LPC5_1+8))
	movt	r0, :upper16:(__MergedGlobals-(LPC5_1+8))
	movw	r1, :lower16:(l_anon.[ID].0-(LPC5_2+8))
	movt	r1, :upper16:(l_anon.[ID].0-(LPC5_2+8))
	movw	r2, :lower16:(l_anon.[ID].2-(LPC5_3+8))
	movt	r2, :upper16:(l_anon.[ID].2-(LPC5_3+8))
LPC5_1:
	add	r0, pc, r0
LPC5_2:
	add	r1, pc, r1
LPC5_3:
	add	r2, pc, r2
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	r8, r0
	ldr	r9, [r5]
	cmp	r9, #0
	bne	LBB5_2
LBB5_4:
	movw	r0, :lower16:(__MergedGlobals-(LPC5_4+8))
	movt	r0, :upper16:(__MergedGlobals-(LPC5_4+8))
	movw	r1, :lower16:(l_anon.[ID].0-(LPC5_5+8))
	movt	r1, :upper16:(l_anon.[ID].0-(LPC5_5+8))
	movw	r2, :lower16:(l_anon.[ID].2-(LPC5_6+8))
	movt	r2, :upper16:(l_anon.[ID].2-(LPC5_6+8))
LPC5_4:
	add	r0, pc, r0
LPC5_5:
	add	r1, pc, r1
LPC5_6:
	add	r2, pc, r2
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	r9, r0
	strd	r8, r9, [r4]
	pop	{r8}
	pop	{r4, r5, r7, pc}

	.globl	_use_in_loop
	.p2align	2
	.code	32
_use_in_loop:
	cmp	r0, #0
	bxeq	lr
LBB6_1:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8}
	movw	r5, :lower16:(SYM(test_dynamic_class[CRATE_ID]::use_in_loop::CACHED_CLASS, 0)-(LPC6_0+8))
	mov	r4, r0
	movt	r5, :upper16:(SYM(test_dynamic_class[CRATE_ID]::use_in_loop::CACHED_CLASS, 0)-(LPC6_0+8))
	movw	r8, :lower16:(l_anon.[ID].10-(LPC6_1+8))
	movt	r8, :upper16:(l_anon.[ID].10-(LPC6_1+8))
	movw	r6, :lower16:(l_anon.[ID].11-(LPC6_2+8))
	movt	r6, :upper16:(l_anon.[ID].11-(LPC6_2+8))
LPC6_0:
	add	r5, pc, r5
LPC6_1:
	add	r8, pc, r8
LPC6_2:
	add	r6, pc, r6
	b	LBB6_3
LBB6_2:
	subs	r4, r4, #1
	beq	LBB6_5
LBB6_3:
	ldr	r0, [r5]
	cmp	r0, #0
	bne	LBB6_2
	mov	r0, r5
	mov	r1, r8
	mov	r2, r6
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	b	LBB6_2
LBB6_5:
	pop	{r8}
	pop	{r4, r5, r6, r7, lr}
	bx	lr

	.section	__TEXT,__const
l_anon.[ID].0:
	.asciz	"NSObject"

l_anon.[ID].1:
	.ascii	"crates/$DIR/../test_static_class/lib.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].2:
	.long	l_anon.[ID].1
	.asciz	"J\000\000\000\b\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].3:
	.long	l_anon.[ID].1
	.asciz	"J\000\000\000\r\000\000\000\005\000\000"

	.section	__TEXT,__const
l_anon.[ID].4:
	.asciz	"NSString"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].5:
	.long	l_anon.[ID].1
	.asciz	"J\000\000\000\022\000\000\000\005\000\000"

	.section	__TEXT,__const
l_anon.[ID].6:
	.asciz	"NSData"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].7:
	.long	l_anon.[ID].1
	.asciz	"J\000\000\000\027\000\000\000\r\000\000"

	.section	__TEXT,__const
l_anon.[ID].8:
	.asciz	"NSException"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].9:
	.long	l_anon.[ID].1
	.asciz	"J\000\000\000\037\000\000\000\016\000\000"

	.section	__TEXT,__const
l_anon.[ID].10:
	.asciz	"NSLock"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].11:
	.long	l_anon.[ID].1
	.asciz	"J\000\000\000-\000\000\000\021\000\000"

.zerofill __DATA,__bss,SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0),4,2
.zerofill __DATA,__bss,SYM(test_dynamic_class[CRATE_ID]::use_in_loop::CACHED_CLASS, 0),4,2
.zerofill __DATA,__bss,__MergedGlobals,16,2
.subsections_via_symbols
