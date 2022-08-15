	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_handle_new
	.p2align	2
	.code	32
_handle_new:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	pop	{r7, pc}

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
	mov	r0, r5
	mov	r1, r4
	mov	lr, pc
	b	SYM(objc2::__macro_helpers::new_failed::GENERATED_ID, 0)

	.globl	_handle_alloc
	.p2align	2
	.code	32
_handle_alloc:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	pop	{r7, pc}

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
	mov	r0, r5
	mov	r1, r4
	mov	lr, pc
	b	SYM(objc2::__macro_helpers::alloc_failed::GENERATED_ID, 0)

	.globl	_handle_init
	.p2align	2
	.code	32
_handle_init:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	pop	{r7, pc}

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
	mov	r0, r5
	mov	r1, r4
	mov	lr, pc
	b	SYM(objc2::__macro_helpers::init_failed::GENERATED_ID, 0)

	.globl	_handle_alloc_init
	.p2align	2
	.code	32
_handle_alloc_init:
	push	{r4, r7, lr}
	add	r7, sp, #4
	mov	r4, r2
	bl	_objc_msgSend
	mov	r1, r4
	bl	_objc_msgSend
	pop	{r4, r7, pc}

	.globl	_handle_alloc_release
	.p2align	2
	.code	32
_handle_alloc_release:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	bl	_objc_release
	pop	{r7, pc}

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
	bl	_objc_release
	pop	{r4, r7, pc}

	.globl	_handle_copy
	.p2align	2
	.code	32
_handle_copy:
	push	{r7, lr}
	mov	r7, sp
	bl	_objc_msgSend
	pop	{r7, pc}

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
	mov	lr, pc
	b	SYM(objc2::__macro_helpers::copy_failed::GENERATED_ID, 0)

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
	mov	r0, r5
	mov	r1, r4
	mov	lr, pc
	b	SYM(objc2::__macro_helpers::normal_failed::GENERATED_ID, 0)

.subsections_via_symbols
