	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_get_sel
	.p2align	2
	.code	32
_get_sel:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9-(LPC0_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9-(LPC0_0+8))
LPC0_0:
	ldr	r0, [pc, r0]
	bx	lr

	.globl	_get_same_sel
	.p2align	2
	.code	32
_get_same_sel:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35-(LPC1_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35-(LPC1_0+8))
LPC1_0:
	ldr	r0, [pc, r0]
	bx	lr

	.globl	_get_common_twice
	.p2align	2
	.code	32
_get_common_twice:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_e4a45d49bfea5d77-(LPC2_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_e4a45d49bfea5d77-(LPC2_0+8))
LPC2_0:
	ldr	r0, [pc, r0]
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_33db9f67352fe9a7-(LPC2_1+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_33db9f67352fe9a7-(LPC2_1+8))
LPC2_1:
	ldr	r1, [pc, r1]
	bx	lr

	.globl	_get_different_sel
	.p2align	2
	.code	32
_get_different_sel:
	movw	r0, :lower16:(L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0-(LPC3_0+8))
	movt	r0, :upper16:(L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0-(LPC3_0+8))
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
	movw	r9, :lower16:(L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99-(LPC5_0+8))
	movt	r9, :upper16:(L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99-(LPC5_0+8))
LPC5_0:
	ldr	r9, [pc, r9]
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0-(LPC5_1+8))
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0-(LPC5_1+8))
LPC5_1:
	ldr	r2, [pc, r2]
	movw	r3, :lower16:(L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35-(LPC5_2+8))
	movt	r3, :upper16:(L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35-(LPC5_2+8))
LPC5_2:
	ldr	r3, [pc, r3]
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9-(LPC5_3+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9-(LPC5_3+8))
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
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9-(LPC6_0+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9-(LPC6_0+8))
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
	.globl	L_OBJC_IMAGE_INFO_68381ba894e318e9
	.p2align	2
L_OBJC_IMAGE_INFO_68381ba894e318e9:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_68381ba894e318e9
L_OBJC_METH_VAR_NAME_68381ba894e318e9:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9:
	.long	L_OBJC_METH_VAR_NAME_68381ba894e318e9

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_cd2fd6e7d2adcc35
	.p2align	2
L_OBJC_IMAGE_INFO_cd2fd6e7d2adcc35:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35
L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35:
	.long	L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_e4a45d49bfea5d77
	.p2align	2
L_OBJC_IMAGE_INFO_e4a45d49bfea5d77:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e4a45d49bfea5d77
L_OBJC_METH_VAR_NAME_e4a45d49bfea5d77:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_e4a45d49bfea5d77
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_e4a45d49bfea5d77:
	.long	L_OBJC_METH_VAR_NAME_e4a45d49bfea5d77

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_33db9f67352fe9a7
	.p2align	2
L_OBJC_IMAGE_INFO_33db9f67352fe9a7:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_33db9f67352fe9a7
L_OBJC_METH_VAR_NAME_33db9f67352fe9a7:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_33db9f67352fe9a7
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_33db9f67352fe9a7:
	.long	L_OBJC_METH_VAR_NAME_33db9f67352fe9a7

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_bb37877368f0b7a0
	.p2align	2
L_OBJC_IMAGE_INFO_bb37877368f0b7a0:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_bb37877368f0b7a0
L_OBJC_METH_VAR_NAME_bb37877368f0b7a0:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0:
	.long	L_OBJC_METH_VAR_NAME_bb37877368f0b7a0

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2c505e110d181b25
	.p2align	2
L_OBJC_IMAGE_INFO_2c505e110d181b25:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2c505e110d181b25
L_OBJC_METH_VAR_NAME_2c505e110d181b25:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2c505e110d181b25
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_2c505e110d181b25:
	.long	L_OBJC_METH_VAR_NAME_2c505e110d181b25

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_5419c3f7fc0a6f99
	.p2align	2
L_OBJC_IMAGE_INFO_5419c3f7fc0a6f99:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_5419c3f7fc0a6f99
L_OBJC_METH_VAR_NAME_5419c3f7fc0a6f99:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99:
	.long	L_OBJC_METH_VAR_NAME_5419c3f7fc0a6f99

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f46908e864c86c6b
	.p2align	2
L_OBJC_IMAGE_INFO_f46908e864c86c6b:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_f46908e864c86c6b
L_OBJC_METH_VAR_NAME_f46908e864c86c6b:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_f46908e864c86c6b
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_f46908e864c86c6b:
	.long	L_OBJC_METH_VAR_NAME_f46908e864c86c6b

.subsections_via_symbols
