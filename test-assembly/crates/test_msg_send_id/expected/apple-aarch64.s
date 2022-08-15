	.section	__TEXT,__text,regular,pure_instructions
	.globl	_handle_new
	.p2align	2
_handle_new:
	b	_objc_msgSend

	.globl	_handle_new_fallible
	.p2align	2
_handle_new_fallible:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x1
	mov	x20, x0
	bl	_objc_msgSend
	cbz	x0, LBB1_2
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB1_2:
	mov	x0, x20
	mov	x1, x19
	bl	SYM(objc2::__macro_helpers::new_failed::GENERATED_ID, 0)

	.globl	_handle_alloc
	.p2align	2
_handle_alloc:
	b	_objc_msgSend

	.globl	_handle_alloc_fallible
	.p2align	2
_handle_alloc_fallible:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x1
	mov	x20, x0
	bl	_objc_msgSend
	cbz	x0, LBB3_2
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB3_2:
	mov	x0, x20
	mov	x1, x19
	bl	SYM(objc2::__macro_helpers::alloc_failed::GENERATED_ID, 0)

	.globl	_handle_init
	.p2align	2
_handle_init:
	b	_objc_msgSend

	.globl	_handle_init_fallible
	.p2align	2
_handle_init_fallible:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x1
	mov	x20, x0
	bl	_objc_msgSend
	cbz	x0, LBB5_2
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB5_2:
	mov	x0, x20
	mov	x1, x19
	bl	SYM(objc2::__macro_helpers::init_failed::GENERATED_ID, 0)

	.globl	_handle_alloc_init
	.p2align	2
_handle_alloc_init:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x2
	bl	_objc_msgSend
	mov	x1, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_msgSend

	.globl	_handle_alloc_release
	.p2align	2
_handle_alloc_release:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_objc_msgSend
	ldp	x29, x30, [sp], #16
	b	_objc_release

	.globl	_handle_alloc_init_release
	.p2align	2
_handle_alloc_init_release:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x2
	bl	_objc_msgSend
	mov	x1, x19
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_release

	.globl	_handle_copy
	.p2align	2
_handle_copy:
	b	_objc_msgSend

	.globl	_handle_copy_fallible
	.p2align	2
_handle_copy_fallible:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_objc_msgSend
	cbz	x0, LBB10_2
	ldp	x29, x30, [sp], #16
	ret
LBB10_2:
	bl	SYM(objc2::__macro_helpers::copy_failed::GENERATED_ID, 0)

	.globl	_handle_autoreleased
	.p2align	2
_handle_autoreleased:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_objc_msgSend
	; InlineAsm Start
	mov	x29, x29
	; InlineAsm End
	ldp	x29, x30, [sp], #16
	b	_objc_retainAutoreleasedReturnValue

	.globl	_handle_autoreleased_fallible
	.p2align	2
_handle_autoreleased_fallible:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x1
	mov	x20, x0
	bl	_objc_msgSend
	; InlineAsm Start
	mov	x29, x29
	; InlineAsm End
	bl	_objc_retainAutoreleasedReturnValue
	cbz	x0, LBB12_2
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB12_2:
	mov	x0, x20
	mov	x1, x19
	bl	SYM(objc2::__macro_helpers::normal_failed::GENERATED_ID, 0)

.subsections_via_symbols
