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
	movt	r2, :upper16:(l_anon.[ID].1-(LPC1_0+8))
LPC1_0:
	add	r2, pc, r2
	mov	r0, r5
	mov	r1, r4
	mov	lr, pc
	b	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<1_u8> as objc2::__macro_helpers::msg_send_retained::MsgSendRetainedFailed>::failed::GENERATED_ID, 0)

	.globl	_handle_alloc
	.p2align	2
	.code	32
_handle_alloc:
	b	_objc_msgSend

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
LBB4_1:
	movw	r2, :lower16:(l_anon.[ID].2-(LPC4_0+8))
	movt	r2, :upper16:(l_anon.[ID].2-(LPC4_0+8))
LPC4_0:
	add	r2, pc, r2
	mov	r0, r5
	mov	r1, r4
	mov	lr, pc
	b	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<3_u8> as objc2::__macro_helpers::msg_send_retained::MsgSendRetainedFailed>::failed::GENERATED_ID, 0)

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
LBB9_1:
	movw	r0, :lower16:(l_anon.[ID].3-(LPC9_0+8))
	movt	r0, :upper16:(l_anon.[ID].3-(LPC9_0+8))
LPC9_0:
	add	r0, pc, r0
	mov	lr, pc
	b	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<4_u8> as objc2::__macro_helpers::msg_send_retained::MsgSendRetainedFailed>::failed::GENERATED_ID, 0)

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

	.globl	_handle_autoreleased_with_arg
	.p2align	2
	.code	32
_handle_autoreleased_with_arg:
	push	{r7, lr}
	mov	r7, sp
	uxtb	r2, r2
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
	movw	r2, :lower16:(l_anon.[ID].4-(LPC12_0+8))
	movt	r2, :upper16:(l_anon.[ID].4-(LPC12_0+8))
LPC12_0:
	add	r2, pc, r2
	mov	r0, r5
	mov	r1, r4
	mov	lr, pc
	b	SYM(<objc2::__macro_helpers::method_family::RetainSemantics<5_u8> as objc2::__macro_helpers::msg_send_retained::MsgSendRetainedFailed>::failed::GENERATED_ID, 0)

	.globl	_handle_with_out_param
	.p2align	2
	.code	32
_handle_with_out_param:
	push	{r4, r5, r6, r7, lr}
	add	r7, sp, #12
	mov	r4, r2
	ldr	r5, [r2]
	bl	_objc_msgSend
	mov	r6, r0
	ldr	r0, [r4]
	bl	_objc_retain
	mov	r0, r5
	bl	_objc_release
	@ InlineAsm Start
	mov	r7, r7
	@ InlineAsm End
	mov	r0, r6
	pop	{r4, r5, r6, r7, lr}
	b	_objc_retainAutoreleasedReturnValue

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	2, 0x0
l_anon.[ID].1:
	.long	l_anon.[ID].0
	.asciz	"9\000\000\000\r\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].2:
	.long	l_anon.[ID].0
	.asciz	"9\000\000\000\034\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].3:
	.long	l_anon.[ID].0
	.asciz	"9\000\000\0008\000\000\000\005\000\000"

	.p2align	2, 0x0
l_anon.[ID].4:
	.long	l_anon.[ID].0
	.asciz	"9\000\000\000L\000\000\000\005\000\000"

.subsections_via_symbols
