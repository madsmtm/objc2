	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.p2align	4
SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0):
Lfunc_begin0:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rax, qword ptr [rdi]
	movzx	ecx, byte ptr [rax]
	mov	byte ptr [rax], 0
	cmp	cl, 1
	jne	LBB0_17
	mov	rax, qword ptr [rip + L_OBJC_CLASSLIST_REFERENCES_$_NSObject@GOTPCREL]
	mov	rdi, qword ptr [rax]
	lea	rsi, [rip + L_anon.[ID].12]
	xor	edx, edx
	call	_objc_allocateClassPair
	test	rax, rax
	je	LBB0_16
	mov	qword ptr [rbp - 16], rax
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_7a01db584ce7207f]
Ltmp0:
	lea	r8, [rip + l_anon.[ID].3]
	lea	r9, [rip + _fn1_get_class]
	lea	rdi, [rbp - 16]
	mov	edx, 8
	xor	ecx, ecx
	call	SYM(objc2::runtime::define::ClassBuilder::add_class_method_inner::GENERATED_ID, 0)
Ltmp1:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_7af348e2d6ca08be]
Ltmp2:
	lea	r8, [rip + l_anon.[ID].4]
	lea	r9, [rip + _fn2_method_simple]
	lea	rdi, [rbp - 16]
	mov	edx, 8
	xor	ecx, ecx
	call	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp3:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_59b65ebff80505d9]
Ltmp4:
	lea	rdx, [rip + l_anon.[ID].5]
	lea	r9, [rip + _fn3_method_bool]
	lea	rdi, [rbp - 16]
	mov	ecx, 1
	mov	r8, rdx
	call	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp5:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_f8d5ad39f22c9fc5]
Ltmp6:
	lea	r8, [rip + l_anon.[ID].6]
	lea	r9, [rip + _fn4_method_retained]
	lea	rdi, [rbp - 16]
	mov	edx, 8
	xor	ecx, ecx
	call	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp7:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_31d62824a4f71757]
Ltmp8:
	lea	rdx, [rip + l_anon.[ID].5]
	lea	r8, [rip + l_anon.[ID].6]
	lea	r9, [rip + _fn5_method_retained_with_param]
	lea	rdi, [rbp - 16]
	mov	ecx, 1
	call	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp9:
Ltmp10:
	lea	rdi, [rip + L_anon.[ID].7]
	mov	esi, 8
	call	SYM(objc2::top_level_traits::get_protocol::GENERATED_ID, 0)
Ltmp11:
	test	rax, rax
	je	LBB0_10
	mov	rdi, qword ptr [rbp - 16]
	mov	rsi, rax
	call	_class_addProtocol
LBB0_10:
Ltmp12:
	lea	rdi, [rip + l_anon.[ID].8]
	mov	esi, 9
	call	SYM(objc2::top_level_traits::get_protocol::GENERATED_ID, 0)
Ltmp13:
	test	rax, rax
	je	LBB0_13
	mov	rdi, qword ptr [rbp - 16]
	mov	rsi, rax
	call	_class_addProtocol
LBB0_13:
	mov	rsi, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_217b1b90b2d164b1]
Ltmp14:
	lea	rdx, [rip + l_anon.[ID].11]
	lea	r8, [rip + l_anon.[ID].6]
	lea	r9, [rip + _fn6_copyWithZone]
	lea	rdi, [rbp - 16]
	mov	ecx, 1
	call	SYM(objc2::runtime::define::ClassBuilder::add_method_inner::GENERATED_ID, 0)
Ltmp15:
	mov	rbx, qword ptr [rbp - 16]
	mov	rdi, rbx
	call	_objc_registerClassPair
	mov	qword ptr [rip + ___CLASS_NoIvars], rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret
LBB0_17:
	lea	rdi, [rip + l_anon.[ID].2]
	call	SYM(core::option::unwrap_failed::GENERATED_ID, 0)
LBB0_16:
	lea	rdi, [rip + L_anon.[ID].12]
	lea	rdx, [rip + l_anon.[ID].14]
	mov	esi, 8
	call	SYM(objc2::__macros::define_class::checks::class_not_unique::GENERATED_ID, 0)
LBB0_15:
Ltmp16:
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
	.uleb128 Ltmp15-Ltmp0
	.uleb128 Ltmp16-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp15-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp15
	.byte	0
	.byte	0
Lcst_end0:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	4
SYM(<<std[CRATE_ID]::sync::poison::once::Once>::call_once<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::poison::once::OnceState,)>>::call_once::{shim:vtable#0}, 0):
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rdi]
	mov	qword ptr [rbp - 8], rax
	lea	rdi, [rbp - 8]
	call	SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)
	add	rsp, 16
	pop	rbp
	ret

	.globl	SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0)
	.p2align	4
SYM(<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class, 0):
	mov	rax, qword ptr [rip + ___REGISTER_CLASS_NoIvars]
	test	rax, rax
	jne	LBB2_1
	mov	rax, qword ptr [rip + ___CLASS_NoIvars]
	ret
LBB2_1:
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
	call	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + ___CLASS_NoIvars]
	ret

	.globl	_fn1_get_class
	.p2align	4
_fn1_get_class:
	mov	rax, qword ptr [rip + ___REGISTER_CLASS_NoIvars]
	test	rax, rax
	jne	LBB3_1
	mov	rax, qword ptr [rip + ___CLASS_NoIvars]
	ret
LBB3_1:
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
	call	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	add	rsp, 16
	pop	rbp
	mov	rax, qword ptr [rip + ___CLASS_NoIvars]
	ret

	.globl	_fn2_method_simple
	.p2align	4
_fn2_method_simple:
	push	rbp
	mov	rbp, rsp
	pop	rbp
	ret

	.globl	_fn3_method_bool
	.p2align	4
_fn3_method_bool:
	push	rbp
	mov	rbp, rsp
	xor	eax, eax
	test	dl, dl
	sete	al
	pop	rbp
	ret

	.globl	_fn4_method_retained
	.p2align	4
_fn4_method_retained:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	rax, qword ptr [rip + ___REGISTER_CLASS_NoIvars]
	test	rax, rax
	jne	LBB6_1
LBB6_2:
	mov	rdi, qword ptr [rip + ___CLASS_NoIvars]
	mov	rax, qword ptr [rip + L_OBJC_SELECTOR_REFERENCES_new@GOTPCREL]
	mov	rsi, qword ptr [rax]
	call	_objc_msgSend
	mov	rdi, rax
	call	_objc_autoreleaseReturnValue
	add	rsp, 16
	pop	rbp
	ret
LBB6_1:
	mov	byte ptr [rbp - 1], 1
	lea	rax, [rbp - 1]
	mov	qword ptr [rbp - 16], rax
	lea	rdi, [rip + ___REGISTER_CLASS_NoIvars]
	lea	rcx, [rip + l_anon.[ID].0]
	lea	r8, [rip + l_anon.[ID].14]
	lea	rdx, [rbp - 16]
	xor	esi, esi
	call	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	jmp	LBB6_2

	.globl	_fn5_method_retained_with_param
	.p2align	4
_fn5_method_retained_with_param:
Lfunc_begin1:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	r14d, edx
	call	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
	mov	rbx, rax
	test	r14b, r14b
	je	LBB7_3
Ltmp17:
	call	SYM(objc2::runtime::nsobject::NSObject::new::GENERATED_ID, 0)
Ltmp18:
	mov	r14, rax
	mov	rdi, rbx
	call	_objc_release
	mov	rbx, r14
LBB7_3:
	mov	rdi, rbx
	pop	rbx
	pop	r14
	pop	rbp
	jmp	_objc_autoreleaseReturnValue
LBB7_5:
Ltmp19:
	mov	r14, rax
Ltmp20:
	mov	rdi, rbx
	call	_objc_release
Ltmp21:
	mov	rdi, r14
	call	__Unwind_Resume
LBB7_4:
Ltmp22:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end1:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table7:
Lexception1:
	.byte	255
	.byte	155
	.uleb128 Lttbase0-Lttbaseref0
Lttbaseref0:
	.byte	1
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
	.uleb128 Lfunc_begin1-Lfunc_begin1
	.uleb128 Ltmp17-Lfunc_begin1
	.byte	0
	.byte	0
	.uleb128 Ltmp17-Lfunc_begin1
	.uleb128 Ltmp18-Ltmp17
	.uleb128 Ltmp19-Lfunc_begin1
	.byte	0
	.uleb128 Ltmp18-Lfunc_begin1
	.uleb128 Ltmp20-Ltmp18
	.byte	0
	.byte	0
	.uleb128 Ltmp20-Lfunc_begin1
	.uleb128 Ltmp21-Ltmp20
	.uleb128 Ltmp22-Lfunc_begin1
	.byte	1
	.uleb128 Ltmp21-Lfunc_begin1
	.uleb128 Lfunc_end1-Ltmp21
	.byte	0
	.byte	0
Lcst_end1:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_fn6_copyWithZone
	.p2align	4
_fn6_copyWithZone:
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
	call	SYM(std::sys::sync::once::queue::Once::call::GENERATED_ID, 0)
	jmp	LBB8_2

	.section	__TEXT,__objc_methname,cstring_literals
	.globl	L_OBJC_METH_VAR_NAME_217b1b90b2d164b1
L_OBJC_METH_VAR_NAME_217b1b90b2d164b1:
	.asciz	"copyWithZone:"

	.globl	L_OBJC_METH_VAR_NAME_31d62824a4f71757
L_OBJC_METH_VAR_NAME_31d62824a4f71757:
	.asciz	"methodRetainedWithParam:"

	.globl	L_OBJC_METH_VAR_NAME_59b65ebff80505d9
L_OBJC_METH_VAR_NAME_59b65ebff80505d9:
	.asciz	"methodBool:"

	.globl	L_OBJC_METH_VAR_NAME_7a01db584ce7207f
L_OBJC_METH_VAR_NAME_7a01db584ce7207f:
	.asciz	"classMethod"

	.globl	L_OBJC_METH_VAR_NAME_7af348e2d6ca08be
L_OBJC_METH_VAR_NAME_7af348e2d6ca08be:
	.asciz	"method"

	.globl	L_OBJC_METH_VAR_NAME_f8d5ad39f22c9fc5
L_OBJC_METH_VAR_NAME_f8d5ad39f22c9fc5:
	.asciz	"methodRetained"

	.section	__DATA,__objc_selrefs,literal_pointers
	.globl	L_OBJC_SELECTOR_REFERENCES_217b1b90b2d164b1
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_217b1b90b2d164b1:
	.quad	L_OBJC_METH_VAR_NAME_217b1b90b2d164b1

	.globl	L_OBJC_SELECTOR_REFERENCES_31d62824a4f71757
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_31d62824a4f71757:
	.quad	L_OBJC_METH_VAR_NAME_31d62824a4f71757

	.globl	L_OBJC_SELECTOR_REFERENCES_59b65ebff80505d9
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_59b65ebff80505d9:
	.quad	L_OBJC_METH_VAR_NAME_59b65ebff80505d9

	.globl	L_OBJC_SELECTOR_REFERENCES_7a01db584ce7207f
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_7a01db584ce7207f:
	.quad	L_OBJC_METH_VAR_NAME_7a01db584ce7207f

	.globl	L_OBJC_SELECTOR_REFERENCES_7af348e2d6ca08be
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_7af348e2d6ca08be:
	.quad	L_OBJC_METH_VAR_NAME_7af348e2d6ca08be

	.globl	L_OBJC_SELECTOR_REFERENCES_f8d5ad39f22c9fc5
	.p2align	3, 0x0
L_OBJC_SELECTOR_REFERENCES_f8d5ad39f22c9fc5:
	.quad	L_OBJC_METH_VAR_NAME_f8d5ad39f22c9fc5

	.section	__DATA,__objc_imageinfo,regular,no_dead_strip
	.globl	L_OBJC_IMAGE_INFO_217b1b90b2d164b1
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_217b1b90b2d164b1:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_31d62824a4f71757
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_31d62824a4f71757:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_59b65ebff80505d9
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_59b65ebff80505d9:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_7a01db584ce7207f
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_7a01db584ce7207f:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_7af348e2d6ca08be
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_7af348e2d6ca08be:
	.asciz	"\000\000\000\000@\000\000"

	.globl	L_OBJC_IMAGE_INFO_f8d5ad39f22c9fc5
	.p2align	2, 0x0
L_OBJC_IMAGE_INFO_f8d5ad39f22c9fc5:
	.asciz	"\000\000\000\000@\000\000"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	SYM(<<std[CRATE_ID]::sync::poison::once::Once>::call_once<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0} as core[CRATE_ID]::ops::function::FnOnce<(&std[CRATE_ID]::sync::poison::once::OnceState,)>>::call_once::{shim:vtable#0}, 0)
	.quad	SYM(<std[CRATE_ID]::sync::poison::once::Once>::call_once::<<test_define_class_no_ivars[CRATE_ID]::NoIvars as objc2[CRATE_ID]::top_level_traits::ClassType>::class::{closure#0}>::{closure#0}, 0)

	.section	__TEXT,__cstring,cstring_literals
L_anon.[ID].1:
	.asciz	"$RUSTC/library/std/src/sync/poison/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].2:
	.quad	L_anon.[ID].1
	.asciz	"\20x\000\000\000\000\000\000\000\234\000\000\0002\000\000"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.[ID].3:
	.byte	21
	.space	39

	.p2align	3, 0x0
l_anon.[ID].4:
	.byte	17
	.space	39

	.p2align	3, 0x0
l_anon.[ID].5:
	.space	1
	.space	39

	.p2align	3, 0x0
l_anon.[ID].6:
	.byte	19
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
	.byte	28
	.space	7
	.quad	l_anon.[ID].9
	.asciz	"\007\000\000\000\000\000\000"
	.quad	8
	.space	8

	.p2align	3, 0x0
l_anon.[ID].11:
	.byte	25
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
