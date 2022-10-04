	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle_new
	.p2align	2
	.code	32
_handle_new:
	b	_objc_msgSend

	.globl	_handle_new_fallible
	.p2align	2
	.code	32
_handle_new_fallible:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	mov	r4, r1
	mov	r5, r0
	bl	_objc_msgSend
	cmp	r0, #0
	popne	{r4, r5, r7, pc}
LBB1_1:
	movw	r2, :lower16:(l_anon.[ID].1-(LPC1_0+8))
	mov	r0, r5
	movt	r2, :upper16:(l_anon.[ID].1-(LPC1_0+8))
	mov	r1, r4
LPC1_0:
	add	r2, pc, r2
	mov	lr, pc
	b	SYM(<objc2::__macro_helpers::RetainSemantics<1_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)

	.globl	_handle_alloc
	.p2align	2
	.code	32
_handle_alloc:
	b	_objc_msgSend

	.globl	_handle_alloc_fallible
	.p2align	2
	.code	32
_handle_alloc_fallible:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	mov	r4, r1
	mov	r5, r0
	bl	_objc_msgSend
	cmp	r0, #0
	popne	{r4, r5, r7, pc}
LBB3_1:
	movw	r2, :lower16:(l_anon.[ID].2-(LPC3_0+8))
	mov	r0, r5
	movt	r2, :upper16:(l_anon.[ID].2-(LPC3_0+8))
	mov	r1, r4
LPC3_0:
	add	r2, pc, r2
	mov	lr, pc
	b	SYM(<objc2::__macro_helpers::RetainSemantics<2_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)

	.globl	_handle_init
	.p2align	2
	.code	32
_handle_init:
	b	_objc_msgSend

	.globl	_handle_init_fallible
	.p2align	2
	.code	32
_handle_init_fallible:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	mov	r4, r1
	mov	r5, r0
	bl	_objc_msgSend
	cmp	r0, #0
	popne	{r4, r5, r7, pc}
LBB5_1:
	movw	r2, :lower16:(l_anon.[ID].3-(LPC5_0+8))
	mov	r0, r5
	movt	r2, :upper16:(l_anon.[ID].3-(LPC5_0+8))
	mov	r1, r4
LPC5_0:
	add	r2, pc, r2
	mov	lr, pc
	b	SYM(<objc2::__macro_helpers::RetainSemantics<3_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)

	.globl	_handle_alloc_init
	.p2align	2
	.code	32
_handle_alloc_init:
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r2
	bl	_objc_msgSend
	mov	r1, r4
	pop	{r4, r7, lr}
	b	_objc_msgSend

	.globl	_handle_alloc_release
	.p2align	2
	.code	32
_handle_alloc_release:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	pop	{r7, lr}
	b	_objc_release

	.globl	_handle_alloc_init_release
	.p2align	2
	.code	32
_handle_alloc_init_release:
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r2
	bl	_objc_msgSend
	mov	r1, r4
	bl	_objc_msgSend
	pop	{r4, r7, lr}
	b	_objc_release

	.globl	_handle_copy
	.p2align	2
	.code	32
_handle_copy:
	b	_objc_msgSend

	.globl	_handle_copy_fallible
	.p2align	2
	.code	32
_handle_copy_fallible:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	cmp	r0, #0
	popne	{r7, pc}
LBB10_1:
	movw	r0, :lower16:(l_anon.[ID].4-(LPC10_0+8))
	movt	r0, :upper16:(l_anon.[ID].4-(LPC10_0+8))
LPC10_0:
	add	r0, pc, r0
	mov	lr, pc
	b	SYM(<objc2::__macro_helpers::RetainSemantics<4_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)

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
	pop	{r7, lr}
	b	_objc_retainAutoreleasedReturnValue

	.globl	_handle_autoreleased_fallible
	.p2align	2
	.code	32
_handle_autoreleased_fallible:
	push	{r4, r5, r7, lr}
	add	r7, sp, #8
	mov	r4, r1
	mov	r5, r0
	bl	_objc_msgSend
	@ InlineAsm Start
	mov	r7, r7
	@ InlineAsm End
	bl	_objc_retainAutoreleasedReturnValue
	cmp	r0, #0
	popne	{r4, r5, r7, pc}
LBB12_1:
	movw	r2, :lower16:(l_anon.[ID].5-(LPC12_0+8))
	mov	r0, r5
	movt	r2, :upper16:(l_anon.[ID].5-(LPC12_0+8))
	mov	r1, r4
LPC12_0:
	add	r2, pc, r2
	mov	lr, pc
	b	SYM(<objc2::__macro_helpers::RetainSemantics<5_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	2
l_anon.[ID].1:
	.long	l_anon.[ID].0
	.asciz	",\000\000\000\r\000\000\000\005\000\000"

	.p2align	2
l_anon.[ID].2:
	.long	l_anon.[ID].0
	.asciz	",\000\000\000\027\000\000\000\005\000\000"

	.p2align	2
l_anon.[ID].3:
	.long	l_anon.[ID].0
	.asciz	",\000\000\000!\000\000\000\005\000\000"

	.p2align	2
l_anon.[ID].4:
	.long	l_anon.[ID].0
	.asciz	",\000\000\000>\000\000\000\005\000\000"

	.p2align	2
l_anon.[ID].5:
	.long	l_anon.[ID].0
	.asciz	",\000\000\000H\000\000\000\005\000\000"

.subsections_via_symbols
