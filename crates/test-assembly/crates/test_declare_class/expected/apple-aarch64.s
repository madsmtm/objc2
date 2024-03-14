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
	sub	sp, sp, #64
	stp	x22, x21, [sp, #16]
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
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
	adrp	x0, l_anon.[ID].17@PAGE
Lloh11:
	add	x0, x0, l_anon.[ID].17@PAGEOFF
	mov	w1, #7
	bl	SYM(objc2::runtime::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB4_8
	str	x0, [sp, #8]
Lloh12:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_03fd85b0462f54e9@PAGE
Lloh13:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_03fd85b0462f54e9@PAGEOFF]
Lloh14:
	adrp	x19, l_anon.[ID].3@PAGE
Lloh15:
	add	x19, x19, l_anon.[ID].3@PAGEOFF
Lloh16:
	adrp	x4, l_anon.[ID].10@PAGE
Lloh17:
	add	x4, x4, l_anon.[ID].10@PAGEOFF
Lloh18:
	adrp	x5, _get_class@PAGE
Lloh19:
	add	x5, x5, _get_class@PAGEOFF
	add	x0, sp, #8
	mov	x2, x19
	mov	x3, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_class_method_inner::GENERATED_ID, 0)
Lloh20:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_cf773331f3cfba54@PAGE
Lloh21:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_cf773331f3cfba54@PAGEOFF]
Lloh22:
	adrp	x4, l_anon.[ID].5@PAGE
Lloh23:
	add	x4, x4, l_anon.[ID].5@PAGEOFF
Lloh24:
	adrp	x5, _method_simple@PAGE
Lloh25:
	add	x5, x5, _method_simple@PAGEOFF
	add	x0, sp, #8
	mov	x2, x19
	mov	x3, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh26:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_abdcbb85641cd990@PAGE
Lloh27:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_abdcbb85641cd990@PAGEOFF]
Lloh28:
	adrp	x20, l_anon.[ID].6@PAGE
Lloh29:
	add	x20, x20, l_anon.[ID].6@PAGEOFF
Lloh30:
	adrp	x5, _method_bool@PAGE
Lloh31:
	add	x5, x5, _method_bool@PAGEOFF
	add	x0, sp, #8
	mov	x2, x20
	mov	w3, #1
	mov	x4, x20
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh32:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_ef8de92414f2d9c8@PAGE
Lloh33:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_ef8de92414f2d9c8@PAGEOFF]
Lloh34:
	adrp	x21, l_anon.[ID].4@PAGE
Lloh35:
	add	x21, x21, l_anon.[ID].4@PAGEOFF
Lloh36:
	adrp	x5, _method_id@PAGE
Lloh37:
	add	x5, x5, _method_id@PAGEOFF
	add	x0, sp, #8
	mov	x2, x19
	mov	x3, #0
	mov	x4, x21
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh38:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_4a611090161f3fae@PAGE
Lloh39:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_4a611090161f3fae@PAGEOFF]
Lloh40:
	adrp	x5, _method_id_with_param@PAGE
Lloh41:
	add	x5, x5, _method_id_with_param@PAGEOFF
	add	x0, sp, #8
	mov	x2, x20
	mov	w3, #1
	mov	x4, x21
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh42:
	adrp	x0, l_anon.[ID].23@PAGE
Lloh43:
	add	x0, x0, l_anon.[ID].23@PAGEOFF
	mov	w1, #8
	bl	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	cbz	x0, LBB4_4
	mov	x1, x0
	add	x0, sp, #8
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_protocol::GENERATED_ID, 0)
LBB4_4:
Lloh44:
	adrp	x0, l_anon.[ID].24@PAGE
Lloh45:
	add	x0, x0, l_anon.[ID].24@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	cbz	x0, LBB4_6
	mov	x1, x0
	add	x0, sp, #8
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_protocol::GENERATED_ID, 0)
LBB4_6:
Lloh46:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_2837f061c311eb14@PAGE
Lloh47:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_2837f061c311eb14@PAGEOFF]
Lloh48:
	adrp	x2, l_anon.[ID].9@PAGE
Lloh49:
	add	x2, x2, l_anon.[ID].9@PAGEOFF
Lloh50:
	adrp	x4, l_anon.[ID].4@PAGE
Lloh51:
	add	x4, x4, l_anon.[ID].4@PAGEOFF
Lloh52:
	adrp	x5, _copyWithZone@PAGE
Lloh53:
	add	x5, x5, _copyWithZone@PAGEOFF
	add	x0, sp, #8
	mov	w3, #1
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x0, [sp, #8]
	bl	SYM(objc2::runtime::declare::ClassBuilder::register::GENERATED_ID, 0)
	adrp	x8, __MergedGlobals@PAGE
	str	x0, [x8, __MergedGlobals@PAGEOFF]
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	ldp	x22, x21, [sp, #16]
	add	sp, sp, #64
	ret
LBB4_7:
Lloh54:
	adrp	x0, l_anon.[ID].20@PAGE
Lloh55:
	add	x0, x0, l_anon.[ID].20@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB4_8:
Lloh56:
	adrp	x0, l_anon.[ID].17@PAGE
Lloh57:
	add	x0, x0, l_anon.[ID].17@PAGEOFF
Lloh58:
	adrp	x2, l_anon.[ID].22@PAGE
Lloh59:
	add	x2, x2, l_anon.[ID].22@PAGEOFF
	mov	w1, #7
	bl	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpLdrGotLdr	Lloh7, Lloh8, Lloh9
	.loh AdrpAdd	Lloh42, Lloh43
	.loh AdrpAdd	Lloh40, Lloh41
	.loh AdrpLdr	Lloh38, Lloh39
	.loh AdrpAdd	Lloh36, Lloh37
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpLdr	Lloh32, Lloh33
	.loh AdrpAdd	Lloh30, Lloh31
	.loh AdrpAdd	Lloh28, Lloh29
	.loh AdrpLdr	Lloh26, Lloh27
	.loh AdrpAdd	Lloh24, Lloh25
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpLdr	Lloh20, Lloh21
	.loh AdrpAdd	Lloh18, Lloh19
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpLdr	Lloh12, Lloh13
	.loh AdrpAdd	Lloh44, Lloh45
	.loh AdrpAdd	Lloh52, Lloh53
	.loh AdrpAdd	Lloh50, Lloh51
	.loh AdrpAdd	Lloh48, Lloh49
	.loh AdrpLdr	Lloh46, Lloh47
	.loh AdrpAdd	Lloh54, Lloh55
	.loh AdrpAdd	Lloh58, Lloh59
	.loh AdrpAdd	Lloh56, Lloh57

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
Lloh60:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh61:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh62:
	ldr	x2, [x8]
Lloh63:
	adrp	x0, l_anon.[ID].18@PAGE
Lloh64:
	add	x0, x0, l_anon.[ID].18@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::runtime::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB5_6
	str	x0, [sp, #24]
Lloh65:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGE
Lloh66:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGEOFF]
Lloh67:
	ldr	x1, [x8]
Lloh68:
	adrp	x19, l_anon.[ID].3@PAGE
Lloh69:
	add	x19, x19, l_anon.[ID].3@PAGEOFF
Lloh70:
	adrp	x4, l_anon.[ID].5@PAGE
Lloh71:
	add	x4, x4, l_anon.[ID].5@PAGEOFF
Lloh72:
	adrp	x5, SYM(objc2[CRATE_ID]::__macro_helpers::declared_ivars::dealloc::<test_declare_class[CRATE_ID]::DropIvars>, 0)@PAGE
Lloh73:
	add	x5, x5, SYM(objc2[CRATE_ID]::__macro_helpers::declared_ivars::dealloc::<test_declare_class[CRATE_ID]::DropIvars>, 0)@PAGEOFF
	add	x0, sp, #24
	mov	x2, x19
	mov	x3, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x8, [sp, #24]
	str	x8, [sp, #8]
Lloh74:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh75:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh76:
	ldr	x1, [x8]
Lloh77:
	adrp	x4, l_anon.[ID].4@PAGE
Lloh78:
	add	x4, x4, l_anon.[ID].4@PAGEOFF
Lloh79:
	adrp	x5, _init_drop_ivars@PAGE
Lloh80:
	add	x5, x5, _init_drop_ivars@PAGEOFF
	add	x0, sp, #8
	mov	x2, x19
	mov	x3, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x8, [sp, #8]
	str	x8, [sp, #16]
	mov	w8, #16
Lloh81:
	adrp	x9, l_anon.[ID].15@PAGE
Lloh82:
	add	x9, x9, l_anon.[ID].15@PAGEOFF
	stp	x8, x9, [sp, #32]
	mov	w8, #27
	strb	w8, [sp, #24]
Lloh83:
	adrp	x20, l_anon.[ID].11@PAGE
Lloh84:
	add	x20, x20, l_anon.[ID].11@PAGEOFF
	add	x0, sp, #16
	add	x5, sp, #24
	mov	x1, x20
	mov	w2, #5
	mov	w3, #16
	mov	w4, #3
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Lloh85:
	adrp	x1, l_anon.[ID].12@PAGE
Lloh86:
	add	x1, x1, l_anon.[ID].12@PAGEOFF
Lloh87:
	adrp	x5, l_anon.[ID].13@PAGE
Lloh88:
	add	x5, x5, l_anon.[ID].13@PAGEOFF
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
Lloh89:
	adrp	x1, l_anon.[ID].12@PAGE
Lloh90:
	add	x1, x1, l_anon.[ID].12@PAGEOFF
	mov	x0, x19
	mov	w2, #9
	bl	SYM(objc2::runtime::AnyClass::instance_variable::GENERATED_ID, 0)
	cbz	x0, LBB5_8
	bl	_ivar_getOffset
Lloh91:
	adrp	x8, __MergedGlobals@PAGE+32
	str	x19, [x8, __MergedGlobals@PAGEOFF+32]
Lloh92:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
	str	x20, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGEOFF]
Lloh93:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGE
	str	x0, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGEOFF]
	ldp	x29, x30, [sp, #80]
	ldp	x20, x19, [sp, #64]
	add	sp, sp, #96
	ret
LBB5_5:
Lloh94:
	adrp	x0, l_anon.[ID].20@PAGE
Lloh95:
	add	x0, x0, l_anon.[ID].20@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB5_6:
Lloh96:
	adrp	x0, l_anon.[ID].18@PAGE
Lloh97:
	add	x0, x0, l_anon.[ID].18@PAGEOFF
Lloh98:
	adrp	x2, l_anon.[ID].26@PAGE
Lloh99:
	add	x2, x2, l_anon.[ID].26@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
LBB5_7:
	bl	SYM(objc2::__macro_helpers::declared_ivars::register_with_ivars::get_ivar_failed::GENERATED_ID, 0)
LBB5_8:
	bl	SYM(objc2::__macro_helpers::declared_ivars::register_with_ivars::get_drop_flag_failed::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh63, Lloh64
	.loh AdrpLdrGotLdr	Lloh60, Lloh61, Lloh62
	.loh AdrpAdd	Lloh87, Lloh88
	.loh AdrpAdd	Lloh85, Lloh86
	.loh AdrpAdd	Lloh83, Lloh84
	.loh AdrpAdd	Lloh81, Lloh82
	.loh AdrpAdd	Lloh79, Lloh80
	.loh AdrpAdd	Lloh77, Lloh78
	.loh AdrpLdrGotLdr	Lloh74, Lloh75, Lloh76
	.loh AdrpAdd	Lloh72, Lloh73
	.loh AdrpAdd	Lloh70, Lloh71
	.loh AdrpAdd	Lloh68, Lloh69
	.loh AdrpLdrGotLdr	Lloh65, Lloh66, Lloh67
	.loh AdrpAdd	Lloh89, Lloh90
	.loh AdrpAdrp	Lloh92, Lloh93
	.loh AdrpAdrp	Lloh91, Lloh92
	.loh AdrpAdd	Lloh94, Lloh95
	.loh AdrpAdd	Lloh98, Lloh99
	.loh AdrpAdd	Lloh96, Lloh97

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
Lloh100:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh101:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh102:
	ldr	x2, [x8]
Lloh103:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh104:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::runtime::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB6_5
	str	x0, [sp, #8]
Lloh105:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh106:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh107:
	ldr	x1, [x8]
Lloh108:
	adrp	x2, l_anon.[ID].3@PAGE
Lloh109:
	add	x2, x2, l_anon.[ID].3@PAGEOFF
Lloh110:
	adrp	x4, l_anon.[ID].4@PAGE
Lloh111:
	add	x4, x4, l_anon.[ID].4@PAGEOFF
Lloh112:
	adrp	x5, _init_forgetable_ivars@PAGE
Lloh113:
	add	x5, x5, _init_forgetable_ivars@PAGEOFF
	add	x0, sp, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x8, [sp, #8]
	str	x8, [sp, #16]
	mov	w8, #8
Lloh114:
	adrp	x9, l_anon.[ID].14@PAGE
Lloh115:
	add	x9, x9, l_anon.[ID].14@PAGEOFF
	stp	x8, x9, [sp, #32]
	mov	w8, #27
	strb	w8, [sp, #24]
Lloh116:
	adrp	x20, l_anon.[ID].11@PAGE
Lloh117:
	add	x20, x20, l_anon.[ID].11@PAGEOFF
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
Lloh118:
	adrp	x8, __MergedGlobals@PAGE+16
	str	x19, [x8, __MergedGlobals@PAGEOFF+16]
Lloh119:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGE
	str	x0, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGEOFF]
	ldp	x29, x30, [sp, #80]
	ldp	x20, x19, [sp, #64]
	add	sp, sp, #96
	ret
LBB6_4:
Lloh120:
	adrp	x0, l_anon.[ID].20@PAGE
Lloh121:
	add	x0, x0, l_anon.[ID].20@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB6_5:
Lloh122:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh123:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
Lloh124:
	adrp	x2, l_anon.[ID].25@PAGE
Lloh125:
	add	x2, x2, l_anon.[ID].25@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
LBB6_6:
	bl	SYM(objc2::__macro_helpers::declared_ivars::register_with_ivars::get_ivar_failed::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh103, Lloh104
	.loh AdrpLdrGotLdr	Lloh100, Lloh101, Lloh102
	.loh AdrpAdd	Lloh116, Lloh117
	.loh AdrpAdd	Lloh114, Lloh115
	.loh AdrpAdd	Lloh112, Lloh113
	.loh AdrpAdd	Lloh110, Lloh111
	.loh AdrpAdd	Lloh108, Lloh109
	.loh AdrpLdrGotLdr	Lloh105, Lloh106, Lloh107
	.loh AdrpAdrp	Lloh118, Lloh119
	.loh AdrpAdd	Lloh120, Lloh121
	.loh AdrpAdd	Lloh124, Lloh125
	.loh AdrpAdd	Lloh122, Lloh123

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
Lloh126:
	adrp	x8, __MergedGlobals@PAGE+24
Lloh127:
	add	x8, x8, __MergedGlobals@PAGEOFF+24
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB10_2
Lloh128:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh129:
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
Lloh130:
	adrp	x0, __MergedGlobals@PAGE+24
Lloh131:
	add	x0, x0, __MergedGlobals@PAGEOFF+24
Lloh132:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh133:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
Lloh134:
	adrp	x4, l_anon.[ID].25@PAGE
Lloh135:
	add	x4, x4, l_anon.[ID].25@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh136:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh137:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
	ret
	.loh AdrpAdd	Lloh126, Lloh127
	.loh AdrpLdr	Lloh128, Lloh129
	.loh AdrpLdr	Lloh136, Lloh137
	.loh AdrpAdd	Lloh134, Lloh135
	.loh AdrpAdd	Lloh132, Lloh133
	.loh AdrpAdd	Lloh130, Lloh131

	.globl	_access_forgetable_ivars
	.p2align	2
_access_forgetable_ivars:
Lloh138:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGE
Lloh139:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGEOFF]
	add	x8, x0, x8
	ldr	w1, [x8]
	ldrb	w0, [x8, #4]
	ret
	.loh AdrpLdr	Lloh138, Lloh139

	.globl	SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0):
	; InlineAsm Start
	; InlineAsm End
	ret

	.globl	_access_drop_ivars_class
	.p2align	2
_access_drop_ivars_class:
Lloh140:
	adrp	x8, __MergedGlobals@PAGE+40
Lloh141:
	add	x8, x8, __MergedGlobals@PAGEOFF+40
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB13_2
Lloh142:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh143:
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
Lloh144:
	adrp	x0, __MergedGlobals@PAGE+40
Lloh145:
	add	x0, x0, __MergedGlobals@PAGEOFF+40
Lloh146:
	adrp	x3, l_anon.[ID].1@PAGE
Lloh147:
	add	x3, x3, l_anon.[ID].1@PAGEOFF
Lloh148:
	adrp	x4, l_anon.[ID].26@PAGE
Lloh149:
	add	x4, x4, l_anon.[ID].26@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh150:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh151:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+32]
	ret
	.loh AdrpAdd	Lloh140, Lloh141
	.loh AdrpLdr	Lloh142, Lloh143
	.loh AdrpLdr	Lloh150, Lloh151
	.loh AdrpAdd	Lloh148, Lloh149
	.loh AdrpAdd	Lloh146, Lloh147
	.loh AdrpAdd	Lloh144, Lloh145

	.globl	_access_drop_ivars
	.p2align	2
_access_drop_ivars:
Lloh152:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
Lloh153:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGEOFF]
	add	x8, x0, x8
	ldp	x0, x1, [x8]
	ret
	.loh AdrpLdr	Lloh152, Lloh153

	.globl	SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh154:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh155:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB15_2
Lloh156:
	adrp	x8, __MergedGlobals@PAGE
Lloh157:
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
Lloh158:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh159:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh160:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh161:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh162:
	adrp	x4, l_anon.[ID].22@PAGE
Lloh163:
	add	x4, x4, l_anon.[ID].22@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh164:
	adrp	x8, __MergedGlobals@PAGE
Lloh165:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh154, Lloh155
	.loh AdrpLdr	Lloh156, Lloh157
	.loh AdrpLdr	Lloh164, Lloh165
	.loh AdrpAdd	Lloh162, Lloh163
	.loh AdrpAdd	Lloh160, Lloh161
	.loh AdrpAdd	Lloh158, Lloh159

	.globl	_get_class
	.p2align	2
_get_class:
Lloh166:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh167:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB16_2
Lloh168:
	adrp	x8, __MergedGlobals@PAGE
Lloh169:
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
Lloh170:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh171:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh172:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh173:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh174:
	adrp	x4, l_anon.[ID].22@PAGE
Lloh175:
	add	x4, x4, l_anon.[ID].22@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh176:
	adrp	x8, __MergedGlobals@PAGE
Lloh177:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh166, Lloh167
	.loh AdrpLdr	Lloh168, Lloh169
	.loh AdrpLdr	Lloh176, Lloh177
	.loh AdrpAdd	Lloh174, Lloh175
	.loh AdrpAdd	Lloh172, Lloh173
	.loh AdrpAdd	Lloh170, Lloh171

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
Lloh178:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh179:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB19_2
LBB19_1:
Lloh180:
	adrp	x8, __MergedGlobals@PAGE
Lloh181:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
Lloh182:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh183:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh184:
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
Lloh185:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh186:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh187:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh188:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh189:
	adrp	x4, l_anon.[ID].22@PAGE
Lloh190:
	add	x4, x4, l_anon.[ID].22@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	b	LBB19_1
	.loh AdrpAdd	Lloh178, Lloh179
	.loh AdrpLdrGotLdr	Lloh182, Lloh183, Lloh184
	.loh AdrpAdrp	Lloh180, Lloh182
	.loh AdrpLdr	Lloh180, Lloh181
	.loh AdrpAdd	Lloh189, Lloh190
	.loh AdrpAdd	Lloh187, Lloh188
	.loh AdrpAdd	Lloh185, Lloh186

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
Lloh191:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh192:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB21_2
LBB21_1:
Lloh193:
	adrp	x8, __MergedGlobals@PAGE
Lloh194:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
Lloh195:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh196:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh197:
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
Lloh198:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh199:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh200:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh201:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh202:
	adrp	x4, l_anon.[ID].22@PAGE
Lloh203:
	add	x4, x4, l_anon.[ID].22@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	b	LBB21_1
	.loh AdrpAdd	Lloh191, Lloh192
	.loh AdrpLdrGotLdr	Lloh195, Lloh196, Lloh197
	.loh AdrpAdrp	Lloh193, Lloh195
	.loh AdrpLdr	Lloh193, Lloh194
	.loh AdrpAdd	Lloh202, Lloh203
	.loh AdrpAdd	Lloh200, Lloh201
	.loh AdrpAdd	Lloh198, Lloh199

	.globl	SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh204:
	adrp	x8, __MergedGlobals@PAGE+24
Lloh205:
	add	x8, x8, __MergedGlobals@PAGEOFF+24
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB22_2
Lloh206:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh207:
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
Lloh208:
	adrp	x0, __MergedGlobals@PAGE+24
Lloh209:
	add	x0, x0, __MergedGlobals@PAGEOFF+24
Lloh210:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh211:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
Lloh212:
	adrp	x4, l_anon.[ID].25@PAGE
Lloh213:
	add	x4, x4, l_anon.[ID].25@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh214:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh215:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
	ret
	.loh AdrpAdd	Lloh204, Lloh205
	.loh AdrpLdr	Lloh206, Lloh207
	.loh AdrpLdr	Lloh214, Lloh215
	.loh AdrpAdd	Lloh212, Lloh213
	.loh AdrpAdd	Lloh210, Lloh211
	.loh AdrpAdd	Lloh208, Lloh209

	.globl	_init_forgetable_ivars
	.p2align	2
_init_forgetable_ivars:
	cbz	x0, LBB23_2
Lloh216:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGE
Lloh217:
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
Lloh218:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_af8966656b8b2b6c@PAGE
Lloh219:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_af8966656b8b2b6c@PAGEOFF]
Lloh220:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh221:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh222:
	ldr	x8, [x8]
	stp	x0, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpLdr	Lloh216, Lloh217
	.loh AdrpLdrGotLdr	Lloh220, Lloh221, Lloh222
	.loh AdrpAdrp	Lloh218, Lloh220
	.loh AdrpLdr	Lloh218, Lloh219

	.globl	SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh223:
	adrp	x8, __MergedGlobals@PAGE+40
Lloh224:
	add	x8, x8, __MergedGlobals@PAGEOFF+40
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB24_2
Lloh225:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh226:
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
Lloh227:
	adrp	x0, __MergedGlobals@PAGE+40
Lloh228:
	add	x0, x0, __MergedGlobals@PAGEOFF+40
Lloh229:
	adrp	x3, l_anon.[ID].1@PAGE
Lloh230:
	add	x3, x3, l_anon.[ID].1@PAGEOFF
Lloh231:
	adrp	x4, l_anon.[ID].26@PAGE
Lloh232:
	add	x4, x4, l_anon.[ID].26@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh233:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh234:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+32]
	ret
	.loh AdrpAdd	Lloh223, Lloh224
	.loh AdrpLdr	Lloh225, Lloh226
	.loh AdrpLdr	Lloh233, Lloh234
	.loh AdrpAdd	Lloh231, Lloh232
	.loh AdrpAdd	Lloh229, Lloh230
	.loh AdrpAdd	Lloh227, Lloh228

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
Lloh235:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
Lloh236:
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
Lloh237:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_6edddcebbded8f32@PAGE
Lloh238:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_6edddcebbded8f32@PAGEOFF]
Lloh239:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh240:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh241:
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
	.loh AdrpLdr	Lloh235, Lloh236
	.loh AdrpLdrGotLdr	Lloh239, Lloh240, Lloh241
	.loh AdrpAdrp	Lloh237, Lloh239
	.loh AdrpLdr	Lloh237, Lloh238

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
	.byte	0

	.p2align	3, 0x0
l_anon.[ID].4:
	.byte	19
	.space	39

	.p2align	3, 0x0
l_anon.[ID].5:
	.byte	17
	.space	39

	.p2align	3, 0x0
l_anon.[ID].6:
	.byte	16
	.space	39

l_anon.[ID].7:
	.ascii	"_NSZone"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].8:
	.byte	28
	.space	7
	.quad	l_anon.[ID].7
	.asciz	"\007\000\000\000\000\000\000"
	.quad	l_anon.[ID].3
	.space	8

	.p2align	3, 0x0
l_anon.[ID].9:
	.byte	25
	.space	7
	.quad	l_anon.[ID].8
	.space	24

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].10:
	.byte	21
	.space	39

l_anon.[ID].11:
	.ascii	"ivars"

l_anon.[ID].12:
	.ascii	"drop_flag"

	.p2align	3, 0x0
l_anon.[ID].13:
	.byte	5
	.space	39

	.p2align	3, 0x0
l_anon.[ID].14:
	.byte	7
	.space	39

	.p2align	3, 0x0
l_anon.[ID].15:
	.byte	9
	.space	39

l_anon.[ID].16:
	.ascii	"ForgetableIvars"

l_anon.[ID].17:
	.ascii	"NoIvars"

l_anon.[ID].18:
	.ascii	"DropIvars"

l_anon.[ID].19:
	.ascii	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].20:
	.quad	l_anon.[ID].19
	.asciz	"p\000\000\000\000\000\000\000\225\000\000\0002\000\000"

	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2),8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 1)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 1),8,3
	.section	__TEXT,__const
l_anon.[ID].21:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].22:
	.quad	l_anon.[ID].21
	.asciz	"5\000\000\000\000\000\000\000\016\000\000\000\001\000\000"

	.section	__TEXT,__literal8,8byte_literals
l_anon.[ID].23:
	.ascii	"NSObject"

	.section	__TEXT,__const
l_anon.[ID].24:
	.ascii	"NSCopying"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_03fd85b0462f54e9
L_OBJC_METH_VAR_NAME_03fd85b0462f54e9:
	.asciz	"classMethod"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_03fd85b0462f54e9
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_03fd85b0462f54e9:
	.quad	L_OBJC_METH_VAR_NAME_03fd85b0462f54e9

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_03fd85b0462f54e9
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_03fd85b0462f54e9:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_cf773331f3cfba54
L_OBJC_METH_VAR_NAME_cf773331f3cfba54:
	.asciz	"method"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_cf773331f3cfba54
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_cf773331f3cfba54:
	.quad	L_OBJC_METH_VAR_NAME_cf773331f3cfba54

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_cf773331f3cfba54
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_cf773331f3cfba54:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_abdcbb85641cd990
L_OBJC_METH_VAR_NAME_abdcbb85641cd990:
	.asciz	"methodBool:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_abdcbb85641cd990
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_abdcbb85641cd990:
	.quad	L_OBJC_METH_VAR_NAME_abdcbb85641cd990

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_abdcbb85641cd990
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_abdcbb85641cd990:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_ef8de92414f2d9c8
L_OBJC_METH_VAR_NAME_ef8de92414f2d9c8:
	.asciz	"methodId"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_ef8de92414f2d9c8
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_ef8de92414f2d9c8:
	.quad	L_OBJC_METH_VAR_NAME_ef8de92414f2d9c8

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_ef8de92414f2d9c8
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ef8de92414f2d9c8:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_4a611090161f3fae
L_OBJC_METH_VAR_NAME_4a611090161f3fae:
	.asciz	"methodIdWithParam:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_4a611090161f3fae
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_4a611090161f3fae:
	.quad	L_OBJC_METH_VAR_NAME_4a611090161f3fae

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_4a611090161f3fae
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_4a611090161f3fae:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2837f061c311eb14
L_OBJC_METH_VAR_NAME_2837f061c311eb14:
	.asciz	"copyWithZone:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2837f061c311eb14
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_2837f061c311eb14:
	.quad	L_OBJC_METH_VAR_NAME_2837f061c311eb14

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2837f061c311eb14
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2837f061c311eb14:
	.asciz	"\000\000\000\000@\000\000"

	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1),8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 2)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 2),8,3
	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].25:
	.quad	l_anon.[ID].21
	.asciz	"5\000\000\000\000\000\000\000O\000\000\000\001\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_af8966656b8b2b6c
L_OBJC_METH_VAR_NAME_af8966656b8b2b6c:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_af8966656b8b2b6c
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_af8966656b8b2b6c:
	.quad	L_OBJC_METH_VAR_NAME_af8966656b8b2b6c

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_af8966656b8b2b6c
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_af8966656b8b2b6c:
	.asciz	"\000\000\000\000@\000\000"

	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0),8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0),8,3
	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].26:
	.quad	l_anon.[ID].21
	.asciz	"5\000\000\000\000\000\000\000x\000\000\000\001\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6edddcebbded8f32
L_OBJC_METH_VAR_NAME_6edddcebbded8f32:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_6edddcebbded8f32
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_6edddcebbded8f32:
	.quad	L_OBJC_METH_VAR_NAME_6edddcebbded8f32

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_6edddcebbded8f32
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_6edddcebbded8f32:
	.asciz	"\000\000\000\000@\000\000"

.zerofill __DATA,__bss,__MergedGlobals,48,3
.subsections_via_symbols
