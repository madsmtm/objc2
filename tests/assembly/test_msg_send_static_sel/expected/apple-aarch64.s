	.section	__TEXT,__text,regular,pure_instructions
	.globl	_handle_with_sel
	.p2align	2
_handle_with_sel:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x0
	bl	__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround17h4bd2e0f12aead0a7E
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
	bl	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h527035549ca8e16bE
	mov	x20, x0
	bl	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17hb38636f8b33d47f4E
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
	bl	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h88b4b045cc9efedaE
	mov	x20, x0
	bl	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17hd455b7ba208abb35E
	mov	x2, x0
	mov	x0, x19
	mov	x1, x20
	bl	_objc_msgSend
	bl	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h3045dd61383fe46dE
	mov	x20, x0
	bl	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17hd455b7ba208abb35E
	mov	x2, x0
	mov	x0, x19
	mov	x1, x20
	bl	_objc_msgSend
	bl	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17haef494cf629bf43fE
	mov	x20, x0
	bl	__ZN24test_msg_send_static_sel7generic22objc_static_workaround17hd455b7ba208abb35E
	mov	x2, x0
	mov	x0, x19
	mov	x1, x20
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_msgSend

	.p2align	2
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround17h4bd2e0f12aead0a7E:
Lloh0:
	adrp	x8, __ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround3REF17hac048478e2245505E@PAGE
Lloh1:
	ldr	x0, [x8, __ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround3REF17hac048478e2245505E@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.p2align	2
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17h527035549ca8e16bE:
Lloh2:
	adrp	x8, __ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h8d106c43f8f016acE@PAGE
Lloh3:
	ldr	x0, [x8, __ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h8d106c43f8f016acE@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh2, Lloh3

	.p2align	2
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround17hb38636f8b33d47f4E:
Lloh4:
	adrp	x8, __ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h9a45824596ee4331E@PAGE
Lloh5:
	ldr	x0, [x8, __ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h9a45824596ee4331E@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh4, Lloh5

	.p2align	2
__ZN24test_msg_send_static_sel7generic22objc_static_workaround17hd455b7ba208abb35E:
Lloh6:
	adrp	x8, __ZN24test_msg_send_static_sel7generic22objc_static_workaround3REF17hc4072be78a5a5a1bE@PAGE
Lloh7:
	ldr	x0, [x8, __ZN24test_msg_send_static_sel7generic22objc_static_workaround3REF17hc4072be78a5a5a1bE@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh6, Lloh7

	.p2align	2
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h88b4b045cc9efedaE:
Lloh8:
	adrp	x8, __ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17hc8472725ff869ed6E@PAGE
Lloh9:
	ldr	x0, [x8, __ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17hc8472725ff869ed6E@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh8, Lloh9

	.p2align	2
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17h3045dd61383fe46dE:
Lloh10:
	adrp	x8, __ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h6b07e7885c87d486E@PAGE
Lloh11:
	ldr	x0, [x8, __ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h6b07e7885c87d486E@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh10, Lloh11

	.p2align	2
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround17haef494cf629bf43fE:
Lloh12:
	adrp	x8, __ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h23a1cd3c801b23b5E@PAGE
Lloh13:
	ldr	x0, [x8, __ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h23a1cd3c801b23b5E@PAGEOFF]
	ret
	.loh AdrpLdr	Lloh12, Lloh13

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround4NAME17h374ab5fe13218218E:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround3REF17hac048478e2245505E:
	.quad	__ZN24test_msg_send_static_sel15handle_with_sel22objc_static_workaround4NAME17h374ab5fe13218218E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17h85d1b2a3012cd21bE:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h8d106c43f8f016acE:
	.quad	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17h85d1b2a3012cd21bE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17h947eb119c81ac655E:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround3REF17h9a45824596ee4331E:
	.quad	__ZN24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround4NAME17h947eb119c81ac655E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel7generic22objc_static_workaround4NAME17h0195309ec581b392E:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel7generic22objc_static_workaround3REF17hc4072be78a5a5a1bE:
	.quad	__ZN24test_msg_send_static_sel7generic22objc_static_workaround4NAME17h0195309ec581b392E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17h42056f820f677913E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17hc8472725ff869ed6E:
	.quad	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17h42056f820f677913E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17he0c88f86b370a316E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h6b07e7885c87d486E:
	.quad	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17he0c88f86b370a316E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17h3ee106e8027713a3E:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	3
__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround3REF17h23a1cd3c801b23b5E:
	.quad	__ZN24test_msg_send_static_sel11use_generic22objc_static_workaround4NAME17h3ee106e8027713a3E

.subsections_via_symbols
