	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_class
	.p2align	2
_get_class:
Lloh0:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_49a0cd2a35b9a474@PAGE
Lloh1:
	ldr	x0, [x8, L_OBJC_CLASSLIST_REFERENCES_$_49a0cd2a35b9a474@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.globl	_get_same_class
	.p2align	2
_get_same_class:
Lloh2:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_60860b498061fbc6@PAGE
Lloh3:
	ldr	x0, [x8, L_OBJC_CLASSLIST_REFERENCES_$_60860b498061fbc6@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh2, Lloh3

	.globl	_get_different_class
	.p2align	2
_get_different_class:
Lloh4:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_089cee9fe04089a4@PAGE
Lloh5:
	ldr	x0, [x8, L_OBJC_CLASSLIST_REFERENCES_$_089cee9fe04089a4@PAGEOFF]
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
	adrp	x9, L_OBJC_CLASSLIST_REFERENCES_$_49a0cd2a35b9a474@PAGE
Lloh7:
	ldr	x9, [x9, L_OBJC_CLASSLIST_REFERENCES_$_49a0cd2a35b9a474@PAGEOFF]
Lloh8:
	adrp	x10, L_OBJC_CLASSLIST_REFERENCES_$_60860b498061fbc6@PAGE
Lloh9:
	ldr	x10, [x10, L_OBJC_CLASSLIST_REFERENCES_$_60860b498061fbc6@PAGEOFF]
Lloh10:
	adrp	x11, L_OBJC_CLASSLIST_REFERENCES_$_089cee9fe04089a4@PAGE
Lloh11:
	ldr	x11, [x11, L_OBJC_CLASSLIST_REFERENCES_$_089cee9fe04089a4@PAGEOFF]
Lloh12:
	adrp	x12, L_OBJC_CLASSLIST_REFERENCES_$_15b3f8b356e4fdb3@PAGE
Lloh13:
	ldr	x12, [x12, L_OBJC_CLASSLIST_REFERENCES_$_15b3f8b356e4fdb3@PAGEOFF]
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
	adrp	x9, L_OBJC_CLASSLIST_REFERENCES_$_49a0cd2a35b9a474@PAGE
Lloh15:
	ldr	x9, [x9, L_OBJC_CLASSLIST_REFERENCES_$_49a0cd2a35b9a474@PAGEOFF]
	stp	x9, x9, [x8]
	ret
	.loh AdrpLdr	Lloh14, Lloh15

	.globl	_use_in_loop
	.p2align	2
_use_in_loop:
	ret

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_49a0cd2a35b9a474
	.p2align	2
L_OBJC_IMAGE_INFO_49a0cd2a35b9a474:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_49a0cd2a35b9a474
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_49a0cd2a35b9a474:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_60860b498061fbc6
	.p2align	2
L_OBJC_IMAGE_INFO_60860b498061fbc6:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_60860b498061fbc6
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_60860b498061fbc6:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_089cee9fe04089a4
	.p2align	2
L_OBJC_IMAGE_INFO_089cee9fe04089a4:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_089cee9fe04089a4
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_089cee9fe04089a4:
	.quad	_OBJC_CLASS_$_NSString

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_5331bb309754c706
	.p2align	2
L_OBJC_IMAGE_INFO_5331bb309754c706:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_5331bb309754c706
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_5331bb309754c706:
	.quad	_OBJC_CLASS_$_NSData

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_15b3f8b356e4fdb3
	.p2align	2
L_OBJC_IMAGE_INFO_15b3f8b356e4fdb3:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_15b3f8b356e4fdb3
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_15b3f8b356e4fdb3:
	.quad	_OBJC_CLASS_$_NSException

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_4fe8a7873c5b5bcf
	.p2align	2
L_OBJC_IMAGE_INFO_4fe8a7873c5b5bcf:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_4fe8a7873c5b5bcf
	.p2align	3
L_OBJC_CLASSLIST_REFERENCES_$_4fe8a7873c5b5bcf:
	.quad	_OBJC_CLASS_$_NSLock

.subsections_via_symbols
