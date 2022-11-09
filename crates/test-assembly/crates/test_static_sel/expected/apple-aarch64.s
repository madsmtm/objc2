	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_sel
	.p2align	2
_get_sel:
Lloh0:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0@PAGE
Lloh1:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.globl	_get_same_sel
	.p2align	2
_get_same_sel:
Lloh2:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83@PAGE
Lloh3:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83@PAGEOFF]
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
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_25911857653c680c@PAGE
Lloh8:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_25911857653c680c@PAGEOFF]
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
	adrp	x9, L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0@PAGE
Lloh10:
	ldr	x9, [x9, L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0@PAGEOFF]
Lloh11:
	adrp	x10, L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83@PAGE
Lloh12:
	ldr	x10, [x10, L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83@PAGEOFF]
Lloh13:
	adrp	x11, L_OBJC_SELECTOR_REFERENCES_25911857653c680c@PAGE
Lloh14:
	ldr	x11, [x11, L_OBJC_SELECTOR_REFERENCES_25911857653c680c@PAGEOFF]
Lloh15:
	adrp	x12, L_OBJC_SELECTOR_REFERENCES_acb291d82e56f534@PAGE
Lloh16:
	ldr	x12, [x12, L_OBJC_SELECTOR_REFERENCES_acb291d82e56f534@PAGEOFF]
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
	adrp	x9, L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0@PAGE
Lloh18:
	ldr	x9, [x9, L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0@PAGEOFF]
	stp	x9, x9, [x8]
	ret
	.loh AdrpLdr	Lloh17, Lloh18

	.globl	_use_in_loop
	.p2align	2
_use_in_loop:
	ret

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2ff5c2d33acc98c0
	.p2align	2
L_OBJC_IMAGE_INFO_2ff5c2d33acc98c0:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2ff5c2d33acc98c0
L_OBJC_METH_VAR_NAME_2ff5c2d33acc98c0:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_2ff5c2d33acc98c0:
	.quad	L_OBJC_METH_VAR_NAME_2ff5c2d33acc98c0

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_6e17eb9d3fa7fa83
	.p2align	2
L_OBJC_IMAGE_INFO_6e17eb9d3fa7fa83:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6e17eb9d3fa7fa83
L_OBJC_METH_VAR_NAME_6e17eb9d3fa7fa83:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_6e17eb9d3fa7fa83:
	.quad	L_OBJC_METH_VAR_NAME_6e17eb9d3fa7fa83

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_25911857653c680c
	.p2align	2
L_OBJC_IMAGE_INFO_25911857653c680c:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_25911857653c680c
L_OBJC_METH_VAR_NAME_25911857653c680c:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_25911857653c680c
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_25911857653c680c:
	.quad	L_OBJC_METH_VAR_NAME_25911857653c680c

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_baa3c09478169afc
	.p2align	2
L_OBJC_IMAGE_INFO_baa3c09478169afc:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_baa3c09478169afc
L_OBJC_METH_VAR_NAME_baa3c09478169afc:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_baa3c09478169afc
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_baa3c09478169afc:
	.quad	L_OBJC_METH_VAR_NAME_baa3c09478169afc

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_acb291d82e56f534
	.p2align	2
L_OBJC_IMAGE_INFO_acb291d82e56f534:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_acb291d82e56f534
L_OBJC_METH_VAR_NAME_acb291d82e56f534:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_acb291d82e56f534
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_acb291d82e56f534:
	.quad	L_OBJC_METH_VAR_NAME_acb291d82e56f534

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_c831c01ba82dcc2e
	.p2align	2
L_OBJC_IMAGE_INFO_c831c01ba82dcc2e:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_c831c01ba82dcc2e
L_OBJC_METH_VAR_NAME_c831c01ba82dcc2e:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_c831c01ba82dcc2e
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_c831c01ba82dcc2e:
	.quad	L_OBJC_METH_VAR_NAME_c831c01ba82dcc2e

.subsections_via_symbols
