	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.p2align	4, 0x90
SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}>, 0):
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.p2align	4, 0x90
SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r12
	push	rbx
	sub	rsp, 64
	mov	rax, qword ptr [rdi]
	cmp	byte ptr [rax], 0
	mov	byte ptr [rax], 0
	je	LBB1_3
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rdx, qword ptr [rax]
	lea	rdi, [rip + l_anon.[ID].16]
	mov	esi, 15
	call	SYM(objc2::declare::ClassBuilder::new::GENERATED_ID, 0)
	test	rax, rax
	je	LBB1_4
	mov	qword ptr [rbp - 80], rax
	lea	rsi, [rip + L_anon.[ID].11]
	lea	r9, [rip + l_anon.[ID].12]
	lea	rbx, [rbp - 80]
	mov	edx, 4
	mov	ecx, 1
	mov	rdi, rbx
	xor	r8d, r8d
	call	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	lea	rsi, [rip + L_anon.[ID].13]
	lea	r14, [rip + l_anon.[ID].2]
	mov	edx, 4
	mov	ecx, 8
	mov	rdi, rbx
	mov	r8d, 3
	mov	r9, r14
	call	SYM(objc2::declare::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_dealloc@GOTPCREL]
	mov	rsi, qword ptr [rax]
	lea	r15, [rip + l_anon.[ID].1]
	lea	r12, [rip + l_anon.[ID].3]
	lea	r9, [rip + SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::__objc2_dealloc, 0)]
	mov	rdi, rbx
	mov	rdx, r15
	xor	ecx, ecx
	mov	r8, r12
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_init@GOTPCREL]
	mov	rsi, qword ptr [rax]
	lea	r9, [rip + _init]
	mov	rdi, rbx
	mov	rdx, r15
	xor	ecx, ecx
	mov	r8, r14
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_d874ee9262978be2]
	lea	r9, [rip + _class_method]
	mov	rdi, rbx
	mov	rdx, r15
	xor	ecx, ecx
	mov	r8, r12
	call	SYM(objc2::declare::ClassBuilder::add_class_method_inner::GENERATED_ID, 0)
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_4539fd1dbda0cddc]
	lea	r9, [rip + _method]
	mov	rdi, rbx
	mov	rdx, r15
	xor	ecx, ecx
	mov	r8, r12
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_2b1b3a94e0ece2e5]
	lea	r12, [rip + l_anon.[ID].4]
	lea	r9, [rip + _method_bool]
	mov	ecx, 1
	mov	rdi, rbx
	mov	rdx, r12
	mov	r8, r12
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_f7f521670860b0ce]
	lea	r9, [rip + _method_id]
	mov	rdi, rbx
	mov	rdx, r15
	xor	ecx, ecx
	mov	r8, r14
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_6addfcf634c6232f]
	lea	r9, [rip + _method_id_with_param]
	mov	ecx, 1
	mov	rdi, rbx
	mov	rdx, r12
	mov	r8, r14
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	lea	rdi, [rip + l_anon.[ID].17]
	mov	esi, 9
	call	SYM(objc2::runtime::AnyProtocol::get::GENERATED_ID, 0)
	mov	rdi, rbx
	mov	rsi, rax
	call	SYM(objc2::__macro_helpers::<impl objc2::declare::ClassBuilder>::__add_protocol_methods::GENERATED_ID, 0)
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_4a8c690dbc9d8166]
	lea	rdx, [rip + l_anon.[ID].7]
	lea	r9, [rip + _copyWithZone]
	mov	ecx, 1
	mov	rdi, rax
	mov	r8, r14
	call	SYM(objc2::declare::ClassBuilder::add_method_inner::GENERATED_ID, 0)
	mov	rdi, qword ptr [rbp - 80]
	call	SYM(objc2::declare::ClassBuilder::register::GENERATED_ID, 0)
	add	rsp, 64
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB1_3:
	lea	rdi, [rip + l_anon.[ID].8]
	lea	rdx, [rip + l_anon.[ID].10]
	mov	esi, 43
	call	SYM(core::panicking::panic::GENERATED_ID, 0)
LBB1_4:
	lea	rax, [rip + l_anon.[ID].21]
	mov	qword ptr [rbp - 96], rax
	lea	rax, [rip + SYM(<&str as core[CRATE_ID]::fmt::Display>::fmt, 0)]
	mov	qword ptr [rbp - 88], rax
	lea	rax, [rip + l_anon.[ID].20]
	mov	qword ptr [rbp - 80], rax
	mov	qword ptr [rbp - 72], 2
	mov	qword ptr [rbp - 48], 0
	lea	rax, [rbp - 96]
	mov	qword ptr [rbp - 64], rax
	mov	qword ptr [rbp - 56], 1
	lea	rsi, [rip + l_anon.[ID].15]
	lea	rdi, [rbp - 80]
	call	SYM(core::panicking::panic_fmt::GENERATED_ID, 0)

	.p2align	4, 0x90
SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rdi]
	mov	qword ptr [rbp - 8], rax
	lea	rdi, [rbp - 8]
	call	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	add	rsp, 16
	pop	rbp
	ret

	.p2align	4, 0x90
SYM(<&str as core[CRATE_ID]::fmt::Display>::fmt, 0):
	push	rbp
	mov	rbp, rsp
	mov	rdx, rsi
	mov	rax, qword ptr [rdi]
	mov	rsi, qword ptr [rdi + 8]
	mov	rdi, rax
	pop	rbp
	jmp	SYM(<str as core::fmt::Display>::fmt::GENERATED_ID, 0)

	.globl	_get_class
	.p2align	4, 0x90
_get_class:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB4_1
LBB4_2:
	lea	rdi, [rip + l_anon.[ID].16]
	mov	esi, 15
	call	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	test	rax, rax
	je	LBB4_4
	add	rsp, 16
	pop	rbp
	ret
LBB4_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].15]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	jmp	LBB4_2
LBB4_4:
	lea	rdi, [rip + l_anon.[ID].8]
	lea	rdx, [rip + l_anon.[ID].15]
	mov	esi, 43
	call	SYM(core::panicking::panic::GENERATED_ID, 0)

	.globl	_get_obj
	.p2align	4, 0x90
_get_obj:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_new@GOTPCREL]
	mov	rbx, qword ptr [rax]
	call	_get_class
	mov	rdi, rax
	mov	rsi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_msgSend

	.globl	_access_ivars
	.p2align	4, 0x90
_access_ivars:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	call	_get_obj
	mov	rbx, rax
	mov	rdi, rax
	call	_object_getClass
	lea	rsi, [rip + L_anon.[ID].11]
	lea	rcx, [rip + l_anon.[ID].12]
	mov	edx, 4
	mov	rdi, rax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	movzx	r14d, byte ptr [rbx + rax]
	mov	rdi, rbx
	call	_object_getClass
	lea	rsi, [rip + L_anon.[ID].13]
	lea	rcx, [rip + l_anon.[ID].2]
	mov	edx, 4
	mov	rdi, rax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	mov	r15, qword ptr [rbx + rax]
	mov	rdi, rbx
	call	_objc_release
	mov	eax, r14d
	mov	rdx, r15
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret

	.globl	SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	4, 0x90
SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rip + SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	cmp	rax, 3
	jne	LBB7_1
LBB7_2:
	lea	rdi, [rip + l_anon.[ID].16]
	mov	esi, 15
	call	SYM(objc2::runtime::AnyClass::get::GENERATED_ID, 0)
	test	rax, rax
	je	LBB7_4
	add	rsp, 16
	pop	rbp
	ret
LBB7_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0)]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].15]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys_common::once::queue::Once::call::GENERATED_ID, 0)
	jmp	LBB7_2
LBB7_4:
	lea	rdi, [rip + l_anon.[ID].8]
	lea	rdx, [rip + l_anon.[ID].15]
	mov	esi, 43
	call	SYM(core::panicking::panic::GENERATED_ID, 0)

	.p2align	4, 0x90
SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}::__objc2_dealloc, 0):
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rsi
	mov	r14, rdi
	call	_object_getClass
	lea	rsi, [rip + L_anon.[ID].13]
	lea	rcx, [rip + l_anon.[ID].2]
	mov	edx, 4
	mov	rdi, rax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	mov	rdi, qword ptr [r14 + rax]
	test	rdi, rdi
	je	LBB8_2
	call	_objc_release
LBB8_2:
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

	.globl	_init
	.p2align	4, 0x90
_init:
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
	je	LBB9_2
	mov	rdi, rbx
	call	_object_getClass
	lea	rsi, [rip + L_anon.[ID].11]
	lea	rcx, [rip + l_anon.[ID].12]
	mov	edx, 4
	mov	rdi, rax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	mov	byte ptr [rbx + rax], 42
	mov	rdi, rbx
	call	_object_getClass
	lea	rsi, [rip + L_anon.[ID].13]
	lea	rcx, [rip + l_anon.[ID].2]
	mov	edx, 4
	mov	rdi, rax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	mov	qword ptr [rbx + rax], 0
LBB9_2:
	mov	rax, rbx
	add	rsp, 24
	pop	rbx
	pop	rbp
	ret

	.globl	_class_method
	.p2align	4, 0x90
_class_method:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.globl	_method
	.p2align	4, 0x90
_method:
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
	push	rbx
	push	rax
	mov	rbx, rdi
	call	_object_getClass
	lea	rsi, [rip + L_anon.[ID].13]
	lea	rcx, [rip + l_anon.[ID].2]
	mov	edx, 4
	mov	rdi, rax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	mov	rdi, qword ptr [rbx + rax]
	test	rdi, rdi
	je	LBB13_1
	call	_objc_retain
	mov	rdi, rax
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_autoreleaseReturnValue
LBB13_1:
	xor	edi, edi
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_autoreleaseReturnValue

	.globl	_method_id_with_param
	.p2align	4, 0x90
_method_id_with_param:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	mov	r15d, edx
	mov	r14, rdi
	call	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	rbx, rax
	test	r15b, r15b
	je	LBB14_5
	mov	rdi, r14
	call	_object_getClass
	lea	rsi, [rip + L_anon.[ID].13]
	lea	rcx, [rip + l_anon.[ID].2]
	mov	edx, 4
	mov	rdi, rax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	mov	rdi, qword ptr [r14 + rax]
	test	rdi, rdi
	je	LBB14_2
	call	_objc_retain
	mov	r14, rax
	jmp	LBB14_4
LBB14_2:
	xor	r14d, r14d
LBB14_4:
	mov	rdi, rbx
	call	_objc_release
	mov	rbx, r14
LBB14_5:
	mov	rdi, rbx
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	jmp	_objc_autoreleaseReturnValue

	.globl	_copyWithZone
	.p2align	4, 0x90
_copyWithZone:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	push	rax
	mov	r14, rdi
	call	_get_obj
	mov	rbx, rax
	test	rax, rax
	je	LBB15_5
	mov	rdi, r14
	call	_object_getClass
	lea	r15, [rip + L_anon.[ID].11]
	lea	r12, [rip + l_anon.[ID].12]
	mov	edx, 4
	mov	rdi, rax
	mov	rsi, r15
	mov	rcx, r12
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	movzx	r13d, byte ptr [r14 + rax]
	mov	rdi, rbx
	call	_object_getClass
	mov	edx, 4
	mov	rdi, rax
	mov	rsi, r15
	mov	rcx, r12
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	mov	byte ptr [rbx + rax], r13b
	mov	rdi, r14
	call	_object_getClass
	lea	rsi, [rip + L_anon.[ID].13]
	lea	rcx, [rip + l_anon.[ID].2]
	mov	edx, 4
	mov	rdi, rax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	mov	rdi, qword ptr [r14 + rax]
	test	rdi, rdi
	je	LBB15_2
	call	_objc_retain
	mov	r14, rax
	jmp	LBB15_4
LBB15_2:
	xor	r14d, r14d
LBB15_4:
	mov	rdi, rbx
	call	_object_getClass
	lea	rsi, [rip + L_anon.[ID].13]
	lea	rcx, [rip + l_anon.[ID].2]
	mov	edx, 4
	mov	rdi, rax
	call	SYM(objc2::runtime::ivar_offset::GENERATED_ID, 0)
	mov	qword ptr [rbx + rax], r14
LBB15_5:
	mov	rax, rbx
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	ret

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.quad	SYM(core[CRATE_ID]::ptr::drop_in_place::<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}>, 0)
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::once::Once>::call_once<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::once::Once>::call_once::<<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].1:
	.byte	0

	.p2align	3, 0x0
l_anon.[ID].2:
	.byte	19
	.space	39

	.p2align	3, 0x0
l_anon.[ID].3:
	.byte	17
	.space	39

	.p2align	3, 0x0
l_anon.[ID].4:
	.space	1
	.space	39

l_anon.[ID].5:
	.ascii	"_NSZone"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].6:
	.byte	28
	.space	7
	.quad	l_anon.[ID].5
	.asciz	"\007\000\000\000\000\000\000"
	.quad	l_anon.[ID].1
	.space	8

	.p2align	3, 0x0
l_anon.[ID].7:
	.byte	25
	.space	7
	.quad	l_anon.[ID].6
	.space	24

	.section	__TEXT,__const
l_anon.[ID].8:
	.ascii	"called `Option::unwrap()` on a `None` value"

l_anon.[ID].9:
	.ascii	"$RUSTC/library/std/src/sync/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].10:
	.quad	l_anon.[ID].9
	.asciz	"p\000\000\000\000\000\000\000\225\000\000\0002\000\000"

	.section	__TEXT,__literal4,4byte_literals
L_anon.[ID].11:
	.ascii	"_foo"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].12:
	.byte	5
	.space	39

	.section	__TEXT,__literal4,4byte_literals
L_anon.[ID].13:
	.ascii	"_obj"

	.section	__TEXT,__const
l_anon.[ID].14:
	.ascii	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].15:
	.quad	l_anon.[ID].14
	.asciz	"5\000\000\000\000\000\000\000\f\000\000\000\001\000\000"

	.section	__TEXT,__const
l_anon.[ID].16:
	.ascii	"CustomClassName"

.zerofill __DATA,__bss,SYM(<test_declare_class[CRATE_ID]::Custom as objc2[CRATE_ID]::top_level_traits::ClassType>::class::REGISTER_CLASS, 0),8,3
l_anon.[ID].17:
	.ascii	"NSCopying"

l_anon.[ID].18:
	.ascii	"could not create new class "

l_anon.[ID].19:
	.ascii	". Perhaps a class with that name already exists?"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].20:
	.quad	l_anon.[ID].18
	.asciz	"\033\000\000\000\000\000\000"
	.quad	l_anon.[ID].19
	.asciz	"0\000\000\000\000\000\000"

	.p2align	3, 0x0
l_anon.[ID].21:
	.quad	l_anon.[ID].16
	.asciz	"\017\000\000\000\000\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_d874ee9262978be2
L_OBJC_METH_VAR_NAME_d874ee9262978be2:
	.asciz	"classMethod"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_d874ee9262978be2
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_d874ee9262978be2:
	.quad	L_OBJC_METH_VAR_NAME_d874ee9262978be2

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_d874ee9262978be2
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_d874ee9262978be2:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_4539fd1dbda0cddc
L_OBJC_METH_VAR_NAME_4539fd1dbda0cddc:
	.asciz	"method"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_4539fd1dbda0cddc
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_4539fd1dbda0cddc:
	.quad	L_OBJC_METH_VAR_NAME_4539fd1dbda0cddc

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_4539fd1dbda0cddc
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_4539fd1dbda0cddc:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_2b1b3a94e0ece2e5
L_OBJC_METH_VAR_NAME_2b1b3a94e0ece2e5:
	.asciz	"methodBool:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_2b1b3a94e0ece2e5
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_2b1b3a94e0ece2e5:
	.quad	L_OBJC_METH_VAR_NAME_2b1b3a94e0ece2e5

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_2b1b3a94e0ece2e5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_2b1b3a94e0ece2e5:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_f7f521670860b0ce
L_OBJC_METH_VAR_NAME_f7f521670860b0ce:
	.asciz	"methodId"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_f7f521670860b0ce
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_f7f521670860b0ce:
	.quad	L_OBJC_METH_VAR_NAME_f7f521670860b0ce

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_f7f521670860b0ce
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f7f521670860b0ce:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_6addfcf634c6232f
L_OBJC_METH_VAR_NAME_6addfcf634c6232f:
	.asciz	"methodIdWithParam:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_6addfcf634c6232f
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_6addfcf634c6232f:
	.quad	L_OBJC_METH_VAR_NAME_6addfcf634c6232f

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_6addfcf634c6232f
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_6addfcf634c6232f:
	.asciz	"\000\000\000\000@\000\000"

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_4a8c690dbc9d8166
L_OBJC_METH_VAR_NAME_4a8c690dbc9d8166:
	.asciz	"copyWithZone:"

	.section	__DATA,__objc_selrefs,literal_pointers,no_dead_strip
	.globl	L_OBJC_SELECTOR_REFERENCES_4a8c690dbc9d8166
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_4a8c690dbc9d8166:
	.quad	L_OBJC_METH_VAR_NAME_4a8c690dbc9d8166

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_4a8c690dbc9d8166
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_4a8c690dbc9d8166:
	.asciz	"\000\000\000\000@\000\000"

.subsections_via_symbols
