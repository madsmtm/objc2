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
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
	sub	sp, sp, #64
	stp	x22, x21, [sp, #16]
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	cbz	w9, LBB3_7
Lloh0:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh1:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh2:
	ldr	x2, [x8]
Lloh3:
	adrp	x0, l_anon.[ID].15@PAGE
Lloh4:
	add	x0, x0, l_anon.[ID].15@PAGEOFF
	mov	w1, #7
	bl	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB3_8
	str	x0, [sp, #8]
Lloh5:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_f249b8b52b9a1205@PAGE
Lloh6:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_f249b8b52b9a1205@PAGEOFF]
Lloh7:
	adrp	x19, l_anon.[ID].3@PAGE
Lloh8:
	add	x19, x19, l_anon.[ID].3@PAGEOFF
Lloh9:
	adrp	x4, l_anon.[ID].10@PAGE
Lloh10:
	add	x4, x4, l_anon.[ID].10@PAGEOFF
Lloh11:
	adrp	x5, _get_class@PAGE
Lloh12:
	add	x5, x5, _get_class@PAGEOFF
	add	x0, sp, #8
	mov	x2, x19
	mov	x3, #0
	bl	SYM(objc2::declare::ClassBuilder::add_class_method_inner::GENERATED_ID, 0)
Lloh13:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_2607fe9c3979c381@PAGE
Lloh14:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_2607fe9c3979c381@PAGEOFF]
Lloh15:
	adrp	x4, l_anon.[ID].5@PAGE
Lloh16:
	add	x4, x4, l_anon.[ID].5@PAGEOFF
Lloh17:
	adrp	x5, _method_simple@PAGE
Lloh18:
	add	x5, x5, _method_simple@PAGEOFF
	add	x0, sp, #8
	mov	x2, x19
	mov	x3, #0
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh19:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_5a4952192b00de7f@PAGE
Lloh20:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_5a4952192b00de7f@PAGEOFF]
Lloh21:
	adrp	x20, l_anon.[ID].6@PAGE
Lloh22:
	add	x20, x20, l_anon.[ID].6@PAGEOFF
Lloh23:
	adrp	x5, _method_bool@PAGE
Lloh24:
	add	x5, x5, _method_bool@PAGEOFF
	add	x0, sp, #8
	mov	x2, x20
	mov	w3, #1
	mov	x4, x20
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh25:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_bf0386bc74a73c00@PAGE
Lloh26:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_bf0386bc74a73c00@PAGEOFF]
Lloh27:
	adrp	x21, l_anon.[ID].4@PAGE
Lloh28:
	add	x21, x21, l_anon.[ID].4@PAGEOFF
Lloh29:
	adrp	x5, _method_id@PAGE
Lloh30:
	add	x5, x5, _method_id@PAGEOFF
	add	x0, sp, #8
	mov	x2, x19
	mov	x3, #0
	mov	x4, x21
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh31:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_6f0ab2a047fe4a09@PAGE
Lloh32:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_6f0ab2a047fe4a09@PAGEOFF]
Lloh33:
	adrp	x5, _method_id_with_param@PAGE
Lloh34:
	add	x5, x5, _method_id_with_param@PAGEOFF
	add	x0, sp, #8
	mov	x2, x20
	mov	w3, #1
	mov	x4, x21
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh35:
	adrp	x0, l_anon.[ID].23@PAGE
Lloh36:
	add	x0, x0, l_anon.[ID].23@PAGEOFF
	mov	w1, #8
	bl	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	cbz	x0, LBB3_4
	mov	x1, x0
	add	x0, sp, #8
	bl	SYM(objc2::declare::ClassBuilder::add_protocol::GENERATED_ID, 0)
LBB3_4:
Lloh37:
	adrp	x0, l_anon.[ID].24@PAGE
Lloh38:
	add	x0, x0, l_anon.[ID].24@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	cbz	x0, LBB3_6
	mov	x1, x0
	add	x0, sp, #8
	bl	SYM(objc2::declare::ClassBuilder::add_protocol::GENERATED_ID, 0)
LBB3_6:
Lloh39:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_062e7a5fdd2d2571@PAGE
Lloh40:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_062e7a5fdd2d2571@PAGEOFF]
Lloh41:
	adrp	x2, l_anon.[ID].9@PAGE
Lloh42:
	add	x2, x2, l_anon.[ID].9@PAGEOFF
Lloh43:
	adrp	x4, l_anon.[ID].4@PAGE
Lloh44:
	add	x4, x4, l_anon.[ID].4@PAGEOFF
Lloh45:
	adrp	x5, _copyWithZone@PAGE
Lloh46:
	add	x5, x5, _copyWithZone@PAGEOFF
	add	x0, sp, #8
	mov	w3, #1
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x0, [sp, #8]
	bl	SYM(objc2::declare::ClassBuilder::register::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	ldp	x22, x21, [sp, #16]
	add	sp, sp, #64
	ret
LBB3_7:
Lloh47:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh48:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
Lloh49:
	adrp	x2, l_anon.[ID].13@PAGE
Lloh50:
	add	x2, x2, l_anon.[ID].13@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB3_8:
Lloh51:
	adrp	x0, l_anon.[ID].15@PAGE
Lloh52:
	add	x0, x0, l_anon.[ID].15@PAGEOFF
Lloh53:
	adrp	x2, l_anon.[ID].22@PAGE
Lloh54:
	add	x2, x2, l_anon.[ID].22@PAGEOFF
	mov	w1, #7
	bl	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh3, Lloh4
	.loh AdrpLdrGotLdr	Lloh0, Lloh1, Lloh2
	.loh AdrpAdd	Lloh35, Lloh36
	.loh AdrpAdd	Lloh33, Lloh34
	.loh AdrpLdr	Lloh31, Lloh32
	.loh AdrpAdd	Lloh29, Lloh30
	.loh AdrpAdd	Lloh27, Lloh28
	.loh AdrpLdr	Lloh25, Lloh26
	.loh AdrpAdd	Lloh23, Lloh24
	.loh AdrpAdd	Lloh21, Lloh22
	.loh AdrpLdr	Lloh19, Lloh20
	.loh AdrpAdd	Lloh17, Lloh18
	.loh AdrpAdd	Lloh15, Lloh16
	.loh AdrpLdr	Lloh13, Lloh14
	.loh AdrpAdd	Lloh11, Lloh12
	.loh AdrpAdd	Lloh9, Lloh10
	.loh AdrpAdd	Lloh7, Lloh8
	.loh AdrpLdr	Lloh5, Lloh6
	.loh AdrpAdd	Lloh37, Lloh38
	.loh AdrpAdd	Lloh45, Lloh46
	.loh AdrpAdd	Lloh43, Lloh44
	.loh AdrpAdd	Lloh41, Lloh42
	.loh AdrpLdr	Lloh39, Lloh40
	.loh AdrpAdd	Lloh49, Lloh50
	.loh AdrpAdd	Lloh47, Lloh48
	.loh AdrpAdd	Lloh53, Lloh54
	.loh AdrpAdd	Lloh51, Lloh52

	.p2align	2
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	cbz	w9, LBB4_3
Lloh55:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh56:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh57:
	ldr	x2, [x8]
Lloh58:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh59:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB4_4
	str	x0, [sp, #8]
Lloh60:
	adrp	x1, l_anon.[ID].18@PAGE
Lloh61:
	add	x1, x1, l_anon.[ID].18@PAGEOFF
Lloh62:
	adrp	x19, l_anon.[ID].4@PAGE
Lloh63:
	add	x19, x19, l_anon.[ID].4@PAGEOFF
	add	x0, sp, #8
	mov	w2, #4
	mov	w3, #8
	mov	w4, #3
	mov	x5, x19
	bl	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Lloh64:
	adrp	x1, l_anon.[ID].17@PAGE
Lloh65:
	add	x1, x1, l_anon.[ID].17@PAGEOFF
	add	x0, sp, #8
	mov	w2, #11
	mov	w3, #8
	mov	w4, #3
	mov	x5, x19
	bl	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Lloh66:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGE
Lloh67:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGEOFF]
Lloh68:
	ldr	x1, [x8]
Lloh69:
	adrp	x20, l_anon.[ID].3@PAGE
Lloh70:
	add	x20, x20, l_anon.[ID].3@PAGEOFF
Lloh71:
	adrp	x4, l_anon.[ID].5@PAGE
Lloh72:
	add	x4, x4, l_anon.[ID].5@PAGEOFF
Lloh73:
	adrp	x5, SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::__objc2_dealloc, 0)@PAGE
Lloh74:
	add	x5, x5, SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::__objc2_dealloc, 0)@PAGEOFF
	add	x0, sp, #8
	mov	x2, x20
	mov	x3, #0
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh75:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh76:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh77:
	ldr	x1, [x8]
Lloh78:
	adrp	x5, _init_drop_ivars@PAGE
Lloh79:
	add	x5, x5, _init_drop_ivars@PAGEOFF
	add	x0, sp, #8
	mov	x2, x20
	mov	x3, #0
	mov	x4, x19
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x0, [sp, #8]
	bl	SYM(objc2::declare::ClassBuilder::register::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB4_3:
Lloh80:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh81:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
Lloh82:
	adrp	x2, l_anon.[ID].13@PAGE
Lloh83:
	add	x2, x2, l_anon.[ID].13@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB4_4:
Lloh84:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh85:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
Lloh86:
	adrp	x2, l_anon.[ID].28@PAGE
Lloh87:
	add	x2, x2, l_anon.[ID].28@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh58, Lloh59
	.loh AdrpLdrGotLdr	Lloh55, Lloh56, Lloh57
	.loh AdrpAdd	Lloh78, Lloh79
	.loh AdrpLdrGotLdr	Lloh75, Lloh76, Lloh77
	.loh AdrpAdd	Lloh73, Lloh74
	.loh AdrpAdd	Lloh71, Lloh72
	.loh AdrpAdd	Lloh69, Lloh70
	.loh AdrpLdrGotLdr	Lloh66, Lloh67, Lloh68
	.loh AdrpAdd	Lloh64, Lloh65
	.loh AdrpAdd	Lloh62, Lloh63
	.loh AdrpAdd	Lloh60, Lloh61
	.loh AdrpAdd	Lloh82, Lloh83
	.loh AdrpAdd	Lloh80, Lloh81
	.loh AdrpAdd	Lloh86, Lloh87
	.loh AdrpAdd	Lloh84, Lloh85

	.p2align	2
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	cbz	w9, LBB5_3
Lloh88:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh89:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh90:
	ldr	x2, [x8]
Lloh91:
	adrp	x0, l_anon.[ID].14@PAGE
Lloh92:
	add	x0, x0, l_anon.[ID].14@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB5_4
	str	x0, [sp, #8]
Lloh93:
	adrp	x1, l_anon.[ID].20@PAGE
Lloh94:
	add	x1, x1, l_anon.[ID].20@PAGEOFF
Lloh95:
	adrp	x5, l_anon.[ID].25@PAGE
Lloh96:
	add	x5, x5, l_anon.[ID].25@PAGEOFF
	add	x0, sp, #8
	mov	w2, #4
	mov	w3, #1
	mov	w4, #0
	bl	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Lloh97:
	adrp	x1, l_anon.[ID].19@PAGE
Lloh98:
	add	x1, x1, l_anon.[ID].19@PAGEOFF
Lloh99:
	adrp	x5, l_anon.[ID].26@PAGE
Lloh100:
	add	x5, x5, l_anon.[ID].26@PAGEOFF
	add	x0, sp, #8
	mov	w2, #4
	mov	w3, #4
	mov	w4, #2
	bl	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Lloh101:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGE
Lloh102:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGEOFF]
Lloh103:
	ldr	x1, [x8]
Lloh104:
	adrp	x19, l_anon.[ID].3@PAGE
Lloh105:
	add	x19, x19, l_anon.[ID].3@PAGEOFF
Lloh106:
	adrp	x4, l_anon.[ID].5@PAGE
Lloh107:
	add	x4, x4, l_anon.[ID].5@PAGEOFF
Lloh108:
	adrp	x5, SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::__objc2_dealloc, 0)@PAGE
Lloh109:
	add	x5, x5, SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::__objc2_dealloc, 0)@PAGEOFF
	add	x0, sp, #8
	mov	x2, x19
	mov	x3, #0
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh110:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh111:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh112:
	ldr	x1, [x8]
Lloh113:
	adrp	x4, l_anon.[ID].4@PAGE
Lloh114:
	add	x4, x4, l_anon.[ID].4@PAGEOFF
Lloh115:
	adrp	x5, _init_forgetable_ivars@PAGE
Lloh116:
	add	x5, x5, _init_forgetable_ivars@PAGEOFF
	add	x0, sp, #8
	mov	x2, x19
	mov	x3, #0
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x0, [sp, #8]
	bl	SYM(objc2::declare::ClassBuilder::register::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB5_3:
Lloh117:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh118:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
Lloh119:
	adrp	x2, l_anon.[ID].13@PAGE
Lloh120:
	add	x2, x2, l_anon.[ID].13@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB5_4:
Lloh121:
	adrp	x0, l_anon.[ID].14@PAGE
Lloh122:
	add	x0, x0, l_anon.[ID].14@PAGEOFF
Lloh123:
	adrp	x2, l_anon.[ID].27@PAGE
Lloh124:
	add	x2, x2, l_anon.[ID].27@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh91, Lloh92
	.loh AdrpLdrGotLdr	Lloh88, Lloh89, Lloh90
	.loh AdrpAdd	Lloh115, Lloh116
	.loh AdrpAdd	Lloh113, Lloh114
	.loh AdrpLdrGotLdr	Lloh110, Lloh111, Lloh112
	.loh AdrpAdd	Lloh108, Lloh109
	.loh AdrpAdd	Lloh106, Lloh107
	.loh AdrpAdd	Lloh104, Lloh105
	.loh AdrpLdrGotLdr	Lloh101, Lloh102, Lloh103
	.loh AdrpAdd	Lloh99, Lloh100
	.loh AdrpAdd	Lloh97, Lloh98
	.loh AdrpAdd	Lloh95, Lloh96
	.loh AdrpAdd	Lloh93, Lloh94
	.loh AdrpAdd	Lloh119, Lloh120
	.loh AdrpAdd	Lloh117, Lloh118
	.loh AdrpAdd	Lloh123, Lloh124
	.loh AdrpAdd	Lloh121, Lloh122

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
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh125:
	adrp	x8, SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh126:
	add	x8, x8, SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB9_3
Lloh127:
	adrp	x0, l_anon.[ID].14@PAGE
Lloh128:
	add	x0, x0, l_anon.[ID].14@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbz	x0, LBB9_4
LBB9_2:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB9_3:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh129:
	adrp	x0, SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh130:
	add	x0, x0, SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
Lloh131:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh132:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
Lloh133:
	adrp	x4, l_anon.[ID].27@PAGE
Lloh134:
	add	x4, x4, l_anon.[ID].27@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
Lloh135:
	adrp	x0, l_anon.[ID].14@PAGE
Lloh136:
	add	x0, x0, l_anon.[ID].14@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbnz	x0, LBB9_2
LBB9_4:
Lloh137:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh138:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
Lloh139:
	adrp	x2, l_anon.[ID].27@PAGE
Lloh140:
	add	x2, x2, l_anon.[ID].27@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh125, Lloh126
	.loh AdrpAdd	Lloh127, Lloh128
	.loh AdrpAdd	Lloh135, Lloh136
	.loh AdrpAdd	Lloh133, Lloh134
	.loh AdrpAdd	Lloh131, Lloh132
	.loh AdrpAdd	Lloh129, Lloh130
	.loh AdrpAdd	Lloh139, Lloh140
	.loh AdrpAdd	Lloh137, Lloh138

	.globl	_access_forgetable_ivars
	.p2align	2
_access_forgetable_ivars:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x0
Lloh141:
	adrp	x1, l_anon.[ID].20@PAGE
Lloh142:
	add	x1, x1, l_anon.[ID].20@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	ldrb	w20, [x19, x0]
Lloh143:
	adrp	x1, l_anon.[ID].19@PAGE
Lloh144:
	add	x1, x1, l_anon.[ID].19@PAGEOFF
	mov	x0, x19
	mov	w2, #4
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	ldr	w1, [x19, x0]
	mov	x0, x20
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
	.loh AdrpAdd	Lloh143, Lloh144
	.loh AdrpAdd	Lloh141, Lloh142

	.globl	SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0):
	; InlineAsm Start
	; InlineAsm End
	ret

	.globl	_access_drop_ivars_class
	.p2align	2
_access_drop_ivars_class:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh145:
	adrp	x8, SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh146:
	add	x8, x8, SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB12_3
Lloh147:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh148:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbz	x0, LBB12_4
LBB12_2:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB12_3:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh149:
	adrp	x0, SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh150:
	add	x0, x0, SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
Lloh151:
	adrp	x3, l_anon.[ID].1@PAGE
Lloh152:
	add	x3, x3, l_anon.[ID].1@PAGEOFF
Lloh153:
	adrp	x4, l_anon.[ID].28@PAGE
Lloh154:
	add	x4, x4, l_anon.[ID].28@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
Lloh155:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh156:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbnz	x0, LBB12_2
LBB12_4:
Lloh157:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh158:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
Lloh159:
	adrp	x2, l_anon.[ID].28@PAGE
Lloh160:
	add	x2, x2, l_anon.[ID].28@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh145, Lloh146
	.loh AdrpAdd	Lloh147, Lloh148
	.loh AdrpAdd	Lloh155, Lloh156
	.loh AdrpAdd	Lloh153, Lloh154
	.loh AdrpAdd	Lloh151, Lloh152
	.loh AdrpAdd	Lloh149, Lloh150
	.loh AdrpAdd	Lloh159, Lloh160
	.loh AdrpAdd	Lloh157, Lloh158

	.globl	_access_drop_ivars
	.p2align	2
_access_drop_ivars:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x0
Lloh161:
	adrp	x1, l_anon.[ID].18@PAGE
Lloh162:
	add	x1, x1, l_anon.[ID].18@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	ldr	x0, [x19, x0]
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
	.loh AdrpAdd	Lloh161, Lloh162

	.globl	SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh163:
	adrp	x8, SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh164:
	add	x8, x8, SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB14_3
Lloh165:
	adrp	x0, l_anon.[ID].15@PAGE
Lloh166:
	add	x0, x0, l_anon.[ID].15@PAGEOFF
	mov	w1, #7
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbz	x0, LBB14_4
LBB14_2:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB14_3:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh167:
	adrp	x0, SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh168:
	add	x0, x0, SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
Lloh169:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh170:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh171:
	adrp	x4, l_anon.[ID].22@PAGE
Lloh172:
	add	x4, x4, l_anon.[ID].22@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
Lloh173:
	adrp	x0, l_anon.[ID].15@PAGE
Lloh174:
	add	x0, x0, l_anon.[ID].15@PAGEOFF
	mov	w1, #7
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbnz	x0, LBB14_2
LBB14_4:
Lloh175:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh176:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
Lloh177:
	adrp	x2, l_anon.[ID].22@PAGE
Lloh178:
	add	x2, x2, l_anon.[ID].22@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh163, Lloh164
	.loh AdrpAdd	Lloh165, Lloh166
	.loh AdrpAdd	Lloh173, Lloh174
	.loh AdrpAdd	Lloh171, Lloh172
	.loh AdrpAdd	Lloh169, Lloh170
	.loh AdrpAdd	Lloh167, Lloh168
	.loh AdrpAdd	Lloh177, Lloh178
	.loh AdrpAdd	Lloh175, Lloh176

	.globl	_get_class
	.p2align	2
_get_class:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh179:
	adrp	x8, SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh180:
	add	x8, x8, SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB15_3
Lloh181:
	adrp	x0, l_anon.[ID].15@PAGE
Lloh182:
	add	x0, x0, l_anon.[ID].15@PAGEOFF
	mov	w1, #7
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbz	x0, LBB15_4
LBB15_2:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB15_3:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh183:
	adrp	x0, SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh184:
	add	x0, x0, SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
Lloh185:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh186:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh187:
	adrp	x4, l_anon.[ID].22@PAGE
Lloh188:
	add	x4, x4, l_anon.[ID].22@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
Lloh189:
	adrp	x0, l_anon.[ID].15@PAGE
Lloh190:
	add	x0, x0, l_anon.[ID].15@PAGEOFF
	mov	w1, #7
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbnz	x0, LBB15_2
LBB15_4:
Lloh191:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh192:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
Lloh193:
	adrp	x2, l_anon.[ID].22@PAGE
Lloh194:
	add	x2, x2, l_anon.[ID].22@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh179, Lloh180
	.loh AdrpAdd	Lloh181, Lloh182
	.loh AdrpAdd	Lloh189, Lloh190
	.loh AdrpAdd	Lloh187, Lloh188
	.loh AdrpAdd	Lloh185, Lloh186
	.loh AdrpAdd	Lloh183, Lloh184
	.loh AdrpAdd	Lloh193, Lloh194
	.loh AdrpAdd	Lloh191, Lloh192

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
Lloh195:
	adrp	x8, SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh196:
	add	x8, x8, SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB18_3
Lloh197:
	adrp	x0, l_anon.[ID].15@PAGE
Lloh198:
	add	x0, x0, l_anon.[ID].15@PAGEOFF
	mov	w1, #7
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbz	x0, LBB18_4
LBB18_2:
Lloh199:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh200:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh201:
	ldr	x1, [x8]
	bl	_objc_msgSend
	bl	_objc_autoreleaseReturnValue
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB18_3:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh202:
	adrp	x0, SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh203:
	add	x0, x0, SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
Lloh204:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh205:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh206:
	adrp	x4, l_anon.[ID].22@PAGE
Lloh207:
	add	x4, x4, l_anon.[ID].22@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
Lloh208:
	adrp	x0, l_anon.[ID].15@PAGE
Lloh209:
	add	x0, x0, l_anon.[ID].15@PAGEOFF
	mov	w1, #7
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbnz	x0, LBB18_2
LBB18_4:
Lloh210:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh211:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
Lloh212:
	adrp	x2, l_anon.[ID].22@PAGE
Lloh213:
	add	x2, x2, l_anon.[ID].22@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh195, Lloh196
	.loh AdrpAdd	Lloh197, Lloh198
	.loh AdrpLdrGotLdr	Lloh199, Lloh200, Lloh201
	.loh AdrpAdd	Lloh208, Lloh209
	.loh AdrpAdd	Lloh206, Lloh207
	.loh AdrpAdd	Lloh204, Lloh205
	.loh AdrpAdd	Lloh202, Lloh203
	.loh AdrpAdd	Lloh212, Lloh213
	.loh AdrpAdd	Lloh210, Lloh211

	.globl	_method_id_with_param
	.p2align	2
_method_id_with_param:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x20, x2
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	x19, x0
	tbz	w20, #0, LBB19_2
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	x20, x0
	mov	x0, x19
	bl	_objc_release
	mov	x19, x20
LBB19_2:
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
Lloh214:
	adrp	x8, SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh215:
	add	x8, x8, SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB20_3
Lloh216:
	adrp	x0, l_anon.[ID].15@PAGE
Lloh217:
	add	x0, x0, l_anon.[ID].15@PAGEOFF
	mov	w1, #7
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbz	x0, LBB20_4
LBB20_2:
Lloh218:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh219:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh220:
	ldr	x1, [x8]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB20_3:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh221:
	adrp	x0, SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh222:
	add	x0, x0, SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
Lloh223:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh224:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh225:
	adrp	x4, l_anon.[ID].22@PAGE
Lloh226:
	add	x4, x4, l_anon.[ID].22@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
Lloh227:
	adrp	x0, l_anon.[ID].15@PAGE
Lloh228:
	add	x0, x0, l_anon.[ID].15@PAGEOFF
	mov	w1, #7
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbnz	x0, LBB20_2
LBB20_4:
Lloh229:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh230:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
Lloh231:
	adrp	x2, l_anon.[ID].22@PAGE
Lloh232:
	add	x2, x2, l_anon.[ID].22@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh214, Lloh215
	.loh AdrpAdd	Lloh216, Lloh217
	.loh AdrpLdrGotLdr	Lloh218, Lloh219, Lloh220
	.loh AdrpAdd	Lloh227, Lloh228
	.loh AdrpAdd	Lloh225, Lloh226
	.loh AdrpAdd	Lloh223, Lloh224
	.loh AdrpAdd	Lloh221, Lloh222
	.loh AdrpAdd	Lloh231, Lloh232
	.loh AdrpAdd	Lloh229, Lloh230

	.globl	SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh233:
	adrp	x8, SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh234:
	add	x8, x8, SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB21_3
Lloh235:
	adrp	x0, l_anon.[ID].14@PAGE
Lloh236:
	add	x0, x0, l_anon.[ID].14@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbz	x0, LBB21_4
LBB21_2:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB21_3:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh237:
	adrp	x0, SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh238:
	add	x0, x0, SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
Lloh239:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh240:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
Lloh241:
	adrp	x4, l_anon.[ID].27@PAGE
Lloh242:
	add	x4, x4, l_anon.[ID].27@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
Lloh243:
	adrp	x0, l_anon.[ID].14@PAGE
Lloh244:
	add	x0, x0, l_anon.[ID].14@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbnz	x0, LBB21_2
LBB21_4:
Lloh245:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh246:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
Lloh247:
	adrp	x2, l_anon.[ID].27@PAGE
Lloh248:
	add	x2, x2, l_anon.[ID].27@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh233, Lloh234
	.loh AdrpAdd	Lloh235, Lloh236
	.loh AdrpAdd	Lloh243, Lloh244
	.loh AdrpAdd	Lloh241, Lloh242
	.loh AdrpAdd	Lloh239, Lloh240
	.loh AdrpAdd	Lloh237, Lloh238
	.loh AdrpAdd	Lloh247, Lloh248
	.loh AdrpAdd	Lloh245, Lloh246

	.p2align	2
SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::__objc2_dealloc, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh249:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh250:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh251:
	ldr	x8, [x8]
	stp	x0, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpLdrGotLdr	Lloh249, Lloh250, Lloh251

	.globl	_init_forgetable_ivars
	.p2align	2
_init_forgetable_ivars:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
Lloh252:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh253:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh254:
	ldr	x1, [x8]
Lloh255:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh256:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh257:
	ldr	x8, [x8]
	stp	x0, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	mov	x19, x0
	cbz	x0, LBB23_2
Lloh258:
	adrp	x1, l_anon.[ID].20@PAGE
Lloh259:
	add	x1, x1, l_anon.[ID].20@PAGEOFF
	mov	x0, x19
	mov	w2, #4
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	mov	w8, #42
	strb	w8, [x19, x0]
Lloh260:
	adrp	x1, l_anon.[ID].19@PAGE
Lloh261:
	add	x1, x1, l_anon.[ID].19@PAGEOFF
	mov	x0, x19
	mov	w2, #4
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	mov	w8, #43
	str	w8, [x19, x0]
LBB23_2:
	mov	x0, x19
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
	.loh AdrpLdrGotLdr	Lloh255, Lloh256, Lloh257
	.loh AdrpLdrGotLdr	Lloh252, Lloh253, Lloh254
	.loh AdrpAdd	Lloh260, Lloh261
	.loh AdrpAdd	Lloh258, Lloh259

	.globl	SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh262:
	adrp	x8, SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh263:
	add	x8, x8, SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB24_3
Lloh264:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh265:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbz	x0, LBB24_4
LBB24_2:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB24_3:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh266:
	adrp	x0, SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh267:
	add	x0, x0, SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
Lloh268:
	adrp	x3, l_anon.[ID].1@PAGE
Lloh269:
	add	x3, x3, l_anon.[ID].1@PAGEOFF
Lloh270:
	adrp	x4, l_anon.[ID].28@PAGE
Lloh271:
	add	x4, x4, l_anon.[ID].28@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
Lloh272:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh273:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbnz	x0, LBB24_2
LBB24_4:
Lloh274:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh275:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
Lloh276:
	adrp	x2, l_anon.[ID].28@PAGE
Lloh277:
	add	x2, x2, l_anon.[ID].28@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh262, Lloh263
	.loh AdrpAdd	Lloh264, Lloh265
	.loh AdrpAdd	Lloh272, Lloh273
	.loh AdrpAdd	Lloh270, Lloh271
	.loh AdrpAdd	Lloh268, Lloh269
	.loh AdrpAdd	Lloh266, Lloh267
	.loh AdrpAdd	Lloh276, Lloh277
	.loh AdrpAdd	Lloh274, Lloh275

	.p2align	2
SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::__objc2_dealloc, 0):
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x1
	mov	x20, x0
	bl	SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0)
Lloh278:
	adrp	x1, l_anon.[ID].18@PAGE
Lloh279:
	add	x1, x1, l_anon.[ID].18@PAGEOFF
	mov	x0, x20
	mov	w2, #4
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	ldr	x0, [x20, x0]
	cbz	x0, LBB25_2
	bl	_objc_release
LBB25_2:
Lloh280:
	adrp	x1, l_anon.[ID].17@PAGE
Lloh281:
	add	x1, x1, l_anon.[ID].17@PAGEOFF
	mov	x0, x20
	mov	w2, #11
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	ldr	x0, [x20, x0]
	cbz	x0, LBB25_4
	bl	_objc_release
LBB25_4:
Lloh282:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh283:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh284:
	ldr	x8, [x8]
	stp	x20, x8, [sp]
	mov	x0, sp
	mov	x1, x19
	bl	_objc_msgSendSuper
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
	.loh AdrpAdd	Lloh278, Lloh279
	.loh AdrpAdd	Lloh280, Lloh281
	.loh AdrpLdrGotLdr	Lloh282, Lloh283, Lloh284

	.globl	_init_drop_ivars
	.p2align	2
_init_drop_ivars:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
Lloh285:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh286:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh287:
	ldr	x1, [x8]
Lloh288:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh289:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh290:
	ldr	x8, [x8]
	stp	x0, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	mov	x19, x0
	cbz	x0, LBB26_2
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	x20, x0
Lloh291:
	adrp	x1, l_anon.[ID].18@PAGE
Lloh292:
	add	x1, x1, l_anon.[ID].18@PAGEOFF
	mov	x0, x19
	mov	w2, #4
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	str	x20, [x19, x0]
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	x20, x0
Lloh293:
	adrp	x1, l_anon.[ID].17@PAGE
Lloh294:
	add	x1, x1, l_anon.[ID].17@PAGEOFF
	mov	x0, x19
	mov	w2, #11
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	str	x20, [x19, x0]
LBB26_2:
	mov	x0, x19
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
	.loh AdrpLdrGotLdr	Lloh288, Lloh289, Lloh290
	.loh AdrpLdrGotLdr	Lloh285, Lloh286, Lloh287
	.loh AdrpAdd	Lloh293, Lloh294
	.loh AdrpAdd	Lloh291, Lloh292

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
	.ascii	"called `Option::unwrap()` on a `None` value"

l_anon.[ID].12:
	.ascii	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].13:
	.quad	l_anon.[ID].12
	.asciz	"p\000\000\000\000\000\000\000\225\000\000\0002\000\000"

	.section	__TEXT,__const
l_anon.[ID].14:
	.ascii	"ForgetableIvars"

l_anon.[ID].15:
	.ascii	"NoIvars"

l_anon.[ID].16:
	.ascii	"DropIvars"

l_anon.[ID].17:
	.ascii	"_obj_option"

	.section	__TEXT,__literal4,4byte_literals
l_anon.[ID].18:
	.ascii	"_obj"

l_anon.[ID].19:
	.ascii	"_bar"

l_anon.[ID].20:
	.ascii	"_foo"

	.section	__TEXT,__const
l_anon.[ID].21:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].22:
	.quad	l_anon.[ID].21
	.asciz	"5\000\000\000\000\000\000\000\017\000\000\000\001\000\000"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0),8,3
	.section	__TEXT,__literal8,8byte_literals
l_anon.[ID].23:
	.ascii	"NSObject"

	.section	__TEXT,__const
l_anon.[ID].24:
	.ascii	"NSCopying"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_f249b8b52b9a1205
L_OBJC_METH_VAR_NAME_f249b8b52b9a1205:
	.asciz	"classMethod"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_f249b8b52b9a1205
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_f249b8b52b9a1205:
	.quad	L_OBJC_METH_VAR_NAME_f249b8b52b9a1205

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f249b8b52b9a1205
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f249b8b52b9a1205:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2607fe9c3979c381
L_OBJC_METH_VAR_NAME_2607fe9c3979c381:
	.asciz	"method"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2607fe9c3979c381
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_2607fe9c3979c381:
	.quad	L_OBJC_METH_VAR_NAME_2607fe9c3979c381

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2607fe9c3979c381
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2607fe9c3979c381:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_5a4952192b00de7f
L_OBJC_METH_VAR_NAME_5a4952192b00de7f:
	.asciz	"methodBool:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_5a4952192b00de7f
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_5a4952192b00de7f:
	.quad	L_OBJC_METH_VAR_NAME_5a4952192b00de7f

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_5a4952192b00de7f
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_5a4952192b00de7f:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_bf0386bc74a73c00
L_OBJC_METH_VAR_NAME_bf0386bc74a73c00:
	.asciz	"methodId"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_bf0386bc74a73c00
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_bf0386bc74a73c00:
	.quad	L_OBJC_METH_VAR_NAME_bf0386bc74a73c00

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_bf0386bc74a73c00
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_bf0386bc74a73c00:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6f0ab2a047fe4a09
L_OBJC_METH_VAR_NAME_6f0ab2a047fe4a09:
	.asciz	"methodIdWithParam:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_6f0ab2a047fe4a09
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_6f0ab2a047fe4a09:
	.quad	L_OBJC_METH_VAR_NAME_6f0ab2a047fe4a09

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_6f0ab2a047fe4a09
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_6f0ab2a047fe4a09:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_062e7a5fdd2d2571
L_OBJC_METH_VAR_NAME_062e7a5fdd2d2571:
	.asciz	"copyWithZone:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_062e7a5fdd2d2571
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_062e7a5fdd2d2571:
	.quad	L_OBJC_METH_VAR_NAME_062e7a5fdd2d2571

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_062e7a5fdd2d2571
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_062e7a5fdd2d2571:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].25:
	.byte	5
	.space	39

	.p2align	3, 0x0
l_anon.[ID].26:
	.byte	7
	.space	39

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].27:
	.quad	l_anon.[ID].21
	.asciz	"5\000\000\000\000\000\000\000I\000\000\000\001\000\000"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0),8,3
	.p2align	3, 0x0
l_anon.[ID].28:
	.quad	l_anon.[ID].21
	.asciz	"5\000\000\000\000\000\000\000u\000\000\000\001\000\000"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0),8,3
.subsections_via_symbols
