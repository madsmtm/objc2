	.section	__TEXT,__text,regular,pure_instructions
	.intel_syntax noprefix
	.p2align	4, 0x90
SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0):
	test	ecx, ecx
	je	LBB0_2
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	mov	esi, edx
	mov	eax, dword ptr [ecx]
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	dword ptr [esp], esi
	call	_objc_release
	add	esp, 4
	pop	esi
	pop	ebp
LBB0_2:
	ret

	.p2align	4, 0x90
SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0):
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	test	ecx, ecx
	je	LBB1_3
	mov	esi, edx
	mov	eax, dword ptr [ecx]
	mov	dword ptr [esp], eax
	call	_objc_retain
	test	esi, esi
	je	LBB1_3
	mov	dword ptr [esp], esi
	call	_objc_release
LBB1_3:
	add	esp, 4
	pop	esi
	pop	ebp
	ret

	.p2align	4, 0x90
SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0):
Lfunc_begin0:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, ecx
	mov	eax, dword ptr [ecx]
	mov	edi, dword ptr [ecx + 4]
	mov	eax, dword ptr [eax]
Ltmp0:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp1:
Ltmp2:
	mov	dword ptr [esp], edi
	call	_objc_release
Ltmp3:
	mov	eax, dword ptr [esi + 8]
	mov	esi, dword ptr [esi + 12]
	mov	eax, dword ptr [eax]
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	dword ptr [esp], esi
	call	_objc_release
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret
LBB2_3:
Ltmp4:
	mov	edi, eax
	mov	eax, dword ptr [esi + 8]
	mov	esi, dword ptr [esi + 12]
	mov	eax, dword ptr [eax]
Ltmp5:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp6:
Ltmp7:
	mov	dword ptr [esp], esi
	call	_objc_release
Ltmp8:
	mov	dword ptr [esp], edi
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
	.p2align	4, 0x90
SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>,)>, 0):
	push	ebp
	mov	ebp, esp
	push	esi
	push	eax
	mov	esi, edx
	mov	eax, dword ptr [ecx]
	mov	dword ptr [esp], eax
	call	_objc_retain
	test	esi, esi
	je	LBB3_2
	mov	dword ptr [esp], esi
	call	_objc_release
LBB3_2:
	add	esp, 4
	pop	esi
	pop	ebp
	ret

	.globl	_nonnull_nonnull
	.p2align	4, 0x90
_nonnull_nonnull:
Lfunc_begin1:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	ebx, dword ptr [ebp + 16]
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	edi, dword ptr [ebx]
Ltmp10:
	mov	dword ptr [esp + 8], ebx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
Ltmp11:
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
LBB4_2:
Ltmp12:
	mov	esi, eax
	mov	eax, dword ptr [ebx]
Ltmp13:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp14:
Ltmp15:
	mov	dword ptr [esp], edi
	call	_objc_release
Ltmp16:
	mov	dword ptr [esp], esi
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
	.p2align	4, 0x90
_null_nonnull:
Lfunc_begin2:
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
	je	LBB5_1
	mov	esi, dword ptr [edi]
	jmp	LBB5_3
LBB5_1:
LBB5_3:
Ltmp18:
	mov	dword ptr [esp + 8], edi
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
Ltmp19:
	test	edi, edi
	je	LBB5_6
	mov	ecx, dword ptr [edi]
	mov	dword ptr [esp], ecx
	mov	edi, eax
	call	_objc_retain
	mov	dword ptr [esp], esi
	call	_objc_release
	mov	eax, edi
LBB5_6:
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB5_8:
Ltmp20:
	mov	ebx, eax
Ltmp21:
	mov	ecx, edi
	mov	edx, esi
	call	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0)
Ltmp22:
	mov	dword ptr [esp], ebx
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
	.p2align	4, 0x90
_nonnull_null:
Lfunc_begin3:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	edi, dword ptr [ebp + 16]
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	esi, dword ptr [edi]
Ltmp24:
	mov	dword ptr [esp + 8], edi
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
Ltmp25:
	mov	ebx, eax
	mov	eax, dword ptr [edi]
	mov	dword ptr [esp], eax
	call	_objc_retain
	test	esi, esi
	je	LBB6_3
	mov	dword ptr [esp], esi
	call	_objc_release
LBB6_3:
	mov	eax, ebx
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB6_5:
Ltmp26:
	mov	ebx, eax
Ltmp27:
	mov	ecx, edi
	mov	edx, esi
	call	SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>,)>, 0)
Ltmp28:
	mov	dword ptr [esp], ebx
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
	.p2align	4, 0x90
_null_null:
Lfunc_begin4:
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
	je	LBB7_1
	mov	esi, dword ptr [edi]
	jmp	LBB7_3
LBB7_1:
LBB7_3:
Ltmp30:
	mov	dword ptr [esp + 8], edi
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
Ltmp31:
	mov	ebx, eax
	test	edi, edi
	je	LBB7_7
	mov	eax, dword ptr [edi]
	mov	dword ptr [esp], eax
	call	_objc_retain
	test	esi, esi
	je	LBB7_7
	mov	dword ptr [esp], esi
	call	_objc_release
LBB7_7:
	mov	eax, ebx
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB7_9:
Ltmp32:
	mov	ebx, eax
Ltmp33:
	mov	ecx, edi
	mov	edx, esi
	call	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0)
Ltmp34:
	mov	dword ptr [esp], ebx
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
	.p2align	4, 0x90
_two_nonnull_nonnull:
Lfunc_begin5:
	push	ebp
	mov	ebp, esp
	push	ebx
	push	edi
	push	esi
	sub	esp, 44
	mov	ebx, dword ptr [ebp + 20]
	mov	edi, dword ptr [ebp + 16]
	mov	eax, dword ptr [ebp + 8]
	mov	esi, dword ptr [edi]
	mov	ecx, dword ptr [ebx]
	mov	dword ptr [ebp - 36], edi
	mov	dword ptr [ebp - 32], esi
	mov	dword ptr [ebp - 28], ebx
	mov	dword ptr [ebp - 16], ecx
	mov	dword ptr [ebp - 24], ecx
	mov	ecx, dword ptr [ebp + 12]
Ltmp36:
	mov	dword ptr [esp + 12], ebx
	mov	dword ptr [esp + 8], edi
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
	mov	dword ptr [ebp - 20], eax
Ltmp37:
	mov	eax, dword ptr [edi]
Ltmp42:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp43:
Ltmp44:
	mov	dword ptr [esp], esi
	call	_objc_release
Ltmp45:
	mov	eax, dword ptr [ebx]
	mov	dword ptr [esp], eax
	call	_objc_retain
	mov	eax, dword ptr [ebp - 16]
	mov	dword ptr [esp], eax
	call	_objc_release
	mov	eax, dword ptr [ebp - 20]
	add	esp, 44
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
LBB8_4:
Ltmp38:
	mov	esi, eax
Ltmp39:
	lea	ecx, [ebp - 36]
	call	SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0)
Ltmp40:
	jmp	LBB8_7
LBB8_8:
Ltmp41:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB8_5:
Ltmp46:
	mov	esi, eax
	mov	eax, dword ptr [ebx]
Ltmp47:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp48:
Ltmp49:
	mov	eax, dword ptr [ebp - 16]
	mov	dword ptr [esp], eax
	call	_objc_release
Ltmp50:
LBB8_7:
	mov	dword ptr [esp], esi
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
Lfunc_begin6:
	push	ebp
	mov	ebp, esp
	push	esi
	sub	esp, 20
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 8], 0
Ltmp52:
	lea	edx, [ebp - 8]
	mov	dword ptr [esp + 8], edx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
Ltmp53:
	mov	esi, eax
	mov	eax, dword ptr [ebp - 8]
Ltmp58:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp59:
	mov	edx, dword ptr [ebp - 8]
	mov	eax, esi
	add	esp, 20
	pop	esi
	pop	ebp
	ret
LBB11_5:
Ltmp60:
	mov	esi, eax
	jmp	LBB11_6
LBB11_3:
Ltmp54:
	mov	esi, eax
	mov	eax, dword ptr [ebp - 8]
Ltmp55:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp56:
LBB11_6:
	mov	eax, dword ptr [ebp - 8]
	test	eax, eax
	je	LBB11_8
Ltmp61:
	mov	dword ptr [esp], eax
	call	_objc_release
Ltmp62:
LBB11_8:
	mov	dword ptr [esp], esi
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
	.p2align	4, 0x90
_call_with_none4:
Lfunc_begin7:
	push	ebp
	mov	ebp, esp
	push	esi
	sub	esp, 20
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 8], 0
Ltmp64:
	lea	edx, [ebp - 8]
	mov	dword ptr [esp + 8], edx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
Ltmp65:
	mov	esi, eax
	mov	eax, dword ptr [ebp - 8]
Ltmp70:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp71:
	mov	edx, dword ptr [ebp - 8]
	mov	eax, esi
	add	esp, 20
	pop	esi
	pop	ebp
	ret
LBB12_5:
Ltmp72:
	mov	esi, eax
	jmp	LBB12_6
LBB12_3:
Ltmp66:
	mov	esi, eax
	mov	eax, dword ptr [ebp - 8]
Ltmp67:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp68:
LBB12_6:
	mov	eax, dword ptr [ebp - 8]
	test	eax, eax
	je	LBB12_8
Ltmp73:
	mov	dword ptr [esp], eax
	call	_objc_release
Ltmp74:
LBB12_8:
	mov	dword ptr [esp], esi
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
	.p2align	4, 0x90
_call_with_some1:
Lfunc_begin8:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	sub	esp, 16
	mov	edi, dword ptr [ebp + 16]
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
Ltmp76:
	lea	edx, [ebp + 16]
	mov	dword ptr [esp + 8], edx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
Ltmp77:
	mov	esi, eax
	mov	eax, dword ptr [ebp + 16]
Ltmp84:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp85:
Ltmp86:
	mov	dword ptr [esp], edi
	call	_objc_release
Ltmp87:
	mov	edx, dword ptr [ebp + 16]
	mov	eax, esi
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret
LBB13_4:
Ltmp78:
	mov	esi, eax
	mov	eax, dword ptr [ebp + 16]
Ltmp79:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp80:
Ltmp81:
	mov	dword ptr [esp], edi
	call	_objc_release
Ltmp82:
	jmp	LBB13_8
LBB13_6:
Ltmp83:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB13_7:
Ltmp88:
	mov	esi, eax
LBB13_8:
	mov	eax, dword ptr [ebp + 16]
Ltmp89:
	mov	dword ptr [esp], eax
	call	_objc_release
Ltmp90:
	mov	dword ptr [esp], esi
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
	.p2align	4, 0x90
_call_with_some2:
Lfunc_begin9:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	sub	esp, 16
	mov	edi, dword ptr [ebp + 16]
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 12], edi
Ltmp92:
	lea	edx, [ebp - 12]
	mov	dword ptr [esp + 8], edx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
Ltmp93:
	mov	esi, eax
	mov	eax, dword ptr [ebp - 12]
Ltmp100:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp101:
Ltmp102:
	mov	dword ptr [esp], edi
	call	_objc_release
Ltmp103:
	mov	edx, dword ptr [ebp - 12]
	mov	eax, esi
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret
LBB14_4:
Ltmp94:
	mov	esi, eax
	mov	eax, dword ptr [ebp - 12]
Ltmp95:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp96:
Ltmp97:
	mov	dword ptr [esp], edi
	call	_objc_release
Ltmp98:
	jmp	LBB14_8
LBB14_6:
Ltmp99:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB14_7:
Ltmp104:
	mov	esi, eax
LBB14_8:
	mov	eax, dword ptr [ebp - 12]
	test	eax, eax
	je	LBB14_10
Ltmp105:
	mov	dword ptr [esp], eax
	call	_objc_release
Ltmp106:
LBB14_10:
	mov	dword ptr [esp], esi
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
	.p2align	4, 0x90
_call_with_some3:
Lfunc_begin10:
	push	ebp
	mov	ebp, esp
	push	edi
	push	esi
	sub	esp, 16
	mov	edi, dword ptr [ebp + 16]
	mov	eax, dword ptr [ebp + 8]
	mov	ecx, dword ptr [ebp + 12]
	mov	dword ptr [ebp - 12], edi
Ltmp108:
	lea	edx, [ebp - 12]
	mov	dword ptr [esp + 8], edx
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], eax
	call	_objc_msgSend
Ltmp109:
	mov	esi, eax
	mov	eax, dword ptr [ebp - 12]
Ltmp116:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp117:
Ltmp118:
	mov	dword ptr [esp], edi
	call	_objc_release
Ltmp119:
	mov	edx, dword ptr [ebp - 12]
	mov	eax, esi
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebp
	ret
LBB15_4:
Ltmp110:
	mov	esi, eax
	mov	eax, dword ptr [ebp - 12]
Ltmp111:
	mov	dword ptr [esp], eax
	call	_objc_retain
Ltmp112:
Ltmp113:
	mov	dword ptr [esp], edi
	call	_objc_release
Ltmp114:
	jmp	LBB15_8
LBB15_6:
Ltmp115:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)
LBB15_7:
Ltmp120:
	mov	esi, eax
LBB15_8:
	mov	eax, dword ptr [ebp - 12]
	test	eax, eax
	je	LBB15_10
Ltmp121:
	mov	dword ptr [esp], eax
	call	_objc_release
Ltmp122:
LBB15_10:
	mov	dword ptr [esp], esi
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

	.section	__IMPORT,__pointers,non_lazy_symbol_pointers
L_rust_eh_personality$non_lazy_ptr:
	.indirect_symbol	_rust_eh_personality
	.long	0

.subsections_via_symbols
