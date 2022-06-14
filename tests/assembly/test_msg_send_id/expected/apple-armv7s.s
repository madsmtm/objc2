	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle_alloc
	.p2align	2
	.code	32
_handle_alloc:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	cmp	r0, #0
	popne	{r7, pc}
LBB0_1:
	movw	r0, :lower16:(l___unnamed_1-(LPC0_0+8))
	mov	r1, #17
	movt	r0, :upper16:(l___unnamed_1-(LPC0_0+8))
	movw	r2, :lower16:(l___unnamed_2-(LPC0_1+8))
	movt	r2, :upper16:(l___unnamed_2-(LPC0_1+8))
LPC0_0:
	add	r0, pc, r0
LPC0_1:
	add	r2, pc, r2
	mov	lr, pc
	b	__ZN4core6option13expect_failed17h9461c0e6a2661999E

	.globl	_handle_init
	.p2align	2
	.code	32
_handle_init:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	pop	{r7, pc}

	.globl	_handle_alloc_init
	.p2align	2
	.code	32
_handle_alloc_init:
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r2
	bl	_objc_msgSend
	cmp	r0, #0
	beq	LBB2_2
	mov	r1, r4
	bl	_objc_msgSend
	pop	{r4, r7, pc}
LBB2_2:
	movw	r0, :lower16:(l___unnamed_1-(LPC2_0+8))
	mov	r1, #17
	movt	r0, :upper16:(l___unnamed_1-(LPC2_0+8))
	movw	r2, :lower16:(l___unnamed_2-(LPC2_1+8))
	movt	r2, :upper16:(l___unnamed_2-(LPC2_1+8))
LPC2_0:
	add	r0, pc, r0
LPC2_1:
	add	r2, pc, r2
	mov	lr, pc
	b	__ZN4core6option13expect_failed17h9461c0e6a2661999E

	.globl	_handle_alloc_release
	.p2align	2
	.code	32
_handle_alloc_release:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	cmp	r0, #0
	beq	LBB3_2
	bl	_objc_release
	pop	{r7, pc}
LBB3_2:
	movw	r0, :lower16:(l___unnamed_1-(LPC3_0+8))
	mov	r1, #17
	movt	r0, :upper16:(l___unnamed_1-(LPC3_0+8))
	movw	r2, :lower16:(l___unnamed_2-(LPC3_1+8))
	movt	r2, :upper16:(l___unnamed_2-(LPC3_1+8))
LPC3_0:
	add	r0, pc, r0
LPC3_1:
	add	r2, pc, r2
	mov	lr, pc
	b	__ZN4core6option13expect_failed17h9461c0e6a2661999E

	.globl	_handle_alloc_init_release
	.p2align	2
	.code	32
_handle_alloc_init_release:
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r2
	bl	_objc_msgSend
	cmp	r0, #0
	beq	LBB4_2
	mov	r1, r4
	bl	_objc_msgSend
	bl	_objc_release
	pop	{r4, r7, pc}
LBB4_2:
	movw	r0, :lower16:(l___unnamed_1-(LPC4_0+8))
	mov	r1, #17
	movt	r0, :upper16:(l___unnamed_1-(LPC4_0+8))
	movw	r2, :lower16:(l___unnamed_2-(LPC4_1+8))
	movt	r2, :upper16:(l___unnamed_2-(LPC4_1+8))
LPC4_0:
	add	r0, pc, r0
LPC4_1:
	add	r2, pc, r2
	mov	lr, pc
	b	__ZN4core6option13expect_failed17h9461c0e6a2661999E

	.globl	_handle_copy
	.p2align	2
	.code	32
_handle_copy:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	pop	{r7, pc}

	.globl	_handle_autoreleased
	.p2align	2
	.code	32
_handle_autoreleased:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	@ InlineAsm Start
	mov	r7, r7
	@ InlineAsm End
	bl	_objc_retainAutoreleasedReturnValue
	pop	{r7, pc}

	.section	__TEXT,__const
l___unnamed_1:
	.ascii	"Failed allocating"

l___unnamed_3:
	.ascii	"$WORKSPACE/objc2/src/__macro_helpers.rs"

	.section	__DATA,__const
	.p2align	2
l___unnamed_2:
	.long	l___unnamed_3
	.asciz	"B\000\000\000\037\000\000\000%\000\000"

.subsections_via_symbols
