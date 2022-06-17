	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_get_sel
	.p2align	2
	.code	32
_get_sel:
	push	{r7, lr}
	mov	r7, sp
	bl	__RNvNvCs9WJZU86XVxM_15test_static_sel7get_sel22objc_static_workaround
	pop	{r7, pc}

	.globl	_get_same_sel
	.p2align	2
	.code	32
_get_same_sel:
	push	{r7, lr}
	mov	r7, sp
	bl	__RNvNvCs9WJZU86XVxM_15test_static_sel12get_same_sel22objc_static_workaround
	pop	{r7, pc}

	.globl	_get_common_twice
	.p2align	2
	.code	32
_get_common_twice:
	push	{r4, r7, lr}
	add	r7, sp, #4
	bl	__RNvNvCs9WJZU86XVxM_15test_static_sel16get_common_twice22objc_static_workaround
	mov	r4, r0
	bl	__RNvNvCs9WJZU86XVxM_15test_static_sel16get_common_twices_22objc_static_workaround
	mov	r1, r0
	mov	r0, r4
	pop	{r4, r7, pc}

	.globl	_get_different_sel
	.p2align	2
	.code	32
_get_different_sel:
	push	{r7, lr}
	mov	r7, sp
	bl	__RNvNvCs9WJZU86XVxM_15test_static_sel17get_different_sel22objc_static_workaround
	pop	{r7, pc}

	.globl	_unused_sel
	.p2align	2
	.code	32
_unused_sel:
	push	{r7, lr}
	mov	r7, sp
	bl	__RNvNvCs9WJZU86XVxM_15test_static_sel10unused_sel22objc_static_workaround
	pop	{r7, pc}

	.globl	_use_fns
	.p2align	2
	.code	32
_use_fns:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8}
	mov	r4, r0
	bl	__RNvNvCs9WJZU86XVxM_15test_static_sel7get_sel22objc_static_workaround
	mov	r8, r0
	bl	__RNvNvCs9WJZU86XVxM_15test_static_sel12get_same_sel22objc_static_workaround
	mov	r6, r0
	bl	__RNvNvCs9WJZU86XVxM_15test_static_sel17get_different_sel22objc_static_workaround
	mov	r5, r0
	bl	__RNvNvCs9WJZU86XVxM_15test_static_sel7use_fns22objc_static_workaround
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
	bl	__RNvNvCs9WJZU86XVxM_15test_static_sel7get_sel22objc_static_workaround
	mov	r8, r0
	bl	__RNvNvCs9WJZU86XVxM_15test_static_sel7get_sel22objc_static_workaround
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
	bl	__RNvNvCs9WJZU86XVxM_15test_static_sel11use_in_loop22objc_static_workaround
	subs	r4, r4, #1
	bne	LBB7_2
	pop	{r4, r7, pc}

	.p2align	2
	.code	32
__RNvNvCs9WJZU86XVxM_15test_static_sel7get_sel22objc_static_workaround:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9-(LPC8_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9-(LPC8_0+8))
LPC8_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__RNvNvCs9WJZU86XVxM_15test_static_sel12get_same_sel22objc_static_workaround:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35-(LPC9_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35-(LPC9_0+8))
LPC9_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__RNvNvCs9WJZU86XVxM_15test_static_sel16get_common_twice22objc_static_workaround:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_e4a45d49bfea5d77-(LPC10_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_e4a45d49bfea5d77-(LPC10_0+8))
LPC10_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__RNvNvCs9WJZU86XVxM_15test_static_sel16get_common_twices_22objc_static_workaround:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_33db9f67352fe9a7-(LPC11_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_33db9f67352fe9a7-(LPC11_0+8))
LPC11_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__RNvNvCs9WJZU86XVxM_15test_static_sel17get_different_sel22objc_static_workaround:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0-(LPC12_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0-(LPC12_0+8))
LPC12_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__RNvNvCs9WJZU86XVxM_15test_static_sel10unused_sel22objc_static_workaround:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_2c505e110d181b25-(LPC13_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_2c505e110d181b25-(LPC13_0+8))
LPC13_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__RNvNvCs9WJZU86XVxM_15test_static_sel7use_fns22objc_static_workaround:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99-(LPC14_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99-(LPC14_0+8))
LPC14_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__RNvNvCs9WJZU86XVxM_15test_static_sel11use_in_loop22objc_static_workaround:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_9845965b987ed54b-(LPC15_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_9845965b987ed54b-(LPC15_0+8))
LPC15_0:
	ldr	r0, [pc, r0]
	bx	lr

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_68381ba894e318e9
	.p2align	2
L_OBJC_IMAGE_INFO_68381ba894e318e9:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_68381ba894e318e9
L_OBJC_METH_VAR_NAME_68381ba894e318e9:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9:
	.long	L_OBJC_METH_VAR_NAME_68381ba894e318e9

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_cd2fd6e7d2adcc35
	.p2align	2
L_OBJC_IMAGE_INFO_cd2fd6e7d2adcc35:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35
L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35:
	.long	L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_e4a45d49bfea5d77
	.p2align	2
L_OBJC_IMAGE_INFO_e4a45d49bfea5d77:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e4a45d49bfea5d77
L_OBJC_METH_VAR_NAME_e4a45d49bfea5d77:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_e4a45d49bfea5d77
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_e4a45d49bfea5d77:
	.long	L_OBJC_METH_VAR_NAME_e4a45d49bfea5d77

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_33db9f67352fe9a7
	.p2align	2
L_OBJC_IMAGE_INFO_33db9f67352fe9a7:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_33db9f67352fe9a7
L_OBJC_METH_VAR_NAME_33db9f67352fe9a7:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_33db9f67352fe9a7
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_33db9f67352fe9a7:
	.long	L_OBJC_METH_VAR_NAME_33db9f67352fe9a7

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_bb37877368f0b7a0
	.p2align	2
L_OBJC_IMAGE_INFO_bb37877368f0b7a0:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_bb37877368f0b7a0
L_OBJC_METH_VAR_NAME_bb37877368f0b7a0:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0:
	.long	L_OBJC_METH_VAR_NAME_bb37877368f0b7a0

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2c505e110d181b25
	.p2align	2
L_OBJC_IMAGE_INFO_2c505e110d181b25:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2c505e110d181b25
L_OBJC_METH_VAR_NAME_2c505e110d181b25:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_2c505e110d181b25
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_2c505e110d181b25:
	.long	L_OBJC_METH_VAR_NAME_2c505e110d181b25

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_5419c3f7fc0a6f99
	.p2align	2
L_OBJC_IMAGE_INFO_5419c3f7fc0a6f99:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_5419c3f7fc0a6f99
L_OBJC_METH_VAR_NAME_5419c3f7fc0a6f99:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99:
	.long	L_OBJC_METH_VAR_NAME_5419c3f7fc0a6f99

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_9845965b987ed54b
	.p2align	2
L_OBJC_IMAGE_INFO_9845965b987ed54b:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_9845965b987ed54b
L_OBJC_METH_VAR_NAME_9845965b987ed54b:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_9845965b987ed54b
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_9845965b987ed54b:
	.long	L_OBJC_METH_VAR_NAME_9845965b987ed54b

.subsections_via_symbols
