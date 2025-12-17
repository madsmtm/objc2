	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin0:
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	cmp	w9, #1
	b.ne	LBB0_15
Lloh0:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh1:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh2:
	ldr	x0, [x8]
Lloh3:
	adrp	x1, l_anon.[ID].12@PAGE
Lloh4:
	add	x1, x1, l_anon.[ID].12@PAGEOFF
	mov	x2, #0
	bl	_objc_allocateClassPair
	cbz	x0, LBB0_16
	str	x0, [sp, #8]
Lloh5:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_7a01db584ce7207f@PAGE
Lloh6:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_7a01db584ce7207f@PAGEOFF]
Ltmp0:
Lloh7:
	adrp	x4, l_anon.[ID].3@PAGE
Lloh8:
	add	x4, x4, l_anon.[ID].3@PAGEOFF
Lloh9:
	adrp	x5, _fn1_get_class@PAGE
Lloh10:
	add	x5, x5, _fn1_get_class@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_class_method_inner, 0)
Ltmp1:
Lloh11:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_7af348e2d6ca08be@PAGE
Lloh12:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_7af348e2d6ca08be@PAGEOFF]
Ltmp2:
Lloh13:
	adrp	x4, l_anon.[ID].4@PAGE
Lloh14:
	add	x4, x4, l_anon.[ID].4@PAGEOFF
Lloh15:
	adrp	x5, _fn2_method_simple@PAGE
Lloh16:
	add	x5, x5, _fn2_method_simple@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp3:
Lloh17:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_59b65ebff80505d9@PAGE
Lloh18:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_59b65ebff80505d9@PAGEOFF]
Ltmp4:
Lloh19:
	adrp	x2, l_anon.[ID].5@PAGE
Lloh20:
	add	x2, x2, l_anon.[ID].5@PAGEOFF
Lloh21:
	adrp	x5, _fn3_method_bool@PAGE
Lloh22:
	add	x5, x5, _fn3_method_bool@PAGEOFF
	add	x0, sp, #8
	mov	w3, #1
	mov	x4, x2
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp5:
Lloh23:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_f8d5ad39f22c9fc5@PAGE
Lloh24:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_f8d5ad39f22c9fc5@PAGEOFF]
Ltmp6:
Lloh25:
	adrp	x4, l_anon.[ID].6@PAGE
Lloh26:
	add	x4, x4, l_anon.[ID].6@PAGEOFF
Lloh27:
	adrp	x5, _fn4_method_retained@PAGE
Lloh28:
	add	x5, x5, _fn4_method_retained@PAGEOFF
	add	x0, sp, #8
	mov	w2, #8
	mov	x3, #0
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp7:
Lloh29:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_31d62824a4f71757@PAGE
Lloh30:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_31d62824a4f71757@PAGEOFF]
Ltmp8:
Lloh31:
	adrp	x2, l_anon.[ID].5@PAGE
Lloh32:
	add	x2, x2, l_anon.[ID].5@PAGEOFF
Lloh33:
	adrp	x4, l_anon.[ID].6@PAGE
Lloh34:
	add	x4, x4, l_anon.[ID].6@PAGEOFF
Lloh35:
	adrp	x5, _fn5_method_retained_with_param@PAGE
Lloh36:
	add	x5, x5, _fn5_method_retained_with_param@PAGEOFF
	add	x0, sp, #8
	mov	w3, #1
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp9:
Ltmp10:
Lloh37:
	adrp	x0, l_anon.[ID].7@PAGE
Lloh38:
	add	x0, x0, l_anon.[ID].7@PAGEOFF
	mov	w1, #8
	bl	SYM(objc2[CRATE_ID]::top_level_traits::get_protocol, 0)
	mov	x1, x0
Ltmp11:
	cbz	x1, LBB0_10
	ldr	x0, [sp, #8]
	bl	_class_addProtocol
LBB0_10:
Ltmp12:
Lloh39:
	adrp	x0, l_anon.[ID].8@PAGE
Lloh40:
	add	x0, x0, l_anon.[ID].8@PAGEOFF
	mov	w1, #9
	bl	SYM(objc2[CRATE_ID]::top_level_traits::get_protocol, 0)
	mov	x1, x0
Ltmp13:
	cbz	x1, LBB0_13
	ldr	x0, [sp, #8]
	bl	_class_addProtocol
LBB0_13:
Lloh41:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_217b1b90b2d164b1@PAGE
Lloh42:
	ldr	x1, [x8, L_OBJC_SELECTOR_REFERENCES_217b1b90b2d164b1@PAGEOFF]
Ltmp14:
Lloh43:
	adrp	x2, l_anon.[ID].11@PAGE
Lloh44:
	add	x2, x2, l_anon.[ID].11@PAGEOFF
Lloh45:
	adrp	x4, l_anon.[ID].6@PAGE
Lloh46:
	add	x4, x4, l_anon.[ID].6@PAGEOFF
Lloh47:
	adrp	x5, _fn6_copyWithZone@PAGE
Lloh48:
	add	x5, x5, _fn6_copyWithZone@PAGEOFF
	add	x0, sp, #8
	mov	w3, #1
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp15:
	ldr	x19, [sp, #8]
	mov	x0, x19
	bl	_objc_registerClassPair
	adrp	x8, ___CLASS_NoIvars@PAGE
	str	x19, [x8, ___CLASS_NoIvars@PAGEOFF]
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB0_15:
Lloh49:
	adrp	x0, l_anon.[ID].2@PAGE
Lloh50:
	add	x0, x0, l_anon.[ID].2@PAGEOFF
	bl	SYM(core[CRATE_ID]::option::unwrap_failed, 0)
LBB0_16:
Lloh51:
	adrp	x0, l_anon.[ID].12@PAGE
Lloh52:
	add	x0, x0, l_anon.[ID].12@PAGEOFF
Lloh53:
	adrp	x2, l_anon.[ID].14@PAGE
Lloh54:
	add	x2, x2, l_anon.[ID].14@PAGEOFF
	mov	w1, #8
	bl	SYM(objc2[CRATE_ID]::__macros::define_class::checks::class_not_unique, 0)
LBB0_17:
Ltmp16:
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
	.loh AdrpAdd	Lloh35, Lloh36
	.loh AdrpAdd	Lloh33, Lloh34
	.loh AdrpAdd	Lloh31, Lloh32
	.loh AdrpLdr	Lloh29, Lloh30
	.loh AdrpAdd	Lloh37, Lloh38
	.loh AdrpAdd	Lloh39, Lloh40
	.loh AdrpAdd	Lloh47, Lloh48
	.loh AdrpAdd	Lloh45, Lloh46
	.loh AdrpAdd	Lloh43, Lloh44
	.loh AdrpLdr	Lloh41, Lloh42
	.loh AdrpAdd	Lloh49, Lloh50
	.loh AdrpAdd	Lloh53, Lloh54
	.loh AdrpAdd	Lloh51, Lloh52
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table0:
Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp15-Ltmp0
	.uleb128 Ltmp16-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp15-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp15
	.byte	0
	.byte	0
Lcst_end0:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(<<std[CRATE_ID]::sync::poison::once::Once>::call_once<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::poison::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	str	x8, [sp, #8]
	add	x0, sp, #8
	bl	SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.globl	SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh55:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh56:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB2_2
Lloh57:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh58:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
	ret
LBB2_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh59:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh60:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh61:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh62:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh63:
	adrp	x4, l_anon.[ID].14@PAGE
Lloh64:
	add	x4, x4, l_anon.[ID].14@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh65:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh66:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh55, Lloh56
	.loh AdrpLdr	Lloh57, Lloh58
	.loh AdrpLdr	Lloh65, Lloh66
	.loh AdrpAdd	Lloh63, Lloh64
	.loh AdrpAdd	Lloh61, Lloh62
	.loh AdrpAdd	Lloh59, Lloh60

	.globl	_fn1_get_class
	.p2align	2
_fn1_get_class:
Lloh67:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh68:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB3_2
Lloh69:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh70:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
	ret
LBB3_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh71:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh72:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh73:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh74:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh75:
	adrp	x4, l_anon.[ID].14@PAGE
Lloh76:
	add	x4, x4, l_anon.[ID].14@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh77:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh78:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh67, Lloh68
	.loh AdrpLdr	Lloh69, Lloh70
	.loh AdrpLdr	Lloh77, Lloh78
	.loh AdrpAdd	Lloh75, Lloh76
	.loh AdrpAdd	Lloh73, Lloh74
	.loh AdrpAdd	Lloh71, Lloh72

	.globl	_fn2_method_simple
	.p2align	2
_fn2_method_simple:
	ret

	.globl	_fn3_method_bool
	.p2align	2
_fn3_method_bool:
	eor	w0, w2, #0x1
	ret

	.globl	_fn4_method_retained
	.p2align	2
_fn4_method_retained:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh79:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh80:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB6_2
LBB6_1:
Lloh81:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh82:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
Lloh83:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh84:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh85:
	ldr	x1, [x8]
	bl	_objc_msgSend
	bl	_objc_autoreleaseReturnValue
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB6_2:
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh86:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh87:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh88:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh89:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh90:
	adrp	x4, l_anon.[ID].14@PAGE
Lloh91:
	add	x4, x4, l_anon.[ID].14@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	b	LBB6_1
	.loh AdrpAdd	Lloh79, Lloh80
	.loh AdrpLdrGotLdr	Lloh83, Lloh84, Lloh85
	.loh AdrpAdrp	Lloh81, Lloh83
	.loh AdrpLdr	Lloh81, Lloh82
	.loh AdrpAdd	Lloh90, Lloh91
	.loh AdrpAdd	Lloh88, Lloh89
	.loh AdrpAdd	Lloh86, Lloh87

	.globl	_fn5_method_retained_with_param
	.p2align	2
_fn5_method_retained_with_param:
Lfunc_begin1:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x20, x2
	bl	SYM(<objc2[CRATE_ID]::runtime::nsobject::NSObject>::new, 0)
	mov	x19, x0
	cbz	w20, LBB7_3
Ltmp17:
	bl	SYM(<objc2[CRATE_ID]::runtime::nsobject::NSObject>::new, 0)
Ltmp18:
	mov	x20, x0
	mov	x0, x19
	bl	_objc_release
	mov	x19, x20
LBB7_3:
	mov	x0, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	_objc_autoreleaseReturnValue
LBB7_4:
Ltmp19:
	mov	x20, x0
Ltmp20:
	mov	x0, x19
	bl	_objc_release
Ltmp21:
	mov	x0, x20
	bl	__Unwind_Resume
LBB7_6:
Ltmp22:
	bl	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end1:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table7:
Lexception1:
	.byte	255
	.byte	155
	.uleb128 Lttbase0-Lttbaseref0
Lttbaseref0:
	.byte	1
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
	.uleb128 Lfunc_begin1-Lfunc_begin1
	.uleb128 Ltmp17-Lfunc_begin1
	.byte	0
	.byte	0
	.uleb128 Ltmp17-Lfunc_begin1
	.uleb128 Ltmp18-Ltmp17
	.uleb128 Ltmp19-Lfunc_begin1
	.byte	0
	.uleb128 Ltmp18-Lfunc_begin1
	.uleb128 Ltmp20-Ltmp18
	.byte	0
	.byte	0
	.uleb128 Ltmp20-Lfunc_begin1
	.uleb128 Ltmp21-Ltmp20
	.uleb128 Ltmp22-Lfunc_begin1
	.byte	1
	.uleb128 Ltmp21-Lfunc_begin1
	.uleb128 Lfunc_end1-Ltmp21
	.byte	0
	.byte	0
Lcst_end1:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn6_copyWithZone
	.p2align	2
_fn6_copyWithZone:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh92:
	adrp	x8, ___REGISTER_CLASS_NoIvars@PAGE
Lloh93:
	add	x8, x8, ___REGISTER_CLASS_NoIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB8_2
LBB8_1:
Lloh94:
	adrp	x8, ___CLASS_NoIvars@PAGE
Lloh95:
	ldr	x0, [x8, ___CLASS_NoIvars@PAGEOFF]
Lloh96:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGE
Lloh97:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_new@GOTPAGEOFF]
Lloh98:
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
Lloh99:
	adrp	x0, ___REGISTER_CLASS_NoIvars@PAGE
Lloh100:
	add	x0, x0, ___REGISTER_CLASS_NoIvars@PAGEOFF
Lloh101:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh102:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh103:
	adrp	x4, l_anon.[ID].14@PAGE
Lloh104:
	add	x4, x4, l_anon.[ID].14@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	b	LBB8_1
	.loh AdrpAdd	Lloh92, Lloh93
	.loh AdrpLdrGotLdr	Lloh96, Lloh97, Lloh98
	.loh AdrpAdrp	Lloh94, Lloh96
	.loh AdrpLdr	Lloh94, Lloh95
	.loh AdrpAdd	Lloh103, Lloh104
	.loh AdrpAdd	Lloh101, Lloh102
	.loh AdrpAdd	Lloh99, Lloh100

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_217b1b90b2d164b1
L_OBJC_METH_VAR_NAME_217b1b90b2d164b1:
	.asciz	"copyWithZone:"

	.globl	L_OBJC_METH_VAR_NAME_31d62824a4f71757
L_OBJC_METH_VAR_NAME_31d62824a4f71757:
	.asciz	"methodRetainedWithParam:"

	.globl	L_OBJC_METH_VAR_NAME_59b65ebff80505d9
L_OBJC_METH_VAR_NAME_59b65ebff80505d9:
	.asciz	"methodBool:"

	.globl	L_OBJC_METH_VAR_NAME_7a01db584ce7207f
L_OBJC_METH_VAR_NAME_7a01db584ce7207f:
	.asciz	"classMethod"

	.globl	L_OBJC_METH_VAR_NAME_7af348e2d6ca08be
L_OBJC_METH_VAR_NAME_7af348e2d6ca08be:
	.asciz	"method"

	.globl	L_OBJC_METH_VAR_NAME_f8d5ad39f22c9fc5
L_OBJC_METH_VAR_NAME_f8d5ad39f22c9fc5:
	.asciz	"methodRetained"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_217b1b90b2d164b1
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_217b1b90b2d164b1:
	.quad	L_OBJC_METH_VAR_NAME_217b1b90b2d164b1

	.globl	L_OBJC_SELECTOR_REFERENCES_31d62824a4f71757
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_31d62824a4f71757:
	.quad	L_OBJC_METH_VAR_NAME_31d62824a4f71757

	.globl	L_OBJC_SELECTOR_REFERENCES_59b65ebff80505d9
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_59b65ebff80505d9:
	.quad	L_OBJC_METH_VAR_NAME_59b65ebff80505d9

	.globl	L_OBJC_SELECTOR_REFERENCES_7a01db584ce7207f
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_7a01db584ce7207f:
	.quad	L_OBJC_METH_VAR_NAME_7a01db584ce7207f

	.globl	L_OBJC_SELECTOR_REFERENCES_7af348e2d6ca08be
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_7af348e2d6ca08be:
	.quad	L_OBJC_METH_VAR_NAME_7af348e2d6ca08be

	.globl	L_OBJC_SELECTOR_REFERENCES_f8d5ad39f22c9fc5
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_f8d5ad39f22c9fc5:
	.quad	L_OBJC_METH_VAR_NAME_f8d5ad39f22c9fc5

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_217b1b90b2d164b1
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_217b1b90b2d164b1:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_31d62824a4f71757
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_31d62824a4f71757:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_59b65ebff80505d9
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_59b65ebff80505d9:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_7a01db584ce7207f
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_7a01db584ce7207f:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_7af348e2d6ca08be
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_7af348e2d6ca08be:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_f8d5ad39f22c9fc5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f8d5ad39f22c9fc5:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::poison::once::Once>::call_once<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::poison::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].1:
	.asciz	"$RUSTC/library/std/src/sync/poison/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].2:
	.quad	l_anon.[ID].1
	.asciz	"\20x\000\000\000\000\000\000\000\234\000\000\0002\000\000"

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
