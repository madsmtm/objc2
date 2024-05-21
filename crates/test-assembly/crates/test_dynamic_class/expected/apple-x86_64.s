	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_class
	.p2align	4, 0x90
_get_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0)]
	test	rax, rax
	je	LBB0_2
	pop	rbp
	ret
LBB0_2:
	lea	rdi, [rip + SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0)]
	lea	rsi, [rip + l_anon.[ID].0]
	lea	rdx, [rip + l_anon.[ID].2]
	pop	rbp
	jmp	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)

	.globl	_get_same_class
	.p2align	4, 0x90
_get_same_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + SYM(test_dynamic_class[CRATE_ID]::get_same_class::CACHED_CLASS, 0)]
	test	rax, rax
	je	LBB1_2
	pop	rbp
	ret
LBB1_2:
	lea	rdi, [rip + SYM(test_dynamic_class[CRATE_ID]::get_same_class::CACHED_CLASS, 0)]
	lea	rsi, [rip + l_anon.[ID].0]
	lea	rdx, [rip + l_anon.[ID].3]
	pop	rbp
	jmp	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)

	.globl	_get_different_class
	.p2align	4, 0x90
_get_different_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + SYM(test_dynamic_class[CRATE_ID]::get_different_class::CACHED_CLASS, 0)]
	test	rax, rax
	je	LBB2_2
	pop	rbp
	ret
LBB2_2:
	lea	rdi, [rip + SYM(test_dynamic_class[CRATE_ID]::get_different_class::CACHED_CLASS, 0)]
	lea	rsi, [rip + l_anon.[ID].4]
	lea	rdx, [rip + l_anon.[ID].5]
	pop	rbp
	jmp	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)

	.globl	_unused_class
	.p2align	4, 0x90
_unused_class:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0)]
	test	rax, rax
	je	LBB3_2
	pop	rbp
	ret
LBB3_2:
	lea	rdi, [rip + SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0)]
	lea	rsi, [rip + l_anon.[ID].6]
	lea	rdx, [rip + l_anon.[ID].7]
	pop	rbp
	jmp	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)

	.globl	_use_fns
	.p2align	4, 0x90
_use_fns:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r12
	push	rbx
	mov	rax, rdi
	mov	rbx, qword ptr [rip + SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0)]
	test	rbx, rbx
	je	LBB4_1
	mov	r14, qword ptr [rip + SYM(test_dynamic_class[CRATE_ID]::get_same_class::CACHED_CLASS, 0)]
	test	r14, r14
	je	LBB4_3
LBB4_4:
	mov	r15, qword ptr [rip + SYM(test_dynamic_class[CRATE_ID]::get_different_class::CACHED_CLASS, 0)]
	test	r15, r15
	je	LBB4_5
LBB4_6:
	mov	rcx, qword ptr [rip + SYM(test_dynamic_class[CRATE_ID]::use_fns::CACHED_CLASS, 0)]
	test	rcx, rcx
	je	LBB4_7
LBB4_8:
	mov	qword ptr [rax], rbx
	mov	qword ptr [rax + 8], r14
	mov	qword ptr [rax + 16], r15
	mov	qword ptr [rax + 24], rcx
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB4_1:
	lea	rdi, [rip + SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0)]
	lea	rsi, [rip + l_anon.[ID].0]
	lea	rdx, [rip + l_anon.[ID].2]
	mov	r14, rax
	call	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	rbx, rax
	mov	rax, r14
	mov	r14, qword ptr [rip + SYM(test_dynamic_class[CRATE_ID]::get_same_class::CACHED_CLASS, 0)]
	test	r14, r14
	jne	LBB4_4
LBB4_3:
	lea	rdi, [rip + SYM(test_dynamic_class[CRATE_ID]::get_same_class::CACHED_CLASS, 0)]
	lea	rsi, [rip + l_anon.[ID].0]
	lea	rdx, [rip + l_anon.[ID].3]
	mov	r15, rax
	call	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	r14, rax
	mov	rax, r15
	mov	r15, qword ptr [rip + SYM(test_dynamic_class[CRATE_ID]::get_different_class::CACHED_CLASS, 0)]
	test	r15, r15
	jne	LBB4_6
LBB4_5:
	lea	rdi, [rip + SYM(test_dynamic_class[CRATE_ID]::get_different_class::CACHED_CLASS, 0)]
	lea	rsi, [rip + l_anon.[ID].4]
	lea	rdx, [rip + l_anon.[ID].5]
	mov	r12, rax
	call	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	r15, rax
	mov	rax, r12
	mov	rcx, qword ptr [rip + SYM(test_dynamic_class[CRATE_ID]::use_fns::CACHED_CLASS, 0)]
	test	rcx, rcx
	jne	LBB4_8
LBB4_7:
	lea	rdi, [rip + SYM(test_dynamic_class[CRATE_ID]::use_fns::CACHED_CLASS, 0)]
	lea	rsi, [rip + l_anon.[ID].8]
	lea	rdx, [rip + l_anon.[ID].9]
	mov	r12, rax
	call	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	rcx, rax
	mov	rax, r12
	jmp	LBB4_8

	.globl	_use_same_twice
	.p2align	4, 0x90
_use_same_twice:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	rax, rdi
	mov	rbx, qword ptr [rip + SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0)]
	test	rbx, rbx
	je	LBB5_1
	mov	rcx, qword ptr [rip + SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0)]
	test	rcx, rcx
	je	LBB5_3
LBB5_4:
	mov	qword ptr [rax], rbx
	mov	qword ptr [rax + 8], rcx
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB5_1:
	lea	rdi, [rip + SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0)]
	lea	rsi, [rip + l_anon.[ID].0]
	lea	rdx, [rip + l_anon.[ID].2]
	mov	r14, rax
	call	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	rbx, rax
	mov	rax, r14
	mov	rcx, qword ptr [rip + SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0)]
	test	rcx, rcx
	jne	LBB5_4
LBB5_3:
	lea	rdi, [rip + SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0)]
	lea	rsi, [rip + l_anon.[ID].0]
	lea	rdx, [rip + l_anon.[ID].2]
	mov	r14, rax
	call	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	mov	rcx, rax
	mov	rax, r14
	jmp	LBB5_4

	.globl	_use_in_loop
	.p2align	4, 0x90
_use_in_loop:
	test	rdi, rdi
	je	LBB6_6
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r12
	push	rbx
	mov	rbx, rdi
	lea	r14, [rip + SYM(test_dynamic_class[CRATE_ID]::use_in_loop::CACHED_CLASS, 0)]
	lea	r15, [rip + l_anon.[ID].10]
	lea	r12, [rip + l_anon.[ID].11]
	jmp	LBB6_2
	.p2align	4, 0x90
LBB6_4:
	dec	rbx
	je	LBB6_5
LBB6_2:
	mov	rax, qword ptr [rip + SYM(test_dynamic_class[CRATE_ID]::use_in_loop::CACHED_CLASS, 0)]
	test	rax, rax
	jne	LBB6_4
	mov	rdi, r14
	mov	rsi, r15
	mov	rdx, r12
	call	SYM(objc2::__macro_helpers::cache::CachedClass::fetch::GENERATED_ID, 0)
	jmp	LBB6_4
LBB6_5:
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	pop	rbp
LBB6_6:
	ret

	.section	__TEXT,__const
l_anon.[ID].0:
	.asciz	"NSObject"

l_anon.[ID].1:
	.ascii	"crates/$DIR/../test_static_class/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].2:
	.quad	l_anon.[ID].1
	.asciz	"J\000\000\000\000\000\000\000\007\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.[ID].3:
	.quad	l_anon.[ID].1
	.asciz	"J\000\000\000\000\000\000\000\f\000\000\000\005\000\000"

	.section	__TEXT,__const
l_anon.[ID].4:
	.asciz	"NSString"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].5:
	.quad	l_anon.[ID].1
	.asciz	"J\000\000\000\000\000\000\000\021\000\000\000\005\000\000"

	.section	__TEXT,__const
l_anon.[ID].6:
	.asciz	"NSData"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].7:
	.quad	l_anon.[ID].1
	.asciz	"J\000\000\000\000\000\000\000\026\000\000\000\r\000\000"

	.section	__TEXT,__const
l_anon.[ID].8:
	.asciz	"NSException"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].9:
	.quad	l_anon.[ID].1
	.asciz	"J\000\000\000\000\000\000\000\036\000\000\000\016\000\000"

	.section	__TEXT,__const
l_anon.[ID].10:
	.asciz	"NSLock"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.[ID].11:
	.quad	l_anon.[ID].1
	.asciz	"J\000\000\000\000\000\000\000,\000\000\000\021\000\000"

.zerofill __DATA,__bss,SYM(test_dynamic_class[CRATE_ID]::get_class::CACHED_CLASS, 0),8,3
.zerofill __DATA,__bss,SYM(test_dynamic_class[CRATE_ID]::get_same_class::CACHED_CLASS, 0),8,3
.zerofill __DATA,__bss,SYM(test_dynamic_class[CRATE_ID]::get_different_class::CACHED_CLASS, 0),8,3
.zerofill __DATA,__bss,SYM(test_dynamic_class[CRATE_ID]::unused_class::CACHED_CLASS, 0),8,3
.zerofill __DATA,__bss,SYM(test_dynamic_class[CRATE_ID]::use_fns::CACHED_CLASS, 0),8,3
.zerofill __DATA,__bss,SYM(test_dynamic_class[CRATE_ID]::use_in_loop::CACHED_CLASS, 0),8,3
.subsections_via_symbols
