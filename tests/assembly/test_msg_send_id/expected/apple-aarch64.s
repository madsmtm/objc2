	.section	__TEXT,__text,regular,pure_instructions
	.globl	_handle_alloc
	.p2align	2
_handle_alloc:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_objc_msgSend
	cbz	x0, LBB0_2
	ldp	x29, x30, [sp], #16
	ret
LBB0_2:
Lloh0:
	adrp	x0, l___unnamed_1@PAGE
Lloh1:
	add	x0, x0, l___unnamed_1@PAGEOFF
Lloh2:
	adrp	x2, l___unnamed_2@PAGE
Lloh3:
	add	x2, x2, l___unnamed_2@PAGEOFF
	mov	w1, #17
	bl	__ZN4core6option13expect_failed17h16e38b99483f925bE
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh0, Lloh1

	.globl	_handle_init
	.p2align	2
_handle_init:
	b	_objc_msgSend

	.globl	_handle_alloc_init
	.p2align	2
_handle_alloc_init:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x2
	bl	_objc_msgSend
	cbz	x0, LBB2_2
	mov	x1, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_msgSend
LBB2_2:
Lloh4:
	adrp	x0, l___unnamed_1@PAGE
Lloh5:
	add	x0, x0, l___unnamed_1@PAGEOFF
Lloh6:
	adrp	x2, l___unnamed_2@PAGE
Lloh7:
	add	x2, x2, l___unnamed_2@PAGEOFF
	mov	w1, #17
	bl	__ZN4core6option13expect_failed17h16e38b99483f925bE
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5

	.globl	_handle_alloc_release
	.p2align	2
_handle_alloc_release:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_objc_msgSend
	cbz	x0, LBB3_2
	ldp	x29, x30, [sp], #16
	b	_objc_release
LBB3_2:
Lloh8:
	adrp	x0, l___unnamed_1@PAGE
Lloh9:
	add	x0, x0, l___unnamed_1@PAGEOFF
Lloh10:
	adrp	x2, l___unnamed_2@PAGE
Lloh11:
	add	x2, x2, l___unnamed_2@PAGEOFF
	mov	w1, #17
	bl	__ZN4core6option13expect_failed17h16e38b99483f925bE
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9

	.globl	_handle_alloc_init_release
	.p2align	2
_handle_alloc_init_release:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x2
	bl	_objc_msgSend
	cbz	x0, LBB4_2
	mov	x1, x19
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_release
LBB4_2:
Lloh12:
	adrp	x0, l___unnamed_1@PAGE
Lloh13:
	add	x0, x0, l___unnamed_1@PAGEOFF
Lloh14:
	adrp	x2, l___unnamed_2@PAGE
Lloh15:
	add	x2, x2, l___unnamed_2@PAGEOFF
	mov	w1, #17
	bl	__ZN4core6option13expect_failed17h16e38b99483f925bE
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13

	.globl	_handle_copy
	.p2align	2
_handle_copy:
	b	_objc_msgSend

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

	.section	__TEXT,__const
l___unnamed_1:
	.ascii	"Failed allocating"

l___unnamed_3:
	.ascii	"$WORKSPACE/objc2/src/__macro_helpers.rs"

	.section	__DATA,__const
	.p2align	3
l___unnamed_2:
	.quad	l___unnamed_3
	.asciz	"B\000\000\000\000\000\000\000\037\000\000\000%\000\000"

.subsections_via_symbols
