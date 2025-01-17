	.text
	.intel_syntax noprefix
	.section	.text.SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0),"ax",@progbits
	.p2align	4, 0x90
	.type	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0),@function
SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0):
	push	rbx
	test	rdi, rdi
	je	.LBB0_1
	mov	rbx, rsi
	mov	rdi, qword ptr [rdi]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	mov	rdi, rbx
	pop	rbx
	jmp	qword ptr [rip + objc_release@GOTPCREL]
.LBB0_1:
	pop	rbx
	ret
.Lfunc_end0:
	.size	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0), .Lfunc_end0-SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0)

	.section	.text.SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0),"ax",@progbits
	.p2align	4, 0x90
	.type	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0),@function
SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0):
	push	rbx
	test	rdi, rdi
	je	.LBB1_2
	mov	rbx, rsi
	mov	rdi, qword ptr [rdi]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	test	rbx, rbx
	je	.LBB1_2
	mov	rdi, rbx
	pop	rbx
	jmp	qword ptr [rip + objc_release@GOTPCREL]
.LBB1_2:
	pop	rbx
	ret
.Lfunc_end1:
	.size	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0), .Lfunc_end1-SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0)

	.section	.text.SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0),"ax",@progbits
	.p2align	4, 0x90
	.type	SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0),@function
SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0):
.Lfunc_begin0:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rdi
	mov	rax, qword ptr [rdi]
	mov	r14, qword ptr [rdi + 8]
	mov	rdi, qword ptr [rax]
.Ltmp0:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp1:
.Ltmp2:
	mov	rdi, r14
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp3:
	mov	rax, qword ptr [rbx + 16]
	mov	rbx, qword ptr [rbx + 24]
	mov	rdi, qword ptr [rax]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	mov	rdi, rbx
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	qword ptr [rip + objc_release@GOTPCREL]
.LBB2_3:
.Ltmp4:
	mov	r14, rax
	mov	rax, qword ptr [rbx + 16]
	mov	rbx, qword ptr [rbx + 24]
	mov	rdi, qword ptr [rax]
.Ltmp5:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp6:
.Ltmp7:
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp8:
	mov	rdi, r14
	call	_Unwind_Resume@PLT
.LBB2_6:
.Ltmp9:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end2:
	.size	SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0), .Lfunc_end2-SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0)
	.section	.gcc_except_table.SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0),"a",@progbits
	.p2align	2, 0x0
GCC_except_table2:
.Lexception0:
	.byte	255
	.byte	155
	.uleb128 .Lttbase0-.Lttbaseref0
.Lttbaseref0:
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Ltmp0-.Lfunc_begin0
	.uleb128 .Ltmp3-.Ltmp0
	.uleb128 .Ltmp4-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp3-.Lfunc_begin0
	.uleb128 .Ltmp5-.Ltmp3
	.byte	0
	.byte	0
	.uleb128 .Ltmp5-.Lfunc_begin0
	.uleb128 .Ltmp8-.Ltmp5
	.uleb128 .Ltmp9-.Lfunc_begin0
	.byte	1
	.uleb128 .Ltmp8-.Lfunc_begin0
	.uleb128 .Lfunc_end2-.Ltmp8
	.byte	0
	.byte	0
.Lcst_end0:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.section	.text.SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>,)>, 0),"ax",@progbits
	.p2align	4, 0x90
	.type	SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>,)>, 0),@function
SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>,)>, 0):
	push	rbx
	mov	rbx, rsi
	mov	rdi, qword ptr [rdi]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	test	rbx, rbx
	je	.LBB3_1
	mov	rdi, rbx
	pop	rbx
	jmp	qword ptr [rip + objc_release@GOTPCREL]
.LBB3_1:
	pop	rbx
	ret
.Lfunc_end3:
	.size	SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>,)>, 0), .Lfunc_end3-SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>,)>, 0)

	.section	.text.nonnull_nonnull,"ax",@progbits
	.globl	nonnull_nonnull
	.p2align	4, 0x90
	.type	nonnull_nonnull,@function
nonnull_nonnull:
.Lfunc_begin1:
	push	r15
	push	r14
	push	r12
	push	rbx
	push	rax
	mov	r14, rdx
	mov	r15, rsi
	mov	r12, rdi
	mov	rbx, qword ptr [rdx]
.Ltmp10:
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
.Ltmp11:
.Ltmp12:
	mov	rdi, r12
	mov	rsi, r15
	mov	rdx, r14
	call	rax
.Ltmp13:
	mov	r15, rax
	mov	rdi, qword ptr [r14]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
	mov	rax, r15
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	ret
.LBB4_3:
.Ltmp14:
	mov	r15, rax
	mov	rdi, qword ptr [r14]
.Ltmp15:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp16:
.Ltmp17:
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp18:
	mov	rdi, r15
	call	_Unwind_Resume@PLT
.LBB4_6:
.Ltmp19:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end4:
	.size	nonnull_nonnull, .Lfunc_end4-nonnull_nonnull
	.section	.gcc_except_table.nonnull_nonnull,"a",@progbits
	.p2align	2, 0x0
GCC_except_table4:
.Lexception1:
	.byte	255
	.byte	155
	.uleb128 .Lttbase1-.Lttbaseref1
.Lttbaseref1:
	.byte	1
	.uleb128 .Lcst_end1-.Lcst_begin1
.Lcst_begin1:
	.uleb128 .Ltmp10-.Lfunc_begin1
	.uleb128 .Ltmp13-.Ltmp10
	.uleb128 .Ltmp14-.Lfunc_begin1
	.byte	0
	.uleb128 .Ltmp13-.Lfunc_begin1
	.uleb128 .Ltmp15-.Ltmp13
	.byte	0
	.byte	0
	.uleb128 .Ltmp15-.Lfunc_begin1
	.uleb128 .Ltmp18-.Ltmp15
	.uleb128 .Ltmp19-.Lfunc_begin1
	.byte	1
	.uleb128 .Ltmp18-.Lfunc_begin1
	.uleb128 .Lfunc_end4-.Ltmp18
	.byte	0
	.byte	0
.Lcst_end1:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase1:
	.byte	0
	.p2align	2, 0x0

	.section	.text.null_nonnull,"ax",@progbits
	.globl	null_nonnull
	.p2align	4, 0x90
	.type	null_nonnull,@function
null_nonnull:
.Lfunc_begin2:
	push	r15
	push	r14
	push	r12
	push	rbx
	push	rax
	mov	rbx, rdx
	mov	r15, rsi
	mov	r12, rdi
	test	rdx, rdx
	je	.LBB5_1
	mov	r14, qword ptr [rbx]
	jmp	.LBB5_3
.LBB5_1:
.LBB5_3:
.Ltmp20:
	mov	rdi, r12
	mov	rsi, r15
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
.Ltmp21:
.Ltmp22:
	mov	rdi, r12
	mov	rsi, r15
	mov	rdx, rbx
	call	rax
.Ltmp23:
	test	rbx, rbx
	je	.LBB5_7
	mov	rdi, qword ptr [rbx]
	mov	rbx, rax
	call	qword ptr [rip + objc_retain@GOTPCREL]
	mov	rdi, r14
	call	qword ptr [rip + objc_release@GOTPCREL]
	mov	rax, rbx
.LBB5_7:
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	ret
.LBB5_9:
.Ltmp24:
	mov	r15, rax
.Ltmp25:
	mov	rdi, rbx
	mov	rsi, r14
	call	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0)
.Ltmp26:
	mov	rdi, r15
	call	_Unwind_Resume@PLT
.LBB5_8:
.Ltmp27:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end5:
	.size	null_nonnull, .Lfunc_end5-null_nonnull
	.section	.gcc_except_table.null_nonnull,"a",@progbits
	.p2align	2, 0x0
GCC_except_table5:
.Lexception2:
	.byte	255
	.byte	155
	.uleb128 .Lttbase2-.Lttbaseref2
.Lttbaseref2:
	.byte	1
	.uleb128 .Lcst_end2-.Lcst_begin2
.Lcst_begin2:
	.uleb128 .Ltmp20-.Lfunc_begin2
	.uleb128 .Ltmp23-.Ltmp20
	.uleb128 .Ltmp24-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp23-.Lfunc_begin2
	.uleb128 .Ltmp25-.Ltmp23
	.byte	0
	.byte	0
	.uleb128 .Ltmp25-.Lfunc_begin2
	.uleb128 .Ltmp26-.Ltmp25
	.uleb128 .Ltmp27-.Lfunc_begin2
	.byte	1
	.uleb128 .Ltmp26-.Lfunc_begin2
	.uleb128 .Lfunc_end5-.Ltmp26
	.byte	0
	.byte	0
.Lcst_end2:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase2:
	.byte	0
	.p2align	2, 0x0

	.section	.text.nonnull_null,"ax",@progbits
	.globl	nonnull_null
	.p2align	4, 0x90
	.type	nonnull_null,@function
nonnull_null:
.Lfunc_begin3:
	push	r15
	push	r14
	push	r12
	push	rbx
	push	rax
	mov	r14, rdx
	mov	r15, rsi
	mov	r12, rdi
	mov	rbx, qword ptr [rdx]
.Ltmp28:
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
.Ltmp29:
.Ltmp30:
	mov	rdi, r12
	mov	rsi, r15
	mov	rdx, r14
	call	rax
.Ltmp31:
	mov	r15, rax
	mov	rdi, qword ptr [r14]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	test	rbx, rbx
	je	.LBB6_4
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
.LBB6_4:
	mov	rax, r15
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	ret
.LBB6_6:
.Ltmp32:
	mov	r15, rax
.Ltmp33:
	mov	rdi, r14
	mov	rsi, rbx
	call	SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>,)>, 0)
.Ltmp34:
	mov	rdi, r15
	call	_Unwind_Resume@PLT
.LBB6_5:
.Ltmp35:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end6:
	.size	nonnull_null, .Lfunc_end6-nonnull_null
	.section	.gcc_except_table.nonnull_null,"a",@progbits
	.p2align	2, 0x0
GCC_except_table6:
.Lexception3:
	.byte	255
	.byte	155
	.uleb128 .Lttbase3-.Lttbaseref3
.Lttbaseref3:
	.byte	1
	.uleb128 .Lcst_end3-.Lcst_begin3
.Lcst_begin3:
	.uleb128 .Ltmp28-.Lfunc_begin3
	.uleb128 .Ltmp31-.Ltmp28
	.uleb128 .Ltmp32-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp31-.Lfunc_begin3
	.uleb128 .Ltmp33-.Ltmp31
	.byte	0
	.byte	0
	.uleb128 .Ltmp33-.Lfunc_begin3
	.uleb128 .Ltmp34-.Ltmp33
	.uleb128 .Ltmp35-.Lfunc_begin3
	.byte	1
	.uleb128 .Ltmp34-.Lfunc_begin3
	.uleb128 .Lfunc_end6-.Ltmp34
	.byte	0
	.byte	0
.Lcst_end3:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase3:
	.byte	0
	.p2align	2, 0x0

	.section	.text.null_null,"ax",@progbits
	.globl	null_null
	.p2align	4, 0x90
	.type	null_null,@function
null_null:
.Lfunc_begin4:
	push	r15
	push	r14
	push	r12
	push	rbx
	push	rax
	mov	r14, rdx
	mov	r15, rsi
	mov	r12, rdi
	test	rdx, rdx
	je	.LBB7_1
	mov	rbx, qword ptr [r14]
	jmp	.LBB7_3
.LBB7_1:
.LBB7_3:
.Ltmp36:
	mov	rdi, r12
	mov	rsi, r15
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
.Ltmp37:
.Ltmp38:
	mov	rdi, r12
	mov	rsi, r15
	mov	rdx, r14
	call	rax
.Ltmp39:
	mov	r15, rax
	test	r14, r14
	je	.LBB7_8
	mov	rdi, qword ptr [r14]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	test	rbx, rbx
	je	.LBB7_8
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
.LBB7_8:
	mov	rax, r15
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	ret
.LBB7_10:
.Ltmp40:
	mov	r15, rax
.Ltmp41:
	mov	rdi, r14
	mov	rsi, rbx
	call	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0)
.Ltmp42:
	mov	rdi, r15
	call	_Unwind_Resume@PLT
.LBB7_9:
.Ltmp43:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end7:
	.size	null_null, .Lfunc_end7-null_null
	.section	.gcc_except_table.null_null,"a",@progbits
	.p2align	2, 0x0
GCC_except_table7:
.Lexception4:
	.byte	255
	.byte	155
	.uleb128 .Lttbase4-.Lttbaseref4
.Lttbaseref4:
	.byte	1
	.uleb128 .Lcst_end4-.Lcst_begin4
.Lcst_begin4:
	.uleb128 .Ltmp36-.Lfunc_begin4
	.uleb128 .Ltmp39-.Ltmp36
	.uleb128 .Ltmp40-.Lfunc_begin4
	.byte	0
	.uleb128 .Ltmp39-.Lfunc_begin4
	.uleb128 .Ltmp41-.Ltmp39
	.byte	0
	.byte	0
	.uleb128 .Ltmp41-.Lfunc_begin4
	.uleb128 .Ltmp42-.Ltmp41
	.uleb128 .Ltmp43-.Lfunc_begin4
	.byte	1
	.uleb128 .Ltmp42-.Lfunc_begin4
	.uleb128 .Lfunc_end7-.Ltmp42
	.byte	0
	.byte	0
.Lcst_end4:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase4:
	.byte	0
	.p2align	2, 0x0

	.section	.text.two_nonnull_nonnull,"ax",@progbits
	.globl	two_nonnull_nonnull
	.p2align	4, 0x90
	.type	two_nonnull_nonnull,@function
two_nonnull_nonnull:
.Lfunc_begin5:
	push	rbp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	sub	rsp, 40
	mov	r14, rcx
	mov	r12, rdx
	mov	r13, rsi
	mov	rbp, rdi
	mov	r15, qword ptr [rdx]
	mov	rbx, qword ptr [rcx]
	mov	qword ptr [rsp + 8], rdx
	mov	qword ptr [rsp + 16], r15
	mov	qword ptr [rsp + 24], rcx
	mov	qword ptr [rsp + 32], rbx
.Ltmp44:
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
.Ltmp45:
.Ltmp46:
	mov	rdi, rbp
	mov	rsi, r13
	mov	rdx, r12
	mov	rcx, r14
	call	rax
.Ltmp47:
	mov	r13, rax
	mov	rdi, qword ptr [r12]
.Ltmp52:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp53:
.Ltmp54:
	mov	rdi, r15
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp55:
	mov	rdi, qword ptr [r14]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
	mov	rax, r13
	add	rsp, 40
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	ret
.LBB8_6:
.Ltmp56:
	mov	r15, rax
	mov	rdi, qword ptr [r14]
.Ltmp57:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp58:
.Ltmp59:
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp60:
	jmp	.LBB8_8
.LBB8_10:
.Ltmp61:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.LBB8_5:
.Ltmp48:
	mov	r15, rax
.Ltmp49:
	lea	rdi, [rsp + 8]
	call	SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0)
.Ltmp50:
.LBB8_8:
	mov	rdi, r15
	call	_Unwind_Resume@PLT
.LBB8_9:
.Ltmp51:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end8:
	.size	two_nonnull_nonnull, .Lfunc_end8-two_nonnull_nonnull
	.section	.gcc_except_table.two_nonnull_nonnull,"a",@progbits
	.p2align	2, 0x0
GCC_except_table8:
.Lexception5:
	.byte	255
	.byte	155
	.uleb128 .Lttbase5-.Lttbaseref5
.Lttbaseref5:
	.byte	1
	.uleb128 .Lcst_end5-.Lcst_begin5
.Lcst_begin5:
	.uleb128 .Ltmp44-.Lfunc_begin5
	.uleb128 .Ltmp47-.Ltmp44
	.uleb128 .Ltmp48-.Lfunc_begin5
	.byte	0
	.uleb128 .Ltmp52-.Lfunc_begin5
	.uleb128 .Ltmp55-.Ltmp52
	.uleb128 .Ltmp56-.Lfunc_begin5
	.byte	0
	.uleb128 .Ltmp55-.Lfunc_begin5
	.uleb128 .Ltmp57-.Ltmp55
	.byte	0
	.byte	0
	.uleb128 .Ltmp57-.Lfunc_begin5
	.uleb128 .Ltmp60-.Ltmp57
	.uleb128 .Ltmp61-.Lfunc_begin5
	.byte	1
	.uleb128 .Ltmp49-.Lfunc_begin5
	.uleb128 .Ltmp50-.Ltmp49
	.uleb128 .Ltmp51-.Lfunc_begin5
	.byte	1
	.uleb128 .Ltmp50-.Lfunc_begin5
	.uleb128 .Lfunc_end8-.Ltmp50
	.byte	0
	.byte	0
.Lcst_end5:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase5:
	.byte	0
	.p2align	2, 0x0

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
.Lfunc_end9:
	.size	call_with_none1, .Lfunc_end9-call_with_none1

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
.Lfunc_end10:
	.size	call_with_none2, .Lfunc_end10-call_with_none2

	.section	.text.call_with_none3,"ax",@progbits
	.globl	call_with_none3
	.p2align	4, 0x90
	.type	call_with_none3,@function
call_with_none3:
.Lfunc_begin6:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	mov	qword ptr [rsp], 0
.Ltmp62:
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
.Ltmp63:
.Ltmp64:
	mov	rdx, rsp
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
.Ltmp65:
	mov	rbx, rax
	mov	rdi, qword ptr [rsp]
.Ltmp70:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp71:
	mov	rdx, qword ptr [rsp]
	mov	rax, rbx
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB11_6:
.Ltmp72:
	mov	rbx, rax
	jmp	.LBB11_7
.LBB11_4:
.Ltmp66:
	mov	rbx, rax
	mov	rdi, qword ptr [rsp]
.Ltmp67:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp68:
.LBB11_7:
	mov	rdi, qword ptr [rsp]
	test	rdi, rdi
	je	.LBB11_9
.Ltmp73:
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp74:
.LBB11_9:
	mov	rdi, rbx
	call	_Unwind_Resume@PLT
.LBB11_10:
.Ltmp75:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.LBB11_5:
.Ltmp69:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end11:
	.size	call_with_none3, .Lfunc_end11-call_with_none3
	.section	.gcc_except_table.call_with_none3,"a",@progbits
	.p2align	2, 0x0
GCC_except_table11:
.Lexception6:
	.byte	255
	.byte	155
	.uleb128 .Lttbase6-.Lttbaseref6
.Lttbaseref6:
	.byte	1
	.uleb128 .Lcst_end6-.Lcst_begin6
.Lcst_begin6:
	.uleb128 .Ltmp62-.Lfunc_begin6
	.uleb128 .Ltmp65-.Ltmp62
	.uleb128 .Ltmp66-.Lfunc_begin6
	.byte	0
	.uleb128 .Ltmp70-.Lfunc_begin6
	.uleb128 .Ltmp71-.Ltmp70
	.uleb128 .Ltmp72-.Lfunc_begin6
	.byte	0
	.uleb128 .Ltmp67-.Lfunc_begin6
	.uleb128 .Ltmp68-.Ltmp67
	.uleb128 .Ltmp69-.Lfunc_begin6
	.byte	1
	.uleb128 .Ltmp73-.Lfunc_begin6
	.uleb128 .Ltmp74-.Ltmp73
	.uleb128 .Ltmp75-.Lfunc_begin6
	.byte	1
	.uleb128 .Ltmp74-.Lfunc_begin6
	.uleb128 .Lfunc_end11-.Ltmp74
	.byte	0
	.byte	0
.Lcst_end6:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase6:
	.byte	0
	.p2align	2, 0x0

	.section	.text.call_with_none4,"ax",@progbits
	.globl	call_with_none4
	.p2align	4, 0x90
	.type	call_with_none4,@function
call_with_none4:
.Lfunc_begin7:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	mov	qword ptr [rsp], 0
.Ltmp76:
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
.Ltmp77:
.Ltmp78:
	mov	rdx, rsp
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
.Ltmp79:
	mov	rbx, rax
	mov	rdi, qword ptr [rsp]
.Ltmp84:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp85:
	mov	rdx, qword ptr [rsp]
	mov	rax, rbx
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB12_6:
.Ltmp86:
	mov	rbx, rax
	jmp	.LBB12_7
.LBB12_4:
.Ltmp80:
	mov	rbx, rax
	mov	rdi, qword ptr [rsp]
.Ltmp81:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp82:
.LBB12_7:
	mov	rdi, qword ptr [rsp]
	test	rdi, rdi
	je	.LBB12_9
.Ltmp87:
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp88:
.LBB12_9:
	mov	rdi, rbx
	call	_Unwind_Resume@PLT
.LBB12_10:
.Ltmp89:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.LBB12_5:
.Ltmp83:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end12:
	.size	call_with_none4, .Lfunc_end12-call_with_none4
	.section	.gcc_except_table.call_with_none4,"a",@progbits
	.p2align	2, 0x0
GCC_except_table12:
.Lexception7:
	.byte	255
	.byte	155
	.uleb128 .Lttbase7-.Lttbaseref7
.Lttbaseref7:
	.byte	1
	.uleb128 .Lcst_end7-.Lcst_begin7
.Lcst_begin7:
	.uleb128 .Ltmp76-.Lfunc_begin7
	.uleb128 .Ltmp79-.Ltmp76
	.uleb128 .Ltmp80-.Lfunc_begin7
	.byte	0
	.uleb128 .Ltmp84-.Lfunc_begin7
	.uleb128 .Ltmp85-.Ltmp84
	.uleb128 .Ltmp86-.Lfunc_begin7
	.byte	0
	.uleb128 .Ltmp81-.Lfunc_begin7
	.uleb128 .Ltmp82-.Ltmp81
	.uleb128 .Ltmp83-.Lfunc_begin7
	.byte	1
	.uleb128 .Ltmp87-.Lfunc_begin7
	.uleb128 .Ltmp88-.Ltmp87
	.uleb128 .Ltmp89-.Lfunc_begin7
	.byte	1
	.uleb128 .Ltmp88-.Lfunc_begin7
	.uleb128 .Lfunc_end12-.Ltmp88
	.byte	0
	.byte	0
.Lcst_end7:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase7:
	.byte	0
	.p2align	2, 0x0

	.section	.text.call_with_some1,"ax",@progbits
	.globl	call_with_some1
	.p2align	4, 0x90
	.type	call_with_some1,@function
call_with_some1:
.Lfunc_begin8:
	push	r15
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rdx
	mov	r14, rsi
	mov	r15, rdi
	mov	qword ptr [rsp + 8], rdx
.Ltmp90:
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
.Ltmp91:
.Ltmp92:
	lea	rdx, [rsp + 8]
	mov	rdi, r15
	mov	rsi, r14
	call	rax
.Ltmp93:
	mov	r14, rax
	mov	rdi, qword ptr [rsp + 8]
.Ltmp100:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp101:
.Ltmp102:
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp103:
	mov	rdx, qword ptr [rsp + 8]
	mov	rax, r14
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	r15
	ret
.LBB13_8:
.Ltmp104:
	mov	r14, rax
	jmp	.LBB13_9
.LBB13_5:
.Ltmp94:
	mov	r14, rax
	mov	rdi, qword ptr [rsp + 8]
.Ltmp95:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp96:
.Ltmp97:
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp98:
.LBB13_9:
	mov	rdi, qword ptr [rsp + 8]
.Ltmp105:
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp106:
	mov	rdi, r14
	call	_Unwind_Resume@PLT
.LBB13_11:
.Ltmp107:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.LBB13_7:
.Ltmp99:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end13:
	.size	call_with_some1, .Lfunc_end13-call_with_some1
	.section	.gcc_except_table.call_with_some1,"a",@progbits
	.p2align	2, 0x0
GCC_except_table13:
.Lexception8:
	.byte	255
	.byte	155
	.uleb128 .Lttbase8-.Lttbaseref8
.Lttbaseref8:
	.byte	1
	.uleb128 .Lcst_end8-.Lcst_begin8
.Lcst_begin8:
	.uleb128 .Ltmp90-.Lfunc_begin8
	.uleb128 .Ltmp93-.Ltmp90
	.uleb128 .Ltmp94-.Lfunc_begin8
	.byte	0
	.uleb128 .Ltmp100-.Lfunc_begin8
	.uleb128 .Ltmp103-.Ltmp100
	.uleb128 .Ltmp104-.Lfunc_begin8
	.byte	0
	.uleb128 .Ltmp95-.Lfunc_begin8
	.uleb128 .Ltmp98-.Ltmp95
	.uleb128 .Ltmp99-.Lfunc_begin8
	.byte	1
	.uleb128 .Ltmp105-.Lfunc_begin8
	.uleb128 .Ltmp106-.Ltmp105
	.uleb128 .Ltmp107-.Lfunc_begin8
	.byte	1
	.uleb128 .Ltmp106-.Lfunc_begin8
	.uleb128 .Lfunc_end13-.Ltmp106
	.byte	0
	.byte	0
.Lcst_end8:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase8:
	.byte	0
	.p2align	2, 0x0

	.section	.text.call_with_some2,"ax",@progbits
	.globl	call_with_some2
	.p2align	4, 0x90
	.type	call_with_some2,@function
call_with_some2:
.Lfunc_begin9:
	push	r15
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rdx
	mov	r14, rsi
	mov	r15, rdi
	mov	qword ptr [rsp + 8], rdx
.Ltmp108:
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
.Ltmp109:
.Ltmp110:
	lea	rdx, [rsp + 8]
	mov	rdi, r15
	mov	rsi, r14
	call	rax
.Ltmp111:
	mov	r14, rax
	mov	rdi, qword ptr [rsp + 8]
.Ltmp118:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp119:
.Ltmp120:
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp121:
	mov	rdx, qword ptr [rsp + 8]
	mov	rax, r14
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	r15
	ret
.LBB14_8:
.Ltmp122:
	mov	r14, rax
	jmp	.LBB14_9
.LBB14_5:
.Ltmp112:
	mov	r14, rax
	mov	rdi, qword ptr [rsp + 8]
.Ltmp113:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp114:
.Ltmp115:
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp116:
.LBB14_9:
	mov	rdi, qword ptr [rsp + 8]
	test	rdi, rdi
	je	.LBB14_11
.Ltmp123:
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp124:
.LBB14_11:
	mov	rdi, r14
	call	_Unwind_Resume@PLT
.LBB14_12:
.Ltmp125:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.LBB14_7:
.Ltmp117:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end14:
	.size	call_with_some2, .Lfunc_end14-call_with_some2
	.section	.gcc_except_table.call_with_some2,"a",@progbits
	.p2align	2, 0x0
GCC_except_table14:
.Lexception9:
	.byte	255
	.byte	155
	.uleb128 .Lttbase9-.Lttbaseref9
.Lttbaseref9:
	.byte	1
	.uleb128 .Lcst_end9-.Lcst_begin9
.Lcst_begin9:
	.uleb128 .Ltmp108-.Lfunc_begin9
	.uleb128 .Ltmp111-.Ltmp108
	.uleb128 .Ltmp112-.Lfunc_begin9
	.byte	0
	.uleb128 .Ltmp118-.Lfunc_begin9
	.uleb128 .Ltmp121-.Ltmp118
	.uleb128 .Ltmp122-.Lfunc_begin9
	.byte	0
	.uleb128 .Ltmp113-.Lfunc_begin9
	.uleb128 .Ltmp116-.Ltmp113
	.uleb128 .Ltmp117-.Lfunc_begin9
	.byte	1
	.uleb128 .Ltmp123-.Lfunc_begin9
	.uleb128 .Ltmp124-.Ltmp123
	.uleb128 .Ltmp125-.Lfunc_begin9
	.byte	1
	.uleb128 .Ltmp124-.Lfunc_begin9
	.uleb128 .Lfunc_end14-.Ltmp124
	.byte	0
	.byte	0
.Lcst_end9:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase9:
	.byte	0
	.p2align	2, 0x0

	.section	.text.call_with_some3,"ax",@progbits
	.globl	call_with_some3
	.p2align	4, 0x90
	.type	call_with_some3,@function
call_with_some3:
.Lfunc_begin10:
	push	r15
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rdx
	mov	r14, rsi
	mov	r15, rdi
	mov	qword ptr [rsp + 8], rdx
.Ltmp126:
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
.Ltmp127:
.Ltmp128:
	lea	rdx, [rsp + 8]
	mov	rdi, r15
	mov	rsi, r14
	call	rax
.Ltmp129:
	mov	r14, rax
	mov	rdi, qword ptr [rsp + 8]
.Ltmp136:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp137:
.Ltmp138:
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp139:
	mov	rdx, qword ptr [rsp + 8]
	mov	rax, r14
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	r15
	ret
.LBB15_8:
.Ltmp140:
	mov	r14, rax
	jmp	.LBB15_9
.LBB15_5:
.Ltmp130:
	mov	r14, rax
	mov	rdi, qword ptr [rsp + 8]
.Ltmp131:
	call	qword ptr [rip + objc_retain@GOTPCREL]
.Ltmp132:
.Ltmp133:
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp134:
.LBB15_9:
	mov	rdi, qword ptr [rsp + 8]
	test	rdi, rdi
	je	.LBB15_11
.Ltmp141:
	call	qword ptr [rip + objc_release@GOTPCREL]
.Ltmp142:
.LBB15_11:
	mov	rdi, r14
	call	_Unwind_Resume@PLT
.LBB15_12:
.Ltmp143:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.LBB15_7:
.Ltmp135:
	call	qword ptr [rip + SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@GOTPCREL]
.Lfunc_end15:
	.size	call_with_some3, .Lfunc_end15-call_with_some3
	.section	.gcc_except_table.call_with_some3,"a",@progbits
	.p2align	2, 0x0
GCC_except_table15:
.Lexception10:
	.byte	255
	.byte	155
	.uleb128 .Lttbase10-.Lttbaseref10
.Lttbaseref10:
	.byte	1
	.uleb128 .Lcst_end10-.Lcst_begin10
.Lcst_begin10:
	.uleb128 .Ltmp126-.Lfunc_begin10
	.uleb128 .Ltmp129-.Ltmp126
	.uleb128 .Ltmp130-.Lfunc_begin10
	.byte	0
	.uleb128 .Ltmp136-.Lfunc_begin10
	.uleb128 .Ltmp139-.Ltmp136
	.uleb128 .Ltmp140-.Lfunc_begin10
	.byte	0
	.uleb128 .Ltmp131-.Lfunc_begin10
	.uleb128 .Ltmp134-.Ltmp131
	.uleb128 .Ltmp135-.Lfunc_begin10
	.byte	1
	.uleb128 .Ltmp141-.Lfunc_begin10
	.uleb128 .Ltmp142-.Ltmp141
	.uleb128 .Ltmp143-.Lfunc_begin10
	.byte	1
	.uleb128 .Ltmp142-.Lfunc_begin10
	.uleb128 .Lfunc_end15-.Ltmp142
	.byte	0
	.byte	0
.Lcst_end10:
	.byte	127
	.byte	0
	.p2align	2, 0x0
.Lttbase10:
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
