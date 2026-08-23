	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.p2align	4
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, &mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, 0):
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdx
	mov	rdi, qword ptr [rdx]
	call	_objc_retain
	mov	rdi, rbx
	call	_external
	mov	rdi, qword ptr [rbx]
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_autorelease

	.p2align	4
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, &mut core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>>, 0):
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdx
	mov	rdi, qword ptr [rdx]
	call	_objc_retain
	mov	rdi, rbx
	call	_external
	mov	rdi, qword ptr [rbx]
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_autorelease

	.p2align	4
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, core[CRATE_ID]::option::Option<&mut core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>>>, 0):
	test	rdx, rdx
	je	LBB2_1
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rdi, qword ptr [rdx]
	mov	rbx, rdx
	call	_objc_retain
	mov	rdi, rbx
	call	_external
	mov	rdi, qword ptr [rbx]
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_autorelease
LBB2_1:
	xor	edi, edi
	jmp	_external

	.p2align	4
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, core[CRATE_ID]::option::Option<&mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>>, 0):
	test	rdx, rdx
	je	LBB3_1
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rdi, qword ptr [rdx]
	mov	rbx, rdx
	call	_objc_retain
	mov	rdi, rbx
	call	_external
	mov	rdi, qword ptr [rbx]
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_autorelease
LBB3_1:
	xor	edi, edi
	jmp	_external

	.p2align	4
SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, &mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>, &mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, 0):
Lfunc_begin0:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	r14, rcx
	mov	rbx, rdx
	mov	rdi, qword ptr [rdx]
	call	_objc_retain
	mov	rdi, qword ptr [r14]
Ltmp0:
	call	_objc_retain
Ltmp1:
	mov	rdi, rbx
	call	_external
	mov	rdi, r14
	call	_external
	mov	rdi, qword ptr [r14]
Ltmp2:
	call	_objc_autorelease
Ltmp3:
	mov	rdi, qword ptr [rbx]
	pop	rbx
	pop	r14
	pop	rbp
	jmp	_objc_autorelease
LBB4_4:
Ltmp4:
	mov	r14, rax
	mov	rdi, qword ptr [rbx]
Ltmp5:
	call	_objc_autorelease
Ltmp6:
	mov	rdi, r14
	call	__Unwind_Resume
LBB4_3:
Ltmp7:
	call	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
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
	.p2align	4
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin1:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rax, qword ptr [rdi]
	movzx	ecx, byte ptr [rax]
	mov	byte ptr [rax], 0
	cmp	cl, 1
	jne	LBB5_10
	mov	rax, qword ptr [rip + _OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rdi, qword ptr [rax]
	lea	rsi, [rip + L_anon.[ID].5]
	xor	edx, edx
	call	_objc_allocateClassPair
	test	rax, rax
	je	LBB5_9
	mov	qword ptr [rbp - 16], rax
	mov	rsi, qword ptr [rip + _OBJC_SELECTOR_REFERENCES_d04fae079395572d]
Ltmp8:
	lea	rdx, [rip + SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, &mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, 0)]
	lea	rcx, [rip + L_anon.[ID].3]
	lea	rdi, [rbp - 16]
	mov	r8d, 6
	call	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_class_method_inner, 0)
Ltmp9:
	mov	rsi, qword ptr [rip + _OBJC_SELECTOR_REFERENCES_48e4a002a77cbcfe]
Ltmp10:
	lea	rdx, [rip + SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, core[CRATE_ID]::option::Option<&mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>>, 0)]
	lea	rcx, [rip + L_anon.[ID].3]
	lea	rdi, [rbp - 16]
	mov	r8d, 6
	call	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_class_method_inner, 0)
Ltmp11:
	mov	rsi, qword ptr [rip + _OBJC_SELECTOR_REFERENCES_06f3315fc4e8da1a]
Ltmp12:
	lea	rdx, [rip + SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, &mut core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>>, 0)]
	lea	rcx, [rip + L_anon.[ID].3]
	lea	rdi, [rbp - 16]
	mov	r8d, 6
	call	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_class_method_inner, 0)
Ltmp13:
	mov	rsi, qword ptr [rip + _OBJC_SELECTOR_REFERENCES_77a278f49aa73e0a]
Ltmp14:
	lea	rdx, [rip + SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, core[CRATE_ID]::option::Option<&mut core[CRATE_ID]::option::Option<objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>>>, 0)]
	lea	rcx, [rip + L_anon.[ID].3]
	lea	rdi, [rbp - 16]
	mov	r8d, 6
	call	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_class_method_inner, 0)
Ltmp15:
	mov	rsi, qword ptr [rip + _OBJC_SELECTOR_REFERENCES_9c279b7fa093536a]
Ltmp16:
	lea	rdx, [rip + SYM(objc2[CRATE_ID]::__macros::define_class::thunk::_::thunk::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::{closure#0}::__FnMarker, &objc2[CRATE_ID]::runtime::anyclass::AnyClass, (), objc2[CRATE_ID]::__macros::method_family::MethodFamily<6u8>, test_define_class_out_parameters[CRATE_ID]::OutParam, &mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>, &mut objc2[CRATE_ID]::rc::retained::Retained<objc2[CRATE_ID]::runtime::nsobject::NSObject>>, 0)]
	lea	rcx, [rip + L_anon.[ID].4]
	lea	rdi, [rbp - 16]
	mov	r8d, 8
	call	SYM(<objc2[CRATE_ID]::runtime::define::ClassBuilder>::add_class_method_inner, 0)
Ltmp17:
	mov	rbx, qword ptr [rbp - 16]
	mov	rdi, rbx
	call	_objc_registerClassPair
	mov	qword ptr [rip + ___CLASS_OutParam], rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret
LBB5_10:
	lea	rdi, [rip + l_anon.[ID].2]
	call	SYM(core[CRATE_ID]::option::unwrap_failed, 0)
LBB5_9:
	lea	rdi, [rip + L_anon.[ID].5]
	lea	rdx, [rip + l_anon.[ID].7]
	mov	esi, 9
	call	SYM(objc2[CRATE_ID]::__macros::define_class::checks::class_not_unique, 0)
LBB5_8:
Ltmp18:
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 16]
	call	_objc_disposeClassPair
	mov	rdi, rbx
	call	__Unwind_Resume
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
	.p2align	4
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rdi]
	mov	qword ptr [rbp - 8], rax
	lea	rdi, [rbp - 8]
	call	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	add	rsp, 16
	pop	rbp
	ret

	.globl	SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::nonnull_null, 0)
	.p2align	4
SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::nonnull_null, 0):
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	_external

	.globl	SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::null_nonnull, 0)
	.p2align	4
SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::null_nonnull, 0):
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	_external

	.globl	SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::nonnull_nonnull, 0)
	.p2align	4
SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::nonnull_nonnull, 0):
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	_external

	.globl	SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::two_nonnull_nonnull, 0)
	.p2align	4
SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::two_nonnull_nonnull, 0):
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rsi
	call	_external
	mov	rdi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_external

	.globl	SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::null_null, 0)
	.p2align	4
SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam>::null_null, 0):
	push	rbp
	mov	rbp, rsp
	pop	rbp
	jmp	_external

	.globl	SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	4
SYM(<test_define_class_out_parameters[CRATE_ID]::OutParam as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
	mov	rax, qword ptr [rip + ___REGISTER_CLASS_OutParam]
	test	rax, rax
	jne	LBB12_1
	mov	rax, qword ptr [rip + ___CLASS_OutParam]
	ret
LBB12_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + ___REGISTER_CLASS_OutParam]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].7]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(<std[CRATE_ID]::sys::sync::once::queue::Once>::call, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + ___CLASS_OutParam]
	ret

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
L_anon.[ID].1:
	.asciz	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].2:
	.quad	L_anon.[ID].1
	.asciz	"p\000\000\000\000\000\000\000\246\000\000\0002\000\000"

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].3:
	.asciz	"v@:^@"

L_anon.[ID].4:
	.asciz	"v@:^@^@"

L_anon.[ID].5:
	.asciz	"OutParam"

L_anon.[ID].6:
	.asciz	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].7:
	.quad	L_anon.[ID].6
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
