	.syntax unified
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_class
	.p2align	2
	.code	32
_get_class:
	push	{r7, lr}
	mov	r7, sp
	movw	r0, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_7443a74fb2d1e4c6-(LPC0_0+8))
	movt	r0, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_7443a74fb2d1e4c6-(LPC0_0+8))
LPC0_0:
	ldr	r0, [pc, r0]
	pop	{r7, pc}

	.globl	_get_same_class
	.p2align	2
	.code	32
_get_same_class:
	push	{r7, lr}
	mov	r7, sp
	movw	r0, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_8f52a951012bf702-(LPC1_0+8))
	movt	r0, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_8f52a951012bf702-(LPC1_0+8))
LPC1_0:
	ldr	r0, [pc, r0]
	pop	{r7, pc}

	.globl	_get_different_class
	.p2align	2
	.code	32
_get_different_class:
	push	{r7, lr}
	mov	r7, sp
	movw	r0, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_4882212c6ef400ba-(LPC2_0+8))
	movt	r0, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_4882212c6ef400ba-(LPC2_0+8))
LPC2_0:
	ldr	r0, [pc, r0]
	pop	{r7, pc}

	.globl	_unused_class
	.p2align	2
	.code	32
_unused_class:
	push	{r7, lr}
	mov	r7, sp
	pop	{r7, pc}

	.globl	_use_fns
	.p2align	2
	.code	32
_use_fns:
	push	{r7, lr}
	mov	r7, sp
	movw	r1, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_7443a74fb2d1e4c6-(LPC4_0+8))
	movt	r1, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_7443a74fb2d1e4c6-(LPC4_0+8))
LPC4_0:
	ldr	r1, [pc, r1]
	movw	r2, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_8f52a951012bf702-(LPC4_1+8))
	movt	r2, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_8f52a951012bf702-(LPC4_1+8))
LPC4_1:
	ldr	r2, [pc, r2]
	movw	r3, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_4882212c6ef400ba-(LPC4_2+8))
	movt	r3, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_4882212c6ef400ba-(LPC4_2+8))
LPC4_2:
	ldr	r3, [pc, r3]
	movw	r9, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_5ca3eecf631727de-(LPC4_3+8))
	movt	r9, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_5ca3eecf631727de-(LPC4_3+8))
LPC4_3:
	ldr	r9, [pc, r9]
	stm	r0, {r1, r2, r3, r9}
	pop	{r7, pc}

	.globl	_use_same_twice
	.p2align	2
	.code	32
_use_same_twice:
	push	{r7, lr}
	mov	r7, sp
	movw	r1, :lower16:(L_OBJC_CLASSLIST_REFERENCES_$_7443a74fb2d1e4c6-(LPC5_0+8))
	movt	r1, :upper16:(L_OBJC_CLASSLIST_REFERENCES_$_7443a74fb2d1e4c6-(LPC5_0+8))
LPC5_0:
	ldr	r1, [pc, r1]
	str	r1, [r0]
	str	r1, [r0, #4]
	pop	{r7, pc}

	.globl	_use_in_loop
	.p2align	2
	.code	32
_use_in_loop:
	push	{r7, lr}
	mov	r7, sp
	pop	{r7, pc}

	.section	__DATA,__objc_classrefs
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_7443a74fb2d1e4c6
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_7443a74fb2d1e4c6:
	.long	_OBJC_CLASS_$_NSObject

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_8f52a951012bf702
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_8f52a951012bf702:
	.long	_OBJC_CLASS_$_NSObject

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_4882212c6ef400ba
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_4882212c6ef400ba:
	.long	_OBJC_CLASS_$_NSString

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_9c6ceff32d4e4b8b
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_9c6ceff32d4e4b8b:
	.long	_OBJC_CLASS_$_NSData

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_5ca3eecf631727de
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_5ca3eecf631727de:
	.long	_OBJC_CLASS_$_NSException

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_76a360f1704b1e39
	.p2align	2, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_76a360f1704b1e39:
	.long	_OBJC_CLASS_$_NSLock

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_7443a74fb2d1e4c6
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_7443a74fb2d1e4c6:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_8f52a951012bf702
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_8f52a951012bf702:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_4882212c6ef400ba
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_4882212c6ef400ba:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_9c6ceff32d4e4b8b
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_9c6ceff32d4e4b8b:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_5ca3eecf631727de
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_5ca3eecf631727de:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_76a360f1704b1e39
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_76a360f1704b1e39:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
