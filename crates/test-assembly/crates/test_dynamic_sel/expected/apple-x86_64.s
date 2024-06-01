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
	push	rbx
	push	rax
	mov	rbx, qword ptr [rip + SYM(objc2::__macro_helpers::common_selectors::alloc_sel::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	mov	rax, qword ptr [rbx]
	test	rax, rax
	je	LBB2_1
	mov	rdx, qword ptr [rbx]
	test	rdx, rdx
	je	LBB2_3
LBB2_4:
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret
LBB2_1:
	mov	rdi, qword ptr [rip + SYM(objc2::__macro_helpers::common_selectors::alloc_sel::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	lea	rsi, [rip + l_anon.[ID].1]
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	rdx, qword ptr [rbx]
	test	rdx, rdx
	jne	LBB2_4
LBB2_3:
	mov	rdi, qword ptr [rip + SYM(objc2::__macro_helpers::common_selectors::alloc_sel::CACHED_SEL::GENERATED_ID, 0)@GOTPCREL]
	lea	rsi, [rip + l_anon.[ID].1]
	mov	rbx, rax
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	rdx, rax
	mov	rax, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret

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
	mov	rax, rdi
	mov	rbx, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_sel::CACHED_SEL, 0)]
	test	rbx, rbx
	je	LBB5_1
	mov	r14, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_same_sel::CACHED_SEL, 0)]
	test	r14, r14
	je	LBB5_3
LBB5_4:
	mov	r15, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_different_sel::CACHED_SEL, 0)]
	test	r15, r15
	je	LBB5_5
LBB5_6:
	mov	rcx, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::use_fns::CACHED_SEL, 0)]
	test	rcx, rcx
	je	LBB5_7
LBB5_8:
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
LBB5_1:
	lea	rdi, [rip + SYM(test_dynamic_sel[CRATE_ID]::get_sel::CACHED_SEL, 0)]
	lea	rsi, [rip + l_anon.[ID].0]
	mov	r14, rax
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	rbx, rax
	mov	rax, r14
	mov	r14, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_same_sel::CACHED_SEL, 0)]
	test	r14, r14
	jne	LBB5_4
LBB5_3:
	lea	rdi, [rip + SYM(test_dynamic_sel[CRATE_ID]::get_same_sel::CACHED_SEL, 0)]
	lea	rsi, [rip + l_anon.[ID].0]
	mov	r15, rax
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r14, rax
	mov	rax, r15
	mov	r15, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_different_sel::CACHED_SEL, 0)]
	test	r15, r15
	jne	LBB5_6
LBB5_5:
	lea	rdi, [rip + SYM(test_dynamic_sel[CRATE_ID]::get_different_sel::CACHED_SEL, 0)]
	lea	rsi, [rip + L_anon.[ID].2]
	mov	r12, rax
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	r15, rax
	mov	rax, r12
	mov	rcx, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::use_fns::CACHED_SEL, 0)]
	test	rcx, rcx
	jne	LBB5_8
LBB5_7:
	lea	rdi, [rip + SYM(test_dynamic_sel[CRATE_ID]::use_fns::CACHED_SEL, 0)]
	lea	rsi, [rip + l_anon.[ID].4]
	mov	r12, rax
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	rcx, rax
	mov	rax, r12
	jmp	LBB5_8

	.globl	_use_same_twice
	.p2align	4, 0x90
_use_same_twice:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	rax, rdi
	mov	rbx, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_sel::CACHED_SEL, 0)]
	test	rbx, rbx
	je	LBB6_1
	mov	rcx, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_sel::CACHED_SEL, 0)]
	test	rcx, rcx
	je	LBB6_3
LBB6_4:
	mov	qword ptr [rax], rbx
	mov	qword ptr [rax + 8], rcx
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB6_1:
	lea	rdi, [rip + SYM(test_dynamic_sel[CRATE_ID]::get_sel::CACHED_SEL, 0)]
	lea	rsi, [rip + l_anon.[ID].0]
	mov	r14, rax
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	rbx, rax
	mov	rax, r14
	mov	rcx, qword ptr [rip + SYM(test_dynamic_sel[CRATE_ID]::get_sel::CACHED_SEL, 0)]
	test	rcx, rcx
	jne	LBB6_4
LBB6_3:
	lea	rdi, [rip + SYM(test_dynamic_sel[CRATE_ID]::get_sel::CACHED_SEL, 0)]
	lea	rsi, [rip + l_anon.[ID].0]
	mov	r14, rax
	call	SYM(objc2::__macro_helpers::cache::CachedSel::fetch::GENERATED_ID, 0)
	mov	rcx, rax
	mov	rax, r14
	jmp	LBB6_4

	.globl	_use_in_loop
	.p2align	4, 0x90
_use_in_loop:
	test	rdi, rdi
	je	LBB7_6
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
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
LBB7_6:
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
