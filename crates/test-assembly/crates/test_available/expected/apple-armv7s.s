	.syntax	unified
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn1_always
	.p2align	2
	.code	32
_fn1_always:
	push	{r7, lr}
	mov	r7, sp
	mov	r0, #1
	pop	{r7, pc}

	.globl	_fn2_never
	.p2align	2
	.code	32
_fn2_never:
	push	{r7, lr}
	mov	r7, sp
	mov	r0, #0
	pop	{r7, pc}

	.globl	_fn3_low
	.p2align	2
	.code	32
_fn3_low:
	push	{r7, lr}
	mov	r7, sp
	mov	r0, #1
	pop	{r7, pc}

	.globl	_fn4_high
	.p2align	2
	.code	32
_fn4_high:
	push	{r4, r7, lr}
	add	r7, sp, #4
	movw	r4, :lower16:(LSYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)$non_lazy_ptr-(LPC3_0+8))
	movt	r4, :upper16:(LSYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)$non_lazy_ptr-(LPC3_0+8))
LPC3_0:
	ldr	r4, [pc, r4]
	ldr	r0, [r4]
	cmp	r0, #0
	beq	LBB3_2
LBB3_1:
	lsr	r0, r0, #17
	cmp	r0, #8
	mov	r0, #0
	movwhi	r0, #1
	pop	{r4, r7, pc}
LBB3_2:
	bl	SYM(objc2::__macro_helpers::os_version::apple::lookup_version::GENERATED_ID, 0)
	str	r0, [r4]
	b	LBB3_1

	.globl	_fn5_only_ios
	.p2align	2
	.code	32
_fn5_only_ios:
	push	{r7, lr}
	mov	r7, sp
	mov	r0, #1
	pop	{r7, pc}

	.globl	_fn6_two_checks
	.p2align	2
	.code	32
_fn6_two_checks:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	movw	r5, :lower16:(LSYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)$non_lazy_ptr-(LPC5_0+8))
	movt	r5, :upper16:(LSYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)$non_lazy_ptr-(LPC5_0+8))
LPC5_0:
	ldr	r5, [pc, r5]
	ldr	r4, [r5]
	cmp	r4, #0
	beq	LBB5_3
	ldr	r0, [r5]
	cmp	r0, #0
	beq	LBB5_4
LBB5_2:
	lsr	r1, r4, #16
	cmp	r1, #16
	mov	r1, #0
	mov	r2, #0
	movwhi	r2, #1
	movw	r3, #65535
	movt	r3, #16
	orr	r3, r3, #65536
	cmp	r0, r3
	movwhi	r1, #1
	and	r0, r2, r1
	pop	{r4, r5, r7, pc}
LBB5_3:
	bl	SYM(objc2::__macro_helpers::os_version::apple::lookup_version::GENERATED_ID, 0)
	mov	r4, r0
	str	r0, [r5]
	ldr	r0, [r5]
	cmp	r0, #0
	bne	LBB5_2
LBB5_4:
	bl	SYM(objc2::__macro_helpers::os_version::apple::lookup_version::GENERATED_ID, 0)
	str	r0, [r5]
	b	LBB5_2

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2, 0x0
LSYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)$non_lazy_ptr:
	.indirect_symbol	SYM(objc2::__macro_helpers::os_version::apple::current_version::CURRENT_VERSION::GENERATED_ID, 0)
	.long	0

.subsections_via_symbols
