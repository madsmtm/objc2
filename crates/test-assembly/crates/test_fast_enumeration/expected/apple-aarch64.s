	.section	__TEXT,__text,regular,pure_instructions
	.globl	_iter_create
	.p2align	2
_iter_create:
<<<<<<< HEAD
	str	x0, [x8]
	movi.2d	v0, #0000000000000000
	stur	q0, [x8, #8]
	stur	q0, [x8, #24]
	stur	q0, [x8, #40]
	stur	q0, [x8, #56]
	stur	q0, [x8, #72]
	stur	q0, [x8, #88]
	stur	q0, [x8, #104]
	stur	q0, [x8, #120]
	stur	q0, [x8, #136]
	stur	q0, [x8, #152]
	stur	q0, [x8, #168]
	stur	q0, [x8, #184]
	stur	q0, [x8, #200]
=======
	str	xzr, [x8, #72]
	movi.2d	v0, #0000000000000000
	stur	q0, [x8, #56]
	stur	q0, [x8, #40]
	stp	q0, q0, [x8, #80]
	stp	q0, q0, [x8, #112]
	stp	q0, q0, [x8, #144]
	stp	q0, q0, [x8, #176]
	str	xzr, [x8]
	stp	xzr, xzr, [x8, #24]
	str	xzr, [x8, #16]
	stp	xzr, xzr, [x8, #208]
	str	x0, [x8, #224]
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	ret

	.globl	_iter_once
	.p2align	2
_iter_once:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x0
	ldp	x8, x9, [x0, #208]
	cmp	x8, x9
<<<<<<< HEAD
	b.lo	LBB1_3
	ldr	x0, [x19]
=======
	b.lo	LBB1_5
	ldr	x21, [x19, #224]
	add	x20, x19, #80
	add	x22, x19, #16
>>>>>>> 11a7eeed2 (Fuzz array mutation)
Lloh0:
	adrp	x8, SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh1:
	ldr	x8, [x8, SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
	ldr	x1, [x8]
	cbz	x1, LBB1_12
LBB1_2:
	add	x2, x19, #136
	add	x3, x19, #8
	mov	w4, #16
	bl	_objc_msgSend
	stp	xzr, x0, [x19, #208]
	cbz	x0, LBB1_11
	ldr	x8, [x19, #24]
	cbz	x8, LBB1_13
	mov	x8, #0
LBB1_5:
	ldr	x9, [x19, #32]
	cbz	x9, LBB1_10
	ldr	x9, [x9]
	ldr	x10, [x19]
	cbz	x10, LBB1_9
	ldr	x10, [x19, #8]
	cmp	x10, x9
	b.eq	LBB1_10
	bl	SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)
LBB1_9:
	mov	w10, #1
	stp	x10, x9, [x19]
LBB1_10:
	ldr	x9, [x19, #24]
	add	x10, x8, #1
	str	x10, [x19, #208]
	ldr	x0, [x9, x8, lsl #3]
<<<<<<< HEAD
LBB1_4:
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB1_5:
Lloh2:
=======
LBB1_11:
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret
LBB1_12:
Lloh3:
	adrp	x0, SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh4:
	ldr	x0, [x0, SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
Lloh5:
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	adrp	x1, l_anon.[ID].0@PAGE
Lloh3:
	add	x1, x1, l_anon.[ID].0@PAGEOFF
	mov	x20, x0
	mov	x0, x8
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	x1, x0
	mov	x0, x20
	b	LBB1_2
<<<<<<< HEAD
	.loh AdrpLdrGot	Lloh0, Lloh1
	.loh AdrpAdd	Lloh2, Lloh3
=======
LBB1_13:
	bl	SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)
	.loh AdrpLdrGotLdr	Lloh0, Lloh1, Lloh2
	.loh AdrpAdd	Lloh5, Lloh6
	.loh AdrpLdrGot	Lloh3, Lloh4
>>>>>>> 11a7eeed2 (Fuzz array mutation)

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
<<<<<<< HEAD
	sub	sp, sp, #288
	stp	x28, x27, [sp, #224]
	stp	x22, x21, [sp, #240]
	stp	x20, x19, [sp, #256]
	stp	x29, x30, [sp, #272]
	add	x29, sp, #272
=======
	sub	sp, sp, #320
	stp	x28, x27, [sp, #240]
	stp	x24, x23, [sp, #256]
	stp	x22, x21, [sp, #272]
	stp	x20, x19, [sp, #288]
	stp	x29, x30, [sp, #304]
	add	x29, sp, #304
	mov	x23, x0
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	mov	x9, #0
	mov	x8, #0
	str	xzr, [sp, #80]
	movi.2d	v0, #0000000000000000
<<<<<<< HEAD
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
	adrp	x20, SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh7:
	ldr	x20, [x20, SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
	b	LBB3_2
=======
	stp	q0, q0, [sp, #48]
	add	x10, sp, #8
	add	x19, x10, #80
	stur	q0, [sp, #88]
	stur	q0, [sp, #104]
	stur	q0, [sp, #120]
	stur	q0, [sp, #136]
	stur	q0, [sp, #152]
	stur	q0, [sp, #168]
	stur	q0, [sp, #184]
	stur	q0, [sp, #200]
	add	x20, x10, #16
	str	xzr, [sp, #8]
	stp	xzr, xzr, [sp, #24]
	str	xzr, [sp, #40]
	stp	xzr, xzr, [sp, #216]
	mov	w24, #1
	str	x0, [sp, #232]
Lloh7:
	adrp	x21, l_anon.[ID].0@PAGE
Lloh8:
	add	x21, x21, l_anon.[ID].0@PAGEOFF
Lloh9:
	adrp	x22, SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh10:
	ldr	x22, [x22, SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
	b	LBB3_3
>>>>>>> 11a7eeed2 (Fuzz array mutation)
LBB3_1:
	stp	x24, x9, [sp, #8]
LBB3_2:
	ldr	x9, [sp, #32]
	add	x10, x8, #1
	str	x10, [sp, #216]
	ldr	x0, [x9, x8, lsl #3]
	bl	_use_obj
<<<<<<< HEAD
	ldr	x0, [sp, #8]
	ldp	x8, x9, [sp, #208]
LBB3_2:
	cmp	x8, x9
	b.lo	LBB3_1
	ldr	x1, [x20]
	cbz	x1, LBB3_6
	add	x2, x21, #136
	add	x3, x21, #8
	mov	w4, #16
	bl	_objc_msgSend
	str	x0, [sp, #216]
	cbz	x0, LBB3_7
LBB3_5:
	mov	x8, #0
	b	LBB3_1
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
	cbnz	x0, LBB3_5
LBB3_7:
	ldp	x29, x30, [sp, #272]
	ldp	x20, x19, [sp, #256]
	ldp	x22, x21, [sp, #240]
	ldp	x28, x27, [sp, #224]
	add	sp, sp, #288
	ret
	.loh AdrpLdrGot	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5
=======
	ldp	x9, x23, [sp, #224]
	ldr	x8, [sp, #216]
LBB3_3:
	cmp	x8, x9
	b.lo	LBB3_8
	ldr	x1, [x22]
	cbz	x1, LBB3_11
LBB3_5:
	mov	x0, x23
	mov	x2, x20
	mov	x3, x19
	mov	w4, #16
	bl	_objc_msgSend
	stp	xzr, x0, [sp, #216]
	cbz	x0, LBB3_12
	ldr	x8, [sp, #32]
	cbz	x8, LBB3_13
	mov	x8, #0
LBB3_8:
	ldr	x9, [sp, #40]
	cbz	x9, LBB3_2
	ldr	x9, [x9]
	ldr	x10, [sp, #8]
	cbz	x10, LBB3_1
	ldr	x10, [sp, #16]
	cmp	x10, x9
	b.eq	LBB3_2
	b	LBB3_14
LBB3_11:
	mov	x0, x22
	mov	x1, x21
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	x1, x0
	b	LBB3_5
LBB3_12:
	ldp	x29, x30, [sp, #304]
	ldp	x20, x19, [sp, #288]
	ldp	x22, x21, [sp, #272]
	ldp	x24, x23, [sp, #256]
	ldp	x28, x27, [sp, #240]
	add	sp, sp, #320
	ret
LBB3_13:
	bl	SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)
LBB3_14:
	bl	SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)
	.loh AdrpLdrGot	Lloh9, Lloh10
	.loh AdrpAdd	Lloh7, Lloh8
>>>>>>> 11a7eeed2 (Fuzz array mutation)

	.globl	_iter_noop
	.p2align	2
_iter_noop:
<<<<<<< HEAD
	sub	sp, sp, #288
	stp	x28, x27, [sp, #224]
	stp	x22, x21, [sp, #240]
	stp	x20, x19, [sp, #256]
	stp	x29, x30, [sp, #272]
	add	x29, sp, #272
	mov	x8, #0
	mov	x9, #0
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
Lloh8:
	adrp	x19, l_anon.[ID].0@PAGE
Lloh9:
	add	x19, x19, l_anon.[ID].0@PAGEOFF
Lloh10:
	adrp	x20, SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh11:
	ldr	x20, [x20, SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
	b	LBB4_2
LBB4_1:
	add	x9, x9, #1
	str	x9, [sp, #208]
LBB4_2:
	cmp	x9, x8
	b.lo	LBB4_1
	ldr	x1, [x20]
	cbz	x1, LBB4_6
	add	x2, x21, #136
	add	x3, x21, #8
	mov	w4, #16
	bl	_objc_msgSend
	str	x0, [sp, #216]
	cbz	x0, LBB4_7
LBB4_5:
	mov	x8, x0
	mov	x9, #0
	ldr	x0, [sp, #8]
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
=======
	sub	sp, sp, #320
	stp	x28, x27, [sp, #240]
	stp	x24, x23, [sp, #256]
	stp	x22, x21, [sp, #272]
	stp	x20, x19, [sp, #288]
	stp	x29, x30, [sp, #304]
	add	x29, sp, #304
	mov	x23, x0
	mov	x9, #0
	mov	x0, #0
	mov	x8, #0
	str	xzr, [sp, #80]
	movi.2d	v0, #0000000000000000
	stp	q0, q0, [sp, #48]
	add	x10, sp, #8
	add	x19, x10, #80
	stur	q0, [sp, #88]
	stur	q0, [sp, #104]
	stur	q0, [sp, #120]
	stur	q0, [sp, #136]
	stur	q0, [sp, #152]
	stur	q0, [sp, #168]
	stur	q0, [sp, #184]
	stur	q0, [sp, #200]
	add	x20, x10, #16
	str	xzr, [sp, #8]
	stp	xzr, xzr, [sp, #24]
	str	xzr, [sp, #40]
	stp	xzr, xzr, [sp, #216]
	mov	w24, #1
	str	x23, [sp, #232]
Lloh11:
	adrp	x21, l_anon.[ID].0@PAGE
Lloh12:
	add	x21, x21, l_anon.[ID].0@PAGEOFF
Lloh13:
	adrp	x22, SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh14:
	ldr	x22, [x22, SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
	b	LBB4_3
LBB4_1:
	stp	x24, x10, [sp, #8]
LBB4_2:
	add	x8, x8, #1
	str	x8, [sp, #216]
	ldr	x23, [sp, #232]
LBB4_3:
	cmp	x8, x0
	b.lo	LBB4_8
	ldr	x1, [x22]
	cbz	x1, LBB4_11
LBB4_5:
	mov	x0, x23
	mov	x2, x20
	mov	x3, x19
	mov	w4, #16
	bl	_objc_msgSend
	stp	xzr, x0, [sp, #216]
	cbz	x0, LBB4_12
	ldr	x8, [sp, #32]
	cbz	x8, LBB4_13
	mov	x8, #0
	ldr	x9, [sp, #40]
LBB4_8:
	cbz	x9, LBB4_2
	ldr	x10, [x9]
	ldr	x11, [sp, #8]
	cbz	x11, LBB4_1
	ldr	x11, [sp, #16]
	cmp	x11, x10
	b.eq	LBB4_2
	b	LBB4_14
LBB4_11:
	mov	x0, x22
	mov	x1, x21
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	x1, x0
	b	LBB4_5
LBB4_12:
	ldp	x29, x30, [sp, #304]
	ldp	x20, x19, [sp, #288]
	ldp	x22, x21, [sp, #272]
	ldp	x24, x23, [sp, #256]
	ldp	x28, x27, [sp, #240]
	add	sp, sp, #320
	ret
LBB4_13:
	bl	SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)
LBB4_14:
	bl	SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)
	.loh AdrpLdrGot	Lloh13, Lloh14
	.loh AdrpAdd	Lloh11, Lloh12
>>>>>>> 11a7eeed2 (Fuzz array mutation)

	.globl	_iter_retained
	.p2align	2
_iter_retained:
<<<<<<< HEAD
	sub	sp, sp, #288
	stp	x28, x27, [sp, #224]
	stp	x22, x21, [sp, #240]
	stp	x20, x19, [sp, #256]
	stp	x29, x30, [sp, #272]
	add	x29, sp, #272
=======
	sub	sp, sp, #320
	stp	x28, x27, [sp, #240]
	stp	x24, x23, [sp, #256]
	stp	x22, x21, [sp, #272]
	stp	x20, x19, [sp, #288]
	stp	x29, x30, [sp, #304]
	add	x29, sp, #304
	mov	x23, x0
>>>>>>> 11a7eeed2 (Fuzz array mutation)
	mov	x9, #0
	mov	x8, #0
	str	xzr, [sp, #80]
	movi.2d	v0, #0000000000000000
<<<<<<< HEAD
	stur	q0, [sp, #184]
	stur	q0, [sp, #168]
	add	x22, sp, #8
	stp	q0, q0, [sp, #16]
	stp	q0, q0, [sp, #48]
	stp	q0, q0, [sp, #80]
	stp	q0, q0, [sp, #112]
	str	x0, [sp, #8]
	stp	xzr, xzr, [sp, #152]
	str	xzr, [sp, #144]
	str	xzr, [sp, #216]
Lloh12:
	adrp	x19, l_anon.[ID].0@PAGE
Lloh13:
	add	x19, x19, l_anon.[ID].0@PAGEOFF
Lloh14:
	adrp	x20, SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh15:
	ldr	x20, [x20, SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
	b	LBB5_2
=======
	stp	q0, q0, [sp, #48]
	add	x10, sp, #8
	add	x19, x10, #80
	stur	q0, [sp, #88]
	stur	q0, [sp, #104]
	stur	q0, [sp, #120]
	stur	q0, [sp, #136]
	stur	q0, [sp, #152]
	stur	q0, [sp, #168]
	stur	q0, [sp, #184]
	stur	q0, [sp, #200]
	add	x20, x10, #16
	str	xzr, [sp, #8]
	stp	xzr, xzr, [sp, #24]
	str	xzr, [sp, #40]
	stp	xzr, xzr, [sp, #216]
	mov	w24, #1
	str	x0, [sp, #232]
Lloh15:
	adrp	x21, l_anon.[ID].0@PAGE
Lloh16:
	add	x21, x21, l_anon.[ID].0@PAGEOFF
Lloh17:
	adrp	x22, SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh18:
	ldr	x22, [x22, SYM(icrate::generated::Foundation::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
	b	LBB5_3
>>>>>>> 11a7eeed2 (Fuzz array mutation)
LBB5_1:
	stp	x24, x9, [sp, #8]
LBB5_2:
	ldr	x9, [sp, #32]
	add	x10, x8, #1
	str	x10, [sp, #216]
	ldr	x0, [x9, x8, lsl #3]
	bl	_objc_retain
	mov	x21, x0
	bl	_use_obj
	mov	x0, x21
	bl	_objc_release
<<<<<<< HEAD
	ldr	x0, [sp, #8]
	ldp	x8, x9, [sp, #208]
LBB5_2:
	cmp	x8, x9
	b.lo	LBB5_1
	ldr	x1, [x20]
	cbz	x1, LBB5_6
	add	x2, x22, #136
	add	x3, x22, #8
	mov	w4, #16
	bl	_objc_msgSend
	str	x0, [sp, #216]
	cbz	x0, LBB5_7
LBB5_5:
	mov	x8, #0
	b	LBB5_1
LBB5_6:
	mov	x21, x0
	mov	x0, x20
	mov	x1, x19
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	x1, x0
	mov	x0, x21
	add	x2, x22, #136
	add	x3, x22, #8
	mov	w4, #16
	bl	_objc_msgSend
	str	x0, [sp, #216]
	cbnz	x0, LBB5_5
LBB5_7:
	ldp	x29, x30, [sp, #272]
	ldp	x20, x19, [sp, #256]
	ldp	x22, x21, [sp, #240]
	ldp	x28, x27, [sp, #224]
	add	sp, sp, #288
	ret
	.loh AdrpLdrGot	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13
=======
	ldp	x9, x23, [sp, #224]
	ldr	x8, [sp, #216]
LBB5_3:
	cmp	x8, x9
	b.lo	LBB5_8
	ldr	x1, [x22]
	cbz	x1, LBB5_11
LBB5_5:
	mov	x0, x23
	mov	x2, x20
	mov	x3, x19
	mov	w4, #16
	bl	_objc_msgSend
	stp	xzr, x0, [sp, #216]
	cbz	x0, LBB5_12
	ldr	x8, [sp, #32]
	cbz	x8, LBB5_13
	mov	x8, #0
LBB5_8:
	ldr	x9, [sp, #40]
	cbz	x9, LBB5_2
	ldr	x9, [x9]
	ldr	x10, [sp, #8]
	cbz	x10, LBB5_1
	ldr	x10, [sp, #16]
	cmp	x10, x9
	b.eq	LBB5_2
	b	LBB5_14
LBB5_11:
	mov	x0, x22
	mov	x1, x21
	bl	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	x1, x0
	b	LBB5_5
LBB5_12:
	ldp	x29, x30, [sp, #304]
	ldp	x20, x19, [sp, #288]
	ldp	x22, x21, [sp, #272]
	ldp	x24, x23, [sp, #256]
	ldp	x28, x27, [sp, #240]
	add	sp, sp, #320
	ret
LBB5_13:
	bl	SYM(icrate::additions::Foundation::iter::items_ptr_null::GENERATED_ID, 0)
LBB5_14:
	bl	SYM(icrate::additions::Foundation::iter::mutation_detected::GENERATED_ID, 0)
	.loh AdrpLdrGot	Lloh17, Lloh18
	.loh AdrpAdd	Lloh15, Lloh16
>>>>>>> 11a7eeed2 (Fuzz array mutation)

	.section	__TEXT,__const
l_anon.[ID].0:
	.asciz	"countByEnumeratingWithState:objects:count:"

.subsections_via_symbols
