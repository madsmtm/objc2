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
Lloh0:
	adrp	x2, l_anon.[ID].1@PAGE
Lloh1:
	add	x2, x2, l_anon.[ID].1@PAGEOFF
	mov	x0, x20
	mov	x1, x19
	bl	SYM(<objc2::__macro_helpers::RetainSemantics<_,_,_,_> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh0, Lloh1

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
Lloh2:
	adrp	x2, l_anon.[ID].2@PAGE
Lloh3:
	add	x2, x2, l_anon.[ID].2@PAGEOFF
	mov	x0, x20
	mov	x1, x19
	bl	SYM(<objc2::__macro_helpers::RetainSemantics<_,_,_,_> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 1)
	.loh AdrpAdd	Lloh2, Lloh3

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
Lloh4:
	adrp	x2, l_anon.[ID].3@PAGE
Lloh5:
	add	x2, x2, l_anon.[ID].3@PAGEOFF
	mov	x0, x20
	mov	x1, x19
	bl	SYM(<objc2::__macro_helpers::RetainSemantics<_,_,_,_> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 2)
	.loh AdrpAdd	Lloh4, Lloh5

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
Lloh6:
	adrp	x0, l_anon.[ID].4@PAGE
Lloh7:
	add	x0, x0, l_anon.[ID].4@PAGEOFF
	bl	SYM(<objc2::__macro_helpers::RetainSemantics<_,_,_,_> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 3)
	.loh AdrpAdd	Lloh6, Lloh7

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
Lloh8:
	adrp	x2, l_anon.[ID].5@PAGE
Lloh9:
	add	x2, x2, l_anon.[ID].5@PAGEOFF
	mov	x0, x20
	mov	x1, x19
	bl	SYM(<objc2::__macro_helpers::RetainSemantics<_,_,_,_> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 4)
	.loh AdrpAdd	Lloh8, Lloh9

	.section	__TEXT,__const
l_anon.[ID].0:
	.ascii	"$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3
l_anon.[ID].1:
	.quad	l_anon.[ID].0
	.asciz	",\000\000\000\000\000\000\000\r\000\000\000\005\000\000"

	.p2align	3
l_anon.[ID].2:
	.quad	l_anon.[ID].0
	.asciz	",\000\000\000\000\000\000\000\027\000\000\000\005\000\000"

	.p2align	3
l_anon.[ID].3:
	.quad	l_anon.[ID].0
	.asciz	",\000\000\000\000\000\000\000!\000\000\000\005\000\000"

	.p2align	3
l_anon.[ID].4:
	.quad	l_anon.[ID].0
	.asciz	",\000\000\000\000\000\000\000@\000\000\000\005\000\000"

	.p2align	3
l_anon.[ID].5:
	.quad	l_anon.[ID].0
	.asciz	",\000\000\000\000\000\000\000J\000\000\000\005\000\000"

.subsections_via_symbols
