	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn1_handle_with_sel
	.p2align	2
_fn1_handle_with_sel:
Lloh0:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_66d4c61f523c7074@PAGE
Lloh1:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_66d4c61f523c7074@PAGEOFF]
	b	_objc_msgSend
	.loh AdrpLdr	Lloh0, Lloh1

	.globl	_fn2_handle_alloc_init
	.p2align	2
_fn2_handle_alloc_init:
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
	bl	SYM(objc2::__macros::retain_semantics::init_fail::GENERATED_ID, 0)
	.loh AdrpLdrGotLdr	Lloh5, Lloh6, Lloh7
	.loh AdrpLdrGotLdr	Lloh2, Lloh3, Lloh4
	.loh AdrpAdd	Lloh8, Lloh9

	.globl	_fn3_use_generic
	.p2align	2
_fn3_use_generic:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x0
Lloh10:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_8d4579a56572fa21@PAGE
Lloh11:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_8d4579a56572fa21@PAGEOFF]
	adrp	x20, L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e@PAGE
	ldr	x2, [x20, L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e@PAGEOFF]
	bl	_objc_msgSend
Lloh12:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_d018146ff130bbd9@PAGE
Lloh13:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_d018146ff130bbd9@PAGEOFF]
	ldr	x2, [x20, L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e@PAGEOFF]
	mov	x0, x19
	bl	_objc_msgSend
Lloh14:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_6659046596384437@PAGE
Lloh15:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_6659046596384437@PAGEOFF]
	ldr	x2, [x20, L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e@PAGEOFF]
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_msgSend
	.loh AdrpLdr	Lloh14, Lloh15
	.loh AdrpLdr	Lloh12, Lloh13
	.loh AdrpLdr	Lloh10, Lloh11

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6659046596384437
L_OBJC_METH_VAR_NAME_6659046596384437:
	.asciz	"performSelector:"

	.globl	L_OBJC_METH_VAR_NAME_66d4c61f523c7074
L_OBJC_METH_VAR_NAME_66d4c61f523c7074:
	.asciz	"someSelector"

	.globl	L_OBJC_METH_VAR_NAME_8d4579a56572fa21
L_OBJC_METH_VAR_NAME_8d4579a56572fa21:
	.asciz	"performSelector:"

	.globl	L_OBJC_METH_VAR_NAME_d018146ff130bbd9
L_OBJC_METH_VAR_NAME_d018146ff130bbd9:
	.asciz	"performSelector:"

	.globl	L_OBJC_METH_VAR_NAME_e348e18690a1021e
L_OBJC_METH_VAR_NAME_e348e18690a1021e:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_6659046596384437
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_6659046596384437:
	.quad	L_OBJC_METH_VAR_NAME_6659046596384437

	.globl	L_OBJC_SELECTOR_REFERENCES_66d4c61f523c7074
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_66d4c61f523c7074:
	.quad	L_OBJC_METH_VAR_NAME_66d4c61f523c7074

	.globl	L_OBJC_SELECTOR_REFERENCES_8d4579a56572fa21
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_8d4579a56572fa21:
	.quad	L_OBJC_METH_VAR_NAME_8d4579a56572fa21

	.globl	L_OBJC_SELECTOR_REFERENCES_d018146ff130bbd9
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_d018146ff130bbd9:
	.quad	L_OBJC_METH_VAR_NAME_d018146ff130bbd9

	.globl	L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e:
	.quad	L_OBJC_METH_VAR_NAME_e348e18690a1021e

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_6659046596384437
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_6659046596384437:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_66d4c61f523c7074
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_66d4c61f523c7074:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_8d4579a56572fa21
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_8d4579a56572fa21:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_d018146ff130bbd9
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_d018146ff130bbd9:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_e348e18690a1021e
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_e348e18690a1021e:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].0:
	.asciz	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].1:
	.quad	l_anon.[ID].0
	.asciz	";\000\000\000\000\000\000\000\016\000\000\000\005\000\000"

.subsections_via_symbols
