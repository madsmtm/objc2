	.syntax unified
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_handle_with_sel
	.p2align	2
	.code	32
_handle_with_sel:
	push	{r7, lr}
	mov	r7, sp
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_80f1580ed33ec51b-(LPC0_0+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_80f1580ed33ec51b-(LPC0_0+8))
LPC0_0:
	ldr	r1, [pc, r1]
	pop	{r7, lr}
	b	_objc_msgSend

	.globl	_handle_alloc_init
	.p2align	2
	.code	32
_handle_alloc_init:
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
	b	SYM(objc2::__macro_helpers::retain_semantics::init_fail::GENERATED_ID, 0)

	.globl	_use_generic
	.p2align	2
	.code	32
_use_generic:
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r0
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_67bf3e41c7e639a3-(LPC2_0+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_67bf3e41c7e639a3-(LPC2_0+8))
LPC2_0:
	ldr	r1, [pc, r1]
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5-(LPC2_1+8))
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5-(LPC2_1+8))
LPC2_1:
	ldr	r2, [pc, r2]
	bl	_objc_msgSend
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_2c2c9a8191012941-(LPC2_2+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_2c2c9a8191012941-(LPC2_2+8))
LPC2_2:
	ldr	r1, [pc, r1]
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5-(LPC2_3+8))
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5-(LPC2_3+8))
LPC2_3:
	ldr	r2, [pc, r2]
	mov	r0, r4
	bl	_objc_msgSend
	movw	r1, :lower16:(L_OBJC_SELECTOR_REFERENCES_993d94b40d47ed52-(LPC2_4+8))
	movt	r1, :upper16:(L_OBJC_SELECTOR_REFERENCES_993d94b40d47ed52-(LPC2_4+8))
LPC2_4:
	ldr	r1, [pc, r1]
	movw	r2, :lower16:(L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5-(LPC2_5+8))
	movt	r2, :upper16:(L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5-(LPC2_5+8))
LPC2_5:
	ldr	r2, [pc, r2]
	mov	r0, r4
	pop	{r4, r7, lr}
	b	_objc_msgSend

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_80f1580ed33ec51b
L_OBJC_METH_VAR_NAME_80f1580ed33ec51b:
	.asciz	"someSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_80f1580ed33ec51b
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_80f1580ed33ec51b:
	.long	L_OBJC_METH_VAR_NAME_80f1580ed33ec51b

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_91c006d97540f4b5
L_OBJC_METH_VAR_NAME_91c006d97540f4b5:
	.asciz	"generic:selector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_91c006d97540f4b5:
	.long	L_OBJC_METH_VAR_NAME_91c006d97540f4b5

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_67bf3e41c7e639a3
L_OBJC_METH_VAR_NAME_67bf3e41c7e639a3:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_67bf3e41c7e639a3
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_67bf3e41c7e639a3:
	.long	L_OBJC_METH_VAR_NAME_67bf3e41c7e639a3

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2c2c9a8191012941
L_OBJC_METH_VAR_NAME_2c2c9a8191012941:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_2c2c9a8191012941
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_2c2c9a8191012941:
	.long	L_OBJC_METH_VAR_NAME_2c2c9a8191012941

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_993d94b40d47ed52
L_OBJC_METH_VAR_NAME_993d94b40d47ed52:
	.asciz	"performSelector:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_993d94b40d47ed52
	.p2align	2, 0x0
L_OBJC_SELECTOR_REFERENCES_993d94b40d47ed52:
	.long	L_OBJC_METH_VAR_NAME_993d94b40d47ed52

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].1:
	.long	l_anon.[ID].0
	.asciz	";\000\000\000\016\000\000\000\005\000\000"

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_80f1580ed33ec51b
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_80f1580ed33ec51b:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_91c006d97540f4b5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_91c006d97540f4b5:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_67bf3e41c7e639a3
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_67bf3e41c7e639a3:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_2c2c9a8191012941
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2c2c9a8191012941:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_993d94b40d47ed52
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_993d94b40d47ed52:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__nl_symbol_ptr,non_lazy_symbol_pointers
	.p2align	2, 0x0
LL_OBJC_SELECTOR_REFERENCES_alloc$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_alloc
	.long	0
LL_OBJC_SELECTOR_REFERENCES_init$non_lazy_ptr:
	.indirect_symbol	L_OBJC_SELECTOR_REFERENCES_init
	.long	0

.subsections_via_symbols
