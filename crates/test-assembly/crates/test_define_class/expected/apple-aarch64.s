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
	adrp	x8, SYM(test_define_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGE
Lloh1:
	ldr	x8, [x8, SYM(test_define_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGEOFF]
	ldrb	w8, [x0, x8]
	cbz	w8, LBB0_6
	cmp	w8, #255
	b.ne	LBB0_3
	bl	SYM(<test_define_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0)
LBB0_3:
Lloh2:
	adrp	x8, SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
Lloh3:
	ldr	x8, [x8, SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGEOFF]
	add	x8, x20, x8
	ldp	x0, x21, [x8]
Ltmp0:
	bl	_objc_release
Ltmp1:
	cbz	x21, LBB0_6
	mov	x0, x21
	bl	_objc_release
LBB0_6:
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
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin1:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	tbz	w9, #0, LBB1_14
Lloh7:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh8:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh9:
	ldr	x2, [x8]
Lloh10:
	adrp	x0, l_anon.[ID].13@PAGE
Lloh11:
	add	x0, x0, l_anon.[ID].13@PAGEOFF
Lloh12:
	adrp	x3, l_anon.[ID].15@PAGE
Lloh13:
	add	x3, x3, l_anon.[ID].15@PAGEOFF
	mov	w1, #7
	bl	SYM(objc2::__macro_helpers::define_class::create_builder::GENERATED_ID, 0)
	str	x0, [sp, #8]
Lloh14:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_da7dfba076f8819b@PAGE
Lloh15:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_da7dfba076f8819b@PAGEOFF]
Ltmp6:
Lloh16:
	adrp	x4, l_anon.[ID].16@PAGE
Lloh17:
	add	x4, x4, l_anon.[ID].16@PAGEOFF
Lloh18:
	adrp	x5, _get_class@PAGE
Lloh19:
	add	x5, x5, _get_class@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::define::ClassBuilder::add_class_method_inner::GENERATED_ID, 0)
Ltmp7:
Lloh20:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_af51813a2f91b268@PAGE
Lloh21:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_af51813a2f91b268@PAGEOFF]
Ltmp8:
Lloh22:
	adrp	x4, l_anon.[ID].3@PAGE
Lloh23:
	add	x4, x4, l_anon.[ID].3@PAGEOFF
Lloh24:
	adrp	x5, _method_simple@PAGE
Lloh25:
	add	x5, x5, _method_simple@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp9:
Lloh26:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_43ff92728f706391@PAGE
Lloh27:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_43ff92728f706391@PAGEOFF]
Ltmp10:
Lloh28:
	adrp	x2, l_anon.[ID].17@PAGE
Lloh29:
	add	x2, x2, l_anon.[ID].17@PAGEOFF
Lloh30:
	adrp	x5, _method_bool@PAGE
Lloh31:
	add	x5, x5, _method_bool@PAGEOFF
	add	x0, sp, #8
	mov	w3, #1
	mov	x4, x2
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp11:
Lloh32:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_defcf1f00a4b4302@PAGE
Lloh33:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_defcf1f00a4b4302@PAGEOFF]
Ltmp12:
Lloh34:
	adrp	x4, l_anon.[ID].18@PAGE
Lloh35:
	add	x4, x4, l_anon.[ID].18@PAGEOFF
Lloh36:
	adrp	x5, _method_retained@PAGE
Lloh37:
	add	x5, x5, _method_retained@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp13:
Lloh38:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_9e0e96761ff41d49@PAGE
Lloh39:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_9e0e96761ff41d49@PAGEOFF]
Ltmp14:
Lloh40:
	adrp	x2, l_anon.[ID].17@PAGE
Lloh41:
	add	x2, x2, l_anon.[ID].17@PAGEOFF
Lloh42:
	adrp	x4, l_anon.[ID].18@PAGE
Lloh43:
	add	x4, x4, l_anon.[ID].18@PAGEOFF
Lloh44:
	adrp	x5, _method_retained_with_param@PAGE
Lloh45:
	add	x5, x5, _method_retained_with_param@PAGEOFF
	add	x0, sp, #8
	mov	w3, #1
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp15:
Ltmp16:
Lloh46:
	adrp	x0, l_anon.[ID].19@PAGE
Lloh47:
	add	x0, x0, l_anon.[ID].19@PAGEOFF
	mov	w1, #8
	bl	SYM(objc2::top_level_traits::get_protocol::GENERATED_ID, 0)
	mov	x1, x0
Ltmp17:
	cbz	x1, LBB1_9
	ldr	x0, [sp, #8]
	bl	_class_addProtocol
LBB1_9:
Ltmp18:
Lloh48:
	adrp	x0, l_anon.[ID].20@PAGE
Lloh49:
	add	x0, x0, l_anon.[ID].20@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::top_level_traits::get_protocol::GENERATED_ID, 0)
	mov	x1, x0
Ltmp19:
	cbz	x1, LBB1_12
	ldr	x0, [sp, #8]
	bl	_class_addProtocol
LBB1_12:
Lloh50:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_597b404e7f57234e@PAGE
Lloh51:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_597b404e7f57234e@PAGEOFF]
Ltmp20:
Lloh52:
	adrp	x2, l_anon.[ID].23@PAGE
Lloh53:
	add	x2, x2, l_anon.[ID].23@PAGEOFF
Lloh54:
	adrp	x4, l_anon.[ID].18@PAGE
Lloh55:
	add	x4, x4, l_anon.[ID].18@PAGEOFF
Lloh56:
	adrp	x5, _copyWithZone@PAGE
Lloh57:
	add	x5, x5, _copyWithZone@PAGEOFF
	add	x0, sp, #8
	mov	w3, #1
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp21:
	ldr	x19, [sp, #8]
	mov	x0, x19
	bl	_objc_registerClassPair
	adrp	x8, __MergedGlobals@PAGE
	str	x19, [x8, __MergedGlobals@PAGEOFF]
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB1_14:
Lloh58:
	adrp	x0, l_anon.[ID].12@PAGE
Lloh59:
	add	x0, x0, l_anon.[ID].12@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB1_15:
Ltmp22:
	mov	x19, x0
	ldr	x0, [sp, #8]
	bl	_objc_disposeClassPair
	mov	x0, x19
	bl	__Unwind_Resume
	.loh AdrpAdd	Lloh18, Lloh19
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpLdr	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpLdrGotLdr	Lloh7, Lloh8, Lloh9
	.loh AdrpAdd	Lloh24, Lloh25
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpLdr	Lloh20, Lloh21
	.loh AdrpAdd	Lloh30, Lloh31
	.loh AdrpAdd	Lloh28, Lloh29
	.loh AdrpLdr	Lloh26, Lloh27
	.loh AdrpAdd	Lloh36, Lloh37
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpLdr	Lloh32, Lloh33
	.loh AdrpAdd	Lloh44, Lloh45
	.loh AdrpAdd	Lloh42, Lloh43
	.loh AdrpAdd	Lloh40, Lloh41
	.loh AdrpLdr	Lloh38, Lloh39
	.loh AdrpAdd	Lloh46, Lloh47
	.loh AdrpAdd	Lloh48, Lloh49
	.loh AdrpAdd	Lloh56, Lloh57
	.loh AdrpAdd	Lloh54, Lloh55
	.loh AdrpAdd	Lloh52, Lloh53
	.loh AdrpLdr	Lloh50, Lloh51
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
	.uleb128 Lfunc_begin1-Lfunc_begin1
	.uleb128 Ltmp6-Lfunc_begin1
	.byte	0
	.byte	0
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
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin2:
	sub	sp, sp, #96
	stp	x20, x19, [sp, #64]
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	tbz	w9, #0, LBB2_8
Lloh60:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh61:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh62:
	ldr	x2, [x8]
Lloh63:
	adrp	x0, l_anon.[ID].10@PAGE
Lloh64:
	add	x0, x0, l_anon.[ID].10@PAGEOFF
Lloh65:
	adrp	x3, l_anon.[ID].25@PAGE
Lloh66:
	add	x3, x3, l_anon.[ID].25@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::__macro_helpers::define_class::create_builder::GENERATED_ID, 0)
	str	x0, [sp, #24]
Lloh67:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGE
Lloh68:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGEOFF]
Lloh69:
	ldr	x1, [x8]
Ltmp23:
Lloh70:
	adrp	x4, l_anon.[ID].3@PAGE
Lloh71:
	add	x4, x4, l_anon.[ID].3@PAGEOFF
Lloh72:
	adrp	x5, SYM(objc2[CRATE_ID]::__macro_helpers::defined_ivars::dealloc::<test_define_class[CRATE_ID]::DropIvars>, 0)@PAGE
Lloh73:
	add	x5, x5, SYM(objc2[CRATE_ID]::__macro_helpers::defined_ivars::dealloc::<test_define_class[CRATE_ID]::DropIvars>, 0)@PAGEOFF
	add	x0, sp, #24
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp24:
	ldr	x8, [sp, #24]
	str	x8, [sp, #8]
Lloh74:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh75:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh76:
	ldr	x1, [x8]
Ltmp26:
Lloh77:
	adrp	x4, l_anon.[ID].18@PAGE
Lloh78:
	add	x4, x4, l_anon.[ID].18@PAGEOFF
Lloh79:
	adrp	x5, _init_drop_ivars@PAGE
Lloh80:
	add	x5, x5, _init_drop_ivars@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp27:
	ldr	x8, [sp, #8]
	str	x8, [sp, #16]
	mov	w8, #16
Lloh81:
	adrp	x9, l_anon.[ID].8@PAGE
Lloh82:
	add	x9, x9, l_anon.[ID].8@PAGEOFF
	stp	x8, x9, [sp, #32]
	mov	w8, #27
	strb	w8, [sp, #24]
Ltmp29:
Lloh83:
	adrp	x1, l_anon.[ID].4@PAGE
Lloh84:
	add	x1, x1, l_anon.[ID].4@PAGEOFF
	add	x0, sp, #16
	add	x5, sp, #24
	mov	w2, #6
	mov	w3, #16
	mov	w4, #3
	bl	SYM(objc2::runtime::define::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Ltmp30:
Ltmp31:
Lloh85:
	adrp	x1, l_anon.[ID].5@PAGE
Lloh86:
	add	x1, x1, l_anon.[ID].5@PAGEOFF
Lloh87:
	adrp	x5, l_anon.[ID].6@PAGE
Lloh88:
	add	x5, x5, l_anon.[ID].6@PAGEOFF
	add	x0, sp, #16
	mov	w2, #10
	mov	w3, #1
	mov	w4, #0
	bl	SYM(objc2::runtime::define::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Ltmp32:
	ldr	x19, [sp, #16]
	mov	x0, x19
	bl	_objc_registerClassPair
Lloh89:
	adrp	x1, l_anon.[ID].4@PAGE
Lloh90:
	add	x1, x1, l_anon.[ID].4@PAGEOFF
	mov	x0, x19
	bl	_class_getInstanceVariable
	cbz	x0, LBB2_9
	bl	_ivar_getOffset
	mov	x20, x0
Lloh91:
	adrp	x1, l_anon.[ID].5@PAGE
Lloh92:
	add	x1, x1, l_anon.[ID].5@PAGEOFF
	mov	x0, x19
	bl	_class_getInstanceVariable
	cbz	x0, LBB2_10
	bl	_ivar_getOffset
Lloh93:
	adrp	x8, __MergedGlobals@PAGE+32
	str	x19, [x8, __MergedGlobals@PAGEOFF+32]
Lloh94:
	adrp	x8, SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
	str	x20, [x8, SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGEOFF]
Lloh95:
	adrp	x8, SYM(test_define_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGE
	str	x0, [x8, SYM(test_define_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGEOFF]
	ldp	x29, x30, [sp, #80]
	ldp	x20, x19, [sp, #64]
	add	sp, sp, #96
	ret
LBB2_8:
Lloh96:
	adrp	x0, l_anon.[ID].12@PAGE
Lloh97:
	add	x0, x0, l_anon.[ID].12@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB2_9:
	bl	SYM(objc2::__macro_helpers::defined_ivars::register_with_ivars::get_ivar_failed::GENERATED_ID, 0)
LBB2_10:
	bl	SYM(objc2::__macro_helpers::defined_ivars::register_with_ivars::get_drop_flag_failed::GENERATED_ID, 0)
LBB2_11:
Ltmp28:
	mov	x19, x0
	add	x8, sp, #8
	ldr	x0, [x8]
	bl	_objc_disposeClassPair
	mov	x0, x19
	bl	__Unwind_Resume
LBB2_12:
Ltmp25:
	mov	x19, x0
	add	x8, sp, #24
	ldr	x0, [x8]
	bl	_objc_disposeClassPair
	mov	x0, x19
	bl	__Unwind_Resume
LBB2_13:
Ltmp33:
	mov	x19, x0
	add	x8, sp, #16
	ldr	x0, [x8]
	bl	_objc_disposeClassPair
	mov	x0, x19
	bl	__Unwind_Resume
	.loh AdrpAdd	Lloh72, Lloh73
	.loh AdrpAdd	Lloh70, Lloh71
	.loh AdrpLdrGotLdr	Lloh67, Lloh68, Lloh69
	.loh AdrpAdd	Lloh65, Lloh66
	.loh AdrpAdd	Lloh63, Lloh64
	.loh AdrpLdrGotLdr	Lloh60, Lloh61, Lloh62
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
	.uleb128 Lfunc_begin2-Lfunc_begin2
	.uleb128 Ltmp23-Lfunc_begin2
	.byte	0
	.byte	0
	.uleb128 Ltmp23-Lfunc_begin2
	.uleb128 Ltmp24-Ltmp23
	.uleb128 Ltmp25-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp26-Lfunc_begin2
	.uleb128 Ltmp27-Ltmp26
	.uleb128 Ltmp28-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp29-Lfunc_begin2
	.uleb128 Ltmp32-Ltmp29
	.uleb128 Ltmp33-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp32-Lfunc_begin2
	.uleb128 Lfunc_end2-Ltmp32
	.byte	0
	.byte	0
Lcst_end2:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin3:
	sub	sp, sp, #96
	stp	x20, x19, [sp, #64]
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	tbz	w9, #0, LBB3_5
Lloh98:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh99:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh100:
	ldr	x2, [x8]
Lloh101:
	adrp	x0, l_anon.[ID].9@PAGE
Lloh102:
	add	x0, x0, l_anon.[ID].9@PAGEOFF
Lloh103:
	adrp	x3, l_anon.[ID].24@PAGE
Lloh104:
	add	x3, x3, l_anon.[ID].24@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::__macro_helpers::define_class::create_builder::GENERATED_ID, 0)
	str	x0, [sp, #8]
Lloh105:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh106:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh107:
	ldr	x1, [x8]
Ltmp34:
Lloh108:
	adrp	x4, l_anon.[ID].18@PAGE
Lloh109:
	add	x4, x4, l_anon.[ID].18@PAGEOFF
Lloh110:
	adrp	x5, _init_forgetable_ivars@PAGE
Lloh111:
	add	x5, x5, _init_forgetable_ivars@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp35:
	ldr	x8, [sp, #8]
	str	x8, [sp, #16]
	mov	w8, #8
Lloh112:
	adrp	x9, l_anon.[ID].7@PAGE
Lloh113:
	add	x9, x9, l_anon.[ID].7@PAGEOFF
	stp	x8, x9, [sp, #32]
	mov	w8, #27
	strb	w8, [sp, #24]
Ltmp37:
Lloh114:
	adrp	x1, l_anon.[ID].4@PAGE
Lloh115:
	add	x1, x1, l_anon.[ID].4@PAGEOFF
	add	x0, sp, #16
	add	x5, sp, #24
	mov	w2, #6
	mov	w3, #8
	mov	w4, #2
	bl	SYM(objc2::runtime::define::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Ltmp38:
	ldr	x19, [sp, #16]
	mov	x0, x19
	bl	_objc_registerClassPair
Lloh116:
	adrp	x1, l_anon.[ID].4@PAGE
Lloh117:
	add	x1, x1, l_anon.[ID].4@PAGEOFF
	mov	x0, x19
	bl	_class_getInstanceVariable
	cbz	x0, LBB3_6
	bl	_ivar_getOffset
Lloh118:
	adrp	x8, __MergedGlobals@PAGE+16
	str	x19, [x8, __MergedGlobals@PAGEOFF+16]
Lloh119:
	adrp	x8, SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGE
	str	x0, [x8, SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGEOFF]
	ldp	x29, x30, [sp, #80]
	ldp	x20, x19, [sp, #64]
	add	sp, sp, #96
	ret
LBB3_5:
Lloh120:
	adrp	x0, l_anon.[ID].12@PAGE
Lloh121:
	add	x0, x0, l_anon.[ID].12@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB3_6:
	bl	SYM(objc2::__macro_helpers::defined_ivars::register_with_ivars::get_ivar_failed::GENERATED_ID, 0)
LBB3_7:
Ltmp39:
	mov	x19, x0
	add	x8, sp, #16
	ldr	x0, [x8]
	bl	_objc_disposeClassPair
	mov	x0, x19
	bl	__Unwind_Resume
LBB3_8:
Ltmp36:
	mov	x19, x0
	add	x8, sp, #8
	ldr	x0, [x8]
	bl	_objc_disposeClassPair
	mov	x0, x19
	bl	__Unwind_Resume
	.loh AdrpAdd	Lloh110, Lloh111
	.loh AdrpAdd	Lloh108, Lloh109
	.loh AdrpLdrGotLdr	Lloh105, Lloh106, Lloh107
	.loh AdrpAdd	Lloh103, Lloh104
	.loh AdrpAdd	Lloh101, Lloh102
	.loh AdrpLdrGotLdr	Lloh98, Lloh99, Lloh100
	.loh AdrpAdd	Lloh114, Lloh115
	.loh AdrpAdd	Lloh112, Lloh113
	.loh AdrpAdd	Lloh116, Lloh117
	.loh AdrpAdrp	Lloh118, Lloh119
	.loh AdrpAdd	Lloh120, Lloh121
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
	.uleb128 Lfunc_begin3-Lfunc_begin3
	.uleb128 Ltmp34-Lfunc_begin3
	.byte	0
	.byte	0
	.uleb128 Ltmp34-Lfunc_begin3
	.uleb128 Ltmp35-Ltmp34
	.uleb128 Ltmp36-Lfunc_begin3
	.byte	0
	.uleb128 Ltmp37-Lfunc_begin3
	.uleb128 Ltmp38-Ltmp37
	.uleb128 Ltmp39-Lfunc_begin3
	.byte	0
	.uleb128 Ltmp38-Lfunc_begin3
	.uleb128 Lfunc_end3-Ltmp38
	.byte	0
	.byte	0
Lcst_end3:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_define_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	str	x8, [sp, #8]
	add	x0, sp, #8
	bl	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.p2align	2
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_define_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	str	x8, [sp, #8]
	add	x0, sp, #8
	bl	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.p2align	2
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_define_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	str	x8, [sp, #8]
	add	x0, sp, #8
	bl	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.globl	_access_forgetable_ivars_class
	.p2align	2
_access_forgetable_ivars_class:
Lloh122:
	adrp	x8, __MergedGlobals@PAGE+24
Lloh123:
	add	x8, x8, __MergedGlobals@PAGEOFF+24
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB7_2
Lloh124:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh125:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
	ret
LBB7_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh126:
	adrp	x0, __MergedGlobals@PAGE+24
Lloh127:
	add	x0, x0, __MergedGlobals@PAGEOFF+24
Lloh128:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh129:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
Lloh130:
	adrp	x4, l_anon.[ID].24@PAGE
Lloh131:
	add	x4, x4, l_anon.[ID].24@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh132:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh133:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
	ret
	.loh AdrpAdd	Lloh122, Lloh123
	.loh AdrpLdr	Lloh124, Lloh125
	.loh AdrpLdr	Lloh132, Lloh133
	.loh AdrpAdd	Lloh130, Lloh131
	.loh AdrpAdd	Lloh128, Lloh129
	.loh AdrpAdd	Lloh126, Lloh127

	.globl	_access_forgetable_ivars
	.p2align	2
_access_forgetable_ivars:
Lloh134:
	adrp	x8, SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGE
Lloh135:
	ldr	x8, [x8, SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGEOFF]
	add	x8, x0, x8
	ldr	w1, [x8]
	ldrb	w0, [x8, #4]
	ret
	.loh AdrpLdr	Lloh134, Lloh135

	.globl	SYM(<test_define_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0)
	.p2align	2
SYM(<test_define_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0):
	; InlineAsm Start
	; InlineAsm End
	ret

	.globl	_access_drop_ivars_class
	.p2align	2
_access_drop_ivars_class:
Lloh136:
	adrp	x8, __MergedGlobals@PAGE+40
Lloh137:
	add	x8, x8, __MergedGlobals@PAGEOFF+40
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB10_2
Lloh138:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh139:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+32]
	ret
LBB10_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh140:
	adrp	x0, __MergedGlobals@PAGE+40
Lloh141:
	add	x0, x0, __MergedGlobals@PAGEOFF+40
Lloh142:
	adrp	x3, l_anon.[ID].1@PAGE
Lloh143:
	add	x3, x3, l_anon.[ID].1@PAGEOFF
Lloh144:
	adrp	x4, l_anon.[ID].25@PAGE
Lloh145:
	add	x4, x4, l_anon.[ID].25@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh146:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh147:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+32]
	ret
	.loh AdrpAdd	Lloh136, Lloh137
	.loh AdrpLdr	Lloh138, Lloh139
	.loh AdrpLdr	Lloh146, Lloh147
	.loh AdrpAdd	Lloh144, Lloh145
	.loh AdrpAdd	Lloh142, Lloh143
	.loh AdrpAdd	Lloh140, Lloh141

	.globl	_access_drop_ivars
	.p2align	2
_access_drop_ivars:
Lloh148:
	adrp	x8, SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
Lloh149:
	ldr	x8, [x8, SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGEOFF]
	add	x8, x0, x8
	ldp	x0, x1, [x8]
	ret
	.loh AdrpLdr	Lloh148, Lloh149

	.globl	SYM(<test_define_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_define_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh150:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh151:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB12_2
Lloh152:
	adrp	x8, __MergedGlobals@PAGE
Lloh153:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
	ret
LBB12_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh154:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh155:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh156:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh157:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh158:
	adrp	x4, l_anon.[ID].15@PAGE
Lloh159:
	add	x4, x4, l_anon.[ID].15@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh160:
	adrp	x8, __MergedGlobals@PAGE
Lloh161:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh150, Lloh151
	.loh AdrpLdr	Lloh152, Lloh153
	.loh AdrpLdr	Lloh160, Lloh161
	.loh AdrpAdd	Lloh158, Lloh159
	.loh AdrpAdd	Lloh156, Lloh157
	.loh AdrpAdd	Lloh154, Lloh155

	.globl	_get_class
	.p2align	2
_get_class:
Lloh162:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh163:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB13_2
Lloh164:
	adrp	x8, __MergedGlobals@PAGE
Lloh165:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
	ret
LBB13_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh166:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh167:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh168:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh169:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh170:
	adrp	x4, l_anon.[ID].15@PAGE
Lloh171:
	add	x4, x4, l_anon.[ID].15@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh172:
	adrp	x8, __MergedGlobals@PAGE
Lloh173:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh162, Lloh163
	.loh AdrpLdr	Lloh164, Lloh165
	.loh AdrpLdr	Lloh172, Lloh173
	.loh AdrpAdd	Lloh170, Lloh171
	.loh AdrpAdd	Lloh168, Lloh169
	.loh AdrpAdd	Lloh166, Lloh167

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
Lloh174:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh175:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB16_2
LBB16_1:
Lloh176:
	adrp	x8, __MergedGlobals@PAGE
Lloh177:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
Lloh178:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh179:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh180:
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
Lloh181:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh182:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh183:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh184:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh185:
	adrp	x4, l_anon.[ID].15@PAGE
Lloh186:
	add	x4, x4, l_anon.[ID].15@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	b	LBB16_1
	.loh AdrpAdd	Lloh174, Lloh175
	.loh AdrpLdrGotLdr	Lloh178, Lloh179, Lloh180
	.loh AdrpAdrp	Lloh176, Lloh178
	.loh AdrpLdr	Lloh176, Lloh177
	.loh AdrpAdd	Lloh185, Lloh186
	.loh AdrpAdd	Lloh183, Lloh184
	.loh AdrpAdd	Lloh181, Lloh182

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
Ltmp40:
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
Ltmp41:
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
Ltmp42:
	mov	x20, x0
Ltmp43:
	mov	x0, x19
	bl	_objc_release
Ltmp44:
	mov	x0, x20
	bl	__Unwind_Resume
LBB17_6:
Ltmp45:
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
	.uleb128 Ltmp40-Lfunc_begin4
	.byte	0
	.byte	0
	.uleb128 Ltmp40-Lfunc_begin4
	.uleb128 Ltmp41-Ltmp40
	.uleb128 Ltmp42-Lfunc_begin4
	.byte	0
	.uleb128 Ltmp41-Lfunc_begin4
	.uleb128 Ltmp43-Ltmp41
	.byte	0
	.byte	0
	.uleb128 Ltmp43-Lfunc_begin4
	.uleb128 Ltmp44-Ltmp43
	.uleb128 Ltmp45-Lfunc_begin4
	.byte	1
	.uleb128 Ltmp44-Lfunc_begin4
	.uleb128 Lfunc_end4-Ltmp44
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
Lloh187:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh188:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB18_2
LBB18_1:
Lloh189:
	adrp	x8, __MergedGlobals@PAGE
Lloh190:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
Lloh191:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh192:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh193:
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
Lloh194:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh195:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh196:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh197:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh198:
	adrp	x4, l_anon.[ID].15@PAGE
Lloh199:
	add	x4, x4, l_anon.[ID].15@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	b	LBB18_1
	.loh AdrpAdd	Lloh187, Lloh188
	.loh AdrpLdrGotLdr	Lloh191, Lloh192, Lloh193
	.loh AdrpAdrp	Lloh189, Lloh191
	.loh AdrpLdr	Lloh189, Lloh190
	.loh AdrpAdd	Lloh198, Lloh199
	.loh AdrpAdd	Lloh196, Lloh197
	.loh AdrpAdd	Lloh194, Lloh195

	.globl	SYM(<test_define_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_define_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh200:
	adrp	x8, __MergedGlobals@PAGE+24
Lloh201:
	add	x8, x8, __MergedGlobals@PAGEOFF+24
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB19_2
Lloh202:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh203:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
	ret
LBB19_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh204:
	adrp	x0, __MergedGlobals@PAGE+24
Lloh205:
	add	x0, x0, __MergedGlobals@PAGEOFF+24
Lloh206:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh207:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
Lloh208:
	adrp	x4, l_anon.[ID].24@PAGE
Lloh209:
	add	x4, x4, l_anon.[ID].24@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh210:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh211:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
	ret
	.loh AdrpAdd	Lloh200, Lloh201
	.loh AdrpLdr	Lloh202, Lloh203
	.loh AdrpLdr	Lloh210, Lloh211
	.loh AdrpAdd	Lloh208, Lloh209
	.loh AdrpAdd	Lloh206, Lloh207
	.loh AdrpAdd	Lloh204, Lloh205

	.globl	_init_forgetable_ivars
	.p2align	2
_init_forgetable_ivars:
	cbz	x0, LBB20_2
Lloh212:
	adrp	x8, SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGE
Lloh213:
	ldr	x8, [x8, SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGEOFF]
	add	x8, x0, x8
	mov	w9, #43
	str	w9, [x8]
	mov	w9, #42
	strb	w9, [x8, #4]
LBB20_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh214:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh215:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh216:
	ldr	x1, [x8]
Lloh217:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh218:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh219:
	ldr	x8, [x8]
	stp	x0, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpLdr	Lloh212, Lloh213
	.loh AdrpLdrGotLdr	Lloh217, Lloh218, Lloh219
	.loh AdrpLdrGotLdr	Lloh214, Lloh215, Lloh216

	.globl	SYM(<test_define_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_define_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh220:
	adrp	x8, __MergedGlobals@PAGE+40
Lloh221:
	add	x8, x8, __MergedGlobals@PAGEOFF+40
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB21_2
Lloh222:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh223:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+32]
	ret
LBB21_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh224:
	adrp	x0, __MergedGlobals@PAGE+40
Lloh225:
	add	x0, x0, __MergedGlobals@PAGEOFF+40
Lloh226:
	adrp	x3, l_anon.[ID].1@PAGE
Lloh227:
	add	x3, x3, l_anon.[ID].1@PAGEOFF
Lloh228:
	adrp	x4, l_anon.[ID].25@PAGE
Lloh229:
	add	x4, x4, l_anon.[ID].25@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh230:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh231:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+32]
	ret
	.loh AdrpAdd	Lloh220, Lloh221
	.loh AdrpLdr	Lloh222, Lloh223
	.loh AdrpLdr	Lloh230, Lloh231
	.loh AdrpAdd	Lloh228, Lloh229
	.loh AdrpAdd	Lloh226, Lloh227
	.loh AdrpAdd	Lloh224, Lloh225

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
Ltmp46:
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
Ltmp47:
Ltmp49:
	mov	x20, x0
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
Ltmp50:
	mov	x21, x0
	adrp	x22, SYM(test_define_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGE
	cbz	x19, LBB22_4
Lloh232:
	adrp	x8, SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
Lloh233:
	ldr	x8, [x8, SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGEOFF]
	add	x8, x19, x8
	stp	x20, x21, [x8]
	ldr	x8, [x22, SYM(test_define_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGEOFF]
	mov	w9, #15
	strb	w9, [x19, x8]
	b	LBB22_6
LBB22_4:
Ltmp57:
	mov	x0, x20
	bl	_objc_release
Ltmp58:
	mov	x0, x21
	bl	_objc_release
LBB22_6:
Lloh234:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh235:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh236:
	ldr	x1, [x8]
Lloh237:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh238:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh239:
	ldr	x8, [x8]
	stp	x19, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	cbz	x0, LBB22_8
	ldr	x8, [x22, SYM(test_define_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGEOFF]
	mov	w9, #255
	strb	w9, [x0, x8]
LBB22_8:
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	ldp	x22, x21, [sp, #16]
	add	sp, sp, #64
	ret
LBB22_9:
Ltmp59:
	mov	x20, x0
Ltmp60:
	mov	x0, x21
	bl	_objc_release
Ltmp61:
	b	LBB22_14
LBB22_10:
Ltmp62:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB22_11:
Ltmp51:
	mov	x8, x20
	mov	x20, x0
Ltmp52:
	mov	x0, x8
	bl	_objc_release
Ltmp53:
	b	LBB22_13
LBB22_12:
Ltmp48:
	mov	x20, x0
LBB22_13:
Ltmp54:
	mov	x0, x19
	bl	_objc_release
Ltmp55:
LBB22_14:
	mov	x0, x20
	bl	__Unwind_Resume
LBB22_15:
Ltmp56:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.loh AdrpLdr	Lloh232, Lloh233
	.loh AdrpLdrGotLdr	Lloh237, Lloh238, Lloh239
	.loh AdrpLdrGotLdr	Lloh234, Lloh235, Lloh236
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
	.uleb128 Ltmp46-Lfunc_begin5
	.uleb128 Ltmp47-Ltmp46
	.uleb128 Ltmp48-Lfunc_begin5
	.byte	0
	.uleb128 Ltmp49-Lfunc_begin5
	.uleb128 Ltmp50-Ltmp49
	.uleb128 Ltmp51-Lfunc_begin5
	.byte	0
	.uleb128 Ltmp57-Lfunc_begin5
	.uleb128 Ltmp58-Ltmp57
	.uleb128 Ltmp59-Lfunc_begin5
	.byte	0
	.uleb128 Ltmp58-Lfunc_begin5
	.uleb128 Ltmp60-Ltmp58
	.byte	0
	.byte	0
	.uleb128 Ltmp60-Lfunc_begin5
	.uleb128 Ltmp61-Ltmp60
	.uleb128 Ltmp62-Lfunc_begin5
	.byte	1
	.uleb128 Ltmp52-Lfunc_begin5
	.uleb128 Ltmp55-Ltmp52
	.uleb128 Ltmp56-Lfunc_begin5
	.byte	1
	.uleb128 Ltmp55-Lfunc_begin5
	.uleb128 Lfunc_end5-Ltmp55
	.byte	0
	.byte	0
Lcst_end5:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase2:
	.byte	0
	.p2align	2, 0x0

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_define_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.p2align	3, 0x0
l_anon.[ID].1:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_define_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.p2align	3, 0x0
l_anon.[ID].2:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_define_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].3:
	.byte	17
	.space	39

l_anon.[ID].4:
	.asciz	"ivars"

l_anon.[ID].5:
	.asciz	"drop_flag"

	.p2align	3, 0x0
l_anon.[ID].6:
	.byte	5
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
	.ascii	"ForgetableIvars"

l_anon.[ID].10:
	.ascii	"DropIvars"

l_anon.[ID].11:
	.ascii	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].12:
	.quad	l_anon.[ID].11
	.asciz	"p\000\000\000\000\000\000\000\236\000\000\0002\000\000"

	.section	__TEXT,__const
l_anon.[ID].13:
	.ascii	"NoIvars"

	.globl	SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2)
.zerofill __DATA,__common,SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2),8,3
	.globl	SYM(test_define_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 1)
.zerofill __DATA,__common,SYM(test_define_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 1),8,3
l_anon.[ID].14:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].15:
	.quad	l_anon.[ID].14
	.asciz	"4\000\000\000\000\000\000\000\016\000\000\000\001\000\000"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].16:
	.byte	21
	.space	39

	.p2align	3, 0x0
l_anon.[ID].17:
	.byte	16
	.space	39

	.p2align	3, 0x0
l_anon.[ID].18:
	.byte	19
	.space	39

	.section	__TEXT,__literal8,8byte_literals
l_anon.[ID].19:
	.ascii	"NSObject"

	.section	__TEXT,__const
l_anon.[ID].20:
	.ascii	"NSCopying"

l_anon.[ID].21:
	.ascii	"_NSZone"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].22:
	.byte	28
	.space	7
	.quad	l_anon.[ID].21
	.asciz	"\007\000\000\000\000\000\000"
	.quad	8
	.space	8

	.p2align	3, 0x0
l_anon.[ID].23:
	.byte	25
	.space	7
	.quad	l_anon.[ID].22
	.space	24

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_da7dfba076f8819b
L_OBJC_METH_VAR_NAME_da7dfba076f8819b:
	.asciz	"classMethod"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_da7dfba076f8819b
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_da7dfba076f8819b:
	.quad	L_OBJC_METH_VAR_NAME_da7dfba076f8819b

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_da7dfba076f8819b
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_da7dfba076f8819b:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_af51813a2f91b268
L_OBJC_METH_VAR_NAME_af51813a2f91b268:
	.asciz	"method"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_af51813a2f91b268
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_af51813a2f91b268:
	.quad	L_OBJC_METH_VAR_NAME_af51813a2f91b268

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_af51813a2f91b268
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_af51813a2f91b268:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_43ff92728f706391
L_OBJC_METH_VAR_NAME_43ff92728f706391:
	.asciz	"methodBool:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_43ff92728f706391
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_43ff92728f706391:
	.quad	L_OBJC_METH_VAR_NAME_43ff92728f706391

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_43ff92728f706391
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_43ff92728f706391:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_defcf1f00a4b4302
L_OBJC_METH_VAR_NAME_defcf1f00a4b4302:
	.asciz	"methodRetained"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_defcf1f00a4b4302
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_defcf1f00a4b4302:
	.quad	L_OBJC_METH_VAR_NAME_defcf1f00a4b4302

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_defcf1f00a4b4302
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_defcf1f00a4b4302:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_9e0e96761ff41d49
L_OBJC_METH_VAR_NAME_9e0e96761ff41d49:
	.asciz	"methodRetainedWithParam:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_9e0e96761ff41d49
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_9e0e96761ff41d49:
	.quad	L_OBJC_METH_VAR_NAME_9e0e96761ff41d49

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_9e0e96761ff41d49
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_9e0e96761ff41d49:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_597b404e7f57234e
L_OBJC_METH_VAR_NAME_597b404e7f57234e:
	.asciz	"copyWithZone:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_597b404e7f57234e
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_597b404e7f57234e:
	.quad	L_OBJC_METH_VAR_NAME_597b404e7f57234e

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_597b404e7f57234e
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_597b404e7f57234e:
	.asciz	"\000\000\000\000@\000\000"

	.globl	SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)
.zerofill __DATA,__common,SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1),8,3
	.globl	SYM(test_define_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 2)
.zerofill __DATA,__common,SYM(test_define_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 2),8,3
	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].24:
	.quad	l_anon.[ID].14
	.asciz	"4\000\000\000\000\000\000\000M\000\000\000\001\000\000"

	.globl	SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)
.zerofill __DATA,__common,SYM(test_define_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0),8,3
	.globl	SYM(test_define_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)
.zerofill __DATA,__common,SYM(test_define_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0),8,3
	.p2align	3, 0x0
l_anon.[ID].25:
	.quad	l_anon.[ID].14
	.asciz	"4\000\000\000\000\000\000\000o\000\000\000\001\000\000"

.zerofill __DATA,__bss,__MergedGlobals,48,3
.subsections_via_symbols
