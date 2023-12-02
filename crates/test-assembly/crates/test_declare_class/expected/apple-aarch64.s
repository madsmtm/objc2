	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}>, 0):
	ret

	.p2align	2
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
	sub	sp, sp, #64
	stp	x22, x21, [sp, #16]
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	cbz	w9, LBB1_5
Lloh0:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh1:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh2:
	ldr	x2, [x8]
Lloh3:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh4:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
	cbz	x0, LBB1_6
	str	x0, [sp, #8]
Lloh5:
	adrp	x1, l_anon.[ID].12@PAGE
Lloh6:
	add	x1, x1, l_anon.[ID].12@PAGEOFF
Lloh7:
	adrp	x5, l_anon.[ID].15@PAGE
Lloh8:
	add	x5, x5, l_anon.[ID].15@PAGEOFF
	add	x0, sp, #8
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
	add	x0, sp, #8
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
	adrp	x5, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::__objc2_dealloc, 0)@PAGE
Lloh21:
	add	x5, x5, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::__objc2_dealloc, 0)@PAGEOFF
	add	x0, sp, #8
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
	add	x0, sp, #8
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
	add	x0, sp, #8
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
	add	x0, sp, #8
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
	add	x0, sp, #8
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
	add	x0, sp, #8
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
	add	x0, sp, #8
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
	cbz	x0, LBB1_4
	mov	x1, x0
	add	x0, sp, #8
	bl	SYM(objc2::declare::ClassBuilder::add_protocol::GENERATED_ID, 0)
LBB1_4:
Lloh51:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_4a8c690dbc9d8166@PAGE
Lloh52:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_4a8c690dbc9d8166@PAGEOFF]
Lloh53:
	adrp	x2, l_anon.[ID].7@PAGE
Lloh54:
	add	x2, x2, l_anon.[ID].7@PAGEOFF
Lloh55:
	adrp	x4, l_anon.[ID].2@PAGE
Lloh56:
	add	x4, x4, l_anon.[ID].2@PAGEOFF
Lloh57:
	adrp	x5, _copyWithZone@PAGE
Lloh58:
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
LBB1_5:
Lloh59:
	adrp	x0, l_anon.[ID].8@PAGE
Lloh60:
	add	x0, x0, l_anon.[ID].8@PAGEOFF
Lloh61:
	adrp	x2, l_anon.[ID].10@PAGE
Lloh62:
	add	x2, x2, l_anon.[ID].10@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB1_6:
Lloh63:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh64:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
Lloh65:
	adrp	x2, l_anon.[ID].16@PAGE
Lloh66:
	add	x2, x2, l_anon.[ID].16@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh3, Lloh4
	.loh AdrpLdrGotLdr	Lloh0, Lloh1, Lloh2
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
	.loh AdrpAdd	Lloh57, Lloh58
	.loh AdrpAdd	Lloh55, Lloh56
	.loh AdrpAdd	Lloh53, Lloh54
	.loh AdrpLdr	Lloh51, Lloh52
	.loh AdrpAdd	Lloh61, Lloh62
	.loh AdrpAdd	Lloh59, Lloh60
	.loh AdrpAdd	Lloh65, Lloh66
	.loh AdrpAdd	Lloh63, Lloh64

	.p2align	2
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	str	x8, [sp, #8]
	add	x0, sp, #8
	bl	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.globl	_get_class
	.p2align	2
_get_class:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh67:
	adrp	x8, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh68:
	add	x8, x8, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB3_3
Lloh69:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh70:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbz	x0, LBB3_4
LBB3_2:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB3_3:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh71:
	adrp	x0, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh72:
	add	x0, x0, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
Lloh73:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh74:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh75:
	adrp	x4, l_anon.[ID].16@PAGE
Lloh76:
	add	x4, x4, l_anon.[ID].16@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
Lloh77:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh78:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbnz	x0, LBB3_2
LBB3_4:
Lloh79:
	adrp	x0, l_anon.[ID].8@PAGE
Lloh80:
	add	x0, x0, l_anon.[ID].8@PAGEOFF
Lloh81:
	adrp	x2, l_anon.[ID].16@PAGE
Lloh82:
	add	x2, x2, l_anon.[ID].16@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh67, Lloh68
	.loh AdrpAdd	Lloh69, Lloh70
	.loh AdrpAdd	Lloh77, Lloh78
	.loh AdrpAdd	Lloh75, Lloh76
	.loh AdrpAdd	Lloh73, Lloh74
	.loh AdrpAdd	Lloh71, Lloh72
	.loh AdrpAdd	Lloh81, Lloh82
	.loh AdrpAdd	Lloh79, Lloh80

	.globl	_get_obj
	.p2align	2
_get_obj:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_get_class
Lloh83:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh84:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh85:
	ldr	x1, [x8]
	ldp	x29, x30, [sp], #16
	b	_objc_msgSend
	.loh AdrpLdrGotLdr	Lloh83, Lloh84, Lloh85

	.globl	_access_ivars
	.p2align	2
_access_ivars:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	bl	_get_obj
	mov	x19, x0
Lloh86:
	adrp	x1, l_anon.[ID].12@PAGE
Lloh87:
	add	x1, x1, l_anon.[ID].12@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	ldrb	w20, [x19, x0]
Lloh88:
	adrp	x1, l_anon.[ID].13@PAGE
Lloh89:
	add	x1, x1, l_anon.[ID].13@PAGEOFF
	mov	x0, x19
	mov	w2, #4
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	ldr	x21, [x19, x0]
	mov	x0, x19
	bl	_objc_release
	mov	x0, x20
	mov	x1, x21
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret
	.loh AdrpAdd	Lloh88, Lloh89
	.loh AdrpAdd	Lloh86, Lloh87

	.globl	SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh90:
	adrp	x8, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh91:
	add	x8, x8, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB6_3
Lloh92:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh93:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbz	x0, LBB6_4
LBB6_2:
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB6_3:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh94:
	adrp	x0, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGE
Lloh95:
	add	x0, x0, SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)@PAGEOFF
Lloh96:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh97:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh98:
	adrp	x4, l_anon.[ID].16@PAGE
Lloh99:
	add	x4, x4, l_anon.[ID].16@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
Lloh100:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh101:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
	mov	w1, #15
	bl	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	cbnz	x0, LBB6_2
LBB6_4:
Lloh102:
	adrp	x0, l_anon.[ID].8@PAGE
Lloh103:
	add	x0, x0, l_anon.[ID].8@PAGEOFF
Lloh104:
	adrp	x2, l_anon.[ID].16@PAGE
Lloh105:
	add	x2, x2, l_anon.[ID].16@PAGEOFF
	mov	w1, #43
	bl	SYM(core::panicking::panic::GENERATED_ID, 0)
	.loh AdrpAdd	Lloh90, Lloh91
	.loh AdrpAdd	Lloh92, Lloh93
	.loh AdrpAdd	Lloh100, Lloh101
	.loh AdrpAdd	Lloh98, Lloh99
	.loh AdrpAdd	Lloh96, Lloh97
	.loh AdrpAdd	Lloh94, Lloh95
	.loh AdrpAdd	Lloh104, Lloh105
	.loh AdrpAdd	Lloh102, Lloh103

	.p2align	2
SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::__objc2_dealloc, 0):
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x19, x1
	mov	x20, x0
Lloh106:
	adrp	x1, l_anon.[ID].13@PAGE
Lloh107:
	add	x1, x1, l_anon.[ID].13@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	ldr	x0, [x20, x0]
	cbz	x0, LBB7_2
	bl	_objc_release
LBB7_2:
Lloh108:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh109:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh110:
	ldr	x8, [x8]
	stp	x20, x8, [sp]
	mov	x0, sp
	mov	x1, x19
	bl	_objc_msgSendSuper
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
	.loh AdrpAdd	Lloh106, Lloh107
	.loh AdrpLdrGotLdr	Lloh108, Lloh109, Lloh110

	.globl	_init
	.p2align	2
_init:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
Lloh111:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh112:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh113:
	ldr	x1, [x8]
Lloh114:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh115:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh116:
	ldr	x8, [x8]
	stp	x0, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	mov	x19, x0
	cbz	x0, LBB8_2
Lloh117:
	adrp	x1, l_anon.[ID].12@PAGE
Lloh118:
	add	x1, x1, l_anon.[ID].12@PAGEOFF
	mov	x0, x19
	mov	w2, #4
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	mov	w8, #42
	strb	w8, [x19, x0]
Lloh119:
	adrp	x1, l_anon.[ID].13@PAGE
Lloh120:
	add	x1, x1, l_anon.[ID].13@PAGEOFF
	mov	x0, x19
	mov	w2, #4
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	str	xzr, [x19, x0]
LBB8_2:
	mov	x0, x19
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
	.loh AdrpLdrGotLdr	Lloh114, Lloh115, Lloh116
	.loh AdrpLdrGotLdr	Lloh111, Lloh112, Lloh113
	.loh AdrpAdd	Lloh119, Lloh120
	.loh AdrpAdd	Lloh117, Lloh118

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
Lloh121:
	adrp	x1, l_anon.[ID].13@PAGE
Lloh122:
	add	x1, x1, l_anon.[ID].13@PAGEOFF
	mov	w2, #4
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	ldr	x0, [x19, x0]
	cbz	x0, LBB12_2
	bl	_objc_retain
LBB12_2:
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_autoreleaseReturnValue
	.loh AdrpAdd	Lloh121, Lloh122

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
	cbz	w21, LBB13_5
Lloh123:
	adrp	x1, l_anon.[ID].13@PAGE
Lloh124:
	add	x1, x1, l_anon.[ID].13@PAGEOFF
	mov	x0, x20
	mov	w2, #4
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	ldr	x0, [x20, x0]
	cbz	x0, LBB13_3
	bl	_objc_retain
	mov	x20, x0
	b	LBB13_4
LBB13_3:
	mov	x20, #0
LBB13_4:
	mov	x0, x19
	bl	_objc_release
	mov	x19, x20
LBB13_5:
	mov	x0, x19
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	b	_objc_autoreleaseReturnValue
	.loh AdrpAdd	Lloh123, Lloh124

	.globl	_copyWithZone
	.p2align	2
_copyWithZone:
	stp	x22, x21, [sp, #-48]!
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	mov	x20, x0
	bl	_get_obj
	mov	x19, x0
	cbz	x0, LBB14_5
Lloh125:
	adrp	x21, l_anon.[ID].12@PAGE
Lloh126:
	add	x21, x21, l_anon.[ID].12@PAGEOFF
	mov	x0, x20
	mov	x1, x21
	mov	w2, #4
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	ldrb	w22, [x20, x0]
	mov	x0, x19
	mov	x1, x21
	mov	w2, #4
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	strb	w22, [x19, x0]
Lloh127:
	adrp	x1, l_anon.[ID].13@PAGE
Lloh128:
	add	x1, x1, l_anon.[ID].13@PAGEOFF
	mov	x0, x20
	mov	w2, #4
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	ldr	x0, [x20, x0]
	cbz	x0, LBB14_3
	bl	_objc_retain
	mov	x20, x0
	b	LBB14_4
LBB14_3:
	mov	x20, #0
LBB14_4:
Lloh129:
	adrp	x1, l_anon.[ID].13@PAGE
Lloh130:
	add	x1, x1, l_anon.[ID].13@PAGEOFF
	mov	x0, x19
	mov	w2, #4
	bl	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	bl	_ivar_getOffset
	str	x20, [x19, x0]
LBB14_5:
	mov	x0, x19
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	ret
	.loh AdrpAdd	Lloh127, Lloh128
	.loh AdrpAdd	Lloh125, Lloh126
	.loh AdrpAdd	Lloh129, Lloh130

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.quad	SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}>, 0)
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

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

	.section	__TEXT,__const
l_anon.[ID].11:
	.ascii	"CustomClassName"

	.section	__TEXT,__literal4,4byte_literals
l_anon.[ID].12:
	.ascii	"_foo"

l_anon.[ID].13:
	.ascii	"_obj"

	.section	__TEXT,__const
l_anon.[ID].14:
	.ascii	"crates/$DIR/lib.rs"

	.p2align	3, 0x0
l_anon.[ID].15:
	.byte	5
	.space	39

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].16:
	.quad	l_anon.[ID].14
	.asciz	"5\000\000\000\000\000\000\000\f\000\000\000\001\000\000"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0),8,3
	.section	__TEXT,__const
l_anon.[ID].17:
	.ascii	"NSCopying"

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
	.globl	L_OBJC_IMAGE_INFO_d874ee9262978be2
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_d874ee9262978be2:
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
	.globl	L_OBJC_IMAGE_INFO_4539fd1dbda0cddc
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_4539fd1dbda0cddc:
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
	.globl	L_OBJC_IMAGE_INFO_2b1b3a94e0ece2e5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2b1b3a94e0ece2e5:
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
	.globl	L_OBJC_IMAGE_INFO_f7f521670860b0ce
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f7f521670860b0ce:
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
	.globl	L_OBJC_IMAGE_INFO_6addfcf634c6232f
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_6addfcf634c6232f:
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

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_4a8c690dbc9d8166
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_4a8c690dbc9d8166:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
