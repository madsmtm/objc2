	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, &mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, 0):
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x2
	ldr	x0, [x2]
	bl	_objc_retain
	mov	x0, x19
	bl	_external
	ldr	x0, [x19]
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_autorelease

	.p2align	2
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, &mut core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>>, 0):
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x2
	ldr	x0, [x2]
	bl	_objc_retain
	mov	x0, x19
	bl	_external
	ldr	x0, [x19]
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_autorelease

	.p2align	2
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, core[CRATE_ID]::option::Option<&mut core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>>>, 0):
	cbz	x2, LBB2_2
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x0, [x2]
	mov	x19, x2
	bl	_objc_retain
	mov	x0, x19
	bl	_external
	ldr	x0, [x19]
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_autorelease
LBB2_2:
	mov	x0, #0
	b	_external

	.p2align	2
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, core[CRATE_ID]::option::Option<&mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>>, 0):
	cbz	x2, LBB3_2
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x0, [x2]
	mov	x19, x2
	bl	_objc_retain
	mov	x0, x19
	bl	_external
	ldr	x0, [x19]
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_autorelease
LBB3_2:
	mov	x0, #0
	b	_external

	.p2align	2
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, &mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>, &mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, 0):
Lfunc_begin0:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x20, x3
	mov	x19, x2
	ldr	x0, [x2]
	bl	_objc_retain
	ldr	x0, [x20]
Ltmp0:
	bl	_objc_retain
Ltmp1:
	mov	x0, x19
	bl	_external
	mov	x0, x20
	bl	_external
	ldr	x0, [x20]
Ltmp2:
	bl	_objc_autorelease
Ltmp3:
	ldr	x0, [x19]
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_autorelease
LBB4_3:
Ltmp4:
	mov	x20, x0
	ldr	x0, [x19]
Ltmp5:
	bl	_objc_autorelease
Ltmp6:
	mov	x0, x20
	bl	__Unwind_Resume
LBB4_5:
Ltmp7:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table4:
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
	.uleb128 Ltmp3-Ltmp0
	.uleb128 Ltmp4-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp3-Lfunc_begin0
	.uleb128 Ltmp5-Ltmp3
	.byte	0
	.byte	0
	.uleb128 Ltmp5-Lfunc_begin0
	.uleb128 Ltmp6-Ltmp5
	.uleb128 Ltmp7-Lfunc_begin0
	.byte	1
	.uleb128 Ltmp6-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp6
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
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin1:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	cmp	w9, #1
	b.ne	LBB5_8
Lloh0:
	adrp	x8, _OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh1:
	ldr	x8, [x8, _OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh2:
	ldr	x0, [x8]
Lloh3:
	adrp	x1, l_anon.[ID].5@PAGE
Lloh4:
	add	x1, x1, l_anon.[ID].5@PAGEOFF
	mov	x2, #0
	bl	_objc_allocateClassPair
	cbz	x0, LBB5_9
	str	x0, [sp, #8]
Lloh5:
	adrp	x8, _OBJC_SELECTOR_REFERENCES_d04fae079395572d@PAGE
Lloh6:
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_d04fae079395572d@PAGEOFF]
Ltmp8:
Lloh7:
	adrp	x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, &mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, 0)@PAGE
Lloh8:
	add	x2, x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, &mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, 0)@PAGEOFF
Lloh9:
	adrp	x3, l_anon.[ID].3@PAGE
Lloh10:
	add	x3, x3, l_anon.[ID].3@PAGEOFF
	add	x0, sp, #8
	mov	w4, #6
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_class_method_inner, 0)
Ltmp9:
Lloh11:
	adrp	x8, _OBJC_SELECTOR_REFERENCES_48e4a002a77cbcfe@PAGE
Lloh12:
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_48e4a002a77cbcfe@PAGEOFF]
Ltmp10:
Lloh13:
	adrp	x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, core[CRATE_ID]::option::Option<&mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>>, 0)@PAGE
Lloh14:
	add	x2, x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, core[CRATE_ID]::option::Option<&mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>>, 0)@PAGEOFF
Lloh15:
	adrp	x3, l_anon.[ID].3@PAGE
Lloh16:
	add	x3, x3, l_anon.[ID].3@PAGEOFF
	add	x0, sp, #8
	mov	w4, #6
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_class_method_inner, 0)
Ltmp11:
Lloh17:
	adrp	x8, _OBJC_SELECTOR_REFERENCES_06f3315fc4e8da1a@PAGE
Lloh18:
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_06f3315fc4e8da1a@PAGEOFF]
Ltmp12:
Lloh19:
	adrp	x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, &mut core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>>, 0)@PAGE
Lloh20:
	add	x2, x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, &mut core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>>, 0)@PAGEOFF
Lloh21:
	adrp	x3, l_anon.[ID].3@PAGE
Lloh22:
	add	x3, x3, l_anon.[ID].3@PAGEOFF
	add	x0, sp, #8
	mov	w4, #6
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_class_method_inner, 0)
Ltmp13:
Lloh23:
	adrp	x8, _OBJC_SELECTOR_REFERENCES_77a278f49aa73e0a@PAGE
Lloh24:
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_77a278f49aa73e0a@PAGEOFF]
Ltmp14:
Lloh25:
	adrp	x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, core[CRATE_ID]::option::Option<&mut core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>>>, 0)@PAGE
Lloh26:
	add	x2, x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, core[CRATE_ID]::option::Option<&mut core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>>>, 0)@PAGEOFF
Lloh27:
	adrp	x3, l_anon.[ID].3@PAGE
Lloh28:
	add	x3, x3, l_anon.[ID].3@PAGEOFF
	add	x0, sp, #8
	mov	w4, #6
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_class_method_inner, 0)
Ltmp15:
Lloh29:
	adrp	x8, _OBJC_SELECTOR_REFERENCES_9c279b7fa093536a@PAGE
Lloh30:
	ldr	x1, [x8, _OBJC_SELECTOR_REFERENCES_9c279b7fa093536a@PAGEOFF]
Ltmp16:
Lloh31:
	adrp	x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, &mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>, &mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, 0)@PAGE
Lloh32:
	add	x2, x2, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, &mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>, &mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, 0)@PAGEOFF
Lloh33:
	adrp	x3, l_anon.[ID].4@PAGE
Lloh34:
	add	x3, x3, l_anon.[ID].4@PAGEOFF
	add	x0, sp, #8
	mov	w4, #8
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_class_method_inner, 0)
Ltmp17:
	ldr	x19, [sp, #8]
	mov	x0, x19
	bl	_objc_registerClassPair
	adrp	x8, ___CLASS_OutParam@PAGE
	str	x19, [x8, ___CLASS_OutParam@PAGEOFF]
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB5_8:
Lloh35:
	adrp	x0, l_anon.[ID].2@PAGE
Lloh36:
	add	x0, x0, l_anon.[ID].2@PAGEOFF
	bl	SYM(core[CRATE_ID]::option::unwrap_failed, 0)
LBB5_9:
Lloh37:
	adrp	x0, l_anon.[ID].5@PAGE
Lloh38:
	add	x0, x0, l_anon.[ID].5@PAGEOFF
Lloh39:
	adrp	x2, l_anon.[ID].7@PAGE
Lloh40:
	add	x2, x2, l_anon.[ID].7@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2[CRATE_ID]::__macros::define_class::checks::class_not_unique, 0)
LBB5_10:
Ltmp18:
	mov	x19, x0
	ldr	x0, [sp, #8]
	bl	_objc_disposeClassPair
	mov	x0, x19
	bl	__Unwind_Resume
	.loh AdrpAdd	Lloh3, Lloh4
	.loh AdrpLdrGotLdr	Lloh0, Lloh1, Lloh2
	.loh AdrpAdd	Lloh9, Lloh10
	.loh AdrpAdd	Lloh7, Lloh8
	.loh AdrpLdr	Lloh5, Lloh6
	.loh AdrpAdd	Lloh15, Lloh16
	.loh AdrpAdd	Lloh13, Lloh14
	.loh AdrpLdr	Lloh11, Lloh12
	.loh AdrpAdd	Lloh21, Lloh22
	.loh AdrpAdd	Lloh19, Lloh20
	.loh AdrpLdr	Lloh17, Lloh18
	.loh AdrpAdd	Lloh27, Lloh28
	.loh AdrpAdd	Lloh25, Lloh26
	.loh AdrpLdr	Lloh23, Lloh24
	.loh AdrpAdd	Lloh33, Lloh34
	.loh AdrpAdd	Lloh31, Lloh32
	.loh AdrpLdr	Lloh29, Lloh30
	.loh AdrpAdd	Lloh35, Lloh36
	.loh AdrpAdd	Lloh39, Lloh40
	.loh AdrpAdd	Lloh37, Lloh38
Lfunc_end1:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table5:
Lexception1:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
	.uleb128 Ltmp8-Lfunc_begin1
	.uleb128 Ltmp17-Ltmp8
	.uleb128 Ltmp18-Lfunc_begin1
	.byte	0
	.uleb128 Ltmp17-Lfunc_begin1
	.uleb128 Lfunc_end1-Ltmp17
	.byte	0
	.byte	0
Lcst_end1:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	str	x8, [sp, #8]
	add	x0, sp, #8
	bl	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.globl	SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::nonnull_null, 0)
	.p2align	2
SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::nonnull_null, 0):
	b	_external

	.globl	SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::null_nonnull, 0)
	.p2align	2
SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::null_nonnull, 0):
	b	_external

	.globl	SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::nonnull_nonnull, 0)
	.p2align	2
SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::nonnull_nonnull, 0):
	b	_external

	.globl	SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::two_nonnull_nonnull, 0)
	.p2align	2
SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::two_nonnull_nonnull, 0):
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x1
	bl	_external
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_external

	.globl	SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::null_null, 0)
	.p2align	2
SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::null_null, 0):
	b	_external

	.globl	SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh41:
	adrp	x8, ___REGISTER_CLASS_OutParam@PAGE
Lloh42:
	add	x8, x8, ___REGISTER_CLASS_OutParam@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB12_2
Lloh43:
	adrp	x8, ___CLASS_OutParam@PAGE
Lloh44:
	ldr	x0, [x8, ___CLASS_OutParam@PAGEOFF]
	ret
LBB12_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh45:
	adrp	x0, ___REGISTER_CLASS_OutParam@PAGE
Lloh46:
	add	x0, x0, ___REGISTER_CLASS_OutParam@PAGEOFF
Lloh47:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh48:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh49:
	adrp	x4, l_anon.[ID].7@PAGE
Lloh50:
	add	x4, x4, l_anon.[ID].7@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh51:
	adrp	x8, ___CLASS_OutParam@PAGE
Lloh52:
	ldr	x0, [x8, ___CLASS_OutParam@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh41, Lloh42
	.loh AdrpLdr	Lloh43, Lloh44
	.loh AdrpLdr	Lloh51, Lloh52
	.loh AdrpAdd	Lloh49, Lloh50
	.loh AdrpAdd	Lloh47, Lloh48
	.loh AdrpAdd	Lloh45, Lloh46

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_06f3315fc4e8da1a
L_OBJC_METH_VAR_NAME_06f3315fc4e8da1a:
	.asciz	"nonnull_null:"

	.globl	L_OBJC_METH_VAR_NAME_48e4a002a77cbcfe
L_OBJC_METH_VAR_NAME_48e4a002a77cbcfe:
	.asciz	"null_nonnull:"

	.globl	L_OBJC_METH_VAR_NAME_77a278f49aa73e0a
L_OBJC_METH_VAR_NAME_77a278f49aa73e0a:
	.asciz	"null_null:"

	.globl	L_OBJC_METH_VAR_NAME_9c279b7fa093536a
L_OBJC_METH_VAR_NAME_9c279b7fa093536a:
	.asciz	"two:nonnull_nonnull:"

	.globl	L_OBJC_METH_VAR_NAME_d04fae079395572d
L_OBJC_METH_VAR_NAME_d04fae079395572d:
	.asciz	"nonnull_nonnull:"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	_OBJC_SELECTOR_REFERENCES_06f3315fc4e8da1a
	.p2align	3, 0x0
_OBJC_SELECTOR_REFERENCES_06f3315fc4e8da1a:
	.quad	L_OBJC_METH_VAR_NAME_06f3315fc4e8da1a

	.globl	_OBJC_SELECTOR_REFERENCES_48e4a002a77cbcfe
	.p2align	3, 0x0
_OBJC_SELECTOR_REFERENCES_48e4a002a77cbcfe:
	.quad	L_OBJC_METH_VAR_NAME_48e4a002a77cbcfe

	.globl	_OBJC_SELECTOR_REFERENCES_77a278f49aa73e0a
	.p2align	3, 0x0
_OBJC_SELECTOR_REFERENCES_77a278f49aa73e0a:
	.quad	L_OBJC_METH_VAR_NAME_77a278f49aa73e0a

	.globl	_OBJC_SELECTOR_REFERENCES_9c279b7fa093536a
	.p2align	3, 0x0
_OBJC_SELECTOR_REFERENCES_9c279b7fa093536a:
	.quad	L_OBJC_METH_VAR_NAME_9c279b7fa093536a

	.globl	_OBJC_SELECTOR_REFERENCES_d04fae079395572d
	.p2align	3, 0x0
_OBJC_SELECTOR_REFERENCES_d04fae079395572d:
	.quad	L_OBJC_METH_VAR_NAME_d04fae079395572d

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_06f3315fc4e8da1a
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_06f3315fc4e8da1a:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_48e4a002a77cbcfe
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_48e4a002a77cbcfe:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_77a278f49aa73e0a
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_77a278f49aa73e0a:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_9c279b7fa093536a
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_9c279b7fa093536a:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_d04fae079395572d
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_d04fae079395572d:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].1:
	.asciz	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].2:
	.quad	l_anon.[ID].1
	.asciz	"p\000\000\000\000\000\000\000\246\000\000\0002\000\000"

	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].3:
	.asciz	"v@:^@"

l_anon.[ID].4:
	.asciz	"v@:^@^@"

l_anon.[ID].5:
	.asciz	"OutParam"

l_anon.[ID].6:
	.asciz	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].7:
	.quad	l_anon.[ID].6
	.asciz	"C\000\000\000\000\000\000\000\033\000\000\000\001\000\000"

	.globl	___CLASS_OutParam
.zerofill __DATA,__common,___CLASS_OutParam,8,3
	.globl	___DROP_FLAG_OFFSET_OutParam
.zerofill __DATA,__common,___DROP_FLAG_OFFSET_OutParam,8,3
	.globl	___IVAR_OFFSET_OutParam
.zerofill __DATA,__common,___IVAR_OFFSET_OutParam,8,3
	.section	__DATA,__data
	.globl	___REGISTER_CLASS_OutParam
	.p2align	3, 0x0
___REGISTER_CLASS_OutParam:
	.asciz	"\003\000\000\000\000\000\000"

.subsections_via_symbols
