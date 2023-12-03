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
	adrp	x0, l_anon.[ID].22@PAGE
Lloh11:
	add	x0, x0, l_anon.[ID].22@PAGEOFF
	mov	w1, #7
	bl	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
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
	bl	SYM(objc2::declare::ClassBuilder::add_class_method_inner::GENERATED_ID, 0)
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
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
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
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
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
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
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
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh42:
	adrp	x0, l_anon.[ID].29@PAGE
Lloh43:
	add	x0, x0, l_anon.[ID].29@PAGEOFF
	mov	w1, #8
	bl	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	cbz	x0, LBB4_4
	mov	x1, x0
	add	x0, sp, #8
	bl	SYM(objc2::declare::ClassBuilder::add_protocol::GENERATED_ID, 0)
LBB4_4:
Lloh44:
	adrp	x0, l_anon.[ID].30@PAGE
Lloh45:
	add	x0, x0, l_anon.[ID].30@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	cbz	x0, LBB4_6
	mov	x1, x0
	add	x0, sp, #8
	bl	SYM(objc2::declare::ClassBuilder::add_protocol::GENERATED_ID, 0)
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
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x0, [sp, #8]
	bl	SYM(objc2::declare::ClassBuilder::register::GENERATED_ID, 0)
Lloh54:
	adrp	x8, __MergedGlobals@PAGE
	str	x0, [x8, __MergedGlobals@PAGEOFF]
Lloh55:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGE
	str	xzr, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)@PAGEOFF]
Lloh56:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 1)@PAGE
	str	xzr, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 1)@PAGEOFF]
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	ldp	x22, x21, [sp, #16]
	add	sp, sp, #64
	ret
LBB4_7:
Lloh57:
	adrp	x0, l_anon.[ID].24@PAGE
Lloh58:
	add	x0, x0, l_anon.[ID].24@PAGEOFF
Lloh59:
	adrp	x2, l_anon.[ID].26@PAGE
Lloh60:
	add	x2, x2, l_anon.[ID].26@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB4_8:
Lloh61:
	adrp	x0, l_anon.[ID].22@PAGE
Lloh62:
	add	x0, x0, l_anon.[ID].22@PAGEOFF
Lloh63:
	adrp	x2, l_anon.[ID].28@PAGE
Lloh64:
	add	x2, x2, l_anon.[ID].28@PAGEOFF
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
	.loh AdrpAdrp	Lloh55, Lloh56
	.loh AdrpAdrp	Lloh54, Lloh55
	.loh AdrpAdd	Lloh52, Lloh53
	.loh AdrpAdd	Lloh50, Lloh51
	.loh AdrpAdd	Lloh48, Lloh49
	.loh AdrpLdr	Lloh46, Lloh47
	.loh AdrpAdd	Lloh59, Lloh60
	.loh AdrpAdd	Lloh57, Lloh58
	.loh AdrpAdd	Lloh63, Lloh64
	.loh AdrpAdd	Lloh61, Lloh62

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
Lloh65:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh66:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh67:
	ldr	x2, [x8]
Lloh68:
	adrp	x0, l_anon.[ID].23@PAGE
Lloh69:
	add	x0, x0, l_anon.[ID].23@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB5_6
	str	x0, [sp, #24]
Lloh70:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGE
Lloh71:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGEOFF]
Lloh72:
	ldr	x1, [x8]
Lloh73:
	adrp	x19, l_anon.[ID].3@PAGE
Lloh74:
	add	x19, x19, l_anon.[ID].3@PAGEOFF
Lloh75:
	adrp	x4, l_anon.[ID].5@PAGE
Lloh76:
	add	x4, x4, l_anon.[ID].5@PAGEOFF
Lloh77:
	adrp	x5, SYM(objc2[CRATE_ID]::__macro_helpers::declared_ivars::dealloc::<test_declare_class[CRATE_ID]::DropIvars>, 0)@PAGE
Lloh78:
	add	x5, x5, SYM(objc2[CRATE_ID]::__macro_helpers::declared_ivars::dealloc::<test_declare_class[CRATE_ID]::DropIvars>, 0)@PAGEOFF
	add	x0, sp, #24
	mov	x2, x19
	mov	x3, #0
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x8, [sp, #24]
	str	x8, [sp, #8]
Lloh79:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh80:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh81:
	ldr	x1, [x8]
Lloh82:
	adrp	x4, l_anon.[ID].4@PAGE
Lloh83:
	add	x4, x4, l_anon.[ID].4@PAGEOFF
Lloh84:
	adrp	x5, _init_drop_ivars@PAGE
Lloh85:
	add	x5, x5, _init_drop_ivars@PAGEOFF
	add	x0, sp, #8
	mov	x2, x19
	mov	x3, #0
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x8, [sp, #8]
	str	x8, [sp, #16]
	mov	w8, #16
Lloh86:
	adrp	x9, l_anon.[ID].15@PAGE
Lloh87:
	add	x9, x9, l_anon.[ID].15@PAGEOFF
	stp	x8, x9, [sp, #32]
	mov	w8, #27
	strb	w8, [sp, #24]
Lloh88:
	adrp	x20, l_anon.[ID].11@PAGE
Lloh89:
	add	x20, x20, l_anon.[ID].11@PAGEOFF
	add	x0, sp, #16
	add	x5, sp, #24
	mov	x1, x20
	mov	w2, #5
	mov	w3, #16
	mov	w4, #3
	bl	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Lloh90:
	adrp	x1, l_anon.[ID].12@PAGE
Lloh91:
	add	x1, x1, l_anon.[ID].12@PAGEOFF
Lloh92:
	adrp	x5, l_anon.[ID].13@PAGE
Lloh93:
	add	x5, x5, l_anon.[ID].13@PAGEOFF
	add	x0, sp, #16
	mov	w2, #9
	mov	w3, #1
	mov	w4, #0
	bl	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	ldr	x0, [sp, #16]
	bl	SYM(objc2::declare::ClassBuilder::register::GENERATED_ID, 0)
	mov	x19, x0
	mov	x1, x20
	mov	w2, #5
	bl	SYM(objc2::runtime::AnyClass::instance_variable::GENERATED_ID, 0)
	cbz	x0, LBB5_7
	bl	_ivar_getOffset
	mov	x20, x0
Lloh94:
	adrp	x1, l_anon.[ID].12@PAGE
Lloh95:
	add	x1, x1, l_anon.[ID].12@PAGEOFF
	mov	x0, x19
	mov	w2, #9
	bl	SYM(objc2::runtime::AnyClass::instance_variable::GENERATED_ID, 0)
	cbz	x0, LBB5_8
	bl	_ivar_getOffset
Lloh96:
	adrp	x8, __MergedGlobals@PAGE+32
	str	x19, [x8, __MergedGlobals@PAGEOFF+32]
Lloh97:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
	str	x20, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGEOFF]
Lloh98:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGE
	str	x0, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)@PAGEOFF]
	ldp	x29, x30, [sp, #80]
	ldp	x20, x19, [sp, #64]
	add	sp, sp, #96
	ret
LBB5_5:
Lloh99:
	adrp	x0, l_anon.[ID].24@PAGE
Lloh100:
	add	x0, x0, l_anon.[ID].24@PAGEOFF
Lloh101:
	adrp	x2, l_anon.[ID].26@PAGE
Lloh102:
	add	x2, x2, l_anon.[ID].26@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB5_6:
Lloh103:
	adrp	x0, l_anon.[ID].23@PAGE
Lloh104:
	add	x0, x0, l_anon.[ID].23@PAGEOFF
Lloh105:
	adrp	x2, l_anon.[ID].32@PAGE
Lloh106:
	add	x2, x2, l_anon.[ID].32@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
LBB5_7:
Lloh107:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh108:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
Lloh109:
	adrp	x2, l_anon.[ID].18@PAGE
Lloh110:
	add	x2, x2, l_anon.[ID].18@PAGEOFF
	mov	w1, #59
	bl	SYM(core::option::expect_failed::GENERATED_ID, 0)
LBB5_8:
Lloh111:
	adrp	x0, l_anon.[ID].19@PAGE
Lloh112:
	add	x0, x0, l_anon.[ID].19@PAGEOFF
Lloh113:
	adrp	x2, l_anon.[ID].20@PAGE
Lloh114:
	add	x2, x2, l_anon.[ID].20@PAGEOFF
	mov	w1, #69
	bl	SYM(core::option::expect_failed::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh68, Lloh69
	.loh AdrpLdrGotLdr	Lloh65, Lloh66, Lloh67
	.loh AdrpAdd	Lloh92, Lloh93
	.loh AdrpAdd	Lloh90, Lloh91
	.loh AdrpAdd	Lloh88, Lloh89
	.loh AdrpAdd	Lloh86, Lloh87
	.loh AdrpAdd	Lloh84, Lloh85
	.loh AdrpAdd	Lloh82, Lloh83
	.loh AdrpLdrGotLdr	Lloh79, Lloh80, Lloh81
	.loh AdrpAdd	Lloh77, Lloh78
	.loh AdrpAdd	Lloh75, Lloh76
	.loh AdrpAdd	Lloh73, Lloh74
	.loh AdrpLdrGotLdr	Lloh70, Lloh71, Lloh72
	.loh AdrpAdd	Lloh94, Lloh95
	.loh AdrpAdrp	Lloh97, Lloh98
	.loh AdrpAdrp	Lloh96, Lloh97
	.loh AdrpAdd	Lloh101, Lloh102
	.loh AdrpAdd	Lloh99, Lloh100
	.loh AdrpAdd	Lloh105, Lloh106
	.loh AdrpAdd	Lloh103, Lloh104
	.loh AdrpAdd	Lloh109, Lloh110
	.loh AdrpAdd	Lloh107, Lloh108
	.loh AdrpAdd	Lloh113, Lloh114
	.loh AdrpAdd	Lloh111, Lloh112

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
Lloh115:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh116:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh117:
	ldr	x2, [x8]
Lloh118:
	adrp	x0, l_anon.[ID].21@PAGE
Lloh119:
	add	x0, x0, l_anon.[ID].21@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB6_5
	str	x0, [sp, #8]
Lloh120:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh121:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh122:
	ldr	x1, [x8]
Lloh123:
	adrp	x2, l_anon.[ID].3@PAGE
Lloh124:
	add	x2, x2, l_anon.[ID].3@PAGEOFF
Lloh125:
	adrp	x4, l_anon.[ID].4@PAGE
Lloh126:
	add	x4, x4, l_anon.[ID].4@PAGEOFF
Lloh127:
	adrp	x5, _init_forgetable_ivars@PAGE
Lloh128:
	add	x5, x5, _init_forgetable_ivars@PAGEOFF
	add	x0, sp, #8
	mov	x3, #0
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x8, [sp, #8]
	str	x8, [sp, #16]
	mov	w8, #8
Lloh129:
	adrp	x9, l_anon.[ID].14@PAGE
Lloh130:
	add	x9, x9, l_anon.[ID].14@PAGEOFF
	stp	x8, x9, [sp, #32]
	mov	w8, #27
	strb	w8, [sp, #24]
Lloh131:
	adrp	x20, l_anon.[ID].11@PAGE
Lloh132:
	add	x20, x20, l_anon.[ID].11@PAGEOFF
	add	x0, sp, #16
	add	x5, sp, #24
	mov	x1, x20
	mov	w2, #5
	mov	w3, #8
	mov	w4, #2
	bl	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	ldr	x0, [sp, #16]
	bl	SYM(objc2::declare::ClassBuilder::register::GENERATED_ID, 0)
	mov	x19, x0
	mov	x1, x20
	mov	w2, #5
	bl	SYM(objc2::runtime::AnyClass::instance_variable::GENERATED_ID, 0)
	cbz	x0, LBB6_6
	bl	_ivar_getOffset
Lloh133:
	adrp	x8, __MergedGlobals@PAGE+16
	str	x19, [x8, __MergedGlobals@PAGEOFF+16]
Lloh134:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2)@PAGE
	str	x0, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2)@PAGEOFF]
Lloh135:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 2)@PAGE
	str	xzr, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 2)@PAGEOFF]
	ldp	x29, x30, [sp, #80]
	ldp	x20, x19, [sp, #64]
	add	sp, sp, #96
	ret
LBB6_4:
Lloh136:
	adrp	x0, l_anon.[ID].24@PAGE
Lloh137:
	add	x0, x0, l_anon.[ID].24@PAGEOFF
Lloh138:
	adrp	x2, l_anon.[ID].26@PAGE
Lloh139:
	add	x2, x2, l_anon.[ID].26@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB6_5:
Lloh140:
	adrp	x0, l_anon.[ID].21@PAGE
Lloh141:
	add	x0, x0, l_anon.[ID].21@PAGEOFF
Lloh142:
	adrp	x2, l_anon.[ID].31@PAGE
Lloh143:
	add	x2, x2, l_anon.[ID].31@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
LBB6_6:
Lloh144:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh145:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
Lloh146:
	adrp	x2, l_anon.[ID].18@PAGE
Lloh147:
	add	x2, x2, l_anon.[ID].18@PAGEOFF
	mov	w1, #59
	bl	SYM(core::option::expect_failed::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh118, Lloh119
	.loh AdrpLdrGotLdr	Lloh115, Lloh116, Lloh117
	.loh AdrpAdd	Lloh131, Lloh132
	.loh AdrpAdd	Lloh129, Lloh130
	.loh AdrpAdd	Lloh127, Lloh128
	.loh AdrpAdd	Lloh125, Lloh126
	.loh AdrpAdd	Lloh123, Lloh124
	.loh AdrpLdrGotLdr	Lloh120, Lloh121, Lloh122
	.loh AdrpAdrp	Lloh134, Lloh135
	.loh AdrpAdrp	Lloh133, Lloh134
	.loh AdrpAdd	Lloh138, Lloh139
	.loh AdrpAdd	Lloh136, Lloh137
	.loh AdrpAdd	Lloh142, Lloh143
	.loh AdrpAdd	Lloh140, Lloh141
	.loh AdrpAdd	Lloh146, Lloh147
	.loh AdrpAdd	Lloh144, Lloh145

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
Lloh148:
	adrp	x8, __MergedGlobals@PAGE+24
Lloh149:
	add	x8, x8, __MergedGlobals@PAGEOFF+24
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB10_2
Lloh150:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh151:
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
Lloh152:
	adrp	x0, __MergedGlobals@PAGE+24
Lloh153:
	add	x0, x0, __MergedGlobals@PAGEOFF+24
Lloh154:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh155:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
Lloh156:
	adrp	x4, l_anon.[ID].31@PAGE
Lloh157:
	add	x4, x4, l_anon.[ID].31@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh158:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh159:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
	ret
	.loh AdrpAdd	Lloh148, Lloh149
	.loh AdrpLdr	Lloh150, Lloh151
	.loh AdrpLdr	Lloh158, Lloh159
	.loh AdrpAdd	Lloh156, Lloh157
	.loh AdrpAdd	Lloh154, Lloh155
	.loh AdrpAdd	Lloh152, Lloh153

	.globl	_access_forgetable_ivars
	.p2align	2
_access_forgetable_ivars:
Lloh160:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2)@PAGE
Lloh161:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2)@PAGEOFF]
	add	x8, x0, x8
	ldr	w1, [x8]
	ldrb	w0, [x8, #4]
	ret
	.loh AdrpLdr	Lloh160, Lloh161

	.globl	SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0):
	; InlineAsm Start
	; InlineAsm End
	ret

	.globl	_access_drop_ivars_class
	.p2align	2
_access_drop_ivars_class:
Lloh162:
	adrp	x8, __MergedGlobals@PAGE+40
Lloh163:
	add	x8, x8, __MergedGlobals@PAGEOFF+40
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB13_2
Lloh164:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh165:
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
Lloh166:
	adrp	x0, __MergedGlobals@PAGE+40
Lloh167:
	add	x0, x0, __MergedGlobals@PAGEOFF+40
Lloh168:
	adrp	x3, l_anon.[ID].1@PAGE
Lloh169:
	add	x3, x3, l_anon.[ID].1@PAGEOFF
Lloh170:
	adrp	x4, l_anon.[ID].32@PAGE
Lloh171:
	add	x4, x4, l_anon.[ID].32@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh172:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh173:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+32]
	ret
	.loh AdrpAdd	Lloh162, Lloh163
	.loh AdrpLdr	Lloh164, Lloh165
	.loh AdrpLdr	Lloh172, Lloh173
	.loh AdrpAdd	Lloh170, Lloh171
	.loh AdrpAdd	Lloh168, Lloh169
	.loh AdrpAdd	Lloh166, Lloh167

	.globl	_access_drop_ivars
	.p2align	2
_access_drop_ivars:
Lloh174:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
Lloh175:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGEOFF]
	add	x8, x0, x8
	ldp	x0, x1, [x8]
	ret
	.loh AdrpLdr	Lloh174, Lloh175

	.globl	SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh176:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh177:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB15_2
Lloh178:
	adrp	x8, __MergedGlobals@PAGE
Lloh179:
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
Lloh180:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh181:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh182:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh183:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh184:
	adrp	x4, l_anon.[ID].28@PAGE
Lloh185:
	add	x4, x4, l_anon.[ID].28@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh186:
	adrp	x8, __MergedGlobals@PAGE
Lloh187:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh176, Lloh177
	.loh AdrpLdr	Lloh178, Lloh179
	.loh AdrpLdr	Lloh186, Lloh187
	.loh AdrpAdd	Lloh184, Lloh185
	.loh AdrpAdd	Lloh182, Lloh183
	.loh AdrpAdd	Lloh180, Lloh181

	.globl	_get_class
	.p2align	2
_get_class:
Lloh188:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh189:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB16_2
Lloh190:
	adrp	x8, __MergedGlobals@PAGE
Lloh191:
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
Lloh192:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh193:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh194:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh195:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh196:
	adrp	x4, l_anon.[ID].28@PAGE
Lloh197:
	add	x4, x4, l_anon.[ID].28@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh198:
	adrp	x8, __MergedGlobals@PAGE
Lloh199:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh188, Lloh189
	.loh AdrpLdr	Lloh190, Lloh191
	.loh AdrpLdr	Lloh198, Lloh199
	.loh AdrpAdd	Lloh196, Lloh197
	.loh AdrpAdd	Lloh194, Lloh195
	.loh AdrpAdd	Lloh192, Lloh193

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
Lloh200:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh201:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB19_2
LBB19_1:
Lloh202:
	adrp	x8, __MergedGlobals@PAGE
Lloh203:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
Lloh204:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh205:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh206:
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
Lloh207:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh208:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh209:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh210:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh211:
	adrp	x4, l_anon.[ID].28@PAGE
Lloh212:
	add	x4, x4, l_anon.[ID].28@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	b	LBB19_1
	.loh AdrpAdd	Lloh200, Lloh201
	.loh AdrpLdrGotLdr	Lloh204, Lloh205, Lloh206
	.loh AdrpAdrp	Lloh202, Lloh204
	.loh AdrpLdr	Lloh202, Lloh203
	.loh AdrpAdd	Lloh211, Lloh212
	.loh AdrpAdd	Lloh209, Lloh210
	.loh AdrpAdd	Lloh207, Lloh208

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
Lloh213:
	adrp	x8, __MergedGlobals@PAGE+8
Lloh214:
	add	x8, x8, __MergedGlobals@PAGEOFF+8
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB21_2
LBB21_1:
Lloh215:
	adrp	x8, __MergedGlobals@PAGE
Lloh216:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF]
Lloh217:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh218:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh219:
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
Lloh220:
	adrp	x0, __MergedGlobals@PAGE+8
Lloh221:
	add	x0, x0, __MergedGlobals@PAGEOFF+8
Lloh222:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh223:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh224:
	adrp	x4, l_anon.[ID].28@PAGE
Lloh225:
	add	x4, x4, l_anon.[ID].28@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	b	LBB21_1
	.loh AdrpAdd	Lloh213, Lloh214
	.loh AdrpLdrGotLdr	Lloh217, Lloh218, Lloh219
	.loh AdrpAdrp	Lloh215, Lloh217
	.loh AdrpLdr	Lloh215, Lloh216
	.loh AdrpAdd	Lloh224, Lloh225
	.loh AdrpAdd	Lloh222, Lloh223
	.loh AdrpAdd	Lloh220, Lloh221

	.globl	SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh226:
	adrp	x8, __MergedGlobals@PAGE+24
Lloh227:
	add	x8, x8, __MergedGlobals@PAGEOFF+24
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB22_2
Lloh228:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh229:
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
Lloh230:
	adrp	x0, __MergedGlobals@PAGE+24
Lloh231:
	add	x0, x0, __MergedGlobals@PAGEOFF+24
Lloh232:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh233:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
Lloh234:
	adrp	x4, l_anon.[ID].31@PAGE
Lloh235:
	add	x4, x4, l_anon.[ID].31@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh236:
	adrp	x8, __MergedGlobals@PAGE+16
Lloh237:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+16]
	ret
	.loh AdrpAdd	Lloh226, Lloh227
	.loh AdrpLdr	Lloh228, Lloh229
	.loh AdrpLdr	Lloh236, Lloh237
	.loh AdrpAdd	Lloh234, Lloh235
	.loh AdrpAdd	Lloh232, Lloh233
	.loh AdrpAdd	Lloh230, Lloh231

	.globl	_init_forgetable_ivars
	.p2align	2
_init_forgetable_ivars:
	cbz	x0, LBB23_2
Lloh238:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2)@PAGE
Lloh239:
	ldr	x8, [x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2)@PAGEOFF]
	add	x8, x0, x8
	mov	w9, #43
	str	w9, [x8]
	mov	w9, #42
	strb	w9, [x8, #4]
LBB23_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh240:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_af8966656b8b2b6c@PAGE
Lloh241:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_af8966656b8b2b6c@PAGEOFF]
Lloh242:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh243:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh244:
	ldr	x8, [x8]
	stp	x0, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpLdr	Lloh238, Lloh239
	.loh AdrpLdrGotLdr	Lloh242, Lloh243, Lloh244
	.loh AdrpAdrp	Lloh240, Lloh242
	.loh AdrpLdr	Lloh240, Lloh241

	.globl	SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh245:
	adrp	x8, __MergedGlobals@PAGE+40
Lloh246:
	add	x8, x8, __MergedGlobals@PAGEOFF+40
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB24_2
Lloh247:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh248:
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
Lloh249:
	adrp	x0, __MergedGlobals@PAGE+40
Lloh250:
	add	x0, x0, __MergedGlobals@PAGEOFF+40
Lloh251:
	adrp	x3, l_anon.[ID].1@PAGE
Lloh252:
	add	x3, x3, l_anon.[ID].1@PAGEOFF
Lloh253:
	adrp	x4, l_anon.[ID].32@PAGE
Lloh254:
	add	x4, x4, l_anon.[ID].32@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh255:
	adrp	x8, __MergedGlobals@PAGE+32
Lloh256:
	ldr	x0, [x8, __MergedGlobals@PAGEOFF+32]
	ret
	.loh AdrpAdd	Lloh245, Lloh246
	.loh AdrpLdr	Lloh247, Lloh248
	.loh AdrpLdr	Lloh255, Lloh256
	.loh AdrpAdd	Lloh253, Lloh254
	.loh AdrpAdd	Lloh251, Lloh252
	.loh AdrpAdd	Lloh249, Lloh250

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
Lloh257:
	adrp	x8, SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)@PAGE
Lloh258:
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
Lloh259:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_6edddcebbded8f32@PAGE
Lloh260:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_6edddcebbded8f32@PAGEOFF]
Lloh261:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh262:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh263:
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
	.loh AdrpLdr	Lloh257, Lloh258
	.loh AdrpLdrGotLdr	Lloh261, Lloh262, Lloh263
	.loh AdrpAdrp	Lloh259, Lloh261
	.loh AdrpLdr	Lloh259, Lloh260

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
	.ascii	"failed retrieving instance variable on newly declared class"

l_anon.[ID].17:
	.ascii	"$WORKSPACE/objc2/src/__macro_helpers/declared_ivars.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].18:
	.quad	l_anon.[ID].17
	.asciz	"P\000\000\000\000\000\000\000\035\001\000\000\016\000\000"

	.section	__TEXT,__const
l_anon.[ID].19:
	.ascii	"failed retrieving drop flag instance variable on newly declared class"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].20:
	.quad	l_anon.[ID].17
	.asciz	"P\000\000\000\000\000\000\000)\001\000\000\016\000\000"

	.section	__TEXT,__const
l_anon.[ID].21:
	.ascii	"ForgetableIvars"

l_anon.[ID].22:
	.ascii	"NoIvars"

l_anon.[ID].23:
	.ascii	"DropIvars"

l_anon.[ID].24:
	.ascii	"called `Option::unwrap()` on a `None` value"

l_anon.[ID].25:
	.ascii	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].26:
	.quad	l_anon.[ID].25
	.asciz	"p\000\000\000\000\000\000\000\225\000\000\0002\000\000"

	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1),8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 1)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 1),8,3
	.section	__TEXT,__const
l_anon.[ID].27:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].28:
	.quad	l_anon.[ID].27
	.asciz	"5\000\000\000\000\000\000\000\016\000\000\000\001\000\000"

	.section	__TEXT,__literal8,8byte_literals
l_anon.[ID].29:
	.ascii	"NSObject"

	.section	__TEXT,__const
l_anon.[ID].30:
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

	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2),8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 2)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 2),8,3
	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].31:
	.quad	l_anon.[ID].27
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
l_anon.[ID].32:
	.quad	l_anon.[ID].27
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
