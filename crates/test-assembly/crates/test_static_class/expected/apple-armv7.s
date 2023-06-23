	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_get_class
	.p2align	2
	.code	32
_get_class:
	movw	r0, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777-(LPC0_0+8))
	movt	r0, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777-(LPC0_0+8))
LPC0_0:
	ldr	r0, [pc, r0]
	bx	lr

	.globl	_get_same_class
	.p2align	2
	.code	32
_get_same_class:
	movw	r0, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07-(LPC1_0+8))
	movt	r0, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07-(LPC1_0+8))
LPC1_0:
	ldr	r0, [pc, r0]
	bx	lr

	.globl	_get_different_class
	.p2align	2
	.code	32
_get_different_class:
	movw	r0, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b-(LPC2_0+8))
	movt	r0, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b-(LPC2_0+8))
LPC2_0:
	ldr	r0, [pc, r0]
	bx	lr

	.globl	_unused_class
	.p2align	2
	.code	32
_unused_class:
	bx	lr

	.globl	_use_fns
	.p2align	2
	.code	32
_use_fns:
	movw	r9, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_97e6a8c6ed5db063-(LPC4_0+8))
	movt	r9, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_97e6a8c6ed5db063-(LPC4_0+8))
LPC4_0:
	ldr	r9, [pc, r9]
	movw	r2, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b-(LPC4_1+8))
	movt	r2, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b-(LPC4_1+8))
LPC4_1:
	ldr	r2, [pc, r2]
	movw	r3, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07-(LPC4_2+8))
	movt	r3, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07-(LPC4_2+8))
LPC4_2:
	ldr	r3, [pc, r3]
	movw	r1, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777-(LPC4_3+8))
	movt	r1, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777-(LPC4_3+8))
	str	r2, [r0, #8]
LPC4_3:
	ldr	r1, [pc, r1]
	str	r9, [r0, #12]
	stm	r0, {r1, r3}
	bx	lr

	.globl	_use_same_twice
	.p2align	2
	.code	32
_use_same_twice:
	movw	r1, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777-(LPC5_0+8))
	movt	r1, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777-(LPC5_0+8))
LPC5_0:
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
	.globl	L_OBJC_IMAGE_INFO_928cf03fcc497777
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_928cf03fcc497777:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777:
	.long	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2fe1990982915f07
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2fe1990982915f07:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07:
	.long	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_dfff3a06c0bf722b
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_dfff3a06c0bf722b:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b:
	.long	_OBJC_CLASS_$_NSString

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_ea6fbcf172f7f513
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ea6fbcf172f7f513:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_ea6fbcf172f7f513
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_ea6fbcf172f7f513:
	.long	_OBJC_CLASS_$_NSData

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_97e6a8c6ed5db063
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_97e6a8c6ed5db063:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_97e6a8c6ed5db063
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_97e6a8c6ed5db063:
	.long	_OBJC_CLASS_$_NSException

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_bb5b616899716c0d
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_bb5b616899716c0d:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_bb5b616899716c0d
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_bb5b616899716c0d:
	.long	_OBJC_CLASS_$_NSLock

.subsections_via_symbols
