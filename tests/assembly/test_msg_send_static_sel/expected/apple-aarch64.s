	.section	__TEXT,__text,regular,pure_instructions
	.globl	_handle_with_sel
	.p2align	2
_handle_with_sel:
Lloh0:
	adrp	x1, __ZN24test_msg_send_static_sel15handle_with_sel5do_it5VALUE17h16ea9d1c425b55c9E@PAGE
Lloh1:
	add	x1, x1, __ZN24test_msg_send_static_sel15handle_with_sel5do_it5VALUE17h16ea9d1c425b55c9E@PAGEOFF
	b	_objc_msgSend
	.loh AdrpAdd	Lloh0, Lloh1

	.globl	_handle_alloc_init
	.p2align	2
_handle_alloc_init:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
Lloh2:
	adrp	x1, __ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h27da69bec7cfa7b0E@PAGE
Lloh3:
	add	x1, x1, __ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h27da69bec7cfa7b0E@PAGEOFF
	bl	_objc_msgSend
Lloh4:
	adrp	x1, __ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17hdf1c4acc22f16e29E@PAGE
Lloh5:
	add	x1, x1, __ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17hdf1c4acc22f16e29E@PAGEOFF
	ldp	x29, x30, [sp], #16
	b	_objc_msgSend
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh2, Lloh3

	.globl	_use_generic
	.p2align	2
_use_generic:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x0
Lloh6:
	adrp	x1, __ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h226e828d50735928E@PAGE
Lloh7:
	add	x1, x1, __ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h226e828d50735928E@PAGEOFF
Lloh8:
	adrp	x20, __ZN24test_msg_send_static_sel7generic5do_it5VALUE17h4bc61eda96fa1ac8E@PAGE
Lloh9:
	add	x20, x20, __ZN24test_msg_send_static_sel7generic5do_it5VALUE17h4bc61eda96fa1ac8E@PAGEOFF
	mov	x2, x20
	bl	_objc_msgSend
Lloh10:
	adrp	x1, __ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h2a3db9ebcc189240E@PAGE
Lloh11:
	add	x1, x1, __ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h2a3db9ebcc189240E@PAGEOFF
	mov	x0, x19
	mov	x2, x20
	bl	_objc_msgSend
Lloh12:
	adrp	x1, __ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h7e31e9de7919b8c1E@PAGE
Lloh13:
	add	x1, x1, __ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h7e31e9de7919b8c1E@PAGEOFF
	mov	x0, x19
	mov	x2, x20
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_msgSend
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh6, Lloh7

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel15handle_with_sel5do_it5VALUE17h16ea9d1c425b55c9E:
	.asciz	"someSelector"

__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17hdf1c4acc22f16e29E:
	.asciz	"init"

__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h27da69bec7cfa7b0E:
	.asciz	"alloc"

__ZN24test_msg_send_static_sel7generic5do_it5VALUE17h4bc61eda96fa1ac8E:
	.asciz	"generic:selector:"

__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h226e828d50735928E:
	.asciz	"performSelector:"

__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h2a3db9ebcc189240E:
	.asciz	"performSelector:"

__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h7e31e9de7919b8c1E:
	.asciz	"performSelector:"

.subsections_via_symbols
