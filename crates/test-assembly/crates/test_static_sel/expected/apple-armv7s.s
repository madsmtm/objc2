	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_get_sel
	.p2align	2
	.code	32
_get_sel:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7-(LPC0_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7-(LPC0_0+8))
LPC0_0:
	ldr	r0, [pc, r0]
	bx	lr

	.globl	_get_same_sel
	.p2align	2
	.code	32
_get_same_sel:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513-(LPC1_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513-(LPC1_0+8))
LPC1_0:
	ldr	r0, [pc, r0]
	bx	lr

	.globl	_get_common_twice
	.p2align	2
	.code	32
_get_common_twice:
	movw	r0, :lower16:(LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr-(LPC2_0+8))
	movt	r0, :upper16:(LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr-(LPC2_0+8))
LPC2_0:
	ldr	r0, [pc, r0]
	ldr	r0, [r0]
	mov	r1, r0
	bx	lr

	.globl	_get_different_sel
	.p2align	2
	.code	32
_get_different_sel:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864-(LPC3_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864-(LPC3_0+8))
LPC3_0:
	ldr	r0, [pc, r0]
	bx	lr

	.globl	_unused_sel
	.p2align	2
	.code	32
_unused_sel:
	bx	lr

	.globl	_use_fns
	.p2align	2
	.code	32
_use_fns:
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7-(LPC5_0+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7-(LPC5_0+8))
LPC5_0:
	ldr	r1, [pc, r1]
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513-(LPC5_1+8))
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513-(LPC5_1+8))
LPC5_1:
	ldr	r2, [pc, r2]
	movw	r3, :lower16:(L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864-(LPC5_2+8))
	movt	r3, :upper16:(L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864-(LPC5_2+8))
LPC5_2:
	ldr	r3, [pc, r3]
	movw	r9, :lower16:(L_OBJC_SELECTOR_REFERENCES_408f5be8f4fd2627-(LPC5_3+8))
	movt	r9, :upper16:(L_OBJC_SELECTOR_REFERENCES_408f5be8f4fd2627-(LPC5_3+8))
LPC5_3:
	ldr	r9, [pc, r9]
	stm	r0, {r1, r2, r3, r9}
	bx	lr

	.globl	_use_same_twice
	.p2align	2
	.code	32
_use_same_twice:
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7-(LPC6_0+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7-(LPC6_0+8))
LPC6_0:
	ldr	r1, [pc, r1]
	str	r1, [r0]
	str	r1, [r0, #4]
	bx	lr

	.globl	_use_in_loop
	.p2align	2
	.code	32
_use_in_loop:
	bx	lr

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_caedaca3f40015a7
L_OBJC_METH_VAR_NAME_caedaca3f40015a7:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7:
	.long	L_OBJC_METH_VAR_NAME_caedaca3f40015a7

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_caedaca3f40015a7
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_caedaca3f40015a7:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_a7c7f3067f40b513
L_OBJC_METH_VAR_NAME_a7c7f3067f40b513:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513:
	.long	L_OBJC_METH_VAR_NAME_a7c7f3067f40b513

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_a7c7f3067f40b513
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a7c7f3067f40b513:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_bae8570d40d73864
L_OBJC_METH_VAR_NAME_bae8570d40d73864:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864:
	.long	L_OBJC_METH_VAR_NAME_bae8570d40d73864

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_bae8570d40d73864
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_bae8570d40d73864:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_9c1b77e8cf40622d
L_OBJC_METH_VAR_NAME_9c1b77e8cf40622d:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_9c1b77e8cf40622d
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_9c1b77e8cf40622d:
	.long	L_OBJC_METH_VAR_NAME_9c1b77e8cf40622d

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_9c1b77e8cf40622d
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_9c1b77e8cf40622d:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_408f5be8f4fd2627
L_OBJC_METH_VAR_NAME_408f5be8f4fd2627:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_408f5be8f4fd2627
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_408f5be8f4fd2627:
	.long	L_OBJC_METH_VAR_NAME_408f5be8f4fd2627

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_408f5be8f4fd2627
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_408f5be8f4fd2627:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_82483a8131827890
L_OBJC_METH_VAR_NAME_82483a8131827890:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_82483a8131827890
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_82483a8131827890:
	.long	L_OBJC_METH_VAR_NAME_82483a8131827890

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_82483a8131827890
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_82483a8131827890:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2, 0x0
LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_alloc
	.long	0

.subsections_via_symbols
