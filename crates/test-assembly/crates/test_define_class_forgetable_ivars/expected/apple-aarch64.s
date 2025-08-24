	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin0:
	sub	sp, sp, #80
	stp	x20, x19, [sp, #48]
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	ldr	x8, [x0]
	ldrb	w9, [x8]
	strb	wzr, [x8]
	cmp	w9, #1
	b.ne	LBB0_6
Lloh0:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh1:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh2:
	ldr	x0, [x8]
Lloh3:
	adrp	x1, l_anon.[ID].7@PAGE
Lloh4:
	add	x1, x1, l_anon.[ID].7@PAGEOFF
	mov	x2, #0
	bl	_objc_allocateClassPair
	cbz	x0, LBB0_7
	str	x0, [sp]
Lloh5:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh6:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh7:
	ldr	x1, [x8]
Ltmp0:
Lloh8:
	adrp	x4, l_anon.[ID].8@PAGE
Lloh9:
	add	x4, x4, l_anon.[ID].8@PAGEOFF
Lloh10:
	adrp	x5, _fn1_init@PAGE
Lloh11:
	add	x5, x5, _fn1_init@PAGEOFF
	mov	x0, sp
	mov	w2, #8
	mov	x3, #0
	bl	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp1:
	mov	w8, #2
Lloh12:
	adrp	x9, l_anon.[ID].2@PAGE
Lloh13:
	add	x9, x9, l_anon.[ID].2@PAGEOFF
	stp	x8, x9, [sp, #16]
	mov	w8, #27
	strb	w8, [sp, #8]
Ltmp2:
Lloh14:
	adrp	x1, l_anon.[ID].1@PAGE
Lloh15:
	add	x1, x1, l_anon.[ID].1@PAGEOFF
	mov	x0, sp
	add	x5, sp, #8
	mov	w2, #6
	mov	w3, #8
	mov	w4, #2
	bl	SYM(objc2::runtime::define::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Ltmp3:
	ldr	x19, [sp]
	mov	x0, x19
	bl	_objc_registerClassPair
Lloh16:
	adrp	x1, l_anon.[ID].1@PAGE
Lloh17:
	add	x1, x1, l_anon.[ID].1@PAGEOFF
	mov	x0, x19
	bl	_class_getInstanceVariable
	cbz	x0, LBB0_8
	bl	_ivar_getOffset
Lloh18:
	adrp	x8, ___CLASS_ForgetableIvars@PAGE
	str	x19, [x8, ___CLASS_ForgetableIvars@PAGEOFF]
Lloh19:
	adrp	x8, ___IVAR_OFFSET_ForgetableIvars@PAGE
	str	x0, [x8, ___IVAR_OFFSET_ForgetableIvars@PAGEOFF]
	ldp	x29, x30, [sp, #64]
	ldp	x20, x19, [sp, #48]
	add	sp, sp, #80
	ret
LBB0_6:
Lloh20:
	adrp	x0, l_anon.[ID].4@PAGE
Lloh21:
	add	x0, x0, l_anon.[ID].4@PAGEOFF
	bl	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB0_7:
Lloh22:
	adrp	x0, l_anon.[ID].7@PAGE
Lloh23:
	add	x0, x0, l_anon.[ID].7@PAGEOFF
Lloh24:
	adrp	x2, l_anon.[ID].6@PAGE
Lloh25:
	add	x2, x2, l_anon.[ID].6@PAGEOFF
	mov	w1, #16
	bl	SYM(objc2::__macro_helpers::define_class::class_not_unique::GENERATED_ID, 0)
LBB0_8:
	bl	SYM(objc2::__macro_helpers::defined_ivars::ivars_offset::get_ivar_failed::GENERATED_ID, 0)
LBB0_9:
Ltmp4:
	mov	x19, x0
	ldr	x0, [sp]
	bl	_objc_disposeClassPair
	mov	x0, x19
	bl	__Unwind_Resume
	.loh AdrpAdd	Lloh3, Lloh4
	.loh AdrpLdrGotLdr	Lloh0, Lloh1, Lloh2
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpLdrGotLdr	Lloh5, Lloh6, Lloh7
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdrp	Lloh18, Lloh19
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpAdd	Lloh24, Lloh25
	.loh AdrpAdd	Lloh22, Lloh23
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
SYM(<<std[CRATE_ID]::sync::poison::once::Once>::call_once<<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::poison::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	ldr	x8, [x0]
	str	x8, [sp, #8]
	add	x0, sp, #8
	bl	SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.globl	_fn2_access_class
	.p2align	2
_fn2_access_class:
Lloh26:
	adrp	x8, ___REGISTER_CLASS_ForgetableIvars@PAGE
Lloh27:
	add	x8, x8, ___REGISTER_CLASS_ForgetableIvars@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB2_2
Lloh28:
	adrp	x8, ___CLASS_ForgetableIvars@PAGE
Lloh29:
	ldr	x0, [x8, ___CLASS_ForgetableIvars@PAGEOFF]
	ret
LBB2_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	w8, #1
	strb	w8, [sp, #7]
	add	x8, sp, #7
	str	x8, [sp, #8]
Lloh30:
	adrp	x0, ___REGISTER_CLASS_ForgetableIvars@PAGE
Lloh31:
	add	x0, x0, ___REGISTER_CLASS_ForgetableIvars@PAGEOFF
Lloh32:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh33:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh34:
	adrp	x4, l_anon.[ID].6@PAGE
Lloh35:
	add	x4, x4, l_anon.[ID].6@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh36:
	adrp	x8, ___CLASS_ForgetableIvars@PAGE
Lloh37:
	ldr	x0, [x8, ___CLASS_ForgetableIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpLdr	Lloh28, Lloh29
	.loh AdrpLdr	Lloh36, Lloh37
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpAdd	Lloh32, Lloh33
	.loh AdrpAdd	Lloh30, Lloh31

	.globl	_fn3_access_ivars
	.p2align	2
_fn3_access_ivars:
Lloh38:
	adrp	x8, ___IVAR_OFFSET_ForgetableIvars@PAGE
Lloh39:
	ldr	x8, [x8, ___IVAR_OFFSET_ForgetableIvars@PAGEOFF]
	add	x8, x0, x8
	ldr	w1, [x8]
	ldrb	w0, [x8, #4]
	ret
	.loh AdrpLdr	Lloh38, Lloh39

	.globl	SYM(<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	2
SYM(<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
Lloh40:
	adrp	x8, ___REGISTER_CLASS_ForgetableIvars@PAGE
Lloh41:
	add	x8, x8, ___REGISTER_CLASS_ForgetableIvars@PAGEOFF
	ldapr	x8, [x8]
	cmp	x8, #3
	b.ne	LBB4_2
Lloh42:
	adrp	x8, ___CLASS_ForgetableIvars@PAGE
Lloh43:
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
Lloh44:
	adrp	x0, ___REGISTER_CLASS_ForgetableIvars@PAGE
Lloh45:
	add	x0, x0, ___REGISTER_CLASS_ForgetableIvars@PAGEOFF
Lloh46:
	adrp	x3, l_anon.[ID].0@PAGE
Lloh47:
	add	x3, x3, l_anon.[ID].0@PAGEOFF
Lloh48:
	adrp	x4, l_anon.[ID].6@PAGE
Lloh49:
	add	x4, x4, l_anon.[ID].6@PAGEOFF
	add	x2, sp, #8
	mov	w1, #0
	bl	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
Lloh50:
	adrp	x8, ___CLASS_ForgetableIvars@PAGE
Lloh51:
	ldr	x0, [x8, ___CLASS_ForgetableIvars@PAGEOFF]
	ret
	.loh AdrpAdd	Lloh40, Lloh41
	.loh AdrpLdr	Lloh42, Lloh43
	.loh AdrpLdr	Lloh50, Lloh51
	.loh AdrpAdd	Lloh48, Lloh49
	.loh AdrpAdd	Lloh46, Lloh47
	.loh AdrpAdd	Lloh44, Lloh45

	.globl	_fn1_init
	.p2align	2
_fn1_init:
	cbz	x0, LBB5_2
Lloh52:
	adrp	x8, ___IVAR_OFFSET_ForgetableIvars@PAGE
Lloh53:
	ldr	x8, [x8, ___IVAR_OFFSET_ForgetableIvars@PAGEOFF]
	add	x8, x0, x8
	mov	w9, #43
	str	w9, [x8]
	mov	w9, #42
	strb	w9, [x8, #4]
LBB5_2:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
Lloh54:
	adrp	x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGE
Lloh55:
	ldr	x8, [x8, L_OBJC_SELECTOR_REFERENCES_init@GOTPAGEOFF]
Lloh56:
	ldr	x1, [x8]
Lloh57:
	adrp	x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGE
Lloh58:
	ldr	x8, [x8, L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPAGEOFF]
Lloh59:
	ldr	x8, [x8]
	stp	x0, x8, [sp]
	mov	x0, sp
	bl	_objc_msgSendSuper
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpLdr	Lloh52, Lloh53
	.loh AdrpLdrGotLdr	Lloh57, Lloh58, Lloh59
	.loh AdrpLdrGotLdr	Lloh54, Lloh55, Lloh56

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::poison::once::Once>::call_once<<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::poison::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].1:
	.asciz	"ivars"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].2:
	.byte	7
	.space	39

l_anon.[ID].3:
	.ascii	"$RUSTC/library/std/src/sync/poison/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].4:
	.quad	l_anon.[ID].3
	.asciz	"\20x\000\000\000\000\000\000\000\233\000\000\0002\000\000"

	.globl	___CLASS_ForgetableIvars
.zerofill __DATA,__common,___CLASS_ForgetableIvars,8,3
	.globl	___IVAR_OFFSET_ForgetableIvars
.zerofill __DATA,__common,___IVAR_OFFSET_ForgetableIvars,8,3
	.globl	___DROP_FLAG_OFFSET_ForgetableIvars
.zerofill __DATA,__common,___DROP_FLAG_OFFSET_ForgetableIvars,8,3
	.section	__TEXT,__const
l_anon.[ID].5:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].6:
	.quad	l_anon.[ID].5
	.asciz	"E\000\000\000\000\000\000\000\022\000\000\000\001\000\000"

	.globl	___REGISTER_CLASS_ForgetableIvars
.zerofill __DATA,__common,___REGISTER_CLASS_ForgetableIvars,8,3
	.section	__TEXT,__cstring,cstring_literals
l_anon.[ID].7:
	.asciz	"ForgetableIvars"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].8:
	.byte	19
	.space	39

.subsections_via_symbols
