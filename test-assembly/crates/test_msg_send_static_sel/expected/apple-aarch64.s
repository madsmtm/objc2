	.section	__TEXT,__text,regular,pure_instructions
	.globl	_handle_with_sel
	.p2align	2
_handle_with_sel:
Lloh0:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc@PAGE
Lloh1:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc@PAGEOFF]
	b	_objc_msgSend
	.loh AdrpLdr	Lloh0, Lloh1

	.globl	_handle_alloc_init
	.p2align	2
_handle_alloc_init:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh2:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_a4d24db6a5648d44@GOTPAGE
Lloh3:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_a4d24db6a5648d44@GOTPAGEOFF]
Lloh4:
	ldr	x19, [x8]
Lloh5:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_234aca269670483c@GOTPAGE
Lloh6:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_234aca269670483c@GOTPAGEOFF]
Lloh7:
	ldr	x1, [x8]
	bl	_objc_msgSend
	mov	x1, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_msgSend
	.loh AdrpLdrGotLdr	Lloh5, Lloh6, Lloh7
	.loh AdrpLdrGotLdr	Lloh2, Lloh3, Lloh4

	.globl	_use_generic
	.p2align	2
_use_generic:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x0
Lloh8:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_f16064a6f68ca673@PAGE
Lloh9:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_f16064a6f68ca673@PAGEOFF]
	adrp	x20, L_OBJC_SELECTOR_REFERENCES_457d234345d46cbe@PAGE
	ldr	x2, [x20, L_OBJC_SELECTOR_REFERENCES_457d234345d46cbe@PAGEOFF]
	bl	_objc_msgSend
Lloh10:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_9f134b97cb598446@PAGE
Lloh11:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_9f134b97cb598446@PAGEOFF]
	ldr	x2, [x20, L_OBJC_SELECTOR_REFERENCES_457d234345d46cbe@PAGEOFF]
	mov	x0, x19
	bl	_objc_msgSend
Lloh12:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_e76e01e8b2327e5d@PAGE
Lloh13:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_e76e01e8b2327e5d@PAGEOFF]
	ldr	x2, [x20, L_OBJC_SELECTOR_REFERENCES_457d234345d46cbe@PAGEOFF]
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_msgSend
	.loh AdrpLdr	Lloh12, Lloh13
	.loh AdrpLdr	Lloh10, Lloh11
	.loh AdrpLdr	Lloh8, Lloh9

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_044375a4329d08dc
	.p2align	2
L_OBJC_IMAGE_INFO_044375a4329d08dc:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_044375a4329d08dc
L_OBJC_METH_VAR_NAME_044375a4329d08dc:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc:
	.quad	L_OBJC_METH_VAR_NAME_044375a4329d08dc

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_457d234345d46cbe
	.p2align	2
L_OBJC_IMAGE_INFO_457d234345d46cbe:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_457d234345d46cbe
L_OBJC_METH_VAR_NAME_457d234345d46cbe:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_457d234345d46cbe
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_457d234345d46cbe:
	.quad	L_OBJC_METH_VAR_NAME_457d234345d46cbe

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f16064a6f68ca673
	.p2align	2
L_OBJC_IMAGE_INFO_f16064a6f68ca673:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_f16064a6f68ca673
L_OBJC_METH_VAR_NAME_f16064a6f68ca673:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_f16064a6f68ca673
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_f16064a6f68ca673:
	.quad	L_OBJC_METH_VAR_NAME_f16064a6f68ca673

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_9f134b97cb598446
	.p2align	2
L_OBJC_IMAGE_INFO_9f134b97cb598446:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_9f134b97cb598446
L_OBJC_METH_VAR_NAME_9f134b97cb598446:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_9f134b97cb598446
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_9f134b97cb598446:
	.quad	L_OBJC_METH_VAR_NAME_9f134b97cb598446

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_e76e01e8b2327e5d
	.p2align	2
L_OBJC_IMAGE_INFO_e76e01e8b2327e5d:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e76e01e8b2327e5d
L_OBJC_METH_VAR_NAME_e76e01e8b2327e5d:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_e76e01e8b2327e5d
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_e76e01e8b2327e5d:
	.quad	L_OBJC_METH_VAR_NAME_e76e01e8b2327e5d

.subsections_via_symbols
