	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_get_sel
	.p2align	2
	.code	32
_get_sel:
	push	{r7, lr}
	mov	r7, sp
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17hfbc0e8d2d2b9b0faE
	pop	{r7, pc}

	.globl	_get_same_sel
	.p2align	2
	.code	32
_get_same_sel:
	push	{r7, lr}
	mov	r7, sp
	bl	__ZN15test_static_sel12get_same_sel22objc_static_workaround17h206241f3cdb815f2E
	pop	{r7, pc}

	.globl	_get_common
	.p2align	2
	.code	32
_get_common:
	push	{r7, lr}
	mov	r7, sp
	bl	__ZN15test_static_sel10get_common22objc_static_workaround17h43aa5ab975f84968E
	pop	{r7, pc}

	.globl	_get_different_sel
	.p2align	2
	.code	32
_get_different_sel:
	push	{r7, lr}
	mov	r7, sp
	bl	__ZN15test_static_sel17get_different_sel22objc_static_workaround17h6d4d0b19565e8bbaE
	pop	{r7, pc}

	.globl	_unused_sel
	.p2align	2
	.code	32
_unused_sel:
	push	{r7, lr}
	mov	r7, sp
	bl	__ZN15test_static_sel10unused_sel22objc_static_workaround17hccfdb864c1636284E
	pop	{r7, pc}

	.globl	_use_fns
	.p2align	2
	.code	32
_use_fns:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	push	{r8}
	mov	r4, r0
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17hfbc0e8d2d2b9b0faE
	mov	r8, r0
	bl	__ZN15test_static_sel12get_same_sel22objc_static_workaround17h206241f3cdb815f2E
	mov	r6, r0
	bl	__ZN15test_static_sel17get_different_sel22objc_static_workaround17h6d4d0b19565e8bbaE
	mov	r5, r0
	bl	__ZN15test_static_sel7use_fns22objc_static_workaround17h08f64a44e1d1f6ffE
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
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17hfbc0e8d2d2b9b0faE
	mov	r8, r0
	bl	__ZN15test_static_sel7get_sel22objc_static_workaround17hfbc0e8d2d2b9b0faE
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
	bl	__ZN15test_static_sel11use_in_loop22objc_static_workaround17hef7c0204ccd46c76E
	subs	r4, r4, #1
	bne	LBB7_2
	pop	{r4, r7, pc}

	.p2align	2
	.code	32
__ZN15test_static_sel7get_sel22objc_static_workaround17hfbc0e8d2d2b9b0faE:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9-(LPC8_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9-(LPC8_0+8))
LPC8_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel12get_same_sel22objc_static_workaround17h206241f3cdb815f2E:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35-(LPC9_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35-(LPC9_0+8))
LPC9_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel10get_common22objc_static_workaround17h43aa5ab975f84968E:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_34d6c3ed70e85964-(LPC10_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_34d6c3ed70e85964-(LPC10_0+8))
LPC10_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel17get_different_sel22objc_static_workaround17h6d4d0b19565e8bbaE:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_ab5e106a55f71e5b-(LPC11_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_ab5e106a55f71e5b-(LPC11_0+8))
LPC11_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel10unused_sel22objc_static_workaround17hccfdb864c1636284E:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_6f2d5ee51a69c477-(LPC12_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_6f2d5ee51a69c477-(LPC12_0+8))
LPC12_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel7use_fns22objc_static_workaround17h08f64a44e1d1f6ffE:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_1d27e854714b8860-(LPC13_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_1d27e854714b8860-(LPC13_0+8))
LPC13_0:
	ldr	r0, [pc, r0]
	bx	lr

	.p2align	2
	.code	32
__ZN15test_static_sel11use_in_loop22objc_static_workaround17hef7c0204ccd46c76E:
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
