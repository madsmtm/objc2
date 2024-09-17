	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.p2align	4, 0x90
SYM(objc2[CRATE_ID]::__macro_helpers::declared_ivars::dealloc::<test_declare_class[CRATE_ID]::DropIvars>, 0):
Lfunc_begin0:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	sub	rsp, 24
	mov	rbx, rsi
	mov	r14, rdi
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)]
	movzx	eax, byte ptr [rdi + rax]
	test	eax, eax
	je	LBB0_6
	cmp	eax, 255
	jne	LBB0_3
	call	SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0)
LBB0_3:
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)]
	mov	rdi, qword ptr [r14 + rax]
	mov	r15, qword ptr [r14 + rax + 8]
Ltmp0:
	call	_objc_release
Ltmp1:
	test	r15, r15
	je	LBB0_6
	mov	rdi, r15
	call	_objc_release
LBB0_6:
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
	.p2align	4, 0x90
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin1:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rax, qword ptr [rdi]
	cmp	byte ptr [rax], 0
	mov	byte ptr [rax], 0
	je	LBB1_14
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rdx, qword ptr [rax]
	lea	rdi, [rip + l_anon.[ID].13]
	lea	rcx, [rip + l_anon.[ID].15]
	mov	esi, 7
	call	SYM(objc2::__macro_helpers::declare_class::create_builder::GENERATED_ID, 0)
	mov	qword ptr [rbp - 16], rax
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_518803e84ea38a73]
Ltmp6:
	lea	r8, [rip + l_anon.[ID].16]
	lea	r9, [rip + _get_class]
	lea	rdi, [rbp - 16]
	mov	edx, 8
	xor	ecx, ecx
	call	SYM(objc2::runtime::declare::ClassBuilder::add_class_method_inner::GENERATED_ID, 0)
Ltmp7:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_05fa1b2ffc15d267]
Ltmp8:
	lea	r8, [rip + l_anon.[ID].3]
	lea	r9, [rip + _method_simple]
	lea	rdi, [rbp - 16]
	mov	edx, 8
	xor	ecx, ecx
	call	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp9:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_58736195c9ca7c7f]
Ltmp10:
	lea	rdx, [rip + l_anon.[ID].17]
	lea	r9, [rip + _method_bool]
	lea	rdi, [rbp - 16]
	mov	ecx, 1
	mov	r8, rdx
	call	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp11:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_61b74dbf9c375668]
Ltmp12:
	lea	r8, [rip + l_anon.[ID].18]
	lea	r9, [rip + _method_id]
	lea	rdi, [rbp - 16]
	mov	edx, 8
	xor	ecx, ecx
	call	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp13:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_96586542870e42e5]
Ltmp14:
	lea	rdx, [rip + l_anon.[ID].17]
	lea	r8, [rip + l_anon.[ID].18]
	lea	r9, [rip + _method_id_with_param]
	lea	rdi, [rbp - 16]
	mov	ecx, 1
	call	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp15:
Ltmp16:
	lea	rdi, [rip + L_anon.[ID].19]
	mov	esi, 8
	call	SYM(objc2::top_level_traits::get_protocol::GENERATED_ID, 0)
Ltmp17:
	test	rax, rax
	je	LBB1_9
	mov	rdi, qword ptr [rbp - 16]
	mov	rsi, rax
	call	_class_addProtocol
LBB1_9:
Ltmp18:
	lea	rdi, [rip + l_anon.[ID].20]
	mov	esi, 9
	call	SYM(objc2::top_level_traits::get_protocol::GENERATED_ID, 0)
Ltmp19:
	test	rax, rax
	je	LBB1_12
	mov	rdi, qword ptr [rbp - 16]
	mov	rsi, rax
	call	_class_addProtocol
LBB1_12:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_f4e71677dafa88a8]
Ltmp20:
	lea	rdx, [rip + l_anon.[ID].23]
	lea	r8, [rip + l_anon.[ID].18]
	lea	r9, [rip + _copyWithZone]
	lea	rdi, [rbp - 16]
	mov	ecx, 1
	call	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp21:
	mov	rbx, qword ptr [rbp - 16]
	mov	rdi, rbx
	call	_objc_registerClassPair
	mov	qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 0).0], rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret
LBB1_14:
	lea	rdi, [rip + l_anon.[ID].12]
	call	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB1_15:
Ltmp22:
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 16]
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
	.uleb128 Lfunc_begin1-Lfunc_begin1
	.uleb128 Ltmp6-Lfunc_begin1
	.byte	0
	.byte	0
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
	.p2align	4, 0x90
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin2:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	sub	rsp, 64
	mov	rax, qword ptr [rdi]
	cmp	byte ptr [rax], 0
	mov	byte ptr [rax], 0
	je	LBB2_14
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rdx, qword ptr [rax]
	lea	rdi, [rip + l_anon.[ID].10]
	lea	rcx, [rip + l_anon.[ID].25]
	mov	esi, 9
	call	SYM(objc2::__macro_helpers::declare_class::create_builder::GENERATED_ID, 0)
	mov	qword ptr [rbp - 72], rax
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPCREL]
	mov	rsi, qword ptr [rax]
Ltmp23:
	lea	r8, [rip + l_anon.[ID].3]
	lea	r9, [rip + SYM(objc2[CRATE_ID]::__macro_helpers::declared_ivars::dealloc::<test_declare_class[CRATE_ID]::DropIvars>, 0)]
	lea	rbx, [rbp - 72]
	mov	edx, 8
	mov	rdi, rbx
	xor	ecx, ecx
	call	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp24:
	mov	rax, qword ptr [rbp - 72]
	mov	qword ptr [rbp - 32], rax
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_init@GOTPCREL]
	mov	rsi, qword ptr [rax]
Ltmp26:
	lea	r8, [rip + l_anon.[ID].18]
	lea	r9, [rip + _init_drop_ivars]
	lea	rbx, [rbp - 32]
	mov	edx, 8
	mov	rdi, rbx
	xor	ecx, ecx
	call	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp27:
	mov	rax, qword ptr [rbp - 32]
	mov	qword ptr [rbp - 24], rax
	mov	qword ptr [rbp - 64], 16
	lea	rax, [rip + l_anon.[ID].8]
	mov	qword ptr [rbp - 56], rax
	mov	byte ptr [rbp - 72], 27
Ltmp29:
	lea	rsi, [rip + l_anon.[ID].4]
	lea	rbx, [rbp - 24]
	lea	r9, [rbp - 72]
	mov	edx, 6
	mov	ecx, 16
	mov	rdi, rbx
	mov	r8d, 3
	call	SYM(objc2::runtime::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Ltmp30:
Ltmp31:
	lea	rsi, [rip + l_anon.[ID].5]
	lea	r9, [rip + l_anon.[ID].6]
	lea	rdi, [rbp - 24]
	mov	edx, 10
	mov	ecx, 1
	xor	r8d, r8d
	call	SYM(objc2::runtime::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Ltmp32:
	mov	rbx, qword ptr [rbp - 24]
	mov	rdi, rbx
	call	_objc_registerClassPair
	lea	rsi, [rip + l_anon.[ID].4]
	mov	rdi, rbx
	call	_class_getInstanceVariable
	test	rax, rax
	je	LBB2_6
	mov	rdi, rax
	call	_ivar_getOffset
	mov	r14, rax
	lea	rsi, [rip + l_anon.[ID].5]
	mov	rdi, rbx
	call	_class_getInstanceVariable
	test	rax, rax
	je	LBB2_10
	mov	rdi, rax
	call	_ivar_getOffset
	mov	qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 1).0], rbx
	mov	qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)], r14
	mov	qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)], rax
	add	rsp, 64
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB2_14:
	lea	rdi, [rip + l_anon.[ID].12]
	call	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB2_6:
	call	SYM(objc2::__macro_helpers::declared_ivars::register_with_ivars::get_ivar_failed::GENERATED_ID, 0)
LBB2_10:
	call	SYM(objc2::__macro_helpers::declared_ivars::register_with_ivars::get_drop_flag_failed::GENERATED_ID, 0)
LBB2_12:
Ltmp28:
	jmp	LBB2_8
LBB2_7:
Ltmp25:
	jmp	LBB2_8
LBB2_11:
Ltmp33:
LBB2_8:
	mov	r14, rax
	mov	rdi, qword ptr [rbx]
	call	_objc_disposeClassPair
	mov	rdi, r14
	call	__Unwind_Resume
Lfunc_end2:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table2:
Lexception2:
	.byte	255
	.byte	255
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
	.uleb128 Ltmp26-Lfunc_begin2
	.uleb128 Ltmp27-Ltmp26
	.uleb128 Ltmp28-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp29-Lfunc_begin2
	.uleb128 Ltmp32-Ltmp29
	.uleb128 Ltmp33-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp32-Lfunc_begin2
	.uleb128 Lfunc_end2-Ltmp32
	.byte	0
	.byte	0
Lcst_end2:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	4, 0x90
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin3:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	sub	rsp, 64
	mov	rax, qword ptr [rdi]
	cmp	byte ptr [rax], 0
	mov	byte ptr [rax], 0
	je	LBB3_9
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rdx, qword ptr [rax]
	lea	rdi, [rip + l_anon.[ID].9]
	lea	rcx, [rip + l_anon.[ID].24]
	mov	esi, 15
	call	SYM(objc2::__macro_helpers::declare_class::create_builder::GENERATED_ID, 0)
	mov	qword ptr [rbp - 24], rax
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_init@GOTPCREL]
	mov	rsi, qword ptr [rax]
Ltmp34:
	lea	r8, [rip + l_anon.[ID].18]
	lea	r9, [rip + _init_forgetable_ivars]
	lea	rbx, [rbp - 24]
	mov	edx, 8
	mov	rdi, rbx
	xor	ecx, ecx
	call	SYM(objc2::runtime::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp35:
	mov	rax, qword ptr [rbp - 24]
	mov	qword ptr [rbp - 32], rax
	mov	qword ptr [rbp - 64], 8
	lea	rax, [rip + l_anon.[ID].7]
	mov	qword ptr [rbp - 56], rax
	mov	byte ptr [rbp - 72], 27
Ltmp37:
	lea	rsi, [rip + l_anon.[ID].4]
	lea	rbx, [rbp - 32]
	lea	r9, [rbp - 72]
	mov	edx, 6
	mov	ecx, 8
	mov	rdi, rbx
	mov	r8d, 2
	call	SYM(objc2::runtime::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Ltmp38:
	mov	rbx, qword ptr [rbp - 32]
	mov	rdi, rbx
	call	_objc_registerClassPair
	lea	rsi, [rip + l_anon.[ID].4]
	mov	rdi, rbx
	call	_class_getInstanceVariable
	test	rax, rax
	je	LBB3_4
	mov	rdi, rax
	call	_ivar_getOffset
	mov	qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 2).0], rbx
	mov	qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)], rax
	add	rsp, 64
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB3_9:
	lea	rdi, [rip + l_anon.[ID].12]
	call	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB3_4:
	call	SYM(objc2::__macro_helpers::declared_ivars::register_with_ivars::get_ivar_failed::GENERATED_ID, 0)
LBB3_5:
Ltmp39:
	jmp	LBB3_6
LBB3_7:
Ltmp36:
LBB3_6:
	mov	r14, rax
	mov	rdi, qword ptr [rbx]
	call	_objc_disposeClassPair
	mov	rdi, r14
	call	__Unwind_Resume
Lfunc_end3:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table3:
Lexception3:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end3-Lcst_begin3
Lcst_begin3:
	.uleb128 Lfunc_begin3-Lfunc_begin3
	.uleb128 Ltmp34-Lfunc_begin3
	.byte	0
	.byte	0
	.uleb128 Ltmp34-Lfunc_begin3
	.uleb128 Ltmp35-Ltmp34
	.uleb128 Ltmp36-Lfunc_begin3
	.byte	0
	.uleb128 Ltmp37-Lfunc_begin3
	.uleb128 Ltmp38-Ltmp37
	.uleb128 Ltmp39-Lfunc_begin3
	.byte	0
	.uleb128 Ltmp38-Lfunc_begin3
	.uleb128 Lfunc_end3-Ltmp38
	.byte	0
	.byte	0
Lcst_end3:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	4, 0x90
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rdi]
	mov	qword ptr [rbp - 8], rax
	lea	rdi, [rbp - 8]
	call	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	add	rsp, 16
	pop	rbp
	ret

	.p2align	4, 0x90
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rdi]
	mov	qword ptr [rbp - 8], rax
	lea	rdi, [rbp - 8]
	call	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	add	rsp, 16
	pop	rbp
	ret

	.p2align	4, 0x90
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rdi]
	mov	qword ptr [rbp - 8], rax
	lea	rdi, [rbp - 8]
	call	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	add	rsp, 16
	pop	rbp
	ret

	.globl	_access_forgetable_ivars_class
	.p2align	4, 0x90
_access_forgetable_ivars_class:
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB7_1
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 2).0]
	ret
LBB7_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].2]
	lea	r8, [rip + l_anon.[ID].24]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 2).0]
	ret

	.globl	_access_forgetable_ivars
	.p2align	4, 0x90
_access_forgetable_ivars:
	push	rbp
	mov	rbp, rsp
	mov	rcx, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)]
	movzx	eax, byte ptr [rdi + rcx + 4]
	mov	edx, dword ptr [rdi + rcx]
	pop	rbp
	ret

	.globl	SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0)
	.p2align	4, 0x90
SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0):
	push	rbp
	mov	rbp, rsp
	## InlineAsm Start
	## InlineAsm End
	pop	rbp
	ret

	.globl	_access_drop_ivars_class
	.p2align	4, 0x90
_access_drop_ivars_class:
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB10_1
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 1).0]
	ret
LBB10_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].1]
	lea	r8, [rip + l_anon.[ID].25]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 1).0]
	ret

	.globl	_access_drop_ivars
	.p2align	4, 0x90
_access_drop_ivars:
	push	rbp
	mov	rbp, rsp
	mov	rcx, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)]
	mov	rax, qword ptr [rdi + rcx]
	mov	rdx, qword ptr [rdi + rcx + 8]
	pop	rbp
	ret

	.globl	SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	4, 0x90
SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB12_1
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 0).0]
	ret
LBB12_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].15]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 0).0]
	ret

	.globl	_get_class
	.p2align	4, 0x90
_get_class:
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB13_1
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 0).0]
	ret
LBB13_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].15]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 0).0]
	ret

	.globl	_method_simple
	.p2align	4, 0x90
_method_simple:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.globl	_method_bool
	.p2align	4, 0x90
_method_bool:
	push	rbp
	mov	rbp, rsp
	xor	eax, eax
	test	dl, dl
	sete	al
	pop	rbp
	ret

	.globl	_method_id
	.p2align	4, 0x90
_method_id:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB16_1
LBB16_2:
	mov	rdi, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 0).0]
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_new@GOTPCREL]
	mov	rsi, qword ptr [rax]
	call	_objc_msgSend
	mov	rdi, rax
	call	_objc_autoreleaseReturnValue
	add	rsp, 16
	pop	rbp
	ret
LBB16_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].15]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	jmp	LBB16_2

	.globl	_method_id_with_param
	.p2align	4, 0x90
_method_id_with_param:
Lfunc_begin4:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	r14d, edx
	call	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	rbx, rax
	test	r14b, r14b
	je	LBB17_3
Ltmp40:
	call	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
Ltmp41:
	mov	r14, rax
	mov	rdi, rbx
	call	_objc_release
	mov	rbx, r14
LBB17_3:
	mov	rdi, rbx
	pop	rbx
	pop	r14
	pop	rbp
	jmp	_objc_autoreleaseReturnValue
LBB17_5:
Ltmp42:
	mov	r14, rax
Ltmp43:
	mov	rdi, rbx
	call	_objc_release
Ltmp44:
	mov	rdi, r14
	call	__Unwind_Resume
LBB17_4:
Ltmp45:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end4:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table17:
Lexception4:
	.byte	255
	.byte	155
	.uleb128 Lttbase1-Lttbaseref1
Lttbaseref1:
	.byte	1
	.uleb128 Lcst_end4-Lcst_begin4
Lcst_begin4:
	.uleb128 Lfunc_begin4-Lfunc_begin4
	.uleb128 Ltmp40-Lfunc_begin4
	.byte	0
	.byte	0
	.uleb128 Ltmp40-Lfunc_begin4
	.uleb128 Ltmp41-Ltmp40
	.uleb128 Ltmp42-Lfunc_begin4
	.byte	0
	.uleb128 Ltmp41-Lfunc_begin4
	.uleb128 Ltmp43-Ltmp41
	.byte	0
	.byte	0
	.uleb128 Ltmp43-Lfunc_begin4
	.uleb128 Ltmp44-Ltmp43
	.uleb128 Ltmp45-Lfunc_begin4
	.byte	1
	.uleb128 Ltmp44-Lfunc_begin4
	.uleb128 Lfunc_end4-Ltmp44
	.byte	0
	.byte	0
Lcst_end4:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase1:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_copyWithZone
	.p2align	4, 0x90
_copyWithZone:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB18_1
LBB18_2:
	mov	rdi, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 0).0]
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_new@GOTPCREL]
	mov	rsi, qword ptr [rax]
	call	_objc_msgSend
	add	rsp, 16
	pop	rbp
	ret
LBB18_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].15]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	jmp	LBB18_2

	.globl	SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	4, 0x90
SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB19_1
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 2).0]
	ret
LBB19_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].2]
	lea	r8, [rip + l_anon.[ID].24]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 2).0]
	ret

	.globl	_init_forgetable_ivars
	.p2align	4, 0x90
_init_forgetable_ivars:
	test	rdi, rdi
	je	LBB20_2
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)]
	mov	dword ptr [rdi + rax], 43
	mov	byte ptr [rdi + rax + 4], 42
LBB20_2:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_7f51a873b0d59f00]
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rax, qword ptr [rax]
	mov	qword ptr [rbp - 16], rdi
	mov	qword ptr [rbp - 8], rax
	lea	rdi, [rbp - 16]
	call	_objc_msgSendSuper
	add	rsp, 16
	pop	rbp
	ret

	.globl	SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	4, 0x90
SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB21_1
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 1).0]
	ret
LBB21_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].1]
	lea	r8, [rip + l_anon.[ID].25]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 1).0]
	ret

	.globl	_init_drop_ivars
	.p2align	4, 0x90
_init_drop_ivars:
Lfunc_begin5:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r12
	push	rbx
	sub	rsp, 16
	mov	rbx, rdi
Ltmp46:
	call	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
Ltmp47:
Ltmp49:
	mov	r15, rax
	call	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
Ltmp50:
	mov	r12, rax
	test	rbx, rbx
	je	LBB22_3
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)]
	mov	qword ptr [rbx + rax], r15
	mov	qword ptr [rbx + rax + 8], r12
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)]
	mov	byte ptr [rbx + rax], 15
	jmp	LBB22_9
LBB22_3:
Ltmp57:
	mov	rdi, r15
	call	_objc_release
Ltmp58:
	mov	rdi, r12
	call	_objc_release
LBB22_9:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_802cb9c5fa0b19dd]
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rax, qword ptr [rax]
	mov	qword ptr [rbp - 48], rbx
	mov	qword ptr [rbp - 40], rax
	lea	rdi, [rbp - 48]
	call	_objc_msgSendSuper
	test	rax, rax
	je	LBB22_11
	mov	rcx, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)]
	mov	byte ptr [rax + rcx], -1
LBB22_11:
	add	rsp, 16
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB22_6:
Ltmp59:
	mov	r14, rax
Ltmp60:
	mov	rdi, r12
	call	_objc_release
Ltmp61:
	jmp	LBB22_15
LBB22_7:
Ltmp62:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB22_5:
Ltmp51:
	mov	r14, rax
Ltmp52:
	mov	rdi, r15
	call	_objc_release
Ltmp53:
	jmp	LBB22_14
LBB22_13:
Ltmp48:
	mov	r14, rax
LBB22_14:
Ltmp54:
	mov	rdi, rbx
	call	_objc_release
Ltmp55:
LBB22_15:
	mov	rdi, r14
	call	__Unwind_Resume
LBB22_12:
Ltmp56:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end5:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table22:
Lexception5:
	.byte	255
	.byte	155
	.uleb128 Lttbase2-Lttbaseref2
Lttbaseref2:
	.byte	1
	.uleb128 Lcst_end5-Lcst_begin5
Lcst_begin5:
	.uleb128 Ltmp46-Lfunc_begin5
	.uleb128 Ltmp47-Ltmp46
	.uleb128 Ltmp48-Lfunc_begin5
	.byte	0
	.uleb128 Ltmp49-Lfunc_begin5
	.uleb128 Ltmp50-Ltmp49
	.uleb128 Ltmp51-Lfunc_begin5
	.byte	0
	.uleb128 Ltmp57-Lfunc_begin5
	.uleb128 Ltmp58-Ltmp57
	.uleb128 Ltmp59-Lfunc_begin5
	.byte	0
	.uleb128 Ltmp58-Lfunc_begin5
	.uleb128 Ltmp60-Ltmp58
	.byte	0
	.byte	0
	.uleb128 Ltmp60-Lfunc_begin5
	.uleb128 Ltmp61-Ltmp60
	.uleb128 Ltmp62-Lfunc_begin5
	.byte	1
	.uleb128 Ltmp52-Lfunc_begin5
	.uleb128 Ltmp55-Ltmp52
	.uleb128 Ltmp56-Lfunc_begin5
	.byte	1
	.uleb128 Ltmp55-Lfunc_begin5
	.uleb128 Lfunc_end5-Ltmp55
	.byte	0
	.byte	0
Lcst_end5:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase2:
	.byte	0
	.p2align	2, 0x0

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.p2align	3, 0x0
l_anon.[ID].1:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.p2align	3, 0x0
l_anon.[ID].2:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].3:
	.byte	17
	.space	39

l_anon.[ID].4:
	.asciz	"ivars"

l_anon.[ID].5:
	.asciz	"drop_flag"

	.p2align	3, 0x0
l_anon.[ID].6:
	.byte	5
	.space	39

	.p2align	3, 0x0
l_anon.[ID].7:
	.byte	7
	.space	39

	.p2align	3, 0x0
l_anon.[ID].8:
	.byte	9
	.space	39

l_anon.[ID].9:
	.ascii	"ForgetableIvars"

l_anon.[ID].10:
	.ascii	"DropIvars"

l_anon.[ID].11:
	.ascii	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].12:
	.quad	l_anon.[ID].11
	.asciz	"p\000\000\000\000\000\000\000\236\000\000\0002\000\000"

	.section	__TEXT,__const
l_anon.[ID].13:
	.ascii	"NoIvars"

.zerofill __DATA,__bss,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 0).0,8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2),8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 1)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 1),8,3
l_anon.[ID].14:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].15:
	.quad	l_anon.[ID].14
	.asciz	"5\000\000\000\000\000\000\000\016\000\000\000\001\000\000"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0),8,3
	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].16:
	.byte	21
	.space	39

	.p2align	3, 0x0
l_anon.[ID].17:
	.space	1
	.space	39

	.p2align	3, 0x0
l_anon.[ID].18:
	.byte	19
	.space	39

	.section	__TEXT,__literal8,8byte_literals
L_anon.[ID].19:
	.ascii	"NSObject"

	.section	__TEXT,__const
l_anon.[ID].20:
	.ascii	"NSCopying"

l_anon.[ID].21:
	.ascii	"_NSZone"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].22:
	.byte	28
	.space	7
	.quad	l_anon.[ID].21
	.asciz	"\007\000\000\000\000\000\000"
	.quad	8
	.space	8

	.p2align	3, 0x0
l_anon.[ID].23:
	.byte	25
	.space	7
	.quad	l_anon.[ID].22
	.space	24

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_518803e84ea38a73
L_OBJC_METH_VAR_NAME_518803e84ea38a73:
	.asciz	"classMethod"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_518803e84ea38a73
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_518803e84ea38a73:
	.quad	L_OBJC_METH_VAR_NAME_518803e84ea38a73

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_518803e84ea38a73
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_518803e84ea38a73:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_05fa1b2ffc15d267
L_OBJC_METH_VAR_NAME_05fa1b2ffc15d267:
	.asciz	"method"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_05fa1b2ffc15d267
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_05fa1b2ffc15d267:
	.quad	L_OBJC_METH_VAR_NAME_05fa1b2ffc15d267

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_05fa1b2ffc15d267
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_05fa1b2ffc15d267:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_58736195c9ca7c7f
L_OBJC_METH_VAR_NAME_58736195c9ca7c7f:
	.asciz	"methodBool:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_58736195c9ca7c7f
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_58736195c9ca7c7f:
	.quad	L_OBJC_METH_VAR_NAME_58736195c9ca7c7f

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_58736195c9ca7c7f
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_58736195c9ca7c7f:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_61b74dbf9c375668
L_OBJC_METH_VAR_NAME_61b74dbf9c375668:
	.asciz	"methodId"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_61b74dbf9c375668
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_61b74dbf9c375668:
	.quad	L_OBJC_METH_VAR_NAME_61b74dbf9c375668

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_61b74dbf9c375668
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_61b74dbf9c375668:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_96586542870e42e5
L_OBJC_METH_VAR_NAME_96586542870e42e5:
	.asciz	"methodIdWithParam:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_96586542870e42e5
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_96586542870e42e5:
	.quad	L_OBJC_METH_VAR_NAME_96586542870e42e5

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_96586542870e42e5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_96586542870e42e5:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_f4e71677dafa88a8
L_OBJC_METH_VAR_NAME_f4e71677dafa88a8:
	.asciz	"copyWithZone:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_f4e71677dafa88a8
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_f4e71677dafa88a8:
	.quad	L_OBJC_METH_VAR_NAME_f4e71677dafa88a8

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f4e71677dafa88a8
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f4e71677dafa88a8:
	.asciz	"\000\000\000\000@\000\000"

.zerofill __DATA,__bss,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 2).0,8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1),8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 2)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 2),8,3
	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].24:
	.quad	l_anon.[ID].14
	.asciz	"5\000\000\000\000\000\000\000R\000\000\000\001\000\000"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0),8,3
	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_7f51a873b0d59f00
L_OBJC_METH_VAR_NAME_7f51a873b0d59f00:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_7f51a873b0d59f00
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_7f51a873b0d59f00:
	.quad	L_OBJC_METH_VAR_NAME_7f51a873b0d59f00

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_7f51a873b0d59f00
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_7f51a873b0d59f00:
	.asciz	"\000\000\000\000@\000\000"

.zerofill __DATA,__bss,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 1).0,8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0),8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0),8,3
	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].25:
	.quad	l_anon.[ID].14
	.asciz	"5\000\000\000\000\000\000\000z\000\000\000\001\000\000"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0),8,3
	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_802cb9c5fa0b19dd
L_OBJC_METH_VAR_NAME_802cb9c5fa0b19dd:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_802cb9c5fa0b19dd
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_802cb9c5fa0b19dd:
	.quad	L_OBJC_METH_VAR_NAME_802cb9c5fa0b19dd

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_802cb9c5fa0b19dd
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_802cb9c5fa0b19dd:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
