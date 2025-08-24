	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.p2align	4
SYM(objc2[CRATE_ID]::__macro_helpers::defined_ivars::dealloc::<test_define_class_drop_ivars[CRATE_ID]::DropIvars>, 0):
Lfunc_begin0:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	sub	rsp, 24
	mov	rbx, rsi
	mov	r14, rdi
	mov	rax, qword ptr [rip + ___DROP_FLAG_OFFSET_DropIvars]
	movzx	eax, byte ptr [rdi + rax]
	cmp	eax, 255
	jne	LBB0_1
	call	_fn2_drop
LBB0_4:
	mov	rax, qword ptr [rip + ___IVAR_OFFSET_DropIvars]
	mov	rdi, qword ptr [r14 + rax]
	mov	r15, qword ptr [r14 + rax + 8]
Ltmp0:
	call	_objc_release
Ltmp1:
	test	r15, r15
	je	LBB0_2
	mov	rdi, r15
	call	_objc_release
LBB0_2:
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rax, qword ptr [rax]
	mov	qword ptr [rbp - 40], r14
	mov	qword ptr [rbp - 32], rax
	lea	rdi, [rbp - 40]
	mov	rsi, rbx
	call	_objc_msgSendSuper
	add	rsp, 24
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB0_1:
	test	eax, eax
	jne	LBB0_4
	jmp	LBB0_2
LBB0_7:
Ltmp2:
	mov	rbx, rax
	test	r15, r15
	je	LBB0_9
Ltmp3:
	mov	rdi, r15
	call	_objc_release
Ltmp4:
LBB0_9:
	mov	rdi, rbx
	call	__Unwind_Resume
LBB0_10:
Ltmp5:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table0:
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
	.p2align	4
SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class_drop_ivars[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin1:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	sub	rsp, 48
	mov	rax, qword ptr [rdi]
	movzx	ecx, byte ptr [rax]
	mov	byte ptr [rax], 0
	cmp	cl, 1
	jne	LBB1_11
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rdi, qword ptr [rax]
	lea	rsi, [rip + L_anon.[ID].9]
	xor	edx, edx
	call	_objc_allocateClassPair
	test	rax, rax
	je	LBB1_8
	mov	qword ptr [rbp - 24], rax
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPCREL]
	mov	rsi, qword ptr [rax]
Ltmp6:
	lea	r8, [rip + l_anon.[ID].3]
	lea	r9, [rip + SYM(objc2[CRATE_ID]::__macro_helpers::defined_ivars::dealloc::<test_define_class_drop_ivars[CRATE_ID]::DropIvars>, 0)]
	lea	rdi, [rbp - 24]
	mov	edx, 8
	xor	ecx, ecx
	call	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp7:
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_init@GOTPCREL]
	mov	rsi, qword ptr [rax]
Ltmp8:
	lea	r8, [rip + l_anon.[ID].8]
	lea	r9, [rip + _fn1_init]
	lea	rdi, [rbp - 24]
	mov	edx, 8
	xor	ecx, ecx
	call	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp9:
	mov	qword ptr [rbp - 56], 2
	lea	rax, [rip + l_anon.[ID].5]
	mov	qword ptr [rbp - 48], rax
	mov	byte ptr [rbp - 64], 27
Ltmp10:
	lea	rsi, [rip + L_anon.[ID].1]
	lea	rdi, [rbp - 24]
	lea	r9, [rbp - 64]
	mov	edx, 6
	mov	ecx, 16
	mov	r8d, 3
	call	SYM(objc2::runtime::define::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Ltmp11:
Ltmp12:
	lea	rsi, [rip + L_anon.[ID].2]
	lea	r9, [rip + l_anon.[ID].4]
	lea	rdi, [rbp - 24]
	mov	edx, 10
	mov	ecx, 1
	xor	r8d, r8d
	call	SYM(objc2::runtime::define::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Ltmp13:
	mov	rbx, qword ptr [rbp - 24]
	mov	rdi, rbx
	call	_objc_registerClassPair
	lea	rsi, [rip + L_anon.[ID].1]
	mov	rdi, rbx
	call	_class_getInstanceVariable
	test	rax, rax
	je	LBB1_7
	mov	rdi, rax
	call	_ivar_getOffset
	mov	r14, rax
	lea	rsi, [rip + L_anon.[ID].2]
	mov	rdi, rbx
	call	_class_getInstanceVariable
	test	rax, rax
	je	LBB1_13
	mov	rdi, rax
	call	_ivar_getOffset
	mov	qword ptr [rip + ___CLASS_DropIvars], rbx
	mov	qword ptr [rip + ___IVAR_OFFSET_DropIvars], r14
	mov	qword ptr [rip + ___DROP_FLAG_OFFSET_DropIvars], rax
	add	rsp, 48
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB1_11:
	lea	rdi, [rip + l_anon.[ID].7]
	call	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB1_8:
	lea	rdi, [rip + L_anon.[ID].9]
	lea	rdx, [rip + l_anon.[ID].11]
	mov	esi, 10
	call	SYM(objc2::__macro_helpers::define_class::class_not_unique::GENERATED_ID, 0)
LBB1_7:
	call	SYM(objc2::__macro_helpers::defined_ivars::ivars_offset::get_ivar_failed::GENERATED_ID, 0)
LBB1_13:
	call	SYM(objc2::__macro_helpers::defined_ivars::drop_flag_offset::get_drop_flag_failed::GENERATED_ID, 0)
LBB1_12:
Ltmp14:
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 24]
	call	_objc_disposeClassPair
	mov	rdi, rbx
	call	__Unwind_Resume
Lfunc_end1:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table1:
Lexception1:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
	.uleb128 Ltmp6-Lfunc_begin1
	.uleb128 Ltmp13-Ltmp6
	.uleb128 Ltmp14-Lfunc_begin1
	.byte	0
	.uleb128 Ltmp13-Lfunc_begin1
	.uleb128 Lfunc_end1-Ltmp13
	.byte	0
	.byte	0
Lcst_end1:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	4
SYM(<<std[CRATE_ID]::sync::poison::once::Once>::call_once<<test_define_class_drop_ivars[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::poison::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rdi]
	mov	qword ptr [rbp - 8], rax
	lea	rdi, [rbp - 8]
	call	SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class_drop_ivars[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	add	rsp, 16
	pop	rbp
	ret

	.globl	SYM(<test_define_class_drop_ivars[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	4
SYM(<test_define_class_drop_ivars[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
	mov	rax, qword ptr [rip + ___REGISTER_CLASS_DropIvars]
	test	rax, rax
	jne	LBB3_1
	mov	rax, qword ptr [rip + ___CLASS_DropIvars]
	ret
LBB3_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + ___REGISTER_CLASS_DropIvars]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].11]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + ___CLASS_DropIvars]
	ret

	.globl	_fn1_init
	.p2align	4
_fn1_init:
Lfunc_begin2:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r12
	push	rbx
	sub	rsp, 16
	mov	rbx, rdi
Ltmp15:
	call	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
Ltmp16:
Ltmp18:
	mov	r15, rax
	call	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
Ltmp19:
	mov	r12, rax
	test	rbx, rbx
	je	LBB4_3
	mov	rax, qword ptr [rip + ___IVAR_OFFSET_DropIvars]
	mov	qword ptr [rbx + rax], r15
	mov	qword ptr [rbx + rax + 8], r12
	mov	rax, qword ptr [rip + ___DROP_FLAG_OFFSET_DropIvars]
	mov	byte ptr [rbx + rax], 15
	jmp	LBB4_9
LBB4_3:
Ltmp26:
	mov	rdi, r15
	call	_objc_release
Ltmp27:
	mov	rdi, r12
	call	_objc_release
LBB4_9:
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_init@GOTPCREL]
	mov	rsi, qword ptr [rax]
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rax, qword ptr [rax]
	mov	qword ptr [rbp - 48], rbx
	mov	qword ptr [rbp - 40], rax
	lea	rdi, [rbp - 48]
	call	_objc_msgSendSuper
	test	rax, rax
	je	LBB4_11
	mov	rcx, qword ptr [rip + ___DROP_FLAG_OFFSET_DropIvars]
	mov	byte ptr [rax + rcx], -1
LBB4_11:
	add	rsp, 16
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB4_6:
Ltmp28:
	mov	r14, rax
Ltmp29:
	mov	rdi, r12
	call	_objc_release
Ltmp30:
	jmp	LBB4_15
LBB4_7:
Ltmp31:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB4_5:
Ltmp20:
	mov	r14, rax
Ltmp21:
	mov	rdi, r15
	call	_objc_release
Ltmp22:
	jmp	LBB4_14
LBB4_13:
Ltmp17:
	mov	r14, rax
LBB4_14:
Ltmp23:
	mov	rdi, rbx
	call	_objc_release
Ltmp24:
LBB4_15:
	mov	rdi, r14
	call	__Unwind_Resume
LBB4_12:
Ltmp25:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end2:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table4:
Lexception2:
	.byte	255
	.byte	155
	.uleb128 Lttbase1-Lttbaseref1
Lttbaseref1:
	.byte	1
	.uleb128 Lcst_end2-Lcst_begin2
Lcst_begin2:
	.uleb128 Ltmp15-Lfunc_begin2
	.uleb128 Ltmp16-Ltmp15
	.uleb128 Ltmp17-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp18-Lfunc_begin2
	.uleb128 Ltmp19-Ltmp18
	.uleb128 Ltmp20-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp26-Lfunc_begin2
	.uleb128 Ltmp27-Ltmp26
	.uleb128 Ltmp28-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp27-Lfunc_begin2
	.uleb128 Ltmp29-Ltmp27
	.byte	0
	.byte	0
	.uleb128 Ltmp29-Lfunc_begin2
	.uleb128 Ltmp30-Ltmp29
	.uleb128 Ltmp31-Lfunc_begin2
	.byte	1
	.uleb128 Ltmp21-Lfunc_begin2
	.uleb128 Ltmp24-Ltmp21
	.uleb128 Ltmp25-Lfunc_begin2
	.byte	1
	.uleb128 Ltmp24-Lfunc_begin2
	.uleb128 Lfunc_end2-Ltmp24
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
	.globl	_fn2_drop
	.p2align	4
_fn2_drop:
	push	rbp
	mov	rbp, rsp
	## InlineAsm Start
	## InlineAsm End
	pop	rbp
	ret

	.globl	_fn3_access_class
	.p2align	4
_fn3_access_class:
	mov	rax, qword ptr [rip + ___REGISTER_CLASS_DropIvars]
	test	rax, rax
	jne	LBB6_1
	mov	rax, qword ptr [rip + ___CLASS_DropIvars]
	ret
LBB6_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + ___REGISTER_CLASS_DropIvars]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].11]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + ___CLASS_DropIvars]
	ret

	.globl	_fn3_access_ivars
	.p2align	4
_fn3_access_ivars:
	push	rbp
	mov	rbp, rsp
	mov	rcx, qword ptr [rip + ___IVAR_OFFSET_DropIvars]
	mov	rax, qword ptr [rdi + rcx]
	mov	rdx, qword ptr [rdi + rcx + 8]
	pop	rbp
	ret

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::poison::once::Once>::call_once<<test_define_class_drop_ivars[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::poison::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class_drop_ivars[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].1:
	.asciz	"ivars"

L_anon.[ID].2:
	.asciz	"drop_flag"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].3:
	.byte	17
	.space	39

	.p2align	3, 0x0
l_anon.[ID].4:
	.byte	5
	.space	39

	.p2align	3, 0x0
l_anon.[ID].5:
	.byte	9
	.space	39

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].6:
	.asciz	"$RUSTC/library/std/src/sync/poison/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].7:
	.quad	L_anon.[ID].6
	.asciz	"\20x\000\000\000\000\000\000\000\234\000\000\0002\000\000"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].8:
	.byte	19
	.space	39

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].9:
	.asciz	"DropIvars"

L_anon.[ID].10:
	.asciz	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].11:
	.quad	L_anon.[ID].10
	.asciz	"?\000\000\000\000\000\000\000\023\000\000\000\001\000\000"

	.globl	___CLASS_DropIvars
.zerofill __DATA,__common,___CLASS_DropIvars,8,3
	.globl	___DROP_FLAG_OFFSET_DropIvars
.zerofill __DATA,__common,___DROP_FLAG_OFFSET_DropIvars,8,3
	.globl	___IVAR_OFFSET_DropIvars
.zerofill __DATA,__common,___IVAR_OFFSET_DropIvars,8,3
	.section	__DATA,__data
	.globl	___REGISTER_CLASS_DropIvars
	.p2align	3, 0x0
___REGISTER_CLASS_DropIvars:
	.asciz	"\003\000\000\000\000\000\000"

.subsections_via_symbols
