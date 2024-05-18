	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_sel
	.p2align	2
_get_sel:
Lloh0:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7@PAGE
Lloh1:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.globl	_get_same_sel
	.p2align	2
_get_same_sel:
Lloh2:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513@PAGE
Lloh3:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh2, Lloh3

	.globl	_get_common_twice
	.p2align	2
_get_common_twice:
Lloh4:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_alloc@GOTPAGE
Lloh5:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_alloc@GOTPAGEOFF]
Lloh6:
	ldr	x0, [x8]
	mov	x1, x0
	ret
	.loh AdrpLdrGotLdr	Lloh4, Lloh5, Lloh6

	.globl	_get_different_sel
	.p2align	2
_get_different_sel:
Lloh7:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864@PAGE
Lloh8:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh7, Lloh8

	.globl	_unused_sel
	.p2align	2
_unused_sel:
	ret

	.globl	_use_fns
	.p2align	2
_use_fns:
Lloh9:
	adrp	x9, L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7@PAGE
Lloh10:
	ldr	x9, [x9, L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7@PAGEOFF]
Lloh11:
	adrp	x10, L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513@PAGE
Lloh12:
	ldr	x10, [x10, L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513@PAGEOFF]
Lloh13:
	adrp	x11, L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864@PAGE
Lloh14:
	ldr	x11, [x11, L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864@PAGEOFF]
Lloh15:
	adrp	x12, L_OBJC_SELECTOR_REFERENCES_408f5be8f4fd2627@PAGE
Lloh16:
	ldr	x12, [x12, L_OBJC_SELECTOR_REFERENCES_408f5be8f4fd2627@PAGEOFF]
	stp	x9, x10, [x8]
	stp	x11, x12, [x8, #16]
	ret
	.loh AdrpLdr	Lloh15, Lloh16
	.loh AdrpLdr	Lloh13, Lloh14
	.loh AdrpLdr	Lloh11, Lloh12
	.loh AdrpLdr	Lloh9, Lloh10

	.globl	_use_same_twice
	.p2align	2
_use_same_twice:
Lloh17:
	adrp	x9, L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7@PAGE
Lloh18:
	ldr	x9, [x9, L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7@PAGEOFF]
	stp	x9, x9, [x8]
	ret
	.loh AdrpLdr	Lloh17, Lloh18

	.globl	_use_in_loop
	.p2align	2
_use_in_loop:
	ret

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_caedaca3f40015a7
L_OBJC_METH_VAR_NAME_caedaca3f40015a7:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_caedaca3f40015a7:
	.quad	L_OBJC_METH_VAR_NAME_caedaca3f40015a7

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_caedaca3f40015a7
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_caedaca3f40015a7:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_a7c7f3067f40b513
L_OBJC_METH_VAR_NAME_a7c7f3067f40b513:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_a7c7f3067f40b513:
	.quad	L_OBJC_METH_VAR_NAME_a7c7f3067f40b513

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_a7c7f3067f40b513
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a7c7f3067f40b513:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_bae8570d40d73864
L_OBJC_METH_VAR_NAME_bae8570d40d73864:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_bae8570d40d73864:
	.quad	L_OBJC_METH_VAR_NAME_bae8570d40d73864

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_bae8570d40d73864
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_bae8570d40d73864:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_9c1b77e8cf40622d
L_OBJC_METH_VAR_NAME_9c1b77e8cf40622d:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_9c1b77e8cf40622d
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_9c1b77e8cf40622d:
	.quad	L_OBJC_METH_VAR_NAME_9c1b77e8cf40622d

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_9c1b77e8cf40622d
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_9c1b77e8cf40622d:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_408f5be8f4fd2627
L_OBJC_METH_VAR_NAME_408f5be8f4fd2627:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_408f5be8f4fd2627
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_408f5be8f4fd2627:
	.quad	L_OBJC_METH_VAR_NAME_408f5be8f4fd2627

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_408f5be8f4fd2627
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_408f5be8f4fd2627:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_82483a8131827890
L_OBJC_METH_VAR_NAME_82483a8131827890:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_82483a8131827890
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_82483a8131827890:
	.quad	L_OBJC_METH_VAR_NAME_82483a8131827890

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_82483a8131827890
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_82483a8131827890:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
