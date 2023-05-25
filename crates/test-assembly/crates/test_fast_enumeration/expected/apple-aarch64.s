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
	stp	x24, x23, [sp, #-64]!
	stp	x22, x21, [sp, #16]
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	mov	x19, x0
	ldp	x8, x9, [x0, #200]
	cmp	x8, x9
	b.lo	LBB1_4
	add	x20, x19, #8
	ldr	x21, [x19]
	add	x22, x19, #136
Lloh0:
	adrp	x23, SYM(icrate::Foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh1:
	ldr	x23, [x23, SYM(icrate::Foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
	ldr	x1, [x23]
	cbnz	x1, LBB1_3
Lloh2:
	adrp	x0, l_anon.[ID].0@PAGE
Lloh3:
	add	x0, x0, l_anon.[ID].0@PAGEOFF
	bl	SYM(objc2::runtime::Sel::register_unchecked::GENERATED_ID, 0)
	mov	x1, x0
	str	x0, [x23]
LBB1_3:
	mov	x0, x21
	mov	x2, x22
	mov	x3, x20
	mov	w4, #16
	bl	_objc_msgSend
	mov	x8, #0
	stp	xzr, x0, [x19, #200]
	cbz	x0, LBB1_5
LBB1_4:
	ldr	x9, [x19, #144]
	add	x10, x8, #1
	str	x10, [x19, #200]
	ldr	x0, [x9, x8, lsl #3]
LBB1_5:
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	ldp	x22, x21, [sp, #16]
	ldp	x24, x23, [sp], #64
	ret
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
	stp	x24, x23, [sp, #224]
	stp	x22, x21, [sp, #240]
	stp	x20, x19, [sp, #256]
	stp	x29, x30, [sp, #272]
	add	x29, sp, #272
	mov	x21, x0
	mov	x9, #0
	mov	x8, #0
	stp	xzr, xzr, [sp, #200]
	movi.2d	v0, #0000000000000000
	stur	q0, [sp, #184]
	stur	q0, [sp, #168]
	add	x10, sp, #8
	add	x19, x10, #8
	stp	q0, q0, [sp, #16]
	stp	q0, q0, [sp, #48]
	stp	q0, q0, [sp, #80]
	stp	q0, q0, [sp, #112]
	str	x0, [sp, #8]
	add	x20, x10, #136
	stp	xzr, xzr, [sp, #152]
	str	xzr, [sp, #144]
	str	xzr, [sp, #216]
Lloh4:
	adrp	x22, l_anon.[ID].0@PAGE
Lloh5:
	add	x22, x22, l_anon.[ID].0@PAGEOFF
Lloh6:
	adrp	x23, SYM(icrate::Foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh7:
	ldr	x23, [x23, SYM(icrate::Foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
	b	LBB3_2
LBB3_1:
	ldr	x9, [sp, #152]
	add	x10, x8, #1
	str	x10, [sp, #208]
	ldr	x0, [x9, x8, lsl #3]
	bl	_use_obj
	ldr	x21, [sp, #8]
	ldp	x8, x9, [sp, #208]
LBB3_2:
	cmp	x8, x9
	b.lo	LBB3_1
	ldr	x1, [x23]
	cbnz	x1, LBB3_5
	mov	x0, x22
	bl	SYM(objc2::runtime::Sel::register_unchecked::GENERATED_ID, 0)
	mov	x1, x0
	str	x0, [x23]
LBB3_5:
	mov	x0, x21
	mov	x2, x20
	mov	x3, x19
	mov	w4, #16
	bl	_objc_msgSend
	str	x0, [sp, #216]
	cbz	x0, LBB3_7
	mov	x8, #0
	b	LBB3_1
LBB3_7:
	ldp	x29, x30, [sp, #272]
	ldp	x20, x19, [sp, #256]
	ldp	x22, x21, [sp, #240]
	ldp	x24, x23, [sp, #224]
	add	sp, sp, #288
	ret
	.loh AdrpLdrGot	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5

	.globl	_iter_noop
	.p2align	2
_iter_noop:
	sub	sp, sp, #288
	stp	x24, x23, [sp, #224]
	stp	x22, x21, [sp, #240]
	stp	x20, x19, [sp, #256]
	stp	x29, x30, [sp, #272]
	add	x29, sp, #272
	mov	x20, x0
	mov	x0, #0
	mov	x8, #0
	stp	xzr, xzr, [sp, #200]
	movi.2d	v0, #0000000000000000
	stur	q0, [sp, #184]
	stur	q0, [sp, #168]
	add	x9, sp, #8
	add	x19, x9, #8
	stp	q0, q0, [sp, #16]
	stp	q0, q0, [sp, #48]
	stp	q0, q0, [sp, #80]
	stp	q0, q0, [sp, #112]
	str	x20, [sp, #8]
	add	x21, x9, #136
	stp	xzr, xzr, [sp, #152]
	str	xzr, [sp, #144]
	str	xzr, [sp, #216]
Lloh8:
	adrp	x22, l_anon.[ID].0@PAGE
Lloh9:
	add	x22, x22, l_anon.[ID].0@PAGEOFF
Lloh10:
	adrp	x23, SYM(icrate::Foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh11:
	ldr	x23, [x23, SYM(icrate::Foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
	b	LBB4_2
LBB4_1:
	add	x8, x8, #1
	str	x8, [sp, #208]
LBB4_2:
	cmp	x8, x0
	b.lo	LBB4_1
	ldr	x1, [x23]
	cbnz	x1, LBB4_5
	mov	x0, x22
	bl	SYM(objc2::runtime::Sel::register_unchecked::GENERATED_ID, 0)
	mov	x1, x0
	str	x0, [x23]
LBB4_5:
	mov	x0, x20
	mov	x2, x21
	mov	x3, x19
	mov	w4, #16
	bl	_objc_msgSend
	str	x0, [sp, #216]
	cbz	x0, LBB4_7
	mov	x8, #0
	ldr	x20, [sp, #8]
	b	LBB4_1
LBB4_7:
	ldp	x29, x30, [sp, #272]
	ldp	x20, x19, [sp, #256]
	ldp	x22, x21, [sp, #240]
	ldp	x24, x23, [sp, #224]
	add	sp, sp, #288
	ret
	.loh AdrpLdrGot	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9

	.globl	_iter_retained
	.p2align	2
_iter_retained:
	sub	sp, sp, #288
	stp	x24, x23, [sp, #224]
	stp	x22, x21, [sp, #240]
	stp	x20, x19, [sp, #256]
	stp	x29, x30, [sp, #272]
	add	x29, sp, #272
	mov	x22, x0
	mov	x9, #0
	mov	x8, #0
	stp	xzr, xzr, [sp, #200]
	movi.2d	v0, #0000000000000000
	stur	q0, [sp, #184]
	stur	q0, [sp, #168]
	add	x10, sp, #8
	add	x19, x10, #8
	stp	q0, q0, [sp, #16]
	stp	q0, q0, [sp, #48]
	stp	q0, q0, [sp, #80]
	stp	q0, q0, [sp, #112]
	str	x0, [sp, #8]
	add	x20, x10, #136
	stp	xzr, xzr, [sp, #152]
	str	xzr, [sp, #144]
	str	xzr, [sp, #216]
Lloh12:
	adrp	x21, l_anon.[ID].0@PAGE
Lloh13:
	add	x21, x21, l_anon.[ID].0@PAGEOFF
Lloh14:
	adrp	x23, SYM(icrate::Foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGE
Lloh15:
	ldr	x23, [x23, SYM(icrate::Foundation::generated::__NSEnumerator::NSFastEnumeration::countByEnumeratingWithState_objects_count::CACHED_SEL::GENERATED_ID, 0)@GOTPAGEOFF]
	b	LBB5_2
LBB5_1:
	ldr	x9, [sp, #152]
	add	x10, x8, #1
	str	x10, [sp, #208]
	ldr	x0, [x9, x8, lsl #3]
	bl	_objc_retain
	mov	x22, x0
	bl	_use_obj
	mov	x0, x22
	bl	_objc_release
	ldr	x22, [sp, #8]
	ldp	x8, x9, [sp, #208]
LBB5_2:
	cmp	x8, x9
	b.lo	LBB5_1
	ldr	x1, [x23]
	cbnz	x1, LBB5_5
	mov	x0, x21
	bl	SYM(objc2::runtime::Sel::register_unchecked::GENERATED_ID, 0)
	mov	x1, x0
	str	x0, [x23]
LBB5_5:
	mov	x0, x22
	mov	x2, x20
	mov	x3, x19
	mov	w4, #16
	bl	_objc_msgSend
	str	x0, [sp, #216]
	cbz	x0, LBB5_7
	mov	x8, #0
	b	LBB5_1
LBB5_7:
	ldp	x29, x30, [sp, #272]
	ldp	x20, x19, [sp, #256]
	ldp	x22, x21, [sp, #240]
	ldp	x24, x23, [sp, #224]
	add	sp, sp, #288
	ret
	.loh AdrpLdrGot	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13

	.section	__TEXT,__const
l_anon.[ID].0:
	.asciz	"countByEnumeratingWithState:objects:count:"

.subsections_via_symbols
