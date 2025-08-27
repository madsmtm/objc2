	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(objc2[CRATE_ID]::__macros::define_class::ivars::dealloc::<test_define_class_drop_ivars[CRATE_ID]::DropIvars>, 0):
Lfunc_begin0:
	sub	sp, sp, #64
	stp	x22, x21, [sp, #16]
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	mov	x19, x1
	mov	x20, x0
Lloh0:
	adrp	x8, ___DROP_FLAG_OFFSET_DropIvars@PAGE
Lloh1:
	ldr	x8, [x8, ___DROP_FLAG_OFFSET_DropIvars@PAGEOFF]
	ldrb	w8, [x0, x8]
	cmp	w8, #255
	b.ne	LBB0_6
	bl	_fn2_drop
LBB0_2:
Lloh2:
	adrp	x8, ___IVAR_OFFSET_DropIvars@PAGE
Lloh3:
	ldr	x8, [x8, ___IVAR_OFFSET_DropIvars@PAGEOFF]
	add	x8, x20, x8
	ldp	x0, x21, [x8]
Ltmp0:
	bl	_objc_release
Ltmp1:
	cbz	x21, LBB0_5
	mov	x0, x21
	bl	_objc_release
LBB0_5:
Lloh4:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh5:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh6:
	ldr	x8, [x8]
	stp	x20, x8, [sp]
	mov	x0, sp
	mov	x1, x19
	bl	_objc_msgSendSuper
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	ldp	x22, x21, [sp, #16]
	add	sp, sp, #64
	ret
LBB0_6:
	cbnz	w8, LBB0_2
	b	LBB0_5
LBB0_7:
Ltmp2:
	mov	x19, x0
	cbz	x21, LBB0_9
Ltmp3:
	mov	x0, x21
	bl	_objc_release
Ltmp4:
LBB0_9:
	mov	x0, x19
	bl	__Unwind_Resume
LBB0_10:
Ltmp5:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.loh AdrpLdr	Lloh0, Lloh1
	.loh AdrpLdr	Lloh2, Lloh3
	.loh AdrpLdrGotLdr	Lloh4, Lloh5, Lloh6
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table0:
Lexception0:
	.byte	255
	.byte	155
	.uleb128 Lttbase0-Lttbaseref0
Lttbaseref0:
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Lfunc_begin0-Lfunc_begin0
	.uleb128 Ltmp0-Lfunc_begin0
	.byte	0
	.byte	0
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp1-Ltmp0
	.uleb128 Ltmp2-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp1-Lfunc_begin0
	.uleb128 Ltmp3-Ltmp1
	.byte	0
	.byte	0
	.uleb128 Ltmp3-Lfunc_begin0
	.uleb128 Ltmp4-Ltmp3
	.uleb128 Ltmp5-Lfunc_begin0
	.byte	1
	.uleb128 Ltmp4-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp4
	.byte	0
	.byte	0
Lcst_end0:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class_drop_ivars[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin1:
	sub	sp, sp, #80
	stp	x20, x19, [sp, #48]
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	cmp	w9, #1
	b.ne	LBB1_9
Lloh7:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh8:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh9:
	ldr	x0, [x8]
Lloh10:
	adrp	x1, l_anon.[ID].9@PAGE
Lloh11:
	add	x1, x1, l_anon.[ID].9@PAGEOFF
	mov	x2, #0
	bl	_objc_allocateClassPair
	cbz	x0, LBB1_10
	str	x0, [sp]
Lloh12:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGE
Lloh13:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGEOFF]
Lloh14:
	ldr	x1, [x8]
Ltmp6:
Lloh15:
	adrp	x4, l_anon.[ID].5@PAGE
Lloh16:
	add	x4, x4, l_anon.[ID].5@PAGEOFF
Lloh17:
	adrp	x5, SYM(objc2[CRATE_ID]::__macros::define_class::ivars::dealloc::<test_define_class_drop_ivars[CRATE_ID]::DropIvars>, 0)@PAGE
Lloh18:
	add	x5, x5, SYM(objc2[CRATE_ID]::__macros::define_class::ivars::dealloc::<test_define_class_drop_ivars[CRATE_ID]::DropIvars>, 0)@PAGEOFF
	mov	x0, sp
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp7:
Lloh19:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh20:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh21:
	ldr	x1, [x8]
Ltmp8:
Lloh22:
	adrp	x4, l_anon.[ID].8@PAGE
Lloh23:
	add	x4, x4, l_anon.[ID].8@PAGEOFF
Lloh24:
	adrp	x5, _fn1_init@PAGE
Lloh25:
	add	x5, x5, _fn1_init@PAGEOFF
	mov	x0, sp
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp9:
	mov	w8, #2
Lloh26:
	adrp	x9, l_anon.[ID].2@PAGE
Lloh27:
	add	x9, x9, l_anon.[ID].2@PAGEOFF
	stp	x8, x9, [sp, #16]
	mov	w8, #27
	strb	w8, [sp, #8]
Ltmp10:
Lloh28:
	adrp	x1, l_anon.[ID].3@PAGE
Lloh29:
	add	x1, x1, l_anon.[ID].3@PAGEOFF
	mov	x0, sp
	add	x5, sp, #8
	mov	w2, #6
	mov	w3, #16
	mov	w4, #3
	bl	SYM(objc2::runtime::define::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Ltmp11:
Ltmp12:
Lloh30:
	adrp	x1, l_anon.[ID].4@PAGE
Lloh31:
	add	x1, x1, l_anon.[ID].4@PAGEOFF
Lloh32:
	adrp	x5, l_anon.[ID].1@PAGE
Lloh33:
	add	x5, x5, l_anon.[ID].1@PAGEOFF
	mov	x0, sp
	mov	w2, #10
	mov	w3, #1
	mov	w4, #0
	bl	SYM(objc2::runtime::define::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Ltmp13:
	ldr	x19, [sp]
	mov	x0, x19
	bl	_objc_registerClassPair
Lloh34:
	adrp	x1, l_anon.[ID].3@PAGE
Lloh35:
	add	x1, x1, l_anon.[ID].3@PAGEOFF
	mov	x0, x19
	bl	_class_getInstanceVariable
	cbz	x0, LBB1_11
	bl	_ivar_getOffset
	mov	x20, x0
Lloh36:
	adrp	x1, l_anon.[ID].4@PAGE
Lloh37:
	add	x1, x1, l_anon.[ID].4@PAGEOFF
	mov	x0, x19
	bl	_class_getInstanceVariable
	cbz	x0, LBB1_12
	bl	_ivar_getOffset
Lloh38:
	adrp	x8, ___CLASS_DropIvars@PAGE
	str	x19, [x8, ___CLASS_DropIvars@PAGEOFF]
Lloh39:
	adrp	x8, ___IVAR_OFFSET_DropIvars@PAGE
	str	x20, [x8, ___IVAR_OFFSET_DropIvars@PAGEOFF]
Lloh40:
	adrp	x8, ___DROP_FLAG_OFFSET_DropIvars@PAGE
	str	x0, [x8, ___DROP_FLAG_OFFSET_DropIvars@PAGEOFF]
	ldp	x29, x30, [sp, #64]
	ldp	x20, x19, [sp, #48]
	add	sp, sp, #80
	ret
LBB1_9:
Lloh41:
	adrp	x0, l_anon.[ID].7@PAGE
Lloh42:
	add	x0, x0, l_anon.[ID].7@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB1_10:
Lloh43:
	adrp	x0, l_anon.[ID].9@PAGE
Lloh44:
	add	x0, x0, l_anon.[ID].9@PAGEOFF
Lloh45:
	adrp	x2, l_anon.[ID].11@PAGE
Lloh46:
	add	x2, x2, l_anon.[ID].11@PAGEOFF
	mov	w1, #10
	bl	SYM(objc2::__macros::define_class::checks::class_not_unique::GENERATED_ID, 0)
LBB1_11:
	bl	SYM(objc2::__macros::define_class::ivars::ivars_offset::get_ivar_failed::GENERATED_ID, 0)
LBB1_12:
	bl	SYM(objc2::__macros::define_class::ivars::drop_flag_offset::get_drop_flag_failed::GENERATED_ID, 0)
LBB1_13:
Ltmp14:
	mov	x19, x0
	ldr	x0, [sp]
	bl	_objc_disposeClassPair
	mov	x0, x19
	bl	__Unwind_Resume
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpLdrGotLdr	Lloh7, Lloh8, Lloh9
	.loh AdrpAdd	Lloh17, Lloh18
	.loh AdrpAdd	Lloh15, Lloh16
	.loh AdrpLdrGotLdr	Lloh12, Lloh13, Lloh14
	.loh AdrpAdd	Lloh24, Lloh25
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpLdrGotLdr	Lloh19, Lloh20, Lloh21
	.loh AdrpAdd	Lloh28, Lloh29
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpAdd	Lloh32, Lloh33
	.loh AdrpAdd	Lloh30, Lloh31
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpAdd	Lloh36, Lloh37
	.loh AdrpAdrp	Lloh39, Lloh40
	.loh AdrpAdrp	Lloh38, Lloh39
	.loh AdrpAdd	Lloh41, Lloh42
	.loh AdrpAdd	Lloh45, Lloh46
	.loh AdrpAdd	Lloh43, Lloh44
Lfunc_end1:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table1:
Lexception1:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
	.uleb128 Ltmp6-Lfunc_begin1
	.uleb128 Ltmp13-Ltmp6
	.uleb128 Ltmp14-Lfunc_begin1
	.byte	0
	.uleb128 Ltmp13-Lfunc_begin1
	.uleb128 Lfunc_end1-Ltmp13
	.byte	0
	.byte	0
Lcst_end1:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(<<std[CRATE_ID]::sync::poison::once::Once>::call_once<<test_define_class_drop_ivars[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::poison::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	str	x8, [sp, #8]
	add	x0, sp, #8
	bl	SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class_drop_ivars[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.globl	SYM(<test_define_class_drop_ivars[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_define_class_drop_ivars[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh47:
	adrp	x8, ___REGISTER_CLASS_DropIvars@PAGE
Lloh48:
	add	x8, x8, ___REGISTER_CLASS_DropIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB3_2
Lloh49:
	adrp	x8, ___CLASS_DropIvars@PAGE
Lloh50:
	ldr	x0, [x8, ___CLASS_DropIvars@PAGEOFF]
	ret
LBB3_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh51:
	adrp	x0, ___REGISTER_CLASS_DropIvars@PAGE
Lloh52:
	add	x0, x0, ___REGISTER_CLASS_DropIvars@PAGEOFF
Lloh53:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh54:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh55:
	adrp	x4, l_anon.[ID].11@PAGE
Lloh56:
	add	x4, x4, l_anon.[ID].11@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh57:
	adrp	x8, ___CLASS_DropIvars@PAGE
Lloh58:
	ldr	x0, [x8, ___CLASS_DropIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh47, Lloh48
	.loh AdrpLdr	Lloh49, Lloh50
	.loh AdrpLdr	Lloh57, Lloh58
	.loh AdrpAdd	Lloh55, Lloh56
	.loh AdrpAdd	Lloh53, Lloh54
	.loh AdrpAdd	Lloh51, Lloh52

	.globl	_fn1_init
	.p2align	2
_fn1_init:
Lfunc_begin2:
	sub	sp, sp, #64
	stp	x22, x21, [sp, #16]
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	mov	x19, x0
Ltmp15:
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
Ltmp16:
Ltmp18:
	mov	x20, x0
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
Ltmp19:
	mov	x21, x0
	adrp	x22, ___DROP_FLAG_OFFSET_DropIvars@PAGE
	cbz	x19, LBB4_4
Lloh59:
	adrp	x8, ___IVAR_OFFSET_DropIvars@PAGE
Lloh60:
	ldr	x8, [x8, ___IVAR_OFFSET_DropIvars@PAGEOFF]
	add	x8, x19, x8
	stp	x20, x21, [x8]
	ldr	x8, [x22, ___DROP_FLAG_OFFSET_DropIvars@PAGEOFF]
	mov	w9, #15
	strb	w9, [x19, x8]
	b	LBB4_6
LBB4_4:
Ltmp26:
	mov	x0, x20
	bl	_objc_release
Ltmp27:
	mov	x0, x21
	bl	_objc_release
LBB4_6:
Lloh61:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh62:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh63:
	ldr	x1, [x8]
Lloh64:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh65:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh66:
	ldr	x8, [x8]
	stp	x19, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	cbz	x0, LBB4_8
	ldr	x8, [x22, ___DROP_FLAG_OFFSET_DropIvars@PAGEOFF]
	mov	w9, #255
	strb	w9, [x0, x8]
LBB4_8:
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	ldp	x22, x21, [sp, #16]
	add	sp, sp, #64
	ret
LBB4_9:
Ltmp28:
	mov	x20, x0
Ltmp29:
	mov	x0, x21
	bl	_objc_release
Ltmp30:
	b	LBB4_14
LBB4_10:
Ltmp31:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB4_11:
Ltmp20:
	mov	x8, x20
	mov	x20, x0
Ltmp21:
	mov	x0, x8
	bl	_objc_release
Ltmp22:
	b	LBB4_13
LBB4_12:
Ltmp17:
	mov	x20, x0
LBB4_13:
Ltmp23:
	mov	x0, x19
	bl	_objc_release
Ltmp24:
LBB4_14:
	mov	x0, x20
	bl	__Unwind_Resume
LBB4_15:
Ltmp25:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.loh AdrpLdr	Lloh59, Lloh60
	.loh AdrpLdrGotLdr	Lloh64, Lloh65, Lloh66
	.loh AdrpLdrGotLdr	Lloh61, Lloh62, Lloh63
Lfunc_end2:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table4:
Lexception2:
	.byte	255
	.byte	155
	.uleb128 Lttbase1-Lttbaseref1
Lttbaseref1:
	.byte	1
	.uleb128 Lcst_end2-Lcst_begin2
Lcst_begin2:
	.uleb128 Ltmp15-Lfunc_begin2
	.uleb128 Ltmp16-Ltmp15
	.uleb128 Ltmp17-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp18-Lfunc_begin2
	.uleb128 Ltmp19-Ltmp18
	.uleb128 Ltmp20-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp26-Lfunc_begin2
	.uleb128 Ltmp27-Ltmp26
	.uleb128 Ltmp28-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp27-Lfunc_begin2
	.uleb128 Ltmp29-Ltmp27
	.byte	0
	.byte	0
	.uleb128 Ltmp29-Lfunc_begin2
	.uleb128 Ltmp30-Ltmp29
	.uleb128 Ltmp31-Lfunc_begin2
	.byte	1
	.uleb128 Ltmp21-Lfunc_begin2
	.uleb128 Ltmp24-Ltmp21
	.uleb128 Ltmp25-Lfunc_begin2
	.byte	1
	.uleb128 Ltmp24-Lfunc_begin2
	.uleb128 Lfunc_end2-Ltmp24
	.byte	0
	.byte	0
Lcst_end2:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase1:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn2_drop
	.p2align	2
_fn2_drop:
	; InlineAsm Start
	; InlineAsm End
	ret

	.globl	_fn3_access_class
	.p2align	2
_fn3_access_class:
Lloh67:
	adrp	x8, ___REGISTER_CLASS_DropIvars@PAGE
Lloh68:
	add	x8, x8, ___REGISTER_CLASS_DropIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB6_2
Lloh69:
	adrp	x8, ___CLASS_DropIvars@PAGE
Lloh70:
	ldr	x0, [x8, ___CLASS_DropIvars@PAGEOFF]
	ret
LBB6_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh71:
	adrp	x0, ___REGISTER_CLASS_DropIvars@PAGE
Lloh72:
	add	x0, x0, ___REGISTER_CLASS_DropIvars@PAGEOFF
Lloh73:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh74:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh75:
	adrp	x4, l_anon.[ID].11@PAGE
Lloh76:
	add	x4, x4, l_anon.[ID].11@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh77:
	adrp	x8, ___CLASS_DropIvars@PAGE
Lloh78:
	ldr	x0, [x8, ___CLASS_DropIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh67, Lloh68
	.loh AdrpLdr	Lloh69, Lloh70
	.loh AdrpLdr	Lloh77, Lloh78
	.loh AdrpAdd	Lloh75, Lloh76
	.loh AdrpAdd	Lloh73, Lloh74
	.loh AdrpAdd	Lloh71, Lloh72

	.globl	_fn3_access_ivars
	.p2align	2
_fn3_access_ivars:
Lloh79:
	adrp	x8, ___IVAR_OFFSET_DropIvars@PAGE
Lloh80:
	ldr	x8, [x8, ___IVAR_OFFSET_DropIvars@PAGEOFF]
	add	x8, x0, x8
	ldp	x0, x1, [x8]
	ret
	.loh AdrpLdr	Lloh79, Lloh80

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::poison::once::Once>::call_once<<test_define_class_drop_ivars[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::poison::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class_drop_ivars[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].1:
	.byte	5
	.space	39

	.p2align	3, 0x0
l_anon.[ID].2:
	.byte	9
	.space	39

	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].3:
	.asciz	"ivars"

l_anon.[ID].4:
	.asciz	"drop_flag"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].5:
	.byte	17
	.space	39

	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].6:
	.asciz	"$RUSTC/library/std/src/sync/poison/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].7:
	.quad	l_anon.[ID].6
	.asciz	"\20x\000\000\000\000\000\000\000\234\000\000\0002\000\000"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].8:
	.byte	19
	.space	39

	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].9:
	.asciz	"DropIvars"

l_anon.[ID].10:
	.asciz	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].11:
	.quad	l_anon.[ID].10
	.asciz	"?\000\000\000\000\000\000\000\023\000\000\000\001\000\000"

	.globl	___CLASS_DropIvars
.zerofill __DATA,__common,___CLASS_DropIvars,8,3
	.globl	___DROP_FLAG_OFFSET_DropIvars
.zerofill __DATA,__common,___DROP_FLAG_OFFSET_DropIvars,8,3
	.globl	___IVAR_OFFSET_DropIvars
.zerofill __DATA,__common,___IVAR_OFFSET_DropIvars,8,3
	.section	__DATA,__data
	.globl	___REGISTER_CLASS_DropIvars
	.p2align	3, 0x0
___REGISTER_CLASS_DropIvars:
	.asciz	"\003\000\000\000\000\000\000"

.subsections_via_symbols
