	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_class
	.p2align	2
_get_class:
Lloh0:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777@PAGE
Lloh1:
	ldr	x0, [x8, L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.globl	_get_same_class
	.p2align	2
_get_same_class:
Lloh2:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07@PAGE
Lloh3:
	ldr	x0, [x8, L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh2, Lloh3

	.globl	_get_different_class
	.p2align	2
_get_different_class:
Lloh4:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b@PAGE
Lloh5:
	ldr	x0, [x8, L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b@PAGEOFF]
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
	adrp	x9, L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777@PAGE
Lloh7:
	ldr	x9, [x9, L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777@PAGEOFF]
Lloh8:
	adrp	x10, L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07@PAGE
Lloh9:
	ldr	x10, [x10, L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07@PAGEOFF]
Lloh10:
	adrp	x11, L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b@PAGE
Lloh11:
	ldr	x11, [x11, L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b@PAGEOFF]
Lloh12:
	adrp	x12, L_OBJC_CLASSLIST_REFERENCES_$_5ab5a81fcf2763fb@PAGE
Lloh13:
	ldr	x12, [x12, L_OBJC_CLASSLIST_REFERENCES_$_5ab5a81fcf2763fb@PAGEOFF]
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
	adrp	x9, L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777@PAGE
Lloh15:
	ldr	x9, [x9, L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777@PAGEOFF]
	stp	x9, x9, [x8]
	ret
	.loh AdrpLdr	Lloh14, Lloh15

	.globl	_use_in_loop
	.p2align	2
_use_in_loop:
	ret

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_928cf03fcc497777
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_928cf03fcc497777:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_928cf03fcc497777:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2fe1990982915f07
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2fe1990982915f07:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_2fe1990982915f07:
	.quad	_OBJC_CLASS_$_NSObject

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_dfff3a06c0bf722b
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_dfff3a06c0bf722b:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_dfff3a06c0bf722b:
	.quad	_OBJC_CLASS_$_NSString

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f6e054106fdbe219
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f6e054106fdbe219:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_f6e054106fdbe219
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_f6e054106fdbe219:
	.quad	_OBJC_CLASS_$_NSData

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_5ab5a81fcf2763fb
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_5ab5a81fcf2763fb:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_5ab5a81fcf2763fb
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_5ab5a81fcf2763fb:
	.quad	_OBJC_CLASS_$_NSException

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_54ecac6d305d112a
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_54ecac6d305d112a:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__objc_classrefs,regular,no_dead_strip
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_54ecac6d305d112a
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_54ecac6d305d112a:
	.quad	_OBJC_CLASS_$_NSLock

.subsections_via_symbols
