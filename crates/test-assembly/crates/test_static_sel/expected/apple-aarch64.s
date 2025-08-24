	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn1_get_sel
	.p2align	2
_fn1_get_sel:
Lloh0:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb@PAGE
Lloh1:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.globl	_fn2_get_same_sel
	.p2align	2
_fn2_get_same_sel:
Lloh2:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f@PAGE
Lloh3:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh2, Lloh3

	.globl	_fn3_get_common_twice
	.p2align	2
_fn3_get_common_twice:
Lloh4:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_alloc@GOTPAGE
Lloh5:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_alloc@GOTPAGEOFF]
Lloh6:
	ldr	x0, [x8]
	mov	x1, x0
	ret
	.loh AdrpLdrGotLdr	Lloh4, Lloh5, Lloh6

	.globl	_fn4_get_different_sel
	.p2align	2
_fn4_get_different_sel:
Lloh7:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85@PAGE
Lloh8:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh7, Lloh8

	.globl	_fn5_unused_sel
	.p2align	2
_fn5_unused_sel:
	ret

	.globl	_fn6_use_fns
	.p2align	2
_fn6_use_fns:
Lloh9:
	adrp	x9, L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb@PAGE
Lloh10:
	ldr	x9, [x9, L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb@PAGEOFF]
Lloh11:
	adrp	x10, L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f@PAGE
Lloh12:
	ldr	x10, [x10, L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f@PAGEOFF]
Lloh13:
	adrp	x11, L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85@PAGE
Lloh14:
	ldr	x11, [x11, L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85@PAGEOFF]
Lloh15:
	adrp	x12, L_OBJC_SELECTOR_REFERENCES_e91a5376ddae3cfc@PAGE
Lloh16:
	ldr	x12, [x12, L_OBJC_SELECTOR_REFERENCES_e91a5376ddae3cfc@PAGEOFF]
	stp	x9, x10, [x8]
	stp	x11, x12, [x8, #16]
	ret
	.loh AdrpLdr	Lloh15, Lloh16
	.loh AdrpLdr	Lloh13, Lloh14
	.loh AdrpLdr	Lloh11, Lloh12
	.loh AdrpLdr	Lloh9, Lloh10

	.globl	_fn7_use_same_twice
	.p2align	2
_fn7_use_same_twice:
Lloh17:
	adrp	x9, L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb@PAGE
Lloh18:
	ldr	x9, [x9, L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb@PAGEOFF]
	stp	x9, x9, [x8]
	ret
	.loh AdrpLdr	Lloh17, Lloh18

	.globl	_fn8_use_in_loop
	.p2align	2
_fn8_use_in_loop:
	ret

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2668dedcc69bf8fb
L_OBJC_METH_VAR_NAME_2668dedcc69bf8fb:
	.asciz	"simple"

	.globl	L_OBJC_METH_VAR_NAME_29d0234b9445d447
L_OBJC_METH_VAR_NAME_29d0234b9445d447:
	.asciz	"unused"

	.globl	L_OBJC_METH_VAR_NAME_9303807037ba4f9f
L_OBJC_METH_VAR_NAME_9303807037ba4f9f:
	.asciz	"simple"

	.globl	L_OBJC_METH_VAR_NAME_a23daf114eba1518
L_OBJC_METH_VAR_NAME_a23daf114eba1518:
	.asciz	"loopedSelector"

	.globl	L_OBJC_METH_VAR_NAME_e91a5376ddae3cfc
L_OBJC_METH_VAR_NAME_e91a5376ddae3cfc:
	.asciz	"fourthSel"

	.globl	L_OBJC_METH_VAR_NAME_ff60dae1998e0c85
L_OBJC_METH_VAR_NAME_ff60dae1998e0c85:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_2668dedcc69bf8fb:
	.quad	L_OBJC_METH_VAR_NAME_2668dedcc69bf8fb

	.globl	L_OBJC_SELECTOR_REFERENCES_29d0234b9445d447
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_29d0234b9445d447:
	.quad	L_OBJC_METH_VAR_NAME_29d0234b9445d447

	.globl	L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_9303807037ba4f9f:
	.quad	L_OBJC_METH_VAR_NAME_9303807037ba4f9f

	.globl	L_OBJC_SELECTOR_REFERENCES_a23daf114eba1518
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_a23daf114eba1518:
	.quad	L_OBJC_METH_VAR_NAME_a23daf114eba1518

	.globl	L_OBJC_SELECTOR_REFERENCES_e91a5376ddae3cfc
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_e91a5376ddae3cfc:
	.quad	L_OBJC_METH_VAR_NAME_e91a5376ddae3cfc

	.globl	L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_ff60dae1998e0c85:
	.quad	L_OBJC_METH_VAR_NAME_ff60dae1998e0c85

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2668dedcc69bf8fb
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2668dedcc69bf8fb:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_29d0234b9445d447
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_29d0234b9445d447:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_9303807037ba4f9f
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_9303807037ba4f9f:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_a23daf114eba1518
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a23daf114eba1518:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_e91a5376ddae3cfc
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_e91a5376ddae3cfc:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_ff60dae1998e0c85
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ff60dae1998e0c85:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
