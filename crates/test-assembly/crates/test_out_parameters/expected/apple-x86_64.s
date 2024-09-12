	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_nonnull_nonnull
	.p2align	4, 0x90
_nonnull_nonnull:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	mov	rbx, rdx
	mov	r14, qword ptr [rdx]
	call	_objc_msgSend
	mov	r15, rax
	mov	rdi, qword ptr [rbx]
	call	_objc_retain
	mov	rdi, r14
	call	_objc_release
	mov	rax, r15
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret

	.globl	_null_nonnull
	.p2align	4, 0x90
_null_nonnull:
	test	rdx, rdx
	je	LBB1_2
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	mov	rbx, qword ptr [rdx]
	mov	r14, rdx
	call	_objc_msgSend
	mov	r15, rax
	mov	rdi, qword ptr [r14]
	call	_objc_retain
	mov	rdi, rbx
	call	_objc_release
	mov	rax, r15
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB1_2:
	xor	edx, edx
	jmp	_objc_msgSend

	.globl	_nonnull_null
	.p2align	4, 0x90
_nonnull_null:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	mov	r15, rdx
	mov	rbx, qword ptr [rdx]
	call	_objc_msgSend
	mov	r14, rax
	mov	rdi, qword ptr [r15]
	call	_objc_retain
	test	rbx, rbx
	je	LBB2_2
	mov	rdi, rbx
	call	_objc_release
LBB2_2:
	mov	rax, r14
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret

	.globl	_null_null
	.p2align	4, 0x90
_null_null:
	test	rdx, rdx
	je	LBB3_4
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	mov	rbx, qword ptr [rdx]
	mov	r15, rdx
	call	_objc_msgSend
	mov	r14, rax
	mov	rdi, qword ptr [r15]
	call	_objc_retain
	test	rbx, rbx
	je	LBB3_3
	mov	rdi, rbx
	call	_objc_release
LBB3_3:
	mov	rax, r14
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB3_4:
	xor	edx, edx
	jmp	_objc_msgSend

	.globl	_two_nonnull_nonnull
	.p2align	4, 0x90
_two_nonnull_nonnull:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	push	rax
	mov	rbx, rcx
	mov	r14, rdx
	mov	r15, qword ptr [rdx]
	mov	r12, qword ptr [rcx]
	call	_objc_msgSend
	mov	r13, rax
	mov	rdi, qword ptr [r14]
	call	_objc_retain
	mov	rdi, r15
	call	_objc_release
	mov	rdi, qword ptr [rbx]
	call	_objc_retain
	mov	rdi, r12
	call	_objc_release
	mov	rax, r13
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	ret

	.globl	_call_with_none1
	.p2align	4, 0x90
_call_with_none1:
	push	rbp
	mov	rbp, rsp
	xor	edx, edx
	pop	rbp
	jmp	_objc_msgSend

	.globl	_call_with_none2
	.p2align	4, 0x90
_call_with_none2:
	push	rbp
	mov	rbp, rsp
	xor	edx, edx
	pop	rbp
	jmp	_objc_msgSend

	.globl	_call_with_none3
	.p2align	4, 0x90
_call_with_none3:
Lfunc_begin0:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	qword ptr [rbp - 16], 0
Ltmp0:
	lea	rdx, [rbp - 16]
	call	_objc_msgSend
Ltmp1:
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 16]
Ltmp2:
	call	_objc_retain
Ltmp3:
	mov	rdx, qword ptr [rbp - 16]
	mov	rax, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret
LBB7_3:
Ltmp4:
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 16]
	test	rdi, rdi
	je	LBB7_5
Ltmp5:
	call	_objc_release
Ltmp6:
LBB7_5:
	mov	rdi, rbx
	call	__Unwind_Resume
LBB7_6:
Ltmp7:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table7:
Lexception0:
	.byte	255
	.byte	155
	.uleb128 Lttbase0-Lttbaseref0
Lttbaseref0:
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp3-Ltmp0
	.uleb128 Ltmp4-Lfunc_begin0
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
	.globl	_call_with_none4
	.p2align	4, 0x90
_call_with_none4:
Lfunc_begin1:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	qword ptr [rbp - 16], 0
Ltmp8:
	lea	rdx, [rbp - 16]
	call	_objc_msgSend
Ltmp9:
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 16]
Ltmp10:
	call	_objc_retain
Ltmp11:
	mov	rdx, qword ptr [rbp - 16]
	mov	rax, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret
LBB8_3:
Ltmp12:
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 16]
	test	rdi, rdi
	je	LBB8_5
Ltmp13:
	call	_objc_release
Ltmp14:
LBB8_5:
	mov	rdi, rbx
	call	__Unwind_Resume
LBB8_6:
Ltmp15:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end1:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table8:
Lexception1:
	.byte	255
	.byte	155
	.uleb128 Lttbase1-Lttbaseref1
Lttbaseref1:
	.byte	1
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
	.uleb128 Ltmp8-Lfunc_begin1
	.uleb128 Ltmp11-Ltmp8
	.uleb128 Ltmp12-Lfunc_begin1
	.byte	0
	.uleb128 Ltmp13-Lfunc_begin1
	.uleb128 Ltmp14-Ltmp13
	.uleb128 Ltmp15-Lfunc_begin1
	.byte	1
	.uleb128 Ltmp14-Lfunc_begin1
	.uleb128 Lfunc_end1-Ltmp14
	.byte	0
	.byte	0
Lcst_end1:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase1:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_call_with_some1
	.p2align	4, 0x90
_call_with_some1:
Lfunc_begin2:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rdx
	mov	qword ptr [rbp - 24], rdx
Ltmp16:
	lea	rdx, [rbp - 24]
	call	_objc_msgSend
Ltmp17:
	mov	r14, rax
	mov	rdi, qword ptr [rbp - 24]
Ltmp18:
	call	_objc_retain
Ltmp19:
Ltmp20:
	mov	rdi, rbx
	call	_objc_release
Ltmp21:
	mov	rdx, qword ptr [rbp - 24]
	mov	rax, r14
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB9_5:
Ltmp22:
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 24]
Ltmp23:
	call	_objc_release
Ltmp24:
	mov	rdi, rbx
	call	__Unwind_Resume
LBB9_4:
Ltmp25:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end2:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table9:
Lexception2:
	.byte	255
	.byte	155
	.uleb128 Lttbase2-Lttbaseref2
Lttbaseref2:
	.byte	1
	.uleb128 Lcst_end2-Lcst_begin2
Lcst_begin2:
	.uleb128 Ltmp16-Lfunc_begin2
	.uleb128 Ltmp21-Ltmp16
	.uleb128 Ltmp22-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp23-Lfunc_begin2
	.uleb128 Ltmp24-Ltmp23
	.uleb128 Ltmp25-Lfunc_begin2
	.byte	1
	.uleb128 Ltmp24-Lfunc_begin2
	.uleb128 Lfunc_end2-Ltmp24
	.byte	0
	.byte	0
Lcst_end2:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase2:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_call_with_some2
	.p2align	4, 0x90
_call_with_some2:
Lfunc_begin3:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rdx
	mov	qword ptr [rbp - 24], rdx
Ltmp26:
	lea	rdx, [rbp - 24]
	call	_objc_msgSend
Ltmp27:
	mov	r14, rax
	mov	rdi, qword ptr [rbp - 24]
Ltmp28:
	call	_objc_retain
Ltmp29:
Ltmp30:
	mov	rdi, rbx
	call	_objc_release
Ltmp31:
	mov	rdx, qword ptr [rbp - 24]
	mov	rax, r14
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB10_4:
Ltmp32:
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 24]
	test	rdi, rdi
	je	LBB10_6
Ltmp33:
	call	_objc_release
Ltmp34:
LBB10_6:
	mov	rdi, rbx
	call	__Unwind_Resume
LBB10_7:
Ltmp35:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end3:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table10:
Lexception3:
	.byte	255
	.byte	155
	.uleb128 Lttbase3-Lttbaseref3
Lttbaseref3:
	.byte	1
	.uleb128 Lcst_end3-Lcst_begin3
Lcst_begin3:
	.uleb128 Ltmp26-Lfunc_begin3
	.uleb128 Ltmp31-Ltmp26
	.uleb128 Ltmp32-Lfunc_begin3
	.byte	0
	.uleb128 Ltmp33-Lfunc_begin3
	.uleb128 Ltmp34-Ltmp33
	.uleb128 Ltmp35-Lfunc_begin3
	.byte	1
	.uleb128 Ltmp34-Lfunc_begin3
	.uleb128 Lfunc_end3-Ltmp34
	.byte	0
	.byte	0
Lcst_end3:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase3:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_call_with_some3
	.p2align	4, 0x90
_call_with_some3:
Lfunc_begin4:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rdx
	mov	qword ptr [rbp - 24], rdx
Ltmp36:
	lea	rdx, [rbp - 24]
	call	_objc_msgSend
Ltmp37:
	mov	r14, rax
	mov	rdi, qword ptr [rbp - 24]
Ltmp38:
	call	_objc_retain
Ltmp39:
Ltmp40:
	mov	rdi, rbx
	call	_objc_release
Ltmp41:
	mov	rdx, qword ptr [rbp - 24]
	mov	rax, r14
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB11_4:
Ltmp42:
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 24]
	test	rdi, rdi
	je	LBB11_6
Ltmp43:
	call	_objc_release
Ltmp44:
LBB11_6:
	mov	rdi, rbx
	call	__Unwind_Resume
LBB11_7:
Ltmp45:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end4:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table11:
Lexception4:
	.byte	255
	.byte	155
	.uleb128 Lttbase4-Lttbaseref4
Lttbaseref4:
	.byte	1
	.uleb128 Lcst_end4-Lcst_begin4
Lcst_begin4:
	.uleb128 Ltmp36-Lfunc_begin4
	.uleb128 Ltmp41-Ltmp36
	.uleb128 Ltmp42-Lfunc_begin4
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
Lttbase4:
	.byte	0
	.p2align	2, 0x0

.subsections_via_symbols
