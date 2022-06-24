	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_sel
	.p2align	2
_get_sel:
Lloh0:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9@PAGE
Lloh1:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.globl	_get_same_sel
	.p2align	2
_get_same_sel:
Lloh2:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35@PAGE
Lloh3:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh2, Lloh3

	.globl	_get_common_twice
	.p2align	2
_get_common_twice:
Lloh4:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_1678d2f7468155d2@GOTPAGE
Lloh5:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_1678d2f7468155d2@GOTPAGEOFF]
Lloh6:
	ldr	x0, [x8]
	mov	x1, x0
	ret
	.loh AdrpLdrGotLdr	Lloh4, Lloh5, Lloh6

	.globl	_get_different_sel
	.p2align	2
_get_different_sel:
Lloh7:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0@PAGE
Lloh8:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0@PAGEOFF]
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
	adrp	x9, L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9@PAGE
Lloh10:
	ldr	x9, [x9, L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9@PAGEOFF]
Lloh11:
	adrp	x10, L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35@PAGE
Lloh12:
	ldr	x10, [x10, L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35@PAGEOFF]
Lloh13:
	adrp	x11, L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0@PAGE
Lloh14:
	ldr	x11, [x11, L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0@PAGEOFF]
Lloh15:
	adrp	x12, L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99@PAGE
Lloh16:
	ldr	x12, [x12, L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99@PAGEOFF]
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
	adrp	x9, L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9@PAGE
Lloh18:
	ldr	x9, [x9, L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9@PAGEOFF]
	stp	x9, x9, [x8]
	ret
	.loh AdrpLdr	Lloh17, Lloh18

	.globl	_use_in_loop
	.p2align	2
_use_in_loop:
	ret

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_68381ba894e318e9
	.p2align	2
L_OBJC_IMAGE_INFO_68381ba894e318e9:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_68381ba894e318e9
L_OBJC_METH_VAR_NAME_68381ba894e318e9:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9:
	.quad	L_OBJC_METH_VAR_NAME_68381ba894e318e9

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_cd2fd6e7d2adcc35
	.p2align	2
L_OBJC_IMAGE_INFO_cd2fd6e7d2adcc35:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35
L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35:
	.asciz	"simple"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35:
	.quad	L_OBJC_METH_VAR_NAME_cd2fd6e7d2adcc35

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_bb37877368f0b7a0
	.p2align	2
L_OBJC_IMAGE_INFO_bb37877368f0b7a0:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_bb37877368f0b7a0
L_OBJC_METH_VAR_NAME_bb37877368f0b7a0:
	.asciz	"i:am:different:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0:
	.quad	L_OBJC_METH_VAR_NAME_bb37877368f0b7a0

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2c505e110d181b25
	.p2align	2
L_OBJC_IMAGE_INFO_2c505e110d181b25:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2c505e110d181b25
L_OBJC_METH_VAR_NAME_2c505e110d181b25:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2c505e110d181b25
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_2c505e110d181b25:
	.quad	L_OBJC_METH_VAR_NAME_2c505e110d181b25

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_5419c3f7fc0a6f99
	.p2align	2
L_OBJC_IMAGE_INFO_5419c3f7fc0a6f99:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_5419c3f7fc0a6f99
L_OBJC_METH_VAR_NAME_5419c3f7fc0a6f99:
	.asciz	"fourthSel"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99:
	.quad	L_OBJC_METH_VAR_NAME_5419c3f7fc0a6f99

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f46908e864c86c6b
	.p2align	2
L_OBJC_IMAGE_INFO_f46908e864c86c6b:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_f46908e864c86c6b
L_OBJC_METH_VAR_NAME_f46908e864c86c6b:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_f46908e864c86c6b
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_f46908e864c86c6b:
	.quad	L_OBJC_METH_VAR_NAME_f46908e864c86c6b

.subsections_via_symbols
