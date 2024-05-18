	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_get_class
	.p2align	2
	.code	32
_get_class:
	movw	r0, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_831fece26e45cd9e-(LPC0_0+8))
	movt	r0, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_831fece26e45cd9e-(LPC0_0+8))
LPC0_0:
	ldr	r0, [pc, r0]
	bx	lr

	.globl	_get_same_class
	.p2align	2
	.code	32
_get_same_class:
	movw	r0, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_723df0da59ee573a-(LPC1_0+8))
	movt	r0, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_723df0da59ee573a-(LPC1_0+8))
LPC1_0:
	ldr	r0, [pc, r0]
	bx	lr

	.globl	_get_different_class
	.p2align	2
	.code	32
_get_different_class:
	movw	r0, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_a1c78af2bef71f32-(LPC2_0+8))
	movt	r0, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_a1c78af2bef71f32-(LPC2_0+8))
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
	movw	r1, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_831fece26e45cd9e-(LPC4_0+8))
	movt	r1, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_831fece26e45cd9e-(LPC4_0+8))
LPC4_0:
	ldr	r1, [pc, r1]
	movw	r2, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_723df0da59ee573a-(LPC4_1+8))
	movt	r2, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_723df0da59ee573a-(LPC4_1+8))
LPC4_1:
	ldr	r2, [pc, r2]
	movw	r3, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_a1c78af2bef71f32-(LPC4_2+8))
	movt	r3, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_a1c78af2bef71f32-(LPC4_2+8))
LPC4_2:
	ldr	r3, [pc, r3]
	movw	r9, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_ae7bef6061eca0c4-(LPC4_3+8))
	movt	r9, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_ae7bef6061eca0c4-(LPC4_3+8))
LPC4_3:
	ldr	r9, [pc, r9]
	stm	r0, {r1, r2, r3, r9}
	bx	lr

	.globl	_use_same_twice
	.p2align	2
	.code	32
_use_same_twice:
	movw	r1, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_831fece26e45cd9e-(LPC5_0+8))
	movt	r1, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_831fece26e45cd9e-(LPC5_0+8))
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

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_831fece26e45cd9e
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_831fece26e45cd9e:
	.long	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_831fece26e45cd9e
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_831fece26e45cd9e:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_723df0da59ee573a
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_723df0da59ee573a:
	.long	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_723df0da59ee573a
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_723df0da59ee573a:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_a1c78af2bef71f32
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_a1c78af2bef71f32:
	.long	_OBJC_CLASS_$_NSString

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_a1c78af2bef71f32
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a1c78af2bef71f32:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_ce452f03dca2d1c0
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_ce452f03dca2d1c0:
	.long	_OBJC_CLASS_$_NSData

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_ce452f03dca2d1c0
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ce452f03dca2d1c0:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_ae7bef6061eca0c4
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_ae7bef6061eca0c4:
	.long	_OBJC_CLASS_$_NSException

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_ae7bef6061eca0c4
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ae7bef6061eca0c4:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_bedb39edf06e7f45
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_bedb39edf06e7f45:
	.long	_OBJC_CLASS_$_NSLock

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_bedb39edf06e7f45
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_bedb39edf06e7f45:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
