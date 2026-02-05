	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.p2align	4
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0):
	mov	rax, qword ptr [rip + ___REGISTER_CLASS_NoIvars]
	test	rax, rax
	jne	LBB0_1
	mov	rax, qword ptr [rip + ___CLASS_NoIvars]
	ret
LBB0_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + ___REGISTER_CLASS_NoIvars]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].14]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + ___CLASS_NoIvars]
	ret

	.p2align	4
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, bool, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, bool>, 0):
	push	rbp
	mov	rbp, rsp
	xor	eax, eax
	test	dl, dl
	sete	al
	pop	rbp
	ret

	.p2align	4
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, bool>, 0):
Lfunc_begin0:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	r14d, edx
	call	SYM(<objc2[CRATE_ID]::runtime::nsobject::NSObject>::new, 0)
	mov	rbx, rax
	test	r14b, r14b
	je	LBB2_3
Ltmp0:
	call	SYM(<objc2[CRATE_ID]::runtime::nsobject::NSObject>::new, 0)
Ltmp1:
	mov	r14, rax
	mov	rdi, rbx
	call	_objc_release
	mov	rbx, r14
LBB2_3:
	mov	rdi, rbx
	pop	rbx
	pop	r14
	pop	rbp
	jmp	_objc_autoreleaseReturnValue
LBB2_5:
Ltmp2:
	mov	r14, rax
Ltmp3:
	mov	rdi, rbx
	call	_objc_release
Ltmp4:
	mov	rdi, r14
	call	__Unwind_Resume
LBB2_4:
Ltmp5:
	call	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
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
	.p2align	4
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<test_define_class_no_ivars[CRATE_ID]::NoIvars>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<4u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, *const objc2[CRATE_ID]::runtime::nszone::NSZone>, 0):
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rip + ___REGISTER_CLASS_NoIvars]
	test	rax, rax
	jne	LBB3_1
LBB3_2:
	mov	rdi, qword ptr [rip + ___CLASS_NoIvars]
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_new@GOTPCREL]
	mov	rsi, qword ptr [rax]
	call	_objc_msgSend
	add	rsp, 16
	pop	rbp
	ret
LBB3_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + ___REGISTER_CLASS_NoIvars]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].14]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	jmp	LBB3_2

	.p2align	4
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0):
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rip + ___REGISTER_CLASS_NoIvars]
	test	rax, rax
	jne	LBB4_1
LBB4_2:
	mov	rdi, qword ptr [rip + ___CLASS_NoIvars]
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_new@GOTPCREL]
	mov	rsi, qword ptr [rax]
	call	_objc_msgSend
	mov	rdi, rax
	call	_objc_autoreleaseReturnValue
	add	rsp, 16
	pop	rbp
	ret
LBB4_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + ___REGISTER_CLASS_NoIvars]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].14]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	jmp	LBB4_2

	.p2align	4
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0):
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.p2align	4
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin1:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rax, qword ptr [rdi]
	movzx	ecx, byte ptr [rax]
	mov	byte ptr [rax], 0
	cmp	cl, 1
	jne	LBB6_17
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rdi, qword ptr [rax]
	lea	rsi, [rip + L_anon.[ID].12]
	xor	edx, edx
	call	_objc_allocateClassPair
	test	rax, rax
	je	LBB6_16
	mov	qword ptr [rbp - 16], rax
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_a9fb958c74006297]
Ltmp6:
	lea	r8, [rip + l_anon.[ID].3]
	lea	r9, [rip + SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0)]
	lea	rdi, [rbp - 16]
	mov	edx, 8
	xor	ecx, ecx
	call	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_class_method_inner, 0)
Ltmp7:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_adfe2eb45443b755]
Ltmp8:
	lea	r8, [rip + l_anon.[ID].4]
	lea	r9, [rip + SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0)]
	lea	rdi, [rbp - 16]
	mov	edx, 8
	xor	ecx, ecx
	call	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp9:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_71045e8f6bc4d4f9]
Ltmp10:
	lea	rdx, [rip + l_anon.[ID].5]
	lea	r9, [rip + SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, bool, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, bool>, 0)]
	lea	rdi, [rbp - 16]
	mov	ecx, 1
	mov	r8, rdx
	call	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp11:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_44ec46acb208c3d4]
Ltmp12:
	lea	r8, [rip + l_anon.[ID].6]
	lea	r9, [rip + SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars>, 0)]
	lea	rdi, [rbp - 16]
	mov	edx, 8
	xor	ecx, ecx
	call	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp13:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_d63b759d5c7ed4f0]
Ltmp14:
	lea	rdx, [rip + l_anon.[ID].5]
	lea	r8, [rip + l_anon.[ID].6]
	lea	r9, [rip + SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, bool>, 0)]
	lea	rdi, [rbp - 16]
	mov	ecx, 1
	call	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp15:
Ltmp16:
	lea	rdi, [rip + L_anon.[ID].7]
	mov	esi, 8
	call	SYM(objc2[CRATE_ID]::top_level_traits::get_protocol, 0)
Ltmp17:
	test	rax, rax
	je	LBB6_10
	mov	rdi, qword ptr [rbp - 16]
	mov	rsi, rax
	call	_class_addProtocol
LBB6_10:
Ltmp18:
	lea	rdi, [rip + l_anon.[ID].8]
	mov	esi, 9
	call	SYM(objc2[CRATE_ID]::top_level_traits::get_protocol, 0)
Ltmp19:
	test	rax, rax
	je	LBB6_13
	mov	rdi, qword ptr [rbp - 16]
	mov	rsi, rax
	call	_class_addProtocol
LBB6_13:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_996a3b5043cf563d]
Ltmp20:
	lea	rdx, [rip + l_anon.[ID].11]
	lea	r8, [rip + l_anon.[ID].6]
	lea	r9, [rip + SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &test_define_class_no_ivars[CRATE_ID]::NoIvars, core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<test_define_class_no_ivars[CRATE_ID]::NoIvars>>, objc2[CRATE_ID]::__macros::method_family::MethodFamily<4u8>, test_define_class_no_ivars[CRATE_ID]::NoIvars, *const objc2[CRATE_ID]::runtime::nszone::NSZone>, 0)]
	lea	rdi, [rbp - 16]
	mov	ecx, 1
	call	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_method_inner, 0)
Ltmp21:
	mov	rbx, qword ptr [rbp - 16]
	mov	rdi, rbx
	call	_objc_registerClassPair
	mov	qword ptr [rip + ___CLASS_NoIvars], rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret
LBB6_17:
	lea	rdi, [rip + l_anon.[ID].2]
	call	SYM(core[CRATE_ID]::option::unwrap_failed, 0)
LBB6_16:
	lea	rdi, [rip + L_anon.[ID].12]
	lea	rdx, [rip + l_anon.[ID].14]
	mov	esi, 8
	call	SYM(objc2[CRATE_ID]::__macros::define_class::checks::class_not_unique, 0)
LBB6_15:
Ltmp22:
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 16]
	call	_objc_disposeClassPair
	mov	rdi, rbx
	call	__Unwind_Resume
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
	.p2align	4
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rdi]
	mov	qword ptr [rbp - 8], rax
	lea	rdi, [rbp - 8]
	call	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	add	rsp, 16
	pop	rbp
	ret

	.globl	SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars>::method_retained, 0)
	.p2align	4
SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars>::method_retained, 0):
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rip + ___REGISTER_CLASS_NoIvars]
	test	rax, rax
	jne	LBB8_1
LBB8_2:
	mov	rdi, qword ptr [rip + ___CLASS_NoIvars]
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_new@GOTPCREL]
	mov	rsi, qword ptr [rax]
	call	_objc_msgSend
	add	rsp, 16
	pop	rbp
	ret
LBB8_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + ___REGISTER_CLASS_NoIvars]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].14]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	jmp	LBB8_2

	.globl	SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars>::method_retained_with_param, 0)
	.p2align	4
SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars>::method_retained_with_param, 0):
Lfunc_begin2:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	r14d, esi
	call	SYM(<objc2[CRATE_ID]::runtime::nsobject::NSObject>::new, 0)
	mov	rbx, rax
	test	r14d, r14d
	je	LBB9_3
Ltmp23:
	call	SYM(<objc2[CRATE_ID]::runtime::nsobject::NSObject>::new, 0)
Ltmp24:
	mov	r14, rax
	mov	rdi, rbx
	call	_objc_release
	mov	rbx, r14
LBB9_3:
	mov	rax, rbx
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB9_5:
Ltmp25:
	mov	r14, rax
Ltmp26:
	mov	rdi, rbx
	call	_objc_release
Ltmp27:
	mov	rdi, r14
	call	__Unwind_Resume
LBB9_4:
Ltmp28:
	call	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
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
	.p2align	4
SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars>::get_class, 0):
	mov	rax, qword ptr [rip + ___REGISTER_CLASS_NoIvars]
	test	rax, rax
	jne	LBB10_1
	mov	rax, qword ptr [rip + ___CLASS_NoIvars]
	ret
LBB10_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + ___REGISTER_CLASS_NoIvars]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].14]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + ___CLASS_NoIvars]
	ret

	.globl	SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars>::copy_with_zone, 0)
	.p2align	4
SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars>::copy_with_zone, 0):
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rip + ___REGISTER_CLASS_NoIvars]
	test	rax, rax
	jne	LBB11_1
LBB11_2:
	mov	rdi, qword ptr [rip + ___CLASS_NoIvars]
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_new@GOTPCREL]
	mov	rsi, qword ptr [rax]
	call	_objc_msgSend
	add	rsp, 16
	pop	rbp
	ret
LBB11_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + ___REGISTER_CLASS_NoIvars]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].14]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	jmp	LBB11_2

	.globl	SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	4
SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
	mov	rax, qword ptr [rip + ___REGISTER_CLASS_NoIvars]
	test	rax, rax
	jne	LBB12_1
	mov	rax, qword ptr [rip + ___CLASS_NoIvars]
	ret
LBB12_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + ___REGISTER_CLASS_NoIvars]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].14]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + ___CLASS_NoIvars]
	ret

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
L_anon.[ID].1:
	.asciz	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].2:
	.quad	L_anon.[ID].1
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
	.space	1
	.space	39

	.p2align	3, 0x0
l_anon.[ID].6:
	.byte	21
	.space	39

	.section	__TEXT,__literal8,8byte_literals
L_anon.[ID].7:
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
L_anon.[ID].12:
	.asciz	"NoIvars"

L_anon.[ID].13:
	.asciz	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].14:
	.quad	L_anon.[ID].13
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
