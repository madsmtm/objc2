	.section	__TEXT,__text,regular,pure_instructions
	.globl	_handle_with_sel
	.p2align	2
_handle_with_sel:
Lloh0:
	adrp	x8, __ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17h3b4abf454e7caef2E@PAGE
Lloh1:
	ldr	x1, [x8, __ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17h3b4abf454e7caef2E@PAGEOFF]
	b	_objc_msgSend
	.loh AdrpLdr	Lloh0, Lloh1

	.globl	_handle_alloc_init
	.p2align	2
_handle_alloc_init:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh2:
	adrp	x8, __MergedGlobals@PAGE
Lloh3:
	add	x8, x8, __MergedGlobals@PAGEOFF
	ldr	x19, [x8]
	ldr	x1, [x8, #8]
	bl	_objc_msgSend
	mov	x1, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_msgSend
	.loh AdrpAdd	Lloh2, Lloh3

	.globl	_use_generic
	.p2align	2
_use_generic:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x0
Lloh4:
	adrp	x20, __MergedGlobals@PAGE+16
Lloh5:
	add	x20, x20, __MergedGlobals@PAGEOFF+16
	ldr	x1, [x20, #8]
	ldr	x2, [x20]
	bl	_objc_msgSend
	ldr	x1, [x20, #16]
	ldr	x2, [x20]
	mov	x0, x19
	bl	_objc_msgSend
	ldr	x1, [x20, #24]
	ldr	x2, [x20]
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_msgSend
	.loh AdrpAdd	Lloh4, Lloh5

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel15handle_with_sel5do_it5VALUE17h16ea9d1c425b55c9E:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__ZN24test_msg_send_static_sel15handle_with_sel5do_it3REF17h3b4abf454e7caef2E:
	.quad	__ZN24test_msg_send_static_sel15handle_with_sel5do_it5VALUE17h16ea9d1c425b55c9E

	.section	__TEXT,__objc_methname,cstring_literals
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

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.p2align	3
__MergedGlobals:
	.quad	__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17hdf1c4acc22f16e29E
	.quad	__ZN24test_msg_send_static_sel17handle_alloc_init5do_it5VALUE17h27da69bec7cfa7b0E
	.quad	__ZN24test_msg_send_static_sel7generic5do_it5VALUE17h4bc61eda96fa1ac8E
	.quad	__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h226e828d50735928E
	.quad	__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h2a3db9ebcc189240E
	.quad	__ZN24test_msg_send_static_sel11use_generic5do_it5VALUE17h7e31e9de7919b8c1E

.subsections_via_symbols
