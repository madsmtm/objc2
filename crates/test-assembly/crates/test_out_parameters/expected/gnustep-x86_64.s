	.text
	.intel_syntax noprefix
	.section	.text.nonnull_nonnull,"ax",@progbits
	.globl	nonnull_nonnull
	.p2align	4, 0x90
	.type	nonnull_nonnull,@function
nonnull_nonnull:
	push	r15
	push	r14
	push	r12
	push	rbx
	push	rax
	mov	rbx, rdx
	mov	r14, rsi
	mov	r15, rdi
	mov	r12, qword ptr [rdx]
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r15
	mov	rsi, r14
	mov	rdx, rbx
	call	rax
	mov	r14, rax
	mov	rdi, qword ptr [rbx]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	mov	rdi, r12
	call	qword ptr [rip + objc_release@GOTPCREL]
	mov	rax, r14
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	ret
.Lfunc_end0:
	.size	nonnull_nonnull, .Lfunc_end0-nonnull_nonnull

	.section	.text.null_nonnull,"ax",@progbits
	.globl	null_nonnull
	.p2align	4, 0x90
	.type	null_nonnull,@function
null_nonnull:
	push	r15
	push	r14
	push	r12
	push	rbx
	push	rax
	mov	rbx, rdx
	mov	r12, rsi
	mov	r15, rdi
	test	rdx, rdx
	je	.LBB1_1
	mov	r14, qword ptr [rbx]
	jmp	.LBB1_3
.LBB1_1:
.LBB1_3:
	mov	rdi, r15
	mov	rsi, r12
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r15
	mov	rsi, r12
	mov	rdx, rbx
	call	rax
	test	rbx, rbx
	je	.LBB1_5
	mov	rdi, qword ptr [rbx]
	mov	rbx, rax
	call	qword ptr [rip + objc_retain@GOTPCREL]
	mov	rdi, r14
	call	qword ptr [rip + objc_release@GOTPCREL]
	mov	rax, rbx
.LBB1_5:
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	ret
.Lfunc_end1:
	.size	null_nonnull, .Lfunc_end1-null_nonnull

	.section	.text.nonnull_null,"ax",@progbits
	.globl	nonnull_null
	.p2align	4, 0x90
	.type	nonnull_null,@function
nonnull_null:
	push	r15
	push	r14
	push	r12
	push	rbx
	push	rax
	mov	r15, rdx
	mov	r14, rsi
	mov	r12, rdi
	mov	rbx, qword ptr [rdx]
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r12
	mov	rsi, r14
	mov	rdx, r15
	call	rax
	mov	r14, rax
	mov	rdi, qword ptr [r15]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	test	rbx, rbx
	je	.LBB2_2
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
.LBB2_2:
	mov	rax, r14
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	ret
.Lfunc_end2:
	.size	nonnull_null, .Lfunc_end2-nonnull_null

	.section	.text.null_null,"ax",@progbits
	.globl	null_null
	.p2align	4, 0x90
	.type	null_null,@function
null_null:
	push	r15
	push	r14
	push	r12
	push	rbx
	push	rax
	mov	r14, rdx
	mov	r12, rsi
	mov	r15, rdi
	test	rdx, rdx
	je	.LBB3_1
	mov	rbx, qword ptr [r14]
	jmp	.LBB3_3
.LBB3_1:
.LBB3_3:
	mov	rdi, r15
	mov	rsi, r12
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r15
	mov	rsi, r12
	mov	rdx, r14
	call	rax
	mov	r15, rax
	test	r14, r14
	je	.LBB3_6
	mov	rdi, qword ptr [r14]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	test	rbx, rbx
	je	.LBB3_6
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
.LBB3_6:
	mov	rax, r15
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	ret
.Lfunc_end3:
	.size	null_null, .Lfunc_end3-null_null

	.section	.text.two_nonnull_nonnull,"ax",@progbits
	.globl	two_nonnull_nonnull
	.p2align	4, 0x90
	.type	two_nonnull_nonnull,@function
two_nonnull_nonnull:
	push	rbp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	push	rax
	mov	r14, rcx
	mov	r15, rdx
	mov	r12, rsi
	mov	r13, rdi
	mov	rbp, qword ptr [rdx]
	mov	rbx, qword ptr [rcx]
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r13
	mov	rsi, r12
	mov	rdx, r15
	mov	rcx, r14
	call	rax
	mov	r12, rax
	mov	rdi, qword ptr [r15]
	mov	r15, qword ptr [rip + objc_retain@GOTPCREL]
	call	r15
	mov	r13, qword ptr [rip + objc_release@GOTPCREL]
	mov	rdi, rbp
	call	r13
	mov	rdi, qword ptr [r14]
	call	r15
	mov	rdi, rbx
	call	r13
	mov	rax, r12
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	ret
.Lfunc_end4:
	.size	two_nonnull_nonnull, .Lfunc_end4-two_nonnull_nonnull

	.section	.text.call_with_none1,"ax",@progbits
	.globl	call_with_none1
	.p2align	4, 0x90
	.type	call_with_none1,@function
call_with_none1:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r14
	mov	rsi, rbx
	xor	edx, edx
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	rax
.Lfunc_end5:
	.size	call_with_none1, .Lfunc_end5-call_with_none1

	.section	.text.call_with_none2,"ax",@progbits
	.globl	call_with_none2
	.p2align	4, 0x90
	.type	call_with_none2,@function
call_with_none2:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r14
	mov	rsi, rbx
	xor	edx, edx
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	rax
.Lfunc_end6:
	.size	call_with_none2, .Lfunc_end6-call_with_none2

	.section	.text.call_with_none3,"ax",@progbits
	.globl	call_with_none3
	.p2align	4, 0x90
	.type	call_with_none3,@function
call_with_none3:
.Lfunc_begin0:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	mov	qword ptr [rsp], 0
.Ltmp0:
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
.Ltmp1:
.Ltmp2:
	mov	rdx, rsp
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
.Ltmp3:
	mov	rbx, rax
	mov	rdi, qword ptr [rsp]
.Ltmp4:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp5:
	mov	rdx, qword ptr [rsp]
	mov	rax, rbx
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB7_4:
.Ltmp6:
	mov	rbx, rax
	mov	rdi, qword ptr [rsp]
	test	rdi, rdi
	je	.LBB7_6
.Ltmp7:
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp8:
.LBB7_6:
	mov	rdi, rbx
	call	_Unwind_Resume@PLT
.LBB7_7:
.Ltmp9:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end7:
	.size	call_with_none3, .Lfunc_end7-call_with_none3
	.section	.gcc_except_table.call_with_none3,"a",@progbits
	.p2align	2, 0x0
GCC_except_table7:
.Lexception0:
	.byte	255
	.byte	155
	.uleb128 .Lttbase0-.Lttbaseref0
.Lttbaseref0:
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Ltmp0-.Lfunc_begin0
	.uleb128 .Ltmp5-.Ltmp0
	.uleb128 .Ltmp6-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp7-.Lfunc_begin0
	.uleb128 .Ltmp8-.Ltmp7
	.uleb128 .Ltmp9-.Lfunc_begin0
	.byte	1
	.uleb128 .Ltmp8-.Lfunc_begin0
	.uleb128 .Lfunc_end7-.Ltmp8
	.byte	0
	.byte	0
.Lcst_end0:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.section	.text.call_with_none4,"ax",@progbits
	.globl	call_with_none4
	.p2align	4, 0x90
	.type	call_with_none4,@function
call_with_none4:
.Lfunc_begin1:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	mov	qword ptr [rsp], 0
.Ltmp10:
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
.Ltmp11:
.Ltmp12:
	mov	rdx, rsp
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
.Ltmp13:
	mov	rbx, rax
	mov	rdi, qword ptr [rsp]
.Ltmp14:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp15:
	mov	rdx, qword ptr [rsp]
	mov	rax, rbx
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB8_4:
.Ltmp16:
	mov	rbx, rax
	mov	rdi, qword ptr [rsp]
	test	rdi, rdi
	je	.LBB8_6
.Ltmp17:
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp18:
.LBB8_6:
	mov	rdi, rbx
	call	_Unwind_Resume@PLT
.LBB8_7:
.Ltmp19:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end8:
	.size	call_with_none4, .Lfunc_end8-call_with_none4
	.section	.gcc_except_table.call_with_none4,"a",@progbits
	.p2align	2, 0x0
GCC_except_table8:
.Lexception1:
	.byte	255
	.byte	155
	.uleb128 .Lttbase1-.Lttbaseref1
.Lttbaseref1:
	.byte	1
	.uleb128 .Lcst_end1-.Lcst_begin1
.Lcst_begin1:
	.uleb128 .Ltmp10-.Lfunc_begin1
	.uleb128 .Ltmp15-.Ltmp10
	.uleb128 .Ltmp16-.Lfunc_begin1
	.byte	0
	.uleb128 .Ltmp17-.Lfunc_begin1
	.uleb128 .Ltmp18-.Ltmp17
	.uleb128 .Ltmp19-.Lfunc_begin1
	.byte	1
	.uleb128 .Ltmp18-.Lfunc_begin1
	.uleb128 .Lfunc_end8-.Ltmp18
	.byte	0
	.byte	0
.Lcst_end1:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase1:
	.byte	0
	.p2align	2, 0x0

	.section	.text.call_with_some1,"ax",@progbits
	.globl	call_with_some1
	.p2align	4, 0x90
	.type	call_with_some1,@function
call_with_some1:
.Lfunc_begin2:
	push	r15
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rdx
	mov	r14, rsi
	mov	r15, rdi
	mov	qword ptr [rsp + 8], rdx
.Ltmp20:
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
.Ltmp21:
.Ltmp22:
	lea	rdx, [rsp + 8]
	mov	rdi, r15
	mov	rsi, r14
	call	rax
.Ltmp23:
	mov	r14, rax
	mov	rdi, qword ptr [rsp + 8]
.Ltmp24:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp25:
.Ltmp26:
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp27:
	mov	rdx, qword ptr [rsp + 8]
	mov	rax, r14
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	r15
	ret
.LBB9_6:
.Ltmp28:
	mov	rbx, rax
	mov	rdi, qword ptr [rsp + 8]
.Ltmp29:
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp30:
	mov	rdi, rbx
	call	_Unwind_Resume@PLT
.LBB9_5:
.Ltmp31:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end9:
	.size	call_with_some1, .Lfunc_end9-call_with_some1
	.section	.gcc_except_table.call_with_some1,"a",@progbits
	.p2align	2, 0x0
GCC_except_table9:
.Lexception2:
	.byte	255
	.byte	155
	.uleb128 .Lttbase2-.Lttbaseref2
.Lttbaseref2:
	.byte	1
	.uleb128 .Lcst_end2-.Lcst_begin2
.Lcst_begin2:
	.uleb128 .Ltmp20-.Lfunc_begin2
	.uleb128 .Ltmp27-.Ltmp20
	.uleb128 .Ltmp28-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp29-.Lfunc_begin2
	.uleb128 .Ltmp30-.Ltmp29
	.uleb128 .Ltmp31-.Lfunc_begin2
	.byte	1
	.uleb128 .Ltmp30-.Lfunc_begin2
	.uleb128 .Lfunc_end9-.Ltmp30
	.byte	0
	.byte	0
.Lcst_end2:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase2:
	.byte	0
	.p2align	2, 0x0

	.section	.text.call_with_some2,"ax",@progbits
	.globl	call_with_some2
	.p2align	4, 0x90
	.type	call_with_some2,@function
call_with_some2:
.Lfunc_begin3:
	push	r15
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rdx
	mov	r14, rsi
	mov	r15, rdi
	mov	qword ptr [rsp + 8], rdx
.Ltmp32:
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
.Ltmp33:
.Ltmp34:
	lea	rdx, [rsp + 8]
	mov	rdi, r15
	mov	rsi, r14
	call	rax
.Ltmp35:
	mov	r14, rax
	mov	rdi, qword ptr [rsp + 8]
.Ltmp36:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp37:
.Ltmp38:
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp39:
	mov	rdx, qword ptr [rsp + 8]
	mov	rax, r14
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	r15
	ret
.LBB10_5:
.Ltmp40:
	mov	rbx, rax
	mov	rdi, qword ptr [rsp + 8]
	test	rdi, rdi
	je	.LBB10_7
.Ltmp41:
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp42:
.LBB10_7:
	mov	rdi, rbx
	call	_Unwind_Resume@PLT
.LBB10_8:
.Ltmp43:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end10:
	.size	call_with_some2, .Lfunc_end10-call_with_some2
	.section	.gcc_except_table.call_with_some2,"a",@progbits
	.p2align	2, 0x0
GCC_except_table10:
.Lexception3:
	.byte	255
	.byte	155
	.uleb128 .Lttbase3-.Lttbaseref3
.Lttbaseref3:
	.byte	1
	.uleb128 .Lcst_end3-.Lcst_begin3
.Lcst_begin3:
	.uleb128 .Ltmp32-.Lfunc_begin3
	.uleb128 .Ltmp39-.Ltmp32
	.uleb128 .Ltmp40-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp41-.Lfunc_begin3
	.uleb128 .Ltmp42-.Ltmp41
	.uleb128 .Ltmp43-.Lfunc_begin3
	.byte	1
	.uleb128 .Ltmp42-.Lfunc_begin3
	.uleb128 .Lfunc_end10-.Ltmp42
	.byte	0
	.byte	0
.Lcst_end3:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase3:
	.byte	0
	.p2align	2, 0x0

	.section	.text.call_with_some3,"ax",@progbits
	.globl	call_with_some3
	.p2align	4, 0x90
	.type	call_with_some3,@function
call_with_some3:
.Lfunc_begin4:
	push	r15
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rdx
	mov	r14, rsi
	mov	r15, rdi
	mov	qword ptr [rsp + 8], rdx
.Ltmp44:
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
.Ltmp45:
.Ltmp46:
	lea	rdx, [rsp + 8]
	mov	rdi, r15
	mov	rsi, r14
	call	rax
.Ltmp47:
	mov	r14, rax
	mov	rdi, qword ptr [rsp + 8]
.Ltmp48:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp49:
.Ltmp50:
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp51:
	mov	rdx, qword ptr [rsp + 8]
	mov	rax, r14
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	r15
	ret
.LBB11_5:
.Ltmp52:
	mov	rbx, rax
	mov	rdi, qword ptr [rsp + 8]
	test	rdi, rdi
	je	.LBB11_7
.Ltmp53:
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp54:
.LBB11_7:
	mov	rdi, rbx
	call	_Unwind_Resume@PLT
.LBB11_8:
.Ltmp55:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end11:
	.size	call_with_some3, .Lfunc_end11-call_with_some3
	.section	.gcc_except_table.call_with_some3,"a",@progbits
	.p2align	2, 0x0
GCC_except_table11:
.Lexception4:
	.byte	255
	.byte	155
	.uleb128 .Lttbase4-.Lttbaseref4
.Lttbaseref4:
	.byte	1
	.uleb128 .Lcst_end4-.Lcst_begin4
.Lcst_begin4:
	.uleb128 .Ltmp44-.Lfunc_begin4
	.uleb128 .Ltmp51-.Ltmp44
	.uleb128 .Ltmp52-.Lfunc_begin4
	.byte	0
	.uleb128 .Ltmp53-.Lfunc_begin4
	.uleb128 .Ltmp54-.Ltmp53
	.uleb128 .Ltmp55-.Lfunc_begin4
	.byte	1
	.uleb128 .Ltmp54-.Lfunc_begin4
	.uleb128 .Lfunc_end11-.Ltmp54
	.byte	0
	.byte	0
.Lcst_end4:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase4:
	.byte	0
	.p2align	2, 0x0

	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"awG",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3, 0x0
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality
	.section	".note.GNU-stack","",@progbits
