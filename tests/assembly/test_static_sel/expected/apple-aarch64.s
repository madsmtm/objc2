	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_sel
	.p2align	2
_get_sel:
	b	__RNvNvCsai54Y9dtG4h_15test_static_sel7get_sel22objc_static_workaround

	.globl	_get_same_sel
	.p2align	2
_get_same_sel:
	b	__RNvNvCsai54Y9dtG4h_15test_static_sel12get_same_sel22objc_static_workaround

	.globl	_get_common_twice
	.p2align	2
_get_common_twice:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	bl	__RNvNvCsai54Y9dtG4h_15test_static_sel16get_common_twice22objc_static_workaround
	mov	x19, x0
	bl	__RNvNvCsai54Y9dtG4h_15test_static_sel16get_common_twices_22objc_static_workaround
	mov	x1, x0
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret

	.globl	_get_different_sel
	.p2align	2
_get_different_sel:
	b	__RNvNvCsai54Y9dtG4h_15test_static_sel17get_different_sel22objc_static_workaround

	.globl	_unused_sel
	.p2align	2
_unused_sel:
	ret

	.globl	_use_fns
	.p2align	2
_use_fns:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x8
	bl	__RNvNvCsai54Y9dtG4h_15test_static_sel7get_sel22objc_static_workaround
	mov	x20, x0
	bl	__RNvNvCsai54Y9dtG4h_15test_static_sel12get_same_sel22objc_static_workaround
	mov	x21, x0
	bl	__RNvNvCsai54Y9dtG4h_15test_static_sel17get_different_sel22objc_static_workaround
	mov	x22, x0
	bl	__RNvNvCsai54Y9dtG4h_15test_static_sel7use_fns22objc_static_workaround
	stp	x20, x21, [x19]
	stp	x22, x0, [x19, #16]
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret

	.globl	_use_same_twice
	.p2align	2
_use_same_twice:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x8
	bl	__RNvNvCsai54Y9dtG4h_15test_static_sel7get_sel22objc_static_workaround
	stp	x0, x0, [x19]
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret

	.globl	_use_in_loop
	.p2align	2
_use_in_loop:
	ret

	.p2align	2
__RNvNvCsai54Y9dtG4h_15test_static_sel7get_sel22objc_static_workaround:
Lloh0:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9@PAGE
Lloh1:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_68381ba894e318e9@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.p2align	2
__RNvNvCsai54Y9dtG4h_15test_static_sel12get_same_sel22objc_static_workaround:
Lloh2:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35@PAGE
Lloh3:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_cd2fd6e7d2adcc35@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh2, Lloh3

	.p2align	2
__RNvNvCsai54Y9dtG4h_15test_static_sel16get_common_twice22objc_static_workaround:
Lloh4:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_e4a45d49bfea5d77@PAGE
Lloh5:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_e4a45d49bfea5d77@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh4, Lloh5

	.p2align	2
__RNvNvCsai54Y9dtG4h_15test_static_sel16get_common_twices_22objc_static_workaround:
Lloh6:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_33db9f67352fe9a7@PAGE
Lloh7:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_33db9f67352fe9a7@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh6, Lloh7

	.p2align	2
__RNvNvCsai54Y9dtG4h_15test_static_sel17get_different_sel22objc_static_workaround:
Lloh8:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0@PAGE
Lloh9:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_bb37877368f0b7a0@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh8, Lloh9

	.p2align	2
__RNvNvCsai54Y9dtG4h_15test_static_sel7use_fns22objc_static_workaround:
Lloh10:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99@PAGE
Lloh11:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_5419c3f7fc0a6f99@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh10, Lloh11

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
	.globl	L_OBJC_IMAGE_INFO_e4a45d49bfea5d77
	.p2align	2
L_OBJC_IMAGE_INFO_e4a45d49bfea5d77:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e4a45d49bfea5d77
L_OBJC_METH_VAR_NAME_e4a45d49bfea5d77:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_e4a45d49bfea5d77
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_e4a45d49bfea5d77:
	.quad	L_OBJC_METH_VAR_NAME_e4a45d49bfea5d77

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_33db9f67352fe9a7
	.p2align	2
L_OBJC_IMAGE_INFO_33db9f67352fe9a7:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_33db9f67352fe9a7
L_OBJC_METH_VAR_NAME_33db9f67352fe9a7:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_33db9f67352fe9a7
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_33db9f67352fe9a7:
	.quad	L_OBJC_METH_VAR_NAME_33db9f67352fe9a7

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
