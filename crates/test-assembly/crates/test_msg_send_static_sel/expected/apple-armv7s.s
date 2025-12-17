	.syntax	unified
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn1_handle_with_sel
	.p2align	2
	.code	32
_fn1_handle_with_sel:
	push	{r7, lr}
	mov	r7, sp
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_66d4c61f523c7074-(LPC0_0+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_66d4c61f523c7074-(LPC0_0+8))
LPC0_0:
	ldr	r1, [pc, r1]
	pop	{r7, lr}
	b	_objc_msgSend

	.globl	_fn2_handle_alloc_init
	.p2align	2
	.code	32
_fn2_handle_alloc_init:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	movw	r1, :lower16:(LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr-(LPC1_0+8))
	movt	r1, :upper16:(LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr-(LPC1_0+8))
LPC1_0:
	ldr	r1, [pc, r1]
	ldr	r1, [r1]
	bl	_objc_msgSend
	movw	r1, :lower16:(LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-(LPC1_1+8))
	movt	r1, :upper16:(LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr-(LPC1_1+8))
LPC1_1:
	ldr	r1, [pc, r1]
	mov	r4, r0
	ldr	r5, [r1]
	mov	r1, r5
	bl	_objc_msgSend
	cmp	r0, #0
	popne	{r4, r5, r7, pc}
LBB1_1:
	movw	r2, :lower16:(l_anon.[ID].1-(LPC1_2+8))
	movt	r2, :upper16:(l_anon.[ID].1-(LPC1_2+8))
LPC1_2:
	add	r2, pc, r2
	mov	r0, r4
	mov	r1, r5
	mov	lr, pc
	b	SYM(objc2[CRATE_ID]::__macros::retain_semantics::init_fail, 0)

	.globl	_fn3_use_generic
	.p2align	2
	.code	32
_fn3_use_generic:
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r0
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_8d4579a56572fa21-(LPC2_0+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_8d4579a56572fa21-(LPC2_0+8))
LPC2_0:
	ldr	r1, [pc, r1]
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e-(LPC2_1+8))
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e-(LPC2_1+8))
LPC2_1:
	ldr	r2, [pc, r2]
	bl	_objc_msgSend
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_d018146ff130bbd9-(LPC2_2+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_d018146ff130bbd9-(LPC2_2+8))
LPC2_2:
	ldr	r1, [pc, r1]
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e-(LPC2_3+8))
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e-(LPC2_3+8))
LPC2_3:
	ldr	r2, [pc, r2]
	mov	r0, r4
	bl	_objc_msgSend
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_6659046596384437-(LPC2_4+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_6659046596384437-(LPC2_4+8))
LPC2_4:
	ldr	r1, [pc, r1]
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e-(LPC2_5+8))
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_e348e18690a1021e-(LPC2_5+8))
LPC2_5:
	ldr	r2, [pc, r2]
	mov	r0, r4
	pop	{r4, r7, lr}
	b	_objc_msgSend

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

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2, 0x0
LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_alloc
	.long	0
LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_init
	.long	0

.subsections_via_symbols
