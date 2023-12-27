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
	adrp	x0, l_anon.[ID].24@PAGE
Lloh43:
	add	x0, x0, l_anon.[ID].24@PAGEOFF
	mov	w1, #8
	bl	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	cbz	x0, LBB4_4
	mov	x1, x0
	add	x0, sp, #8
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_protocol::GENERATED_ID, 0)
LBB4_4:
Lloh44:
	adrp	x0, l_anon.[ID].25@PAGE
Lloh45:
	add	x0, x0, l_anon.[ID].25@PAGEOFF
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
	adrp	x0, l_anon.[ID].19@PAGE
Lloh55:
	add	x0, x0, l_anon.[ID].19@PAGEOFF
Lloh56:
	adrp	x2, l_anon.[ID].21@PAGE
Lloh57:
	add	x2, x2, l_anon.[ID].21@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB4_8:
Lloh58:
	adrp	x0, l_anon.[ID].17@PAGE
Lloh59:
	add	x0, x0, l_anon.[ID].17@PAGEOFF
Lloh60:
	adrp	x2, l_anon.[ID].23@PAGE
Lloh61:
	add	x2, x2, l_anon.[ID].23@PAGEOFF
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
	.loh AdrpAdd	Lloh56, Lloh57
	.loh AdrpAdd	Lloh54, Lloh55
	.loh AdrpAdd	Lloh60, Lloh61
	.loh AdrpAdd	Lloh58, Lloh59

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
Lloh62:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh63:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh64:
	ldr	x2, [x8]
Lloh65:
	adrp	x0, l_anon.[ID].18@PAGE
Lloh66:
	add	x0, x0, l_anon.[ID].18@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::runtime::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB5_6
	str	x0, [sp, #24]
Lloh67:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGE
Lloh68:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGEOFF]
Lloh69:
	ldr	x1, [x8]
Lloh70:
	adrp	x19, l_anon.[ID].3@PAGE
Lloh71:
	add	x19, x19, l_anon.[ID].3@PAGEOFF
Lloh72:
	adrp	x4, l_anon.[ID].5@PAGE
Lloh73:
	add	x4, x4, l_anon.[ID].5@PAGEOFF
Lloh74:
	adrp	x5, SYM(objc2[CRATE_ID]::__macro_helpers::declared_ivars::dealloc::<test_declare_class[CRATE_ID]::DropIvars>, 0)@PAGE
Lloh75:
	add	x5, x5, SYM(objc2[CRATE_ID]::__macro_helpers::declared_ivars::dealloc::<test_declare_class[CRATE_ID]::DropIvars>, 0)@PAGEOFF
	add	x0, sp, #24
	mov	x2, x19
	mov	x3, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x8, [sp, #24]
	str	x8, [sp, #8]
Lloh76:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh77:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh78:
	ldr	x1, [x8]
Lloh79:
	adrp	x4, l_anon.[ID].4@PAGE
Lloh80:
	add	x4, x4, l_anon.[ID].4@PAGEOFF
Lloh81:
	adrp	x5, _init_drop_ivars@PAGE
Lloh82:
	add	x5, x5, _init_drop_ivars@PAGEOFF
	add	x0, sp, #8
	mov	x2, x19
	mov	x3, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x8, [sp, #8]
	str	x8, [sp, #16]
	mov	w8, #16
Lloh83:
	adrp	x9, l_anon.[ID].15@PAGE
Lloh84:
	add	x9, x9, l_anon.[ID].15@PAGEOFF
	stp	x8, x9, [sp, #32]
	mov	w8, #27
	strb	w8, [sp, #24]
Lloh85:
	adrp	x20, l_anon.[ID].11@PAGE
Lloh86:
	add	x20, x20, l_anon.[ID].11@PAGEOFF
	add	x0, sp, #16
	add	x5, sp, #24
	mov	x1, x20
	mov	w2, #5
	mov	w3, #16
	mov	w4, #3
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Lloh87:
	adrp	x1, l_anon.[ID].12@PAGE
Lloh88:
	add	x1, x1, l_anon.[ID].12@PAGEOFF
Lloh89:
	adrp	x5, l_anon.[ID].13@PAGE
Lloh90:
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
Lloh91:
	adrp	x1, l_anon.[ID].12@PAGE
Lloh92:
	add	x1, x1, l_anon.[ID].12@PAGEOFF
	mov	x0, x19
	mov	w2, #9
	bl	SYM(objc2::runtime::AnyClass::instance_variable::GENERATED_ID, 0)
	cbz	x0, LBB5_8
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
LBB5_5:
Lloh96:
	adrp	x0, l_anon.[ID].19@PAGE
Lloh97:
	add	x0, x0, l_anon.[ID].19@PAGEOFF
Lloh98:
	adrp	x2, l_anon.[ID].21@PAGE
Lloh99:
	add	x2, x2, l_anon.[ID].21@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB5_6:
Lloh100:
	adrp	x0, l_anon.[ID].18@PAGE
Lloh101:
	add	x0, x0, l_anon.[ID].18@PAGEOFF
Lloh102:
	adrp	x2, l_anon.[ID].27@PAGE
Lloh103:
	add	x2, x2, l_anon.[ID].27@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
LBB5_7:
	bl	SYM(objc2::__macro_helpers::declared_ivars::register_with_ivars::get_ivar_failed::GENERATED_ID, 0)
LBB5_8:
	bl	SYM(objc2::__macro_helpers::declared_ivars::register_with_ivars::get_drop_flag_failed::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh65, Lloh66
	.loh AdrpLdrGotLdr	Lloh62, Lloh63, Lloh64
	.loh AdrpAdd	Lloh89, Lloh90
	.loh AdrpAdd	Lloh87, Lloh88
	.loh AdrpAdd	Lloh85, Lloh86
	.loh AdrpAdd	Lloh83, Lloh84
	.loh AdrpAdd	Lloh81, Lloh82
	.loh AdrpAdd	Lloh79, Lloh80
	.loh AdrpLdrGotLdr	Lloh76, Lloh77, Lloh78
	.loh AdrpAdd	Lloh74, Lloh75
	.loh AdrpAdd	Lloh72, Lloh73
	.loh AdrpAdd	Lloh70, Lloh71
	.loh AdrpLdrGotLdr	Lloh67, Lloh68, Lloh69
	.loh AdrpAdd	Lloh91, Lloh92
	.loh AdrpAdrp	Lloh94, Lloh95
	.loh AdrpAdrp	Lloh93, Lloh94
	.loh AdrpAdd	Lloh98, Lloh99
	.loh AdrpAdd	Lloh96, Lloh97
	.loh AdrpAdd	Lloh102, Lloh103
	.loh AdrpAdd	Lloh100, Lloh101

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
Lloh104:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh105:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh106:
	ldr	x2, [x8]
Lloh107:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh108:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::runtime::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB6_5
	str	x0, [sp, #8]
Lloh109:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh110:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh111:
	ldr	x1, [x8]
Lloh112:
	adrp	x2, l_anon.[ID].3@PAGE
Lloh113:
	add	x2, x2, l_anon.[ID].3@PAGEOFF
Lloh114:
	adrp	x4, l_anon.[ID].4@PAGE
Lloh115:
	add	x4, x4, l_anon.[ID].4@PAGEOFF
Lloh116:
	adrp	x5, _init_forgetable_ivars@PAGE
Lloh117:
	add	x5, x5, _init_forgetable_ivars@PAGEOFF
	add	x0, sp, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x8, [sp, #8]
	str	x8, [sp, #16]
	mov	w8, #8
Lloh118:
	adrp	x9, l_anon.[ID].14@PAGE
Lloh119:
	add	x9, x9, l_anon.[ID].14@PAGEOFF
	stp	x8, x9, [sp, #32]
	mov	w8, #27
	strb	w8, [sp, #24]
Lloh120:
	adrp	x20, l_anon.[ID].11@PAGE
Lloh121:
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
Lloh122:
	adrp	x8, __MergedGlobals@PAGE+16
	str	x19, [x8, __MergedGlobals@PAGEOFF+16]
Lloh123:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGE
	str	x0, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGEOFF]
	ldp	x29, x30, [sp, #80]
	ldp	x20, x19, [sp, #64]
	add	sp, sp, #96
	ret
LBB6_4:
Lloh124:
	adrp	x0, l_anon.[ID].19@PAGE
Lloh125:
	add	x0, x0, l_anon.[ID].19@PAGEOFF
Lloh126:
	adrp	x2, l_anon.[ID].21@PAGE
Lloh127:
	add	x2, x2, l_anon.[ID].21@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB6_5:
Lloh128:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh129:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
Lloh130:
	adrp	x2, l_anon.[ID].26@PAGE
Lloh131:
	add	x2, x2, l_anon.[ID].26@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
LBB6_6:
	bl	SYM(objc2::__macro_helpers::declared_ivars::register_with_ivars::get_ivar_failed::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh107, Lloh108
	.loh AdrpLdrGotLdr	Lloh104, Lloh105, Lloh106
	.loh AdrpAdd	Lloh120, Lloh121
	.loh AdrpAdd	Lloh118, Lloh119
	.loh AdrpAdd	Lloh116, Lloh117
	.loh AdrpAdd	Lloh114, Lloh115
	.loh AdrpAdd	Lloh112, Lloh113
	.loh AdrpLdrGotLdr	Lloh109, Lloh110, Lloh111
	.loh AdrpAdrp	Lloh122, Lloh123
	.loh AdrpAdd	Lloh126, Lloh127
	.loh AdrpAdd	Lloh124, Lloh125
	.loh AdrpAdd	Lloh130, Lloh131
	.loh AdrpAdd	Lloh128, Lloh129

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
Lloh132:
	adrp	x8, __MergedGlobals@PAGE+24
Lloh133:
	add	x8, x8, __MergedGlobals@PAGEOFF+24
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB10_2
Lloh134:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh135:
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
Lloh136:
	adrp	x0, __MergedGlobals@PAGE+24
Lloh137:
	add	x0, x0, __MergedGlobals@PAGEOFF+24
Lloh138:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh139:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
Lloh140:
	adrp	x4, l_anon.[ID].26@PAGE
Lloh141:
	add	x4, x4, l_anon.[ID].26@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh142:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh143:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
	ret
	.loh AdrpAdd	Lloh132, Lloh133
	.loh AdrpLdr	Lloh134, Lloh135
	.loh AdrpLdr	Lloh142, Lloh143
	.loh AdrpAdd	Lloh140, Lloh141
	.loh AdrpAdd	Lloh138, Lloh139
	.loh AdrpAdd	Lloh136, Lloh137

	.globl	_access_forgetable_ivars
	.p2align	2
_access_forgetable_ivars:
Lloh144:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGE
Lloh145:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGEOFF]
	add	x8, x0, x8
	ldr	w1, [x8]
	ldrb	w0, [x8, #4]
	ret
	.loh AdrpLdr	Lloh144, Lloh145

	.globl	SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0):
	; InlineAsm Start
	; InlineAsm End
	ret

	.globl	_access_drop_ivars_class
	.p2align	2
_access_drop_ivars_class:
Lloh146:
	adrp	x8, __MergedGlobals@PAGE+40
Lloh147:
	add	x8, x8, __MergedGlobals@PAGEOFF+40
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB13_2
Lloh148:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh149:
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
Lloh150:
	adrp	x0, __MergedGlobals@PAGE+40
Lloh151:
	add	x0, x0, __MergedGlobals@PAGEOFF+40
Lloh152:
	adrp	x3, l_anon.[ID].1@PAGE
Lloh153:
	add	x3, x3, l_anon.[ID].1@PAGEOFF
Lloh154:
	adrp	x4, l_anon.[ID].27@PAGE
Lloh155:
	add	x4, x4, l_anon.[ID].27@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh156:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh157:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+32]
	ret
	.loh AdrpAdd	Lloh146, Lloh147
	.loh AdrpLdr	Lloh148, Lloh149
	.loh AdrpLdr	Lloh156, Lloh157
	.loh AdrpAdd	Lloh154, Lloh155
	.loh AdrpAdd	Lloh152, Lloh153
	.loh AdrpAdd	Lloh150, Lloh151

	.globl	_access_drop_ivars
	.p2align	2
_access_drop_ivars:
Lloh158:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
Lloh159:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGEOFF]
	add	x8, x0, x8
	ldp	x0, x1, [x8]
	ret
	.loh AdrpLdr	Lloh158, Lloh159

	.globl	SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh160:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh161:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB15_2
Lloh162:
	adrp	x8, __MergedGlobals@PAGE
Lloh163:
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
Lloh164:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh165:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh166:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh167:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh168:
	adrp	x4, l_anon.[ID].23@PAGE
Lloh169:
	add	x4, x4, l_anon.[ID].23@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
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

	.globl	_get_class
	.p2align	2
_get_class:
Lloh172:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh173:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB16_2
Lloh174:
	adrp	x8, __MergedGlobals@PAGE
Lloh175:
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
Lloh176:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh177:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh178:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh179:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh180:
	adrp	x4, l_anon.[ID].23@PAGE
Lloh181:
	add	x4, x4, l_anon.[ID].23@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh182:
	adrp	x8, __MergedGlobals@PAGE
Lloh183:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh172, Lloh173
	.loh AdrpLdr	Lloh174, Lloh175
	.loh AdrpLdr	Lloh182, Lloh183
	.loh AdrpAdd	Lloh180, Lloh181
	.loh AdrpAdd	Lloh178, Lloh179
	.loh AdrpAdd	Lloh176, Lloh177

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
Lloh184:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh185:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB19_2
LBB19_1:
Lloh186:
	adrp	x8, __MergedGlobals@PAGE
Lloh187:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
Lloh188:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh189:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh190:
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
Lloh191:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh192:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh193:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh194:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh195:
	adrp	x4, l_anon.[ID].23@PAGE
Lloh196:
	add	x4, x4, l_anon.[ID].23@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	b	LBB19_1
	.loh AdrpAdd	Lloh184, Lloh185
	.loh AdrpLdrGotLdr	Lloh188, Lloh189, Lloh190
	.loh AdrpAdrp	Lloh186, Lloh188
	.loh AdrpLdr	Lloh186, Lloh187
	.loh AdrpAdd	Lloh195, Lloh196
	.loh AdrpAdd	Lloh193, Lloh194
	.loh AdrpAdd	Lloh191, Lloh192

	.globl	_method_id_with_param
	.p2align	2
_method_id_with_param:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x20, x2
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	x19, x0
	tbz	w20, #0, LBB20_2
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	x20, x0
	mov	x0, x19
	bl	_objc_release
	mov	x19, x20
LBB20_2:
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_autoreleaseReturnValue

	.globl	_copyWithZone
	.p2align	2
_copyWithZone:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh197:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh198:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB21_2
LBB21_1:
Lloh199:
	adrp	x8, __MergedGlobals@PAGE
Lloh200:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
Lloh201:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh202:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh203:
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
Lloh204:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh205:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh206:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh207:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh208:
	adrp	x4, l_anon.[ID].23@PAGE
Lloh209:
	add	x4, x4, l_anon.[ID].23@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	b	LBB21_1
	.loh AdrpAdd	Lloh197, Lloh198
	.loh AdrpLdrGotLdr	Lloh201, Lloh202, Lloh203
	.loh AdrpAdrp	Lloh199, Lloh201
	.loh AdrpLdr	Lloh199, Lloh200
	.loh AdrpAdd	Lloh208, Lloh209
	.loh AdrpAdd	Lloh206, Lloh207
	.loh AdrpAdd	Lloh204, Lloh205

	.globl	SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh210:
	adrp	x8, __MergedGlobals@PAGE+24
Lloh211:
	add	x8, x8, __MergedGlobals@PAGEOFF+24
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB22_2
Lloh212:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh213:
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
Lloh214:
	adrp	x0, __MergedGlobals@PAGE+24
Lloh215:
	add	x0, x0, __MergedGlobals@PAGEOFF+24
Lloh216:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh217:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
Lloh218:
	adrp	x4, l_anon.[ID].26@PAGE
Lloh219:
	add	x4, x4, l_anon.[ID].26@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh220:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh221:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
	ret
	.loh AdrpAdd	Lloh210, Lloh211
	.loh AdrpLdr	Lloh212, Lloh213
	.loh AdrpLdr	Lloh220, Lloh221
	.loh AdrpAdd	Lloh218, Lloh219
	.loh AdrpAdd	Lloh216, Lloh217
	.loh AdrpAdd	Lloh214, Lloh215

	.globl	_init_forgetable_ivars
	.p2align	2
_init_forgetable_ivars:
	cbz	x0, LBB23_2
Lloh222:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGE
Lloh223:
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
Lloh224:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_af8966656b8b2b6c@PAGE
Lloh225:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_af8966656b8b2b6c@PAGEOFF]
Lloh226:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh227:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh228:
	ldr	x8, [x8]
	stp	x0, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpLdr	Lloh222, Lloh223
	.loh AdrpLdrGotLdr	Lloh226, Lloh227, Lloh228
	.loh AdrpAdrp	Lloh224, Lloh226
	.loh AdrpLdr	Lloh224, Lloh225

	.globl	SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh229:
	adrp	x8, __MergedGlobals@PAGE+40
Lloh230:
	add	x8, x8, __MergedGlobals@PAGEOFF+40
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB24_2
Lloh231:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh232:
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
Lloh233:
	adrp	x0, __MergedGlobals@PAGE+40
Lloh234:
	add	x0, x0, __MergedGlobals@PAGEOFF+40
Lloh235:
	adrp	x3, l_anon.[ID].1@PAGE
Lloh236:
	add	x3, x3, l_anon.[ID].1@PAGEOFF
Lloh237:
	adrp	x4, l_anon.[ID].27@PAGE
Lloh238:
	add	x4, x4, l_anon.[ID].27@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh239:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh240:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+32]
	ret
	.loh AdrpAdd	Lloh229, Lloh230
	.loh AdrpLdr	Lloh231, Lloh232
	.loh AdrpLdr	Lloh239, Lloh240
	.loh AdrpAdd	Lloh237, Lloh238
	.loh AdrpAdd	Lloh235, Lloh236
	.loh AdrpAdd	Lloh233, Lloh234

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
	mov	x21, x0
	adrp	x22, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGE
	cbz	x19, LBB25_2
Lloh241:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
Lloh242:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGEOFF]
	add	x8, x19, x8
	stp	x20, x21, [x8]
	ldr	x8, [x22, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGEOFF]
	mov	w9, #15
	strb	w9, [x19, x8]
	b	LBB25_3
LBB25_2:
	mov	x0, x20
	bl	_objc_release
	mov	x0, x21
	bl	_objc_release
LBB25_3:
Lloh243:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_6edddcebbded8f32@PAGE
Lloh244:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_6edddcebbded8f32@PAGEOFF]
Lloh245:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh246:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh247:
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
	.loh AdrpLdr	Lloh241, Lloh242
	.loh AdrpLdrGotLdr	Lloh245, Lloh246, Lloh247
	.loh AdrpAdrp	Lloh243, Lloh245
	.loh AdrpLdr	Lloh243, Lloh244

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
	.ascii	"called `Option::unwrap()` on a `None` value"

l_anon.[ID].20:
	.ascii	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].21:
	.quad	l_anon.[ID].20
	.asciz	"p\000\000\000\000\000\000\000\225\000\000\0002\000\000"

	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2),8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 1)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 1),8,3
	.section	__TEXT,__const
l_anon.[ID].22:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].23:
	.quad	l_anon.[ID].22
	.asciz	"5\000\000\000\000\000\000\000\016\000\000\000\001\000\000"

	.section	__TEXT,__literal8,8byte_literals
l_anon.[ID].24:
	.ascii	"NSObject"

	.section	__TEXT,__const
l_anon.[ID].25:
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
l_anon.[ID].26:
	.quad	l_anon.[ID].22
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
l_anon.[ID].27:
	.quad	l_anon.[ID].22
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
