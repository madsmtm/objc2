	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_get_sel
	.p2align	2
	.code	32
_get_sel:
	b	__ZN15test_static_sel7get_sel22objc_static_workaround17hb4f8ebf12620f024E

	.globl	_get_same_sel
	.p2align	2
	.code	32
_get_same_sel:
	b	__ZN15test_static_sel12get_same_sel22objc_static_workaround17ha0db901a0e219311E

	.globl	_get_common
	.p2align	2
	.code	32
_get_common:
	b	__ZN15test_static_sel10get_common22objc_static_workaround17h8e0d36867c2deb3fE

	.globl	_get_different_sel
	.p2align	2
	.code	32
_get_different_sel:
	b	__ZN15test_static_sel17get_different_sel22objc_static_workaround17hac89f9a0e9ae4aafE

	.globl	_unused_sel
	.p2align	2
	.code	32
_unused_sel:
	b	__ZN15test_static_sel10unused_sel22objc_static_workaround17h7e5361a34aa0b8f3E

	.globl	_use_fns
	.p2align	2
	.code	32
_use_fns:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8}
	mov	r4, r0
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17hb4f8ebf12620f024E
	mov	r8, r0
	bl	__ZN15test_static_sel12get_same_sel22objc_static_workaround17ha0db901a0e219311E
	mov	r6, r0
	bl	__ZN15test_static_sel17get_different_sel22objc_static_workaround17hac89f9a0e9ae4aafE
	mov	r5, r0
	bl	__ZN15test_static_sel7use_fns22objc_static_workaround17h8513ca3de8f36ba6E
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
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17hb4f8ebf12620f024E
	mov	r8, r0
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17hb4f8ebf12620f024E
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
	bl	__ZN15test_static_sel11use_in_loop22objc_static_workaround17h0b1654ea6eda9039E
	subs	r4, r4, #1
	bne	LBB7_2
	pop	{r4, r7, pc}

	.p2align	2
	.code	32
__ZN15test_static_sel7get_sel22objc_static_workaround17hb4f8ebf12620f024E:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9-(LPC8_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9-(LPC8_0+8))
LPC8_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel12get_same_sel22objc_static_workaround17ha0db901a0e219311E:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35-(LPC9_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35-(LPC9_0+8))
LPC9_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel10get_common22objc_static_workaround17h8e0d36867c2deb3fE:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_34d6c3ed70e85964-(LPC10_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_34d6c3ed70e85964-(LPC10_0+8))
LPC10_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel17get_different_sel22objc_static_workaround17hac89f9a0e9ae4aafE:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_ab5e106a55f71e5b-(LPC11_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_ab5e106a55f71e5b-(LPC11_0+8))
LPC11_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel10unused_sel22objc_static_workaround17h7e5361a34aa0b8f3E:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_6f2d5ee51a69c477-(LPC12_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_6f2d5ee51a69c477-(LPC12_0+8))
LPC12_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel7use_fns22objc_static_workaround17h8513ca3de8f36ba6E:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_1d27e854714b8860-(LPC13_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_1d27e854714b8860-(LPC13_0+8))
LPC13_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel11use_in_loop22objc_static_workaround17h0b1654ea6eda9039E:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_e56637a4c1a15b07-(LPC14_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_e56637a4c1a15b07-(LPC14_0+8))
LPC14_0:
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
	.globl	L_OBJC_IMAGE_INFO_34d6c3ed70e85964
	.p2align	2
L_OBJC_IMAGE_INFO_34d6c3ed70e85964:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_34d6c3ed70e85964
L_OBJC_METH_VAR_NAME_34d6c3ed70e85964:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_34d6c3ed70e85964
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_34d6c3ed70e85964:
	.long	L_OBJC_METH_VAR_NAME_34d6c3ed70e85964

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_ab5e106a55f71e5b
	.p2align	2
L_OBJC_IMAGE_INFO_ab5e106a55f71e5b:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_ab5e106a55f71e5b
L_OBJC_METH_VAR_NAME_ab5e106a55f71e5b:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_ab5e106a55f71e5b
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_ab5e106a55f71e5b:
	.long	L_OBJC_METH_VAR_NAME_ab5e106a55f71e5b

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_6f2d5ee51a69c477
	.p2align	2
L_OBJC_IMAGE_INFO_6f2d5ee51a69c477:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6f2d5ee51a69c477
L_OBJC_METH_VAR_NAME_6f2d5ee51a69c477:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_6f2d5ee51a69c477
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_6f2d5ee51a69c477:
	.long	L_OBJC_METH_VAR_NAME_6f2d5ee51a69c477

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_1d27e854714b8860
	.p2align	2
L_OBJC_IMAGE_INFO_1d27e854714b8860:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_1d27e854714b8860
L_OBJC_METH_VAR_NAME_1d27e854714b8860:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_1d27e854714b8860
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_1d27e854714b8860:
	.long	L_OBJC_METH_VAR_NAME_1d27e854714b8860

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_e56637a4c1a15b07
	.p2align	2
L_OBJC_IMAGE_INFO_e56637a4c1a15b07:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e56637a4c1a15b07
L_OBJC_METH_VAR_NAME_e56637a4c1a15b07:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_e56637a4c1a15b07
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_e56637a4c1a15b07:
	.long	L_OBJC_METH_VAR_NAME_e56637a4c1a15b07

.subsections_via_symbols
