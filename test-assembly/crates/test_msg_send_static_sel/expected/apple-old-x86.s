	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_handle_with_sel
	.p2align	4, 0x90
_handle_with_sel:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	call	L0$pb
L0$pb:
	pop	eax
	sub	esp, 8
	push	dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc-L0$pb]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 24
	pop	ebp
	ret

	.globl	_handle_alloc_init
	.p2align	4, 0x90
_handle_alloc_init:
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	call	L1$pb
L1$pb:
	pop	eax
	mov	esi, dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_cb49b9ab1b00e328-L1$pb]
	sub	esp, 8
	push	dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_dcb825748735621d-L1$pb]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 8
	push	esi
	push	eax
	call	_objc_msgSend
	add	esp, 20
	pop	esi
	pop	ebp
	ret

	.globl	_use_generic
	.p2align	4, 0x90
_use_generic:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	call	L2$pb
L2$pb:
	pop	esi
	mov	edi, dword ptr [ebp + 8]
	sub	esp, 4
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_457d234345d46cbe-L2$pb]
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_f16064a6f68ca673-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 12
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_457d234345d46cbe-L2$pb]
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_9f134b97cb598446-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 12
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_457d234345d46cbe-L2$pb]
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_e76e01e8b2327e5d-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_044375a4329d08dc
	.p2align	2
L_OBJC_IMAGE_INFO_044375a4329d08dc:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_044375a4329d08dc
L_OBJC_METH_VAR_NAME_044375a4329d08dc:
	.asciz	"someSelector"

	.section	__OBJC,__message_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc:
	.long	L_OBJC_METH_VAR_NAME_044375a4329d08dc

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_cb49b9ab1b00e328
	.p2align	2
L_OBJC_IMAGE_INFO_cb49b9ab1b00e328:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_cb49b9ab1b00e328
L_OBJC_METH_VAR_NAME_cb49b9ab1b00e328:
	.asciz	"init"

	.section	__OBJC,__message_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_cb49b9ab1b00e328
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_cb49b9ab1b00e328:
	.long	L_OBJC_METH_VAR_NAME_cb49b9ab1b00e328

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_dcb825748735621d
	.p2align	2
L_OBJC_IMAGE_INFO_dcb825748735621d:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_dcb825748735621d
L_OBJC_METH_VAR_NAME_dcb825748735621d:
	.asciz	"alloc"

	.section	__OBJC,__message_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_dcb825748735621d
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_dcb825748735621d:
	.long	L_OBJC_METH_VAR_NAME_dcb825748735621d

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_457d234345d46cbe
	.p2align	2
L_OBJC_IMAGE_INFO_457d234345d46cbe:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_457d234345d46cbe
L_OBJC_METH_VAR_NAME_457d234345d46cbe:
	.asciz	"generic:selector:"

	.section	__OBJC,__message_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_457d234345d46cbe
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_457d234345d46cbe:
	.long	L_OBJC_METH_VAR_NAME_457d234345d46cbe

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_f16064a6f68ca673
	.p2align	2
L_OBJC_IMAGE_INFO_f16064a6f68ca673:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_f16064a6f68ca673
L_OBJC_METH_VAR_NAME_f16064a6f68ca673:
	.asciz	"performSelector:"

	.section	__OBJC,__message_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_f16064a6f68ca673
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_f16064a6f68ca673:
	.long	L_OBJC_METH_VAR_NAME_f16064a6f68ca673

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_9f134b97cb598446
	.p2align	2
L_OBJC_IMAGE_INFO_9f134b97cb598446:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_9f134b97cb598446
L_OBJC_METH_VAR_NAME_9f134b97cb598446:
	.asciz	"performSelector:"

	.section	__OBJC,__message_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_9f134b97cb598446
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_9f134b97cb598446:
	.long	L_OBJC_METH_VAR_NAME_9f134b97cb598446

	.section	__OBJC,__image_info
	.globl	L_OBJC_IMAGE_INFO_e76e01e8b2327e5d
	.p2align	2
L_OBJC_IMAGE_INFO_e76e01e8b2327e5d:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e76e01e8b2327e5d
L_OBJC_METH_VAR_NAME_e76e01e8b2327e5d:
	.asciz	"performSelector:"

	.section	__OBJC,__message_refs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_e76e01e8b2327e5d
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_e76e01e8b2327e5d:
	.long	L_OBJC_METH_VAR_NAME_e76e01e8b2327e5d

.subsections_via_symbols
