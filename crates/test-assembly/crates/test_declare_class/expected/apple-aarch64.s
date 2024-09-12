	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(objc2[CRATE_ID]::__macro_helpers::declared_ivars::dealloc::<test_declare_class[CRATE_ID]::DropIvars>, 0):
Lfunc_begin0:
	sub	sp, sp, #64
	stp	x22, x21, [sp, #16]
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	mov	x19, x1
	mov	x20, x0
Lloh0:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGE
Lloh1:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGEOFF]
	ldrb	w8, [x0, x8]
	cbz	w8, LBB0_6
	cmp	w8, #255
	b.ne	LBB0_3
	bl	SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0)
LBB0_3:
Lloh2:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
Lloh3:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGEOFF]
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
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin1:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	tbz	w9, #0, LBB1_15
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
	mov	w1, #7
	bl	SYM(objc2::runtime::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB1_16
	str	x0, [sp, #8]
Lloh12:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_518803e84ea38a73@PAGE
Lloh13:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_518803e84ea38a73@PAGEOFF]
Ltmp6:
Lloh14:
	adrp	x4, l_anon.[ID].16@PAGE
Lloh15:
	add	x4, x4, l_anon.[ID].16@PAGEOFF
Lloh16:
	adrp	x5, _get_class@PAGE
Lloh17:
	add	x5, x5, _get_class@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_class_method_inner::GENERATED_ID, 0)
Ltmp7:
Lloh18:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_05fa1b2ffc15d267@PAGE
Lloh19:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_05fa1b2ffc15d267@PAGEOFF]
Ltmp8:
Lloh20:
	adrp	x4, l_anon.[ID].3@PAGE
Lloh21:
	add	x4, x4, l_anon.[ID].3@PAGEOFF
Lloh22:
	adrp	x5, _method_simple@PAGE
Lloh23:
	add	x5, x5, _method_simple@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp9:
Lloh24:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_58736195c9ca7c7f@PAGE
Lloh25:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_58736195c9ca7c7f@PAGEOFF]
Ltmp10:
Lloh26:
	adrp	x2, l_anon.[ID].17@PAGE
Lloh27:
	add	x2, x2, l_anon.[ID].17@PAGEOFF
Lloh28:
	adrp	x5, _method_bool@PAGE
Lloh29:
	add	x5, x5, _method_bool@PAGEOFF
	add	x0, sp, #8
	mov	w3, #1
	mov	x4, x2
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp11:
Lloh30:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_61b74dbf9c375668@PAGE
Lloh31:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_61b74dbf9c375668@PAGEOFF]
Ltmp12:
Lloh32:
	adrp	x4, l_anon.[ID].18@PAGE
Lloh33:
	add	x4, x4, l_anon.[ID].18@PAGEOFF
Lloh34:
	adrp	x5, _method_id@PAGE
Lloh35:
	add	x5, x5, _method_id@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp13:
Lloh36:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_96586542870e42e5@PAGE
Lloh37:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_96586542870e42e5@PAGEOFF]
Ltmp14:
Lloh38:
	adrp	x2, l_anon.[ID].17@PAGE
Lloh39:
	add	x2, x2, l_anon.[ID].17@PAGEOFF
Lloh40:
	adrp	x4, l_anon.[ID].18@PAGE
Lloh41:
	add	x4, x4, l_anon.[ID].18@PAGEOFF
Lloh42:
	adrp	x5, _method_id_with_param@PAGE
Lloh43:
	add	x5, x5, _method_id_with_param@PAGEOFF
	add	x0, sp, #8
	mov	w3, #1
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp15:
Ltmp16:
Lloh44:
	adrp	x0, l_anon.[ID].19@PAGE
Lloh45:
	add	x0, x0, l_anon.[ID].19@PAGEOFF
	mov	w1, #8
	bl	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	mov	x1, x0
Ltmp17:
	cbz	x1, LBB1_10
	ldr	x0, [sp, #8]
	bl	_class_addProtocol
LBB1_10:
Ltmp18:
Lloh46:
	adrp	x0, l_anon.[ID].20@PAGE
Lloh47:
	add	x0, x0, l_anon.[ID].20@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	mov	x1, x0
Ltmp19:
	cbz	x1, LBB1_13
	ldr	x0, [sp, #8]
	bl	_class_addProtocol
LBB1_13:
Lloh48:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_f4e71677dafa88a8@PAGE
Lloh49:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_f4e71677dafa88a8@PAGEOFF]
Ltmp20:
Lloh50:
	adrp	x2, l_anon.[ID].23@PAGE
Lloh51:
	add	x2, x2, l_anon.[ID].23@PAGEOFF
Lloh52:
	adrp	x4, l_anon.[ID].18@PAGE
Lloh53:
	add	x4, x4, l_anon.[ID].18@PAGEOFF
Lloh54:
	adrp	x5, _copyWithZone@PAGE
Lloh55:
	add	x5, x5, _copyWithZone@PAGEOFF
	add	x0, sp, #8
	mov	w3, #1
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
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
LBB1_15:
Lloh56:
	adrp	x0, l_anon.[ID].12@PAGE
Lloh57:
	add	x0, x0, l_anon.[ID].12@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB1_16:
Lloh58:
	adrp	x0, l_anon.[ID].13@PAGE
Lloh59:
	add	x0, x0, l_anon.[ID].13@PAGEOFF
Lloh60:
	adrp	x2, l_anon.[ID].15@PAGE
Lloh61:
	add	x2, x2, l_anon.[ID].15@PAGEOFF
	mov	w1, #7
	bl	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
LBB1_17:
Ltmp22:
	mov	x19, x0
Ltmp23:
	add	x0, sp, #8
	bl	SYM(<objc2::runtime::declare::ClassBuilder as core::ops::drop::Drop>::drop::GENERATED_ID, 0)
Ltmp24:
	mov	x0, x19
	bl	__Unwind_Resume
LBB1_19:
Ltmp25:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
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
	.byte	155
	.uleb128 Lttbase1-Lttbaseref1
Lttbaseref1:
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
	.uleb128 Ltmp23-Ltmp21
	.byte	0
	.byte	0
	.uleb128 Ltmp23-Lfunc_begin1
	.uleb128 Ltmp24-Ltmp23
	.uleb128 Ltmp25-Lfunc_begin1
	.byte	1
	.uleb128 Ltmp24-Lfunc_begin1
	.uleb128 Lfunc_end1-Ltmp24
	.byte	0
	.byte	0
Lcst_end1:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase1:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin2:
	sub	sp, sp, #96
	stp	x20, x19, [sp, #64]
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	tbz	w9, #0, LBB2_9
Lloh62:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh63:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh64:
	ldr	x2, [x8]
Lloh65:
	adrp	x0, l_anon.[ID].10@PAGE
Lloh66:
	add	x0, x0, l_anon.[ID].10@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::runtime::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB2_10
	str	x0, [sp, #24]
Lloh67:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGE
Lloh68:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGEOFF]
Lloh69:
	ldr	x1, [x8]
Ltmp26:
Lloh70:
	adrp	x4, l_anon.[ID].3@PAGE
Lloh71:
	add	x4, x4, l_anon.[ID].3@PAGEOFF
Lloh72:
	adrp	x5, SYM(objc2[CRATE_ID]::__macro_helpers::declared_ivars::dealloc::<test_declare_class[CRATE_ID]::DropIvars>, 0)@PAGE
Lloh73:
	add	x5, x5, SYM(objc2[CRATE_ID]::__macro_helpers::declared_ivars::dealloc::<test_declare_class[CRATE_ID]::DropIvars>, 0)@PAGEOFF
	add	x0, sp, #24
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp27:
	ldr	x8, [sp, #24]
	str	x8, [sp, #8]
Lloh74:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh75:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh76:
	ldr	x1, [x8]
Ltmp32:
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
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp33:
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
Ltmp38:
Lloh83:
	adrp	x1, l_anon.[ID].4@PAGE
Lloh84:
	add	x1, x1, l_anon.[ID].4@PAGEOFF
	add	x0, sp, #16
	add	x5, sp, #24
	mov	w2, #5
	mov	w3, #16
	mov	w4, #3
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Ltmp39:
Ltmp40:
Lloh85:
	adrp	x1, l_anon.[ID].5@PAGE
Lloh86:
	add	x1, x1, l_anon.[ID].5@PAGEOFF
Lloh87:
	adrp	x5, l_anon.[ID].6@PAGE
Lloh88:
	add	x5, x5, l_anon.[ID].6@PAGEOFF
	add	x0, sp, #16
	mov	w2, #9
	mov	w3, #1
	mov	w4, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Ltmp41:
	ldr	x19, [sp, #16]
	mov	x0, x19
	bl	_objc_registerClassPair
Lloh89:
	adrp	x1, l_anon.[ID].4@PAGE
Lloh90:
	add	x1, x1, l_anon.[ID].4@PAGEOFF
	mov	x0, x19
	mov	w2, #5
	bl	SYM(objc2::runtime::AnyClass::instance_variable::GENERATED_ID, 0)
	cbz	x0, LBB2_11
	bl	_ivar_getOffset
	mov	x20, x0
Lloh91:
	adrp	x1, l_anon.[ID].5@PAGE
Lloh92:
	add	x1, x1, l_anon.[ID].5@PAGEOFF
	mov	x0, x19
	mov	w2, #9
	bl	SYM(objc2::runtime::AnyClass::instance_variable::GENERATED_ID, 0)
	cbz	x0, LBB2_12
	bl	_ivar_getOffset
Lloh93:
	adrp	x8, __MergedGlobals@PAGE+32
	str	x19, [x8, __MergedGlobals@PAGEOFF+32]
Lloh94:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
	str	x20, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGEOFF]
Lloh95:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGE
	str	x0, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGEOFF]
	ldp	x29, x30, [sp, #80]
	ldp	x20, x19, [sp, #64]
	add	sp, sp, #96
	ret
LBB2_9:
Lloh96:
	adrp	x0, l_anon.[ID].12@PAGE
Lloh97:
	add	x0, x0, l_anon.[ID].12@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB2_10:
Lloh98:
	adrp	x0, l_anon.[ID].10@PAGE
Lloh99:
	add	x0, x0, l_anon.[ID].10@PAGEOFF
Lloh100:
	adrp	x2, l_anon.[ID].25@PAGE
Lloh101:
	add	x2, x2, l_anon.[ID].25@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
LBB2_11:
	bl	SYM(objc2::__macro_helpers::declared_ivars::register_with_ivars::get_ivar_failed::GENERATED_ID, 0)
LBB2_12:
	bl	SYM(objc2::__macro_helpers::declared_ivars::register_with_ivars::get_drop_flag_failed::GENERATED_ID, 0)
LBB2_13:
Ltmp34:
	mov	x19, x0
Ltmp35:
	add	x0, sp, #8
	bl	SYM(<objc2::runtime::declare::ClassBuilder as core::ops::drop::Drop>::drop::GENERATED_ID, 0)
Ltmp36:
	b	LBB2_18
LBB2_14:
Ltmp37:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB2_15:
Ltmp28:
	mov	x19, x0
Ltmp29:
	add	x0, sp, #24
	bl	SYM(<objc2::runtime::declare::ClassBuilder as core::ops::drop::Drop>::drop::GENERATED_ID, 0)
Ltmp30:
	b	LBB2_18
LBB2_16:
Ltmp31:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB2_17:
Ltmp42:
	mov	x19, x0
Ltmp43:
	add	x0, sp, #16
	bl	SYM(<objc2::runtime::declare::ClassBuilder as core::ops::drop::Drop>::drop::GENERATED_ID, 0)
Ltmp44:
LBB2_18:
	mov	x0, x19
	bl	__Unwind_Resume
LBB2_19:
Ltmp45:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
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
	.byte	155
	.uleb128 Lttbase2-Lttbaseref2
Lttbaseref2:
	.byte	1
	.uleb128 Lcst_end2-Lcst_begin2
Lcst_begin2:
	.uleb128 Lfunc_begin2-Lfunc_begin2
	.uleb128 Ltmp26-Lfunc_begin2
	.byte	0
	.byte	0
	.uleb128 Ltmp26-Lfunc_begin2
	.uleb128 Ltmp27-Ltmp26
	.uleb128 Ltmp28-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp32-Lfunc_begin2
	.uleb128 Ltmp33-Ltmp32
	.uleb128 Ltmp34-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp38-Lfunc_begin2
	.uleb128 Ltmp41-Ltmp38
	.uleb128 Ltmp42-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp41-Lfunc_begin2
	.uleb128 Ltmp35-Ltmp41
	.byte	0
	.byte	0
	.uleb128 Ltmp35-Lfunc_begin2
	.uleb128 Ltmp36-Ltmp35
	.uleb128 Ltmp37-Lfunc_begin2
	.byte	1
	.uleb128 Ltmp29-Lfunc_begin2
	.uleb128 Ltmp30-Ltmp29
	.uleb128 Ltmp31-Lfunc_begin2
	.byte	1
	.uleb128 Ltmp43-Lfunc_begin2
	.uleb128 Ltmp44-Ltmp43
	.uleb128 Ltmp45-Lfunc_begin2
	.byte	1
	.uleb128 Ltmp44-Lfunc_begin2
	.uleb128 Lfunc_end2-Ltmp44
	.byte	0
	.byte	0
Lcst_end2:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase2:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin3:
	sub	sp, sp, #96
	stp	x20, x19, [sp, #64]
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	tbz	w9, #0, LBB3_6
Lloh102:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh103:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh104:
	ldr	x2, [x8]
Lloh105:
	adrp	x0, l_anon.[ID].9@PAGE
Lloh106:
	add	x0, x0, l_anon.[ID].9@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::runtime::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB3_7
	str	x0, [sp, #8]
Lloh107:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh108:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh109:
	ldr	x1, [x8]
Ltmp46:
Lloh110:
	adrp	x4, l_anon.[ID].18@PAGE
Lloh111:
	add	x4, x4, l_anon.[ID].18@PAGEOFF
Lloh112:
	adrp	x5, _init_forgetable_ivars@PAGE
Lloh113:
	add	x5, x5, _init_forgetable_ivars@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp47:
	ldr	x8, [sp, #8]
	str	x8, [sp, #16]
	mov	w8, #8
Lloh114:
	adrp	x9, l_anon.[ID].7@PAGE
Lloh115:
	add	x9, x9, l_anon.[ID].7@PAGEOFF
	stp	x8, x9, [sp, #32]
	mov	w8, #27
	strb	w8, [sp, #24]
Ltmp52:
Lloh116:
	adrp	x1, l_anon.[ID].4@PAGE
Lloh117:
	add	x1, x1, l_anon.[ID].4@PAGEOFF
	add	x0, sp, #16
	add	x5, sp, #24
	mov	w2, #5
	mov	w3, #8
	mov	w4, #2
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Ltmp53:
	ldr	x19, [sp, #16]
	mov	x0, x19
	bl	_objc_registerClassPair
Lloh118:
	adrp	x1, l_anon.[ID].4@PAGE
Lloh119:
	add	x1, x1, l_anon.[ID].4@PAGEOFF
	mov	x0, x19
	mov	w2, #5
	bl	SYM(objc2::runtime::AnyClass::instance_variable::GENERATED_ID, 0)
	cbz	x0, LBB3_8
	bl	_ivar_getOffset
Lloh120:
	adrp	x8, __MergedGlobals@PAGE+16
	str	x19, [x8, __MergedGlobals@PAGEOFF+16]
Lloh121:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGE
	str	x0, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGEOFF]
	ldp	x29, x30, [sp, #80]
	ldp	x20, x19, [sp, #64]
	add	sp, sp, #96
	ret
LBB3_6:
Lloh122:
	adrp	x0, l_anon.[ID].12@PAGE
Lloh123:
	add	x0, x0, l_anon.[ID].12@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB3_7:
Lloh124:
	adrp	x0, l_anon.[ID].9@PAGE
Lloh125:
	add	x0, x0, l_anon.[ID].9@PAGEOFF
Lloh126:
	adrp	x2, l_anon.[ID].24@PAGE
Lloh127:
	add	x2, x2, l_anon.[ID].24@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
LBB3_8:
	bl	SYM(objc2::__macro_helpers::declared_ivars::register_with_ivars::get_ivar_failed::GENERATED_ID, 0)
LBB3_9:
Ltmp54:
	mov	x19, x0
Ltmp55:
	add	x0, sp, #16
	bl	SYM(<objc2::runtime::declare::ClassBuilder as core::ops::drop::Drop>::drop::GENERATED_ID, 0)
Ltmp56:
	b	LBB3_12
LBB3_10:
Ltmp57:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB3_11:
Ltmp48:
	mov	x19, x0
Ltmp49:
	add	x0, sp, #8
	bl	SYM(<objc2::runtime::declare::ClassBuilder as core::ops::drop::Drop>::drop::GENERATED_ID, 0)
Ltmp50:
LBB3_12:
	mov	x0, x19
	bl	__Unwind_Resume
LBB3_13:
Ltmp51:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
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
	.byte	155
	.uleb128 Lttbase3-Lttbaseref3
Lttbaseref3:
	.byte	1
	.uleb128 Lcst_end3-Lcst_begin3
Lcst_begin3:
	.uleb128 Lfunc_begin3-Lfunc_begin3
	.uleb128 Ltmp46-Lfunc_begin3
	.byte	0
	.byte	0
	.uleb128 Ltmp46-Lfunc_begin3
	.uleb128 Ltmp47-Ltmp46
	.uleb128 Ltmp48-Lfunc_begin3
	.byte	0
	.uleb128 Ltmp52-Lfunc_begin3
	.uleb128 Ltmp53-Ltmp52
	.uleb128 Ltmp54-Lfunc_begin3
	.byte	0
	.uleb128 Ltmp53-Lfunc_begin3
	.uleb128 Ltmp55-Ltmp53
	.byte	0
	.byte	0
	.uleb128 Ltmp55-Lfunc_begin3
	.uleb128 Ltmp56-Ltmp55
	.uleb128 Ltmp57-Lfunc_begin3
	.byte	1
	.uleb128 Ltmp49-Lfunc_begin3
	.uleb128 Ltmp50-Ltmp49
	.uleb128 Ltmp51-Lfunc_begin3
	.byte	1
	.uleb128 Ltmp50-Lfunc_begin3
	.uleb128 Lfunc_end3-Ltmp50
	.byte	0
	.byte	0
Lcst_end3:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase3:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	str	x8, [sp, #8]
	add	x0, sp, #8
	bl	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.p2align	2
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	str	x8, [sp, #8]
	add	x0, sp, #8
	bl	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.p2align	2
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	str	x8, [sp, #8]
	add	x0, sp, #8
	bl	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.globl	_access_forgetable_ivars_class
	.p2align	2
_access_forgetable_ivars_class:
Lloh128:
	adrp	x8, __MergedGlobals@PAGE+24
Lloh129:
	add	x8, x8, __MergedGlobals@PAGEOFF+24
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB7_2
Lloh130:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh131:
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
Lloh132:
	adrp	x0, __MergedGlobals@PAGE+24
Lloh133:
	add	x0, x0, __MergedGlobals@PAGEOFF+24
Lloh134:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh135:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
Lloh136:
	adrp	x4, l_anon.[ID].24@PAGE
Lloh137:
	add	x4, x4, l_anon.[ID].24@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh138:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh139:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
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
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGE
Lloh141:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGEOFF]
	add	x8, x0, x8
	ldr	w1, [x8]
	ldrb	w0, [x8, #4]
	ret
	.loh AdrpLdr	Lloh140, Lloh141

	.globl	SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0):
	; InlineAsm Start
	; InlineAsm End
	ret

	.globl	_access_drop_ivars_class
	.p2align	2
_access_drop_ivars_class:
Lloh142:
	adrp	x8, __MergedGlobals@PAGE+40
Lloh143:
	add	x8, x8, __MergedGlobals@PAGEOFF+40
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB10_2
Lloh144:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh145:
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
Lloh146:
	adrp	x0, __MergedGlobals@PAGE+40
Lloh147:
	add	x0, x0, __MergedGlobals@PAGEOFF+40
Lloh148:
	adrp	x3, l_anon.[ID].1@PAGE
Lloh149:
	add	x3, x3, l_anon.[ID].1@PAGEOFF
Lloh150:
	adrp	x4, l_anon.[ID].25@PAGE
Lloh151:
	add	x4, x4, l_anon.[ID].25@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh152:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh153:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+32]
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
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
Lloh155:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGEOFF]
	add	x8, x0, x8
	ldp	x0, x1, [x8]
	ret
	.loh AdrpLdr	Lloh154, Lloh155

	.globl	SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh156:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh157:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB12_2
Lloh158:
	adrp	x8, __MergedGlobals@PAGE
Lloh159:
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
Lloh160:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh161:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh162:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh163:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh164:
	adrp	x4, l_anon.[ID].15@PAGE
Lloh165:
	add	x4, x4, l_anon.[ID].15@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh166:
	adrp	x8, __MergedGlobals@PAGE
Lloh167:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
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
	adrp	x8, __MergedGlobals@PAGE+8
Lloh169:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB13_2
Lloh170:
	adrp	x8, __MergedGlobals@PAGE
Lloh171:
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
Lloh172:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh173:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh174:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh175:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh176:
	adrp	x4, l_anon.[ID].15@PAGE
Lloh177:
	add	x4, x4, l_anon.[ID].15@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh178:
	adrp	x8, __MergedGlobals@PAGE
Lloh179:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
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

	.globl	_method_id
	.p2align	2
_method_id:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh180:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh181:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB16_2
LBB16_1:
Lloh182:
	adrp	x8, __MergedGlobals@PAGE
Lloh183:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
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
	adrp	x0, __MergedGlobals@PAGE+8
Lloh188:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh189:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh190:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh191:
	adrp	x4, l_anon.[ID].15@PAGE
Lloh192:
	add	x4, x4, l_anon.[ID].15@PAGEOFF
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

	.globl	_method_id_with_param
	.p2align	2
_method_id_with_param:
Lfunc_begin4:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x20, x2
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	x19, x0
	cbz	w20, LBB17_3
Ltmp58:
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
Ltmp59:
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
Ltmp60:
	mov	x20, x0
Ltmp61:
	mov	x0, x19
	bl	_objc_release
Ltmp62:
	mov	x0, x20
	bl	__Unwind_Resume
LBB17_6:
Ltmp63:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end4:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table17:
Lexception4:
	.byte	255
	.byte	155
	.uleb128 Lttbase4-Lttbaseref4
Lttbaseref4:
	.byte	1
	.uleb128 Lcst_end4-Lcst_begin4
Lcst_begin4:
	.uleb128 Lfunc_begin4-Lfunc_begin4
	.uleb128 Ltmp58-Lfunc_begin4
	.byte	0
	.byte	0
	.uleb128 Ltmp58-Lfunc_begin4
	.uleb128 Ltmp59-Ltmp58
	.uleb128 Ltmp60-Lfunc_begin4
	.byte	0
	.uleb128 Ltmp59-Lfunc_begin4
	.uleb128 Ltmp61-Ltmp59
	.byte	0
	.byte	0
	.uleb128 Ltmp61-Lfunc_begin4
	.uleb128 Ltmp62-Ltmp61
	.uleb128 Ltmp63-Lfunc_begin4
	.byte	1
	.uleb128 Ltmp62-Lfunc_begin4
	.uleb128 Lfunc_end4-Ltmp62
	.byte	0
	.byte	0
Lcst_end4:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase4:
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
	adrp	x8, __MergedGlobals@PAGE+8
Lloh194:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB18_2
LBB18_1:
Lloh195:
	adrp	x8, __MergedGlobals@PAGE
Lloh196:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
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
	adrp	x0, __MergedGlobals@PAGE+8
Lloh201:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh202:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh203:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh204:
	adrp	x4, l_anon.[ID].15@PAGE
Lloh205:
	add	x4, x4, l_anon.[ID].15@PAGEOFF
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

	.globl	SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh206:
	adrp	x8, __MergedGlobals@PAGE+24
Lloh207:
	add	x8, x8, __MergedGlobals@PAGEOFF+24
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB19_2
Lloh208:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh209:
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
Lloh210:
	adrp	x0, __MergedGlobals@PAGE+24
Lloh211:
	add	x0, x0, __MergedGlobals@PAGEOFF+24
Lloh212:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh213:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
Lloh214:
	adrp	x4, l_anon.[ID].24@PAGE
Lloh215:
	add	x4, x4, l_anon.[ID].24@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh216:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh217:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
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
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGE
Lloh219:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGEOFF]
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
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_7f51a873b0d59f00@PAGE
Lloh221:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_7f51a873b0d59f00@PAGEOFF]
Lloh222:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh223:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh224:
	ldr	x8, [x8]
	stp	x0, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpLdr	Lloh218, Lloh219
	.loh AdrpLdrGotLdr	Lloh222, Lloh223, Lloh224
	.loh AdrpAdrp	Lloh220, Lloh222
	.loh AdrpLdr	Lloh220, Lloh221

	.globl	SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh225:
	adrp	x8, __MergedGlobals@PAGE+40
Lloh226:
	add	x8, x8, __MergedGlobals@PAGEOFF+40
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB21_2
Lloh227:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh228:
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
Lloh229:
	adrp	x0, __MergedGlobals@PAGE+40
Lloh230:
	add	x0, x0, __MergedGlobals@PAGEOFF+40
Lloh231:
	adrp	x3, l_anon.[ID].1@PAGE
Lloh232:
	add	x3, x3, l_anon.[ID].1@PAGEOFF
Lloh233:
	adrp	x4, l_anon.[ID].25@PAGE
Lloh234:
	add	x4, x4, l_anon.[ID].25@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh235:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh236:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+32]
	ret
	.loh AdrpAdd	Lloh225, Lloh226
	.loh AdrpLdr	Lloh227, Lloh228
	.loh AdrpLdr	Lloh235, Lloh236
	.loh AdrpAdd	Lloh233, Lloh234
	.loh AdrpAdd	Lloh231, Lloh232
	.loh AdrpAdd	Lloh229, Lloh230

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
Ltmp64:
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
Ltmp65:
Ltmp67:
	mov	x20, x0
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
Ltmp68:
	mov	x21, x0
	adrp	x22, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGE
	cbz	x19, LBB22_4
Lloh237:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
Lloh238:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGEOFF]
	add	x8, x19, x8
	stp	x20, x21, [x8]
	ldr	x8, [x22, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGEOFF]
	mov	w9, #15
	strb	w9, [x19, x8]
	b	LBB22_6
LBB22_4:
Ltmp75:
	mov	x0, x20
	bl	_objc_release
Ltmp76:
	mov	x0, x21
	bl	_objc_release
LBB22_6:
Lloh239:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_802cb9c5fa0b19dd@PAGE
Lloh240:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_802cb9c5fa0b19dd@PAGEOFF]
Lloh241:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh242:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh243:
	ldr	x8, [x8]
	stp	x19, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	cbz	x0, LBB22_8
	ldr	x8, [x22, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGEOFF]
	mov	w9, #255
	strb	w9, [x0, x8]
LBB22_8:
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	ldp	x22, x21, [sp, #16]
	add	sp, sp, #64
	ret
LBB22_9:
Ltmp77:
	mov	x20, x0
Ltmp78:
	mov	x0, x21
	bl	_objc_release
Ltmp79:
	b	LBB22_14
LBB22_10:
Ltmp80:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB22_11:
Ltmp69:
	mov	x8, x20
	mov	x20, x0
Ltmp70:
	mov	x0, x8
	bl	_objc_release
Ltmp71:
	b	LBB22_13
LBB22_12:
Ltmp66:
	mov	x20, x0
LBB22_13:
Ltmp72:
	mov	x0, x19
	bl	_objc_release
Ltmp73:
LBB22_14:
	mov	x0, x20
	bl	__Unwind_Resume
LBB22_15:
Ltmp74:
	bl	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
	.loh AdrpLdr	Lloh237, Lloh238
	.loh AdrpLdrGotLdr	Lloh241, Lloh242, Lloh243
	.loh AdrpAdrp	Lloh239, Lloh241
	.loh AdrpLdr	Lloh239, Lloh240
Lfunc_end5:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table22:
Lexception5:
	.byte	255
	.byte	155
	.uleb128 Lttbase5-Lttbaseref5
Lttbaseref5:
	.byte	1
	.uleb128 Lcst_end5-Lcst_begin5
Lcst_begin5:
	.uleb128 Ltmp64-Lfunc_begin5
	.uleb128 Ltmp65-Ltmp64
	.uleb128 Ltmp66-Lfunc_begin5
	.byte	0
	.uleb128 Ltmp67-Lfunc_begin5
	.uleb128 Ltmp68-Ltmp67
	.uleb128 Ltmp69-Lfunc_begin5
	.byte	0
	.uleb128 Ltmp75-Lfunc_begin5
	.uleb128 Ltmp76-Ltmp75
	.uleb128 Ltmp77-Lfunc_begin5
	.byte	0
	.uleb128 Ltmp76-Lfunc_begin5
	.uleb128 Ltmp78-Ltmp76
	.byte	0
	.byte	0
	.uleb128 Ltmp78-Lfunc_begin5
	.uleb128 Ltmp79-Ltmp78
	.uleb128 Ltmp80-Lfunc_begin5
	.byte	1
	.uleb128 Ltmp70-Lfunc_begin5
	.uleb128 Ltmp73-Ltmp70
	.uleb128 Ltmp74-Lfunc_begin5
	.byte	1
	.uleb128 Ltmp73-Lfunc_begin5
	.uleb128 Lfunc_end5-Ltmp73
	.byte	0
	.byte	0
Lcst_end5:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase5:
	.byte	0
	.p2align	2, 0x0

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.p2align	3, 0x0
l_anon.[ID].1:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.p2align	3, 0x0
l_anon.[ID].2:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].3:
	.byte	17
	.space	39

l_anon.[ID].4:
	.ascii	"ivars"

l_anon.[ID].5:
	.ascii	"drop_flag"

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

	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2),8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 1)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 1),8,3
l_anon.[ID].14:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].15:
	.quad	l_anon.[ID].14
	.asciz	"5\000\000\000\000\000\000\000\016\000\000\000\001\000\000"

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
	.globl	L_OBJC_METH_VAR_NAME_518803e84ea38a73
L_OBJC_METH_VAR_NAME_518803e84ea38a73:
	.asciz	"classMethod"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_518803e84ea38a73
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_518803e84ea38a73:
	.quad	L_OBJC_METH_VAR_NAME_518803e84ea38a73

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_518803e84ea38a73
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_518803e84ea38a73:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_05fa1b2ffc15d267
L_OBJC_METH_VAR_NAME_05fa1b2ffc15d267:
	.asciz	"method"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_05fa1b2ffc15d267
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_05fa1b2ffc15d267:
	.quad	L_OBJC_METH_VAR_NAME_05fa1b2ffc15d267

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_05fa1b2ffc15d267
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_05fa1b2ffc15d267:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_58736195c9ca7c7f
L_OBJC_METH_VAR_NAME_58736195c9ca7c7f:
	.asciz	"methodBool:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_58736195c9ca7c7f
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_58736195c9ca7c7f:
	.quad	L_OBJC_METH_VAR_NAME_58736195c9ca7c7f

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_58736195c9ca7c7f
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_58736195c9ca7c7f:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_61b74dbf9c375668
L_OBJC_METH_VAR_NAME_61b74dbf9c375668:
	.asciz	"methodId"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_61b74dbf9c375668
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_61b74dbf9c375668:
	.quad	L_OBJC_METH_VAR_NAME_61b74dbf9c375668

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_61b74dbf9c375668
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_61b74dbf9c375668:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_96586542870e42e5
L_OBJC_METH_VAR_NAME_96586542870e42e5:
	.asciz	"methodIdWithParam:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_96586542870e42e5
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_96586542870e42e5:
	.quad	L_OBJC_METH_VAR_NAME_96586542870e42e5

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_96586542870e42e5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_96586542870e42e5:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_f4e71677dafa88a8
L_OBJC_METH_VAR_NAME_f4e71677dafa88a8:
	.asciz	"copyWithZone:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_f4e71677dafa88a8
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_f4e71677dafa88a8:
	.quad	L_OBJC_METH_VAR_NAME_f4e71677dafa88a8

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f4e71677dafa88a8
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f4e71677dafa88a8:
	.asciz	"\000\000\000\000@\000\000"

	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1),8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 2)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 2),8,3
	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].24:
	.quad	l_anon.[ID].14
	.asciz	"5\000\000\000\000\000\000\000R\000\000\000\001\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_7f51a873b0d59f00
L_OBJC_METH_VAR_NAME_7f51a873b0d59f00:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_7f51a873b0d59f00
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_7f51a873b0d59f00:
	.quad	L_OBJC_METH_VAR_NAME_7f51a873b0d59f00

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_7f51a873b0d59f00
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_7f51a873b0d59f00:
	.asciz	"\000\000\000\000@\000\000"

	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0),8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0),8,3
	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].25:
	.quad	l_anon.[ID].14
	.asciz	"5\000\000\000\000\000\000\000z\000\000\000\001\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_802cb9c5fa0b19dd
L_OBJC_METH_VAR_NAME_802cb9c5fa0b19dd:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_802cb9c5fa0b19dd
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_802cb9c5fa0b19dd:
	.quad	L_OBJC_METH_VAR_NAME_802cb9c5fa0b19dd

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_802cb9c5fa0b19dd
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_802cb9c5fa0b19dd:
	.asciz	"\000\000\000\000@\000\000"

.zerofill __DATA,__bss,__MergedGlobals,48,3
.subsections_via_symbols
