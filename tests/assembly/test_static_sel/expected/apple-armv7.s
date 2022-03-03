	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_get_sel
	.p2align	2
	.code	32
_get_sel:
	b	__ZN15test_static_sel7get_sel22objc_static_workaround17h72e7bc422453dd4bE

	.globl	_get_same_sel
	.p2align	2
	.code	32
_get_same_sel:
	b	__ZN15test_static_sel12get_same_sel22objc_static_workaround17hd2469b9e7fb1c0f2E

	.globl	_get_common
	.p2align	2
	.code	32
_get_common:
	b	__ZN15test_static_sel10get_common22objc_static_workaround17h3f794ff9f42acf9eE

	.globl	_get_different_sel
	.p2align	2
	.code	32
_get_different_sel:
	b	__ZN15test_static_sel17get_different_sel22objc_static_workaround17h01062977b636a32dE

	.globl	_unused_sel
	.p2align	2
	.code	32
_unused_sel:
	b	__ZN15test_static_sel10unused_sel22objc_static_workaround17h6da711f1e3b49352E

	.globl	_use_fns
	.p2align	2
	.code	32
_use_fns:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8}
	mov	r4, r0
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17h72e7bc422453dd4bE
	mov	r8, r0
	bl	__ZN15test_static_sel12get_same_sel22objc_static_workaround17hd2469b9e7fb1c0f2E
	mov	r6, r0
	bl	__ZN15test_static_sel17get_different_sel22objc_static_workaround17h01062977b636a32dE
	mov	r5, r0
	bl	__ZN15test_static_sel7use_fns22objc_static_workaround17h0114609dfd50ccc2E
	str	r8, [r4]
	str	r6, [r4, #4]
	str	r5, [r4, #8]
	str	r0, [r4, #12]
	pop	{r8}
	pop	{r4, r5, r6, r7, pc}

	.globl	_use_same_twice
	.p2align	2
	.code	32
_use_same_twice:
	push	{r4, r7, lr}
	add	r7, sp, #4
	push	{r8}
	mov	r4, r0
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17h72e7bc422453dd4bE
	mov	r8, r0
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17h72e7bc422453dd4bE
	mov	r9, r0
	strd	r8, r9, [r4]
	pop	{r8}
	pop	{r4, r7, pc}

	.globl	_use_in_loop
	.p2align	2
	.code	32
_use_in_loop:
	push	{r4, r7, lr}
	add	r7, sp, #4
	cmp	r0, #0
	popeq	{r4, r7, pc}
LBB7_1:
	mov	r4, r0
LBB7_2:
	bl	__ZN15test_static_sel11use_in_loop22objc_static_workaround17hfddcd2cc792b0ea4E
	subs	r4, r4, #1
	bne	LBB7_2
	pop	{r4, r7, pc}

	.p2align	2
	.code	32
__ZN15test_static_sel7get_sel22objc_static_workaround17h72e7bc422453dd4bE:
	movw	r0, :lower16:(__ZN15test_static_sel7get_sel22objc_static_workaround3REF17haed2d29deaa594a5E-(LPC8_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel7get_sel22objc_static_workaround3REF17haed2d29deaa594a5E-(LPC8_0+8))
LPC8_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel12get_same_sel22objc_static_workaround17hd2469b9e7fb1c0f2E:
	movw	r0, :lower16:(__ZN15test_static_sel12get_same_sel22objc_static_workaround3REF17ha96ad30cde590111E-(LPC9_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel12get_same_sel22objc_static_workaround3REF17ha96ad30cde590111E-(LPC9_0+8))
LPC9_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel10get_common22objc_static_workaround17h3f794ff9f42acf9eE:
	movw	r0, :lower16:(__ZN15test_static_sel10get_common22objc_static_workaround3REF17h9dd8d669df75f145E-(LPC10_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel10get_common22objc_static_workaround3REF17h9dd8d669df75f145E-(LPC10_0+8))
LPC10_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel17get_different_sel22objc_static_workaround17h01062977b636a32dE:
	movw	r0, :lower16:(__ZN15test_static_sel17get_different_sel22objc_static_workaround3REF17hfbfc6a5f52210736E-(LPC11_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel17get_different_sel22objc_static_workaround3REF17hfbfc6a5f52210736E-(LPC11_0+8))
LPC11_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel10unused_sel22objc_static_workaround17h6da711f1e3b49352E:
	movw	r0, :lower16:(__ZN15test_static_sel10unused_sel22objc_static_workaround3REF17h3f6ed3f067235ff3E-(LPC12_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel10unused_sel22objc_static_workaround3REF17h3f6ed3f067235ff3E-(LPC12_0+8))
LPC12_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel7use_fns22objc_static_workaround17h0114609dfd50ccc2E:
	movw	r0, :lower16:(__ZN15test_static_sel7use_fns22objc_static_workaround3REF17hd9b45383a5f14d7fE-(LPC13_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel7use_fns22objc_static_workaround3REF17hd9b45383a5f14d7fE-(LPC13_0+8))
LPC13_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel11use_in_loop22objc_static_workaround17hfddcd2cc792b0ea4E:
	movw	r0, :lower16:(__ZN15test_static_sel11use_in_loop22objc_static_workaround3REF17h4273a0a2541d4d90E-(LPC14_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel11use_in_loop22objc_static_workaround3REF17h4273a0a2541d4d90E-(LPC14_0+8))
LPC14_0:
	ldr	r0, [pc, r0]
	bx	lr

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7get_sel22objc_static_workaround4NAME17ha3f8077c962753c6E:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel7get_sel22objc_static_workaround3REF17haed2d29deaa594a5E:
	.long	__ZN15test_static_sel7get_sel22objc_static_workaround4NAME17ha3f8077c962753c6E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel12get_same_sel22objc_static_workaround4NAME17h414b71130d291bcbE:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel12get_same_sel22objc_static_workaround3REF17ha96ad30cde590111E:
	.long	__ZN15test_static_sel12get_same_sel22objc_static_workaround4NAME17h414b71130d291bcbE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel10get_common22objc_static_workaround4NAME17h59407464b3ae7e14E:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel10get_common22objc_static_workaround3REF17h9dd8d669df75f145E:
	.long	__ZN15test_static_sel10get_common22objc_static_workaround4NAME17h59407464b3ae7e14E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel17get_different_sel22objc_static_workaround4NAME17hf259bc1c6d7a8474E:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel17get_different_sel22objc_static_workaround3REF17hfbfc6a5f52210736E:
	.long	__ZN15test_static_sel17get_different_sel22objc_static_workaround4NAME17hf259bc1c6d7a8474E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel10unused_sel22objc_static_workaround4NAME17h524d9abe91019351E:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel10unused_sel22objc_static_workaround3REF17h3f6ed3f067235ff3E:
	.long	__ZN15test_static_sel10unused_sel22objc_static_workaround4NAME17h524d9abe91019351E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7use_fns22objc_static_workaround4NAME17he05a8a8a090de466E:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel7use_fns22objc_static_workaround3REF17hd9b45383a5f14d7fE:
	.long	__ZN15test_static_sel7use_fns22objc_static_workaround4NAME17he05a8a8a090de466E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel11use_in_loop22objc_static_workaround4NAME17hd8d7210562c66839E:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel11use_in_loop22objc_static_workaround3REF17h4273a0a2541d4d90E:
	.long	__ZN15test_static_sel11use_in_loop22objc_static_workaround4NAME17hd8d7210562c66839E

.subsections_via_symbols
