	.section	__TEXT,__text,regular,pure_instructions
	.globl	_handle_with_sel
	.p2align	2
_handle_with_sel:
Lloh0:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_ac93c21013e05a7d@PAGE
Lloh1:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_ac93c21013e05a7d@PAGEOFF]
	b	_objc_msgSend
	.loh AdrpLdr	Lloh0, Lloh1

	.globl	_handle_alloc_init
	.p2align	2
_handle_alloc_init:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	bl	_objc_alloc
	mov	x19, x0
Lloh2:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh3:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh4:
	ldr	x20, [x8]
	mov	x1, x20
	bl	_objc_msgSend
	cbz	x0, LBB1_2
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB1_2:
Lloh5:
	adrp	x2, l_anon.[ID].1@PAGE
Lloh6:
	add	x2, x2, l_anon.[ID].1@PAGEOFF
	mov	x0, x19
	mov	x1, x20
	bl	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<3_u8> as objc2::__macro_helpers::msg_send_id::MsgSendIdFailed>::failed::GENERATED_ID, 0)
	.loh AdrpLdrGotLdr	Lloh2, Lloh3, Lloh4
	.loh AdrpAdd	Lloh5, Lloh6

	.globl	_use_generic
	.p2align	2
_use_generic:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x0
Lloh7:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_df2cc6f80edda422@PAGE
Lloh8:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_df2cc6f80edda422@PAGEOFF]
	adrp	x20, L_OBJC_SELECTOR_REFERENCES_8071e784bfc1c0df@PAGE
	ldr	x2, [x20, L_OBJC_SELECTOR_REFERENCES_8071e784bfc1c0df@PAGEOFF]
	bl	_objc_msgSend
Lloh9:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_e520bf80aade9209@PAGE
Lloh10:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_e520bf80aade9209@PAGEOFF]
	ldr	x2, [x20, L_OBJC_SELECTOR_REFERENCES_8071e784bfc1c0df@PAGEOFF]
	mov	x0, x19
	bl	_objc_msgSend
Lloh11:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_bab7ff70ff0c78f0@PAGE
Lloh12:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_bab7ff70ff0c78f0@PAGEOFF]
	ldr	x2, [x20, L_OBJC_SELECTOR_REFERENCES_8071e784bfc1c0df@PAGEOFF]
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_msgSend
	.loh AdrpLdr	Lloh11, Lloh12
	.loh AdrpLdr	Lloh9, Lloh10
	.loh AdrpLdr	Lloh7, Lloh8

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].1:
	.quad	l_anon.[ID].0
	.asciz	";\000\000\000\000\000\000\000\016\000\000\000\005\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_ac93c21013e05a7d
L_OBJC_METH_VAR_NAME_ac93c21013e05a7d:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_ac93c21013e05a7d
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_ac93c21013e05a7d:
	.quad	L_OBJC_METH_VAR_NAME_ac93c21013e05a7d

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_ac93c21013e05a7d
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ac93c21013e05a7d:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_8071e784bfc1c0df
L_OBJC_METH_VAR_NAME_8071e784bfc1c0df:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_8071e784bfc1c0df
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_8071e784bfc1c0df:
	.quad	L_OBJC_METH_VAR_NAME_8071e784bfc1c0df

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_8071e784bfc1c0df
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_8071e784bfc1c0df:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_df2cc6f80edda422
L_OBJC_METH_VAR_NAME_df2cc6f80edda422:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_df2cc6f80edda422
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_df2cc6f80edda422:
	.quad	L_OBJC_METH_VAR_NAME_df2cc6f80edda422

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_df2cc6f80edda422
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_df2cc6f80edda422:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e520bf80aade9209
L_OBJC_METH_VAR_NAME_e520bf80aade9209:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_e520bf80aade9209
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_e520bf80aade9209:
	.quad	L_OBJC_METH_VAR_NAME_e520bf80aade9209

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_e520bf80aade9209
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_e520bf80aade9209:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_bab7ff70ff0c78f0
L_OBJC_METH_VAR_NAME_bab7ff70ff0c78f0:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_bab7ff70ff0c78f0
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_bab7ff70ff0c78f0:
	.quad	L_OBJC_METH_VAR_NAME_bab7ff70ff0c78f0

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_bab7ff70ff0c78f0
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_bab7ff70ff0c78f0:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
