	.text
	.intel_syntax noprefix
	.section	.text.nonnull_nonnull,"ax",@progbits
	.globl	nonnull_nonnull
	.p2align	4, 0x90
	.type	nonnull_nonnull,@function
nonnull_nonnull:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	ebp, dword ptr [esp + 40]
	mov	eax, dword ptr [esp + 32]
	mov	esi, dword ptr [esp + 36]
	call	.L0$pb
.L0$pb:
	pop	ebx
.Ltmp0:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp0-.L0$pb)
	mov	edi, dword ptr [ebp]
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], eax
	call	objc_msg_lookup@PLT
	mov	ecx, dword ptr [esp + 32]
	mov	dword ptr [esp + 8], ebp
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], ecx
	call	eax
	mov	esi, eax
	mov	eax, dword ptr [ebp]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	mov	dword ptr [esp], edi
	call	objc_release@PLT
	mov	eax, esi
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end0:
	.size	nonnull_nonnull, .Lfunc_end0-nonnull_nonnull

	.section	.text.null_nonnull,"ax",@progbits
	.globl	null_nonnull
	.p2align	4, 0x90
	.type	null_nonnull,@function
null_nonnull:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 28
	mov	edi, dword ptr [esp + 56]
	mov	esi, dword ptr [esp + 52]
	mov	ebp, dword ptr [esp + 48]
	call	.L1$pb
.L1$pb:
	pop	ebx
.Ltmp1:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp1-.L1$pb)
	test	edi, edi
	je	.LBB1_1
	mov	eax, dword ptr [edi]
	mov	dword ptr [esp + 24], eax
	jmp	.LBB1_3
.LBB1_1:
.LBB1_3:
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], ebp
	call	objc_msg_lookup@PLT
	mov	dword ptr [esp + 8], edi
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], ebp
	call	eax
	test	edi, edi
	je	.LBB1_5
	mov	ecx, dword ptr [edi]
	mov	esi, eax
	mov	dword ptr [esp], ecx
	call	objc_retain@PLT
	mov	eax, dword ptr [esp + 24]
	mov	dword ptr [esp], eax
	call	objc_release@PLT
	mov	eax, esi
.LBB1_5:
	add	esp, 28
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end1:
	.size	null_nonnull, .Lfunc_end1-null_nonnull

	.section	.text.nonnull_null,"ax",@progbits
	.globl	nonnull_null
	.p2align	4, 0x90
	.type	nonnull_null,@function
nonnull_null:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	ebp, dword ptr [esp + 40]
	mov	eax, dword ptr [esp + 32]
	mov	esi, dword ptr [esp + 36]
	call	.L2$pb
.L2$pb:
	pop	ebx
.Ltmp2:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp2-.L2$pb)
	mov	edi, dword ptr [ebp]
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], eax
	call	objc_msg_lookup@PLT
	mov	ecx, dword ptr [esp + 32]
	mov	dword ptr [esp + 8], ebp
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], ecx
	call	eax
	mov	esi, eax
	mov	eax, dword ptr [ebp]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	test	edi, edi
	je	.LBB2_2
	mov	dword ptr [esp], edi
	call	objc_release@PLT
.LBB2_2:
	mov	eax, esi
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end2:
	.size	nonnull_null, .Lfunc_end2-nonnull_null

	.section	.text.null_null,"ax",@progbits
	.globl	null_null
	.p2align	4, 0x90
	.type	null_null,@function
null_null:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	ebp, dword ptr [esp + 40]
	mov	esi, dword ptr [esp + 36]
	mov	ecx, dword ptr [esp + 32]
	call	.L3$pb
.L3$pb:
	pop	ebx
.Ltmp3:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp3-.L3$pb)
	test	ebp, ebp
	je	.LBB3_1
	mov	edi, dword ptr [ebp]
	jmp	.LBB3_3
.LBB3_1:
.LBB3_3:
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], ecx
	call	objc_msg_lookup@PLT
	mov	ecx, dword ptr [esp + 32]
	mov	dword ptr [esp + 8], ebp
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], ecx
	call	eax
	mov	esi, eax
	test	ebp, ebp
	je	.LBB3_6
	mov	eax, dword ptr [ebp]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	test	edi, edi
	je	.LBB3_6
	mov	dword ptr [esp], edi
	call	objc_release@PLT
.LBB3_6:
	mov	eax, esi
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end3:
	.size	null_null, .Lfunc_end3-null_null

	.section	.text.two_nonnull_nonnull,"ax",@progbits
	.globl	two_nonnull_nonnull
	.p2align	4, 0x90
	.type	two_nonnull_nonnull,@function
two_nonnull_nonnull:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 28
	mov	ebp, dword ptr [esp + 56]
	mov	edi, dword ptr [esp + 60]
	mov	edx, dword ptr [esp + 48]
	mov	esi, dword ptr [esp + 52]
	call	.L4$pb
.L4$pb:
	pop	ebx
.Ltmp4:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp4-.L4$pb)
	mov	eax, dword ptr [ebp]
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], edx
	mov	dword ptr [esp + 20], eax
	mov	eax, dword ptr [edi]
	mov	dword ptr [esp + 24], eax
	call	objc_msg_lookup@PLT
	mov	ecx, dword ptr [esp + 48]
	mov	dword ptr [esp + 12], edi
	mov	dword ptr [esp + 8], ebp
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], ecx
	call	eax
	mov	esi, eax
	mov	eax, dword ptr [ebp]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	mov	eax, dword ptr [esp + 20]
	mov	dword ptr [esp], eax
	call	objc_release@PLT
	mov	eax, dword ptr [esp + 60]
	mov	eax, dword ptr [eax]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	mov	eax, dword ptr [esp + 24]
	mov	dword ptr [esp], eax
	call	objc_release@PLT
	mov	eax, esi
	add	esp, 28
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end4:
	.size	two_nonnull_nonnull, .Lfunc_end4-two_nonnull_nonnull

	.section	.text.call_with_none1,"ax",@progbits
	.globl	call_with_none1
	.p2align	4, 0x90
	.type	call_with_none1,@function
call_with_none1:
	push	ebx
	push	edi
	push	esi
	mov	esi, dword ptr [esp + 16]
	mov	edi, dword ptr [esp + 20]
	call	.L5$pb
.L5$pb:
	pop	ebx
.Ltmp5:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp5-.L5$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 12
	push	0
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end5:
	.size	call_with_none1, .Lfunc_end5-call_with_none1

	.section	.text.call_with_none2,"ax",@progbits
	.globl	call_with_none2
	.p2align	4, 0x90
	.type	call_with_none2,@function
call_with_none2:
	push	ebx
	push	edi
	push	esi
	mov	esi, dword ptr [esp + 16]
	mov	edi, dword ptr [esp + 20]
	call	.L6$pb
.L6$pb:
	pop	ebx
.Ltmp6:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp6-.L6$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 12
	push	0
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end6:
	.size	call_with_none2, .Lfunc_end6-call_with_none2

	.section	.text.call_with_none3,"ax",@progbits
	.globl	call_with_none3
	.p2align	4, 0x90
	.type	call_with_none3,@function
call_with_none3:
.Lfunc_begin0:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	edi, dword ptr [esp + 36]
	mov	esi, dword ptr [esp + 32]
	call	.L7$pb
.L7$pb:
	pop	ebx
	mov	dword ptr [esp + 12], 0
.Ltmp17:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp17-.L7$pb)
.Ltmp7:
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
.Ltmp8:
.Ltmp9:
	lea	ecx, [esp + 12]
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	mov	dword ptr [esp + 8], ecx
	call	eax
.Ltmp10:
	mov	esi, eax
	mov	eax, dword ptr [esp + 12]
.Ltmp11:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp12:
	mov	edx, dword ptr [esp + 12]
	mov	eax, esi
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB7_4:
.Ltmp13:
	mov	esi, eax
	mov	eax, dword ptr [esp + 12]
	test	eax, eax
	je	.LBB7_6
.Ltmp14:
	mov	dword ptr [esp], eax
	call	objc_release@PLT
.Ltmp15:
.LBB7_6:
	mov	dword ptr [esp], esi
	call	_Unwind_Resume@PLT
.LBB7_7:
.Ltmp16:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
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
	.uleb128 .Ltmp7-.Lfunc_begin0
	.uleb128 .Ltmp12-.Ltmp7
	.uleb128 .Ltmp13-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp14-.Lfunc_begin0
	.uleb128 .Ltmp15-.Ltmp14
	.uleb128 .Ltmp16-.Lfunc_begin0
	.byte	1
	.uleb128 .Ltmp15-.Lfunc_begin0
	.uleb128 .Lfunc_end7-.Ltmp15
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
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	edi, dword ptr [esp + 36]
	mov	esi, dword ptr [esp + 32]
	call	.L8$pb
.L8$pb:
	pop	ebx
	mov	dword ptr [esp + 12], 0
.Ltmp28:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp28-.L8$pb)
.Ltmp18:
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
.Ltmp19:
.Ltmp20:
	lea	ecx, [esp + 12]
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	mov	dword ptr [esp + 8], ecx
	call	eax
.Ltmp21:
	mov	esi, eax
	mov	eax, dword ptr [esp + 12]
.Ltmp22:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp23:
	mov	edx, dword ptr [esp + 12]
	mov	eax, esi
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB8_4:
.Ltmp24:
	mov	esi, eax
	mov	eax, dword ptr [esp + 12]
	test	eax, eax
	je	.LBB8_6
.Ltmp25:
	mov	dword ptr [esp], eax
	call	objc_release@PLT
.Ltmp26:
.LBB8_6:
	mov	dword ptr [esp], esi
	call	_Unwind_Resume@PLT
.LBB8_7:
.Ltmp27:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
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
	.uleb128 .Ltmp18-.Lfunc_begin1
	.uleb128 .Ltmp23-.Ltmp18
	.uleb128 .Ltmp24-.Lfunc_begin1
	.byte	0
	.uleb128 .Ltmp25-.Lfunc_begin1
	.uleb128 .Ltmp26-.Ltmp25
	.uleb128 .Ltmp27-.Lfunc_begin1
	.byte	1
	.uleb128 .Ltmp26-.Lfunc_begin1
	.uleb128 .Lfunc_end8-.Ltmp26
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
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	edi, dword ptr [esp + 40]
	mov	ebp, dword ptr [esp + 36]
	mov	esi, dword ptr [esp + 32]
	call	.L9$pb
.L9$pb:
	pop	ebx
.Ltmp41:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp41-.L9$pb)
.Ltmp29:
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
.Ltmp30:
.Ltmp31:
	lea	ecx, [esp + 40]
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	mov	dword ptr [esp + 8], ecx
	call	eax
.Ltmp32:
	mov	esi, eax
	mov	eax, dword ptr [esp + 40]
.Ltmp33:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp34:
.Ltmp35:
	mov	dword ptr [esp], edi
	call	objc_release@PLT
.Ltmp36:
	mov	edx, dword ptr [esp + 40]
	mov	eax, esi
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB9_6:
.Ltmp37:
	mov	esi, eax
	mov	eax, dword ptr [esp + 40]
.Ltmp38:
	mov	dword ptr [esp], eax
	call	objc_release@PLT
.Ltmp39:
	mov	dword ptr [esp], esi
	call	_Unwind_Resume@PLT
.LBB9_5:
.Ltmp40:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
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
	.uleb128 .Ltmp29-.Lfunc_begin2
	.uleb128 .Ltmp36-.Ltmp29
	.uleb128 .Ltmp37-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp38-.Lfunc_begin2
	.uleb128 .Ltmp39-.Ltmp38
	.uleb128 .Ltmp40-.Lfunc_begin2
	.byte	1
	.uleb128 .Ltmp39-.Lfunc_begin2
	.uleb128 .Lfunc_end9-.Ltmp39
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
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 28
	mov	edi, dword ptr [esp + 56]
	mov	ebp, dword ptr [esp + 52]
	mov	esi, dword ptr [esp + 48]
	call	.L10$pb
.L10$pb:
	pop	ebx
.Ltmp54:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp54-.L10$pb)
	mov	dword ptr [esp + 24], edi
.Ltmp42:
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
.Ltmp43:
.Ltmp44:
	lea	ecx, [esp + 24]
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	mov	dword ptr [esp + 8], ecx
	call	eax
.Ltmp45:
	mov	esi, eax
	mov	eax, dword ptr [esp + 24]
.Ltmp46:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp47:
.Ltmp48:
	mov	dword ptr [esp], edi
	call	objc_release@PLT
.Ltmp49:
	mov	edx, dword ptr [esp + 24]
	mov	eax, esi
	add	esp, 28
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB10_5:
.Ltmp50:
	mov	esi, eax
	mov	eax, dword ptr [esp + 24]
	test	eax, eax
	je	.LBB10_7
.Ltmp51:
	mov	dword ptr [esp], eax
	call	objc_release@PLT
.Ltmp52:
.LBB10_7:
	mov	dword ptr [esp], esi
	call	_Unwind_Resume@PLT
.LBB10_8:
.Ltmp53:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
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
	.uleb128 .Ltmp42-.Lfunc_begin3
	.uleb128 .Ltmp49-.Ltmp42
	.uleb128 .Ltmp50-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp51-.Lfunc_begin3
	.uleb128 .Ltmp52-.Ltmp51
	.uleb128 .Ltmp53-.Lfunc_begin3
	.byte	1
	.uleb128 .Ltmp52-.Lfunc_begin3
	.uleb128 .Lfunc_end10-.Ltmp52
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
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 28
	mov	edi, dword ptr [esp + 56]
	mov	ebp, dword ptr [esp + 52]
	mov	esi, dword ptr [esp + 48]
	call	.L11$pb
.L11$pb:
	pop	ebx
.Ltmp67:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp67-.L11$pb)
	mov	dword ptr [esp + 24], edi
.Ltmp55:
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
.Ltmp56:
.Ltmp57:
	lea	ecx, [esp + 24]
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	mov	dword ptr [esp + 8], ecx
	call	eax
.Ltmp58:
	mov	esi, eax
	mov	eax, dword ptr [esp + 24]
.Ltmp59:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp60:
.Ltmp61:
	mov	dword ptr [esp], edi
	call	objc_release@PLT
.Ltmp62:
	mov	edx, dword ptr [esp + 24]
	mov	eax, esi
	add	esp, 28
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB11_5:
.Ltmp63:
	mov	esi, eax
	mov	eax, dword ptr [esp + 24]
	test	eax, eax
	je	.LBB11_7
.Ltmp64:
	mov	dword ptr [esp], eax
	call	objc_release@PLT
.Ltmp65:
.LBB11_7:
	mov	dword ptr [esp], esi
	call	_Unwind_Resume@PLT
.LBB11_8:
.Ltmp66:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
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
	.uleb128 .Ltmp55-.Lfunc_begin4
	.uleb128 .Ltmp62-.Ltmp55
	.uleb128 .Ltmp63-.Lfunc_begin4
	.byte	0
	.uleb128 .Ltmp64-.Lfunc_begin4
	.uleb128 .Ltmp65-.Ltmp64
	.uleb128 .Ltmp66-.Lfunc_begin4
	.byte	1
	.uleb128 .Ltmp65-.Lfunc_begin4
	.uleb128 .Lfunc_end11-.Ltmp65
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
	.p2align	2, 0x0
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 4
DW.ref.rust_eh_personality:
	.long	rust_eh_personality
	.section	".note.GNU-stack","",@progbits
