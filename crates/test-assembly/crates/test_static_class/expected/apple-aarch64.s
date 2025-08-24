	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn1_get_class
	.p2align	2
_fn1_get_class:
Lloh0:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34@PAGE
Lloh1:
	ldr	x0, [x8, L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.globl	_fn1_get_same_class
	.p2align	2
_fn1_get_same_class:
Lloh2:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a@PAGE
Lloh3:
	ldr	x0, [x8, L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh2, Lloh3

	.globl	_fn3_get_different_class
	.p2align	2
_fn3_get_different_class:
Lloh4:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1@PAGE
Lloh5:
	ldr	x0, [x8, L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh4, Lloh5

	.globl	_fn4_unused_class
	.p2align	2
_fn4_unused_class:
	ret

	.globl	_fn5_use_fns
	.p2align	2
_fn5_use_fns:
Lloh6:
	adrp	x9, L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34@PAGE
Lloh7:
	ldr	x9, [x9, L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34@PAGEOFF]
Lloh8:
	adrp	x10, L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a@PAGE
Lloh9:
	ldr	x10, [x10, L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a@PAGEOFF]
Lloh10:
	adrp	x11, L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1@PAGE
Lloh11:
	ldr	x11, [x11, L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1@PAGEOFF]
Lloh12:
	adrp	x12, L_OBJC_CLASSLIST_REFERENCES_$_a92f01d3b55d29c5@PAGE
Lloh13:
	ldr	x12, [x12, L_OBJC_CLASSLIST_REFERENCES_$_a92f01d3b55d29c5@PAGEOFF]
	stp	x9, x10, [x8]
	stp	x11, x12, [x8, #16]
	ret
	.loh AdrpLdr	Lloh12, Lloh13
	.loh AdrpLdr	Lloh10, Lloh11
	.loh AdrpLdr	Lloh8, Lloh9
	.loh AdrpLdr	Lloh6, Lloh7

	.globl	_fn6_use_same_twice
	.p2align	2
_fn6_use_same_twice:
Lloh14:
	adrp	x9, L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34@PAGE
Lloh15:
	ldr	x9, [x9, L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34@PAGEOFF]
	stp	x9, x9, [x8]
	ret
	.loh AdrpLdr	Lloh14, Lloh15

	.globl	_fn7_use_in_loop
	.p2align	2
_fn7_use_in_loop:
	ret

	.section	__DATA,__objc_classrefs
	.globl	L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_1f36dafa1e0a7b34:
	.quad	_OBJC_CLASS_$_NSObject

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_e1a6d3426ab3be5a:
	.quad	_OBJC_CLASS_$_NSObject

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_5a6ce274a9f949e1:
	.quad	_OBJC_CLASS_$_NSString

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_9f503c7582f87b48
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_9f503c7582f87b48:
	.quad	_OBJC_CLASS_$_NSData

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_a92f01d3b55d29c5
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_a92f01d3b55d29c5:
	.quad	_OBJC_CLASS_$_NSException

	.globl	L_OBJC_CLASSLIST_REFERENCES_$_d4ef9efb3ee49ab7
	.p2align	3, 0x0
L_OBJC_CLASSLIST_REFERENCES_$_d4ef9efb3ee49ab7:
	.quad	_OBJC_CLASS_$_NSLock

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_1f36dafa1e0a7b34
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_1f36dafa1e0a7b34:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_e1a6d3426ab3be5a
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_e1a6d3426ab3be5a:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_5a6ce274a9f949e1
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_5a6ce274a9f949e1:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_9f503c7582f87b48
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_9f503c7582f87b48:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_a92f01d3b55d29c5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a92f01d3b55d29c5:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_d4ef9efb3ee49ab7
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_d4ef9efb3ee49ab7:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
