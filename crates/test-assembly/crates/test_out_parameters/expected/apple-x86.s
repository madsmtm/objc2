	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.globl	_nonnull_nonnull
	.p2align	4, 0x90
_nonnull_nonnull:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	ebx, dword ptr [ebp + 16]
	mov	edi, dword ptr [ebx]
	mov	dword ptr [esp + 8], ebx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	mov	esi, eax
	mov	eax, dword ptr [ebx]
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	dword ptr [esp], edi
	call	_objc_release
	mov	eax, esi
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.globl	_null_nonnull
	.p2align	4, 0x90
_null_nonnull:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	edi, dword ptr [ebp + 16]
	mov	ecx, dword ptr [ebp + 12]
	mov	eax, dword ptr [ebp + 8]
	test	edi, edi
	je	LBB1_1
	mov	ebx, dword ptr [edi]
	sub	esp, 4
	push	edi
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 16
	mov	esi, eax
	sub	esp, 12
	push	dword ptr [edi]
	call	_objc_retain
	add	esp, 4
	push	ebx
	call	_objc_release
	add	esp, 16
	mov	eax, esi
	add	esp, 12
	jmp	LBB1_2
LBB1_1:
	sub	esp, 4
	push	0
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 28
LBB1_2:
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.globl	_nonnull_null
	.p2align	4, 0x90
_nonnull_null:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	ebx, dword ptr [ebp + 16]
	mov	edi, dword ptr [ebx]
	mov	dword ptr [esp + 8], ebx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	mov	esi, eax
	mov	eax, dword ptr [ebx]
	mov	dword ptr [esp], eax
	call	_objc_retain
	test	edi, edi
	je	LBB2_2
	mov	dword ptr [esp], edi
	call	_objc_release
LBB2_2:
	mov	eax, esi
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.globl	_null_null
	.p2align	4, 0x90
_null_null:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	ebx, dword ptr [ebp + 16]
	mov	ecx, dword ptr [ebp + 12]
	mov	eax, dword ptr [ebp + 8]
	test	ebx, ebx
	je	LBB3_1
	mov	edi, dword ptr [ebx]
	sub	esp, 4
	push	ebx
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 16
	mov	esi, eax
	sub	esp, 12
	push	dword ptr [ebx]
	call	_objc_retain
	add	esp, 16
	test	edi, edi
	je	LBB3_4
	sub	esp, 12
	push	edi
	call	_objc_release
	add	esp, 16
LBB3_4:
	mov	eax, esi
	add	esp, 12
	jmp	LBB3_5
LBB3_1:
	sub	esp, 4
	push	0
	push	ecx
	push	eax
	call	_objc_msgSend
	add	esp, 28
LBB3_5:
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.globl	_two_nonnull_nonnull
	.p2align	4, 0x90
_two_nonnull_nonnull:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 28
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	esi, dword ptr [ebp + 20]
	mov	ebx, dword ptr [ebp + 16]
	mov	edi, dword ptr [ebx]
	mov	edx, dword ptr [esi]
	mov	dword ptr [ebp - 16], edx
	mov	dword ptr [esp + 12], esi
	mov	dword ptr [esp + 8], ebx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	mov	esi, eax
	mov	eax, dword ptr [ebx]
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	dword ptr [esp], edi
	call	_objc_release
	mov	eax, dword ptr [ebp + 20]
	mov	eax, dword ptr [eax]
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	eax, dword ptr [ebp - 16]
	mov	dword ptr [esp], eax
	call	_objc_release
	mov	eax, esi
	add	esp, 28
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret

	.globl	_call_with_none1
	.p2align	4, 0x90
_call_with_none1:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	sub	esp, 4
	push	0
	push	dword ptr [ebp + 12]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 24
	pop	ebp
	ret

	.globl	_call_with_none2
	.p2align	4, 0x90
_call_with_none2:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	sub	esp, 4
	push	0
	push	dword ptr [ebp + 12]
	push	dword ptr [ebp + 8]
	call	_objc_msgSend
	add	esp, 24
	pop	ebp
	ret

	.globl	_call_with_none3
	.p2align	4, 0x90
_call_with_none3:
Lfunc_begin0:
	push	ebp
	mov	ebp, esp
	push	esi
	sub	esp, 20
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 8], 0
Ltmp0:
	lea	edx, [ebp - 8]
	mov	dword ptr [esp + 8], edx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
Ltmp1:
	mov	esi, eax
	mov	eax, dword ptr [ebp - 8]
Ltmp2:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp3:
	mov	edx, dword ptr [ebp - 8]
	mov	eax, esi
	add	esp, 20
	pop	esi
	pop	ebp
	ret
LBB7_3:
Ltmp4:
	mov	esi, eax
	mov	eax, dword ptr [ebp - 8]
	test	eax, eax
	je	LBB7_5
Ltmp5:
	mov	dword ptr [esp], eax
	call	_objc_release
Ltmp6:
LBB7_5:
	mov	dword ptr [esp], esi
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
	push	ebp
	mov	ebp, esp
	push	esi
	sub	esp, 20
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 8], 0
Ltmp8:
	lea	edx, [ebp - 8]
	mov	dword ptr [esp + 8], edx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
Ltmp9:
	mov	esi, eax
	mov	eax, dword ptr [ebp - 8]
Ltmp10:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp11:
	mov	edx, dword ptr [ebp - 8]
	mov	eax, esi
	add	esp, 20
	pop	esi
	pop	ebp
	ret
LBB8_3:
Ltmp12:
	mov	esi, eax
	mov	eax, dword ptr [ebp - 8]
	test	eax, eax
	je	LBB8_5
Ltmp13:
	mov	dword ptr [esp], eax
	call	_objc_release
Ltmp14:
LBB8_5:
	mov	dword ptr [esp], esi
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
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	sub	esp, 16
	mov	edi, dword ptr [ebp + 16]
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
Ltmp16:
	lea	edx, [ebp + 16]
	mov	dword ptr [esp + 8], edx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
Ltmp17:
	mov	esi, eax
	mov	eax, dword ptr [ebp + 16]
Ltmp18:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp19:
Ltmp20:
	mov	dword ptr [esp], edi
	call	_objc_release
Ltmp21:
	mov	edx, dword ptr [ebp + 16]
	mov	eax, esi
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret
LBB9_5:
Ltmp22:
	mov	esi, eax
	mov	eax, dword ptr [ebp + 16]
Ltmp23:
	mov	dword ptr [esp], eax
	call	_objc_release
Ltmp24:
	mov	dword ptr [esp], esi
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
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	sub	esp, 16
	mov	edi, dword ptr [ebp + 16]
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 12], edi
Ltmp26:
	lea	edx, [ebp - 12]
	mov	dword ptr [esp + 8], edx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
Ltmp27:
	mov	esi, eax
	mov	eax, dword ptr [ebp - 12]
Ltmp28:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp29:
Ltmp30:
	mov	dword ptr [esp], edi
	call	_objc_release
Ltmp31:
	mov	edx, dword ptr [ebp - 12]
	mov	eax, esi
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret
LBB10_4:
Ltmp32:
	mov	esi, eax
	mov	eax, dword ptr [ebp - 12]
	test	eax, eax
	je	LBB10_6
Ltmp33:
	mov	dword ptr [esp], eax
	call	_objc_release
Ltmp34:
LBB10_6:
	mov	dword ptr [esp], esi
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
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	sub	esp, 16
	mov	edi, dword ptr [ebp + 16]
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 12], edi
Ltmp36:
	lea	edx, [ebp - 12]
	mov	dword ptr [esp + 8], edx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
Ltmp37:
	mov	esi, eax
	mov	eax, dword ptr [ebp - 12]
Ltmp38:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp39:
Ltmp40:
	mov	dword ptr [esp], edi
	call	_objc_release
Ltmp41:
	mov	edx, dword ptr [ebp - 12]
	mov	eax, esi
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret
LBB11_4:
Ltmp42:
	mov	esi, eax
	mov	eax, dword ptr [ebp - 12]
	test	eax, eax
	je	LBB11_6
Ltmp43:
	mov	dword ptr [esp], eax
	call	_objc_release
Ltmp44:
LBB11_6:
	mov	dword ptr [esp], esi
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

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
L_rust_eh_personality$non_lazy_ptr:
	.indirect_symbol	_rust_eh_personality
	.long	0

.subsections_via_symbols
