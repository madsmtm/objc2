	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
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

.subsections_via_symbols
