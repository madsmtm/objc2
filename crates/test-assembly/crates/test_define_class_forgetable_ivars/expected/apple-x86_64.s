	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.p2align	4
SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin0:
	push	rbp
	mov	rbp, rsp
	push	rbx
	sub	rsp, 56
	mov	rax, qword ptr [rdi]
	movzx	ecx, byte ptr [rax]
	mov	byte ptr [rax], 0
	cmp	cl, 1
	jne	LBB0_8
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rdi, qword ptr [rax]
	lea	rsi, [rip + L_anon.[ID].6]
	xor	edx, edx
	call	_objc_allocateClassPair
	test	rax, rax
	je	LBB0_6
	mov	qword ptr [rbp - 16], rax
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_init@GOTPCREL]
	mov	rsi, qword ptr [rax]
Ltmp0:
	lea	r8, [rip + l_anon.[ID].5]
	lea	r9, [rip + _fn1_init]
	lea	rdi, [rbp - 16]
	mov	edx, 8
	xor	ecx, ecx
	call	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp1:
	mov	qword ptr [rbp - 48], 2
	lea	rax, [rip + l_anon.[ID].1]
	mov	qword ptr [rbp - 40], rax
	mov	byte ptr [rbp - 56], 29
Ltmp2:
	lea	rsi, [rip + L_anon.[ID].2]
	lea	rdi, [rbp - 16]
	lea	r9, [rbp - 56]
	mov	edx, 6
	mov	ecx, 8
	mov	r8d, 2
	call	SYM(objc2::runtime::define::ClassBuilder::add_ivar_inner_mono::GENERATED_ID, 0)
Ltmp3:
	mov	rbx, qword ptr [rbp - 16]
	mov	rdi, rbx
	call	_objc_registerClassPair
	lea	rsi, [rip + L_anon.[ID].2]
	mov	rdi, rbx
	call	_class_getInstanceVariable
	test	rax, rax
	je	LBB0_5
	mov	rdi, rax
	call	_ivar_getOffset
	mov	qword ptr [rip + ___CLASS_ForgetableIvars], rbx
	mov	qword ptr [rip + ___IVAR_OFFSET_ForgetableIvars], rax
	add	rsp, 56
	pop	rbx
	pop	rbp
	ret
LBB0_8:
	lea	rdi, [rip + l_anon.[ID].4]
	call	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB0_6:
	lea	rdi, [rip + L_anon.[ID].6]
	lea	rdx, [rip + l_anon.[ID].8]
	mov	esi, 16
	call	SYM(objc2::__macros::define_class::checks::class_not_unique::GENERATED_ID, 0)
LBB0_5:
	call	SYM(objc2::__macros::define_class::ivars::ivars_offset::get_ivar_failed::GENERATED_ID, 0)
LBB0_9:
Ltmp4:
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 16]
	call	_objc_disposeClassPair
	mov	rdi, rbx
	call	__Unwind_Resume
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
	.p2align	4
SYM(<<std[CRATE_ID]::sync::poison::once::Once>::call_once<<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::poison::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rdi]
	mov	qword ptr [rbp - 8], rax
	lea	rdi, [rbp - 8]
	call	SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	add	rsp, 16
	pop	rbp
	ret

	.globl	SYM(<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	4
SYM(<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
	mov	rax, qword ptr [rip + ___REGISTER_CLASS_ForgetableIvars]
	test	rax, rax
	jne	LBB2_1
	mov	rax, qword ptr [rip + ___CLASS_ForgetableIvars]
	ret
LBB2_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + ___REGISTER_CLASS_ForgetableIvars]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].8]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + ___CLASS_ForgetableIvars]
	ret

	.globl	_fn1_init
	.p2align	4
_fn1_init:
	test	rdi, rdi
	je	LBB3_2
	mov	rax, qword ptr [rip + ___IVAR_OFFSET_ForgetableIvars]
	mov	dword ptr [rdi + rax], 43
	mov	byte ptr [rdi + rax + 4], 42
LBB3_2:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_init@GOTPCREL]
	mov	rsi, qword ptr [rax]
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rax, qword ptr [rax]
	mov	qword ptr [rbp - 16], rdi
	mov	qword ptr [rbp - 8], rax
	lea	rdi, [rbp - 16]
	call	_objc_msgSendSuper
	add	rsp, 16
	pop	rbp
	ret

	.globl	_fn2_access_class
	.p2align	4
_fn2_access_class:
	mov	rax, qword ptr [rip + ___REGISTER_CLASS_ForgetableIvars]
	test	rax, rax
	jne	LBB4_1
	mov	rax, qword ptr [rip + ___CLASS_ForgetableIvars]
	ret
LBB4_1:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + ___REGISTER_CLASS_ForgetableIvars]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].8]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + ___CLASS_ForgetableIvars]
	ret

	.globl	_fn3_access_ivars
	.p2align	4
_fn3_access_ivars:
	push	rbp
	mov	rbp, rsp
	mov	rcx, qword ptr [rip + ___IVAR_OFFSET_ForgetableIvars]
	movzx	eax, byte ptr [rdi + rcx + 4]
	mov	edx, dword ptr [rdi + rcx]
	pop	rbp
	ret

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::poison::once::Once>::call_once<<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::poison::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class_forgetable_ivars[CRATE_ID]::ForgetableIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].1:
	.byte	8
	.space	39

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].2:
	.asciz	"ivars"

L_anon.[ID].3:
	.asciz	"$RUSTC/library/std/src/sync/poison/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].4:
	.quad	L_anon.[ID].3
	.asciz	"\20x\000\000\000\000\000\000\000\234\000\000\0002\000\000"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].5:
	.byte	21
	.space	39

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].6:
	.asciz	"ForgetableIvars"

L_anon.[ID].7:
	.asciz	"crates/$DIR/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].8:
	.quad	L_anon.[ID].7
	.asciz	"E\000\000\000\000\000\000\000\022\000\000\000\001\000\000"

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
