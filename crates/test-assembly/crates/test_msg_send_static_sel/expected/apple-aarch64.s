	.section	__TEXT,__text,regular,pure_instructions
	.globl	_handle_with_sel
	.p2align	2
_handle_with_sel:
Lloh0:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_77d2b75bddfbef7c@PAGE
Lloh1:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_77d2b75bddfbef7c@PAGEOFF]
	b	_objc_msgSend
	.loh AdrpLdr	Lloh0, Lloh1

	.globl	_handle_alloc_init
	.p2align	2
_handle_alloc_init:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh2:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh3:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh4:
	ldr	x19, [x8]
Lloh5:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_alloc@GOTPAGE
Lloh6:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_alloc@GOTPAGEOFF]
Lloh7:
	ldr	x1, [x8]
	bl	_objc_msgSend
	mov	x20, x0
	mov	x1, x19
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
	mov	x0, x20
	mov	x1, x19
	bl	SYM(<objc2::__macro_helpers::RetainSemantics<3_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)
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
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_038d21a6277de1da@PAGE
Lloh11:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_038d21a6277de1da@PAGEOFF]
	adrp	x20, L_OBJC_SELECTOR_REFERENCES_379095321e06c060@PAGE
	ldr	x2, [x20, L_OBJC_SELECTOR_REFERENCES_379095321e06c060@PAGEOFF]
	bl	_objc_msgSend
Lloh12:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_573c1e9c42ae1ea1@PAGE
Lloh13:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_573c1e9c42ae1ea1@PAGEOFF]
	ldr	x2, [x20, L_OBJC_SELECTOR_REFERENCES_379095321e06c060@PAGEOFF]
	mov	x0, x19
	bl	_objc_msgSend
Lloh14:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_9885c1be4d03110d@PAGE
Lloh15:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_9885c1be4d03110d@PAGEOFF]
	ldr	x2, [x20, L_OBJC_SELECTOR_REFERENCES_379095321e06c060@PAGEOFF]
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
	.p2align	3
l_anon.[ID].1:
	.quad	l_anon.[ID].0
	.asciz	";\000\000\000\000\000\000\000\016\000\000\000\005\000\000"

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_77d2b75bddfbef7c
	.p2align	2
L_OBJC_IMAGE_INFO_77d2b75bddfbef7c:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_77d2b75bddfbef7c
L_OBJC_METH_VAR_NAME_77d2b75bddfbef7c:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_77d2b75bddfbef7c
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_77d2b75bddfbef7c:
	.quad	L_OBJC_METH_VAR_NAME_77d2b75bddfbef7c

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_379095321e06c060
	.p2align	2
L_OBJC_IMAGE_INFO_379095321e06c060:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_379095321e06c060
L_OBJC_METH_VAR_NAME_379095321e06c060:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_379095321e06c060
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_379095321e06c060:
	.quad	L_OBJC_METH_VAR_NAME_379095321e06c060

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_038d21a6277de1da
	.p2align	2
L_OBJC_IMAGE_INFO_038d21a6277de1da:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_038d21a6277de1da
L_OBJC_METH_VAR_NAME_038d21a6277de1da:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_038d21a6277de1da
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_038d21a6277de1da:
	.quad	L_OBJC_METH_VAR_NAME_038d21a6277de1da

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_573c1e9c42ae1ea1
	.p2align	2
L_OBJC_IMAGE_INFO_573c1e9c42ae1ea1:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_573c1e9c42ae1ea1
L_OBJC_METH_VAR_NAME_573c1e9c42ae1ea1:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_573c1e9c42ae1ea1
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_573c1e9c42ae1ea1:
	.quad	L_OBJC_METH_VAR_NAME_573c1e9c42ae1ea1

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_9885c1be4d03110d
	.p2align	2
L_OBJC_IMAGE_INFO_9885c1be4d03110d:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_9885c1be4d03110d
L_OBJC_METH_VAR_NAME_9885c1be4d03110d:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_9885c1be4d03110d
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_9885c1be4d03110d:
	.quad	L_OBJC_METH_VAR_NAME_9885c1be4d03110d

.subsections_via_symbols
