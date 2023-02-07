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
	push	dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_77d2b75bddfbef7c-L0$pb]
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
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	call	L1$pb
L1$pb:
	pop	ebx
	mov	eax, dword ptr [ebx + LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-L1$pb]
	mov	edi, dword ptr [eax]
	mov	eax, dword ptr [ebx + LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr-L1$pb]
	sub	esp, 8
	push	dword ptr [eax]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 16
	mov	esi, eax
	sub	esp, 8
	push	edi
	push	eax
	call	_objc_msgSend
	add	esp, 16
	test	eax, eax
	je	LBB1_2
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB1_2:
	sub	esp, 4
	lea	eax, [ebx + l_anon.[ID].1-L1$pb]
	push	eax
	push	edi
	push	esi
	call	SYM(<objc2::__macro_helpers::RetainSemantics<3_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)

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
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_379095321e06c060-L2$pb]
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_038d21a6277de1da-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 12
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_379095321e06c060-L2$pb]
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_573c1e9c42ae1ea1-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 12
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_379095321e06c060-L2$pb]
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_9885c1be4d03110d-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	2
l_anon.[ID].1:
	.long	l_anon.[ID].0
	.asciz	";\000\000\000\016\000\000\000\005\000\000"

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_77d2b75bddfbef7c
	.p2align	2
L_OBJC_IMAGE_INFO_77d2b75bddfbef7c:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_77d2b75bddfbef7c
L_OBJC_METH_VAR_NAME_77d2b75bddfbef7c:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_77d2b75bddfbef7c
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_77d2b75bddfbef7c:
	.long	L_OBJC_METH_VAR_NAME_77d2b75bddfbef7c

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_379095321e06c060
	.p2align	2
L_OBJC_IMAGE_INFO_379095321e06c060:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_379095321e06c060
L_OBJC_METH_VAR_NAME_379095321e06c060:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_379095321e06c060
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_379095321e06c060:
	.long	L_OBJC_METH_VAR_NAME_379095321e06c060

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_038d21a6277de1da
	.p2align	2
L_OBJC_IMAGE_INFO_038d21a6277de1da:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_038d21a6277de1da
L_OBJC_METH_VAR_NAME_038d21a6277de1da:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_038d21a6277de1da
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_038d21a6277de1da:
	.long	L_OBJC_METH_VAR_NAME_038d21a6277de1da

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_573c1e9c42ae1ea1
	.p2align	2
L_OBJC_IMAGE_INFO_573c1e9c42ae1ea1:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_573c1e9c42ae1ea1
L_OBJC_METH_VAR_NAME_573c1e9c42ae1ea1:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_573c1e9c42ae1ea1
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_573c1e9c42ae1ea1:
	.long	L_OBJC_METH_VAR_NAME_573c1e9c42ae1ea1

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_9885c1be4d03110d
	.p2align	2
L_OBJC_IMAGE_INFO_9885c1be4d03110d:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_9885c1be4d03110d
L_OBJC_METH_VAR_NAME_9885c1be4d03110d:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_9885c1be4d03110d
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_9885c1be4d03110d:
	.long	L_OBJC_METH_VAR_NAME_9885c1be4d03110d

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_alloc
	.long	0
LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_init
	.long	0

.subsections_via_symbols
