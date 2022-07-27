	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_class
	.p2align	2
_get_class:
Lloh0:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f@PAGE
Lloh1:
	ldr	x0, [x8, L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.globl	_get_same_class
	.p2align	2
_get_same_class:
Lloh2:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79@PAGE
Lloh3:
	ldr	x0, [x8, L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh2, Lloh3

	.globl	_get_different_class
	.p2align	2
_get_different_class:
Lloh4:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4@PAGE
Lloh5:
	ldr	x0, [x8, L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh4, Lloh5

	.globl	_unused_sel
	.p2align	2
_unused_sel:
	ret

	.globl	_use_fns
	.p2align	2
_use_fns:
Lloh6:
	adrp	x9, L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f@PAGE
Lloh7:
	ldr	x9, [x9, L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f@PAGEOFF]
Lloh8:
	adrp	x10, L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79@PAGE
Lloh9:
	ldr	x10, [x10, L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79@PAGEOFF]
Lloh10:
	adrp	x11, L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4@PAGE
Lloh11:
	ldr	x11, [x11, L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4@PAGEOFF]
Lloh12:
	adrp	x12, L_OBJC_CLASSLIST_REFERENCES_$_2f45d8445f72bd9b@PAGE
Lloh13:
	ldr	x12, [x12, L_OBJC_CLASSLIST_REFERENCES_$_2f45d8445f72bd9b@PAGEOFF]
	stp	x9, x10, [x8]
	stp	x11, x12, [x8, #16]
	ret
	.loh AdrpLdr	Lloh12, Lloh13
	.loh AdrpLdr	Lloh10, Lloh11
	.loh AdrpLdr	Lloh8, Lloh9
	.loh AdrpLdr	Lloh6, Lloh7

	.globl	_use_same_twice
	.p2align	2
_use_same_twice:
Lloh14:
	adrp	x9, L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f@PAGE
Lloh15:
	ldr	x9, [x9, L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f@PAGEOFF]
	stp	x9, x9, [x8]
	ret
	.loh AdrpLdr	Lloh14, Lloh15

	.globl	_use_in_loop
	.p2align	2
_use_in_loop:
	ret

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_9657804a2a54ab6f
	.p2align	2
L_OBJC_IMAGE_INFO_9657804a2a54ab6f:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_9657804a2a54ab6f:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_e897a41b218dcf79
	.p2align	2
L_OBJC_IMAGE_INFO_e897a41b218dcf79:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_e897a41b218dcf79:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_3b7780b4dcfcb9d4
	.p2align	2
L_OBJC_IMAGE_INFO_3b7780b4dcfcb9d4:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_3b7780b4dcfcb9d4:
	.quad	_OBJC_CLASS_$_NSString

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f2fb7c579d3c0a74
	.p2align	2
L_OBJC_IMAGE_INFO_f2fb7c579d3c0a74:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_f2fb7c579d3c0a74
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_f2fb7c579d3c0a74:
	.quad	_OBJC_CLASS_$_NSData

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2f45d8445f72bd9b
	.p2align	2
L_OBJC_IMAGE_INFO_2f45d8445f72bd9b:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_2f45d8445f72bd9b
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_2f45d8445f72bd9b:
	.quad	_OBJC_CLASS_$_NSException

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_3bf610c78df2b6bb
	.p2align	2
L_OBJC_IMAGE_INFO_3bf610c78df2b6bb:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_3bf610c78df2b6bb
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_3bf610c78df2b6bb:
	.quad	_OBJC_CLASS_$_NSLock

.subsections_via_symbols
