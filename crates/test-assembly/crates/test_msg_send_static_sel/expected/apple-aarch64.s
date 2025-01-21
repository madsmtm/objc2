	.section	__TEXT,__text,regular,pure_instructions
	.globl	_handle_with_sel
	.p2align	2
_handle_with_sel:
Lloh0:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_80f1580ed33ec51b@PAGE
Lloh1:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_80f1580ed33ec51b@PAGEOFF]
	b	_objc_msgSend
	.loh AdrpLdr	Lloh0, Lloh1

	.globl	_handle_alloc_init
	.p2align	2
_handle_alloc_init:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh2:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_alloc@GOTPAGE
Lloh3:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_alloc@GOTPAGEOFF]
Lloh4:
	ldr	x1, [x8]
	bl	_objc_msgSend
	mov	x19, x0
Lloh5:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh6:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh7:
	ldr	x20, [x8]
	mov	x1, x20
	bl	_objc_msgSend
	cbz	x0, LBB1_2
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB1_2:
Lloh8:
	adrp	x2, l_anon.[ID].1@PAGE
Lloh9:
	add	x2, x2, l_anon.[ID].1@PAGEOFF
	mov	x0, x19
	mov	x1, x20
	bl	SYM(objc2::__macro_helpers::retain_semantics::init_fail::GENERATED_ID, 0)
	.loh AdrpLdrGotLdr	Lloh5, Lloh6, Lloh7
	.loh AdrpLdrGotLdr	Lloh2, Lloh3, Lloh4
	.loh AdrpAdd	Lloh8, Lloh9

	.globl	_use_generic
	.p2align	2
_use_generic:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x0
Lloh10:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_67bf3e41c7e639a3@PAGE
Lloh11:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_67bf3e41c7e639a3@PAGEOFF]
	adrp	x20, L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5@PAGE
	ldr	x2, [x20, L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5@PAGEOFF]
	bl	_objc_msgSend
Lloh12:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_2c2c9a8191012941@PAGE
Lloh13:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_2c2c9a8191012941@PAGEOFF]
	ldr	x2, [x20, L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5@PAGEOFF]
	mov	x0, x19
	bl	_objc_msgSend
Lloh14:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_993d94b40d47ed52@PAGE
Lloh15:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_993d94b40d47ed52@PAGEOFF]
	ldr	x2, [x20, L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5@PAGEOFF]
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_msgSend
	.loh AdrpLdr	Lloh14, Lloh15
	.loh AdrpLdr	Lloh12, Lloh13
	.loh AdrpLdr	Lloh10, Lloh11

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].1:
	.quad	l_anon.[ID].0
	.asciz	";\000\000\000\000\000\000\000\016\000\000\000\005\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_80f1580ed33ec51b
L_OBJC_METH_VAR_NAME_80f1580ed33ec51b:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_80f1580ed33ec51b
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_80f1580ed33ec51b:
	.quad	L_OBJC_METH_VAR_NAME_80f1580ed33ec51b

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_80f1580ed33ec51b
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_80f1580ed33ec51b:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_91c006d97540f4b5
L_OBJC_METH_VAR_NAME_91c006d97540f4b5:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5:
	.quad	L_OBJC_METH_VAR_NAME_91c006d97540f4b5

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_91c006d97540f4b5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_91c006d97540f4b5:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_67bf3e41c7e639a3
L_OBJC_METH_VAR_NAME_67bf3e41c7e639a3:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_67bf3e41c7e639a3
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_67bf3e41c7e639a3:
	.quad	L_OBJC_METH_VAR_NAME_67bf3e41c7e639a3

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_67bf3e41c7e639a3
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_67bf3e41c7e639a3:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2c2c9a8191012941
L_OBJC_METH_VAR_NAME_2c2c9a8191012941:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_2c2c9a8191012941
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_2c2c9a8191012941:
	.quad	L_OBJC_METH_VAR_NAME_2c2c9a8191012941

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2c2c9a8191012941
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2c2c9a8191012941:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_993d94b40d47ed52
L_OBJC_METH_VAR_NAME_993d94b40d47ed52:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_993d94b40d47ed52
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_993d94b40d47ed52:
	.quad	L_OBJC_METH_VAR_NAME_993d94b40d47ed52

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_993d94b40d47ed52
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_993d94b40d47ed52:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
