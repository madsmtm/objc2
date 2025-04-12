	.section	__TEXT,__text,regular,pure_instructions
	.globl	_iter_create
	.p2align	2
_iter_create:
	stp	xzr, xzr, [x8, #192]
	movi.2d	v0, #0000000000000000
	stp	q0, q0, [x8, #160]
	stur	q0, [x8, #8]
	stur	q0, [x8, #24]
	stur	q0, [x8, #40]
	stur	q0, [x8, #56]
	stur	q0, [x8, #72]
	stur	q0, [x8, #88]
	stur	q0, [x8, #104]
	stur	q0, [x8, #120]
	str	x0, [x8]
	stp	xzr, xzr, [x8, #144]
	str	xzr, [x8, #136]
	str	xzr, [x8, #208]
	ret

	.globl	_iter_once
	.p2align	2
_iter_once:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x0
	ldp	x8, x9, [x0, #200]
	cmp	x8, x9
	b.lo	LBB1_3
	ldr	x0, [x19]
Lloh0:
	adrp	x8, SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh1:
	ldr	x8, [x8, SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
	ldr	x1, [x8]
	cbz	x1, LBB1_5
LBB1_2:
	add	x2, x19, #136
	add	x3, x19, #8
	mov	w4, #16
	bl	_objc_msgSend
	mov	x8, #0
	stp	xzr, x0, [x19, #200]
	cbz	x0, LBB1_4
LBB1_3:
	ldr	x9, [x19, #144]
	add	x10, x8, #1
	str	x10, [x19, #200]
	ldr	x0, [x9, x8, lsl #3]
LBB1_4:
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB1_5:
Lloh2:
	adrp	x1, l_anon.[ID].0@PAGE
Lloh3:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
	str	x0, [sp, #8]
	mov	x0, x8
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	x1, x0
	ldr	x0, [sp, #8]
	b	LBB1_2
	.loh AdrpLdrGot	Lloh0, Lloh1
	.loh AdrpAdd	Lloh2, Lloh3

	.globl	_use_obj
	.p2align	2
_use_obj:
	sub	sp, sp, #16
	str	x0, [sp, #8]
	add	x8, sp, #8
	; InlineAsm Start
	; InlineAsm End
	add	sp, sp, #16
	ret

	.globl	_iter
	.p2align	2
_iter:
	sub	sp, sp, #288
	stp	x28, x27, [sp, #224]
	stp	x22, x21, [sp, #240]
	stp	x20, x19, [sp, #256]
	stp	x29, x30, [sp, #272]
	add	x29, sp, #272
	mov	x9, #0
	mov	x8, #0
	stp	xzr, xzr, [sp, #200]
	movi.2d	v0, #0000000000000000
	stur	q0, [sp, #184]
	stur	q0, [sp, #168]
	add	x21, sp, #8
	stp	q0, q0, [sp, #16]
	stp	q0, q0, [sp, #48]
	stp	q0, q0, [sp, #80]
	stp	q0, q0, [sp, #112]
	str	x0, [sp, #8]
	stp	xzr, xzr, [sp, #152]
	str	xzr, [sp, #144]
	str	xzr, [sp, #216]
Lloh4:
	adrp	x19, l_anon.[ID].0@PAGE
Lloh5:
	add	x19, x19, l_anon.[ID].0@PAGEOFF
Lloh6:
	adrp	x20, SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh7:
	ldr	x20, [x20, SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
	cmp	x8, x9
	b.lo	LBB3_4
LBB3_1:
	ldr	x1, [x20]
	cbz	x1, LBB3_6
	add	x2, x21, #136
	add	x3, x21, #8
	mov	w4, #16
	bl	_objc_msgSend
	str	x0, [sp, #216]
	cbz	x0, LBB3_7
LBB3_3:
	mov	x8, #0
LBB3_4:
	ldr	x9, [sp, #152]
	add	x10, x8, #1
	str	x10, [sp, #208]
	ldr	x0, [x9, x8, lsl #3]
	cbz	x0, LBB3_7
	bl	_use_obj
	ldr	x0, [sp, #8]
	ldp	x8, x9, [sp, #208]
	cmp	x8, x9
	b.hs	LBB3_1
	b	LBB3_4
LBB3_6:
	mov	x22, x0
	mov	x0, x20
	mov	x1, x19
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	x1, x0
	mov	x0, x22
	add	x2, x21, #136
	add	x3, x21, #8
	mov	w4, #16
	bl	_objc_msgSend
	str	x0, [sp, #216]
	cbnz	x0, LBB3_3
LBB3_7:
	ldp	x29, x30, [sp, #272]
	ldp	x20, x19, [sp, #256]
	ldp	x22, x21, [sp, #240]
	ldp	x28, x27, [sp, #224]
	add	sp, sp, #288
	ret
	.loh AdrpLdrGot	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5

	.globl	_iter_noop
	.p2align	2
_iter_noop:
	sub	sp, sp, #288
	stp	x28, x27, [sp, #224]
	stp	x22, x21, [sp, #240]
	stp	x20, x19, [sp, #256]
	stp	x29, x30, [sp, #272]
	add	x29, sp, #272
	mov	x8, x0
	mov	x9, #0
	mov	x0, #0
	mov	x10, #0
	stp	xzr, xzr, [sp, #200]
	movi.2d	v0, #0000000000000000
	stur	q0, [sp, #184]
	stur	q0, [sp, #168]
	add	x21, sp, #8
	stp	q0, q0, [sp, #16]
	stp	q0, q0, [sp, #48]
	stp	q0, q0, [sp, #80]
	stp	q0, q0, [sp, #112]
	str	x8, [sp, #8]
	stp	xzr, xzr, [sp, #152]
	str	xzr, [sp, #144]
	str	xzr, [sp, #216]
Lloh8:
	adrp	x19, l_anon.[ID].0@PAGE
Lloh9:
	add	x19, x19, l_anon.[ID].0@PAGEOFF
Lloh10:
	adrp	x20, SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh11:
	ldr	x20, [x20, SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
	b	LBB4_2
LBB4_1:
	add	x8, x10, #1
	str	x8, [sp, #208]
	ldr	x11, [x9, x10, lsl #3]
	mov	x10, x8
	cbz	x11, LBB4_7
LBB4_2:
	cmp	x10, x0
	b.lo	LBB4_1
	ldr	x0, [sp, #8]
	ldr	x1, [x20]
	cbz	x1, LBB4_6
	add	x2, x21, #136
	add	x3, x21, #8
	mov	w4, #16
	bl	_objc_msgSend
	str	x0, [sp, #216]
	cbz	x0, LBB4_7
LBB4_5:
	mov	x10, #0
	ldr	x9, [sp, #152]
	b	LBB4_1
LBB4_6:
	mov	x22, x0
	mov	x0, x20
	mov	x1, x19
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	x1, x0
	mov	x0, x22
	add	x2, x21, #136
	add	x3, x21, #8
	mov	w4, #16
	bl	_objc_msgSend
	str	x0, [sp, #216]
	cbnz	x0, LBB4_5
LBB4_7:
	ldp	x29, x30, [sp, #272]
	ldp	x20, x19, [sp, #256]
	ldp	x22, x21, [sp, #240]
	ldp	x28, x27, [sp, #224]
	add	sp, sp, #288
	ret
	.loh AdrpLdrGot	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9

	.globl	_iter_retained
	.p2align	2
_iter_retained:
Lfunc_begin0:
	sub	sp, sp, #304
	stp	x24, x23, [sp, #240]
	stp	x22, x21, [sp, #256]
	stp	x20, x19, [sp, #272]
	stp	x29, x30, [sp, #288]
	add	x29, sp, #288
	mov	x9, #0
	mov	x8, #0
	stp	xzr, xzr, [sp, #216]
	movi.2d	v0, #0000000000000000
	stur	q0, [sp, #200]
	stur	q0, [sp, #184]
	add	x22, sp, #8
	stp	q0, q0, [sp, #32]
	stp	q0, q0, [sp, #64]
	stp	q0, q0, [sp, #96]
	stp	q0, q0, [sp, #128]
	str	xzr, [sp, #8]
	str	x0, [sp, #24]
	stp	xzr, xzr, [sp, #168]
	str	xzr, [sp, #160]
	mov	w23, #1
	str	xzr, [sp, #232]
Lloh12:
	adrp	x19, l_anon.[ID].0@PAGE
Lloh13:
	add	x19, x19, l_anon.[ID].0@PAGEOFF
Lloh14:
	adrp	x20, SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh15:
	ldr	x20, [x20, SYM(objc2_foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
	cmp	x8, x9
	b.lo	LBB5_5
LBB5_1:
	ldr	x1, [x20]
	cbz	x1, LBB5_12
	add	x2, x22, #152
	add	x3, x22, #24
	mov	w4, #16
	bl	_objc_msgSend
	stp	xzr, x0, [sp, #224]
	cbz	x0, LBB5_13
LBB5_3:
	ldr	x8, [sp, #168]
	cbz	x8, LBB5_14
	mov	x8, #0
LBB5_5:
	ldr	x9, [sp, #176]
	cbz	x9, LBB5_8
	ldr	x9, [x9]
	ldr	w10, [sp, #8]
	cmp	w10, #1
	b.ne	LBB5_9
	ldr	x10, [sp, #16]
	cmp	x10, x9
	b.ne	LBB5_15
LBB5_8:
	ldr	x9, [sp, #168]
	add	x10, x8, #1
	str	x10, [sp, #224]
	ldr	x0, [x9, x8, lsl #3]
	cbnz	x0, LBB5_10
	b	LBB5_13
LBB5_9:
	stp	x23, x9, [sp, #8]
	ldr	x9, [sp, #168]
	add	x10, x8, #1
	str	x10, [sp, #224]
	ldr	x0, [x9, x8, lsl #3]
	cbz	x0, LBB5_13
LBB5_10:
	bl	_objc_retain
	mov	x21, x0
Ltmp1:
	bl	_use_obj
Ltmp2:
	mov	x0, x21
	bl	_objc_release
	ldr	x0, [sp, #24]
	ldp	x8, x9, [sp, #224]
	cmp	x8, x9
	b.hs	LBB5_1
	b	LBB5_5
LBB5_12:
	mov	x21, x0
	mov	x0, x20
	mov	x1, x19
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	x1, x0
	mov	x0, x21
	add	x2, x22, #152
	add	x3, x22, #24
	mov	w4, #16
	bl	_objc_msgSend
	stp	xzr, x0, [sp, #224]
	cbnz	x0, LBB5_3
LBB5_13:
	ldp	x29, x30, [sp, #288]
	ldp	x20, x19, [sp, #272]
	ldp	x22, x21, [sp, #256]
	ldp	x24, x23, [sp, #240]
	add	sp, sp, #304
	ret
LBB5_14:
	bl	SYM(objc2_foundation::iter::items_ptr_null::GENERATED_ID, 0)
LBB5_15:
	bl	SYM(objc2_foundation::iter::mutation_detected::GENERATED_ID, 0)
LBB5_16:
Ltmp3:
	mov	x19, x0
Ltmp4:
	mov	x0, x21
	bl	_objc_release
Ltmp5:
	mov	x0, x19
	bl	__Unwind_Resume
LBB5_18:
Ltmp6:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.loh AdrpLdrGot	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table5:
Lexception0:
	.byte	255
	.byte	155
	.uleb128 Lttbase0-Lttbaseref0
Lttbaseref0:
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Lfunc_begin0-Lfunc_begin0
	.uleb128 Ltmp1-Lfunc_begin0
	.byte	0
	.byte	0
	.uleb128 Ltmp1-Lfunc_begin0
	.uleb128 Ltmp2-Ltmp1
	.uleb128 Ltmp3-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp2-Lfunc_begin0
	.uleb128 Ltmp4-Ltmp2
	.byte	0
	.byte	0
	.uleb128 Ltmp4-Lfunc_begin0
	.uleb128 Ltmp5-Ltmp4
	.uleb128 Ltmp6-Lfunc_begin0
	.byte	1
	.uleb128 Ltmp5-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp5
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
	.asciz	"countByEnumeratingWithState:objects:count:"

.subsections_via_symbols
