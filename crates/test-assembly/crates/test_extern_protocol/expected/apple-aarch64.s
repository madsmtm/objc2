	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_protocol
	.p2align	2
_get_protocol:
Lloh0:
	adrp	x0, l_anon.[ID].0@PAGE
Lloh1:
	add	x0, x0, l_anon.[ID].0@PAGEOFF
	mov	w1, #10
	b	SYM(objc2::runtime::Protocol::get::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh0, Lloh1

	.globl	_dyn_call
	.p2align	2
_dyn_call:
Lloh2:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_36d5bd8a3470dcf9@PAGE
Lloh3:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_36d5bd8a3470dcf9@PAGEOFF]
	b	_objc_msgSend
	.loh AdrpLdr	Lloh2, Lloh3

	.globl	_dyn_consume
	.p2align	2
_dyn_consume:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x0
Lloh4:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_36d5bd8a3470dcf9@PAGE
Lloh5:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_36d5bd8a3470dcf9@PAGEOFF]
	bl	_objc_msgSend
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_release
	.loh AdrpLdr	Lloh4, Lloh5

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"MyProtocol"

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_36d5bd8a3470dcf9
	.p2align	2
L_OBJC_IMAGE_INFO_36d5bd8a3470dcf9:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_36d5bd8a3470dcf9
L_OBJC_METH_VAR_NAME_36d5bd8a3470dcf9:
	.asciz	"aMethod"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_36d5bd8a3470dcf9
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_36d5bd8a3470dcf9:
	.quad	L_OBJC_METH_VAR_NAME_36d5bd8a3470dcf9

.subsections_via_symbols
