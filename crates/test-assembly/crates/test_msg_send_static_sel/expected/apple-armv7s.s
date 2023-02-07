	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle_with_sel
	.p2align	2
	.code	32
_handle_with_sel:
	push	{r7, lr}
	mov	r7, sp
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_77d2b75bddfbef7c-(LPC0_0+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_77d2b75bddfbef7c-(LPC0_0+8))
LPC0_0:
	ldr	r1, [pc, r1]
	bl	_objc_msgSend
	pop	{r7, pc}

	.globl	_handle_alloc_init
	.p2align	2
	.code	32
_handle_alloc_init:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	movw	r1, :lower16:(LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-(LPC1_0+8))
	movt	r1, :upper16:(LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-(LPC1_0+8))
	movw	r2, :lower16:(LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr-(LPC1_1+8))
	movt	r2, :upper16:(LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr-(LPC1_1+8))
LPC1_0:
	ldr	r1, [pc, r1]
LPC1_1:
	ldr	r2, [pc, r2]
	ldr	r4, [r1]
	ldr	r1, [r2]
	bl	_objc_msgSend
	mov	r1, r4
	mov	r5, r0
	bl	_objc_msgSend
	cmp	r0, #0
	popne	{r4, r5, r7, pc}
LBB1_1:
	movw	r2, :lower16:(l_anon.[ID].1-(LPC1_2+8))
	mov	r0, r5
	movt	r2, :upper16:(l_anon.[ID].1-(LPC1_2+8))
	mov	r1, r4
LPC1_2:
	add	r2, pc, r2
	mov	lr, pc
	b	SYM(<objc2::__macro_helpers::RetainSemantics<3_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)

	.globl	_use_generic
	.p2align	2
	.code	32
_use_generic:
	push	{r4, r7, lr}
	add	r7, sp, #4
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_379095321e06c060-(LPC2_0+8))
	mov	r4, r0
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_379095321e06c060-(LPC2_0+8))
LPC2_0:
	ldr	r2, [pc, r2]
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_038d21a6277de1da-(LPC2_1+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_038d21a6277de1da-(LPC2_1+8))
LPC2_1:
	ldr	r1, [pc, r1]
	bl	_objc_msgSend
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_379095321e06c060-(LPC2_2+8))
	mov	r0, r4
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_379095321e06c060-(LPC2_2+8))
LPC2_2:
	ldr	r2, [pc, r2]
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_573c1e9c42ae1ea1-(LPC2_3+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_573c1e9c42ae1ea1-(LPC2_3+8))
LPC2_3:
	ldr	r1, [pc, r1]
	bl	_objc_msgSend
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_379095321e06c060-(LPC2_4+8))
	mov	r0, r4
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_379095321e06c060-(LPC2_4+8))
LPC2_4:
	ldr	r2, [pc, r2]
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_9885c1be4d03110d-(LPC2_5+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_9885c1be4d03110d-(LPC2_5+8))
LPC2_5:
	ldr	r1, [pc, r1]
	bl	_objc_msgSend
	pop	{r4, r7, pc}

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

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2
LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_alloc
	.long	0
LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_init
	.long	0

.subsections_via_symbols
