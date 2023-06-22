	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_get_sel
	.p2align	4, 0x90
_get_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_sel::CACHED_SEL, 0)]
	test	rax, rax
	je	LBB0_2
	pop	rbp
	ret
LBB0_2:
	lea	rdi, [rip + SYM(test_dynamic_sel[CRATE_ID]::get_sel::CACHED_SEL, 0)]
	lea	rsi, [rip + l_anon.[ID].0]
	pop	rbp
	jmp	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)

	.globl	_get_same_sel
	.p2align	4, 0x90
_get_same_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_same_sel::CACHED_SEL, 0)]
	test	rax, rax
	je	LBB1_2
	pop	rbp
	ret
LBB1_2:
	lea	rdi, [rip + SYM(test_dynamic_sel[CRATE_ID]::get_same_sel::CACHED_SEL, 0)]
	lea	rsi, [rip + l_anon.[ID].0]
	pop	rbp
	jmp	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)

	.globl	_get_common_twice
	.p2align	4, 0x90
_get_common_twice:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	r14, qword ptr [rip + SYM(objc2::__macro_helpers::common_selectors::alloc_sel::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	mov	rbx, qword ptr [r14]
	test	rbx, rbx
	je	LBB2_1
	mov	rdx, qword ptr [r14]
	test	rdx, rdx
	je	LBB2_3
LBB2_4:
	mov	rax, rbx
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB2_1:
	mov	rdi, qword ptr [rip + SYM(objc2::__macro_helpers::common_selectors::alloc_sel::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	lea	rsi, [rip + l_anon.[ID].1]
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	rbx, rax
	mov	rdx, qword ptr [r14]
	test	rdx, rdx
	jne	LBB2_4
LBB2_3:
	mov	rdi, qword ptr [rip + SYM(objc2::__macro_helpers::common_selectors::alloc_sel::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	lea	rsi, [rip + l_anon.[ID].1]
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	rdx, rax
	jmp	LBB2_4

	.globl	_get_different_sel
	.p2align	4, 0x90
_get_different_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_different_sel::CACHED_SEL, 0)]
	test	rax, rax
	je	LBB3_2
	pop	rbp
	ret
LBB3_2:
	lea	rdi, [rip + SYM(test_dynamic_sel[CRATE_ID]::get_different_sel::CACHED_SEL, 0)]
	lea	rsi, [rip + L_anon.[ID].2]
	pop	rbp
	jmp	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)

	.globl	_unused_sel
	.p2align	4, 0x90
_unused_sel:
	push	rbp
	mov	rbp, rsp
	mov	rax, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::unused_sel::CACHED_SEL, 0)]
	test	rax, rax
	je	LBB4_2
	pop	rbp
	ret
LBB4_2:
	lea	rdi, [rip + SYM(test_dynamic_sel[CRATE_ID]::unused_sel::CACHED_SEL, 0)]
	lea	rsi, [rip + l_anon.[ID].3]
	pop	rbp
	jmp	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)

	.globl	_use_fns
	.p2align	4, 0x90
_use_fns:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r12
	push	rbx
	mov	rbx, rdi
	mov	r14, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_sel::CACHED_SEL, 0)]
	test	r14, r14
	je	LBB5_1
	mov	r15, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_same_sel::CACHED_SEL, 0)]
	test	r15, r15
	je	LBB5_3
LBB5_4:
	mov	r12, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_different_sel::CACHED_SEL, 0)]
	test	r12, r12
	je	LBB5_5
LBB5_6:
	mov	rax, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::use_fns::CACHED_SEL, 0)]
	test	rax, rax
	je	LBB5_7
LBB5_8:
	mov	qword ptr [rbx], r14
	mov	qword ptr [rbx + 8], r15
	mov	qword ptr [rbx + 16], r12
	mov	qword ptr [rbx + 24], rax
	mov	rax, rbx
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB5_1:
	lea	rdi, [rip + SYM(test_dynamic_sel[CRATE_ID]::get_sel::CACHED_SEL, 0)]
	lea	rsi, [rip + l_anon.[ID].0]
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r14, rax
	mov	r15, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_same_sel::CACHED_SEL, 0)]
	test	r15, r15
	jne	LBB5_4
LBB5_3:
	lea	rdi, [rip + SYM(test_dynamic_sel[CRATE_ID]::get_same_sel::CACHED_SEL, 0)]
	lea	rsi, [rip + l_anon.[ID].0]
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r15, rax
	mov	r12, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_different_sel::CACHED_SEL, 0)]
	test	r12, r12
	jne	LBB5_6
LBB5_5:
	lea	rdi, [rip + SYM(test_dynamic_sel[CRATE_ID]::get_different_sel::CACHED_SEL, 0)]
	lea	rsi, [rip + L_anon.[ID].2]
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r12, rax
	mov	rax, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::use_fns::CACHED_SEL, 0)]
	test	rax, rax
	jne	LBB5_8
LBB5_7:
	lea	rdi, [rip + SYM(test_dynamic_sel[CRATE_ID]::use_fns::CACHED_SEL, 0)]
	lea	rsi, [rip + l_anon.[ID].4]
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	jmp	LBB5_8

	.globl	_use_same_twice
	.p2align	4, 0x90
_use_same_twice:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	rbx, rdi
	mov	r14, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_sel::CACHED_SEL, 0)]
	test	r14, r14
	je	LBB6_1
	mov	rax, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_sel::CACHED_SEL, 0)]
	test	rax, rax
	je	LBB6_3
LBB6_4:
	mov	qword ptr [rbx], r14
	mov	qword ptr [rbx + 8], rax
	mov	rax, rbx
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB6_1:
	lea	rdi, [rip + SYM(test_dynamic_sel[CRATE_ID]::get_sel::CACHED_SEL, 0)]
	lea	rsi, [rip + l_anon.[ID].0]
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r14, rax
	mov	rax, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_sel::CACHED_SEL, 0)]
	test	rax, rax
	jne	LBB6_4
LBB6_3:
	lea	rdi, [rip + SYM(test_dynamic_sel[CRATE_ID]::get_sel::CACHED_SEL, 0)]
	lea	rsi, [rip + l_anon.[ID].0]
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	jmp	LBB6_4

	.globl	_use_in_loop
	.p2align	4, 0x90
_use_in_loop:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	test	rdi, rdi
	je	LBB7_5
	mov	rbx, rdi
	lea	r14, [rip + SYM(test_dynamic_sel[CRATE_ID]::use_in_loop::CACHED_SEL, 0)]
	lea	r15, [rip + l_anon.[ID].5]
	jmp	LBB7_2
	.p2align	4, 0x90
LBB7_4:
	dec	rbx
	je	LBB7_5
LBB7_2:
	mov	rax, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::use_in_loop::CACHED_SEL, 0)]
	test	rax, rax
	jne	LBB7_4
	mov	rdi, r14
	mov	rsi, r15
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	jmp	LBB7_4
LBB7_5:
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret

	.section	__TEXT,__const
l_anon.[ID].0:
	.asciz	"simple"

l_anon.[ID].1:
	.asciz	"alloc"

	.section	__TEXT,__literal16,16byte_literals
L_anon.[ID].2:
	.asciz	"i:am:different:"

	.section	__TEXT,__const
l_anon.[ID].3:
	.asciz	"unused"

l_anon.[ID].4:
	.asciz	"fourthSel"

l_anon.[ID].5:
	.asciz	"loopedSelector"

.zerofill __DATA,__bss,SYM(test_dynamic_sel[CRATE_ID]::get_sel::CACHED_SEL, 0),8,3
.zerofill __DATA,__bss,SYM(test_dynamic_sel[CRATE_ID]::get_same_sel::CACHED_SEL, 0),8,3
.zerofill __DATA,__bss,SYM(test_dynamic_sel[CRATE_ID]::get_different_sel::CACHED_SEL, 0),8,3
.zerofill __DATA,__bss,SYM(test_dynamic_sel[CRATE_ID]::unused_sel::CACHED_SEL, 0),8,3
.zerofill __DATA,__bss,SYM(test_dynamic_sel[CRATE_ID]::use_fns::CACHED_SEL, 0),8,3
.zerofill __DATA,__bss,SYM(test_dynamic_sel[CRATE_ID]::use_in_loop::CACHED_SEL, 0),8,3
.subsections_via_symbols
