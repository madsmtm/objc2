	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(objc2[CRATE_ID]::__macro_helpers::defined_ivars::dealloc::<test_define_class[CRATE_ID]::DropIvars>, 0):
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
	bl	SYM(<test_define_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0)
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
SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin1:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	cmp	w9, #1
	b.ne	LBB1_15
Lloh7:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh8:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh9:
	ldr	x0, [x8]
Lloh10:
	adrp	x1, l_anon.[ID].13@PAGE
Lloh11:
	add	x1, x1, l_anon.[ID].13@PAGEOFF
	mov	x2, #0
	bl	_objc_allocateClassPair
	cbz	x0, LBB1_16
	str	x0, [sp, #8]
Lloh12:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_da7dfba076f8819b@PAGE
Lloh13:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_da7dfba076f8819b@PAGEOFF]
Ltmp6:
Lloh14:
	adrp	x4, l_anon.[ID].14@PAGE
Lloh15:
	add	x4, x4, l_anon.[ID].14@PAGEOFF
Lloh16:
	adrp	x5, _get_class@PAGE
Lloh17:
	add	x5, x5, _get_class@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::define::ClassBuilder::add_class_method_inner::GENERATED_ID, 0)
Ltmp7:
Lloh18:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_af51813a2f91b268@PAGE
Lloh19:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_af51813a2f91b268@PAGEOFF]
Ltmp8:
Lloh20:
	adrp	x4, l_anon.[ID].6@PAGE
Lloh21:
	add	x4, x4, l_anon.[ID].6@PAGEOFF
Lloh22:
	adrp	x5, _method_simple@PAGE
Lloh23:
	add	x5, x5, _method_simple@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp9:
Lloh24:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_43ff92728f706391@PAGE
Lloh25:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_43ff92728f706391@PAGEOFF]
Ltmp10:
Lloh26:
	adrp	x2, l_anon.[ID].15@PAGE
Lloh27:
	add	x2, x2, l_anon.[ID].15@PAGEOFF
Lloh28:
	adrp	x5, _method_bool@PAGE
Lloh29:
	add	x5, x5, _method_bool@PAGEOFF
	add	x0, sp, #8
	mov	w3, #1
	mov	x4, x2
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp11:
Lloh30:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_defcf1f00a4b4302@PAGE
Lloh31:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_defcf1f00a4b4302@PAGEOFF]
Ltmp12:
Lloh32:
	adrp	x4, l_anon.[ID].16@PAGE
Lloh33:
	add	x4, x4, l_anon.[ID].16@PAGEOFF
Lloh34:
	adrp	x5, _method_retained@PAGE
Lloh35:
	add	x5, x5, _method_retained@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp13:
Lloh36:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_9e0e96761ff41d49@PAGE
Lloh37:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_9e0e96761ff41d49@PAGEOFF]
Ltmp14:
Lloh38:
	adrp	x2, l_anon.[ID].15@PAGE
Lloh39:
	add	x2, x2, l_anon.[ID].15@PAGEOFF
Lloh40:
	adrp	x4, l_anon.[ID].16@PAGE
Lloh41:
	add	x4, x4, l_anon.[ID].16@PAGEOFF
Lloh42:
	adrp	x5, _method_retained_with_param@PAGE
Lloh43:
	add	x5, x5, _method_retained_with_param@PAGEOFF
	add	x0, sp, #8
	mov	w3, #1
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp15:
Ltmp16:
Lloh44:
	adrp	x0, l_anon.[ID].17@PAGE
Lloh45:
	add	x0, x0, l_anon.[ID].17@PAGEOFF
	mov	w1, #8
	bl	SYM(objc2::top_level_traits::get_protocol::GENERATED_ID, 0)
	mov	x1, x0
Ltmp17:
	cbz	x1, LBB1_10
	ldr	x0, [sp, #8]
	bl	_class_addProtocol
LBB1_10:
Ltmp18:
Lloh46:
	adrp	x0, l_anon.[ID].18@PAGE
Lloh47:
	add	x0, x0, l_anon.[ID].18@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::top_level_traits::get_protocol::GENERATED_ID, 0)
	mov	x1, x0
Ltmp19:
	cbz	x1, LBB1_13
	ldr	x0, [sp, #8]
	bl	_class_addProtocol
LBB1_13:
Lloh48:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_597b404e7f57234e@PAGE
Lloh49:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_597b404e7f57234e@PAGEOFF]
Ltmp20:
Lloh50:
	adrp	x2, l_anon.[ID].21@PAGE
Lloh51:
	add	x2, x2, l_anon.[ID].21@PAGEOFF
Lloh52:
	adrp	x4, l_anon.[ID].16@PAGE
Lloh53:
	add	x4, x4, l_anon.[ID].16@PAGEOFF
Lloh54:
	adrp	x5, _copyWithZone@PAGE
Lloh55:
	add	x5, x5, _copyWithZone@PAGEOFF
	add	x0, sp, #8
	mov	w3, #1
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp21:
	ldr	x19, [sp, #8]
	mov	x0, x19
	bl	_objc_registerClassPair
	adrp	x8, ___CLASS_NoIvars@PAGE
	str	x19, [x8, ___CLASS_NoIvars@PAGEOFF]
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB1_15:
Lloh56:
	adrp	x0, l_anon.[ID].10@PAGE
Lloh57:
	add	x0, x0, l_anon.[ID].10@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB1_16:
Lloh58:
	adrp	x0, l_anon.[ID].13@PAGE
Lloh59:
	add	x0, x0, l_anon.[ID].13@PAGEOFF
Lloh60:
	adrp	x2, l_anon.[ID].12@PAGE
Lloh61:
	add	x2, x2, l_anon.[ID].12@PAGEOFF
	mov	w1, #8
	bl	SYM(objc2::__macro_helpers::define_class::class_not_unique::GENERATED_ID, 0)
LBB1_17:
Ltmp22:
	mov	x19, x0
	ldr	x0, [sp, #8]
	bl	_objc_disposeClassPair
	mov	x0, x19
	bl	__Unwind_Resume
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpLdrGotLdr	Lloh7, Lloh8, Lloh9
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpLdr	Lloh12, Lloh13
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpLdr	Lloh18, Lloh19
	.loh AdrpAdd	Lloh28, Lloh29
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpLdr	Lloh24, Lloh25
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpAdd	Lloh32, Lloh33
	.loh AdrpLdr	Lloh30, Lloh31
	.loh AdrpAdd	Lloh42, Lloh43
	.loh AdrpAdd	Lloh40, Lloh41
	.loh AdrpAdd	Lloh38, Lloh39
	.loh AdrpLdr	Lloh36, Lloh37
	.loh AdrpAdd	Lloh44, Lloh45
	.loh AdrpAdd	Lloh46, Lloh47
	.loh AdrpAdd	Lloh54, Lloh55
	.loh AdrpAdd	Lloh52, Lloh53
	.loh AdrpAdd	Lloh50, Lloh51
	.loh AdrpLdr	Lloh48, Lloh49
	.loh AdrpAdd	Lloh56, Lloh57
	.loh AdrpAdd	Lloh60, Lloh61
	.loh AdrpAdd	Lloh58, Lloh59
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
	.uleb128 Ltmp21-Ltmp6
	.uleb128 Ltmp22-Lfunc_begin1
	.byte	0
	.uleb128 Ltmp21-Lfunc_begin1
	.uleb128 Lfunc_end1-Ltmp21
	.byte	0
	.byte	0
Lcst_end1:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin2:
	sub	sp, sp, #80
	stp	x20, x19, [sp, #48]
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	cmp	w9, #1
	b.ne	LBB2_9
Lloh62:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh63:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh64:
	ldr	x0, [x8]
Lloh65:
	adrp	x1, l_anon.[ID].25@PAGE
Lloh66:
	add	x1, x1, l_anon.[ID].25@PAGEOFF
	mov	x2, #0
	bl	_objc_allocateClassPair
	cbz	x0, LBB2_10
	str	x0, [sp]
Lloh67:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGE
Lloh68:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGEOFF]
Lloh69:
	ldr	x1, [x8]
Ltmp23:
Lloh70:
	adrp	x4, l_anon.[ID].6@PAGE
Lloh71:
	add	x4, x4, l_anon.[ID].6@PAGEOFF
Lloh72:
	adrp	x5, SYM(objc2[CRATE_ID]::__macro_helpers::defined_ivars::dealloc::<test_define_class[CRATE_ID]::DropIvars>, 0)@PAGE
Lloh73:
	add	x5, x5, SYM(objc2[CRATE_ID]::__macro_helpers::defined_ivars::dealloc::<test_define_class[CRATE_ID]::DropIvars>, 0)@PAGEOFF
	mov	x0, sp
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp24:
Lloh74:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh75:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh76:
	ldr	x1, [x8]
Ltmp25:
Lloh77:
	adrp	x4, l_anon.[ID].16@PAGE
Lloh78:
	add	x4, x4, l_anon.[ID].16@PAGEOFF
Lloh79:
	adrp	x5, _init_drop_ivars@PAGE
Lloh80:
	add	x5, x5, _init_drop_ivars@PAGEOFF
	mov	x0, sp
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp26:
	mov	w8, #2
Lloh81:
	adrp	x9, l_anon.[ID].8@PAGE
Lloh82:
	add	x9, x9, l_anon.[ID].8@PAGEOFF
	stp	x8, x9, [sp, #16]
	mov	w8, #27
	strb	w8, [sp, #8]
Ltmp27:
Lloh83:
	adrp	x1, l_anon.[ID].3@PAGE
Lloh84:
	add	x1, x1, l_anon.[ID].3@PAGEOFF
	mov	x0, sp
	add	x5, sp, #8
	mov	w2, #6
	mov	w3, #16
	mov	w4, #3
	bl	SYM(objc2::runtime::define::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Ltmp28:
Ltmp29:
Lloh85:
	adrp	x1, l_anon.[ID].4@PAGE
Lloh86:
	add	x1, x1, l_anon.[ID].4@PAGEOFF
Lloh87:
	adrp	x5, l_anon.[ID].5@PAGE
Lloh88:
	add	x5, x5, l_anon.[ID].5@PAGEOFF
	mov	x0, sp
	mov	w2, #10
	mov	w3, #1
	mov	w4, #0
	bl	SYM(objc2::runtime::define::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Ltmp30:
	ldr	x19, [sp]
	mov	x0, x19
	bl	_objc_registerClassPair
Lloh89:
	adrp	x1, l_anon.[ID].3@PAGE
Lloh90:
	add	x1, x1, l_anon.[ID].3@PAGEOFF
	mov	x0, x19
	bl	_class_getInstanceVariable
	cbz	x0, LBB2_11
	bl	_ivar_getOffset
	mov	x20, x0
Lloh91:
	adrp	x1, l_anon.[ID].4@PAGE
Lloh92:
	add	x1, x1, l_anon.[ID].4@PAGEOFF
	mov	x0, x19
	bl	_class_getInstanceVariable
	cbz	x0, LBB2_12
	bl	_ivar_getOffset
Lloh93:
	adrp	x8, ___CLASS_DropIvars@PAGE
	str	x19, [x8, ___CLASS_DropIvars@PAGEOFF]
Lloh94:
	adrp	x8, ___IVAR_OFFSET_DropIvars@PAGE
	str	x20, [x8, ___IVAR_OFFSET_DropIvars@PAGEOFF]
Lloh95:
	adrp	x8, ___DROP_FLAG_OFFSET_DropIvars@PAGE
	str	x0, [x8, ___DROP_FLAG_OFFSET_DropIvars@PAGEOFF]
	ldp	x29, x30, [sp, #64]
	ldp	x20, x19, [sp, #48]
	add	sp, sp, #80
	ret
LBB2_9:
Lloh96:
	adrp	x0, l_anon.[ID].10@PAGE
Lloh97:
	add	x0, x0, l_anon.[ID].10@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB2_10:
Lloh98:
	adrp	x0, l_anon.[ID].25@PAGE
Lloh99:
	add	x0, x0, l_anon.[ID].25@PAGEOFF
Lloh100:
	adrp	x2, l_anon.[ID].24@PAGE
Lloh101:
	add	x2, x2, l_anon.[ID].24@PAGEOFF
	mov	w1, #10
	bl	SYM(objc2::__macro_helpers::define_class::class_not_unique::GENERATED_ID, 0)
LBB2_11:
	bl	SYM(objc2::__macro_helpers::defined_ivars::ivars_offset::get_ivar_failed::GENERATED_ID, 0)
LBB2_12:
	bl	SYM(objc2::__macro_helpers::defined_ivars::drop_flag_offset::get_drop_flag_failed::GENERATED_ID, 0)
LBB2_13:
Ltmp31:
	mov	x19, x0
	ldr	x0, [sp]
	bl	_objc_disposeClassPair
	mov	x0, x19
	bl	__Unwind_Resume
	.loh AdrpAdd	Lloh65, Lloh66
	.loh AdrpLdrGotLdr	Lloh62, Lloh63, Lloh64
	.loh AdrpAdd	Lloh72, Lloh73
	.loh AdrpAdd	Lloh70, Lloh71
	.loh AdrpLdrGotLdr	Lloh67, Lloh68, Lloh69
	.loh AdrpAdd	Lloh79, Lloh80
	.loh AdrpAdd	Lloh77, Lloh78
	.loh AdrpLdrGotLdr	Lloh74, Lloh75, Lloh76
	.loh AdrpAdd	Lloh83, Lloh84
	.loh AdrpAdd	Lloh81, Lloh82
	.loh AdrpAdd	Lloh87, Lloh88
	.loh AdrpAdd	Lloh85, Lloh86
	.loh AdrpAdd	Lloh89, Lloh90
	.loh AdrpAdd	Lloh91, Lloh92
	.loh AdrpAdrp	Lloh94, Lloh95
	.loh AdrpAdrp	Lloh93, Lloh94
	.loh AdrpAdd	Lloh96, Lloh97
	.loh AdrpAdd	Lloh100, Lloh101
	.loh AdrpAdd	Lloh98, Lloh99
Lfunc_end2:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table2:
Lexception2:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end2-Lcst_begin2
Lcst_begin2:
	.uleb128 Ltmp23-Lfunc_begin2
	.uleb128 Ltmp30-Ltmp23
	.uleb128 Ltmp31-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp30-Lfunc_begin2
	.uleb128 Lfunc_end2-Ltmp30
	.byte	0
	.byte	0
Lcst_end2:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin3:
	sub	sp, sp, #80
	stp	x20, x19, [sp, #48]
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	cmp	w9, #1
	b.ne	LBB3_6
Lloh102:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh103:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh104:
	ldr	x0, [x8]
Lloh105:
	adrp	x1, l_anon.[ID].23@PAGE
Lloh106:
	add	x1, x1, l_anon.[ID].23@PAGEOFF
	mov	x2, #0
	bl	_objc_allocateClassPair
	cbz	x0, LBB3_7
	str	x0, [sp]
Lloh107:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh108:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh109:
	ldr	x1, [x8]
Ltmp32:
Lloh110:
	adrp	x4, l_anon.[ID].16@PAGE
Lloh111:
	add	x4, x4, l_anon.[ID].16@PAGEOFF
Lloh112:
	adrp	x5, _init_forgetable_ivars@PAGE
Lloh113:
	add	x5, x5, _init_forgetable_ivars@PAGEOFF
	mov	x0, sp
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp33:
	mov	w8, #2
Lloh114:
	adrp	x9, l_anon.[ID].7@PAGE
Lloh115:
	add	x9, x9, l_anon.[ID].7@PAGEOFF
	stp	x8, x9, [sp, #16]
	mov	w8, #27
	strb	w8, [sp, #8]
Ltmp34:
Lloh116:
	adrp	x1, l_anon.[ID].3@PAGE
Lloh117:
	add	x1, x1, l_anon.[ID].3@PAGEOFF
	mov	x0, sp
	add	x5, sp, #8
	mov	w2, #6
	mov	w3, #8
	mov	w4, #2
	bl	SYM(objc2::runtime::define::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Ltmp35:
	ldr	x19, [sp]
	mov	x0, x19
	bl	_objc_registerClassPair
Lloh118:
	adrp	x1, l_anon.[ID].3@PAGE
Lloh119:
	add	x1, x1, l_anon.[ID].3@PAGEOFF
	mov	x0, x19
	bl	_class_getInstanceVariable
	cbz	x0, LBB3_8
	bl	_ivar_getOffset
Lloh120:
	adrp	x8, ___CLASS_ForgetableIvars@PAGE
	str	x19, [x8, ___CLASS_ForgetableIvars@PAGEOFF]
Lloh121:
	adrp	x8, ___IVAR_OFFSET_ForgetableIvars@PAGE
	str	x0, [x8, ___IVAR_OFFSET_ForgetableIvars@PAGEOFF]
	ldp	x29, x30, [sp, #64]
	ldp	x20, x19, [sp, #48]
	add	sp, sp, #80
	ret
LBB3_6:
Lloh122:
	adrp	x0, l_anon.[ID].10@PAGE
Lloh123:
	add	x0, x0, l_anon.[ID].10@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB3_7:
Lloh124:
	adrp	x0, l_anon.[ID].23@PAGE
Lloh125:
	add	x0, x0, l_anon.[ID].23@PAGEOFF
Lloh126:
	adrp	x2, l_anon.[ID].22@PAGE
Lloh127:
	add	x2, x2, l_anon.[ID].22@PAGEOFF
	mov	w1, #16
	bl	SYM(objc2::__macro_helpers::define_class::class_not_unique::GENERATED_ID, 0)
LBB3_8:
	bl	SYM(objc2::__macro_helpers::defined_ivars::ivars_offset::get_ivar_failed::GENERATED_ID, 0)
LBB3_9:
Ltmp36:
	mov	x19, x0
	ldr	x0, [sp]
	bl	_objc_disposeClassPair
	mov	x0, x19
	bl	__Unwind_Resume
	.loh AdrpAdd	Lloh105, Lloh106
	.loh AdrpLdrGotLdr	Lloh102, Lloh103, Lloh104
	.loh AdrpAdd	Lloh112, Lloh113
	.loh AdrpAdd	Lloh110, Lloh111
	.loh AdrpLdrGotLdr	Lloh107, Lloh108, Lloh109
	.loh AdrpAdd	Lloh116, Lloh117
	.loh AdrpAdd	Lloh114, Lloh115
	.loh AdrpAdd	Lloh118, Lloh119
	.loh AdrpAdrp	Lloh120, Lloh121
	.loh AdrpAdd	Lloh122, Lloh123
	.loh AdrpAdd	Lloh126, Lloh127
	.loh AdrpAdd	Lloh124, Lloh125
Lfunc_end3:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table3:
Lexception3:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end3-Lcst_begin3
Lcst_begin3:
	.uleb128 Ltmp32-Lfunc_begin3
	.uleb128 Ltmp35-Ltmp32
	.uleb128 Ltmp36-Lfunc_begin3
	.byte	0
	.uleb128 Ltmp35-Lfunc_begin3
	.uleb128 Lfunc_end3-Ltmp35
	.byte	0
	.byte	0
Lcst_end3:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(<<std[CRATE_ID]::sync::poison::once::Once>::call_once<<test_define_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::poison::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	str	x8, [sp, #8]
	add	x0, sp, #8
	bl	SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.p2align	2
SYM(<<std[CRATE_ID]::sync::poison::once::Once>::call_once<<test_define_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::poison::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	str	x8, [sp, #8]
	add	x0, sp, #8
	bl	SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.p2align	2
SYM(<<std[CRATE_ID]::sync::poison::once::Once>::call_once<<test_define_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::poison::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	str	x8, [sp, #8]
	add	x0, sp, #8
	bl	SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.globl	_access_forgetable_ivars_class
	.p2align	2
_access_forgetable_ivars_class:
Lloh128:
	adrp	x8, ___REGISTER_CLASS_ForgetableIvars@PAGE
Lloh129:
	add	x8, x8, ___REGISTER_CLASS_ForgetableIvars@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB7_2
Lloh130:
	adrp	x8, ___CLASS_ForgetableIvars@PAGE
Lloh131:
	ldr	x0, [x8, ___CLASS_ForgetableIvars@PAGEOFF]
	ret
LBB7_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh132:
	adrp	x0, ___REGISTER_CLASS_ForgetableIvars@PAGE
Lloh133:
	add	x0, x0, ___REGISTER_CLASS_ForgetableIvars@PAGEOFF
Lloh134:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh135:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
Lloh136:
	adrp	x4, l_anon.[ID].22@PAGE
Lloh137:
	add	x4, x4, l_anon.[ID].22@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh138:
	adrp	x8, ___CLASS_ForgetableIvars@PAGE
Lloh139:
	ldr	x0, [x8, ___CLASS_ForgetableIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh128, Lloh129
	.loh AdrpLdr	Lloh130, Lloh131
	.loh AdrpLdr	Lloh138, Lloh139
	.loh AdrpAdd	Lloh136, Lloh137
	.loh AdrpAdd	Lloh134, Lloh135
	.loh AdrpAdd	Lloh132, Lloh133

	.globl	_access_forgetable_ivars
	.p2align	2
_access_forgetable_ivars:
Lloh140:
	adrp	x8, ___IVAR_OFFSET_ForgetableIvars@PAGE
Lloh141:
	ldr	x8, [x8, ___IVAR_OFFSET_ForgetableIvars@PAGEOFF]
	add	x8, x0, x8
	ldr	w1, [x8]
	ldrb	w0, [x8, #4]
	ret
	.loh AdrpLdr	Lloh140, Lloh141

	.globl	SYM(<test_define_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0)
	.p2align	2
SYM(<test_define_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0):
	; InlineAsm Start
	; InlineAsm End
	ret

	.globl	_access_drop_ivars_class
	.p2align	2
_access_drop_ivars_class:
Lloh142:
	adrp	x8, ___REGISTER_CLASS_DropIvars@PAGE
Lloh143:
	add	x8, x8, ___REGISTER_CLASS_DropIvars@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB10_2
Lloh144:
	adrp	x8, ___CLASS_DropIvars@PAGE
Lloh145:
	ldr	x0, [x8, ___CLASS_DropIvars@PAGEOFF]
	ret
LBB10_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh146:
	adrp	x0, ___REGISTER_CLASS_DropIvars@PAGE
Lloh147:
	add	x0, x0, ___REGISTER_CLASS_DropIvars@PAGEOFF
Lloh148:
	adrp	x3, l_anon.[ID].1@PAGE
Lloh149:
	add	x3, x3, l_anon.[ID].1@PAGEOFF
Lloh150:
	adrp	x4, l_anon.[ID].24@PAGE
Lloh151:
	add	x4, x4, l_anon.[ID].24@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh152:
	adrp	x8, ___CLASS_DropIvars@PAGE
Lloh153:
	ldr	x0, [x8, ___CLASS_DropIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh142, Lloh143
	.loh AdrpLdr	Lloh144, Lloh145
	.loh AdrpLdr	Lloh152, Lloh153
	.loh AdrpAdd	Lloh150, Lloh151
	.loh AdrpAdd	Lloh148, Lloh149
	.loh AdrpAdd	Lloh146, Lloh147

	.globl	_access_drop_ivars
	.p2align	2
_access_drop_ivars:
Lloh154:
	adrp	x8, ___IVAR_OFFSET_DropIvars@PAGE
Lloh155:
	ldr	x8, [x8, ___IVAR_OFFSET_DropIvars@PAGEOFF]
	add	x8, x0, x8
	ldp	x0, x1, [x8]
	ret
	.loh AdrpLdr	Lloh154, Lloh155

	.globl	SYM(<test_define_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_define_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh156:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh157:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB12_2
Lloh158:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh159:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
	ret
LBB12_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh160:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh161:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh162:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh163:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh164:
	adrp	x4, l_anon.[ID].12@PAGE
Lloh165:
	add	x4, x4, l_anon.[ID].12@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh166:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh167:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh156, Lloh157
	.loh AdrpLdr	Lloh158, Lloh159
	.loh AdrpLdr	Lloh166, Lloh167
	.loh AdrpAdd	Lloh164, Lloh165
	.loh AdrpAdd	Lloh162, Lloh163
	.loh AdrpAdd	Lloh160, Lloh161

	.globl	_get_class
	.p2align	2
_get_class:
Lloh168:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh169:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB13_2
Lloh170:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh171:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
	ret
LBB13_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh172:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh173:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh174:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh175:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh176:
	adrp	x4, l_anon.[ID].12@PAGE
Lloh177:
	add	x4, x4, l_anon.[ID].12@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh178:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh179:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh168, Lloh169
	.loh AdrpLdr	Lloh170, Lloh171
	.loh AdrpLdr	Lloh178, Lloh179
	.loh AdrpAdd	Lloh176, Lloh177
	.loh AdrpAdd	Lloh174, Lloh175
	.loh AdrpAdd	Lloh172, Lloh173

	.globl	_method_simple
	.p2align	2
_method_simple:
	ret

	.globl	_method_bool
	.p2align	2
_method_bool:
	eor	w0, w2, #0x1
	ret

	.globl	_method_retained
	.p2align	2
_method_retained:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh180:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh181:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB16_2
LBB16_1:
Lloh182:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh183:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
Lloh184:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh185:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh186:
	ldr	x1, [x8]
	bl	_objc_msgSend
	bl	_objc_autoreleaseReturnValue
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB16_2:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh187:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh188:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh189:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh190:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh191:
	adrp	x4, l_anon.[ID].12@PAGE
Lloh192:
	add	x4, x4, l_anon.[ID].12@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	b	LBB16_1
	.loh AdrpAdd	Lloh180, Lloh181
	.loh AdrpLdrGotLdr	Lloh184, Lloh185, Lloh186
	.loh AdrpAdrp	Lloh182, Lloh184
	.loh AdrpLdr	Lloh182, Lloh183
	.loh AdrpAdd	Lloh191, Lloh192
	.loh AdrpAdd	Lloh189, Lloh190
	.loh AdrpAdd	Lloh187, Lloh188

	.globl	_method_retained_with_param
	.p2align	2
_method_retained_with_param:
Lfunc_begin4:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x20, x2
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	x19, x0
	cbz	w20, LBB17_3
Ltmp37:
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
Ltmp38:
	mov	x20, x0
	mov	x0, x19
	bl	_objc_release
	mov	x19, x20
LBB17_3:
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_autoreleaseReturnValue
LBB17_4:
Ltmp39:
	mov	x20, x0
Ltmp40:
	mov	x0, x19
	bl	_objc_release
Ltmp41:
	mov	x0, x20
	bl	__Unwind_Resume
LBB17_6:
Ltmp42:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end4:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table17:
Lexception4:
	.byte	255
	.byte	155
	.uleb128 Lttbase1-Lttbaseref1
Lttbaseref1:
	.byte	1
	.uleb128 Lcst_end4-Lcst_begin4
Lcst_begin4:
	.uleb128 Lfunc_begin4-Lfunc_begin4
	.uleb128 Ltmp37-Lfunc_begin4
	.byte	0
	.byte	0
	.uleb128 Ltmp37-Lfunc_begin4
	.uleb128 Ltmp38-Ltmp37
	.uleb128 Ltmp39-Lfunc_begin4
	.byte	0
	.uleb128 Ltmp38-Lfunc_begin4
	.uleb128 Ltmp40-Ltmp38
	.byte	0
	.byte	0
	.uleb128 Ltmp40-Lfunc_begin4
	.uleb128 Ltmp41-Ltmp40
	.uleb128 Ltmp42-Lfunc_begin4
	.byte	1
	.uleb128 Ltmp41-Lfunc_begin4
	.uleb128 Lfunc_end4-Ltmp41
	.byte	0
	.byte	0
Lcst_end4:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase1:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_copyWithZone
	.p2align	2
_copyWithZone:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh193:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh194:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB18_2
LBB18_1:
Lloh195:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh196:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
Lloh197:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh198:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh199:
	ldr	x1, [x8]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB18_2:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh200:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh201:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh202:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh203:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh204:
	adrp	x4, l_anon.[ID].12@PAGE
Lloh205:
	add	x4, x4, l_anon.[ID].12@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	b	LBB18_1
	.loh AdrpAdd	Lloh193, Lloh194
	.loh AdrpLdrGotLdr	Lloh197, Lloh198, Lloh199
	.loh AdrpAdrp	Lloh195, Lloh197
	.loh AdrpLdr	Lloh195, Lloh196
	.loh AdrpAdd	Lloh204, Lloh205
	.loh AdrpAdd	Lloh202, Lloh203
	.loh AdrpAdd	Lloh200, Lloh201

	.globl	SYM(<test_define_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_define_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh206:
	adrp	x8, ___REGISTER_CLASS_ForgetableIvars@PAGE
Lloh207:
	add	x8, x8, ___REGISTER_CLASS_ForgetableIvars@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB19_2
Lloh208:
	adrp	x8, ___CLASS_ForgetableIvars@PAGE
Lloh209:
	ldr	x0, [x8, ___CLASS_ForgetableIvars@PAGEOFF]
	ret
LBB19_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh210:
	adrp	x0, ___REGISTER_CLASS_ForgetableIvars@PAGE
Lloh211:
	add	x0, x0, ___REGISTER_CLASS_ForgetableIvars@PAGEOFF
Lloh212:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh213:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
Lloh214:
	adrp	x4, l_anon.[ID].22@PAGE
Lloh215:
	add	x4, x4, l_anon.[ID].22@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh216:
	adrp	x8, ___CLASS_ForgetableIvars@PAGE
Lloh217:
	ldr	x0, [x8, ___CLASS_ForgetableIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh206, Lloh207
	.loh AdrpLdr	Lloh208, Lloh209
	.loh AdrpLdr	Lloh216, Lloh217
	.loh AdrpAdd	Lloh214, Lloh215
	.loh AdrpAdd	Lloh212, Lloh213
	.loh AdrpAdd	Lloh210, Lloh211

	.globl	_init_forgetable_ivars
	.p2align	2
_init_forgetable_ivars:
	cbz	x0, LBB20_2
Lloh218:
	adrp	x8, ___IVAR_OFFSET_ForgetableIvars@PAGE
Lloh219:
	ldr	x8, [x8, ___IVAR_OFFSET_ForgetableIvars@PAGEOFF]
	add	x8, x0, x8
	mov	w9, #43
	str	w9, [x8]
	mov	w9, #42
	strb	w9, [x8, #4]
LBB20_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh220:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh221:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh222:
	ldr	x1, [x8]
Lloh223:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh224:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh225:
	ldr	x8, [x8]
	stp	x0, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpLdr	Lloh218, Lloh219
	.loh AdrpLdrGotLdr	Lloh223, Lloh224, Lloh225
	.loh AdrpLdrGotLdr	Lloh220, Lloh221, Lloh222

	.globl	SYM(<test_define_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_define_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh226:
	adrp	x8, ___REGISTER_CLASS_DropIvars@PAGE
Lloh227:
	add	x8, x8, ___REGISTER_CLASS_DropIvars@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB21_2
Lloh228:
	adrp	x8, ___CLASS_DropIvars@PAGE
Lloh229:
	ldr	x0, [x8, ___CLASS_DropIvars@PAGEOFF]
	ret
LBB21_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh230:
	adrp	x0, ___REGISTER_CLASS_DropIvars@PAGE
Lloh231:
	add	x0, x0, ___REGISTER_CLASS_DropIvars@PAGEOFF
Lloh232:
	adrp	x3, l_anon.[ID].1@PAGE
Lloh233:
	add	x3, x3, l_anon.[ID].1@PAGEOFF
Lloh234:
	adrp	x4, l_anon.[ID].24@PAGE
Lloh235:
	add	x4, x4, l_anon.[ID].24@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh236:
	adrp	x8, ___CLASS_DropIvars@PAGE
Lloh237:
	ldr	x0, [x8, ___CLASS_DropIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh226, Lloh227
	.loh AdrpLdr	Lloh228, Lloh229
	.loh AdrpLdr	Lloh236, Lloh237
	.loh AdrpAdd	Lloh234, Lloh235
	.loh AdrpAdd	Lloh232, Lloh233
	.loh AdrpAdd	Lloh230, Lloh231

	.globl	_init_drop_ivars
	.p2align	2
_init_drop_ivars:
Lfunc_begin5:
	sub	sp, sp, #64
	stp	x22, x21, [sp, #16]
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	mov	x19, x0
Ltmp43:
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
Ltmp44:
Ltmp46:
	mov	x20, x0
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
Ltmp47:
	mov	x21, x0
	adrp	x22, ___DROP_FLAG_OFFSET_DropIvars@PAGE
	cbz	x19, LBB22_4
Lloh238:
	adrp	x8, ___IVAR_OFFSET_DropIvars@PAGE
Lloh239:
	ldr	x8, [x8, ___IVAR_OFFSET_DropIvars@PAGEOFF]
	add	x8, x19, x8
	stp	x20, x21, [x8]
	ldr	x8, [x22, ___DROP_FLAG_OFFSET_DropIvars@PAGEOFF]
	mov	w9, #15
	strb	w9, [x19, x8]
	b	LBB22_6
LBB22_4:
Ltmp54:
	mov	x0, x20
	bl	_objc_release
Ltmp55:
	mov	x0, x21
	bl	_objc_release
LBB22_6:
Lloh240:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh241:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh242:
	ldr	x1, [x8]
Lloh243:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh244:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh245:
	ldr	x8, [x8]
	stp	x19, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	cbz	x0, LBB22_8
	ldr	x8, [x22, ___DROP_FLAG_OFFSET_DropIvars@PAGEOFF]
	mov	w9, #255
	strb	w9, [x0, x8]
LBB22_8:
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	ldp	x22, x21, [sp, #16]
	add	sp, sp, #64
	ret
LBB22_9:
Ltmp56:
	mov	x20, x0
Ltmp57:
	mov	x0, x21
	bl	_objc_release
Ltmp58:
	b	LBB22_14
LBB22_10:
Ltmp59:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB22_11:
Ltmp48:
	mov	x8, x20
	mov	x20, x0
Ltmp49:
	mov	x0, x8
	bl	_objc_release
Ltmp50:
	b	LBB22_13
LBB22_12:
Ltmp45:
	mov	x20, x0
LBB22_13:
Ltmp51:
	mov	x0, x19
	bl	_objc_release
Ltmp52:
LBB22_14:
	mov	x0, x20
	bl	__Unwind_Resume
LBB22_15:
Ltmp53:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.loh AdrpLdr	Lloh238, Lloh239
	.loh AdrpLdrGotLdr	Lloh243, Lloh244, Lloh245
	.loh AdrpLdrGotLdr	Lloh240, Lloh241, Lloh242
Lfunc_end5:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table22:
Lexception5:
	.byte	255
	.byte	155
	.uleb128 Lttbase2-Lttbaseref2
Lttbaseref2:
	.byte	1
	.uleb128 Lcst_end5-Lcst_begin5
Lcst_begin5:
	.uleb128 Ltmp43-Lfunc_begin5
	.uleb128 Ltmp44-Ltmp43
	.uleb128 Ltmp45-Lfunc_begin5
	.byte	0
	.uleb128 Ltmp46-Lfunc_begin5
	.uleb128 Ltmp47-Ltmp46
	.uleb128 Ltmp48-Lfunc_begin5
	.byte	0
	.uleb128 Ltmp54-Lfunc_begin5
	.uleb128 Ltmp55-Ltmp54
	.uleb128 Ltmp56-Lfunc_begin5
	.byte	0
	.uleb128 Ltmp55-Lfunc_begin5
	.uleb128 Ltmp57-Ltmp55
	.byte	0
	.byte	0
	.uleb128 Ltmp57-Lfunc_begin5
	.uleb128 Ltmp58-Ltmp57
	.uleb128 Ltmp59-Lfunc_begin5
	.byte	1
	.uleb128 Ltmp49-Lfunc_begin5
	.uleb128 Ltmp52-Ltmp49
	.uleb128 Ltmp53-Lfunc_begin5
	.byte	1
	.uleb128 Ltmp52-Lfunc_begin5
	.uleb128 Lfunc_end5-Ltmp52
	.byte	0
	.byte	0
Lcst_end5:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase2:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_da7dfba076f8819b
L_OBJC_METH_VAR_NAME_da7dfba076f8819b:
	.asciz	"classMethod"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_da7dfba076f8819b
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_da7dfba076f8819b:
	.quad	L_OBJC_METH_VAR_NAME_da7dfba076f8819b

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_af51813a2f91b268
L_OBJC_METH_VAR_NAME_af51813a2f91b268:
	.asciz	"method"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_af51813a2f91b268
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_af51813a2f91b268:
	.quad	L_OBJC_METH_VAR_NAME_af51813a2f91b268

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_43ff92728f706391
L_OBJC_METH_VAR_NAME_43ff92728f706391:
	.asciz	"methodBool:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_43ff92728f706391
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_43ff92728f706391:
	.quad	L_OBJC_METH_VAR_NAME_43ff92728f706391

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_defcf1f00a4b4302
L_OBJC_METH_VAR_NAME_defcf1f00a4b4302:
	.asciz	"methodRetained"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_defcf1f00a4b4302
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_defcf1f00a4b4302:
	.quad	L_OBJC_METH_VAR_NAME_defcf1f00a4b4302

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_9e0e96761ff41d49
L_OBJC_METH_VAR_NAME_9e0e96761ff41d49:
	.asciz	"methodRetainedWithParam:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_9e0e96761ff41d49
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_9e0e96761ff41d49:
	.quad	L_OBJC_METH_VAR_NAME_9e0e96761ff41d49

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_597b404e7f57234e
L_OBJC_METH_VAR_NAME_597b404e7f57234e:
	.asciz	"copyWithZone:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_597b404e7f57234e
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_597b404e7f57234e:
	.quad	L_OBJC_METH_VAR_NAME_597b404e7f57234e

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::poison::once::Once>::call_once<<test_define_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::poison::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.p2align	3, 0x0
l_anon.[ID].1:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::poison::once::Once>::call_once<<test_define_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::poison::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.p2align	3, 0x0
l_anon.[ID].2:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::poison::once::Once>::call_once<<test_define_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::poison::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].3:
	.asciz	"ivars"

l_anon.[ID].4:
	.asciz	"drop_flag"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].5:
	.byte	5
	.space	39

	.p2align	3, 0x0
l_anon.[ID].6:
	.byte	17
	.space	39

	.p2align	3, 0x0
l_anon.[ID].7:
	.byte	7
	.space	39

	.p2align	3, 0x0
l_anon.[ID].8:
	.byte	9
	.space	39

l_anon.[ID].9:
	.ascii	"$RUSTC/library/std/src/sync/poison/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].10:
	.quad	l_anon.[ID].9
	.asciz	"\20x\000\000\000\000\000\000\000\233\000\000\0002\000\000"

	.globl	___CLASS_NoIvars
.zerofill __DATA,__common,___CLASS_NoIvars,8,3
	.globl	___IVAR_OFFSET_NoIvars
.zerofill __DATA,__common,___IVAR_OFFSET_NoIvars,8,3
	.globl	___DROP_FLAG_OFFSET_NoIvars
.zerofill __DATA,__common,___DROP_FLAG_OFFSET_NoIvars,8,3
	.section	__TEXT,__const
l_anon.[ID].11:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].12:
	.quad	l_anon.[ID].11
	.asciz	"4\000\000\000\000\000\000\000\016\000\000\000\001\000\000"

	.globl	___REGISTER_CLASS_NoIvars
.zerofill __DATA,__common,___REGISTER_CLASS_NoIvars,8,3
	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].13:
	.asciz	"NoIvars"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].14:
	.byte	21
	.space	39

	.p2align	3, 0x0
l_anon.[ID].15:
	.byte	16
	.space	39

	.p2align	3, 0x0
l_anon.[ID].16:
	.byte	19
	.space	39

	.section	__TEXT,__literal8,8byte_literals
l_anon.[ID].17:
	.ascii	"NSObject"

	.section	__TEXT,__const
l_anon.[ID].18:
	.ascii	"NSCopying"

l_anon.[ID].19:
	.ascii	"_NSZone"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].20:
	.byte	28
	.space	7
	.quad	l_anon.[ID].19
	.asciz	"\007\000\000\000\000\000\000"
	.quad	8
	.space	8

	.p2align	3, 0x0
l_anon.[ID].21:
	.byte	25
	.space	7
	.quad	l_anon.[ID].20
	.space	24

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_da7dfba076f8819b
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_da7dfba076f8819b:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_af51813a2f91b268
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_af51813a2f91b268:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_43ff92728f706391
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_43ff92728f706391:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_defcf1f00a4b4302
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_defcf1f00a4b4302:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_9e0e96761ff41d49
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_9e0e96761ff41d49:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_597b404e7f57234e
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_597b404e7f57234e:
	.asciz	"\000\000\000\000@\000\000"

	.globl	___CLASS_ForgetableIvars
.zerofill __DATA,__common,___CLASS_ForgetableIvars,8,3
	.globl	___IVAR_OFFSET_ForgetableIvars
.zerofill __DATA,__common,___IVAR_OFFSET_ForgetableIvars,8,3
	.globl	___DROP_FLAG_OFFSET_ForgetableIvars
.zerofill __DATA,__common,___DROP_FLAG_OFFSET_ForgetableIvars,8,3
	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].22:
	.quad	l_anon.[ID].11
	.asciz	"4\000\000\000\000\000\000\000M\000\000\000\001\000\000"

	.globl	___REGISTER_CLASS_ForgetableIvars
.zerofill __DATA,__common,___REGISTER_CLASS_ForgetableIvars,8,3
	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].23:
	.asciz	"ForgetableIvars"

	.globl	___CLASS_DropIvars
.zerofill __DATA,__common,___CLASS_DropIvars,8,3
	.globl	___IVAR_OFFSET_DropIvars
.zerofill __DATA,__common,___IVAR_OFFSET_DropIvars,8,3
	.globl	___DROP_FLAG_OFFSET_DropIvars
.zerofill __DATA,__common,___DROP_FLAG_OFFSET_DropIvars,8,3
	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].24:
	.quad	l_anon.[ID].11
	.asciz	"4\000\000\000\000\000\000\000o\000\000\000\001\000\000"

	.globl	___REGISTER_CLASS_DropIvars
.zerofill __DATA,__common,___REGISTER_CLASS_DropIvars,8,3
	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].25:
	.asciz	"DropIvars"

.subsections_via_symbols
