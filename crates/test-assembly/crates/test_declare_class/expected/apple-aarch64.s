	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0}>, 0):
	ret

	.p2align	2
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0}, 0):
	sub	sp, sp, #112
	stp	x22, x21, [sp, #64]
	stp	x20, x19, [sp, #80]
	stp	x29, x30, [sp, #96]
	add	x29, sp, #96
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	cbz	w9, LBB1_3
	bl	SYM(<objc2::runtime::nsobject::NSObject as objc2::class_type::ClassType>::class::objc_static_workaround::GENERATED_ID, 0)
	mov	x2, x0
Lloh0:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh1:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB1_4
	str	x0, [sp]
Lloh2:
	adrp	x1, l_anon.[ID].5@PAGE
Lloh3:
	add	x1, x1, l_anon.[ID].5@PAGEOFF
Lloh4:
	adrp	x5, l_anon.[ID].6@PAGE
Lloh5:
	add	x5, x5, l_anon.[ID].6@PAGEOFF
	mov	x0, sp
	mov	w2, #4
	mov	w3, #1
	mov	w4, #0
	bl	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Lloh6:
	adrp	x1, l_anon.[ID].7@PAGE
Lloh7:
	add	x1, x1, l_anon.[ID].7@PAGEOFF
Lloh8:
	adrp	x19, l_anon.[ID].8@PAGE
Lloh9:
	add	x19, x19, l_anon.[ID].8@PAGEOFF
	mov	x0, sp
	mov	w2, #4
	mov	w3, #8
	mov	w4, #3
	mov	x5, x19
	bl	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Lloh10:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_694e247a0bc88869@PAGE
Lloh11:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_694e247a0bc88869@PAGEOFF]
Lloh12:
	adrp	x20, l_anon.[ID].1@PAGE
Lloh13:
	add	x20, x20, l_anon.[ID].1@PAGEOFF
Lloh14:
	adrp	x21, l_anon.[ID].12@PAGE
Lloh15:
	add	x21, x21, l_anon.[ID].12@PAGEOFF
Lloh16:
	adrp	x5, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}::__objc2_dealloc, 0)@PAGE
Lloh17:
	add	x5, x5, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}::__objc2_dealloc, 0)@PAGEOFF
	mov	x0, sp
	mov	x2, x20
	mov	x3, #0
	mov	x4, x21
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh18:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh19:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh20:
	ldr	x1, [x8]
Lloh21:
	adrp	x5, _init@PAGE
Lloh22:
	add	x5, x5, _init@PAGEOFF
	mov	x0, sp
	mov	x2, x20
	mov	x3, #0
	mov	x4, x19
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh23:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_8dd788dbcc16b9bc@PAGE
Lloh24:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_8dd788dbcc16b9bc@PAGEOFF]
Lloh25:
	adrp	x5, _class_method@PAGE
Lloh26:
	add	x5, x5, _class_method@PAGEOFF
	mov	x0, sp
	mov	x2, x20
	mov	x3, #0
	mov	x4, x21
	bl	SYM(objc2::declare::ClassBuilder::add_class_method_inner::GENERATED_ID, 0)
Lloh27:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_450db9db0953dff5@PAGE
Lloh28:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_450db9db0953dff5@PAGEOFF]
Lloh29:
	adrp	x5, _method@PAGE
Lloh30:
	add	x5, x5, _method@PAGEOFF
	mov	x0, sp
	mov	x2, x20
	mov	x3, #0
	mov	x4, x21
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh31:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_783b35bc45c6e4a6@PAGE
Lloh32:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_783b35bc45c6e4a6@PAGEOFF]
Lloh33:
	adrp	x21, l_anon.[ID].13@PAGE
Lloh34:
	add	x21, x21, l_anon.[ID].13@PAGEOFF
Lloh35:
	adrp	x5, _method_bool@PAGE
Lloh36:
	add	x5, x5, _method_bool@PAGEOFF
	mov	x0, sp
	mov	x2, x21
	mov	w3, #1
	mov	x4, x21
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh37:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_828e9fbc6d0b4498@PAGE
Lloh38:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_828e9fbc6d0b4498@PAGEOFF]
Lloh39:
	adrp	x5, _method_id@PAGE
Lloh40:
	add	x5, x5, _method_id@PAGEOFF
	mov	x0, sp
	mov	x2, x20
	mov	x3, #0
	mov	x4, x19
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh41:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_788cc14ba6a28eb8@PAGE
Lloh42:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_788cc14ba6a28eb8@PAGEOFF]
Lloh43:
	adrp	x5, _method_id_with_param@PAGE
Lloh44:
	add	x5, x5, _method_id_with_param@PAGEOFF
	mov	x0, sp
	mov	x2, x21
	mov	w3, #1
	mov	x4, x19
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh45:
	adrp	x0, l_anon.[ID].14@PAGE
Lloh46:
	add	x0, x0, l_anon.[ID].14@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	mov	x1, x0
	mov	x0, sp
	bl	SYM(objc2::__macro_helpers::<impl objc2::declare::ClassBuilder>::__add_protocol_methods::GENERATED_ID, 0)
Lloh47:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_f058a81939de2cb9@PAGE
Lloh48:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_f058a81939de2cb9@PAGEOFF]
Lloh49:
	adrp	x2, l_anon.[ID].17@PAGE
Lloh50:
	add	x2, x2, l_anon.[ID].17@PAGEOFF
Lloh51:
	adrp	x5, _copy_with_zone@PAGE
Lloh52:
	add	x5, x5, _copy_with_zone@PAGEOFF
	mov	w3, #1
	mov	x4, x19
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	ldr	x0, [sp]
	bl	SYM(objc2::declare::ClassBuilder::register::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #96]
	ldp	x20, x19, [sp, #80]
	ldp	x22, x21, [sp, #64]
	add	sp, sp, #112
	ret
LBB1_3:
Lloh53:
	adrp	x0, l_anon.[ID].2@PAGE
Lloh54:
	add	x0, x0, l_anon.[ID].2@PAGEOFF
Lloh55:
	adrp	x2, l_anon.[ID].4@PAGE
Lloh56:
	add	x2, x2, l_anon.[ID].4@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB1_4:
Lloh57:
	adrp	x8, l_anon.[ID].21@PAGE
Lloh58:
	add	x8, x8, l_anon.[ID].21@PAGEOFF
Lloh59:
	adrp	x9, SYM(<&str as core[CRATE_ID]::fmt::Display>::fmt, 0)@PAGE
Lloh60:
	add	x9, x9, SYM(<&str as core[CRATE_ID]::fmt::Display>::fmt, 0)@PAGEOFF
	stp	x8, x9, [sp, #48]
Lloh61:
	adrp	x8, l_anon.[ID].20@PAGE
Lloh62:
	add	x8, x8, l_anon.[ID].20@PAGEOFF
	mov	w9, #2
	stp	x8, x9, [sp]
	add	x8, sp, #48
	mov	w9, #1
	str	x8, [sp, #16]
	stp	x9, xzr, [sp, #24]
Lloh63:
	adrp	x1, l_anon.[ID].10@PAGE
Lloh64:
	add	x1, x1, l_anon.[ID].10@PAGEOFF
	mov	x0, sp
	bl	SYM(core::panicking::panic_fmt::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh0, Lloh1
	.loh AdrpAdd	Lloh51, Lloh52
	.loh AdrpAdd	Lloh49, Lloh50
	.loh AdrpLdr	Lloh47, Lloh48
	.loh AdrpAdd	Lloh45, Lloh46
	.loh AdrpAdd	Lloh43, Lloh44
	.loh AdrpLdr	Lloh41, Lloh42
	.loh AdrpAdd	Lloh39, Lloh40
	.loh AdrpLdr	Lloh37, Lloh38
	.loh AdrpAdd	Lloh35, Lloh36
	.loh AdrpAdd	Lloh33, Lloh34
	.loh AdrpLdr	Lloh31, Lloh32
	.loh AdrpAdd	Lloh29, Lloh30
	.loh AdrpLdr	Lloh27, Lloh28
	.loh AdrpAdd	Lloh25, Lloh26
	.loh AdrpLdr	Lloh23, Lloh24
	.loh AdrpAdd	Lloh21, Lloh22
	.loh AdrpLdrGotLdr	Lloh18, Lloh19, Lloh20
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpLdr	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh55, Lloh56
	.loh AdrpAdd	Lloh53, Lloh54
	.loh AdrpAdd	Lloh63, Lloh64
	.loh AdrpAdd	Lloh61, Lloh62
	.loh AdrpAdd	Lloh59, Lloh60
	.loh AdrpAdd	Lloh57, Lloh58

	.p2align	2
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	str	x8, [sp, #8]
	add	x0, sp, #8
	bl	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0}, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.p2align	2
SYM(<&str as core[CRATE_ID]::fmt::Display>::fmt, 0):
	mov	x2, x1
	ldp	x8, x1, [x0]
	mov	x0, x8
	b	SYM(<str as core::fmt::Display>::fmt::GENERATED_ID, 0)

	.globl	_get_class
	.p2align	2
_get_class:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh65:
	adrp	x8, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh66:
	add	x8, x8, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB4_3
Lloh67:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh68:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbz	x0, LBB4_4
LBB4_2:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB4_3:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh69:
	adrp	x0, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh70:
	add	x0, x0, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
Lloh71:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh72:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh73:
	adrp	x4, l_anon.[ID].10@PAGE
Lloh74:
	add	x4, x4, l_anon.[ID].10@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
Lloh75:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh76:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbnz	x0, LBB4_2
LBB4_4:
Lloh77:
	adrp	x0, l_anon.[ID].2@PAGE
Lloh78:
	add	x0, x0, l_anon.[ID].2@PAGEOFF
Lloh79:
	adrp	x2, l_anon.[ID].10@PAGE
Lloh80:
	add	x2, x2, l_anon.[ID].10@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh65, Lloh66
	.loh AdrpAdd	Lloh67, Lloh68
	.loh AdrpAdd	Lloh75, Lloh76
	.loh AdrpAdd	Lloh73, Lloh74
	.loh AdrpAdd	Lloh71, Lloh72
	.loh AdrpAdd	Lloh69, Lloh70
	.loh AdrpAdd	Lloh79, Lloh80
	.loh AdrpAdd	Lloh77, Lloh78

	.globl	_get_obj
	.p2align	2
_get_obj:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh81:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh82:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh83:
	ldr	x19, [x8]
	bl	_get_class
	mov	x1, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_msgSend
	.loh AdrpLdrGotLdr	Lloh81, Lloh82, Lloh83

	.globl	_access_ivars
	.p2align	2
_access_ivars:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	bl	_get_obj
	mov	x19, x0
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
Lloh84:
	adrp	x1, l_anon.[ID].5@PAGE
Lloh85:
	add	x1, x1, l_anon.[ID].5@PAGEOFF
Lloh86:
	adrp	x3, l_anon.[ID].6@PAGE
Lloh87:
	add	x3, x3, l_anon.[ID].6@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldrb	w20, [x19, x0]
	mov	x0, x19
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
Lloh88:
	adrp	x1, l_anon.[ID].7@PAGE
Lloh89:
	add	x1, x1, l_anon.[ID].7@PAGEOFF
Lloh90:
	adrp	x3, l_anon.[ID].8@PAGE
Lloh91:
	add	x3, x3, l_anon.[ID].8@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldr	x21, [x19, x0]
	mov	x0, x19
	bl	_objc_release
	mov	x0, x20
	mov	x1, x21
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret
	.loh AdrpAdd	Lloh90, Lloh91
	.loh AdrpAdd	Lloh88, Lloh89
	.loh AdrpAdd	Lloh86, Lloh87
	.loh AdrpAdd	Lloh84, Lloh85

	.globl	SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh92:
	adrp	x8, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh93:
	add	x8, x8, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB7_3
Lloh94:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh95:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbz	x0, LBB7_4
LBB7_2:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB7_3:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh96:
	adrp	x0, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh97:
	add	x0, x0, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
Lloh98:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh99:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh100:
	adrp	x4, l_anon.[ID].10@PAGE
Lloh101:
	add	x4, x4, l_anon.[ID].10@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
Lloh102:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh103:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbnz	x0, LBB7_2
LBB7_4:
Lloh104:
	adrp	x0, l_anon.[ID].2@PAGE
Lloh105:
	add	x0, x0, l_anon.[ID].2@PAGEOFF
Lloh106:
	adrp	x2, l_anon.[ID].10@PAGE
Lloh107:
	add	x2, x2, l_anon.[ID].10@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh92, Lloh93
	.loh AdrpAdd	Lloh94, Lloh95
	.loh AdrpAdd	Lloh102, Lloh103
	.loh AdrpAdd	Lloh100, Lloh101
	.loh AdrpAdd	Lloh98, Lloh99
	.loh AdrpAdd	Lloh96, Lloh97
	.loh AdrpAdd	Lloh106, Lloh107
	.loh AdrpAdd	Lloh104, Lloh105

	.p2align	2
SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}::__objc2_dealloc, 0):
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x1
	mov	x20, x0
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
Lloh108:
	adrp	x1, l_anon.[ID].7@PAGE
Lloh109:
	add	x1, x1, l_anon.[ID].7@PAGEOFF
Lloh110:
	adrp	x3, l_anon.[ID].8@PAGE
Lloh111:
	add	x3, x3, l_anon.[ID].8@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldr	x0, [x20, x0]
	cbz	x0, LBB8_2
	bl	_objc_release
LBB8_2:
	bl	SYM(<objc2::runtime::nsobject::NSObject as objc2::class_type::ClassType>::class::objc_static_workaround::GENERATED_ID, 0)
	stp	x20, x0, [sp]
	mov	x0, sp
	mov	x1, x19
	bl	_objc_msgSendSuper
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
	.loh AdrpAdd	Lloh110, Lloh111
	.loh AdrpAdd	Lloh108, Lloh109

	.globl	_init
	.p2align	2
_init:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x0
Lloh112:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh113:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh114:
	ldr	x20, [x8]
	bl	SYM(<objc2::runtime::nsobject::NSObject as objc2::class_type::ClassType>::class::objc_static_workaround::GENERATED_ID, 0)
	stp	x19, x0, [sp]
	mov	x0, sp
	mov	x1, x20
	bl	_objc_msgSendSuper
	mov	x19, x0
	cbz	x0, LBB9_2
	mov	x0, x19
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
Lloh115:
	adrp	x1, l_anon.[ID].5@PAGE
Lloh116:
	add	x1, x1, l_anon.[ID].5@PAGEOFF
Lloh117:
	adrp	x3, l_anon.[ID].6@PAGE
Lloh118:
	add	x3, x3, l_anon.[ID].6@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	mov	w8, #42
	strb	w8, [x19, x0]
	mov	x0, x19
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
Lloh119:
	adrp	x1, l_anon.[ID].7@PAGE
Lloh120:
	add	x1, x1, l_anon.[ID].7@PAGEOFF
Lloh121:
	adrp	x3, l_anon.[ID].8@PAGE
Lloh122:
	add	x3, x3, l_anon.[ID].8@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	str	xzr, [x19, x0]
LBB9_2:
	mov	x0, x19
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
	.loh AdrpLdrGotLdr	Lloh112, Lloh113, Lloh114
	.loh AdrpAdd	Lloh121, Lloh122
	.loh AdrpAdd	Lloh119, Lloh120
	.loh AdrpAdd	Lloh117, Lloh118
	.loh AdrpAdd	Lloh115, Lloh116

	.globl	_class_method
	.p2align	2
_class_method:
	ret

	.globl	_method
	.p2align	2
_method:
	ret

	.globl	_method_bool
	.p2align	2
_method_bool:
	eor	w0, w2, #0x1
	ret

	.globl	_method_id
	.p2align	2
_method_id:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x0
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
Lloh123:
	adrp	x1, l_anon.[ID].7@PAGE
Lloh124:
	add	x1, x1, l_anon.[ID].7@PAGEOFF
Lloh125:
	adrp	x3, l_anon.[ID].8@PAGE
Lloh126:
	add	x3, x3, l_anon.[ID].8@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldr	x0, [x19, x0]
	cbz	x0, LBB13_2
	bl	_objc_retain
LBB13_2:
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_autoreleaseReturnValue
	.loh AdrpAdd	Lloh125, Lloh126
	.loh AdrpAdd	Lloh123, Lloh124

	.globl	_method_id_with_param
	.p2align	2
_method_id_with_param:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x21, x2
	mov	x20, x0
	bl	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	x19, x0
	cbz	w21, LBB14_5
	mov	x0, x20
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
Lloh127:
	adrp	x1, l_anon.[ID].7@PAGE
Lloh128:
	add	x1, x1, l_anon.[ID].7@PAGEOFF
Lloh129:
	adrp	x3, l_anon.[ID].8@PAGE
Lloh130:
	add	x3, x3, l_anon.[ID].8@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldr	x0, [x20, x0]
	cbz	x0, LBB14_3
	bl	_objc_retain
	mov	x20, x0
	b	LBB14_4
LBB14_3:
	mov	x20, #0
LBB14_4:
	mov	x0, x19
	bl	_objc_release
	mov	x19, x20
LBB14_5:
	mov	x0, x19
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	b	_objc_autoreleaseReturnValue
	.loh AdrpAdd	Lloh129, Lloh130
	.loh AdrpAdd	Lloh127, Lloh128

	.globl	_copy_with_zone
	.p2align	2
_copy_with_zone:
	stp	x24, x23, [sp, #-64]!
	stp	x22, x21, [sp, #16]
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	mov	x20, x0
	bl	_get_obj
	mov	x19, x0
	cbz	x0, LBB15_5
	mov	x0, x20
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
Lloh131:
	adrp	x21, l_anon.[ID].5@PAGE
Lloh132:
	add	x21, x21, l_anon.[ID].5@PAGEOFF
Lloh133:
	adrp	x22, l_anon.[ID].6@PAGE
Lloh134:
	add	x22, x22, l_anon.[ID].6@PAGEOFF
	mov	x1, x21
	mov	w2, #4
	mov	x3, x22
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldrb	w23, [x20, x0]
	mov	x0, x19
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
	mov	x1, x21
	mov	w2, #4
	mov	x3, x22
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	strb	w23, [x19, x0]
	mov	x0, x20
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
Lloh135:
	adrp	x1, l_anon.[ID].7@PAGE
Lloh136:
	add	x1, x1, l_anon.[ID].7@PAGEOFF
Lloh137:
	adrp	x3, l_anon.[ID].8@PAGE
Lloh138:
	add	x3, x3, l_anon.[ID].8@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldr	x0, [x20, x0]
	cbz	x0, LBB15_3
	bl	_objc_retain
	mov	x20, x0
	b	LBB15_4
LBB15_3:
	mov	x20, #0
LBB15_4:
	mov	x0, x19
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
Lloh139:
	adrp	x1, l_anon.[ID].7@PAGE
Lloh140:
	add	x1, x1, l_anon.[ID].7@PAGEOFF
Lloh141:
	adrp	x3, l_anon.[ID].8@PAGE
Lloh142:
	add	x3, x3, l_anon.[ID].8@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	str	x20, [x19, x0]
LBB15_5:
	mov	x0, x19
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	ldp	x22, x21, [sp, #16]
	ldp	x24, x23, [sp], #64
	ret
	.loh AdrpAdd	Lloh137, Lloh138
	.loh AdrpAdd	Lloh135, Lloh136
	.loh AdrpAdd	Lloh133, Lloh134
	.loh AdrpAdd	Lloh131, Lloh132
	.loh AdrpAdd	Lloh141, Lloh142
	.loh AdrpAdd	Lloh139, Lloh140

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.quad	SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0}>, 0)
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].1:
	.byte	0

l_anon.[ID].2:
	.ascii	"called `Option::unwrap()` on a `None` value"

l_anon.[ID].3:
	.ascii	"/Users/madsmarquart/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].4:
	.quad	l_anon.[ID].3
	.asciz	"t\000\000\000\000\000\000\000\225\000\000\0002\000\000"

	.section	__TEXT,__literal4,4byte_literals
l_anon.[ID].5:
	.ascii	"_foo"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].6:
	.byte	5
	.space	39

	.section	__TEXT,__literal4,4byte_literals
l_anon.[ID].7:
	.ascii	"_obj"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].8:
	.byte	19
	.space	39

l_anon.[ID].9:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].10:
	.quad	l_anon.[ID].9
	.asciz	"5\000\000\000\000\000\000\000\013\000\000\000\001\000\000"

	.section	__TEXT,__const
l_anon.[ID].11:
	.ascii	"CustomClassName"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0),8,3
	.p2align	3, 0x0
l_anon.[ID].12:
	.byte	17
	.space	39

	.p2align	3, 0x0
l_anon.[ID].13:
	.byte	16
	.space	39

l_anon.[ID].14:
	.ascii	"NSCopying"

l_anon.[ID].15:
	.ascii	"_NSZone"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].16:
	.byte	28
	.space	7
	.quad	l_anon.[ID].15
	.asciz	"\007\000\000\000\000\000\000"
	.quad	l_anon.[ID].1
	.space	8

	.p2align	3, 0x0
l_anon.[ID].17:
	.byte	25
	.space	7
	.quad	l_anon.[ID].16
	.space	24

	.section	__TEXT,__const
l_anon.[ID].18:
	.ascii	"could not create new class "

l_anon.[ID].19:
	.ascii	". Perhaps a class with that name already exists?"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].20:
	.quad	l_anon.[ID].18
	.asciz	"\033\000\000\000\000\000\000"
	.quad	l_anon.[ID].19
	.asciz	"0\000\000\000\000\000\000"

	.p2align	3, 0x0
l_anon.[ID].21:
	.quad	l_anon.[ID].11
	.asciz	"\017\000\000\000\000\000\000"

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_694e247a0bc88869
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_694e247a0bc88869:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_694e247a0bc88869
L_OBJC_METH_VAR_NAME_694e247a0bc88869:
	.asciz	"dealloc"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_694e247a0bc88869
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_694e247a0bc88869:
	.quad	L_OBJC_METH_VAR_NAME_694e247a0bc88869

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_8dd788dbcc16b9bc
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_8dd788dbcc16b9bc:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_8dd788dbcc16b9bc
L_OBJC_METH_VAR_NAME_8dd788dbcc16b9bc:
	.asciz	"classMethod"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_8dd788dbcc16b9bc
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_8dd788dbcc16b9bc:
	.quad	L_OBJC_METH_VAR_NAME_8dd788dbcc16b9bc

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_450db9db0953dff5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_450db9db0953dff5:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_450db9db0953dff5
L_OBJC_METH_VAR_NAME_450db9db0953dff5:
	.asciz	"method"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_450db9db0953dff5
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_450db9db0953dff5:
	.quad	L_OBJC_METH_VAR_NAME_450db9db0953dff5

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_783b35bc45c6e4a6
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_783b35bc45c6e4a6:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_783b35bc45c6e4a6
L_OBJC_METH_VAR_NAME_783b35bc45c6e4a6:
	.asciz	"methodBool:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_783b35bc45c6e4a6
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_783b35bc45c6e4a6:
	.quad	L_OBJC_METH_VAR_NAME_783b35bc45c6e4a6

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_828e9fbc6d0b4498
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_828e9fbc6d0b4498:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_828e9fbc6d0b4498
L_OBJC_METH_VAR_NAME_828e9fbc6d0b4498:
	.asciz	"methodId"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_828e9fbc6d0b4498
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_828e9fbc6d0b4498:
	.quad	L_OBJC_METH_VAR_NAME_828e9fbc6d0b4498

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_788cc14ba6a28eb8
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_788cc14ba6a28eb8:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_788cc14ba6a28eb8
L_OBJC_METH_VAR_NAME_788cc14ba6a28eb8:
	.asciz	"methodIdWithParam:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_788cc14ba6a28eb8
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_788cc14ba6a28eb8:
	.quad	L_OBJC_METH_VAR_NAME_788cc14ba6a28eb8

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f058a81939de2cb9
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f058a81939de2cb9:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_f058a81939de2cb9
L_OBJC_METH_VAR_NAME_f058a81939de2cb9:
	.asciz	"copyWithZone:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_f058a81939de2cb9
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_f058a81939de2cb9:
	.quad	L_OBJC_METH_VAR_NAME_f058a81939de2cb9

.subsections_via_symbols
