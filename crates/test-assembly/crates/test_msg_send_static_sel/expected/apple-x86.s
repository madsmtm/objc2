	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_fn1_handle_with_sel
	.p2align	4
_fn1_handle_with_sel:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	call	L0$pb
L0$pb:
	pop	eax
	sub	esp, 8
	push	dword ptr [eax + L_OBJC_SELECTOR_REFERENCES_66d4c61f523c7074-L0$pb]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 24
	pop	ebp
	ret

	.globl	_fn2_handle_alloc_init
	.p2align	4
_fn2_handle_alloc_init:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	call	L1$pb
L1$pb:
	pop	edi
	mov	eax, dword ptr [edi + LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr-L1$pb]
	sub	esp, 8
	push	dword ptr [eax]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 16
	mov	esi, eax
	mov	eax, dword ptr [edi + LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-L1$pb]
	mov	ebx, dword ptr [eax]
	sub	esp, 8
	push	ebx
	push	esi
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
	lea	eax, [edi + l_anon.[ID].1-L1$pb]
	push	eax
	push	ebx
	push	esi
	call	SYM(objc2::__macros::retain_semantics::init_fail::GENERATED_ID, 0)

	.globl	_fn3_use_generic
	.p2align	4
_fn3_use_generic:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	call	L2$pb
L2$pb:
	pop	esi
	mov	edi, dword ptr [ebp + 8]
	sub	esp, 4
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e-L2$pb]
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_8d4579a56572fa21-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 12
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e-L2$pb]
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_d018146ff130bbd9-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 12
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e-L2$pb]
	push	dword ptr [esi + L_OBJC_SELECTOR_REFERENCES_6659046596384437-L2$pb]
	push	edi
	call	_objc_msgSend
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6659046596384437
L_OBJC_METH_VAR_NAME_6659046596384437:
	.asciz	"performSelector:"

	.globl	L_OBJC_METH_VAR_NAME_66d4c61f523c7074
L_OBJC_METH_VAR_NAME_66d4c61f523c7074:
	.asciz	"someSelector"

	.globl	L_OBJC_METH_VAR_NAME_8d4579a56572fa21
L_OBJC_METH_VAR_NAME_8d4579a56572fa21:
	.asciz	"performSelector:"

	.globl	L_OBJC_METH_VAR_NAME_d018146ff130bbd9
L_OBJC_METH_VAR_NAME_d018146ff130bbd9:
	.asciz	"performSelector:"

	.globl	L_OBJC_METH_VAR_NAME_e348e18690a1021e
L_OBJC_METH_VAR_NAME_e348e18690a1021e:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_6659046596384437
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_6659046596384437:
	.long	L_OBJC_METH_VAR_NAME_6659046596384437

	.globl	L_OBJC_SELECTOR_REFERENCES_66d4c61f523c7074
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_66d4c61f523c7074:
	.long	L_OBJC_METH_VAR_NAME_66d4c61f523c7074

	.globl	L_OBJC_SELECTOR_REFERENCES_8d4579a56572fa21
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_8d4579a56572fa21:
	.long	L_OBJC_METH_VAR_NAME_8d4579a56572fa21

	.globl	L_OBJC_SELECTOR_REFERENCES_d018146ff130bbd9
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_d018146ff130bbd9:
	.long	L_OBJC_METH_VAR_NAME_d018146ff130bbd9

	.globl	L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e:
	.long	L_OBJC_METH_VAR_NAME_e348e18690a1021e

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_6659046596384437
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_6659046596384437:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_66d4c61f523c7074
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_66d4c61f523c7074:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_8d4579a56572fa21
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_8d4579a56572fa21:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_d018146ff130bbd9
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_d018146ff130bbd9:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_e348e18690a1021e
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_e348e18690a1021e:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].0:
	.asciz	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].1:
	.long	L_anon.[ID].0
	.asciz	";\000\000\000\016\000\000\000\005\000\000"

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_alloc
	.long	0
LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_init
	.long	0

.subsections_via_symbols
