	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_fn1_strong_none
	.p2align	4
_fn1_strong_none:
	push	rbp
	mov	rbp, rsp
	xor	esi, esi
	pop	rbp
	jmp	_SecTrustEvaluateWithError

	.globl	_fn2_strong_some_none
	.p2align	4
_fn2_strong_some_none:
Lfunc_begin0:
	push	rbp
	mov	rbp, rsp
	push	rbx
	push	rax
	mov	qword ptr [rbp - 16], 0
Ltmp0:
	lea	rsi, [rbp - 16]
	call	_SecTrustEvaluateWithError
Ltmp1:
	mov	rax, qword ptr [rbp - 16]
	add	rsp, 8
	pop	rbx
	pop	rbp
	ret
LBB1_2:
Ltmp2:
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 16]
	test	rdi, rdi
	je	LBB1_4
Ltmp3:
	call	_CFRelease
Ltmp4:
LBB1_4:
	mov	rdi, rbx
	call	__Unwind_Resume
LBB1_5:
Ltmp5:
	call	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end0:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table1:
Lexception0:
	.byte	255
	.byte	155
	.uleb128 Lttbase0-Lttbaseref0
Lttbaseref0:
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp1-Ltmp0
	.uleb128 Ltmp2-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp3-Lfunc_begin0
	.uleb128 Ltmp4-Ltmp3
	.uleb128 Ltmp5-Lfunc_begin0
	.byte	1
	.uleb128 Ltmp4-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp4
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
	.globl	_fn3_autoreleasing_none
	.p2align	4
_fn3_autoreleasing_none:
	push	rbp
	mov	rbp, rsp
	sub	rsp, 16
	mov	dword ptr [rbp - 8], 0
	mov	byte ptr [rbp - 1], 0
	lea	rdx, [rbp - 8]
	lea	rcx, [rbp - 1]
	xor	esi, esi
	call	_CMAudioDeviceClockGetAudioDevice
	add	rsp, 16
	pop	rbp
	ret

	.globl	_fn4_autoreleasing_some_none
	.p2align	4
_fn4_autoreleasing_some_none:
Lfunc_begin1:
	push	rbp
	mov	rbp, rsp
	push	rbx
	sub	rsp, 24
	mov	qword ptr [rbp - 24], 0
	mov	dword ptr [rbp - 28], 0
	mov	byte ptr [rbp - 9], 0
Ltmp6:
	lea	rsi, [rbp - 24]
	lea	rdx, [rbp - 28]
	lea	rcx, [rbp - 9]
	call	_CMAudioDeviceClockGetAudioDevice
Ltmp7:
	mov	rdi, qword ptr [rbp - 24]
	test	rdi, rdi
	je	LBB3_2
Ltmp12:
	call	_CFRetain
Ltmp13:
	mov	rax, qword ptr [rbp - 24]
	add	rsp, 24
	pop	rbx
	pop	rbp
	ret
LBB3_2:
	xor	eax, eax
	add	rsp, 24
	pop	rbx
	pop	rbp
	ret
LBB3_9:
Ltmp14:
	mov	rbx, rax
	jmp	LBB3_10
LBB3_3:
Ltmp8:
	mov	rbx, rax
	mov	rdi, qword ptr [rbp - 24]
	test	rdi, rdi
	je	LBB3_12
Ltmp9:
	call	_CFRetain
Ltmp10:
LBB3_10:
	mov	rdi, qword ptr [rbp - 24]
	test	rdi, rdi
	je	LBB3_12
Ltmp15:
	call	_CFRelease
Ltmp16:
LBB3_12:
	mov	rdi, rbx
	call	__Unwind_Resume
LBB3_13:
Ltmp17:
	call	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
LBB3_8:
Ltmp11:
	call	SYM(core[CRATE_ID]::panicking::panic_in_cleanup, 0)
Lfunc_end1:
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table3:
Lexception1:
	.byte	255
	.byte	155
	.uleb128 Lttbase1-Lttbaseref1
Lttbaseref1:
	.byte	1
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
	.uleb128 Ltmp6-Lfunc_begin1
	.uleb128 Ltmp7-Ltmp6
	.uleb128 Ltmp8-Lfunc_begin1
	.byte	0
	.uleb128 Ltmp12-Lfunc_begin1
	.uleb128 Ltmp13-Ltmp12
	.uleb128 Ltmp14-Lfunc_begin1
	.byte	0
	.uleb128 Ltmp9-Lfunc_begin1
	.uleb128 Ltmp10-Ltmp9
	.uleb128 Ltmp11-Lfunc_begin1
	.byte	1
	.uleb128 Ltmp15-Lfunc_begin1
	.uleb128 Ltmp16-Ltmp15
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

.subsections_via_symbols
