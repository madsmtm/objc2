	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle_with_sel
	.p2align	2
	.code	32
_handle_with_sel:
	push	{r7, lr}
	mov	r7, sp
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc-(LPC0_0+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc-(LPC0_0+8))
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
	mov	r0, r5
	mov	r1, r4
	mov	lr, pc
	b	SYM(objc2::__macro_helpers::init_failed::GENERATED_ID, 0)

	.globl	_use_generic
	.p2align	2
	.code	32
_use_generic:
	push	{r4, r7, lr}
	add	r7, sp, #4
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05-(LPC2_0+8))
	mov	r4, r0
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05-(LPC2_0+8))
LPC2_0:
	ldr	r2, [pc, r2]
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_36a6e334f5aeb023-(LPC2_1+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_36a6e334f5aeb023-(LPC2_1+8))
LPC2_1:
	ldr	r1, [pc, r1]
	bl	_objc_msgSend
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05-(LPC2_2+8))
	mov	r0, r4
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05-(LPC2_2+8))
LPC2_2:
	ldr	r2, [pc, r2]
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_2c3c38f6ea036343-(LPC2_3+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_2c3c38f6ea036343-(LPC2_3+8))
LPC2_3:
	ldr	r1, [pc, r1]
	bl	_objc_msgSend
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05-(LPC2_4+8))
	mov	r0, r4
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05-(LPC2_4+8))
LPC2_4:
	ldr	r2, [pc, r2]
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_e1e97023e8bcf6a4-(LPC2_5+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_e1e97023e8bcf6a4-(LPC2_5+8))
LPC2_5:
	ldr	r1, [pc, r1]
	bl	_objc_msgSend
	pop	{r4, r7, pc}

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_044375a4329d08dc
	.p2align	2
L_OBJC_IMAGE_INFO_044375a4329d08dc:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_044375a4329d08dc
L_OBJC_METH_VAR_NAME_044375a4329d08dc:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_044375a4329d08dc:
	.long	L_OBJC_METH_VAR_NAME_044375a4329d08dc

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_5ace898e385eba05
	.p2align	2
L_OBJC_IMAGE_INFO_5ace898e385eba05:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_5ace898e385eba05
L_OBJC_METH_VAR_NAME_5ace898e385eba05:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_5ace898e385eba05:
	.long	L_OBJC_METH_VAR_NAME_5ace898e385eba05

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_36a6e334f5aeb023
	.p2align	2
L_OBJC_IMAGE_INFO_36a6e334f5aeb023:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_36a6e334f5aeb023
L_OBJC_METH_VAR_NAME_36a6e334f5aeb023:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_36a6e334f5aeb023
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_36a6e334f5aeb023:
	.long	L_OBJC_METH_VAR_NAME_36a6e334f5aeb023

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2c3c38f6ea036343
	.p2align	2
L_OBJC_IMAGE_INFO_2c3c38f6ea036343:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2c3c38f6ea036343
L_OBJC_METH_VAR_NAME_2c3c38f6ea036343:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2c3c38f6ea036343
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_2c3c38f6ea036343:
	.long	L_OBJC_METH_VAR_NAME_2c3c38f6ea036343

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_e1e97023e8bcf6a4
	.p2align	2
L_OBJC_IMAGE_INFO_e1e97023e8bcf6a4:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_e1e97023e8bcf6a4
L_OBJC_METH_VAR_NAME_e1e97023e8bcf6a4:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_e1e97023e8bcf6a4
	.p2align	2
L_OBJC_SELECTOR_REFERENCES_e1e97023e8bcf6a4:
	.long	L_OBJC_METH_VAR_NAME_e1e97023e8bcf6a4

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2
LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_alloc
	.long	0
LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_init
	.long	0

.subsections_via_symbols
