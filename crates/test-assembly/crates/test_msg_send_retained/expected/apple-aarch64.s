	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn01_handle_new
	.p2align	2
_fn01_handle_new:
	b	_objc_msgSend

	.globl	_fn02_handle_new_fallible
	.p2align	2
_fn02_handle_new_fallible:
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
	bl	SYM(objc2[CRATE_ID]::__macros::retain_semantics::new_fail, 0)
	.loh AdrpAdd	Lloh0, Lloh1

	.globl	_fn03_handle_alloc
	.p2align	2
_fn03_handle_alloc:
	b	_objc_msgSend

	.globl	_fn04_handle_init
	.p2align	2
_fn04_handle_init:
	b	_objc_msgSend

	.globl	_fn05_handle_init_fallible
	.p2align	2
_fn05_handle_init_fallible:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x1
	mov	x20, x0
	bl	_objc_msgSend
	cbz	x0, LBB4_2
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB4_2:
Lloh2:
	adrp	x2, l_anon.[ID].2@PAGE
Lloh3:
	add	x2, x2, l_anon.[ID].2@PAGEOFF
	mov	x0, x20
	mov	x1, x19
	bl	SYM(objc2[CRATE_ID]::__macros::retain_semantics::init_fail, 0)
	.loh AdrpAdd	Lloh2, Lloh3

	.globl	_fn06_handle_alloc_init
	.p2align	2
_fn06_handle_alloc_init:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x2
	bl	_objc_msgSend
	mov	x1, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_msgSend

	.globl	_fn07_handle_alloc_release
	.p2align	2
_fn07_handle_alloc_release:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_objc_msgSend
	ldp	x29, x30, [sp], #16
	b	_objc_release

	.globl	_fn08_handle_alloc_init_release
	.p2align	2
_fn08_handle_alloc_init_release:
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

	.globl	_fn09_handle_copy
	.p2align	2
_fn09_handle_copy:
	b	_objc_msgSend

	.globl	_fn10_handle_copy_fallible
	.p2align	2
_fn10_handle_copy_fallible:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_objc_msgSend
	cbz	x0, LBB9_2
	ldp	x29, x30, [sp], #16
	ret
LBB9_2:
Lloh4:
	adrp	x0, l_anon.[ID].3@PAGE
Lloh5:
	add	x0, x0, l_anon.[ID].3@PAGEOFF
	bl	SYM(objc2[CRATE_ID]::__macros::retain_semantics::copy_fail, 0)
	.loh AdrpAdd	Lloh4, Lloh5

	.globl	_fn11_handle_mutable_copy
	.p2align	2
_fn11_handle_mutable_copy:
	b	_objc_msgSend

	.globl	_fn12_handle_mutable_copy_fallible
	.p2align	2
_fn12_handle_mutable_copy_fallible:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_objc_msgSend
	cbz	x0, LBB11_2
	ldp	x29, x30, [sp], #16
	ret
LBB11_2:
Lloh6:
	adrp	x0, l_anon.[ID].4@PAGE
Lloh7:
	add	x0, x0, l_anon.[ID].4@PAGEOFF
	bl	SYM(objc2[CRATE_ID]::__macros::retain_semantics::mutable_copy_fail, 0)
	.loh AdrpAdd	Lloh6, Lloh7

	.globl	_fn13_handle_autoreleased
	.p2align	2
_fn13_handle_autoreleased:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_objc_msgSend
	; InlineAsm Start
	mov	x29, x29
	; InlineAsm End
	ldp	x29, x30, [sp], #16
	b	_objc_retainAutoreleasedReturnValue

	.globl	_fn14_handle_autoreleased_with_arg
	.p2align	2
_fn14_handle_autoreleased_with_arg:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	and	w2, w2, #0xff
	bl	_objc_msgSend
	; InlineAsm Start
	mov	x29, x29
	; InlineAsm End
	ldp	x29, x30, [sp], #16
	b	_objc_retainAutoreleasedReturnValue

	.globl	_fn15_handle_autoreleased_fallible
	.p2align	2
_fn15_handle_autoreleased_fallible:
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
	cbz	x0, LBB14_2
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB14_2:
Lloh8:
	adrp	x2, l_anon.[ID].5@PAGE
Lloh9:
	add	x2, x2, l_anon.[ID].5@PAGEOFF
	mov	x0, x20
	mov	x1, x19
	bl	SYM(objc2[CRATE_ID]::__macros::retain_semantics::none_fail, 0)
	.loh AdrpAdd	Lloh8, Lloh9

	.globl	_fn16_handle_with_out_param
	.p2align	2
_fn16_handle_with_out_param:
Lfunc_begin0:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x20, x2
	ldr	x19, [x2]
Ltmp0:
	bl	_objc_msgSend
Ltmp1:
	; InlineAsm Start
	mov	x29, x29
	; InlineAsm End
Ltmp2:
	bl	_objc_retainAutoreleasedReturnValue
Ltmp3:
	mov	x21, x0
	ldr	x0, [x20]
	bl	_objc_retain
	mov	x0, x19
	bl	_objc_release
	mov	x0, x21
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret
LBB15_3:
Ltmp4:
	mov	x21, x0
	ldr	x0, [x20]
Ltmp5:
	bl	_objc_retain
Ltmp6:
Ltmp7:
	mov	x0, x19
	bl	_objc_release
Ltmp8:
	mov	x0, x21
	bl	__Unwind_Resume
LBB15_6:
Ltmp9:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table15:
Lexception0:
	.byte	255
	.byte	155
	.uleb128 Lttbase0-Lttbaseref0
Lttbaseref0:
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp3-Ltmp0
	.uleb128 Ltmp4-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp3-Lfunc_begin0
	.uleb128 Ltmp5-Ltmp3
	.byte	0
	.byte	0
	.uleb128 Ltmp5-Lfunc_begin0
	.uleb128 Ltmp8-Ltmp5
	.uleb128 Ltmp9-Lfunc_begin0
	.byte	1
	.uleb128 Ltmp8-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp8
	.byte	0
	.byte	0
Lcst_end0:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].0:
	.asciz	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].1:
	.quad	l_anon.[ID].0
	.asciz	"9\000\000\000\000\000\000\000\017\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].2:
	.quad	l_anon.[ID].0
	.asciz	"9\000\000\000\000\000\000\000\036\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].3:
	.quad	l_anon.[ID].0
	.asciz	"9\000\000\000\000\000\000\000:\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].4:
	.quad	l_anon.[ID].0
	.asciz	"9\000\000\000\000\000\000\000D\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].5:
	.quad	l_anon.[ID].0
	.asciz	"9\000\000\000\000\000\000\000X\000\000\000\005\000\000"

.subsections_via_symbols
