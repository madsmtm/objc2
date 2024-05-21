	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}>, 0):
	ret

	.p2align	2
SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}>, 0):
	ret

	.p2align	2
SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}>, 0):
	ret

	.p2align	2
SYM(objc2[CRATE_ID]::__macro_helpers::declared_ivars::dealloc::<test_declare_class[CRATE_ID]::DropIvars>, 0):
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
	cbz	w8, LBB3_5
	cmp	w8, #255
	b.ne	LBB3_3
	bl	SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0)
LBB3_3:
Lloh2:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
Lloh3:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGEOFF]
	add	x8, x20, x8
	ldp	x0, x21, [x8]
	bl	_objc_release
	cbz	x21, LBB3_5
	mov	x0, x21
	bl	_objc_release
LBB3_5:
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
	.loh AdrpLdr	Lloh0, Lloh1
	.loh AdrpLdr	Lloh2, Lloh3
	.loh AdrpLdrGotLdr	Lloh4, Lloh5, Lloh6

	.p2align	2
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	cbz	w9, LBB4_7
Lloh7:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh8:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh9:
	ldr	x2, [x8]
Lloh10:
	adrp	x0, l_anon.[ID].19@PAGE
Lloh11:
	add	x0, x0, l_anon.[ID].19@PAGEOFF
	mov	w1, #7
	bl	SYM(objc2::runtime::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB4_8
	str	x0, [sp, #8]
Lloh12:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_c1ccd9f2c8e68869@PAGE
Lloh13:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_c1ccd9f2c8e68869@PAGEOFF]
Lloh14:
	adrp	x4, l_anon.[ID].9@PAGE
Lloh15:
	add	x4, x4, l_anon.[ID].9@PAGEOFF
Lloh16:
	adrp	x5, _get_class@PAGE
Lloh17:
	add	x5, x5, _get_class@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_class_method_inner::GENERATED_ID, 0)
Lloh18:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_654faaf1a88864b3@PAGE
Lloh19:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_654faaf1a88864b3@PAGEOFF]
Lloh20:
	adrp	x4, l_anon.[ID].4@PAGE
Lloh21:
	add	x4, x4, l_anon.[ID].4@PAGEOFF
Lloh22:
	adrp	x5, _method_simple@PAGE
Lloh23:
	add	x5, x5, _method_simple@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh24:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_5d27bc76c3596041@PAGE
Lloh25:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_5d27bc76c3596041@PAGEOFF]
Lloh26:
	adrp	x19, l_anon.[ID].5@PAGE
Lloh27:
	add	x19, x19, l_anon.[ID].5@PAGEOFF
Lloh28:
	adrp	x5, _method_bool@PAGE
Lloh29:
	add	x5, x5, _method_bool@PAGEOFF
	add	x0, sp, #8
	mov	x2, x19
	mov	w3, #1
	mov	x4, x19
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh30:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_026f8b3b5bb3f00d@PAGE
Lloh31:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_026f8b3b5bb3f00d@PAGEOFF]
Lloh32:
	adrp	x20, l_anon.[ID].3@PAGE
Lloh33:
	add	x20, x20, l_anon.[ID].3@PAGEOFF
Lloh34:
	adrp	x5, _method_id@PAGE
Lloh35:
	add	x5, x5, _method_id@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	mov	x4, x20
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh36:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_50b1cacde7465981@PAGE
Lloh37:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_50b1cacde7465981@PAGEOFF]
Lloh38:
	adrp	x5, _method_id_with_param@PAGE
Lloh39:
	add	x5, x5, _method_id_with_param@PAGEOFF
	add	x0, sp, #8
	mov	x2, x19
	mov	w3, #1
	mov	x4, x20
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh40:
	adrp	x0, l_anon.[ID].22@PAGE
Lloh41:
	add	x0, x0, l_anon.[ID].22@PAGEOFF
	mov	w1, #8
	bl	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	cbz	x0, LBB4_4
	mov	x1, x0
	add	x0, sp, #8
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_protocol::GENERATED_ID, 0)
LBB4_4:
Lloh42:
	adrp	x0, l_anon.[ID].23@PAGE
Lloh43:
	add	x0, x0, l_anon.[ID].23@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	cbz	x0, LBB4_6
	mov	x1, x0
	add	x0, sp, #8
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_protocol::GENERATED_ID, 0)
LBB4_6:
Lloh44:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_f2913b8ffb9882fe@PAGE
Lloh45:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_f2913b8ffb9882fe@PAGEOFF]
Lloh46:
	adrp	x2, l_anon.[ID].8@PAGE
Lloh47:
	add	x2, x2, l_anon.[ID].8@PAGEOFF
Lloh48:
	adrp	x4, l_anon.[ID].3@PAGE
Lloh49:
	add	x4, x4, l_anon.[ID].3@PAGEOFF
Lloh50:
	adrp	x5, _copyWithZone@PAGE
Lloh51:
	add	x5, x5, _copyWithZone@PAGEOFF
	add	x0, sp, #8
	mov	w3, #1
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x0, [sp, #8]
	bl	SYM(objc2::runtime::declare::ClassBuilder::register::GENERATED_ID, 0)
	adrp	x8, __MergedGlobals@PAGE
	str	x0, [x8, __MergedGlobals@PAGEOFF]
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB4_7:
Lloh52:
	adrp	x0, l_anon.[ID].18@PAGE
Lloh53:
	add	x0, x0, l_anon.[ID].18@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB4_8:
Lloh54:
	adrp	x0, l_anon.[ID].19@PAGE
Lloh55:
	add	x0, x0, l_anon.[ID].19@PAGEOFF
Lloh56:
	adrp	x2, l_anon.[ID].21@PAGE
Lloh57:
	add	x2, x2, l_anon.[ID].21@PAGEOFF
	mov	w1, #7
	bl	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpLdrGotLdr	Lloh7, Lloh8, Lloh9
	.loh AdrpAdd	Lloh40, Lloh41
	.loh AdrpAdd	Lloh38, Lloh39
	.loh AdrpLdr	Lloh36, Lloh37
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpAdd	Lloh32, Lloh33
	.loh AdrpLdr	Lloh30, Lloh31
	.loh AdrpAdd	Lloh28, Lloh29
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpLdr	Lloh24, Lloh25
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpLdr	Lloh18, Lloh19
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpLdr	Lloh12, Lloh13
	.loh AdrpAdd	Lloh42, Lloh43
	.loh AdrpAdd	Lloh50, Lloh51
	.loh AdrpAdd	Lloh48, Lloh49
	.loh AdrpAdd	Lloh46, Lloh47
	.loh AdrpLdr	Lloh44, Lloh45
	.loh AdrpAdd	Lloh52, Lloh53
	.loh AdrpAdd	Lloh56, Lloh57
	.loh AdrpAdd	Lloh54, Lloh55

	.p2align	2
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
	sub	sp, sp, #96
	stp	x20, x19, [sp, #64]
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	cbz	w9, LBB5_5
Lloh58:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh59:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh60:
	ldr	x2, [x8]
Lloh61:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh62:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::runtime::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB5_6
	str	x0, [sp, #24]
Lloh63:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGE
Lloh64:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGEOFF]
Lloh65:
	ldr	x1, [x8]
Lloh66:
	adrp	x4, l_anon.[ID].4@PAGE
Lloh67:
	add	x4, x4, l_anon.[ID].4@PAGEOFF
Lloh68:
	adrp	x5, SYM(objc2[CRATE_ID]::__macro_helpers::declared_ivars::dealloc::<test_declare_class[CRATE_ID]::DropIvars>, 0)@PAGE
Lloh69:
	add	x5, x5, SYM(objc2[CRATE_ID]::__macro_helpers::declared_ivars::dealloc::<test_declare_class[CRATE_ID]::DropIvars>, 0)@PAGEOFF
	add	x0, sp, #24
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x8, [sp, #24]
	str	x8, [sp, #8]
Lloh70:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh71:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh72:
	ldr	x1, [x8]
Lloh73:
	adrp	x4, l_anon.[ID].3@PAGE
Lloh74:
	add	x4, x4, l_anon.[ID].3@PAGEOFF
Lloh75:
	adrp	x5, _init_drop_ivars@PAGE
Lloh76:
	add	x5, x5, _init_drop_ivars@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x8, [sp, #8]
	str	x8, [sp, #16]
	mov	w8, #16
Lloh77:
	adrp	x9, l_anon.[ID].14@PAGE
Lloh78:
	add	x9, x9, l_anon.[ID].14@PAGEOFF
	stp	x8, x9, [sp, #32]
	mov	w8, #27
	strb	w8, [sp, #24]
Lloh79:
	adrp	x20, l_anon.[ID].10@PAGE
Lloh80:
	add	x20, x20, l_anon.[ID].10@PAGEOFF
	add	x0, sp, #16
	add	x5, sp, #24
	mov	x1, x20
	mov	w2, #5
	mov	w3, #16
	mov	w4, #3
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Lloh81:
	adrp	x1, l_anon.[ID].11@PAGE
Lloh82:
	add	x1, x1, l_anon.[ID].11@PAGEOFF
Lloh83:
	adrp	x5, l_anon.[ID].12@PAGE
Lloh84:
	add	x5, x5, l_anon.[ID].12@PAGEOFF
	add	x0, sp, #16
	mov	w2, #9
	mov	w3, #1
	mov	w4, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	ldr	x0, [sp, #16]
	bl	SYM(objc2::runtime::declare::ClassBuilder::register::GENERATED_ID, 0)
	mov	x19, x0
	mov	x1, x20
	mov	w2, #5
	bl	SYM(objc2::runtime::AnyClass::instance_variable::GENERATED_ID, 0)
	cbz	x0, LBB5_7
	bl	_ivar_getOffset
	mov	x20, x0
Lloh85:
	adrp	x1, l_anon.[ID].11@PAGE
Lloh86:
	add	x1, x1, l_anon.[ID].11@PAGEOFF
	mov	x0, x19
	mov	w2, #9
	bl	SYM(objc2::runtime::AnyClass::instance_variable::GENERATED_ID, 0)
	cbz	x0, LBB5_8
	bl	_ivar_getOffset
Lloh87:
	adrp	x8, __MergedGlobals@PAGE+32
	str	x19, [x8, __MergedGlobals@PAGEOFF+32]
Lloh88:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
	str	x20, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGEOFF]
Lloh89:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGE
	str	x0, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGEOFF]
	ldp	x29, x30, [sp, #80]
	ldp	x20, x19, [sp, #64]
	add	sp, sp, #96
	ret
LBB5_5:
Lloh90:
	adrp	x0, l_anon.[ID].18@PAGE
Lloh91:
	add	x0, x0, l_anon.[ID].18@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB5_6:
Lloh92:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh93:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
Lloh94:
	adrp	x2, l_anon.[ID].25@PAGE
Lloh95:
	add	x2, x2, l_anon.[ID].25@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
LBB5_7:
	bl	SYM(objc2::__macro_helpers::declared_ivars::register_with_ivars::get_ivar_failed::GENERATED_ID, 0)
LBB5_8:
	bl	SYM(objc2::__macro_helpers::declared_ivars::register_with_ivars::get_drop_flag_failed::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh61, Lloh62
	.loh AdrpLdrGotLdr	Lloh58, Lloh59, Lloh60
	.loh AdrpAdd	Lloh83, Lloh84
	.loh AdrpAdd	Lloh81, Lloh82
	.loh AdrpAdd	Lloh79, Lloh80
	.loh AdrpAdd	Lloh77, Lloh78
	.loh AdrpAdd	Lloh75, Lloh76
	.loh AdrpAdd	Lloh73, Lloh74
	.loh AdrpLdrGotLdr	Lloh70, Lloh71, Lloh72
	.loh AdrpAdd	Lloh68, Lloh69
	.loh AdrpAdd	Lloh66, Lloh67
	.loh AdrpLdrGotLdr	Lloh63, Lloh64, Lloh65
	.loh AdrpAdd	Lloh85, Lloh86
	.loh AdrpAdrp	Lloh88, Lloh89
	.loh AdrpAdrp	Lloh87, Lloh88
	.loh AdrpAdd	Lloh90, Lloh91
	.loh AdrpAdd	Lloh94, Lloh95
	.loh AdrpAdd	Lloh92, Lloh93

	.p2align	2
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
	sub	sp, sp, #96
	stp	x20, x19, [sp, #64]
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	cbz	w9, LBB6_4
Lloh96:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh97:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh98:
	ldr	x2, [x8]
Lloh99:
	adrp	x0, l_anon.[ID].15@PAGE
Lloh100:
	add	x0, x0, l_anon.[ID].15@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::runtime::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB6_5
	str	x0, [sp, #8]
Lloh101:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh102:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh103:
	ldr	x1, [x8]
Lloh104:
	adrp	x4, l_anon.[ID].3@PAGE
Lloh105:
	add	x4, x4, l_anon.[ID].3@PAGEOFF
Lloh106:
	adrp	x5, _init_forgetable_ivars@PAGE
Lloh107:
	add	x5, x5, _init_forgetable_ivars@PAGEOFF
	mov	w19, #8
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x8, [sp, #8]
	str	x8, [sp, #16]
Lloh108:
	adrp	x8, l_anon.[ID].13@PAGE
Lloh109:
	add	x8, x8, l_anon.[ID].13@PAGEOFF
	stp	x19, x8, [sp, #32]
	mov	w8, #27
	strb	w8, [sp, #24]
Lloh110:
	adrp	x20, l_anon.[ID].10@PAGE
Lloh111:
	add	x20, x20, l_anon.[ID].10@PAGEOFF
	add	x0, sp, #16
	add	x5, sp, #24
	mov	x1, x20
	mov	w2, #5
	mov	w3, #8
	mov	w4, #2
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	ldr	x0, [sp, #16]
	bl	SYM(objc2::runtime::declare::ClassBuilder::register::GENERATED_ID, 0)
	mov	x19, x0
	mov	x1, x20
	mov	w2, #5
	bl	SYM(objc2::runtime::AnyClass::instance_variable::GENERATED_ID, 0)
	cbz	x0, LBB6_6
	bl	_ivar_getOffset
Lloh112:
	adrp	x8, __MergedGlobals@PAGE+16
	str	x19, [x8, __MergedGlobals@PAGEOFF+16]
Lloh113:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGE
	str	x0, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGEOFF]
	ldp	x29, x30, [sp, #80]
	ldp	x20, x19, [sp, #64]
	add	sp, sp, #96
	ret
LBB6_4:
Lloh114:
	adrp	x0, l_anon.[ID].18@PAGE
Lloh115:
	add	x0, x0, l_anon.[ID].18@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB6_5:
Lloh116:
	adrp	x0, l_anon.[ID].15@PAGE
Lloh117:
	add	x0, x0, l_anon.[ID].15@PAGEOFF
Lloh118:
	adrp	x2, l_anon.[ID].24@PAGE
Lloh119:
	add	x2, x2, l_anon.[ID].24@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
LBB6_6:
	bl	SYM(objc2::__macro_helpers::declared_ivars::register_with_ivars::get_ivar_failed::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh99, Lloh100
	.loh AdrpLdrGotLdr	Lloh96, Lloh97, Lloh98
	.loh AdrpAdd	Lloh110, Lloh111
	.loh AdrpAdd	Lloh108, Lloh109
	.loh AdrpAdd	Lloh106, Lloh107
	.loh AdrpAdd	Lloh104, Lloh105
	.loh AdrpLdrGotLdr	Lloh101, Lloh102, Lloh103
	.loh AdrpAdrp	Lloh112, Lloh113
	.loh AdrpAdd	Lloh114, Lloh115
	.loh AdrpAdd	Lloh118, Lloh119
	.loh AdrpAdd	Lloh116, Lloh117

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
Lloh120:
	adrp	x8, __MergedGlobals@PAGE+24
Lloh121:
	add	x8, x8, __MergedGlobals@PAGEOFF+24
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB10_2
Lloh122:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh123:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
	ret
LBB10_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh124:
	adrp	x0, __MergedGlobals@PAGE+24
Lloh125:
	add	x0, x0, __MergedGlobals@PAGEOFF+24
Lloh126:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh127:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
Lloh128:
	adrp	x4, l_anon.[ID].24@PAGE
Lloh129:
	add	x4, x4, l_anon.[ID].24@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh130:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh131:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
	ret
	.loh AdrpAdd	Lloh120, Lloh121
	.loh AdrpLdr	Lloh122, Lloh123
	.loh AdrpLdr	Lloh130, Lloh131
	.loh AdrpAdd	Lloh128, Lloh129
	.loh AdrpAdd	Lloh126, Lloh127
	.loh AdrpAdd	Lloh124, Lloh125

	.globl	_access_forgetable_ivars
	.p2align	2
_access_forgetable_ivars:
Lloh132:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGE
Lloh133:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGEOFF]
	add	x8, x0, x8
	ldr	w1, [x8]
	ldrb	w0, [x8, #4]
	ret
	.loh AdrpLdr	Lloh132, Lloh133

	.globl	SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0):
	; InlineAsm Start
	; InlineAsm End
	ret

	.globl	_access_drop_ivars_class
	.p2align	2
_access_drop_ivars_class:
Lloh134:
	adrp	x8, __MergedGlobals@PAGE+40
Lloh135:
	add	x8, x8, __MergedGlobals@PAGEOFF+40
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB13_2
Lloh136:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh137:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+32]
	ret
LBB13_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh138:
	adrp	x0, __MergedGlobals@PAGE+40
Lloh139:
	add	x0, x0, __MergedGlobals@PAGEOFF+40
Lloh140:
	adrp	x3, l_anon.[ID].1@PAGE
Lloh141:
	add	x3, x3, l_anon.[ID].1@PAGEOFF
Lloh142:
	adrp	x4, l_anon.[ID].25@PAGE
Lloh143:
	add	x4, x4, l_anon.[ID].25@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh144:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh145:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+32]
	ret
	.loh AdrpAdd	Lloh134, Lloh135
	.loh AdrpLdr	Lloh136, Lloh137
	.loh AdrpLdr	Lloh144, Lloh145
	.loh AdrpAdd	Lloh142, Lloh143
	.loh AdrpAdd	Lloh140, Lloh141
	.loh AdrpAdd	Lloh138, Lloh139

	.globl	_access_drop_ivars
	.p2align	2
_access_drop_ivars:
Lloh146:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
Lloh147:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGEOFF]
	add	x8, x0, x8
	ldp	x0, x1, [x8]
	ret
	.loh AdrpLdr	Lloh146, Lloh147

	.globl	SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh148:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh149:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB15_2
Lloh150:
	adrp	x8, __MergedGlobals@PAGE
Lloh151:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
	ret
LBB15_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh152:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh153:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh154:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh155:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh156:
	adrp	x4, l_anon.[ID].21@PAGE
Lloh157:
	add	x4, x4, l_anon.[ID].21@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh158:
	adrp	x8, __MergedGlobals@PAGE
Lloh159:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh148, Lloh149
	.loh AdrpLdr	Lloh150, Lloh151
	.loh AdrpLdr	Lloh158, Lloh159
	.loh AdrpAdd	Lloh156, Lloh157
	.loh AdrpAdd	Lloh154, Lloh155
	.loh AdrpAdd	Lloh152, Lloh153

	.globl	_get_class
	.p2align	2
_get_class:
Lloh160:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh161:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB16_2
Lloh162:
	adrp	x8, __MergedGlobals@PAGE
Lloh163:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
	ret
LBB16_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh164:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh165:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh166:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh167:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh168:
	adrp	x4, l_anon.[ID].21@PAGE
Lloh169:
	add	x4, x4, l_anon.[ID].21@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh170:
	adrp	x8, __MergedGlobals@PAGE
Lloh171:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh160, Lloh161
	.loh AdrpLdr	Lloh162, Lloh163
	.loh AdrpLdr	Lloh170, Lloh171
	.loh AdrpAdd	Lloh168, Lloh169
	.loh AdrpAdd	Lloh166, Lloh167
	.loh AdrpAdd	Lloh164, Lloh165

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
Lloh172:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh173:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB19_2
LBB19_1:
Lloh174:
	adrp	x8, __MergedGlobals@PAGE
Lloh175:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
Lloh176:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh177:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh178:
	ldr	x1, [x8]
	bl	_objc_msgSend
	bl	_objc_autoreleaseReturnValue
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB19_2:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh179:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh180:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh181:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh182:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh183:
	adrp	x4, l_anon.[ID].21@PAGE
Lloh184:
	add	x4, x4, l_anon.[ID].21@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	b	LBB19_1
	.loh AdrpAdd	Lloh172, Lloh173
	.loh AdrpLdrGotLdr	Lloh176, Lloh177, Lloh178
	.loh AdrpAdrp	Lloh174, Lloh176
	.loh AdrpLdr	Lloh174, Lloh175
	.loh AdrpAdd	Lloh183, Lloh184
	.loh AdrpAdd	Lloh181, Lloh182
	.loh AdrpAdd	Lloh179, Lloh180

	.globl	_method_id_with_param
	.p2align	2
_method_id_with_param:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x2
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	cbz	w19, LBB20_2
	mov	x19, x0
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	x20, x0
	mov	x0, x19
	bl	_objc_release
	mov	x0, x20
LBB20_2:
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_autoreleaseReturnValue

	.globl	_copyWithZone
	.p2align	2
_copyWithZone:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh185:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh186:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB21_2
LBB21_1:
Lloh187:
	adrp	x8, __MergedGlobals@PAGE
Lloh188:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
Lloh189:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh190:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh191:
	ldr	x1, [x8]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB21_2:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh192:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh193:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh194:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh195:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh196:
	adrp	x4, l_anon.[ID].21@PAGE
Lloh197:
	add	x4, x4, l_anon.[ID].21@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	b	LBB21_1
	.loh AdrpAdd	Lloh185, Lloh186
	.loh AdrpLdrGotLdr	Lloh189, Lloh190, Lloh191
	.loh AdrpAdrp	Lloh187, Lloh189
	.loh AdrpLdr	Lloh187, Lloh188
	.loh AdrpAdd	Lloh196, Lloh197
	.loh AdrpAdd	Lloh194, Lloh195
	.loh AdrpAdd	Lloh192, Lloh193

	.globl	SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh198:
	adrp	x8, __MergedGlobals@PAGE+24
Lloh199:
	add	x8, x8, __MergedGlobals@PAGEOFF+24
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB22_2
Lloh200:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh201:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
	ret
LBB22_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh202:
	adrp	x0, __MergedGlobals@PAGE+24
Lloh203:
	add	x0, x0, __MergedGlobals@PAGEOFF+24
Lloh204:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh205:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
Lloh206:
	adrp	x4, l_anon.[ID].24@PAGE
Lloh207:
	add	x4, x4, l_anon.[ID].24@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh208:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh209:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
	ret
	.loh AdrpAdd	Lloh198, Lloh199
	.loh AdrpLdr	Lloh200, Lloh201
	.loh AdrpLdr	Lloh208, Lloh209
	.loh AdrpAdd	Lloh206, Lloh207
	.loh AdrpAdd	Lloh204, Lloh205
	.loh AdrpAdd	Lloh202, Lloh203

	.globl	_init_forgetable_ivars
	.p2align	2
_init_forgetable_ivars:
	cbz	x0, LBB23_2
Lloh210:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGE
Lloh211:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGEOFF]
	add	x8, x0, x8
	mov	w9, #43
	str	w9, [x8]
	mov	w9, #42
	strb	w9, [x8, #4]
LBB23_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh212:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_dea6e68a0f2fe4ca@PAGE
Lloh213:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_dea6e68a0f2fe4ca@PAGEOFF]
Lloh214:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh215:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh216:
	ldr	x8, [x8]
	stp	x0, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpLdr	Lloh210, Lloh211
	.loh AdrpLdrGotLdr	Lloh214, Lloh215, Lloh216
	.loh AdrpAdrp	Lloh212, Lloh214
	.loh AdrpLdr	Lloh212, Lloh213

	.globl	SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh217:
	adrp	x8, __MergedGlobals@PAGE+40
Lloh218:
	add	x8, x8, __MergedGlobals@PAGEOFF+40
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB24_2
Lloh219:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh220:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+32]
	ret
LBB24_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh221:
	adrp	x0, __MergedGlobals@PAGE+40
Lloh222:
	add	x0, x0, __MergedGlobals@PAGEOFF+40
Lloh223:
	adrp	x3, l_anon.[ID].1@PAGE
Lloh224:
	add	x3, x3, l_anon.[ID].1@PAGEOFF
Lloh225:
	adrp	x4, l_anon.[ID].25@PAGE
Lloh226:
	add	x4, x4, l_anon.[ID].25@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh227:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh228:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+32]
	ret
	.loh AdrpAdd	Lloh217, Lloh218
	.loh AdrpLdr	Lloh219, Lloh220
	.loh AdrpLdr	Lloh227, Lloh228
	.loh AdrpAdd	Lloh225, Lloh226
	.loh AdrpAdd	Lloh223, Lloh224
	.loh AdrpAdd	Lloh221, Lloh222

	.globl	_init_drop_ivars
	.p2align	2
_init_drop_ivars:
	sub	sp, sp, #64
	stp	x22, x21, [sp, #16]
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	mov	x19, x0
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	x20, x0
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	adrp	x22, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGE
	cbz	x19, LBB25_2
Lloh229:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
Lloh230:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGEOFF]
	add	x8, x19, x8
	stp	x20, x0, [x8]
	ldr	x8, [x22, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGEOFF]
	mov	w9, #15
	strb	w9, [x19, x8]
	b	LBB25_3
LBB25_2:
	mov	x21, x0
	mov	x0, x20
	bl	_objc_release
	mov	x0, x21
	bl	_objc_release
LBB25_3:
Lloh231:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_46e92b66c48ba6b7@PAGE
Lloh232:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_46e92b66c48ba6b7@PAGEOFF]
Lloh233:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh234:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh235:
	ldr	x8, [x8]
	stp	x19, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	cbz	x0, LBB25_5
	ldr	x8, [x22, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGEOFF]
	mov	w9, #255
	strb	w9, [x0, x8]
LBB25_5:
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	ldp	x22, x21, [sp, #16]
	add	sp, sp, #64
	ret
	.loh AdrpLdr	Lloh229, Lloh230
	.loh AdrpLdrGotLdr	Lloh233, Lloh234, Lloh235
	.loh AdrpAdrp	Lloh231, Lloh233
	.loh AdrpLdr	Lloh231, Lloh232

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.quad	SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}>, 0)
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.p2align	3, 0x0
l_anon.[ID].1:
	.quad	SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}>, 0)
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.p2align	3, 0x0
l_anon.[ID].2:
	.quad	SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}>, 0)
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].3:
	.byte	19
	.space	39

	.p2align	3, 0x0
l_anon.[ID].4:
	.byte	17
	.space	39

	.p2align	3, 0x0
l_anon.[ID].5:
	.byte	16
	.space	39

l_anon.[ID].6:
	.ascii	"_NSZone"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].7:
	.byte	28
	.space	7
	.quad	l_anon.[ID].6
	.asciz	"\007\000\000\000\000\000\000"
	.quad	8
	.space	8

	.p2align	3, 0x0
l_anon.[ID].8:
	.byte	25
	.space	7
	.quad	l_anon.[ID].7
	.space	24

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].9:
	.byte	21
	.space	39

l_anon.[ID].10:
	.ascii	"ivars"

l_anon.[ID].11:
	.ascii	"drop_flag"

	.p2align	3, 0x0
l_anon.[ID].12:
	.byte	5
	.space	39

	.p2align	3, 0x0
l_anon.[ID].13:
	.byte	7
	.space	39

	.p2align	3, 0x0
l_anon.[ID].14:
	.byte	9
	.space	39

l_anon.[ID].15:
	.ascii	"ForgetableIvars"

l_anon.[ID].16:
	.ascii	"DropIvars"

l_anon.[ID].17:
	.ascii	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].18:
	.quad	l_anon.[ID].17
	.asciz	"p\000\000\000\000\000\000\000\225\000\000\0002\000\000"

	.section	__TEXT,__const
l_anon.[ID].19:
	.ascii	"NoIvars"

	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2),8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 1)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 1),8,3
l_anon.[ID].20:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].21:
	.quad	l_anon.[ID].20
	.asciz	"5\000\000\000\000\000\000\000\016\000\000\000\001\000\000"

	.section	__TEXT,__literal8,8byte_literals
l_anon.[ID].22:
	.ascii	"NSObject"

	.section	__TEXT,__const
l_anon.[ID].23:
	.ascii	"NSCopying"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_c1ccd9f2c8e68869
L_OBJC_METH_VAR_NAME_c1ccd9f2c8e68869:
	.asciz	"classMethod"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_c1ccd9f2c8e68869
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_c1ccd9f2c8e68869:
	.quad	L_OBJC_METH_VAR_NAME_c1ccd9f2c8e68869

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_c1ccd9f2c8e68869
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_c1ccd9f2c8e68869:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_654faaf1a88864b3
L_OBJC_METH_VAR_NAME_654faaf1a88864b3:
	.asciz	"method"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_654faaf1a88864b3
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_654faaf1a88864b3:
	.quad	L_OBJC_METH_VAR_NAME_654faaf1a88864b3

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_654faaf1a88864b3
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_654faaf1a88864b3:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_5d27bc76c3596041
L_OBJC_METH_VAR_NAME_5d27bc76c3596041:
	.asciz	"methodBool:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_5d27bc76c3596041
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_5d27bc76c3596041:
	.quad	L_OBJC_METH_VAR_NAME_5d27bc76c3596041

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_5d27bc76c3596041
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_5d27bc76c3596041:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_026f8b3b5bb3f00d
L_OBJC_METH_VAR_NAME_026f8b3b5bb3f00d:
	.asciz	"methodId"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_026f8b3b5bb3f00d
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_026f8b3b5bb3f00d:
	.quad	L_OBJC_METH_VAR_NAME_026f8b3b5bb3f00d

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_026f8b3b5bb3f00d
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_026f8b3b5bb3f00d:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_50b1cacde7465981
L_OBJC_METH_VAR_NAME_50b1cacde7465981:
	.asciz	"methodIdWithParam:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_50b1cacde7465981
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_50b1cacde7465981:
	.quad	L_OBJC_METH_VAR_NAME_50b1cacde7465981

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_50b1cacde7465981
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_50b1cacde7465981:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_f2913b8ffb9882fe
L_OBJC_METH_VAR_NAME_f2913b8ffb9882fe:
	.asciz	"copyWithZone:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_f2913b8ffb9882fe
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_f2913b8ffb9882fe:
	.quad	L_OBJC_METH_VAR_NAME_f2913b8ffb9882fe

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f2913b8ffb9882fe
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f2913b8ffb9882fe:
	.asciz	"\000\000\000\000@\000\000"

	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1),8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 2)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 2),8,3
	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].24:
	.quad	l_anon.[ID].20
	.asciz	"5\000\000\000\000\000\000\000O\000\000\000\001\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_dea6e68a0f2fe4ca
L_OBJC_METH_VAR_NAME_dea6e68a0f2fe4ca:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_dea6e68a0f2fe4ca
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_dea6e68a0f2fe4ca:
	.quad	L_OBJC_METH_VAR_NAME_dea6e68a0f2fe4ca

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_dea6e68a0f2fe4ca
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_dea6e68a0f2fe4ca:
	.asciz	"\000\000\000\000@\000\000"

	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0),8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0),8,3
	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].25:
	.quad	l_anon.[ID].20
	.asciz	"5\000\000\000\000\000\000\000x\000\000\000\001\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_46e92b66c48ba6b7
L_OBJC_METH_VAR_NAME_46e92b66c48ba6b7:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_46e92b66c48ba6b7
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_46e92b66c48ba6b7:
	.quad	L_OBJC_METH_VAR_NAME_46e92b66c48ba6b7

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_46e92b66c48ba6b7
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_46e92b66c48ba6b7:
	.asciz	"\000\000\000\000@\000\000"

.zerofill __DATA,__bss,__MergedGlobals,48,3
.subsections_via_symbols
