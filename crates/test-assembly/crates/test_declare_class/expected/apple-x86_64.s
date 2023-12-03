	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.p2align	4, 0x90
SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}>, 0):
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.p2align	4, 0x90
SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}>, 0):
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.p2align	4, 0x90
SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}>, 0):
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.p2align	4, 0x90
SYM(objc2[CRATE_ID]::__macro_helpers::declared_ivars::dealloc::<test_declare_class[CRATE_ID]::DropIvars>, 0):
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
	je	LBB3_5
	cmp	eax, 255
	jne	LBB3_3
	call	SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0)
LBB3_3:
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)]
	mov	rdi, qword ptr [r14 + rax]
	mov	r15, qword ptr [r14 + rax + 8]
	call	_objc_release
	test	r15, r15
	je	LBB3_5
	mov	rdi, r15
	call	_objc_release
LBB3_5:
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

	.p2align	4, 0x90
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r12
	push	rbx
	sub	rsp, 16
	mov	rax, qword ptr [rdi]
	cmp	byte ptr [rax], 0
	mov	byte ptr [rax], 0
	je	LBB4_7
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rdx, qword ptr [rax]
	lea	rdi, [rip + l_anon.[ID].17]
	mov	esi, 7
	call	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
	test	rax, rax
	je	LBB4_8
	mov	qword ptr [rbp - 40], rax
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_03fd85b0462f54e9]
	lea	r14, [rip + l_anon.[ID].3]
	lea	r8, [rip + l_anon.[ID].10]
	lea	r9, [rip + _get_class]
	lea	rbx, [rbp - 40]
	mov	rdi, rbx
	mov	rdx, r14
	xor	ecx, ecx
	call	SYM(objc2::declare::ClassBuilder::add_class_method_inner::GENERATED_ID, 0)
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_cf773331f3cfba54]
	lea	r8, [rip + l_anon.[ID].5]
	lea	r9, [rip + _method_simple]
	mov	rdi, rbx
	mov	rdx, r14
	xor	ecx, ecx
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_abdcbb85641cd990]
	lea	r15, [rip + l_anon.[ID].6]
	lea	r9, [rip + _method_bool]
	mov	ecx, 1
	mov	rdi, rbx
	mov	rdx, r15
	mov	r8, r15
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_ef8de92414f2d9c8]
	lea	r12, [rip + l_anon.[ID].4]
	lea	r9, [rip + _method_id]
	mov	rdi, rbx
	mov	rdx, r14
	xor	ecx, ecx
	mov	r8, r12
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_4a611090161f3fae]
	lea	r9, [rip + _method_id_with_param]
	mov	ecx, 1
	mov	rdi, rbx
	mov	rdx, r15
	mov	r8, r12
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	lea	rdi, [rip + L_anon.[ID].24]
	mov	esi, 8
	call	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	test	rax, rax
	je	LBB4_4
	lea	rdi, [rbp - 40]
	mov	rsi, rax
	call	SYM(objc2::declare::ClassBuilder::add_protocol::GENERATED_ID, 0)
LBB4_4:
	lea	rdi, [rip + l_anon.[ID].25]
	mov	esi, 9
	call	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	test	rax, rax
	je	LBB4_6
	lea	rdi, [rbp - 40]
	mov	rsi, rax
	call	SYM(objc2::declare::ClassBuilder::add_protocol::GENERATED_ID, 0)
LBB4_6:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_2837f061c311eb14]
	lea	rdx, [rip + l_anon.[ID].9]
	lea	r8, [rip + l_anon.[ID].4]
	lea	r9, [rip + _copyWithZone]
	lea	rdi, [rbp - 40]
	mov	ecx, 1
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rdi, qword ptr [rbp - 40]
	call	SYM(objc2::declare::ClassBuilder::register::GENERATED_ID, 0)
	mov	qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 0).0], rax
	add	rsp, 16
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB4_7:
	lea	rdi, [rip + l_anon.[ID].19]
	lea	rdx, [rip + l_anon.[ID].21]
	mov	esi, 43
	call	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB4_8:
	lea	rdi, [rip + l_anon.[ID].17]
	lea	rdx, [rip + l_anon.[ID].23]
	mov	esi, 7
	call	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)

	.p2align	4, 0x90
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	sub	rsp, 64
	mov	rax, qword ptr [rdi]
	cmp	byte ptr [rax], 0
	mov	byte ptr [rax], 0
	je	LBB5_5
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rdx, qword ptr [rax]
	lea	rdi, [rip + l_anon.[ID].18]
	mov	esi, 9
	call	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
	test	rax, rax
	je	LBB5_6
	mov	qword ptr [rbp - 72], rax
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPCREL]
	mov	rsi, qword ptr [rax]
	lea	rbx, [rip + l_anon.[ID].3]
	lea	r8, [rip + l_anon.[ID].5]
	lea	r9, [rip + SYM(objc2[CRATE_ID]::__macro_helpers::declared_ivars::dealloc::<test_declare_class[CRATE_ID]::DropIvars>, 0)]
	lea	rdi, [rbp - 72]
	mov	rdx, rbx
	xor	ecx, ecx
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rax, qword ptr [rbp - 72]
	mov	qword ptr [rbp - 24], rax
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_init@GOTPCREL]
	mov	rsi, qword ptr [rax]
	lea	r8, [rip + l_anon.[ID].4]
	lea	r9, [rip + _init_drop_ivars]
	lea	rdi, [rbp - 24]
	mov	rdx, rbx
	xor	ecx, ecx
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rax, qword ptr [rbp - 24]
	mov	qword ptr [rbp - 32], rax
	mov	qword ptr [rbp - 64], 16
	lea	rax, [rip + l_anon.[ID].15]
	mov	qword ptr [rbp - 56], rax
	mov	byte ptr [rbp - 72], 27
	lea	r14, [rip + l_anon.[ID].11]
	lea	rbx, [rbp - 32]
	lea	r9, [rbp - 72]
	mov	edx, 5
	mov	ecx, 16
	mov	rdi, rbx
	mov	rsi, r14
	mov	r8d, 3
	call	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	lea	rsi, [rip + l_anon.[ID].12]
	lea	r9, [rip + l_anon.[ID].13]
	mov	edx, 9
	mov	ecx, 1
	mov	rdi, rbx
	xor	r8d, r8d
	call	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	mov	rdi, qword ptr [rbp - 32]
	call	SYM(objc2::declare::ClassBuilder::register::GENERATED_ID, 0)
	mov	rbx, rax
	mov	edx, 5
	mov	rdi, rax
	mov	rsi, r14
	call	SYM(objc2::runtime::AnyClass::instance_variable::GENERATED_ID, 0)
	test	rax, rax
	je	LBB5_7
	mov	rdi, rax
	call	_ivar_getOffset
	mov	r14, rax
	lea	rsi, [rip + l_anon.[ID].12]
	mov	edx, 9
	mov	rdi, rbx
	call	SYM(objc2::runtime::AnyClass::instance_variable::GENERATED_ID, 0)
	test	rax, rax
	je	LBB5_8
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
LBB5_5:
	lea	rdi, [rip + l_anon.[ID].19]
	lea	rdx, [rip + l_anon.[ID].21]
	mov	esi, 43
	call	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB5_6:
	lea	rdi, [rip + l_anon.[ID].18]
	lea	rdx, [rip + l_anon.[ID].27]
	mov	esi, 9
	call	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
LBB5_7:
	call	SYM(objc2::__macro_helpers::declared_ivars::register_with_ivars::get_ivar_failed::GENERATED_ID, 0)
LBB5_8:
	call	SYM(objc2::__macro_helpers::declared_ivars::register_with_ivars::get_drop_flag_failed::GENERATED_ID, 0)

	.p2align	4, 0x90
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	sub	rsp, 64
	mov	rax, qword ptr [rdi]
	cmp	byte ptr [rax], 0
	mov	byte ptr [rax], 0
	je	LBB6_4
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rdx, qword ptr [rax]
	lea	rdi, [rip + l_anon.[ID].16]
	mov	esi, 15
	call	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
	test	rax, rax
	je	LBB6_5
	mov	qword ptr [rbp - 24], rax
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_init@GOTPCREL]
	mov	rsi, qword ptr [rax]
	lea	rdx, [rip + l_anon.[ID].3]
	lea	r8, [rip + l_anon.[ID].4]
	lea	r9, [rip + _init_forgetable_ivars]
	lea	rdi, [rbp - 24]
	xor	ecx, ecx
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rax, qword ptr [rbp - 24]
	mov	qword ptr [rbp - 32], rax
	mov	qword ptr [rbp - 64], 8
	lea	rax, [rip + l_anon.[ID].14]
	mov	qword ptr [rbp - 56], rax
	mov	byte ptr [rbp - 72], 27
	lea	r14, [rip + l_anon.[ID].11]
	lea	rdi, [rbp - 32]
	lea	r9, [rbp - 72]
	mov	edx, 5
	mov	ecx, 8
	mov	rsi, r14
	mov	r8d, 2
	call	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	mov	rdi, qword ptr [rbp - 32]
	call	SYM(objc2::declare::ClassBuilder::register::GENERATED_ID, 0)
	mov	rbx, rax
	mov	edx, 5
	mov	rdi, rax
	mov	rsi, r14
	call	SYM(objc2::runtime::AnyClass::instance_variable::GENERATED_ID, 0)
	test	rax, rax
	je	LBB6_6
	mov	rdi, rax
	call	_ivar_getOffset
	mov	qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 2).0], rbx
	mov	qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)], rax
	add	rsp, 64
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB6_4:
	lea	rdi, [rip + l_anon.[ID].19]
	lea	rdx, [rip + l_anon.[ID].21]
	mov	esi, 43
	call	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB6_5:
	lea	rdi, [rip + l_anon.[ID].16]
	lea	rdx, [rip + l_anon.[ID].26]
	mov	esi, 15
	call	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)
LBB6_6:
	call	SYM(objc2::__macro_helpers::declared_ivars::register_with_ivars::get_ivar_failed::GENERATED_ID, 0)

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
	jne	LBB10_1
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 2).0]
	ret
LBB10_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].2]
	lea	r8, [rip + l_anon.[ID].26]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
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
	jne	LBB13_1
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 1).0]
	ret
LBB13_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].1]
	lea	r8, [rip + l_anon.[ID].27]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
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
	jne	LBB15_1
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 0).0]
	ret
LBB15_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].23]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 0).0]
	ret

	.globl	_get_class
	.p2align	4, 0x90
_get_class:
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB16_1
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 0).0]
	ret
LBB16_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].23]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
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
	jne	LBB19_1
LBB19_2:
	mov	rdi, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 0).0]
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_new@GOTPCREL]
	mov	rsi, qword ptr [rax]
	call	_objc_msgSend
	mov	rdi, rax
	call	_objc_autoreleaseReturnValue
	add	rsp, 16
	pop	rbp
	ret
LBB19_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].23]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	jmp	LBB19_2

	.globl	_method_id_with_param
	.p2align	4, 0x90
_method_id_with_param:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	r14d, edx
	call	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	rbx, rax
	test	r14b, r14b
	je	LBB20_2
	call	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	r14, rax
	mov	rdi, rbx
	call	_objc_release
	mov	rbx, r14
LBB20_2:
	mov	rdi, rbx
	pop	rbx
	pop	r14
	pop	rbp
	jmp	_objc_autoreleaseReturnValue

	.globl	_copyWithZone
	.p2align	4, 0x90
_copyWithZone:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB21_1
LBB21_2:
	mov	rdi, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 0).0]
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_new@GOTPCREL]
	mov	rsi, qword ptr [rax]
	call	_objc_msgSend
	add	rsp, 16
	pop	rbp
	ret
LBB21_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].23]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	jmp	LBB21_2

	.globl	SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	4, 0x90
SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB22_1
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 2).0]
	ret
LBB22_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].2]
	lea	r8, [rip + l_anon.[ID].26]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 2).0]
	ret

	.globl	_init_forgetable_ivars
	.p2align	4, 0x90
_init_forgetable_ivars:
	test	rdi, rdi
	je	LBB23_2
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)]
	mov	dword ptr [rdi + rax], 43
	mov	byte ptr [rdi + rax + 4], 42
LBB23_2:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_af8966656b8b2b6c]
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
	jne	LBB24_1
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 1).0]
	ret
LBB24_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].1]
	lea	r8, [rip + l_anon.[ID].27]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 1).0]
	ret

	.globl	_init_drop_ivars
	.p2align	4, 0x90
_init_drop_ivars:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	sub	rsp, 24
	mov	rbx, rdi
	call	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	r14, rax
	call	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	r15, rax
	test	rbx, rbx
	je	LBB25_2
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)]
	mov	qword ptr [rbx + rax], r14
	mov	qword ptr [rbx + rax + 8], r15
	mov	rax, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)]
	mov	byte ptr [rbx + rax], 15
	jmp	LBB25_3
LBB25_2:
	mov	rdi, r14
	call	_objc_release
	mov	rdi, r15
	call	_objc_release
LBB25_3:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_6edddcebbded8f32]
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rax, qword ptr [rax]
	mov	qword ptr [rbp - 40], rbx
	mov	qword ptr [rbp - 32], rax
	lea	rdi, [rbp - 40]
	call	_objc_msgSendSuper
	test	rax, rax
	je	LBB25_5
	mov	rcx, qword ptr [rip + SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)]
	mov	byte ptr [rax + rcx], -1
LBB25_5:
	add	rsp, 24
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.quad	SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}>, 0)
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.p2align	3, 0x0
l_anon.[ID].1:
	.quad	SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}>, 0)
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.p2align	3, 0x0
l_anon.[ID].2:
	.quad	SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}>, 0)
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].3:
	.byte	0

	.p2align	3, 0x0
l_anon.[ID].4:
	.byte	19
	.space	39

	.p2align	3, 0x0
l_anon.[ID].5:
	.byte	17
	.space	39

	.p2align	3, 0x0
l_anon.[ID].6:
	.space	1
	.space	39

l_anon.[ID].7:
	.ascii	"_NSZone"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].8:
	.byte	28
	.space	7
	.quad	l_anon.[ID].7
	.asciz	"\007\000\000\000\000\000\000"
	.quad	l_anon.[ID].3
	.space	8

	.p2align	3, 0x0
l_anon.[ID].9:
	.byte	25
	.space	7
	.quad	l_anon.[ID].8
	.space	24

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].10:
	.byte	21
	.space	39

l_anon.[ID].11:
	.ascii	"ivars"

l_anon.[ID].12:
	.ascii	"drop_flag"

	.p2align	3, 0x0
l_anon.[ID].13:
	.byte	5
	.space	39

	.p2align	3, 0x0
l_anon.[ID].14:
	.byte	7
	.space	39

	.p2align	3, 0x0
l_anon.[ID].15:
	.byte	9
	.space	39

l_anon.[ID].16:
	.ascii	"ForgetableIvars"

l_anon.[ID].17:
	.ascii	"NoIvars"

l_anon.[ID].18:
	.ascii	"DropIvars"

l_anon.[ID].19:
	.ascii	"called `Option::unwrap()` on a `None` value"

l_anon.[ID].20:
	.ascii	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].21:
	.quad	l_anon.[ID].20
	.asciz	"p\000\000\000\000\000\000\000\225\000\000\0002\000\000"

.zerofill __DATA,__bss,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 0).0,8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 2),8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 1)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 1),8,3
	.section	__TEXT,__const
l_anon.[ID].22:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].23:
	.quad	l_anon.[ID].22
	.asciz	"5\000\000\000\000\000\000\000\016\000\000\000\001\000\000"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0),8,3
	.section	__TEXT,__literal8,8byte_literals
L_anon.[ID].24:
	.ascii	"NSObject"

	.section	__TEXT,__const
l_anon.[ID].25:
	.ascii	"NSCopying"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_03fd85b0462f54e9
L_OBJC_METH_VAR_NAME_03fd85b0462f54e9:
	.asciz	"classMethod"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_03fd85b0462f54e9
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_03fd85b0462f54e9:
	.quad	L_OBJC_METH_VAR_NAME_03fd85b0462f54e9

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_03fd85b0462f54e9
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_03fd85b0462f54e9:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_cf773331f3cfba54
L_OBJC_METH_VAR_NAME_cf773331f3cfba54:
	.asciz	"method"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_cf773331f3cfba54
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_cf773331f3cfba54:
	.quad	L_OBJC_METH_VAR_NAME_cf773331f3cfba54

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_cf773331f3cfba54
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_cf773331f3cfba54:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_abdcbb85641cd990
L_OBJC_METH_VAR_NAME_abdcbb85641cd990:
	.asciz	"methodBool:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_abdcbb85641cd990
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_abdcbb85641cd990:
	.quad	L_OBJC_METH_VAR_NAME_abdcbb85641cd990

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_abdcbb85641cd990
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_abdcbb85641cd990:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_ef8de92414f2d9c8
L_OBJC_METH_VAR_NAME_ef8de92414f2d9c8:
	.asciz	"methodId"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_ef8de92414f2d9c8
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_ef8de92414f2d9c8:
	.quad	L_OBJC_METH_VAR_NAME_ef8de92414f2d9c8

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_ef8de92414f2d9c8
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_ef8de92414f2d9c8:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_4a611090161f3fae
L_OBJC_METH_VAR_NAME_4a611090161f3fae:
	.asciz	"methodIdWithParam:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_4a611090161f3fae
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_4a611090161f3fae:
	.quad	L_OBJC_METH_VAR_NAME_4a611090161f3fae

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_4a611090161f3fae
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_4a611090161f3fae:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2837f061c311eb14
L_OBJC_METH_VAR_NAME_2837f061c311eb14:
	.asciz	"copyWithZone:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2837f061c311eb14
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_2837f061c311eb14:
	.quad	L_OBJC_METH_VAR_NAME_2837f061c311eb14

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2837f061c311eb14
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2837f061c311eb14:
	.asciz	"\000\000\000\000@\000\000"

.zerofill __DATA,__bss,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 2).0,8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 1),8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 2)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 2),8,3
	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].26:
	.quad	l_anon.[ID].22
	.asciz	"5\000\000\000\000\000\000\000O\000\000\000\001\000\000"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0),8,3
	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_af8966656b8b2b6c
L_OBJC_METH_VAR_NAME_af8966656b8b2b6c:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_af8966656b8b2b6c
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_af8966656b8b2b6c:
	.quad	L_OBJC_METH_VAR_NAME_af8966656b8b2b6c

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_af8966656b8b2b6c
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_af8966656b8b2b6c:
	.asciz	"\000\000\000\000@\000\000"

.zerofill __DATA,__bss,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_CLASS, 1).0,8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_IVAR_OFFSET, 0),8,3
	.globl	SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0)
.zerofill __DATA,__common,SYM(test_declare_class[CRATE_ID]::_::__OBJC2_DROP_FLAG_OFFSET, 0),8,3
	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].27:
	.quad	l_anon.[ID].22
	.asciz	"5\000\000\000\000\000\000\000x\000\000\000\001\000\000"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0),8,3
	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6edddcebbded8f32
L_OBJC_METH_VAR_NAME_6edddcebbded8f32:
	.asciz	"init"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_6edddcebbded8f32
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_6edddcebbded8f32:
	.quad	L_OBJC_METH_VAR_NAME_6edddcebbded8f32

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_6edddcebbded8f32
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_6edddcebbded8f32:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
