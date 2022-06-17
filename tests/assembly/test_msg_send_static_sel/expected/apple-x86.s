	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_with_sel
	.p2align	4, 0x90
_handle_with_sel:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	mov	esi, dword ptr [ebp + 8]
	call	__RNvNvCs66pPOBPZDax_24test_msg_send_static_sel15handle_with_sel22objc_static_workaround
	sub	esp, 8
	push	eax
	push	esi
	call	_objc_msgSend
	add	esp, 20
	pop	esi
	pop	ebp
	ret

	.globl	_handle_alloc_init
	.p2align	4, 0x90
_handle_alloc_init:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	mov	edi, dword ptr [ebp + 8]
	call	__RNvNvCs66pPOBPZDax_24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround
	mov	esi, eax
	call	__RNvNvCs66pPOBPZDax_24test_msg_send_static_sel17handle_alloc_inits_22objc_static_workaround
	sub	esp, 8
	push	eax
	push	edi
	call	_objc_msgSend
	add	esp, 8
	push	esi
	push	eax
	call	_objc_msgSend
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret

	.globl	_use_generic
	.p2align	4, 0x90
_use_generic:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	mov	edi, dword ptr [ebp + 8]
	call	__RNvNvCs66pPOBPZDax_24test_msg_send_static_sel11use_generic22objc_static_workaround
	mov	esi, eax
	call	__RNvNvCs66pPOBPZDax_24test_msg_send_static_sel7generic22objc_static_workaround
	sub	esp, 4
	push	eax
	push	esi
	push	edi
	call	_objc_msgSend
	add	esp, 16
	call	__RNvNvCs66pPOBPZDax_24test_msg_send_static_sel11use_generics_22objc_static_workaround
	mov	esi, eax
	call	__RNvNvCs66pPOBPZDax_24test_msg_send_static_sel7generic22objc_static_workaround
	sub	esp, 4
	push	eax
	push	esi
	push	edi
	call	_objc_msgSend
	add	esp, 16
	call	__RNvNvCs66pPOBPZDax_24test_msg_send_static_sel11use_generics0_22objc_static_workaround
	mov	esi, eax
	call	__RNvNvCs66pPOBPZDax_24test_msg_send_static_sel7generic22objc_static_workaround
	sub	esp, 4
	push	eax
	push	esi
	push	edi
	call	_objc_msgSend
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret

	.p2align	4, 0x90
__RNvNvCs66pPOBPZDax_24test_msg_send_static_sel15handle_with_sel22objc_static_workaround:
	push	ebp
	mov	ebp, esp
	call	L3$pb
L3$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_40f5b12005284286-L3$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__RNvNvCs66pPOBPZDax_24test_msg_send_static_sel17handle_alloc_init22objc_static_workaround:
	push	ebp
	mov	ebp, esp
	call	L4$pb
L4$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9-L4$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__RNvNvCs66pPOBPZDax_24test_msg_send_static_sel17handle_alloc_inits_22objc_static_workaround:
	push	ebp
	mov	ebp, esp
	call	L5$pb
L5$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9-L5$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__RNvNvCs66pPOBPZDax_24test_msg_send_static_sel7generic22objc_static_workaround:
	push	ebp
	mov	ebp, esp
	call	L6$pb
L6$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_31f63858e271db32-L6$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__RNvNvCs66pPOBPZDax_24test_msg_send_static_sel11use_generic22objc_static_workaround:
	push	ebp
	mov	ebp, esp
	call	L7$pb
L7$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4-L7$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__RNvNvCs66pPOBPZDax_24test_msg_send_static_sel11use_generics_22objc_static_workaround:
	push	ebp
	mov	ebp, esp
	call	L8$pb
L8$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1-L8$pb]
	pop	ebp
	ret

	.p2align	4, 0x90
__RNvNvCs66pPOBPZDax_24test_msg_send_static_sel11use_generics0_22objc_static_workaround:
	push	ebp
	mov	ebp, esp
	call	L9$pb
L9$pb:
	pop	eax
	mov	eax, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720-L9$pb]
	pop	ebp
	ret

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
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_40f5b12005284286:
	.long	L_OBJC_METH_VAR_NAME_40f5b12005284286

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
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_904c14aa63c4eec9:
	.long	L_OBJC_METH_VAR_NAME_904c14aa63c4eec9

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
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_b1ab35d3713395f9:
	.long	L_OBJC_METH_VAR_NAME_b1ab35d3713395f9

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
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_31f63858e271db32:
	.long	L_OBJC_METH_VAR_NAME_31f63858e271db32

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
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_cdfe92d39025fdf4:
	.long	L_OBJC_METH_VAR_NAME_cdfe92d39025fdf4

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
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_79bd65c86d46fbf1:
	.long	L_OBJC_METH_VAR_NAME_79bd65c86d46fbf1

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
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_8e0840c6b39b7720:
	.long	L_OBJC_METH_VAR_NAME_8e0840c6b39b7720

.subsections_via_symbols
