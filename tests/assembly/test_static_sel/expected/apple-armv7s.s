	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_get_sel
	.p2align	2
	.code	32
_get_sel:
	push	{r7, lr}
	mov	r7, sp
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17h727796eb777ca212E
	pop	{r7, pc}

	.globl	_get_same_sel
	.p2align	2
	.code	32
_get_same_sel:
	push	{r7, lr}
	mov	r7, sp
	bl	__ZN15test_static_sel12get_same_sel22objc_static_workaround17h05bab5e52e062a47E
	pop	{r7, pc}

	.globl	_get_common
	.p2align	2
	.code	32
_get_common:
	push	{r7, lr}
	mov	r7, sp
	bl	__ZN15test_static_sel10get_common22objc_static_workaround17hd33621f07152712dE
	pop	{r7, pc}

	.globl	_get_different_sel
	.p2align	2
	.code	32
_get_different_sel:
	push	{r7, lr}
	mov	r7, sp
	bl	__ZN15test_static_sel17get_different_sel22objc_static_workaround17h20e00ed4cf62ec6cE
	pop	{r7, pc}

	.globl	_unused_sel
	.p2align	2
	.code	32
_unused_sel:
	push	{r7, lr}
	mov	r7, sp
	bl	__ZN15test_static_sel10unused_sel22objc_static_workaround17hd8ba138112e5d612E
	pop	{r7, pc}

	.globl	_use_fns
	.p2align	2
	.code	32
_use_fns:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8}
	mov	r4, r0
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17h727796eb777ca212E
	mov	r8, r0
	bl	__ZN15test_static_sel12get_same_sel22objc_static_workaround17h05bab5e52e062a47E
	mov	r6, r0
	bl	__ZN15test_static_sel17get_different_sel22objc_static_workaround17h20e00ed4cf62ec6cE
	mov	r5, r0
	bl	__ZN15test_static_sel7use_fns22objc_static_workaround17h613888fa0e7e36bdE
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
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17h727796eb777ca212E
	mov	r8, r0
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17h727796eb777ca212E
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
	bl	__ZN15test_static_sel11use_in_loop22objc_static_workaround17h806742d54569416eE
	subs	r4, r4, #1
	bne	LBB7_2
	pop	{r4, r7, pc}

	.p2align	2
	.code	32
__ZN15test_static_sel7get_sel22objc_static_workaround17h727796eb777ca212E:
	movw	r0, :lower16:(__ZN15test_static_sel7get_sel22objc_static_workaround3REF17hefeb1efa701445a4E-(LPC8_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel7get_sel22objc_static_workaround3REF17hefeb1efa701445a4E-(LPC8_0+8))
LPC8_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel12get_same_sel22objc_static_workaround17h05bab5e52e062a47E:
	movw	r0, :lower16:(__ZN15test_static_sel12get_same_sel22objc_static_workaround3REF17hc58f200ac0b74570E-(LPC9_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel12get_same_sel22objc_static_workaround3REF17hc58f200ac0b74570E-(LPC9_0+8))
LPC9_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel10get_common22objc_static_workaround17hd33621f07152712dE:
	movw	r0, :lower16:(__ZN15test_static_sel10get_common22objc_static_workaround3REF17h11bf08c1651e11acE-(LPC10_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel10get_common22objc_static_workaround3REF17h11bf08c1651e11acE-(LPC10_0+8))
LPC10_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel17get_different_sel22objc_static_workaround17h20e00ed4cf62ec6cE:
	movw	r0, :lower16:(__ZN15test_static_sel17get_different_sel22objc_static_workaround3REF17h31f43ec056144797E-(LPC11_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel17get_different_sel22objc_static_workaround3REF17h31f43ec056144797E-(LPC11_0+8))
LPC11_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel10unused_sel22objc_static_workaround17hd8ba138112e5d612E:
	movw	r0, :lower16:(__ZN15test_static_sel10unused_sel22objc_static_workaround3REF17hbfa5418e3430bd57E-(LPC12_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel10unused_sel22objc_static_workaround3REF17hbfa5418e3430bd57E-(LPC12_0+8))
LPC12_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel7use_fns22objc_static_workaround17h613888fa0e7e36bdE:
	movw	r0, :lower16:(__ZN15test_static_sel7use_fns22objc_static_workaround3REF17hd98abf55861d9c7eE-(LPC13_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel7use_fns22objc_static_workaround3REF17hd98abf55861d9c7eE-(LPC13_0+8))
LPC13_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel11use_in_loop22objc_static_workaround17h806742d54569416eE:
	movw	r0, :lower16:(__ZN15test_static_sel11use_in_loop22objc_static_workaround3REF17h3be3974baf782dd5E-(LPC14_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel11use_in_loop22objc_static_workaround3REF17h3be3974baf782dd5E-(LPC14_0+8))
LPC14_0:
	ldr	r0, [pc, r0]
	bx	lr

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7get_sel22objc_static_workaround4NAME17h15c0f2d5bb72484eE:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel7get_sel22objc_static_workaround3REF17hefeb1efa701445a4E:
	.long	__ZN15test_static_sel7get_sel22objc_static_workaround4NAME17h15c0f2d5bb72484eE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel12get_same_sel22objc_static_workaround4NAME17h9742a2b7783b7cf1E:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel12get_same_sel22objc_static_workaround3REF17hc58f200ac0b74570E:
	.long	__ZN15test_static_sel12get_same_sel22objc_static_workaround4NAME17h9742a2b7783b7cf1E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel10get_common22objc_static_workaround4NAME17h5666fd5c6ed96293E:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel10get_common22objc_static_workaround3REF17h11bf08c1651e11acE:
	.long	__ZN15test_static_sel10get_common22objc_static_workaround4NAME17h5666fd5c6ed96293E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel17get_different_sel22objc_static_workaround4NAME17hd7e073585b5ca385E:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel17get_different_sel22objc_static_workaround3REF17h31f43ec056144797E:
	.long	__ZN15test_static_sel17get_different_sel22objc_static_workaround4NAME17hd7e073585b5ca385E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel10unused_sel22objc_static_workaround4NAME17h57ecf7de5ba63d5dE:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel10unused_sel22objc_static_workaround3REF17hbfa5418e3430bd57E:
	.long	__ZN15test_static_sel10unused_sel22objc_static_workaround4NAME17h57ecf7de5ba63d5dE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7use_fns22objc_static_workaround4NAME17h9496e7395093f09bE:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel7use_fns22objc_static_workaround3REF17hd98abf55861d9c7eE:
	.long	__ZN15test_static_sel7use_fns22objc_static_workaround4NAME17h9496e7395093f09bE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel11use_in_loop22objc_static_workaround4NAME17hf5353e0a47539e1dE:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel11use_in_loop22objc_static_workaround3REF17h3be3974baf782dd5E:
	.long	__ZN15test_static_sel11use_in_loop22objc_static_workaround4NAME17hf5353e0a47539e1dE

.subsections_via_symbols
