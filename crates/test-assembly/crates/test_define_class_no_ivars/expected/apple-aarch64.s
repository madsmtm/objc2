	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0):
Lloh0:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh1:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB0_2
Lloh2:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh3:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
	ret
LBB0_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh4:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh5:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh6:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh7:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh8:
	adrp	x4, l_anon.[ID].13@PAGE
Lloh9:
	add	x4, x4, l_anon.[ID].13@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh10:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh11:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh0, Lloh1
	.loh AdrpLdr	Lloh2, Lloh3
	.loh AdrpLdr	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5

	.p2align	2
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, bool, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, bool>, 0):
	eor	w0, w2, #0x1
	ret

	.p2align	2
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, bool>, 0):
Lfunc_begin0:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x20, x2
	bl	SYM(<objc2[CRATE_ID]::runtime::nsobject::NSObject>::new, 0)
	mov	x19, x0
	cbz	w20, LBB2_3
Ltmp0:
	bl	SYM(<objc2[CRATE_ID]::runtime::nsobject::NSObject>::new, 0)
Ltmp1:
	mov	x20, x0
	mov	x0, x19
	bl	_objc_release
	mov	x19, x20
LBB2_3:
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_autoreleaseReturnValue
LBB2_4:
Ltmp2:
	mov	x20, x0
Ltmp3:
	mov	x0, x19
	bl	_objc_release
Ltmp4:
	mov	x0, x20
	bl	__Unwind_Resume
LBB2_6:
Ltmp5:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table2:
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
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<test_define_class_no_ivars[CRATE_ID]::NoIvars>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<4u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, *const objc2[CRATE_ID]::runtime::nszone::NSZone>, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh12:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh13:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB3_2
LBB3_1:
Lloh14:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh15:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
Lloh16:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh17:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh18:
	ldr	x1, [x8]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB3_2:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh19:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh20:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh21:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh22:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh23:
	adrp	x4, l_anon.[ID].13@PAGE
Lloh24:
	add	x4, x4, l_anon.[ID].13@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	b	LBB3_1
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpLdrGotLdr	Lloh16, Lloh17, Lloh18
	.loh AdrpAdrp	Lloh14, Lloh16
	.loh AdrpLdr	Lloh14, Lloh15
	.loh AdrpAdd	Lloh23, Lloh24
	.loh AdrpAdd	Lloh21, Lloh22
	.loh AdrpAdd	Lloh19, Lloh20

	.p2align	2
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh25:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh26:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB4_2
LBB4_1:
Lloh27:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh28:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
Lloh29:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh30:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh31:
	ldr	x1, [x8]
	bl	_objc_msgSend
	bl	_objc_autoreleaseReturnValue
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB4_2:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh32:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh33:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh34:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh35:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh36:
	adrp	x4, l_anon.[ID].13@PAGE
Lloh37:
	add	x4, x4, l_anon.[ID].13@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	b	LBB4_1
	.loh AdrpAdd	Lloh25, Lloh26
	.loh AdrpLdrGotLdr	Lloh29, Lloh30, Lloh31
	.loh AdrpAdrp	Lloh27, Lloh29
	.loh AdrpLdr	Lloh27, Lloh28
	.loh AdrpAdd	Lloh36, Lloh37
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpAdd	Lloh32, Lloh33

	.p2align	2
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0):
	ret

	.p2align	2
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin1:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	cmp	w9, #1
	b.ne	LBB6_15
Lloh38:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh39:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh40:
	ldr	x0, [x8]
Lloh41:
	adrp	x1, l_anon.[ID].11@PAGE
Lloh42:
	add	x1, x1, l_anon.[ID].11@PAGEOFF
	mov	x2, #0
	bl	_objc_allocateClassPair
	cbz	x0, LBB6_16
	str	x0, [sp, #8]
Lloh43:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_a9fb958c74006297@PAGE
Lloh44:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_a9fb958c74006297@PAGEOFF]
Ltmp6:
Lloh45:
	adrp	x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0)@PAGE
Lloh46:
	add	x2, x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0)@PAGEOFF
Lloh47:
	adrp	x3, l_anon.[ID].3@PAGE
Lloh48:
	add	x3, x3, l_anon.[ID].3@PAGEOFF
	add	x0, sp, #8
	mov	w4, #4
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_class_method_inner, 0)
Ltmp7:
Lloh49:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_adfe2eb45443b755@PAGE
Lloh50:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_adfe2eb45443b755@PAGEOFF]
Ltmp8:
Lloh51:
	adrp	x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0)@PAGE
Lloh52:
	add	x2, x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0)@PAGEOFF
Lloh53:
	adrp	x3, l_anon.[ID].4@PAGE
Lloh54:
	add	x3, x3, l_anon.[ID].4@PAGEOFF
	add	x0, sp, #8
	mov	w4, #4
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp9:
Lloh55:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_71045e8f6bc4d4f9@PAGE
Lloh56:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_71045e8f6bc4d4f9@PAGEOFF]
Ltmp10:
Lloh57:
	adrp	x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, bool, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, bool>, 0)@PAGE
Lloh58:
	add	x2, x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, bool, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, bool>, 0)@PAGEOFF
Lloh59:
	adrp	x3, l_anon.[ID].5@PAGE
Lloh60:
	add	x3, x3, l_anon.[ID].5@PAGEOFF
	add	x0, sp, #8
	mov	w4, #5
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp11:
Lloh61:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_44ec46acb208c3d4@PAGE
Lloh62:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_44ec46acb208c3d4@PAGEOFF]
Ltmp12:
Lloh63:
	adrp	x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0)@PAGE
Lloh64:
	add	x2, x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0)@PAGEOFF
Lloh65:
	adrp	x3, l_anon.[ID].6@PAGE
Lloh66:
	add	x3, x3, l_anon.[ID].6@PAGEOFF
	add	x0, sp, #8
	mov	w4, #4
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp13:
Lloh67:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_d63b759d5c7ed4f0@PAGE
Lloh68:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_d63b759d5c7ed4f0@PAGEOFF]
Ltmp14:
Lloh69:
	adrp	x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, bool>, 0)@PAGE
Lloh70:
	add	x2, x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, bool>, 0)@PAGEOFF
Lloh71:
	adrp	x3, l_anon.[ID].7@PAGE
Lloh72:
	add	x3, x3, l_anon.[ID].7@PAGEOFF
	add	x0, sp, #8
	mov	w4, #5
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp15:
Ltmp16:
Lloh73:
	adrp	x0, l_anon.[ID].8@PAGE
Lloh74:
	add	x0, x0, l_anon.[ID].8@PAGEOFF
	mov	w1, #8
	bl	SYM(objc2[CRATE_ID]::top_level_traits::get_protocol, 0)
	mov	x1, x0
Ltmp17:
	cbz	x1, LBB6_10
	ldr	x0, [sp, #8]
	bl	_class_addProtocol
LBB6_10:
Ltmp18:
Lloh75:
	adrp	x0, l_anon.[ID].9@PAGE
Lloh76:
	add	x0, x0, l_anon.[ID].9@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2[CRATE_ID]::top_level_traits::get_protocol, 0)
	mov	x1, x0
Ltmp19:
	cbz	x1, LBB6_13
	ldr	x0, [sp, #8]
	bl	_class_addProtocol
LBB6_13:
Lloh77:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_996a3b5043cf563d@PAGE
Lloh78:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_996a3b5043cf563d@PAGEOFF]
Ltmp20:
Lloh79:
	adrp	x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<test_define_class_no_ivars[CRATE_ID]::NoIvars>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<4u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, *const objc2[CRATE_ID]::runtime::nszone::NSZone>, 0)@PAGE
Lloh80:
	add	x2, x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<test_define_class_no_ivars[CRATE_ID]::NoIvars>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<4u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, *const objc2[CRATE_ID]::runtime::nszone::NSZone>, 0)@PAGEOFF
Lloh81:
	adrp	x3, l_anon.[ID].10@PAGE
Lloh82:
	add	x3, x3, l_anon.[ID].10@PAGEOFF
	add	x0, sp, #8
	mov	w4, #15
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp21:
	ldr	x19, [sp, #8]
	mov	x0, x19
	bl	_objc_registerClassPair
	adrp	x8, ___CLASS_NoIvars@PAGE
	str	x19, [x8, ___CLASS_NoIvars@PAGEOFF]
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB6_15:
Lloh83:
	adrp	x0, l_anon.[ID].2@PAGE
Lloh84:
	add	x0, x0, l_anon.[ID].2@PAGEOFF
	bl	SYM(core[CRATE_ID]::option::unwrap_failed, 0)
LBB6_16:
Lloh85:
	adrp	x0, l_anon.[ID].11@PAGE
Lloh86:
	add	x0, x0, l_anon.[ID].11@PAGEOFF
Lloh87:
	adrp	x2, l_anon.[ID].13@PAGE
Lloh88:
	add	x2, x2, l_anon.[ID].13@PAGEOFF
	mov	w1, #8
	bl	SYM(objc2[CRATE_ID]::__macros::define_class::checks::class_not_unique, 0)
LBB6_17:
Ltmp22:
	mov	x19, x0
	ldr	x0, [sp, #8]
	bl	_objc_disposeClassPair
	mov	x0, x19
	bl	__Unwind_Resume
	.loh AdrpAdd	Lloh41, Lloh42
	.loh AdrpLdrGotLdr	Lloh38, Lloh39, Lloh40
	.loh AdrpAdd	Lloh47, Lloh48
	.loh AdrpAdd	Lloh45, Lloh46
	.loh AdrpLdr	Lloh43, Lloh44
	.loh AdrpAdd	Lloh53, Lloh54
	.loh AdrpAdd	Lloh51, Lloh52
	.loh AdrpLdr	Lloh49, Lloh50
	.loh AdrpAdd	Lloh59, Lloh60
	.loh AdrpAdd	Lloh57, Lloh58
	.loh AdrpLdr	Lloh55, Lloh56
	.loh AdrpAdd	Lloh65, Lloh66
	.loh AdrpAdd	Lloh63, Lloh64
	.loh AdrpLdr	Lloh61, Lloh62
	.loh AdrpAdd	Lloh71, Lloh72
	.loh AdrpAdd	Lloh69, Lloh70
	.loh AdrpLdr	Lloh67, Lloh68
	.loh AdrpAdd	Lloh73, Lloh74
	.loh AdrpAdd	Lloh75, Lloh76
	.loh AdrpAdd	Lloh81, Lloh82
	.loh AdrpAdd	Lloh79, Lloh80
	.loh AdrpLdr	Lloh77, Lloh78
	.loh AdrpAdd	Lloh83, Lloh84
	.loh AdrpAdd	Lloh87, Lloh88
	.loh AdrpAdd	Lloh85, Lloh86
Lfunc_end1:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table6:
Lexception1:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
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
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	str	x8, [sp, #8]
	add	x0, sp, #8
	bl	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.globl	SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars>::method_retained, 0)
	.p2align	2
SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars>::method_retained, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh89:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh90:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB8_2
LBB8_1:
Lloh91:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh92:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
Lloh93:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh94:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh95:
	ldr	x1, [x8]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB8_2:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh96:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh97:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh98:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh99:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh100:
	adrp	x4, l_anon.[ID].13@PAGE
Lloh101:
	add	x4, x4, l_anon.[ID].13@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	b	LBB8_1
	.loh AdrpAdd	Lloh89, Lloh90
	.loh AdrpLdrGotLdr	Lloh93, Lloh94, Lloh95
	.loh AdrpAdrp	Lloh91, Lloh93
	.loh AdrpLdr	Lloh91, Lloh92
	.loh AdrpAdd	Lloh100, Lloh101
	.loh AdrpAdd	Lloh98, Lloh99
	.loh AdrpAdd	Lloh96, Lloh97

	.globl	SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars>::method_retained_with_param, 0)
	.p2align	2
SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars>::method_retained_with_param, 0):
Lfunc_begin2:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x20, x1
	bl	SYM(<objc2[CRATE_ID]::runtime::nsobject::NSObject>::new, 0)
	mov	x19, x0
	cbz	w20, LBB9_3
Ltmp23:
	bl	SYM(<objc2[CRATE_ID]::runtime::nsobject::NSObject>::new, 0)
Ltmp24:
	mov	x20, x0
	mov	x0, x19
	bl	_objc_release
	mov	x19, x20
LBB9_3:
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
LBB9_4:
Ltmp25:
	mov	x20, x0
Ltmp26:
	mov	x0, x19
	bl	_objc_release
Ltmp27:
	mov	x0, x20
	bl	__Unwind_Resume
LBB9_6:
Ltmp28:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end2:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table9:
Lexception2:
	.byte	255
	.byte	155
	.uleb128 Lttbase1-Lttbaseref1
Lttbaseref1:
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
	.uleb128 Ltmp24-Lfunc_begin2
	.uleb128 Ltmp26-Ltmp24
	.byte	0
	.byte	0
	.uleb128 Ltmp26-Lfunc_begin2
	.uleb128 Ltmp27-Ltmp26
	.uleb128 Ltmp28-Lfunc_begin2
	.byte	1
	.uleb128 Ltmp27-Lfunc_begin2
	.uleb128 Lfunc_end2-Ltmp27
	.byte	0
	.byte	0
Lcst_end2:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase1:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars>::get_class, 0)
	.p2align	2
SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars>::get_class, 0):
Lloh102:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh103:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB10_2
Lloh104:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh105:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
	ret
LBB10_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh106:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh107:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh108:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh109:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh110:
	adrp	x4, l_anon.[ID].13@PAGE
Lloh111:
	add	x4, x4, l_anon.[ID].13@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh112:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh113:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh102, Lloh103
	.loh AdrpLdr	Lloh104, Lloh105
	.loh AdrpLdr	Lloh112, Lloh113
	.loh AdrpAdd	Lloh110, Lloh111
	.loh AdrpAdd	Lloh108, Lloh109
	.loh AdrpAdd	Lloh106, Lloh107

	.globl	SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars>::copy_with_zone, 0)
	.p2align	2
SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars>::copy_with_zone, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh114:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh115:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB11_2
LBB11_1:
Lloh116:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh117:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
Lloh118:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh119:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh120:
	ldr	x1, [x8]
	bl	_objc_msgSend
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB11_2:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh121:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh122:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh123:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh124:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh125:
	adrp	x4, l_anon.[ID].13@PAGE
Lloh126:
	add	x4, x4, l_anon.[ID].13@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	b	LBB11_1
	.loh AdrpAdd	Lloh114, Lloh115
	.loh AdrpLdrGotLdr	Lloh118, Lloh119, Lloh120
	.loh AdrpAdrp	Lloh116, Lloh118
	.loh AdrpLdr	Lloh116, Lloh117
	.loh AdrpAdd	Lloh125, Lloh126
	.loh AdrpAdd	Lloh123, Lloh124
	.loh AdrpAdd	Lloh121, Lloh122

	.globl	SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh127:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh128:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB12_2
Lloh129:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh130:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
	ret
LBB12_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh131:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh132:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh133:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh134:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh135:
	adrp	x4, l_anon.[ID].13@PAGE
Lloh136:
	add	x4, x4, l_anon.[ID].13@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh137:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh138:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh127, Lloh128
	.loh AdrpLdr	Lloh129, Lloh130
	.loh AdrpLdr	Lloh137, Lloh138
	.loh AdrpAdd	Lloh135, Lloh136
	.loh AdrpAdd	Lloh133, Lloh134
	.loh AdrpAdd	Lloh131, Lloh132

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_44ec46acb208c3d4
L_OBJC_METH_VAR_NAME_44ec46acb208c3d4:
	.asciz	"methodRetained"

	.globl	L_OBJC_METH_VAR_NAME_71045e8f6bc4d4f9
L_OBJC_METH_VAR_NAME_71045e8f6bc4d4f9:
	.asciz	"methodBool:"

	.globl	L_OBJC_METH_VAR_NAME_996a3b5043cf563d
L_OBJC_METH_VAR_NAME_996a3b5043cf563d:
	.asciz	"copyWithZone:"

	.globl	L_OBJC_METH_VAR_NAME_a9fb958c74006297
L_OBJC_METH_VAR_NAME_a9fb958c74006297:
	.asciz	"classMethod"

	.globl	L_OBJC_METH_VAR_NAME_adfe2eb45443b755
L_OBJC_METH_VAR_NAME_adfe2eb45443b755:
	.asciz	"method"

	.globl	L_OBJC_METH_VAR_NAME_d63b759d5c7ed4f0
L_OBJC_METH_VAR_NAME_d63b759d5c7ed4f0:
	.asciz	"methodRetainedWithParam:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_44ec46acb208c3d4
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_44ec46acb208c3d4:
	.quad	L_OBJC_METH_VAR_NAME_44ec46acb208c3d4

	.globl	L_OBJC_SELECTOR_REFERENCES_71045e8f6bc4d4f9
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_71045e8f6bc4d4f9:
	.quad	L_OBJC_METH_VAR_NAME_71045e8f6bc4d4f9

	.globl	L_OBJC_SELECTOR_REFERENCES_996a3b5043cf563d
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_996a3b5043cf563d:
	.quad	L_OBJC_METH_VAR_NAME_996a3b5043cf563d

	.globl	L_OBJC_SELECTOR_REFERENCES_a9fb958c74006297
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_a9fb958c74006297:
	.quad	L_OBJC_METH_VAR_NAME_a9fb958c74006297

	.globl	L_OBJC_SELECTOR_REFERENCES_adfe2eb45443b755
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_adfe2eb45443b755:
	.quad	L_OBJC_METH_VAR_NAME_adfe2eb45443b755

	.globl	L_OBJC_SELECTOR_REFERENCES_d63b759d5c7ed4f0
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_d63b759d5c7ed4f0:
	.quad	L_OBJC_METH_VAR_NAME_d63b759d5c7ed4f0

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_44ec46acb208c3d4
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_44ec46acb208c3d4:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_71045e8f6bc4d4f9
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_71045e8f6bc4d4f9:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_996a3b5043cf563d
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_996a3b5043cf563d:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_a9fb958c74006297
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a9fb958c74006297:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_adfe2eb45443b755
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_adfe2eb45443b755:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_d63b759d5c7ed4f0
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_d63b759d5c7ed4f0:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].1:
	.asciz	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].2:
	.quad	l_anon.[ID].1
	.asciz	"p\000\000\000\000\000\000\000\237\000\000\0002\000\000"

	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].3:
	.asciz	"#@:"

l_anon.[ID].4:
	.asciz	"v@:"

l_anon.[ID].5:
	.asciz	"B@:B"

l_anon.[ID].6:
	.asciz	"@@:"

l_anon.[ID].7:
	.asciz	"@@:B"

	.section	__TEXT,__literal8,8byte_literals
l_anon.[ID].8:
	.ascii	"NSObject"

	.section	__TEXT,__const
l_anon.[ID].9:
	.ascii	"NSCopying"

	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].10:
	.asciz	"@@:^{_NSZone=}"

l_anon.[ID].11:
	.asciz	"NoIvars"

l_anon.[ID].12:
	.asciz	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].13:
	.quad	l_anon.[ID].12
	.asciz	"=\000\000\000\000\000\000\000\r\000\000\000\001\000\000"

	.globl	___CLASS_NoIvars
.zerofill __DATA,__common,___CLASS_NoIvars,8,3
	.globl	___DROP_FLAG_OFFSET_NoIvars
.zerofill __DATA,__common,___DROP_FLAG_OFFSET_NoIvars,8,3
	.globl	___IVAR_OFFSET_NoIvars
.zerofill __DATA,__common,___IVAR_OFFSET_NoIvars,8,3
	.section	__DATA,__data
	.globl	___REGISTER_CLASS_NoIvars
	.p2align	3, 0x0
___REGISTER_CLASS_NoIvars:
	.asciz	"\003\000\000\000\000\000\000"

.subsections_via_symbols
