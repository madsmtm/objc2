	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_get_class
	.p2align	2
	.code	32
_get_class:
	movw	r0, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f-(LPC0_0+8))
	movt	r0, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f-(LPC0_0+8))
LPC0_0:
	ldr	r0, [pc, r0]
	bx	lr

	.globl	_get_same_class
	.p2align	2
	.code	32
_get_same_class:
	movw	r0, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79-(LPC1_0+8))
	movt	r0, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79-(LPC1_0+8))
LPC1_0:
	ldr	r0, [pc, r0]
	bx	lr

	.globl	_get_different_class
	.p2align	2
	.code	32
_get_different_class:
	movw	r0, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4-(LPC2_0+8))
	movt	r0, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4-(LPC2_0+8))
LPC2_0:
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
	movw	r9, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_2f45d8445f72bd9b-(LPC4_0+8))
	movt	r9, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_2f45d8445f72bd9b-(LPC4_0+8))
LPC4_0:
	ldr	r9, [pc, r9]
	movw	r2, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4-(LPC4_1+8))
	movt	r2, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4-(LPC4_1+8))
LPC4_1:
	ldr	r2, [pc, r2]
	movw	r3, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79-(LPC4_2+8))
	movt	r3, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79-(LPC4_2+8))
LPC4_2:
	ldr	r3, [pc, r3]
	movw	r1, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f-(LPC4_3+8))
	movt	r1, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f-(LPC4_3+8))
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
	movw	r1, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f-(LPC5_0+8))
	movt	r1, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f-(LPC5_0+8))
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
	.globl	L_OBJC_IMAGE_INFO_9657804a2a54ab6f
	.p2align	2
L_OBJC_IMAGE_INFO_9657804a2a54ab6f:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f
	.p2align	2
L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f:
	.long	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_e897a41b218dcf79
	.p2align	2
L_OBJC_IMAGE_INFO_e897a41b218dcf79:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79
	.p2align	2
L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79:
	.long	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_3b7780b4dcfcb9d4
	.p2align	2
L_OBJC_IMAGE_INFO_3b7780b4dcfcb9d4:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4
	.p2align	2
L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4:
	.long	_OBJC_CLASS_$_NSString

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f2fb7c579d3c0a74
	.p2align	2
L_OBJC_IMAGE_INFO_f2fb7c579d3c0a74:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_f2fb7c579d3c0a74
	.p2align	2
L_OBJC_CLASSLIST_REFERENCES_$_f2fb7c579d3c0a74:
	.long	_OBJC_CLASS_$_NSData

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2f45d8445f72bd9b
	.p2align	2
L_OBJC_IMAGE_INFO_2f45d8445f72bd9b:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_2f45d8445f72bd9b
	.p2align	2
L_OBJC_CLASSLIST_REFERENCES_$_2f45d8445f72bd9b:
	.long	_OBJC_CLASS_$_NSException

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_3bf610c78df2b6bb
	.p2align	2
L_OBJC_IMAGE_INFO_3bf610c78df2b6bb:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_3bf610c78df2b6bb
	.p2align	2
L_OBJC_CLASSLIST_REFERENCES_$_3bf610c78df2b6bb:
	.long	_OBJC_CLASS_$_NSLock

.subsections_via_symbols
