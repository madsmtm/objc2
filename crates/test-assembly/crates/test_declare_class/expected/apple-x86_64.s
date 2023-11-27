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
	je	LBB3_7
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rdx, qword ptr [rax]
	lea	rdi, [rip + l_anon.[ID].15]
	mov	esi, 7
	call	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
	test	rax, rax
	je	LBB3_8
	mov	qword ptr [rbp - 40], rax
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_f249b8b52b9a1205]
	lea	r14, [rip + l_anon.[ID].3]
	lea	r8, [rip + l_anon.[ID].10]
	lea	r9, [rip + _get_class]
	lea	rbx, [rbp - 40]
	mov	rdi, rbx
	mov	rdx, r14
	xor	ecx, ecx
	call	SYM(objc2::declare::ClassBuilder::add_class_method_inner::GENERATED_ID, 0)
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_2607fe9c3979c381]
	lea	r8, [rip + l_anon.[ID].5]
	lea	r9, [rip + _method_simple]
	mov	rdi, rbx
	mov	rdx, r14
	xor	ecx, ecx
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_5a4952192b00de7f]
	lea	r15, [rip + l_anon.[ID].6]
	lea	r9, [rip + _method_bool]
	mov	ecx, 1
	mov	rdi, rbx
	mov	rdx, r15
	mov	r8, r15
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_bf0386bc74a73c00]
	lea	r12, [rip + l_anon.[ID].4]
	lea	r9, [rip + _method_id]
	mov	rdi, rbx
	mov	rdx, r14
	xor	ecx, ecx
	mov	r8, r12
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_6f0ab2a047fe4a09]
	lea	r9, [rip + _method_id_with_param]
	mov	ecx, 1
	mov	rdi, rbx
	mov	rdx, r15
	mov	r8, r12
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	lea	rdi, [rip + L_anon.[ID].23]
	mov	esi, 8
	call	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	test	rax, rax
	je	LBB3_4
	lea	rdi, [rbp - 40]
	mov	rsi, rax
	call	SYM(objc2::declare::ClassBuilder::add_protocol::GENERATED_ID, 0)
LBB3_4:
	lea	rdi, [rip + l_anon.[ID].24]
	mov	esi, 9
	call	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	test	rax, rax
	je	LBB3_6
	lea	rdi, [rbp - 40]
	mov	rsi, rax
	call	SYM(objc2::declare::ClassBuilder::add_protocol::GENERATED_ID, 0)
LBB3_6:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_062e7a5fdd2d2571]
	lea	rdx, [rip + l_anon.[ID].9]
	lea	r8, [rip + l_anon.[ID].4]
	lea	r9, [rip + _copyWithZone]
	lea	rdi, [rbp - 40]
	mov	ecx, 1
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rdi, qword ptr [rbp - 40]
	call	SYM(objc2::declare::ClassBuilder::register::GENERATED_ID, 0)
	add	rsp, 16
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB3_7:
	lea	rdi, [rip + l_anon.[ID].11]
	lea	rdx, [rip + l_anon.[ID].13]
	mov	esi, 43
	call	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB3_8:
	lea	rdi, [rip + l_anon.[ID].15]
	lea	rdx, [rip + l_anon.[ID].22]
	mov	esi, 7
	call	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)

	.p2align	4, 0x90
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	mov	rax, qword ptr [rdi]
	cmp	byte ptr [rax], 0
	mov	byte ptr [rax], 0
	je	LBB4_3
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rdx, qword ptr [rax]
	lea	rdi, [rip + l_anon.[ID].16]
	mov	esi, 9
	call	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
	test	rax, rax
	je	LBB4_4
	mov	qword ptr [rbp - 32], rax
	lea	rsi, [rip + L_anon.[ID].18]
	lea	rbx, [rip + l_anon.[ID].4]
	lea	r14, [rbp - 32]
	mov	edx, 4
	mov	ecx, 8
	mov	rdi, r14
	mov	r8d, 3
	mov	r9, rbx
	call	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	lea	rsi, [rip + l_anon.[ID].17]
	mov	edx, 11
	mov	ecx, 8
	mov	rdi, r14
	mov	r8d, 3
	mov	r9, rbx
	call	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPCREL]
	mov	rsi, qword ptr [rax]
	lea	r15, [rip + l_anon.[ID].3]
	lea	r8, [rip + l_anon.[ID].5]
	lea	r9, [rip + SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::__objc2_dealloc, 0)]
	mov	rdi, r14
	mov	rdx, r15
	xor	ecx, ecx
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_init@GOTPCREL]
	mov	rsi, qword ptr [rax]
	lea	r9, [rip + _init_drop_ivars]
	mov	rdi, r14
	mov	rdx, r15
	xor	ecx, ecx
	mov	r8, rbx
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rdi, qword ptr [rbp - 32]
	call	SYM(objc2::declare::ClassBuilder::register::GENERATED_ID, 0)
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB4_3:
	lea	rdi, [rip + l_anon.[ID].11]
	lea	rdx, [rip + l_anon.[ID].13]
	mov	esi, 43
	call	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB4_4:
	lea	rdi, [rip + l_anon.[ID].16]
	lea	rdx, [rip + l_anon.[ID].28]
	mov	esi, 9
	call	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)

	.p2align	4, 0x90
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rax, qword ptr [rdi]
	cmp	byte ptr [rax], 0
	mov	byte ptr [rax], 0
	je	LBB5_3
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rdx, qword ptr [rax]
	lea	rdi, [rip + l_anon.[ID].14]
	mov	esi, 15
	call	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
	test	rax, rax
	je	LBB5_4
	mov	qword ptr [rbp - 24], rax
	lea	rsi, [rip + L_anon.[ID].20]
	lea	r9, [rip + l_anon.[ID].25]
	lea	rbx, [rbp - 24]
	mov	edx, 4
	mov	ecx, 1
	mov	rdi, rbx
	xor	r8d, r8d
	call	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	lea	rsi, [rip + L_anon.[ID].19]
	lea	r9, [rip + l_anon.[ID].26]
	mov	edx, 4
	mov	ecx, 4
	mov	rdi, rbx
	mov	r8d, 2
	call	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPCREL]
	mov	rsi, qword ptr [rax]
	lea	r14, [rip + l_anon.[ID].3]
	lea	r8, [rip + l_anon.[ID].5]
	lea	r9, [rip + SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::__objc2_dealloc, 0)]
	mov	rdi, rbx
	mov	rdx, r14
	xor	ecx, ecx
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_init@GOTPCREL]
	mov	rsi, qword ptr [rax]
	lea	r8, [rip + l_anon.[ID].4]
	lea	r9, [rip + _init_forgetable_ivars]
	mov	rdi, rbx
	mov	rdx, r14
	xor	ecx, ecx
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rdi, qword ptr [rbp - 24]
	call	SYM(objc2::declare::ClassBuilder::register::GENERATED_ID, 0)
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB5_3:
	lea	rdi, [rip + l_anon.[ID].11]
	lea	rdx, [rip + l_anon.[ID].13]
	mov	esi, 43
	call	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB5_4:
	lea	rdi, [rip + l_anon.[ID].14]
	lea	rdx, [rip + l_anon.[ID].27]
	mov	esi, 15
	call	SYM(objc2::__macro_helpers::declare_class::failed_declaring_class::GENERATED_ID, 0)

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
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB9_1
LBB9_2:
	lea	rdi, [rip + l_anon.[ID].14]
	mov	esi, 15
	call	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	test	rax, rax
	je	LBB9_4
	add	rsp, 16
	pop	rbp
	ret
LBB9_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].2]
	lea	r8, [rip + l_anon.[ID].27]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	jmp	LBB9_2
LBB9_4:
	lea	rdi, [rip + l_anon.[ID].11]
	lea	rdx, [rip + l_anon.[ID].27]
	mov	esi, 43
	call	SYM(core::panicking::panic::GENERATED_ID, 0)

	.globl	_access_forgetable_ivars
	.p2align	4, 0x90
_access_forgetable_ivars:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	rbx, rdi
	lea	rsi, [rip + L_anon.[ID].20]
	mov	edx, 4
	call	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	mov	rdi, rax
	call	_ivar_getOffset
	movzx	r14d, byte ptr [rbx + rax]
	lea	rsi, [rip + L_anon.[ID].19]
	mov	edx, 4
	mov	rdi, rbx
	call	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	mov	rdi, rax
	call	_ivar_getOffset
	mov	edx, dword ptr [rbx + rax]
	mov	eax, r14d
	pop	rbx
	pop	r14
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
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB12_1
LBB12_2:
	lea	rdi, [rip + l_anon.[ID].16]
	mov	esi, 9
	call	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	test	rax, rax
	je	LBB12_4
	add	rsp, 16
	pop	rbp
	ret
LBB12_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].1]
	lea	r8, [rip + l_anon.[ID].28]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	jmp	LBB12_2
LBB12_4:
	lea	rdi, [rip + l_anon.[ID].11]
	lea	rdx, [rip + l_anon.[ID].28]
	mov	esi, 43
	call	SYM(core::panicking::panic::GENERATED_ID, 0)

	.globl	_access_drop_ivars
	.p2align	4, 0x90
_access_drop_ivars:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rdi
	lea	rsi, [rip + L_anon.[ID].18]
	mov	edx, 4
	call	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	mov	rdi, rax
	call	_ivar_getOffset
	mov	rax, qword ptr [rbx + rax]
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret

	.globl	SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	4, 0x90
SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB14_1
LBB14_2:
	lea	rdi, [rip + l_anon.[ID].15]
	mov	esi, 7
	call	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	test	rax, rax
	je	LBB14_4
	add	rsp, 16
	pop	rbp
	ret
LBB14_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].22]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	jmp	LBB14_2
LBB14_4:
	lea	rdi, [rip + l_anon.[ID].11]
	lea	rdx, [rip + l_anon.[ID].22]
	mov	esi, 43
	call	SYM(core::panicking::panic::GENERATED_ID, 0)

	.globl	_get_class
	.p2align	4, 0x90
_get_class:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB15_1
LBB15_2:
	lea	rdi, [rip + l_anon.[ID].15]
	mov	esi, 7
	call	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	test	rax, rax
	je	LBB15_4
	add	rsp, 16
	pop	rbp
	ret
LBB15_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].22]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	jmp	LBB15_2
LBB15_4:
	lea	rdi, [rip + l_anon.[ID].11]
	lea	rdx, [rip + l_anon.[ID].22]
	mov	esi, 43
	call	SYM(core::panicking::panic::GENERATED_ID, 0)

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
	jne	LBB18_1
LBB18_2:
	lea	rdi, [rip + l_anon.[ID].15]
	mov	esi, 7
	call	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	test	rax, rax
	je	LBB18_4
	mov	rcx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_new@GOTPCREL]
	mov	rsi, qword ptr [rcx]
	mov	rdi, rax
	call	_objc_msgSend
	mov	rdi, rax
	call	_objc_autoreleaseReturnValue
	add	rsp, 16
	pop	rbp
	ret
LBB18_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].22]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	jmp	LBB18_2
LBB18_4:
	lea	rdi, [rip + l_anon.[ID].11]
	lea	rdx, [rip + l_anon.[ID].22]
	mov	esi, 43
	call	SYM(core::panicking::panic::GENERATED_ID, 0)

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
	je	LBB19_2
	call	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	r14, rax
	mov	rdi, rbx
	call	_objc_release
	mov	rbx, r14
LBB19_2:
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
	jne	LBB20_1
LBB20_2:
	lea	rdi, [rip + l_anon.[ID].15]
	mov	esi, 7
	call	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	test	rax, rax
	je	LBB20_4
	mov	rcx, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_new@GOTPCREL]
	mov	rsi, qword ptr [rcx]
	mov	rdi, rax
	call	_objc_msgSend
	add	rsp, 16
	pop	rbp
	ret
LBB20_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].22]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	jmp	LBB20_2
LBB20_4:
	lea	rdi, [rip + l_anon.[ID].11]
	lea	rdx, [rip + l_anon.[ID].22]
	mov	esi, 43
	call	SYM(core::panicking::panic::GENERATED_ID, 0)

	.globl	SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	4, 0x90
SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB21_1
LBB21_2:
	lea	rdi, [rip + l_anon.[ID].14]
	mov	esi, 15
	call	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	test	rax, rax
	je	LBB21_4
	add	rsp, 16
	pop	rbp
	ret
LBB21_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].2]
	lea	r8, [rip + l_anon.[ID].27]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	jmp	LBB21_2
LBB21_4:
	lea	rdi, [rip + l_anon.[ID].11]
	lea	rdx, [rip + l_anon.[ID].27]
	mov	esi, 43
	call	SYM(core::panicking::panic::GENERATED_ID, 0)

	.p2align	4, 0x90
SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::__objc2_dealloc, 0):
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rax, qword ptr [rax]
	mov	qword ptr [rbp - 16], rdi
	mov	qword ptr [rbp - 8], rax
	lea	rdi, [rbp - 16]
	call	_objc_msgSendSuper
	add	rsp, 16
	pop	rbp
	ret

	.globl	_init_forgetable_ivars
	.p2align	4, 0x90
_init_forgetable_ivars:
	push	rbp
	mov	rbp, rsp
	push	rbx
	sub	rsp, 24
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_init@GOTPCREL]
	mov	rsi, qword ptr [rax]
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rax, qword ptr [rax]
	mov	qword ptr [rbp - 24], rdi
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rbp - 24]
	call	_objc_msgSendSuper
	mov	rbx, rax
	test	rax, rax
	je	LBB23_2
	lea	rsi, [rip + L_anon.[ID].20]
	mov	edx, 4
	mov	rdi, rbx
	call	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	mov	rdi, rax
	call	_ivar_getOffset
	mov	byte ptr [rbx + rax], 42
	lea	rsi, [rip + L_anon.[ID].19]
	mov	edx, 4
	mov	rdi, rbx
	call	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	mov	rdi, rax
	call	_ivar_getOffset
	mov	dword ptr [rbx + rax], 43
LBB23_2:
	mov	rax, rbx
	add	rsp, 24
	pop	rbx
	pop	rbp
	ret

	.globl	SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	4, 0x90
SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB24_1
LBB24_2:
	lea	rdi, [rip + l_anon.[ID].16]
	mov	esi, 9
	call	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	test	rax, rax
	je	LBB24_4
	add	rsp, 16
	pop	rbp
	ret
LBB24_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].1]
	lea	r8, [rip + l_anon.[ID].28]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	jmp	LBB24_2
LBB24_4:
	lea	rdi, [rip + l_anon.[ID].11]
	lea	rdx, [rip + l_anon.[ID].28]
	mov	esi, 43
	call	SYM(core::panicking::panic::GENERATED_ID, 0)

	.p2align	4, 0x90
SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::__objc2_dealloc, 0):
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rsi
	mov	r14, rdi
	call	SYM(<test_declare_class[CRATE_ID]::DropIvars as core[CRATE_ID]::ops::drop::Drop>::drop, 0)
	lea	rsi, [rip + L_anon.[ID].18]
	mov	edx, 4
	mov	rdi, r14
	call	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	mov	rdi, rax
	call	_ivar_getOffset
	mov	rdi, qword ptr [r14 + rax]
	test	rdi, rdi
	je	LBB25_2
	call	_objc_release
LBB25_2:
	lea	rsi, [rip + l_anon.[ID].17]
	mov	edx, 11
	mov	rdi, r14
	call	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	mov	rdi, rax
	call	_ivar_getOffset
	mov	rdi, qword ptr [r14 + rax]
	test	rdi, rdi
	je	LBB25_4
	call	_objc_release
LBB25_4:
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rax, qword ptr [rax]
	mov	qword ptr [rbp - 32], r14
	mov	qword ptr [rbp - 24], rax
	lea	rdi, [rbp - 32]
	mov	rsi, rbx
	call	_objc_msgSendSuper
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	rbp
	ret

	.globl	_init_drop_ivars
	.p2align	4, 0x90
_init_drop_ivars:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_init@GOTPCREL]
	mov	rsi, qword ptr [rax]
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rax, qword ptr [rax]
	mov	qword ptr [rbp - 32], rdi
	mov	qword ptr [rbp - 24], rax
	lea	rdi, [rbp - 32]
	call	_objc_msgSendSuper
	mov	rbx, rax
	test	rax, rax
	je	LBB26_2
	call	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	r14, rax
	lea	rsi, [rip + L_anon.[ID].18]
	mov	edx, 4
	mov	rdi, rbx
	call	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	mov	rdi, rax
	call	_ivar_getOffset
	mov	qword ptr [rbx + rax], r14
	call	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	r14, rax
	lea	rsi, [rip + l_anon.[ID].17]
	mov	edx, 11
	mov	rdi, rbx
	call	SYM(objc2::runtime::AnyObject::lookup_instance_variable_dynamically::GENERATED_ID, 0)
	mov	rdi, rax
	call	_ivar_getOffset
	mov	qword ptr [rbx + rax], r14
LBB26_2:
	mov	rax, rbx
	add	rsp, 16
	pop	rbx
	pop	r14
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
	.ascii	"called `Option::unwrap()` on a `None` value"

l_anon.[ID].12:
	.ascii	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].13:
	.quad	l_anon.[ID].12
	.asciz	"p\000\000\000\000\000\000\000\225\000\000\0002\000\000"

	.section	__TEXT,__const
l_anon.[ID].14:
	.ascii	"ForgetableIvars"

l_anon.[ID].15:
	.ascii	"NoIvars"

l_anon.[ID].16:
	.ascii	"DropIvars"

l_anon.[ID].17:
	.ascii	"_obj_option"

	.section	__TEXT,__literal4,4byte_literals
L_anon.[ID].18:
	.ascii	"_obj"

L_anon.[ID].19:
	.ascii	"_bar"

L_anon.[ID].20:
	.ascii	"_foo"

	.section	__TEXT,__const
l_anon.[ID].21:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].22:
	.quad	l_anon.[ID].21
	.asciz	"5\000\000\000\000\000\000\000\017\000\000\000\001\000\000"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0),8,3
	.section	__TEXT,__literal8,8byte_literals
L_anon.[ID].23:
	.ascii	"NSObject"

	.section	__TEXT,__const
l_anon.[ID].24:
	.ascii	"NSCopying"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_f249b8b52b9a1205
L_OBJC_METH_VAR_NAME_f249b8b52b9a1205:
	.asciz	"classMethod"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_f249b8b52b9a1205
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_f249b8b52b9a1205:
	.quad	L_OBJC_METH_VAR_NAME_f249b8b52b9a1205

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f249b8b52b9a1205
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f249b8b52b9a1205:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2607fe9c3979c381
L_OBJC_METH_VAR_NAME_2607fe9c3979c381:
	.asciz	"method"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2607fe9c3979c381
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_2607fe9c3979c381:
	.quad	L_OBJC_METH_VAR_NAME_2607fe9c3979c381

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2607fe9c3979c381
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2607fe9c3979c381:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_5a4952192b00de7f
L_OBJC_METH_VAR_NAME_5a4952192b00de7f:
	.asciz	"methodBool:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_5a4952192b00de7f
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_5a4952192b00de7f:
	.quad	L_OBJC_METH_VAR_NAME_5a4952192b00de7f

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_5a4952192b00de7f
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_5a4952192b00de7f:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_bf0386bc74a73c00
L_OBJC_METH_VAR_NAME_bf0386bc74a73c00:
	.asciz	"methodId"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_bf0386bc74a73c00
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_bf0386bc74a73c00:
	.quad	L_OBJC_METH_VAR_NAME_bf0386bc74a73c00

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_bf0386bc74a73c00
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_bf0386bc74a73c00:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6f0ab2a047fe4a09
L_OBJC_METH_VAR_NAME_6f0ab2a047fe4a09:
	.asciz	"methodIdWithParam:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_6f0ab2a047fe4a09
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_6f0ab2a047fe4a09:
	.quad	L_OBJC_METH_VAR_NAME_6f0ab2a047fe4a09

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_6f0ab2a047fe4a09
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_6f0ab2a047fe4a09:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_062e7a5fdd2d2571
L_OBJC_METH_VAR_NAME_062e7a5fdd2d2571:
	.asciz	"copyWithZone:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_062e7a5fdd2d2571
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_062e7a5fdd2d2571:
	.quad	L_OBJC_METH_VAR_NAME_062e7a5fdd2d2571

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_062e7a5fdd2d2571
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_062e7a5fdd2d2571:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].25:
	.byte	5
	.space	39

	.p2align	3, 0x0
l_anon.[ID].26:
	.byte	7
	.space	39

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].27:
	.quad	l_anon.[ID].21
	.asciz	"5\000\000\000\000\000\000\000I\000\000\000\001\000\000"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0),8,3
	.p2align	3, 0x0
l_anon.[ID].28:
	.quad	l_anon.[ID].21
	.asciz	"5\000\000\000\000\000\000\000u\000\000\000\001\000\000"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::DropIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0),8,3
.subsections_via_symbols
