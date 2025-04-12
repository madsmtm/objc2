	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.p2align	4
SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0):
	test	rdi, rdi
	je	LBB0_1
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	rdi, qword ptr [rdi]
	call	_objc_retain
	mov	rdi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_release
LBB0_1:
	ret

	.p2align	4
SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0):
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	test	rdi, rdi
	je	LBB1_2
	mov	rbx, rsi
	mov	rdi, qword ptr [rdi]
	call	_objc_retain
	test	rbx, rbx
	je	LBB1_2
	mov	rdi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_release
LBB1_2:
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret

	.p2align	4
SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0):
Lfunc_begin0:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	mov	rbx, rdi
	mov	rax, qword ptr [rdi]
	mov	r14, qword ptr [rdi + 8]
	mov	rdi, qword ptr [rax]
Ltmp0:
	call	_objc_retain
Ltmp1:
Ltmp2:
	mov	rdi, r14
	call	_objc_release
Ltmp3:
	mov	rax, qword ptr [rbx + 16]
	mov	rbx, qword ptr [rbx + 24]
	mov	rdi, qword ptr [rax]
	call	_objc_retain
	mov	rdi, rbx
	pop	rbx
	pop	r14
	pop	rbp
	jmp	_objc_release
LBB2_3:
Ltmp4:
	mov	r14, rax
	mov	rax, qword ptr [rbx + 16]
	mov	rbx, qword ptr [rbx + 24]
	mov	rdi, qword ptr [rax]
Ltmp5:
	call	_objc_retain
Ltmp6:
Ltmp7:
	mov	rdi, rbx
	call	_objc_release
Ltmp8:
	mov	rdi, r14
	call	__Unwind_Resume
LBB2_6:
Ltmp9:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table2:
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
	.uleb128 Ltmp3-Lfunc_begin0
	.uleb128 Ltmp5-Ltmp3
	.byte	0
	.byte	0
	.uleb128 Ltmp5-Lfunc_begin0
	.uleb128 Ltmp8-Ltmp5
	.uleb128 Ltmp9-Lfunc_begin0
	.byte	1
	.uleb128 Ltmp8-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp8
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
	.p2align	4
SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>,)>, 0):
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	rdi, qword ptr [rdi]
	call	_objc_retain
	test	rbx, rbx
	je	LBB3_1
	mov	rdi, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	jmp	_objc_release
LBB3_1:
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret

	.globl	_nonnull_nonnull
	.p2align	4
_nonnull_nonnull:
Lfunc_begin1:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	mov	r15, rdx
	mov	rbx, qword ptr [rdx]
Ltmp10:
	call	_objc_msgSend
Ltmp11:
	mov	r14, rax
	mov	rdi, qword ptr [r15]
	call	_objc_retain
	mov	rdi, rbx
	call	_objc_release
	mov	rax, r14
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB4_2:
Ltmp12:
	mov	r14, rax
	mov	rdi, qword ptr [r15]
Ltmp13:
	call	_objc_retain
Ltmp14:
Ltmp15:
	mov	rdi, rbx
	call	_objc_release
Ltmp16:
	mov	rdi, r14
	call	__Unwind_Resume
LBB4_5:
Ltmp17:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end1:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table4:
Lexception1:
	.byte	255
	.byte	155
	.uleb128 Lttbase1-Lttbaseref1
Lttbaseref1:
	.byte	1
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
	.uleb128 Ltmp10-Lfunc_begin1
	.uleb128 Ltmp11-Ltmp10
	.uleb128 Ltmp12-Lfunc_begin1
	.byte	0
	.uleb128 Ltmp11-Lfunc_begin1
	.uleb128 Ltmp13-Ltmp11
	.byte	0
	.byte	0
	.uleb128 Ltmp13-Lfunc_begin1
	.uleb128 Ltmp16-Ltmp13
	.uleb128 Ltmp17-Lfunc_begin1
	.byte	1
	.uleb128 Ltmp16-Lfunc_begin1
	.uleb128 Lfunc_end1-Ltmp16
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
	.globl	_null_nonnull
	.p2align	4
_null_nonnull:
Lfunc_begin2:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	mov	rbx, rdx
	test	rdx, rdx
	je	LBB5_1
	mov	r14, qword ptr [rbx]
	jmp	LBB5_3
LBB5_1:
LBB5_3:
Ltmp18:
	mov	rdx, rbx
	call	_objc_msgSend
Ltmp19:
	test	rbx, rbx
	je	LBB5_6
	mov	rdi, qword ptr [rbx]
	mov	rbx, rax
	call	_objc_retain
	mov	rdi, r14
	call	_objc_release
	mov	rax, rbx
LBB5_6:
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB5_8:
Ltmp20:
	mov	r15, rax
Ltmp21:
	mov	rdi, rbx
	mov	rsi, r14
	call	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0)
Ltmp22:
	mov	rdi, r15
	call	__Unwind_Resume
LBB5_7:
Ltmp23:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end2:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table5:
Lexception2:
	.byte	255
	.byte	155
	.uleb128 Lttbase2-Lttbaseref2
Lttbaseref2:
	.byte	1
	.uleb128 Lcst_end2-Lcst_begin2
Lcst_begin2:
	.uleb128 Ltmp18-Lfunc_begin2
	.uleb128 Ltmp19-Ltmp18
	.uleb128 Ltmp20-Lfunc_begin2
	.byte	0
	.uleb128 Ltmp19-Lfunc_begin2
	.uleb128 Ltmp21-Ltmp19
	.byte	0
	.byte	0
	.uleb128 Ltmp21-Lfunc_begin2
	.uleb128 Ltmp22-Ltmp21
	.uleb128 Ltmp23-Lfunc_begin2
	.byte	1
	.uleb128 Ltmp22-Lfunc_begin2
	.uleb128 Lfunc_end2-Ltmp22
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
	.globl	_nonnull_null
	.p2align	4
_nonnull_null:
Lfunc_begin3:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	mov	r14, rdx
	mov	rbx, qword ptr [rdx]
Ltmp24:
	call	_objc_msgSend
Ltmp25:
	mov	r15, rax
	mov	rdi, qword ptr [r14]
	call	_objc_retain
	test	rbx, rbx
	je	LBB6_3
	mov	rdi, rbx
	call	_objc_release
LBB6_3:
	mov	rax, r15
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB6_5:
Ltmp26:
	mov	r15, rax
Ltmp27:
	mov	rdi, r14
	mov	rsi, rbx
	call	SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>,)>, 0)
Ltmp28:
	mov	rdi, r15
	call	__Unwind_Resume
LBB6_4:
Ltmp29:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end3:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table6:
Lexception3:
	.byte	255
	.byte	155
	.uleb128 Lttbase3-Lttbaseref3
Lttbaseref3:
	.byte	1
	.uleb128 Lcst_end3-Lcst_begin3
Lcst_begin3:
	.uleb128 Ltmp24-Lfunc_begin3
	.uleb128 Ltmp25-Ltmp24
	.uleb128 Ltmp26-Lfunc_begin3
	.byte	0
	.uleb128 Ltmp25-Lfunc_begin3
	.uleb128 Ltmp27-Ltmp25
	.byte	0
	.byte	0
	.uleb128 Ltmp27-Lfunc_begin3
	.uleb128 Ltmp28-Ltmp27
	.uleb128 Ltmp29-Lfunc_begin3
	.byte	1
	.uleb128 Ltmp28-Lfunc_begin3
	.uleb128 Lfunc_end3-Ltmp28
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
	.globl	_null_null
	.p2align	4
_null_null:
Lfunc_begin4:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	rbx
	push	rax
	mov	r14, rdx
	test	rdx, rdx
	je	LBB7_1
	mov	rbx, qword ptr [r14]
	jmp	LBB7_3
LBB7_1:
LBB7_3:
Ltmp30:
	mov	rdx, r14
	call	_objc_msgSend
Ltmp31:
	mov	r15, rax
	test	r14, r14
	je	LBB7_7
	mov	rdi, qword ptr [r14]
	call	_objc_retain
	test	rbx, rbx
	je	LBB7_7
	mov	rdi, rbx
	call	_objc_release
LBB7_7:
	mov	rax, r15
	add	rsp, 8
	pop	rbx
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB7_9:
Ltmp32:
	mov	r15, rax
Ltmp33:
	mov	rdi, r14
	mov	rsi, rbx
	call	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0)
Ltmp34:
	mov	rdi, r15
	call	__Unwind_Resume
LBB7_8:
Ltmp35:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end4:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table7:
Lexception4:
	.byte	255
	.byte	155
	.uleb128 Lttbase4-Lttbaseref4
Lttbaseref4:
	.byte	1
	.uleb128 Lcst_end4-Lcst_begin4
Lcst_begin4:
	.uleb128 Ltmp30-Lfunc_begin4
	.uleb128 Ltmp31-Ltmp30
	.uleb128 Ltmp32-Lfunc_begin4
	.byte	0
	.uleb128 Ltmp31-Lfunc_begin4
	.uleb128 Ltmp33-Ltmp31
	.byte	0
	.byte	0
	.uleb128 Ltmp33-Lfunc_begin4
	.uleb128 Ltmp34-Ltmp33
	.uleb128 Ltmp35-Lfunc_begin4
	.byte	1
	.uleb128 Ltmp34-Lfunc_begin4
	.uleb128 Lfunc_end4-Ltmp34
	.byte	0
	.byte	0
Lcst_end4:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase4:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_two_nonnull_nonnull
	.p2align	4
_two_nonnull_nonnull:
Lfunc_begin5:
	push	rbp
	mov	rbp, rsp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	sub	rsp, 40
	mov	r14, rcx
	mov	r13, rdx
	mov	r12, qword ptr [rdx]
	mov	rbx, qword ptr [rcx]
	mov	qword ptr [rbp - 72], rdx
	mov	qword ptr [rbp - 64], r12
	mov	qword ptr [rbp - 56], rcx
	mov	qword ptr [rbp - 48], rbx
Ltmp36:
	call	_objc_msgSend
Ltmp37:
	mov	r15, rax
	mov	rdi, qword ptr [r13]
Ltmp42:
	call	_objc_retain
Ltmp43:
Ltmp44:
	mov	rdi, r12
	call	_objc_release
Ltmp45:
	mov	rdi, qword ptr [r14]
	call	_objc_retain
	mov	rdi, rbx
	call	_objc_release
	mov	rax, r15
	add	rsp, 40
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	ret
LBB8_4:
Ltmp38:
	mov	r15, rax
Ltmp39:
	lea	rdi, [rbp - 72]
	call	SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0)
Ltmp40:
	jmp	LBB8_7
LBB8_8:
Ltmp41:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB8_5:
Ltmp46:
	mov	r15, rax
	mov	rdi, qword ptr [r14]
Ltmp47:
	call	_objc_retain
Ltmp48:
Ltmp49:
	mov	rdi, rbx
	call	_objc_release
Ltmp50:
LBB8_7:
	mov	rdi, r15
	call	__Unwind_Resume
LBB8_9:
Ltmp51:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end5:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table8:
Lexception5:
	.byte	255
	.byte	155
	.uleb128 Lttbase5-Lttbaseref5
Lttbaseref5:
	.byte	1
	.uleb128 Lcst_end5-Lcst_begin5
Lcst_begin5:
	.uleb128 Ltmp36-Lfunc_begin5
	.uleb128 Ltmp37-Ltmp36
	.uleb128 Ltmp38-Lfunc_begin5
	.byte	0
	.uleb128 Ltmp42-Lfunc_begin5
	.uleb128 Ltmp45-Ltmp42
	.uleb128 Ltmp46-Lfunc_begin5
	.byte	0
	.uleb128 Ltmp45-Lfunc_begin5
	.uleb128 Ltmp39-Ltmp45
	.byte	0
	.byte	0
	.uleb128 Ltmp39-Lfunc_begin5
	.uleb128 Ltmp40-Ltmp39
	.uleb128 Ltmp41-Lfunc_begin5
	.byte	1
	.uleb128 Ltmp47-Lfunc_begin5
	.uleb128 Ltmp50-Ltmp47
	.uleb128 Ltmp51-Lfunc_begin5
	.byte	1
	.uleb128 Ltmp50-Lfunc_begin5
	.uleb128 Lfunc_end5-Ltmp50
	.byte	0
	.byte	0
Lcst_end5:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase5:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_call_with_none1
	.p2align	4
_call_with_none1:
	push	rbp
	mov	rbp, rsp
	xor	edx, edx
	pop	rbp
	jmp	_objc_msgSend

	.globl	_call_with_none2
	.p2align	4
_call_with_none2:
	push	rbp
	mov	rbp, rsp
	xor	edx, edx
	pop	rbp
	jmp	_objc_msgSend

	.globl	_call_with_none3
	.p2align	4
_call_with_none3:
Lfunc_begin6:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	qword ptr [rbp - 16], 0
Ltmp52:
	lea	rdx, [rbp - 16]
	call	_objc_msgSend
Ltmp53:
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 16]
Ltmp58:
	call	_objc_retain
Ltmp59:
	mov	rdx, qword ptr [rbp - 16]
	mov	rax, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret
LBB11_5:
Ltmp60:
	mov	rbx, rax
	jmp	LBB11_6
LBB11_3:
Ltmp54:
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 16]
Ltmp55:
	call	_objc_retain
Ltmp56:
LBB11_6:
	mov	rdi, qword ptr [rbp - 16]
	test	rdi, rdi
	je	LBB11_8
Ltmp61:
	call	_objc_release
Ltmp62:
LBB11_8:
	mov	rdi, rbx
	call	__Unwind_Resume
LBB11_9:
Ltmp63:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB11_4:
Ltmp57:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end6:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table11:
Lexception6:
	.byte	255
	.byte	155
	.uleb128 Lttbase6-Lttbaseref6
Lttbaseref6:
	.byte	1
	.uleb128 Lcst_end6-Lcst_begin6
Lcst_begin6:
	.uleb128 Ltmp52-Lfunc_begin6
	.uleb128 Ltmp53-Ltmp52
	.uleb128 Ltmp54-Lfunc_begin6
	.byte	0
	.uleb128 Ltmp58-Lfunc_begin6
	.uleb128 Ltmp59-Ltmp58
	.uleb128 Ltmp60-Lfunc_begin6
	.byte	0
	.uleb128 Ltmp55-Lfunc_begin6
	.uleb128 Ltmp56-Ltmp55
	.uleb128 Ltmp57-Lfunc_begin6
	.byte	1
	.uleb128 Ltmp61-Lfunc_begin6
	.uleb128 Ltmp62-Ltmp61
	.uleb128 Ltmp63-Lfunc_begin6
	.byte	1
	.uleb128 Ltmp62-Lfunc_begin6
	.uleb128 Lfunc_end6-Ltmp62
	.byte	0
	.byte	0
Lcst_end6:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase6:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_call_with_none4
	.p2align	4
_call_with_none4:
Lfunc_begin7:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	qword ptr [rbp - 16], 0
Ltmp64:
	lea	rdx, [rbp - 16]
	call	_objc_msgSend
Ltmp65:
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 16]
Ltmp70:
	call	_objc_retain
Ltmp71:
	mov	rdx, qword ptr [rbp - 16]
	mov	rax, rbx
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret
LBB12_5:
Ltmp72:
	mov	rbx, rax
	jmp	LBB12_6
LBB12_3:
Ltmp66:
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 16]
Ltmp67:
	call	_objc_retain
Ltmp68:
LBB12_6:
	mov	rdi, qword ptr [rbp - 16]
	test	rdi, rdi
	je	LBB12_8
Ltmp73:
	call	_objc_release
Ltmp74:
LBB12_8:
	mov	rdi, rbx
	call	__Unwind_Resume
LBB12_9:
Ltmp75:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB12_4:
Ltmp69:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end7:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table12:
Lexception7:
	.byte	255
	.byte	155
	.uleb128 Lttbase7-Lttbaseref7
Lttbaseref7:
	.byte	1
	.uleb128 Lcst_end7-Lcst_begin7
Lcst_begin7:
	.uleb128 Ltmp64-Lfunc_begin7
	.uleb128 Ltmp65-Ltmp64
	.uleb128 Ltmp66-Lfunc_begin7
	.byte	0
	.uleb128 Ltmp70-Lfunc_begin7
	.uleb128 Ltmp71-Ltmp70
	.uleb128 Ltmp72-Lfunc_begin7
	.byte	0
	.uleb128 Ltmp67-Lfunc_begin7
	.uleb128 Ltmp68-Ltmp67
	.uleb128 Ltmp69-Lfunc_begin7
	.byte	1
	.uleb128 Ltmp73-Lfunc_begin7
	.uleb128 Ltmp74-Ltmp73
	.uleb128 Ltmp75-Lfunc_begin7
	.byte	1
	.uleb128 Ltmp74-Lfunc_begin7
	.uleb128 Lfunc_end7-Ltmp74
	.byte	0
	.byte	0
Lcst_end7:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase7:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_call_with_some1
	.p2align	4
_call_with_some1:
Lfunc_begin8:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rdx
	mov	qword ptr [rbp - 24], rdx
Ltmp76:
	lea	rdx, [rbp - 24]
	call	_objc_msgSend
Ltmp77:
	mov	r14, rax
	mov	rdi, qword ptr [rbp - 24]
Ltmp84:
	call	_objc_retain
Ltmp85:
Ltmp86:
	mov	rdi, rbx
	call	_objc_release
Ltmp87:
	mov	rdx, qword ptr [rbp - 24]
	mov	rax, r14
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB13_4:
Ltmp78:
	mov	r14, rax
	mov	rdi, qword ptr [rbp - 24]
Ltmp79:
	call	_objc_retain
Ltmp80:
Ltmp81:
	mov	rdi, rbx
	call	_objc_release
Ltmp82:
	jmp	LBB13_8
LBB13_6:
Ltmp83:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB13_7:
Ltmp88:
	mov	r14, rax
LBB13_8:
	mov	rdi, qword ptr [rbp - 24]
Ltmp89:
	call	_objc_release
Ltmp90:
	mov	rdi, r14
	call	__Unwind_Resume
LBB13_10:
Ltmp91:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end8:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table13:
Lexception8:
	.byte	255
	.byte	155
	.uleb128 Lttbase8-Lttbaseref8
Lttbaseref8:
	.byte	1
	.uleb128 Lcst_end8-Lcst_begin8
Lcst_begin8:
	.uleb128 Ltmp76-Lfunc_begin8
	.uleb128 Ltmp77-Ltmp76
	.uleb128 Ltmp78-Lfunc_begin8
	.byte	0
	.uleb128 Ltmp84-Lfunc_begin8
	.uleb128 Ltmp87-Ltmp84
	.uleb128 Ltmp88-Lfunc_begin8
	.byte	0
	.uleb128 Ltmp79-Lfunc_begin8
	.uleb128 Ltmp82-Ltmp79
	.uleb128 Ltmp83-Lfunc_begin8
	.byte	1
	.uleb128 Ltmp89-Lfunc_begin8
	.uleb128 Ltmp90-Ltmp89
	.uleb128 Ltmp91-Lfunc_begin8
	.byte	1
	.uleb128 Ltmp90-Lfunc_begin8
	.uleb128 Lfunc_end8-Ltmp90
	.byte	0
	.byte	0
Lcst_end8:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase8:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_call_with_some2
	.p2align	4
_call_with_some2:
Lfunc_begin9:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rdx
	mov	qword ptr [rbp - 24], rdx
Ltmp92:
	lea	rdx, [rbp - 24]
	call	_objc_msgSend
Ltmp93:
	mov	r14, rax
	mov	rdi, qword ptr [rbp - 24]
Ltmp100:
	call	_objc_retain
Ltmp101:
Ltmp102:
	mov	rdi, rbx
	call	_objc_release
Ltmp103:
	mov	rdx, qword ptr [rbp - 24]
	mov	rax, r14
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB14_4:
Ltmp94:
	mov	r14, rax
	mov	rdi, qword ptr [rbp - 24]
Ltmp95:
	call	_objc_retain
Ltmp96:
Ltmp97:
	mov	rdi, rbx
	call	_objc_release
Ltmp98:
	jmp	LBB14_8
LBB14_6:
Ltmp99:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB14_7:
Ltmp104:
	mov	r14, rax
LBB14_8:
	mov	rdi, qword ptr [rbp - 24]
	test	rdi, rdi
	je	LBB14_10
Ltmp105:
	call	_objc_release
Ltmp106:
LBB14_10:
	mov	rdi, r14
	call	__Unwind_Resume
LBB14_11:
Ltmp107:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end9:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table14:
Lexception9:
	.byte	255
	.byte	155
	.uleb128 Lttbase9-Lttbaseref9
Lttbaseref9:
	.byte	1
	.uleb128 Lcst_end9-Lcst_begin9
Lcst_begin9:
	.uleb128 Ltmp92-Lfunc_begin9
	.uleb128 Ltmp93-Ltmp92
	.uleb128 Ltmp94-Lfunc_begin9
	.byte	0
	.uleb128 Ltmp100-Lfunc_begin9
	.uleb128 Ltmp103-Ltmp100
	.uleb128 Ltmp104-Lfunc_begin9
	.byte	0
	.uleb128 Ltmp95-Lfunc_begin9
	.uleb128 Ltmp98-Ltmp95
	.uleb128 Ltmp99-Lfunc_begin9
	.byte	1
	.uleb128 Ltmp105-Lfunc_begin9
	.uleb128 Ltmp106-Ltmp105
	.uleb128 Ltmp107-Lfunc_begin9
	.byte	1
	.uleb128 Ltmp106-Lfunc_begin9
	.uleb128 Lfunc_end9-Ltmp106
	.byte	0
	.byte	0
Lcst_end9:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase9:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_call_with_some3
	.p2align	4
_call_with_some3:
Lfunc_begin10:
	push	rbp
	mov	rbp, rsp
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rdx
	mov	qword ptr [rbp - 24], rdx
Ltmp108:
	lea	rdx, [rbp - 24]
	call	_objc_msgSend
Ltmp109:
	mov	r14, rax
	mov	rdi, qword ptr [rbp - 24]
Ltmp116:
	call	_objc_retain
Ltmp117:
Ltmp118:
	mov	rdi, rbx
	call	_objc_release
Ltmp119:
	mov	rdx, qword ptr [rbp - 24]
	mov	rax, r14
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	rbp
	ret
LBB15_4:
Ltmp110:
	mov	r14, rax
	mov	rdi, qword ptr [rbp - 24]
Ltmp111:
	call	_objc_retain
Ltmp112:
Ltmp113:
	mov	rdi, rbx
	call	_objc_release
Ltmp114:
	jmp	LBB15_8
LBB15_6:
Ltmp115:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB15_7:
Ltmp120:
	mov	r14, rax
LBB15_8:
	mov	rdi, qword ptr [rbp - 24]
	test	rdi, rdi
	je	LBB15_10
Ltmp121:
	call	_objc_release
Ltmp122:
LBB15_10:
	mov	rdi, r14
	call	__Unwind_Resume
LBB15_11:
Ltmp123:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
Lfunc_end10:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table15:
Lexception10:
	.byte	255
	.byte	155
	.uleb128 Lttbase10-Lttbaseref10
Lttbaseref10:
	.byte	1
	.uleb128 Lcst_end10-Lcst_begin10
Lcst_begin10:
	.uleb128 Ltmp108-Lfunc_begin10
	.uleb128 Ltmp109-Ltmp108
	.uleb128 Ltmp110-Lfunc_begin10
	.byte	0
	.uleb128 Ltmp116-Lfunc_begin10
	.uleb128 Ltmp119-Ltmp116
	.uleb128 Ltmp120-Lfunc_begin10
	.byte	0
	.uleb128 Ltmp111-Lfunc_begin10
	.uleb128 Ltmp114-Ltmp111
	.uleb128 Ltmp115-Lfunc_begin10
	.byte	1
	.uleb128 Ltmp121-Lfunc_begin10
	.uleb128 Ltmp122-Ltmp121
	.uleb128 Ltmp123-Lfunc_begin10
	.byte	1
	.uleb128 Ltmp122-Lfunc_begin10
	.uleb128 Lfunc_end10-Ltmp122
	.byte	0
	.byte	0
Lcst_end10:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase10:
	.byte	0
	.p2align	2, 0x0

.subsections_via_symbols
