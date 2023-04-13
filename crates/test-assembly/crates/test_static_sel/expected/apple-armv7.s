	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_get_sel
	.p2align	2
	.code	32
_get_sel:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0-(LPC0_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0-(LPC0_0+8))
LPC0_0:
	ldr	r0, [pc, r0]
	bx	lr

	.globl	_get_same_sel
	.p2align	2
	.code	32
_get_same_sel:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83-(LPC1_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83-(LPC1_0+8))
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
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_25911857653c680c-(LPC3_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_25911857653c680c-(LPC3_0+8))
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
	movw	r9, :lower16:(L_OBJC_SELECTOR_REFERENCES_acb291d82e56f534-(LPC5_0+8))
	movt	r9, :upper16:(L_OBJC_SELECTOR_REFERENCES_acb291d82e56f534-(LPC5_0+8))
LPC5_0:
	ldr	r9, [pc, r9]
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_25911857653c680c-(LPC5_1+8))
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_25911857653c680c-(LPC5_1+8))
LPC5_1:
	ldr	r2, [pc, r2]
	movw	r3, :lower16:(L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83-(LPC5_2+8))
	movt	r3, :upper16:(L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83-(LPC5_2+8))
LPC5_2:
	ldr	r3, [pc, r3]
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0-(LPC5_3+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0-(LPC5_3+8))
	str	r2, [r0, #8]
LPC5_3:
	ldr	r1, [pc, r1]
	str	r9, [r0, #12]
	stm	r0, {r1, r3}
	bx	lr

	.globl	_use_same_twice
	.p2align	2
	.code	32
_use_same_twice:
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0-(LPC6_0+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0-(LPC6_0+8))
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

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2ff5c2d33acc98c0
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2ff5c2d33acc98c0:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2ff5c2d33acc98c0
L_OBJC_METH_VAR_NAME_2ff5c2d33acc98c0:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0:
	.long	L_OBJC_METH_VAR_NAME_2ff5c2d33acc98c0

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_6e17eb9d3fa7fa83
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_6e17eb9d3fa7fa83:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6e17eb9d3fa7fa83
L_OBJC_METH_VAR_NAME_6e17eb9d3fa7fa83:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83:
	.long	L_OBJC_METH_VAR_NAME_6e17eb9d3fa7fa83

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_25911857653c680c
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_25911857653c680c:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_25911857653c680c
L_OBJC_METH_VAR_NAME_25911857653c680c:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_25911857653c680c
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_25911857653c680c:
	.long	L_OBJC_METH_VAR_NAME_25911857653c680c

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_baa3c09478169afc
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_baa3c09478169afc:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_baa3c09478169afc
L_OBJC_METH_VAR_NAME_baa3c09478169afc:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_baa3c09478169afc
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_baa3c09478169afc:
	.long	L_OBJC_METH_VAR_NAME_baa3c09478169afc

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_acb291d82e56f534
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_acb291d82e56f534:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_acb291d82e56f534
L_OBJC_METH_VAR_NAME_acb291d82e56f534:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_acb291d82e56f534
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_acb291d82e56f534:
	.long	L_OBJC_METH_VAR_NAME_acb291d82e56f534

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_c831c01ba82dcc2e
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_c831c01ba82dcc2e:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_c831c01ba82dcc2e
L_OBJC_METH_VAR_NAME_c831c01ba82dcc2e:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_c831c01ba82dcc2e
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_c831c01ba82dcc2e:
	.long	L_OBJC_METH_VAR_NAME_c831c01ba82dcc2e

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2, 0x0
LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_alloc
	.long	0

.subsections_via_symbols
