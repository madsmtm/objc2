	.section	__TEXT,__text,regular,pure_instructions
	.globl	_handle_with_sel
	.p2align	2
_handle_with_sel:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x0
	bl	__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround17hc2646b773d8b21f1E
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
	bl	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17hef5436b0c5b82354E
	mov	x20, x0
	bl	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17hab60d3c69560846dE
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
	bl	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17hf3b0aee156775a82E
	mov	x20, x0
	bl	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h7ecc61ed0718f1d3E
	mov	x2, x0
	mov	x0, x19
	mov	x1, x20
	bl	_objc_msgSend
	bl	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h66cda140112bf183E
	mov	x20, x0
	bl	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h7ecc61ed0718f1d3E
	mov	x2, x0
	mov	x0, x19
	mov	x1, x20
	bl	_objc_msgSend
	bl	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h5cc6e34851580441E
	mov	x20, x0
	bl	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h7ecc61ed0718f1d3E
	mov	x2, x0
	mov	x0, x19
	mov	x1, x20
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_msgSend

	.p2align	2
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround17hc2646b773d8b21f1E:
Lloh0:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_40f5b12005284286@PAGE
Lloh1:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_40f5b12005284286@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.p2align	2
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17hef5436b0c5b82354E:
Lloh2:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9@PAGE
Lloh3:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh2, Lloh3

	.p2align	2
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17hab60d3c69560846dE:
Lloh4:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9@PAGE
Lloh5:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh4, Lloh5

	.p2align	2
__ZN24test_msg_send_static_sel7generic22objc_static_workaround17h7ecc61ed0718f1d3E:
Lloh6:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_31f63858e271db32@PAGE
Lloh7:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_31f63858e271db32@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh6, Lloh7

	.p2align	2
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17hf3b0aee156775a82E:
Lloh8:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4@PAGE
Lloh9:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh8, Lloh9

	.p2align	2
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h66cda140112bf183E:
Lloh10:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1@PAGE
Lloh11:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh10, Lloh11

	.p2align	2
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h5cc6e34851580441E:
Lloh12:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720@PAGE
Lloh13:
	ldr	x0, [x8, L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh12, Lloh13

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_40f5b12005284286
	.p2align	2
L_OBJC_IMAGE_INFO_40f5b12005284286:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_40f5b12005284286
L_OBJC_METH_VAR_NAME_40f5b12005284286:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_40f5b12005284286
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_40f5b12005284286:
	.quad	L_OBJC_METH_VAR_NAME_40f5b12005284286

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_904c14aa63c4eec9
	.p2align	2
L_OBJC_IMAGE_INFO_904c14aa63c4eec9:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_904c14aa63c4eec9
L_OBJC_METH_VAR_NAME_904c14aa63c4eec9:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9:
	.quad	L_OBJC_METH_VAR_NAME_904c14aa63c4eec9

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_b1ab35d3713395f9
	.p2align	2
L_OBJC_IMAGE_INFO_b1ab35d3713395f9:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_b1ab35d3713395f9
L_OBJC_METH_VAR_NAME_b1ab35d3713395f9:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9:
	.quad	L_OBJC_METH_VAR_NAME_b1ab35d3713395f9

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_31f63858e271db32
	.p2align	2
L_OBJC_IMAGE_INFO_31f63858e271db32:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_31f63858e271db32
L_OBJC_METH_VAR_NAME_31f63858e271db32:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_31f63858e271db32
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_31f63858e271db32:
	.quad	L_OBJC_METH_VAR_NAME_31f63858e271db32

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_cdfe92d39025fdf4
	.p2align	2
L_OBJC_IMAGE_INFO_cdfe92d39025fdf4:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4
L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4:
	.quad	L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_79bd65c86d46fbf1
	.p2align	2
L_OBJC_IMAGE_INFO_79bd65c86d46fbf1:
	.space	8

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1
L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1
	.p2align	3
L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1:
	.quad	L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_8e0840c6b39b7720
	.p2align	2
L_OBJC_IMAGE_INFO_8e0840c6b39b7720:
	.space	8

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
