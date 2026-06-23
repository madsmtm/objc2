	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_fn1_strong_none
	.p2align	4
_fn1_strong_none:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	sub	esp, 8
	push	0
	push	dword ptr [ebp + 8]
	call	_SecTrustEvaluateWithError
	add	esp, 24
	pop	ebp
	ret

	.globl	_fn2_strong_some_none
	.p2align	4
_fn2_strong_some_none:
Lfunc_begin0:
	push	ebp
	mov	ebp, esp
	push	esi
	sub	esp, 20
	mov	eax, dword ptr [ebp + 8]
	mov	dword ptr [ebp - 8], 0
Ltmp0:
	lea	ecx, [ebp - 8]
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_SecTrustEvaluateWithError
Ltmp1:
	mov	eax, dword ptr [ebp - 8]
	add	esp, 20
	pop	esi
	pop	ebp
	ret
LBB1_2:
Ltmp2:
	mov	esi, eax
	mov	eax, dword ptr [ebp - 8]
	test	eax, eax
	je	LBB1_4
Ltmp3:
	mov	dword ptr [esp], eax
	call	_CFRelease
Ltmp4:
LBB1_4:
	mov	dword ptr [esp], esi
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
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	mov	eax, dword ptr [ebp + 8]
	mov	dword ptr [ebp - 8], 0
	mov	byte ptr [ebp - 1], 0
	lea	ecx, [ebp - 1]
	lea	edx, [ebp - 8]
	push	ecx
	push	edx
	push	0
	push	eax
	call	_CMAudioDeviceClockGetAudioDevice
	add	esp, 24
	pop	ebp
	ret

	.globl	_fn4_autoreleasing_some_none
	.p2align	4
_fn4_autoreleasing_some_none:
Lfunc_begin1:
	push	ebp
	mov	ebp, esp
	push	esi
	sub	esp, 36
	mov	eax, dword ptr [ebp + 8]
	mov	dword ptr [ebp - 12], 0
	mov	dword ptr [ebp - 16], 0
	mov	byte ptr [ebp - 5], 0
Ltmp6:
	lea	ecx, [ebp - 5]
	mov	dword ptr [esp + 12], ecx
	lea	ecx, [ebp - 16]
	mov	dword ptr [esp + 8], ecx
	lea	ecx, [ebp - 12]
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_CMAudioDeviceClockGetAudioDevice
Ltmp7:
	mov	eax, dword ptr [ebp - 12]
	test	eax, eax
	je	LBB3_2
Ltmp12:
	mov	dword ptr [esp], eax
	call	_CFRetain
Ltmp13:
	mov	eax, dword ptr [ebp - 12]
	add	esp, 36
	pop	esi
	pop	ebp
	ret
LBB3_2:
	xor	eax, eax
	add	esp, 36
	pop	esi
	pop	ebp
	ret
LBB3_9:
Ltmp14:
	mov	esi, eax
	jmp	LBB3_10
LBB3_3:
Ltmp8:
	mov	esi, eax
	mov	eax, dword ptr [ebp - 12]
	test	eax, eax
	je	LBB3_12
Ltmp9:
	mov	dword ptr [esp], eax
	call	_CFRetain
Ltmp10:
LBB3_10:
	mov	eax, dword ptr [ebp - 12]
	test	eax, eax
	je	LBB3_12
Ltmp15:
	mov	dword ptr [esp], eax
	call	_CFRelease
Ltmp16:
LBB3_12:
	mov	dword ptr [esp], esi
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

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
L_rust_eh_personality$non_lazy_ptr:
	.indirect_symbol	_rust_eh_personality
	.long	0

.subsections_via_symbols
