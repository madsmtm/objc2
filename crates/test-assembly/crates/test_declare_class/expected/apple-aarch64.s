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
Lloh0:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh1:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh2:
	ldr	x2, [x8]
Lloh3:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh4:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB1_4
	str	x0, [sp]
Lloh5:
	adrp	x1, l_anon.[ID].11@PAGE
Lloh6:
	add	x1, x1, l_anon.[ID].11@PAGEOFF
Lloh7:
	adrp	x5, l_anon.[ID].12@PAGE
Lloh8:
	add	x5, x5, l_anon.[ID].12@PAGEOFF
	mov	x0, sp
	mov	w2, #4
	mov	w3, #1
	mov	w4, #0
	bl	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Lloh9:
	adrp	x1, l_anon.[ID].13@PAGE
Lloh10:
	add	x1, x1, l_anon.[ID].13@PAGEOFF
Lloh11:
	adrp	x19, l_anon.[ID].2@PAGE
Lloh12:
	add	x19, x19, l_anon.[ID].2@PAGEOFF
	mov	x0, sp
	mov	w2, #4
	mov	w3, #8
	mov	w4, #3
	mov	x5, x19
	bl	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Lloh13:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGE
Lloh14:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPAGEOFF]
Lloh15:
	ldr	x1, [x8]
Lloh16:
	adrp	x20, l_anon.[ID].1@PAGE
Lloh17:
	add	x20, x20, l_anon.[ID].1@PAGEOFF
Lloh18:
	adrp	x21, l_anon.[ID].3@PAGE
Lloh19:
	add	x21, x21, l_anon.[ID].3@PAGEOFF
Lloh20:
	adrp	x5, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}::__objc2_dealloc, 0)@PAGE
Lloh21:
	add	x5, x5, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}::__objc2_dealloc, 0)@PAGEOFF
	mov	x0, sp
	mov	x2, x20
	mov	x3, #0
	mov	x4, x21
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh22:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh23:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh24:
	ldr	x1, [x8]
Lloh25:
	adrp	x5, _init@PAGE
Lloh26:
	add	x5, x5, _init@PAGEOFF
	mov	x0, sp
	mov	x2, x20
	mov	x3, #0
	mov	x4, x19
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh27:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_d874ee9262978be2@PAGE
Lloh28:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_d874ee9262978be2@PAGEOFF]
Lloh29:
	adrp	x5, _class_method@PAGE
Lloh30:
	add	x5, x5, _class_method@PAGEOFF
	mov	x0, sp
	mov	x2, x20
	mov	x3, #0
	mov	x4, x21
	bl	SYM(objc2::declare::ClassBuilder::add_class_method_inner::GENERATED_ID, 0)
Lloh31:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_4539fd1dbda0cddc@PAGE
Lloh32:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_4539fd1dbda0cddc@PAGEOFF]
Lloh33:
	adrp	x5, _method@PAGE
Lloh34:
	add	x5, x5, _method@PAGEOFF
	mov	x0, sp
	mov	x2, x20
	mov	x3, #0
	mov	x4, x21
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh35:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_2b1b3a94e0ece2e5@PAGE
Lloh36:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_2b1b3a94e0ece2e5@PAGEOFF]
Lloh37:
	adrp	x21, l_anon.[ID].4@PAGE
Lloh38:
	add	x21, x21, l_anon.[ID].4@PAGEOFF
Lloh39:
	adrp	x5, _method_bool@PAGE
Lloh40:
	add	x5, x5, _method_bool@PAGEOFF
	mov	x0, sp
	mov	x2, x21
	mov	w3, #1
	mov	x4, x21
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh41:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_f7f521670860b0ce@PAGE
Lloh42:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_f7f521670860b0ce@PAGEOFF]
Lloh43:
	adrp	x5, _method_id@PAGE
Lloh44:
	add	x5, x5, _method_id@PAGEOFF
	mov	x0, sp
	mov	x2, x20
	mov	x3, #0
	mov	x4, x19
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh45:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_6addfcf634c6232f@PAGE
Lloh46:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_6addfcf634c6232f@PAGEOFF]
Lloh47:
	adrp	x5, _method_id_with_param@PAGE
Lloh48:
	add	x5, x5, _method_id_with_param@PAGEOFF
	mov	x0, sp
	mov	x2, x21
	mov	w3, #1
	mov	x4, x19
	bl	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Lloh49:
	adrp	x0, l_anon.[ID].17@PAGE
Lloh50:
	add	x0, x0, l_anon.[ID].17@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	mov	x1, x0
	mov	x0, sp
	bl	SYM(objc2::__macro_helpers::<impl objc2::declare::ClassBuilder>::__add_protocol_methods::GENERATED_ID, 0)
Lloh51:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_4a8c690dbc9d8166@PAGE
Lloh52:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_4a8c690dbc9d8166@PAGEOFF]
Lloh53:
	adrp	x2, l_anon.[ID].7@PAGE
Lloh54:
	add	x2, x2, l_anon.[ID].7@PAGEOFF
Lloh55:
	adrp	x5, _copyWithZone@PAGE
Lloh56:
	add	x5, x5, _copyWithZone@PAGEOFF
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
Lloh57:
	adrp	x0, l_anon.[ID].8@PAGE
Lloh58:
	add	x0, x0, l_anon.[ID].8@PAGEOFF
Lloh59:
	adrp	x2, l_anon.[ID].10@PAGE
Lloh60:
	add	x2, x2, l_anon.[ID].10@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB1_4:
Lloh61:
	adrp	x8, l_anon.[ID].21@PAGE
Lloh62:
	add	x8, x8, l_anon.[ID].21@PAGEOFF
Lloh63:
	adrp	x9, SYM(<&str as core[CRATE_ID]::fmt::Display>::fmt, 0)@PAGE
Lloh64:
	add	x9, x9, SYM(<&str as core[CRATE_ID]::fmt::Display>::fmt, 0)@PAGEOFF
	stp	x8, x9, [sp, #48]
Lloh65:
	adrp	x8, l_anon.[ID].20@PAGE
Lloh66:
	add	x8, x8, l_anon.[ID].20@PAGEOFF
	mov	w9, #2
	stp	x8, x9, [sp]
	add	x8, sp, #48
	mov	w9, #1
	str	x8, [sp, #16]
	stp	x9, xzr, [sp, #24]
Lloh67:
	adrp	x1, l_anon.[ID].15@PAGE
Lloh68:
	add	x1, x1, l_anon.[ID].15@PAGEOFF
	mov	x0, sp
	bl	SYM(core::panicking::panic_fmt::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh3, Lloh4
	.loh AdrpLdrGotLdr	Lloh0, Lloh1, Lloh2
	.loh AdrpAdd	Lloh55, Lloh56
	.loh AdrpAdd	Lloh53, Lloh54
	.loh AdrpLdr	Lloh51, Lloh52
	.loh AdrpAdd	Lloh49, Lloh50
	.loh AdrpAdd	Lloh47, Lloh48
	.loh AdrpLdr	Lloh45, Lloh46
	.loh AdrpAdd	Lloh43, Lloh44
	.loh AdrpLdr	Lloh41, Lloh42
	.loh AdrpAdd	Lloh39, Lloh40
	.loh AdrpAdd	Lloh37, Lloh38
	.loh AdrpLdr	Lloh35, Lloh36
	.loh AdrpAdd	Lloh33, Lloh34
	.loh AdrpLdr	Lloh31, Lloh32
	.loh AdrpAdd	Lloh29, Lloh30
	.loh AdrpLdr	Lloh27, Lloh28
	.loh AdrpAdd	Lloh25, Lloh26
	.loh AdrpLdrGotLdr	Lloh22, Lloh23, Lloh24
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpAdd	Lloh18, Lloh19
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpLdrGotLdr	Lloh13, Lloh14, Lloh15
	.loh AdrpAdd	Lloh11, Lloh12
	.loh AdrpAdd	Lloh9, Lloh10
	.loh AdrpAdd	Lloh7, Lloh8
	.loh AdrpAdd	Lloh5, Lloh6
	.loh AdrpAdd	Lloh59, Lloh60
	.loh AdrpAdd	Lloh57, Lloh58
	.loh AdrpAdd	Lloh67, Lloh68
	.loh AdrpAdd	Lloh65, Lloh66
	.loh AdrpAdd	Lloh63, Lloh64
	.loh AdrpAdd	Lloh61, Lloh62

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
Lloh69:
	adrp	x8, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh70:
	add	x8, x8, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB4_3
Lloh71:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh72:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
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
Lloh73:
	adrp	x0, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh74:
	add	x0, x0, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
Lloh75:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh76:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh77:
	adrp	x4, l_anon.[ID].15@PAGE
Lloh78:
	add	x4, x4, l_anon.[ID].15@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
Lloh79:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh80:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbnz	x0, LBB4_2
LBB4_4:
Lloh81:
	adrp	x0, l_anon.[ID].8@PAGE
Lloh82:
	add	x0, x0, l_anon.[ID].8@PAGEOFF
Lloh83:
	adrp	x2, l_anon.[ID].15@PAGE
Lloh84:
	add	x2, x2, l_anon.[ID].15@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh69, Lloh70
	.loh AdrpAdd	Lloh71, Lloh72
	.loh AdrpAdd	Lloh79, Lloh80
	.loh AdrpAdd	Lloh77, Lloh78
	.loh AdrpAdd	Lloh75, Lloh76
	.loh AdrpAdd	Lloh73, Lloh74
	.loh AdrpAdd	Lloh83, Lloh84
	.loh AdrpAdd	Lloh81, Lloh82

	.globl	_get_obj
	.p2align	2
_get_obj:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh85:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh86:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh87:
	ldr	x19, [x8]
	bl	_get_class
	mov	x1, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_msgSend
	.loh AdrpLdrGotLdr	Lloh85, Lloh86, Lloh87

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
Lloh88:
	adrp	x1, l_anon.[ID].11@PAGE
Lloh89:
	add	x1, x1, l_anon.[ID].11@PAGEOFF
Lloh90:
	adrp	x3, l_anon.[ID].12@PAGE
Lloh91:
	add	x3, x3, l_anon.[ID].12@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldrb	w20, [x19, x0]
	mov	x0, x19
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
Lloh92:
	adrp	x1, l_anon.[ID].13@PAGE
Lloh93:
	add	x1, x1, l_anon.[ID].13@PAGEOFF
Lloh94:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh95:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
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
	.loh AdrpAdd	Lloh94, Lloh95
	.loh AdrpAdd	Lloh92, Lloh93
	.loh AdrpAdd	Lloh90, Lloh91
	.loh AdrpAdd	Lloh88, Lloh89

	.globl	SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh96:
	adrp	x8, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh97:
	add	x8, x8, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB7_3
Lloh98:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh99:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
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
Lloh100:
	adrp	x0, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh101:
	add	x0, x0, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
Lloh102:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh103:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh104:
	adrp	x4, l_anon.[ID].15@PAGE
Lloh105:
	add	x4, x4, l_anon.[ID].15@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
Lloh106:
	adrp	x0, l_anon.[ID].16@PAGE
Lloh107:
	add	x0, x0, l_anon.[ID].16@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbnz	x0, LBB7_2
LBB7_4:
Lloh108:
	adrp	x0, l_anon.[ID].8@PAGE
Lloh109:
	add	x0, x0, l_anon.[ID].8@PAGEOFF
Lloh110:
	adrp	x2, l_anon.[ID].15@PAGE
Lloh111:
	add	x2, x2, l_anon.[ID].15@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh96, Lloh97
	.loh AdrpAdd	Lloh98, Lloh99
	.loh AdrpAdd	Lloh106, Lloh107
	.loh AdrpAdd	Lloh104, Lloh105
	.loh AdrpAdd	Lloh102, Lloh103
	.loh AdrpAdd	Lloh100, Lloh101
	.loh AdrpAdd	Lloh110, Lloh111
	.loh AdrpAdd	Lloh108, Lloh109

	.p2align	2
SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::{closure#0}::__objc2_dealloc, 0):
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x1
	mov	x20, x0
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
Lloh112:
	adrp	x1, l_anon.[ID].13@PAGE
Lloh113:
	add	x1, x1, l_anon.[ID].13@PAGEOFF
Lloh114:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh115:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldr	x0, [x20, x0]
	cbz	x0, LBB8_2
	bl	_objc_release
LBB8_2:
Lloh116:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh117:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh118:
	ldr	x8, [x8]
	stp	x20, x8, [sp]
	mov	x0, sp
	mov	x1, x19
	bl	_objc_msgSendSuper
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
	.loh AdrpAdd	Lloh114, Lloh115
	.loh AdrpAdd	Lloh112, Lloh113
	.loh AdrpLdrGotLdr	Lloh116, Lloh117, Lloh118

	.globl	_init
	.p2align	2
_init:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
Lloh119:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh120:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh121:
	ldr	x1, [x8]
Lloh122:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh123:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh124:
	ldr	x8, [x8]
	stp	x0, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	mov	x19, x0
	cbz	x0, LBB9_2
	mov	x0, x19
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
Lloh125:
	adrp	x1, l_anon.[ID].11@PAGE
Lloh126:
	add	x1, x1, l_anon.[ID].11@PAGEOFF
Lloh127:
	adrp	x3, l_anon.[ID].12@PAGE
Lloh128:
	add	x3, x3, l_anon.[ID].12@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	mov	w8, #42
	strb	w8, [x19, x0]
	mov	x0, x19
	bl	SYM(objc2::runtime::AnyObject::class::GENERATED_ID, 0)
Lloh129:
	adrp	x1, l_anon.[ID].13@PAGE
Lloh130:
	add	x1, x1, l_anon.[ID].13@PAGEOFF
Lloh131:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh132:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	str	xzr, [x19, x0]
LBB9_2:
	mov	x0, x19
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
	.loh AdrpLdrGotLdr	Lloh122, Lloh123, Lloh124
	.loh AdrpLdrGotLdr	Lloh119, Lloh120, Lloh121
	.loh AdrpAdd	Lloh131, Lloh132
	.loh AdrpAdd	Lloh129, Lloh130
	.loh AdrpAdd	Lloh127, Lloh128
	.loh AdrpAdd	Lloh125, Lloh126

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
Lloh133:
	adrp	x1, l_anon.[ID].13@PAGE
Lloh134:
	add	x1, x1, l_anon.[ID].13@PAGEOFF
Lloh135:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh136:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	ldr	x0, [x19, x0]
	cbz	x0, LBB13_2
	bl	_objc_retain
LBB13_2:
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_autoreleaseReturnValue
	.loh AdrpAdd	Lloh135, Lloh136
	.loh AdrpAdd	Lloh133, Lloh134

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
Lloh137:
	adrp	x1, l_anon.[ID].13@PAGE
Lloh138:
	add	x1, x1, l_anon.[ID].13@PAGEOFF
Lloh139:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh140:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
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
	.loh AdrpAdd	Lloh139, Lloh140
	.loh AdrpAdd	Lloh137, Lloh138

	.globl	_copyWithZone
	.p2align	2
_copyWithZone:
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
Lloh141:
	adrp	x21, l_anon.[ID].11@PAGE
Lloh142:
	add	x21, x21, l_anon.[ID].11@PAGEOFF
Lloh143:
	adrp	x22, l_anon.[ID].12@PAGE
Lloh144:
	add	x22, x22, l_anon.[ID].12@PAGEOFF
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
Lloh145:
	adrp	x1, l_anon.[ID].13@PAGE
Lloh146:
	add	x1, x1, l_anon.[ID].13@PAGEOFF
Lloh147:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh148:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
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
Lloh149:
	adrp	x1, l_anon.[ID].13@PAGE
Lloh150:
	add	x1, x1, l_anon.[ID].13@PAGEOFF
Lloh151:
	adrp	x3, l_anon.[ID].2@PAGE
Lloh152:
	add	x3, x3, l_anon.[ID].2@PAGEOFF
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
	.loh AdrpAdd	Lloh147, Lloh148
	.loh AdrpAdd	Lloh145, Lloh146
	.loh AdrpAdd	Lloh143, Lloh144
	.loh AdrpAdd	Lloh141, Lloh142
	.loh AdrpAdd	Lloh151, Lloh152
	.loh AdrpAdd	Lloh149, Lloh150

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

	.p2align	3, 0x0
l_anon.[ID].2:
	.byte	19
	.space	39

	.p2align	3, 0x0
l_anon.[ID].3:
	.byte	17
	.space	39

	.p2align	3, 0x0
l_anon.[ID].4:
	.byte	16
	.space	39

l_anon.[ID].5:
	.ascii	"_NSZone"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].6:
	.byte	28
	.space	7
	.quad	l_anon.[ID].5
	.asciz	"\007\000\000\000\000\000\000"
	.quad	l_anon.[ID].1
	.space	8

	.p2align	3, 0x0
l_anon.[ID].7:
	.byte	25
	.space	7
	.quad	l_anon.[ID].6
	.space	24

	.section	__TEXT,__const
l_anon.[ID].8:
	.ascii	"called `Option::unwrap()` on a `None` value"

l_anon.[ID].9:
	.ascii	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].10:
	.quad	l_anon.[ID].9
	.asciz	"p\000\000\000\000\000\000\000\225\000\000\0002\000\000"

	.section	__TEXT,__literal4,4byte_literals
l_anon.[ID].11:
	.ascii	"_foo"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].12:
	.byte	5
	.space	39

	.section	__TEXT,__literal4,4byte_literals
l_anon.[ID].13:
	.ascii	"_obj"

	.section	__TEXT,__const
l_anon.[ID].14:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].15:
	.quad	l_anon.[ID].14
	.asciz	"5\000\000\000\000\000\000\000\f\000\000\000\001\000\000"

	.section	__TEXT,__const
l_anon.[ID].16:
	.ascii	"CustomClassName"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::class_type::ClassType>::class::REGISTER_CLASS, 0),8,3
l_anon.[ID].17:
	.ascii	"NSCopying"

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
	.quad	l_anon.[ID].16
	.asciz	"\017\000\000\000\000\000\000"

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_d874ee9262978be2
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_d874ee9262978be2:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_d874ee9262978be2
L_OBJC_METH_VAR_NAME_d874ee9262978be2:
	.asciz	"classMethod"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_d874ee9262978be2
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_d874ee9262978be2:
	.quad	L_OBJC_METH_VAR_NAME_d874ee9262978be2

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_4539fd1dbda0cddc
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_4539fd1dbda0cddc:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_4539fd1dbda0cddc
L_OBJC_METH_VAR_NAME_4539fd1dbda0cddc:
	.asciz	"method"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_4539fd1dbda0cddc
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_4539fd1dbda0cddc:
	.quad	L_OBJC_METH_VAR_NAME_4539fd1dbda0cddc

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2b1b3a94e0ece2e5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2b1b3a94e0ece2e5:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2b1b3a94e0ece2e5
L_OBJC_METH_VAR_NAME_2b1b3a94e0ece2e5:
	.asciz	"methodBool:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2b1b3a94e0ece2e5
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_2b1b3a94e0ece2e5:
	.quad	L_OBJC_METH_VAR_NAME_2b1b3a94e0ece2e5

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f7f521670860b0ce
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f7f521670860b0ce:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_f7f521670860b0ce
L_OBJC_METH_VAR_NAME_f7f521670860b0ce:
	.asciz	"methodId"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_f7f521670860b0ce
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_f7f521670860b0ce:
	.quad	L_OBJC_METH_VAR_NAME_f7f521670860b0ce

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_6addfcf634c6232f
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_6addfcf634c6232f:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6addfcf634c6232f
L_OBJC_METH_VAR_NAME_6addfcf634c6232f:
	.asciz	"methodIdWithParam:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_6addfcf634c6232f
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_6addfcf634c6232f:
	.quad	L_OBJC_METH_VAR_NAME_6addfcf634c6232f

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_4a8c690dbc9d8166
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_4a8c690dbc9d8166:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_4a8c690dbc9d8166
L_OBJC_METH_VAR_NAME_4a8c690dbc9d8166:
	.asciz	"copyWithZone:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_4a8c690dbc9d8166
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_4a8c690dbc9d8166:
	.quad	L_OBJC_METH_VAR_NAME_4a8c690dbc9d8166

.subsections_via_symbols
