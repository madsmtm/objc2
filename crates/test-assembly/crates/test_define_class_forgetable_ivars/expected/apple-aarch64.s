	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, objc2[CRATE_ID]::rc::allocated_partial_init::Allocated<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars>, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<3u8>, test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars>, 0):
	cbz	x0, LBB0_2
Lloh0:
	adrp	x8, ___IVAR_OFFSET_ForgetableIvars@PAGE
Lloh1:
	ldr	x8, [x8, ___IVAR_OFFSET_ForgetableIvars@PAGEOFF]
	add	x8, x0, x8
	mov	w9, #43
	str	w9, [x8]
	mov	w9, #42
	strb	w9, [x8, #4]
LBB0_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh2:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh3:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh4:
	ldr	x1, [x8]
Lloh5:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh6:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh7:
	ldr	x8, [x8]
	stp	x0, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpLdr	Lloh0, Lloh1
	.loh AdrpLdrGotLdr	Lloh5, Lloh6, Lloh7
	.loh AdrpLdrGotLdr	Lloh2, Lloh3, Lloh4

	.p2align	2
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin0:
	sub	sp, sp, #80
	stp	x20, x19, [sp, #48]
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	cmp	w9, #1
	b.ne	LBB1_6
Lloh8:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh9:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh10:
	ldr	x0, [x8]
Lloh11:
	adrp	x1, l_anon.[ID].6@PAGE
Lloh12:
	add	x1, x1, l_anon.[ID].6@PAGEOFF
	mov	x2, #0
	bl	_objc_allocateClassPair
	cbz	x0, LBB1_7
	str	x0, [sp]
Lloh13:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh14:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh15:
	ldr	x1, [x8]
Ltmp0:
Lloh16:
	adrp	x4, l_anon.[ID].5@PAGE
Lloh17:
	add	x4, x4, l_anon.[ID].5@PAGEOFF
Lloh18:
	adrp	x5, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, objc2[CRATE_ID]::rc::allocated_partial_init::Allocated<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars>, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<3u8>, test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars>, 0)@PAGE
Lloh19:
	add	x5, x5, SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, objc2[CRATE_ID]::rc::allocated_partial_init::Allocated<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars>, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<3u8>, test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars>, 0)@PAGEOFF
	mov	x0, sp
	mov	w2, #8
	mov	x3, #0
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp1:
	mov	w8, #2
Lloh20:
	adrp	x9, l_anon.[ID].2@PAGE
Lloh21:
	add	x9, x9, l_anon.[ID].2@PAGEOFF
	stp	x8, x9, [sp, #16]
	mov	w8, #29
	strb	w8, [sp, #8]
Ltmp2:
Lloh22:
	adrp	x1, l_anon.[ID].1@PAGE
Lloh23:
	add	x1, x1, l_anon.[ID].1@PAGEOFF
	mov	x0, sp
	add	x5, sp, #8
	mov	w2, #6
	mov	w3, #8
	mov	w4, #2
	bl	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_ivar_inner_mono, 0)
Ltmp3:
	ldr	x19, [sp]
	mov	x0, x19
	bl	_objc_registerClassPair
Lloh24:
	adrp	x1, l_anon.[ID].1@PAGE
Lloh25:
	add	x1, x1, l_anon.[ID].1@PAGEOFF
	mov	x0, x19
	bl	_class_getInstanceVariable
	cbz	x0, LBB1_8
	bl	_ivar_getOffset
Lloh26:
	adrp	x8, ___CLASS_ForgetableIvars@PAGE
	str	x19, [x8, ___CLASS_ForgetableIvars@PAGEOFF]
Lloh27:
	adrp	x8, ___IVAR_OFFSET_ForgetableIvars@PAGE
	str	x0, [x8, ___IVAR_OFFSET_ForgetableIvars@PAGEOFF]
	ldp	x29, x30, [sp, #64]
	ldp	x20, x19, [sp, #48]
	add	sp, sp, #80
	ret
LBB1_6:
Lloh28:
	adrp	x0, l_anon.[ID].4@PAGE
Lloh29:
	add	x0, x0, l_anon.[ID].4@PAGEOFF
	bl	SYM(core[CRATE_ID]::option::unwrap_failed, 0)
LBB1_7:
Lloh30:
	adrp	x0, l_anon.[ID].6@PAGE
Lloh31:
	add	x0, x0, l_anon.[ID].6@PAGEOFF
Lloh32:
	adrp	x2, l_anon.[ID].8@PAGE
Lloh33:
	add	x2, x2, l_anon.[ID].8@PAGEOFF
	mov	w1, #16
	bl	SYM(objc2[CRATE_ID]::__macros::define_class::checks::class_not_unique, 0)
LBB1_8:
	bl	SYM(objc2[CRATE_ID]::__macros::define_class::ivars::ivars_offset::get_ivar_failed, 0)
LBB1_9:
Ltmp4:
	mov	x19, x0
	ldr	x0, [sp]
	bl	_objc_disposeClassPair
	mov	x0, x19
	bl	__Unwind_Resume
	.loh AdrpAdd	Lloh11, Lloh12
	.loh AdrpLdrGotLdr	Lloh8, Lloh9, Lloh10
	.loh AdrpAdd	Lloh18, Lloh19
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpLdrGotLdr	Lloh13, Lloh14, Lloh15
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpAdd	Lloh24, Lloh25
	.loh AdrpAdrp	Lloh26, Lloh27
	.loh AdrpAdd	Lloh28, Lloh29
	.loh AdrpAdd	Lloh32, Lloh33
	.loh AdrpAdd	Lloh30, Lloh31
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table1:
Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp3-Ltmp0
	.uleb128 Ltmp4-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp3-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp3
	.byte	0
	.byte	0
Lcst_end0:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	str	x8, [sp, #8]
	add	x0, sp, #8
	bl	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.globl	SYM(<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars>::init, 0)
	.p2align	2
SYM(<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars>::init, 0):
	cbz	x0, LBB3_2
Lloh34:
	adrp	x8, ___IVAR_OFFSET_ForgetableIvars@PAGE
Lloh35:
	ldr	x8, [x8, ___IVAR_OFFSET_ForgetableIvars@PAGEOFF]
	add	x8, x0, x8
	mov	w9, #43
	str	w9, [x8]
	mov	w9, #42
	strb	w9, [x8, #4]
LBB3_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh36:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh37:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh38:
	ldr	x1, [x8]
Lloh39:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh40:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh41:
	ldr	x8, [x8]
	stp	x0, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpLdr	Lloh34, Lloh35
	.loh AdrpLdrGotLdr	Lloh39, Lloh40, Lloh41
	.loh AdrpLdrGotLdr	Lloh36, Lloh37, Lloh38

	.globl	SYM(<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh42:
	adrp	x8, ___REGISTER_CLASS_ForgetableIvars@PAGE
Lloh43:
	add	x8, x8, ___REGISTER_CLASS_ForgetableIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB4_2
Lloh44:
	adrp	x8, ___CLASS_ForgetableIvars@PAGE
Lloh45:
	ldr	x0, [x8, ___CLASS_ForgetableIvars@PAGEOFF]
	ret
LBB4_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh46:
	adrp	x0, ___REGISTER_CLASS_ForgetableIvars@PAGE
Lloh47:
	add	x0, x0, ___REGISTER_CLASS_ForgetableIvars@PAGEOFF
Lloh48:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh49:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh50:
	adrp	x4, l_anon.[ID].8@PAGE
Lloh51:
	add	x4, x4, l_anon.[ID].8@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh52:
	adrp	x8, ___CLASS_ForgetableIvars@PAGE
Lloh53:
	ldr	x0, [x8, ___CLASS_ForgetableIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh42, Lloh43
	.loh AdrpLdr	Lloh44, Lloh45
	.loh AdrpLdr	Lloh52, Lloh53
	.loh AdrpAdd	Lloh50, Lloh51
	.loh AdrpAdd	Lloh48, Lloh49
	.loh AdrpAdd	Lloh46, Lloh47

	.globl	_fn_access_class
	.p2align	2
_fn_access_class:
Lloh54:
	adrp	x8, ___REGISTER_CLASS_ForgetableIvars@PAGE
Lloh55:
	add	x8, x8, ___REGISTER_CLASS_ForgetableIvars@PAGEOFF
	ldapr	x8, [x8]
	cbnz	x8, LBB5_2
Lloh56:
	adrp	x8, ___CLASS_ForgetableIvars@PAGE
Lloh57:
	ldr	x0, [x8, ___CLASS_ForgetableIvars@PAGEOFF]
	ret
LBB5_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh58:
	adrp	x0, ___REGISTER_CLASS_ForgetableIvars@PAGE
Lloh59:
	add	x0, x0, ___REGISTER_CLASS_ForgetableIvars@PAGEOFF
Lloh60:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh61:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh62:
	adrp	x4, l_anon.[ID].8@PAGE
Lloh63:
	add	x4, x4, l_anon.[ID].8@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh64:
	adrp	x8, ___CLASS_ForgetableIvars@PAGE
Lloh65:
	ldr	x0, [x8, ___CLASS_ForgetableIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh54, Lloh55
	.loh AdrpLdr	Lloh56, Lloh57
	.loh AdrpLdr	Lloh64, Lloh65
	.loh AdrpAdd	Lloh62, Lloh63
	.loh AdrpAdd	Lloh60, Lloh61
	.loh AdrpAdd	Lloh58, Lloh59

	.globl	_fn_access_ivars
	.p2align	2
_fn_access_ivars:
Lloh66:
	adrp	x8, ___IVAR_OFFSET_ForgetableIvars@PAGE
Lloh67:
	ldr	x8, [x8, ___IVAR_OFFSET_ForgetableIvars@PAGEOFF]
	add	x8, x0, x8
	ldr	w1, [x8]
	ldrb	w0, [x8, #4]
	ret
	.loh AdrpLdr	Lloh66, Lloh67

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].1:
	.asciz	"ivars"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].2:
	.byte	8
	.space	39

	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].3:
	.asciz	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].4:
	.quad	l_anon.[ID].3
	.asciz	"p\000\000\000\000\000\000\000\237\000\000\0002\000\000"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].5:
	.byte	21
	.space	39

	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].6:
	.asciz	"ForgetableIvars"

l_anon.[ID].7:
	.asciz	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].8:
	.quad	l_anon.[ID].7
	.asciz	"E\000\000\000\000\000\000\000\r\000\000\000\001\000\000"

	.globl	___CLASS_ForgetableIvars
.zerofill __DATA,__common,___CLASS_ForgetableIvars,8,3
	.globl	___DROP_FLAG_OFFSET_ForgetableIvars
.zerofill __DATA,__common,___DROP_FLAG_OFFSET_ForgetableIvars,8,3
	.globl	___IVAR_OFFSET_ForgetableIvars
.zerofill __DATA,__common,___IVAR_OFFSET_ForgetableIvars,8,3
	.section	__DATA,__data
	.globl	___REGISTER_CLASS_ForgetableIvars
	.p2align	3, 0x0
___REGISTER_CLASS_ForgetableIvars:
	.asciz	"\003\000\000\000\000\000\000"

.subsections_via_symbols
