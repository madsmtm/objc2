	.syntax	unified
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn1_get_class
	.p2align	2
	.code	32
_fn1_get_class:
	push	{r7, lr}
	mov	r7, sp
	movw	r1, :lower16:(__MergedGlobals-(LPC0_0+8))
	movt	r1, :upper16:(__MergedGlobals-(LPC0_0+8))
LPC0_0:
	add	r1, pc, r1
	ldr	r0, [r1, #12]
	cmp	r0, #0
	popne	{r7, pc}
LBB0_1:
	add	r0, r1, #12
	movw	r1, :lower16:(L_anon.[ID].0-(LPC0_1+8))
	movt	r1, :upper16:(L_anon.[ID].0-(LPC0_1+8))
LPC0_1:
	add	r1, pc, r1
	movw	r2, :lower16:(l_anon.[ID].2-(LPC0_2+8))
	movt	r2, :upper16:(l_anon.[ID].2-(LPC0_2+8))
LPC0_2:
	add	r2, pc, r2
	pop	{r7, lr}
	b	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)

	.globl	_fn1_get_same_class
	.p2align	2
	.code	32
_fn1_get_same_class:
	push	{r7, lr}
	mov	r7, sp
	movw	r0, :lower16:(__MergedGlobals-(LPC1_0+8))
	movt	r0, :upper16:(__MergedGlobals-(LPC1_0+8))
LPC1_0:
	add	r0, pc, r0
	ldr	r0, [r0]
	cmp	r0, #0
	popne	{r7, pc}
LBB1_1:
	movw	r0, :lower16:(__MergedGlobals-(LPC1_1+8))
	movt	r0, :upper16:(__MergedGlobals-(LPC1_1+8))
LPC1_1:
	add	r0, pc, r0
	movw	r1, :lower16:(L_anon.[ID].0-(LPC1_2+8))
	movt	r1, :upper16:(L_anon.[ID].0-(LPC1_2+8))
LPC1_2:
	add	r1, pc, r1
	movw	r2, :lower16:(l_anon.[ID].3-(LPC1_3+8))
	movt	r2, :upper16:(l_anon.[ID].3-(LPC1_3+8))
LPC1_3:
	add	r2, pc, r2
	pop	{r7, lr}
	b	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)

	.globl	_fn3_get_different_class
	.p2align	2
	.code	32
_fn3_get_different_class:
	push	{r7, lr}
	mov	r7, sp
	movw	r1, :lower16:(__MergedGlobals-(LPC2_0+8))
	movt	r1, :upper16:(__MergedGlobals-(LPC2_0+8))
LPC2_0:
	add	r1, pc, r1
	ldr	r0, [r1, #4]
	cmp	r0, #0
	popne	{r7, pc}
LBB2_1:
	add	r0, r1, #4
	movw	r1, :lower16:(L_anon.[ID].4-(LPC2_1+8))
	movt	r1, :upper16:(L_anon.[ID].4-(LPC2_1+8))
LPC2_1:
	add	r1, pc, r1
	movw	r2, :lower16:(l_anon.[ID].5-(LPC2_2+8))
	movt	r2, :upper16:(l_anon.[ID].5-(LPC2_2+8))
LPC2_2:
	add	r2, pc, r2
	pop	{r7, lr}
	b	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)

	.globl	_fn4_unused_class
	.p2align	2
	.code	32
_fn4_unused_class:
	push	{r7, lr}
	mov	r7, sp
	movw	r0, :lower16:(SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0)-(LPC3_0+8))
	movt	r0, :upper16:(SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0)-(LPC3_0+8))
LPC3_0:
	add	r0, pc, r0
	ldr	r0, [r0]
	cmp	r0, #0
	popne	{r7, pc}
LBB3_1:
	movw	r0, :lower16:(SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0)-(LPC3_1+8))
	movt	r0, :upper16:(SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0)-(LPC3_1+8))
LPC3_1:
	add	r0, pc, r0
	movw	r1, :lower16:(L_anon.[ID].6-(LPC3_2+8))
	movt	r1, :upper16:(L_anon.[ID].6-(LPC3_2+8))
LPC3_2:
	add	r1, pc, r1
	movw	r2, :lower16:(l_anon.[ID].7-(LPC3_3+8))
	movt	r2, :upper16:(l_anon.[ID].7-(LPC3_3+8))
LPC3_3:
	add	r2, pc, r2
	pop	{r7, lr}
	b	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)

	.globl	_fn5_use_fns
	.p2align	2
	.code	32
_fn5_use_fns:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8, r10}
	movw	r8, :lower16:(__MergedGlobals-(LPC4_0+8))
	movt	r8, :upper16:(__MergedGlobals-(LPC4_0+8))
LPC4_0:
	add	r8, pc, r8
	ldr	r4, [r8, #12]
	cmp	r4, #0
	beq	LBB4_5
	ldr	r5, [r8]
	cmp	r5, #0
	beq	LBB4_6
LBB4_2:
	ldr	r6, [r8, #4]
	cmp	r6, #0
	beq	LBB4_7
LBB4_3:
	ldr	r1, [r8, #8]
	cmp	r1, #0
	beq	LBB4_8
LBB4_4:
	stm	r0, {r4, r5, r6}
	str	r1, [r0, #12]
	pop	{r8, r10}
	pop	{r4, r5, r6, r7, pc}
LBB4_5:
	add	r3, r8, #12
	movw	r1, :lower16:(L_anon.[ID].0-(LPC4_1+8))
	movt	r1, :upper16:(L_anon.[ID].0-(LPC4_1+8))
LPC4_1:
	add	r1, pc, r1
	movw	r2, :lower16:(l_anon.[ID].2-(LPC4_2+8))
	movt	r2, :upper16:(l_anon.[ID].2-(LPC4_2+8))
LPC4_2:
	add	r2, pc, r2
	mov	r5, r0
	mov	r0, r3
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	r4, r0
	mov	r0, r5
	ldr	r5, [r8]
	cmp	r5, #0
	bne	LBB4_2
LBB4_6:
	movw	r3, :lower16:(__MergedGlobals-(LPC4_3+8))
	movt	r3, :upper16:(__MergedGlobals-(LPC4_3+8))
LPC4_3:
	add	r3, pc, r3
	movw	r1, :lower16:(L_anon.[ID].0-(LPC4_4+8))
	movt	r1, :upper16:(L_anon.[ID].0-(LPC4_4+8))
LPC4_4:
	add	r1, pc, r1
	movw	r2, :lower16:(l_anon.[ID].3-(LPC4_5+8))
	movt	r2, :upper16:(l_anon.[ID].3-(LPC4_5+8))
LPC4_5:
	add	r2, pc, r2
	mov	r6, r0
	mov	r0, r3
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	r5, r0
	mov	r0, r6
	ldr	r6, [r8, #4]
	cmp	r6, #0
	bne	LBB4_3
LBB4_7:
	add	r3, r8, #4
	movw	r1, :lower16:(L_anon.[ID].4-(LPC4_6+8))
	movt	r1, :upper16:(L_anon.[ID].4-(LPC4_6+8))
LPC4_6:
	add	r1, pc, r1
	movw	r2, :lower16:(l_anon.[ID].5-(LPC4_7+8))
	movt	r2, :upper16:(l_anon.[ID].5-(LPC4_7+8))
LPC4_7:
	add	r2, pc, r2
	mov	r10, r0
	mov	r0, r3
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	r6, r0
	mov	r0, r10
	ldr	r1, [r8, #8]
	cmp	r1, #0
	bne	LBB4_4
LBB4_8:
	add	r3, r8, #8
	movw	r1, :lower16:(L_anon.[ID].8-(LPC4_8+8))
	movt	r1, :upper16:(L_anon.[ID].8-(LPC4_8+8))
LPC4_8:
	add	r1, pc, r1
	movw	r2, :lower16:(l_anon.[ID].9-(LPC4_9+8))
	movt	r2, :upper16:(l_anon.[ID].9-(LPC4_9+8))
LPC4_9:
	add	r2, pc, r2
	mov	r8, r0
	mov	r0, r3
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	r1, r0
	mov	r0, r8
	stm	r0, {r4, r5, r6}
	str	r1, [r0, #12]
	pop	{r8, r10}
	pop	{r4, r5, r6, r7, pc}

	.globl	_fn6_use_same_twice
	.p2align	2
	.code	32
_fn6_use_same_twice:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	movw	r5, :lower16:(__MergedGlobals-(LPC5_0+8))
	movt	r5, :upper16:(__MergedGlobals-(LPC5_0+8))
LPC5_0:
	add	r5, pc, r5
	ldr	r4, [r5, #12]
	cmp	r4, #0
	beq	LBB5_3
	ldr	r1, [r5, #12]
	cmp	r1, #0
	beq	LBB5_4
LBB5_2:
	str	r4, [r0]
	str	r1, [r0, #4]
	pop	{r4, r5, r6, r7, pc}
LBB5_3:
	add	r3, r5, #12
	movw	r1, :lower16:(L_anon.[ID].0-(LPC5_1+8))
	movt	r1, :upper16:(L_anon.[ID].0-(LPC5_1+8))
LPC5_1:
	add	r1, pc, r1
	movw	r2, :lower16:(l_anon.[ID].2-(LPC5_2+8))
	movt	r2, :upper16:(l_anon.[ID].2-(LPC5_2+8))
LPC5_2:
	add	r2, pc, r2
	mov	r6, r0
	mov	r0, r3
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	r4, r0
	mov	r0, r6
	ldr	r1, [r5, #12]
	cmp	r1, #0
	bne	LBB5_2
LBB5_4:
	add	r3, r5, #12
	movw	r1, :lower16:(L_anon.[ID].0-(LPC5_3+8))
	movt	r1, :upper16:(L_anon.[ID].0-(LPC5_3+8))
LPC5_3:
	add	r1, pc, r1
	movw	r2, :lower16:(l_anon.[ID].2-(LPC5_4+8))
	movt	r2, :upper16:(l_anon.[ID].2-(LPC5_4+8))
LPC5_4:
	add	r2, pc, r2
	mov	r5, r0
	mov	r0, r3
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	r1, r0
	mov	r0, r5
	str	r4, [r0]
	str	r1, [r0, #4]
	pop	{r4, r5, r6, r7, pc}

	.globl	_fn7_use_in_loop
	.p2align	2
	.code	32
_fn7_use_in_loop:
	cmp	r0, #0
	bxeq	lr
LBB6_1:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8}
	movw	r4, :lower16:(SYM(test_dynamic_class[CRATE_ID]::use_in_loop::CACHED_CLASS, 0)-(LPC6_0+8))
	movt	r4, :upper16:(SYM(test_dynamic_class[CRATE_ID]::use_in_loop::CACHED_CLASS, 0)-(LPC6_0+8))
LPC6_0:
	add	r4, pc, r4
	movw	r5, :lower16:(L_anon.[ID].10-(LPC6_1+8))
	movt	r5, :upper16:(L_anon.[ID].10-(LPC6_1+8))
LPC6_1:
	add	r5, pc, r5
	movw	r6, :lower16:(l_anon.[ID].11-(LPC6_2+8))
	movt	r6, :upper16:(l_anon.[ID].11-(LPC6_2+8))
LPC6_2:
	add	r6, pc, r6
LBB6_2:
	ldr	r1, [r4]
	cmp	r1, #0
	beq	LBB6_4
	subs	r0, r0, #1
	bne	LBB6_2
	b	LBB6_5
LBB6_4:
	mov	r8, r0
	mov	r0, r4
	mov	r1, r5
	mov	r2, r6
	bl	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	r0, r8
	subs	r0, r0, #1
	bne	LBB6_2
LBB6_5:
	pop	{r8}
	pop	{r4, r5, r6, r7, lr}
	bx	lr

.zerofill __DATA,__bss,SYM(test_dynamic_class[CRATE_ID]::use_in_loop::CACHED_CLASS, 0),4,2
.zerofill __DATA,__bss,SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0),4,2
	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].0:
	.asciz	"NSObject"

L_anon.[ID].1:
	.asciz	"crates/$DIR/../test_static_class/lib.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].2:
	.long	L_anon.[ID].1
	.asciz	"J\000\000\000\007\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].3:
	.long	L_anon.[ID].1
	.asciz	"J\000\000\000\f\000\000\000\005\000\000"

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].4:
	.asciz	"NSString"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].5:
	.long	L_anon.[ID].1
	.asciz	"J\000\000\000\021\000\000\000\005\000\000"

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].6:
	.asciz	"NSData"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].7:
	.long	L_anon.[ID].1
	.asciz	"J\000\000\000\026\000\000\000\r\000\000"

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].8:
	.asciz	"NSException"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].9:
	.long	L_anon.[ID].1
	.asciz	"J\000\000\000\036\000\000\000\016\000\000"

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].10:
	.asciz	"NSLock"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].11:
	.long	L_anon.[ID].1
	.asciz	"J\000\000\000,\000\000\000\021\000\000"

.zerofill __DATA,__bss,__MergedGlobals,16,2
.subsections_via_symbols
