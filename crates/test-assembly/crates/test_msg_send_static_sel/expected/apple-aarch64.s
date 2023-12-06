	.section	__TEXT,__text,regular,pure_instructions
	.globl	_handle_with_sel
	.p2align	2
_handle_with_sel:
Lloh0:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_ad1b815073641351@PAGE
Lloh1:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_ad1b815073641351@PAGEOFF]
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
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_124acdf5c403f4b7@PAGE
Lloh8:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_124acdf5c403f4b7@PAGEOFF]
	adrp	x20, L_OBJC_SELECTOR_REFERENCES_f0e0414fd9875b81@PAGE
	ldr	x2, [x20, L_OBJC_SELECTOR_REFERENCES_f0e0414fd9875b81@PAGEOFF]
	bl	_objc_msgSend
Lloh9:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_2949aa14a0c1ed09@PAGE
Lloh10:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_2949aa14a0c1ed09@PAGEOFF]
	ldr	x2, [x20, L_OBJC_SELECTOR_REFERENCES_f0e0414fd9875b81@PAGEOFF]
	mov	x0, x19
	bl	_objc_msgSend
Lloh11:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_cfad9aae979f14dd@PAGE
Lloh12:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_cfad9aae979f14dd@PAGEOFF]
	ldr	x2, [x20, L_OBJC_SELECTOR_REFERENCES_f0e0414fd9875b81@PAGEOFF]
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
	.globl	L_OBJC_METH_VAR_NAME_ad1b815073641351
L_OBJC_METH_VAR_NAME_ad1b815073641351:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_ad1b815073641351
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_ad1b815073641351:
	.quad	L_OBJC_METH_VAR_NAME_ad1b815073641351

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_ad1b815073641351
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ad1b815073641351:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_f0e0414fd9875b81
L_OBJC_METH_VAR_NAME_f0e0414fd9875b81:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_f0e0414fd9875b81
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_f0e0414fd9875b81:
	.quad	L_OBJC_METH_VAR_NAME_f0e0414fd9875b81

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f0e0414fd9875b81
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f0e0414fd9875b81:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_124acdf5c403f4b7
L_OBJC_METH_VAR_NAME_124acdf5c403f4b7:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_124acdf5c403f4b7
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_124acdf5c403f4b7:
	.quad	L_OBJC_METH_VAR_NAME_124acdf5c403f4b7

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_124acdf5c403f4b7
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_124acdf5c403f4b7:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2949aa14a0c1ed09
L_OBJC_METH_VAR_NAME_2949aa14a0c1ed09:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2949aa14a0c1ed09
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_2949aa14a0c1ed09:
	.quad	L_OBJC_METH_VAR_NAME_2949aa14a0c1ed09

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2949aa14a0c1ed09
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2949aa14a0c1ed09:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_cfad9aae979f14dd
L_OBJC_METH_VAR_NAME_cfad9aae979f14dd:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_cfad9aae979f14dd
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_cfad9aae979f14dd:
	.quad	L_OBJC_METH_VAR_NAME_cfad9aae979f14dd

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_cfad9aae979f14dd
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_cfad9aae979f14dd:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
