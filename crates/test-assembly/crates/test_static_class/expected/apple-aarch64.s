	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_class
	.p2align	2
_get_class:
Lloh0:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_7443a74fb2d1e4c6@PAGE
Lloh1:
	ldr	x0, [x8, L_OBJC_CLASSLIST_REFERENCES_$_7443a74fb2d1e4c6@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.globl	_get_same_class
	.p2align	2
_get_same_class:
Lloh2:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_8f52a951012bf702@PAGE
Lloh3:
	ldr	x0, [x8, L_OBJC_CLASSLIST_REFERENCES_$_8f52a951012bf702@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh2, Lloh3

	.globl	_get_different_class
	.p2align	2
_get_different_class:
Lloh4:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_4882212c6ef400ba@PAGE
Lloh5:
	ldr	x0, [x8, L_OBJC_CLASSLIST_REFERENCES_$_4882212c6ef400ba@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh4, Lloh5

	.globl	_unused_class
	.p2align	2
_unused_class:
	ret

	.globl	_use_fns
	.p2align	2
_use_fns:
Lloh6:
	adrp	x9, L_OBJC_CLASSLIST_REFERENCES_$_7443a74fb2d1e4c6@PAGE
Lloh7:
	ldr	x9, [x9, L_OBJC_CLASSLIST_REFERENCES_$_7443a74fb2d1e4c6@PAGEOFF]
Lloh8:
	adrp	x10, L_OBJC_CLASSLIST_REFERENCES_$_8f52a951012bf702@PAGE
Lloh9:
	ldr	x10, [x10, L_OBJC_CLASSLIST_REFERENCES_$_8f52a951012bf702@PAGEOFF]
Lloh10:
	adrp	x11, L_OBJC_CLASSLIST_REFERENCES_$_4882212c6ef400ba@PAGE
Lloh11:
	ldr	x11, [x11, L_OBJC_CLASSLIST_REFERENCES_$_4882212c6ef400ba@PAGEOFF]
Lloh12:
	adrp	x12, L_OBJC_CLASSLIST_REFERENCES_$_5ca3eecf631727de@PAGE
Lloh13:
	ldr	x12, [x12, L_OBJC_CLASSLIST_REFERENCES_$_5ca3eecf631727de@PAGEOFF]
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
	adrp	x9, L_OBJC_CLASSLIST_REFERENCES_$_7443a74fb2d1e4c6@PAGE
Lloh15:
	ldr	x9, [x9, L_OBJC_CLASSLIST_REFERENCES_$_7443a74fb2d1e4c6@PAGEOFF]
	stp	x9, x9, [x8]
	ret
	.loh AdrpLdr	Lloh14, Lloh15

	.globl	_use_in_loop
	.p2align	2
_use_in_loop:
	ret

	.section	__DATA,__objc_classrefs
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_7443a74fb2d1e4c6
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_7443a74fb2d1e4c6:
	.quad	_OBJC_CLASS_$_NSObject

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_8f52a951012bf702
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_8f52a951012bf702:
	.quad	_OBJC_CLASS_$_NSObject

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_4882212c6ef400ba
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_4882212c6ef400ba:
	.quad	_OBJC_CLASS_$_NSString

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_9c6ceff32d4e4b8b
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_9c6ceff32d4e4b8b:
	.quad	_OBJC_CLASS_$_NSData

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_5ca3eecf631727de
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_5ca3eecf631727de:
	.quad	_OBJC_CLASS_$_NSException

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_76a360f1704b1e39
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_76a360f1704b1e39:
	.quad	_OBJC_CLASS_$_NSLock

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
