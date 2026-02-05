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
	adrp	x4, l_anon.[ID].14@PAGE
Lloh9:
	add	x4, x4, l_anon.[ID].14@PAGEOFF
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
	adrp	x4, l_anon.[ID].14@PAGE
Lloh24:
	add	x4, x4, l_anon.[ID].14@PAGEOFF
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
	adrp	x4, l_anon.[ID].14@PAGE
Lloh37:
	add	x4, x4, l_anon.[ID].14@PAGEOFF
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
	adrp	x1, l_anon.[ID].12@PAGE
Lloh42:
	add	x1, x1, l_anon.[ID].12@PAGEOFF
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
	adrp	x4, l_anon.[ID].3@PAGE
Lloh46:
	add	x4, x4, l_anon.[ID].3@PAGEOFF
Lloh47:
	adrp	x5, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0)@PAGE
Lloh48:
	add	x5, x5, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0)@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_class_method_inner, 0)
Ltmp7:
Lloh49:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_adfe2eb45443b755@PAGE
Lloh50:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_adfe2eb45443b755@PAGEOFF]
Ltmp8:
Lloh51:
	adrp	x4, l_anon.[ID].4@PAGE
Lloh52:
	add	x4, x4, l_anon.[ID].4@PAGEOFF
Lloh53:
	adrp	x5, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0)@PAGE
Lloh54:
	add	x5, x5, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0)@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp9:
Lloh55:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_71045e8f6bc4d4f9@PAGE
Lloh56:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_71045e8f6bc4d4f9@PAGEOFF]
Ltmp10:
Lloh57:
	adrp	x2, l_anon.[ID].5@PAGE
Lloh58:
	add	x2, x2, l_anon.[ID].5@PAGEOFF
Lloh59:
	adrp	x5, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, bool, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, bool>, 0)@PAGE
Lloh60:
	add	x5, x5, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, bool, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, bool>, 0)@PAGEOFF
	add	x0, sp, #8
	mov	w3, #1
	mov	x4, x2
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp11:
Lloh61:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_67a6d939e037a80c@PAGE
Lloh62:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_67a6d939e037a80c@PAGEOFF]
Ltmp12:
Lloh63:
	adrp	x4, l_anon.[ID].6@PAGE
Lloh64:
	add	x4, x4, l_anon.[ID].6@PAGEOFF
Lloh65:
	adrp	x5, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0)@PAGE
Lloh66:
	add	x5, x5, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0)@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp13:
Lloh67:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_a83bccc921370512@PAGE
Lloh68:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_a83bccc921370512@PAGEOFF]
Ltmp14:
Lloh69:
	adrp	x2, l_anon.[ID].5@PAGE
Lloh70:
	add	x2, x2, l_anon.[ID].5@PAGEOFF
Lloh71:
	adrp	x4, l_anon.[ID].6@PAGE
Lloh72:
	add	x4, x4, l_anon.[ID].6@PAGEOFF
Lloh73:
	adrp	x5, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, bool>, 0)@PAGE
Lloh74:
	add	x5, x5, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, bool>, 0)@PAGEOFF
	add	x0, sp, #8
	mov	w3, #1
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp15:
Ltmp16:
Lloh75:
	adrp	x0, l_anon.[ID].7@PAGE
Lloh76:
	add	x0, x0, l_anon.[ID].7@PAGEOFF
	mov	w1, #8
	bl	SYM(objc2[CRATE_ID]::top_level_traits::get_protocol, 0)
	mov	x1, x0
Ltmp17:
	cbz	x1, LBB6_10
	ldr	x0, [sp, #8]
	bl	_class_addProtocol
LBB6_10:
Ltmp18:
Lloh77:
	adrp	x0, l_anon.[ID].8@PAGE
Lloh78:
	add	x0, x0, l_anon.[ID].8@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2[CRATE_ID]::top_level_traits::get_protocol, 0)
	mov	x1, x0
Ltmp19:
	cbz	x1, LBB6_13
	ldr	x0, [sp, #8]
	bl	_class_addProtocol
LBB6_13:
Lloh79:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_0be654719a6c0ed4@PAGE
Lloh80:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_0be654719a6c0ed4@PAGEOFF]
Ltmp20:
Lloh81:
	adrp	x2, l_anon.[ID].11@PAGE
Lloh82:
	add	x2, x2, l_anon.[ID].11@PAGEOFF
Lloh83:
	adrp	x4, l_anon.[ID].6@PAGE
Lloh84:
	add	x4, x4, l_anon.[ID].6@PAGEOFF
Lloh85:
	adrp	x5, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<test_define_class_no_ivars[CRATE_ID]::NoIvars>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<4u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, *const objc2[CRATE_ID]::runtime::nszone::NSZone>, 0)@PAGE
Lloh86:
	add	x5, x5, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<test_define_class_no_ivars[CRATE_ID]::NoIvars>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<4u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, *const objc2[CRATE_ID]::runtime::nszone::NSZone>, 0)@PAGEOFF
	add	x0, sp, #8
	mov	w3, #1
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
Lloh87:
	adrp	x0, l_anon.[ID].2@PAGE
Lloh88:
	add	x0, x0, l_anon.[ID].2@PAGEOFF
	bl	SYM(core[CRATE_ID]::option::unwrap_failed, 0)
LBB6_16:
Lloh89:
	adrp	x0, l_anon.[ID].12@PAGE
Lloh90:
	add	x0, x0, l_anon.[ID].12@PAGEOFF
Lloh91:
	adrp	x2, l_anon.[ID].14@PAGE
Lloh92:
	add	x2, x2, l_anon.[ID].14@PAGEOFF
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
	.loh AdrpAdd	Lloh73, Lloh74
	.loh AdrpAdd	Lloh71, Lloh72
	.loh AdrpAdd	Lloh69, Lloh70
	.loh AdrpLdr	Lloh67, Lloh68
	.loh AdrpAdd	Lloh75, Lloh76
	.loh AdrpAdd	Lloh77, Lloh78
	.loh AdrpAdd	Lloh85, Lloh86
	.loh AdrpAdd	Lloh83, Lloh84
	.loh AdrpAdd	Lloh81, Lloh82
	.loh AdrpLdr	Lloh79, Lloh80
	.loh AdrpAdd	Lloh87, Lloh88
	.loh AdrpAdd	Lloh91, Lloh92
	.loh AdrpAdd	Lloh89, Lloh90
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
Lloh93:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh94:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB8_2
LBB8_1:
Lloh95:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh96:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
Lloh97:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh98:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh99:
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
Lloh100:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh101:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh102:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh103:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh104:
	adrp	x4, l_anon.[ID].14@PAGE
Lloh105:
	add	x4, x4, l_anon.[ID].14@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	b	LBB8_1
	.loh AdrpAdd	Lloh93, Lloh94
	.loh AdrpLdrGotLdr	Lloh97, Lloh98, Lloh99
	.loh AdrpAdrp	Lloh95, Lloh97
	.loh AdrpLdr	Lloh95, Lloh96
	.loh AdrpAdd	Lloh104, Lloh105
	.loh AdrpAdd	Lloh102, Lloh103
	.loh AdrpAdd	Lloh100, Lloh101

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
Lloh106:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh107:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB10_2
Lloh108:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh109:
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
Lloh110:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh111:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh112:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh113:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh114:
	adrp	x4, l_anon.[ID].14@PAGE
Lloh115:
	add	x4, x4, l_anon.[ID].14@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh116:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh117:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh106, Lloh107
	.loh AdrpLdr	Lloh108, Lloh109
	.loh AdrpLdr	Lloh116, Lloh117
	.loh AdrpAdd	Lloh114, Lloh115
	.loh AdrpAdd	Lloh112, Lloh113
	.loh AdrpAdd	Lloh110, Lloh111

	.globl	SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars>::copy_with_zone, 0)
	.p2align	2
SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars>::copy_with_zone, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh118:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh119:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB11_2
LBB11_1:
Lloh120:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh121:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
Lloh122:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh123:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh124:
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
Lloh125:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh126:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh127:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh128:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh129:
	adrp	x4, l_anon.[ID].14@PAGE
Lloh130:
	add	x4, x4, l_anon.[ID].14@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	b	LBB11_1
	.loh AdrpAdd	Lloh118, Lloh119
	.loh AdrpLdrGotLdr	Lloh122, Lloh123, Lloh124
	.loh AdrpAdrp	Lloh120, Lloh122
	.loh AdrpLdr	Lloh120, Lloh121
	.loh AdrpAdd	Lloh129, Lloh130
	.loh AdrpAdd	Lloh127, Lloh128
	.loh AdrpAdd	Lloh125, Lloh126

	.globl	SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh131:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh132:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB12_2
Lloh133:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh134:
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
Lloh135:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh136:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh137:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh138:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh139:
	adrp	x4, l_anon.[ID].14@PAGE
Lloh140:
	add	x4, x4, l_anon.[ID].14@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh141:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh142:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh131, Lloh132
	.loh AdrpLdr	Lloh133, Lloh134
	.loh AdrpLdr	Lloh141, Lloh142
	.loh AdrpAdd	Lloh139, Lloh140
	.loh AdrpAdd	Lloh137, Lloh138
	.loh AdrpAdd	Lloh135, Lloh136

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_0be654719a6c0ed4
L_OBJC_METH_VAR_NAME_0be654719a6c0ed4:
	.asciz	"copyWithZone:"

	.globl	L_OBJC_METH_VAR_NAME_67a6d939e037a80c
L_OBJC_METH_VAR_NAME_67a6d939e037a80c:
	.asciz	"methodRetained"

	.globl	L_OBJC_METH_VAR_NAME_71045e8f6bc4d4f9
L_OBJC_METH_VAR_NAME_71045e8f6bc4d4f9:
	.asciz	"methodBool:"

	.globl	L_OBJC_METH_VAR_NAME_a83bccc921370512
L_OBJC_METH_VAR_NAME_a83bccc921370512:
	.asciz	"methodRetainedWithParam:"

	.globl	L_OBJC_METH_VAR_NAME_a9fb958c74006297
L_OBJC_METH_VAR_NAME_a9fb958c74006297:
	.asciz	"classMethod"

	.globl	L_OBJC_METH_VAR_NAME_adfe2eb45443b755
L_OBJC_METH_VAR_NAME_adfe2eb45443b755:
	.asciz	"method"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_0be654719a6c0ed4
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_0be654719a6c0ed4:
	.quad	L_OBJC_METH_VAR_NAME_0be654719a6c0ed4

	.globl	L_OBJC_SELECTOR_REFERENCES_67a6d939e037a80c
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_67a6d939e037a80c:
	.quad	L_OBJC_METH_VAR_NAME_67a6d939e037a80c

	.globl	L_OBJC_SELECTOR_REFERENCES_71045e8f6bc4d4f9
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_71045e8f6bc4d4f9:
	.quad	L_OBJC_METH_VAR_NAME_71045e8f6bc4d4f9

	.globl	L_OBJC_SELECTOR_REFERENCES_a83bccc921370512
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_a83bccc921370512:
	.quad	L_OBJC_METH_VAR_NAME_a83bccc921370512

	.globl	L_OBJC_SELECTOR_REFERENCES_a9fb958c74006297
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_a9fb958c74006297:
	.quad	L_OBJC_METH_VAR_NAME_a9fb958c74006297

	.globl	L_OBJC_SELECTOR_REFERENCES_adfe2eb45443b755
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_adfe2eb45443b755:
	.quad	L_OBJC_METH_VAR_NAME_adfe2eb45443b755

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_0be654719a6c0ed4
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_0be654719a6c0ed4:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_67a6d939e037a80c
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_67a6d939e037a80c:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_71045e8f6bc4d4f9
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_71045e8f6bc4d4f9:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_a83bccc921370512
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a83bccc921370512:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_a9fb958c74006297
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_a9fb958c74006297:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_adfe2eb45443b755
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_adfe2eb45443b755:
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

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].3:
	.byte	23
	.space	39

	.p2align	3, 0x0
l_anon.[ID].4:
	.byte	19
	.space	39

	.p2align	3, 0x0
l_anon.[ID].5:
	.byte	18
	.space	39

	.p2align	3, 0x0
l_anon.[ID].6:
	.byte	21
	.space	39

	.section	__TEXT,__literal8,8byte_literals
l_anon.[ID].7:
	.ascii	"NSObject"

	.section	__TEXT,__const
l_anon.[ID].8:
	.ascii	"NSCopying"

l_anon.[ID].9:
	.ascii	"_NSZone"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].10:
	.byte	30
	.space	7
	.quad	l_anon.[ID].9
	.asciz	"\007\000\000\000\000\000\000"
	.quad	8
	.space	8

	.p2align	3, 0x0
l_anon.[ID].11:
	.byte	27
	.space	7
	.quad	l_anon.[ID].10
	.space	24

	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].12:
	.asciz	"NoIvars"

l_anon.[ID].13:
	.asciz	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].14:
	.quad	l_anon.[ID].13
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
