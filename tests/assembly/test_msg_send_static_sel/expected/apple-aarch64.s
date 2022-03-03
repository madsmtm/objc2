	.section	__TEXT,__text,regular,pure_instructions
	.globl	_handle_with_sel
	.p2align	2
_handle_with_sel:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x0
	bl	__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround17h7e2219bd31d55268E
	mov	x1, x0
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_msgSend

	.globl	_handle_alloc_init
	.p2align	2
_handle_alloc_init:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x0
	bl	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h3d919388c23fdf68E
	mov	x20, x0
	bl	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h208b45a18d15451eE
	mov	x1, x0
	mov	x0, x19
	bl	_objc_msgSend
	mov	x1, x20
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_msgSend

	.globl	_use_generic
	.p2align	2
_use_generic:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x0
	bl	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h29677622e36cf330E
	mov	x20, x0
	bl	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h432a20390f91c26fE
	mov	x2, x0
	mov	x0, x19
	mov	x1, x20
	bl	_objc_msgSend
	bl	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h777a3aa99de98b73E
	mov	x20, x0
	bl	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h432a20390f91c26fE
	mov	x2, x0
	mov	x0, x19
	mov	x1, x20
	bl	_objc_msgSend
	bl	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h82d96cce58f7d605E
	mov	x20, x0
	bl	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h432a20390f91c26fE
	mov	x2, x0
	mov	x0, x19
	mov	x1, x20
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_msgSend

	.p2align	2
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround17h7e2219bd31d55268E:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	__ZN5objc210image_info17h1a00393fd20ac219E
Lloh0:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_40f5b12005284286@PAGE
Lloh1:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_40f5b12005284286@PAGEOFF]
	ldp	x29, x30, [sp], #16
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.p2align	2
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h3d919388c23fdf68E:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	__ZN5objc210image_info17h1a00393fd20ac219E
Lloh2:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9@PAGE
Lloh3:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9@PAGEOFF]
	ldp	x29, x30, [sp], #16
	ret
	.loh AdrpLdr	Lloh2, Lloh3

	.p2align	2
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h208b45a18d15451eE:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	__ZN5objc210image_info17h1a00393fd20ac219E
Lloh4:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9@PAGE
Lloh5:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9@PAGEOFF]
	ldp	x29, x30, [sp], #16
	ret
	.loh AdrpLdr	Lloh4, Lloh5

	.p2align	2
__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h432a20390f91c26fE:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	__ZN5objc210image_info17h1a00393fd20ac219E
Lloh6:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_31f63858e271db32@PAGE
Lloh7:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_31f63858e271db32@PAGEOFF]
	ldp	x29, x30, [sp], #16
	ret
	.loh AdrpLdr	Lloh6, Lloh7

	.p2align	2
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h29677622e36cf330E:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	__ZN5objc210image_info17h1a00393fd20ac219E
Lloh8:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4@PAGE
Lloh9:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4@PAGEOFF]
	ldp	x29, x30, [sp], #16
	ret
	.loh AdrpLdr	Lloh8, Lloh9

	.p2align	2
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h777a3aa99de98b73E:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	__ZN5objc210image_info17h1a00393fd20ac219E
Lloh10:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1@PAGE
Lloh11:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1@PAGEOFF]
	ldp	x29, x30, [sp], #16
	ret
	.loh AdrpLdr	Lloh10, Lloh11

	.p2align	2
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h82d96cce58f7d605E:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	__ZN5objc210image_info17h1a00393fd20ac219E
Lloh12:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720@PAGE
Lloh13:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720@PAGEOFF]
	ldp	x29, x30, [sp], #16
	ret
	.loh AdrpLdr	Lloh12, Lloh13

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_40f5b12005284286
L_OBJC_METH_VAR_NAME_40f5b12005284286:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_40f5b12005284286
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_40f5b12005284286:
	.quad	L_OBJC_METH_VAR_NAME_40f5b12005284286

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_904c14aa63c4eec9
L_OBJC_METH_VAR_NAME_904c14aa63c4eec9:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9:
	.quad	L_OBJC_METH_VAR_NAME_904c14aa63c4eec9

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_b1ab35d3713395f9
L_OBJC_METH_VAR_NAME_b1ab35d3713395f9:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9:
	.quad	L_OBJC_METH_VAR_NAME_b1ab35d3713395f9

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_31f63858e271db32
L_OBJC_METH_VAR_NAME_31f63858e271db32:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_31f63858e271db32
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_31f63858e271db32:
	.quad	L_OBJC_METH_VAR_NAME_31f63858e271db32

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4
L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4:
	.quad	L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1
L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1:
	.quad	L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_8e0840c6b39b7720
L_OBJC_METH_VAR_NAME_8e0840c6b39b7720:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720:
	.quad	L_OBJC_METH_VAR_NAME_8e0840c6b39b7720

.subsections_via_symbols
