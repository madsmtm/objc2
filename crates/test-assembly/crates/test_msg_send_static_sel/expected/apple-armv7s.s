	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle_with_sel
	.p2align	2
	.code	32
_handle_with_sel:
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_664c1e40eb8cd76e-(LPC0_0+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_664c1e40eb8cd76e-(LPC0_0+8))
LPC0_0:
	ldr	r1, [pc, r1]
	b	_objc_msgSend

	.globl	_handle_alloc_init
	.p2align	2
	.code	32
_handle_alloc_init:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	bl	_objc_alloc
	movw	r1, :lower16:(LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-(LPC1_0+8))
	movt	r1, :upper16:(LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-(LPC1_0+8))
LPC1_0:
	ldr	r1, [pc, r1]
	mov	r4, r0
	ldr	r5, [r1]
	mov	r1, r5
	bl	_objc_msgSend
	cmp	r0, #0
	popne	{r4, r5, r7, pc}
LBB1_1:
	movw	r2, :lower16:(l_anon.[ID].1-(LPC1_1+8))
	movt	r2, :upper16:(l_anon.[ID].1-(LPC1_1+8))
LPC1_1:
	add	r2, pc, r2
	mov	r0, r4
	mov	r1, r5
	mov	lr, pc
	b	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<3_u8> as objc2::__macro_helpers::msg_send_retained::MsgSendIdFailed>::failed::GENERATED_ID, 0)

	.globl	_use_generic
	.p2align	2
	.code	32
_use_generic:
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r0
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_e4e4edcd2d17efb8-(LPC2_0+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_e4e4edcd2d17efb8-(LPC2_0+8))
LPC2_0:
	ldr	r1, [pc, r1]
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_80036160fc60677b-(LPC2_1+8))
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_80036160fc60677b-(LPC2_1+8))
LPC2_1:
	ldr	r2, [pc, r2]
	bl	_objc_msgSend
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_bf9373a91792acd9-(LPC2_2+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_bf9373a91792acd9-(LPC2_2+8))
LPC2_2:
	ldr	r1, [pc, r1]
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_80036160fc60677b-(LPC2_3+8))
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_80036160fc60677b-(LPC2_3+8))
LPC2_3:
	ldr	r2, [pc, r2]
	mov	r0, r4
	bl	_objc_msgSend
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_65f663aa0a6ddc1d-(LPC2_4+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_65f663aa0a6ddc1d-(LPC2_4+8))
LPC2_4:
	ldr	r1, [pc, r1]
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_80036160fc60677b-(LPC2_5+8))
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_80036160fc60677b-(LPC2_5+8))
LPC2_5:
	ldr	r2, [pc, r2]
	mov	r0, r4
	pop	{r4, r7, lr}
	b	_objc_msgSend

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].1:
	.long	l_anon.[ID].0
	.asciz	";\000\000\000\016\000\000\000\005\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_664c1e40eb8cd76e
L_OBJC_METH_VAR_NAME_664c1e40eb8cd76e:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_664c1e40eb8cd76e
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_664c1e40eb8cd76e:
	.long	L_OBJC_METH_VAR_NAME_664c1e40eb8cd76e

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_664c1e40eb8cd76e
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_664c1e40eb8cd76e:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_80036160fc60677b
L_OBJC_METH_VAR_NAME_80036160fc60677b:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_80036160fc60677b
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_80036160fc60677b:
	.long	L_OBJC_METH_VAR_NAME_80036160fc60677b

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_80036160fc60677b
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_80036160fc60677b:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e4e4edcd2d17efb8
L_OBJC_METH_VAR_NAME_e4e4edcd2d17efb8:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_e4e4edcd2d17efb8
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_e4e4edcd2d17efb8:
	.long	L_OBJC_METH_VAR_NAME_e4e4edcd2d17efb8

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_e4e4edcd2d17efb8
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_e4e4edcd2d17efb8:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_bf9373a91792acd9
L_OBJC_METH_VAR_NAME_bf9373a91792acd9:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_bf9373a91792acd9
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_bf9373a91792acd9:
	.long	L_OBJC_METH_VAR_NAME_bf9373a91792acd9

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_bf9373a91792acd9
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_bf9373a91792acd9:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_65f663aa0a6ddc1d
L_OBJC_METH_VAR_NAME_65f663aa0a6ddc1d:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_65f663aa0a6ddc1d
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_65f663aa0a6ddc1d:
	.long	L_OBJC_METH_VAR_NAME_65f663aa0a6ddc1d

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_65f663aa0a6ddc1d
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_65f663aa0a6ddc1d:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2, 0x0
LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_init
	.long	0

.subsections_via_symbols
