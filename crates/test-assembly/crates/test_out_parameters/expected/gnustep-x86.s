	.text
	.intel_syntax noprefix
	.section	.text.SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0),"ax",@progbits
	.p2align	4, 0x90
	.type	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0),@function
SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0):
	push	ebx
	push	esi
	push	eax
	call	.L0$pb
.L0$pb:
	pop	ebx
.Ltmp0:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp0-.L0$pb)
	test	ecx, ecx
	je	.LBB0_2
	mov	eax, dword ptr [ecx]
	mov	esi, edx
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	mov	dword ptr [esp], esi
	call	objc_release@PLT
.LBB0_2:
	add	esp, 4
	pop	esi
	pop	ebx
	ret
.Lfunc_end0:
	.size	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0), .Lfunc_end0-SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0)

	.section	.text.SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0),"ax",@progbits
	.p2align	4, 0x90
	.type	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0),@function
SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0):
	push	ebx
	push	esi
	push	eax
	call	.L1$pb
.L1$pb:
	pop	ebx
.Ltmp1:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp1-.L1$pb)
	test	ecx, ecx
	je	.LBB1_3
	mov	eax, dword ptr [ecx]
	mov	esi, edx
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	test	esi, esi
	je	.LBB1_3
	mov	dword ptr [esp], esi
	call	objc_release@PLT
.LBB1_3:
	add	esp, 4
	pop	esi
	pop	ebx
	ret
.Lfunc_end1:
	.size	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0), .Lfunc_end1-SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0)

	.section	.text.SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0),"ax",@progbits
	.p2align	4, 0x90
	.type	SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0),@function
SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0):
.Lfunc_begin0:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	eax, dword ptr [ecx]
	mov	edi, dword ptr [ecx + 4]
	call	.L2$pb
.L2$pb:
	pop	ebx
	mov	esi, ecx
.Ltmp12:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp12-.L2$pb)
	mov	eax, dword ptr [eax]
.Ltmp2:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp3:
.Ltmp4:
	mov	dword ptr [esp], edi
	call	objc_release@PLT
.Ltmp5:
	mov	eax, dword ptr [esi + 8]
	mov	esi, dword ptr [esi + 12]
	mov	eax, dword ptr [eax]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	mov	dword ptr [esp], esi
	call	objc_release@PLT
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB2_3:
.Ltmp6:
	mov	edi, eax
	mov	eax, dword ptr [esi + 8]
	mov	esi, dword ptr [esi + 12]
	mov	eax, dword ptr [eax]
.Ltmp7:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp8:
.Ltmp9:
	mov	dword ptr [esp], esi
	call	objc_release@PLT
.Ltmp10:
	mov	dword ptr [esp], edi
	call	_Unwind_Resume@PLT
.LBB2_6:
.Ltmp11:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
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
	.uleb128 .Ltmp2-.Lfunc_begin0
	.uleb128 .Ltmp5-.Ltmp2
	.uleb128 .Ltmp6-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp5-.Lfunc_begin0
	.uleb128 .Ltmp7-.Ltmp5
	.byte	0
	.byte	0
	.uleb128 .Ltmp7-.Lfunc_begin0
	.uleb128 .Ltmp10-.Ltmp7
	.uleb128 .Ltmp11-.Lfunc_begin0
	.byte	1
	.uleb128 .Ltmp10-.Lfunc_begin0
	.uleb128 .Lfunc_end2-.Ltmp10
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
	push	ebx
	push	esi
	push	eax
	mov	eax, dword ptr [ecx]
	call	.L3$pb
.L3$pb:
	pop	ebx
	mov	esi, edx
.Ltmp13:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp13-.L3$pb)
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	test	esi, esi
	je	.LBB3_2
	mov	dword ptr [esp], esi
	call	objc_release@PLT
.LBB3_2:
	add	esp, 4
	pop	esi
	pop	ebx
	ret
.Lfunc_end3:
	.size	SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>,)>, 0), .Lfunc_end3-SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>,)>, 0)

	.section	.text.nonnull_nonnull,"ax",@progbits
	.globl	nonnull_nonnull
	.p2align	4, 0x90
	.type	nonnull_nonnull,@function
nonnull_nonnull:
.Lfunc_begin1:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 28
	mov	ebp, dword ptr [esp + 56]
	mov	edi, dword ptr [esp + 52]
	mov	esi, dword ptr [esp + 48]
	call	.L4$pb
.L4$pb:
	pop	ebx
.Ltmp24:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp24-.L4$pb)
	mov	eax, dword ptr [ebp]
	mov	dword ptr [esp + 24], eax
.Ltmp14:
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
.Ltmp15:
.Ltmp16:
	mov	dword ptr [esp + 8], ebp
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	eax
.Ltmp17:
	mov	esi, eax
	mov	eax, dword ptr [ebp]
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
.LBB4_3:
.Ltmp18:
	mov	esi, eax
	mov	eax, dword ptr [ebp]
.Ltmp19:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp20:
.Ltmp21:
	mov	eax, dword ptr [esp + 24]
	mov	dword ptr [esp], eax
	call	objc_release@PLT
.Ltmp22:
	mov	dword ptr [esp], esi
	call	_Unwind_Resume@PLT
.LBB4_6:
.Ltmp23:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
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
	.uleb128 .Ltmp14-.Lfunc_begin1
	.uleb128 .Ltmp17-.Ltmp14
	.uleb128 .Ltmp18-.Lfunc_begin1
	.byte	0
	.uleb128 .Ltmp17-.Lfunc_begin1
	.uleb128 .Ltmp19-.Ltmp17
	.byte	0
	.byte	0
	.uleb128 .Ltmp19-.Lfunc_begin1
	.uleb128 .Ltmp22-.Ltmp19
	.uleb128 .Ltmp23-.Lfunc_begin1
	.byte	1
	.uleb128 .Ltmp22-.Lfunc_begin1
	.uleb128 .Lfunc_end4-.Ltmp22
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
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 28
	mov	edi, dword ptr [esp + 56]
	mov	esi, dword ptr [esp + 52]
	mov	ebp, dword ptr [esp + 48]
	call	.L5$pb
.L5$pb:
	pop	ebx
.Ltmp33:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp33-.L5$pb)
	test	edi, edi
	je	.LBB5_1
	mov	eax, dword ptr [edi]
	mov	dword ptr [esp + 24], eax
	jmp	.LBB5_3
.LBB5_1:
.LBB5_3:
.Ltmp25:
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], ebp
	call	objc_msg_lookup@PLT
.Ltmp26:
.Ltmp27:
	mov	dword ptr [esp + 8], edi
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], ebp
	call	eax
.Ltmp28:
	test	edi, edi
	je	.LBB5_7
	mov	ecx, dword ptr [edi]
	mov	esi, eax
	mov	dword ptr [esp], ecx
	call	objc_retain@PLT
	mov	eax, dword ptr [esp + 24]
	mov	dword ptr [esp], eax
	call	objc_release@PLT
	mov	eax, esi
.LBB5_7:
	add	esp, 28
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB5_9:
.Ltmp29:
	mov	ebp, eax
.Ltmp30:
	mov	edx, dword ptr [esp + 24]
	mov	ecx, edi
	call	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0)
.Ltmp31:
	mov	dword ptr [esp], ebp
	call	_Unwind_Resume@PLT
.LBB5_8:
.Ltmp32:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
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
	.uleb128 .Ltmp25-.Lfunc_begin2
	.uleb128 .Ltmp28-.Ltmp25
	.uleb128 .Ltmp29-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp28-.Lfunc_begin2
	.uleb128 .Ltmp30-.Ltmp28
	.byte	0
	.byte	0
	.uleb128 .Ltmp30-.Lfunc_begin2
	.uleb128 .Ltmp31-.Ltmp30
	.uleb128 .Ltmp32-.Lfunc_begin2
	.byte	1
	.uleb128 .Ltmp31-.Lfunc_begin2
	.uleb128 .Lfunc_end5-.Ltmp31
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
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	eax, dword ptr [esp + 40]
	mov	edi, dword ptr [esp + 36]
	mov	ebp, dword ptr [esp + 32]
	call	.L6$pb
.L6$pb:
	pop	ebx
.Ltmp42:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp42-.L6$pb)
	mov	esi, dword ptr [eax]
.Ltmp34:
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], ebp
	call	objc_msg_lookup@PLT
.Ltmp35:
.Ltmp36:
	mov	ecx, dword ptr [esp + 40]
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], ebp
	mov	dword ptr [esp + 8], ecx
	call	eax
.Ltmp37:
	mov	ebp, eax
	mov	eax, dword ptr [esp + 40]
	mov	eax, dword ptr [eax]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	test	esi, esi
	je	.LBB6_4
	mov	dword ptr [esp], esi
	call	objc_release@PLT
.LBB6_4:
	mov	eax, ebp
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB6_6:
.Ltmp38:
	mov	ebp, eax
.Ltmp39:
	mov	ecx, dword ptr [esp + 40]
	mov	edx, esi
	call	SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>,)>, 0)
.Ltmp40:
	mov	dword ptr [esp], ebp
	call	_Unwind_Resume@PLT
.LBB6_5:
.Ltmp41:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
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
	.uleb128 .Ltmp34-.Lfunc_begin3
	.uleb128 .Ltmp37-.Ltmp34
	.uleb128 .Ltmp38-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp37-.Lfunc_begin3
	.uleb128 .Ltmp39-.Ltmp37
	.byte	0
	.byte	0
	.uleb128 .Ltmp39-.Lfunc_begin3
	.uleb128 .Ltmp40-.Ltmp39
	.uleb128 .Ltmp41-.Lfunc_begin3
	.byte	1
	.uleb128 .Ltmp40-.Lfunc_begin3
	.uleb128 .Lfunc_end6-.Ltmp40
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
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 28
	mov	edi, dword ptr [esp + 56]
	mov	esi, dword ptr [esp + 52]
	mov	ebp, dword ptr [esp + 48]
	call	.L7$pb
.L7$pb:
	pop	ebx
.Ltmp51:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp51-.L7$pb)
	test	edi, edi
	je	.LBB7_1
	mov	eax, dword ptr [edi]
	mov	dword ptr [esp + 24], eax
	jmp	.LBB7_3
.LBB7_1:
.LBB7_3:
.Ltmp43:
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], ebp
	call	objc_msg_lookup@PLT
.Ltmp44:
.Ltmp45:
	mov	dword ptr [esp + 8], edi
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], ebp
	call	eax
.Ltmp46:
	mov	ebp, eax
	test	edi, edi
	je	.LBB7_8
	mov	eax, dword ptr [edi]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	cmp	dword ptr [esp + 24], 0
	je	.LBB7_8
	mov	eax, dword ptr [esp + 24]
	mov	dword ptr [esp], eax
	call	objc_release@PLT
.LBB7_8:
	mov	eax, ebp
	add	esp, 28
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB7_10:
.Ltmp47:
	mov	ebp, eax
.Ltmp48:
	mov	edx, dword ptr [esp + 24]
	mov	ecx, edi
	call	SYM(core[CRATE_ID]::ptr::drop_in_place::<(core[CRATE_ID]::option::Option<objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDropNullable<objc2[CRATE_ID]::runtime::nsobject::NSObject>>,)>, 0)
.Ltmp49:
	mov	dword ptr [esp], ebp
	call	_Unwind_Resume@PLT
.LBB7_9:
.Ltmp50:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
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
	.uleb128 .Ltmp43-.Lfunc_begin4
	.uleb128 .Ltmp46-.Ltmp43
	.uleb128 .Ltmp47-.Lfunc_begin4
	.byte	0
	.uleb128 .Ltmp46-.Lfunc_begin4
	.uleb128 .Ltmp48-.Ltmp46
	.byte	0
	.byte	0
	.uleb128 .Ltmp48-.Lfunc_begin4
	.uleb128 .Ltmp49-.Ltmp48
	.uleb128 .Ltmp50-.Lfunc_begin4
	.byte	1
	.uleb128 .Ltmp49-.Lfunc_begin4
	.uleb128 .Lfunc_end7-.Ltmp49
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
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 44
	mov	ebp, dword ptr [esp + 72]
	mov	esi, dword ptr [esp + 76]
	mov	edx, dword ptr [esp + 68]
	mov	ecx, dword ptr [esp + 64]
	call	.L8$pb
.L8$pb:
	pop	ebx
.Ltmp70:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp70-.L8$pb)
	mov	eax, dword ptr [esi]
	mov	edi, dword ptr [ebp]
	mov	dword ptr [esp + 28], ebp
	mov	dword ptr [esp + 32], edi
	mov	dword ptr [esp + 36], esi
	mov	dword ptr [esp + 20], eax
	mov	dword ptr [esp + 40], eax
.Ltmp52:
	mov	dword ptr [esp + 4], edx
	mov	dword ptr [esp], ecx
	call	objc_msg_lookup@PLT
.Ltmp53:
.Ltmp54:
	mov	ecx, dword ptr [esp + 68]
	mov	edx, dword ptr [esp + 64]
	mov	dword ptr [esp + 12], esi
	mov	dword ptr [esp + 8], ebp
	mov	dword ptr [esp + 4], ecx
	mov	dword ptr [esp], edx
	call	eax
	mov	dword ptr [esp + 24], eax
.Ltmp55:
	mov	eax, dword ptr [ebp]
.Ltmp60:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp61:
.Ltmp62:
	mov	dword ptr [esp], edi
	call	objc_release@PLT
.Ltmp63:
	mov	eax, dword ptr [esi]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	mov	eax, dword ptr [esp + 20]
	mov	dword ptr [esp], eax
	call	objc_release@PLT
	mov	eax, dword ptr [esp + 24]
	add	esp, 44
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB8_6:
.Ltmp64:
	mov	edi, eax
	mov	eax, dword ptr [esi]
.Ltmp65:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp66:
.Ltmp67:
	mov	eax, dword ptr [esp + 20]
	mov	dword ptr [esp], eax
	call	objc_release@PLT
.Ltmp68:
	jmp	.LBB8_8
.LBB8_10:
.Ltmp69:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
.LBB8_5:
.Ltmp56:
	mov	edi, eax
.Ltmp57:
	lea	ecx, [esp + 28]
	call	SYM(core[CRATE_ID]::ptr::drop_in_place::<(objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>, objc2[CRATE_ID]::__macro_helpers::writeback::WritebackOnDrop<objc2[CRATE_ID]::runtime::nsobject::NSObject>)>, 0)
.Ltmp58:
.LBB8_8:
	mov	dword ptr [esp], edi
	call	_Unwind_Resume@PLT
.LBB8_9:
.Ltmp59:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
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
	.uleb128 .Ltmp52-.Lfunc_begin5
	.uleb128 .Ltmp55-.Ltmp52
	.uleb128 .Ltmp56-.Lfunc_begin5
	.byte	0
	.uleb128 .Ltmp60-.Lfunc_begin5
	.uleb128 .Ltmp63-.Ltmp60
	.uleb128 .Ltmp64-.Lfunc_begin5
	.byte	0
	.uleb128 .Ltmp63-.Lfunc_begin5
	.uleb128 .Ltmp65-.Ltmp63
	.byte	0
	.byte	0
	.uleb128 .Ltmp65-.Lfunc_begin5
	.uleb128 .Ltmp68-.Ltmp65
	.uleb128 .Ltmp69-.Lfunc_begin5
	.byte	1
	.uleb128 .Ltmp57-.Lfunc_begin5
	.uleb128 .Ltmp58-.Ltmp57
	.uleb128 .Ltmp59-.Lfunc_begin5
	.byte	1
	.uleb128 .Ltmp58-.Lfunc_begin5
	.uleb128 .Lfunc_end8-.Ltmp58
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
	push	ebx
	push	edi
	push	esi
	mov	esi, dword ptr [esp + 16]
	mov	edi, dword ptr [esp + 20]
	call	.L9$pb
.L9$pb:
	pop	ebx
.Ltmp71:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp71-.L9$pb)
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
.Lfunc_end9:
	.size	call_with_none1, .Lfunc_end9-call_with_none1

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
	call	.L10$pb
.L10$pb:
	pop	ebx
.Ltmp72:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp72-.L10$pb)
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
.Lfunc_end10:
	.size	call_with_none2, .Lfunc_end10-call_with_none2

	.section	.text.call_with_none3,"ax",@progbits
	.globl	call_with_none3
	.p2align	4, 0x90
	.type	call_with_none3,@function
call_with_none3:
.Lfunc_begin6:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	edi, dword ptr [esp + 36]
	mov	esi, dword ptr [esp + 32]
	call	.L11$pb
.L11$pb:
	pop	ebx
	mov	dword ptr [esp + 12], 0
.Ltmp87:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp87-.L11$pb)
.Ltmp73:
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
.Ltmp74:
.Ltmp75:
	lea	ecx, [esp + 12]
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	mov	dword ptr [esp + 8], ecx
	call	eax
.Ltmp76:
	mov	esi, eax
	mov	eax, dword ptr [esp + 12]
.Ltmp81:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp82:
	mov	edx, dword ptr [esp + 12]
	mov	eax, esi
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB11_6:
.Ltmp83:
	mov	esi, eax
	jmp	.LBB11_7
.LBB11_4:
.Ltmp77:
	mov	esi, eax
	mov	eax, dword ptr [esp + 12]
.Ltmp78:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp79:
.LBB11_7:
	mov	eax, dword ptr [esp + 12]
	test	eax, eax
	je	.LBB11_9
.Ltmp84:
	mov	dword ptr [esp], eax
	call	objc_release@PLT
.Ltmp85:
.LBB11_9:
	mov	dword ptr [esp], esi
	call	_Unwind_Resume@PLT
.LBB11_10:
.Ltmp86:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
.LBB11_5:
.Ltmp80:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
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
	.uleb128 .Ltmp73-.Lfunc_begin6
	.uleb128 .Ltmp76-.Ltmp73
	.uleb128 .Ltmp77-.Lfunc_begin6
	.byte	0
	.uleb128 .Ltmp81-.Lfunc_begin6
	.uleb128 .Ltmp82-.Ltmp81
	.uleb128 .Ltmp83-.Lfunc_begin6
	.byte	0
	.uleb128 .Ltmp78-.Lfunc_begin6
	.uleb128 .Ltmp79-.Ltmp78
	.uleb128 .Ltmp80-.Lfunc_begin6
	.byte	1
	.uleb128 .Ltmp84-.Lfunc_begin6
	.uleb128 .Ltmp85-.Ltmp84
	.uleb128 .Ltmp86-.Lfunc_begin6
	.byte	1
	.uleb128 .Ltmp85-.Lfunc_begin6
	.uleb128 .Lfunc_end11-.Ltmp85
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
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	edi, dword ptr [esp + 36]
	mov	esi, dword ptr [esp + 32]
	call	.L12$pb
.L12$pb:
	pop	ebx
	mov	dword ptr [esp + 12], 0
.Ltmp102:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp102-.L12$pb)
.Ltmp88:
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
.Ltmp89:
.Ltmp90:
	lea	ecx, [esp + 12]
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	mov	dword ptr [esp + 8], ecx
	call	eax
.Ltmp91:
	mov	esi, eax
	mov	eax, dword ptr [esp + 12]
.Ltmp96:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp97:
	mov	edx, dword ptr [esp + 12]
	mov	eax, esi
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB12_6:
.Ltmp98:
	mov	esi, eax
	jmp	.LBB12_7
.LBB12_4:
.Ltmp92:
	mov	esi, eax
	mov	eax, dword ptr [esp + 12]
.Ltmp93:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp94:
.LBB12_7:
	mov	eax, dword ptr [esp + 12]
	test	eax, eax
	je	.LBB12_9
.Ltmp99:
	mov	dword ptr [esp], eax
	call	objc_release@PLT
.Ltmp100:
.LBB12_9:
	mov	dword ptr [esp], esi
	call	_Unwind_Resume@PLT
.LBB12_10:
.Ltmp101:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
.LBB12_5:
.Ltmp95:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
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
	.uleb128 .Ltmp88-.Lfunc_begin7
	.uleb128 .Ltmp91-.Ltmp88
	.uleb128 .Ltmp92-.Lfunc_begin7
	.byte	0
	.uleb128 .Ltmp96-.Lfunc_begin7
	.uleb128 .Ltmp97-.Ltmp96
	.uleb128 .Ltmp98-.Lfunc_begin7
	.byte	0
	.uleb128 .Ltmp93-.Lfunc_begin7
	.uleb128 .Ltmp94-.Ltmp93
	.uleb128 .Ltmp95-.Lfunc_begin7
	.byte	1
	.uleb128 .Ltmp99-.Lfunc_begin7
	.uleb128 .Ltmp100-.Ltmp99
	.uleb128 .Ltmp101-.Lfunc_begin7
	.byte	1
	.uleb128 .Ltmp100-.Lfunc_begin7
	.uleb128 .Lfunc_end12-.Ltmp100
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
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	edi, dword ptr [esp + 40]
	mov	ebp, dword ptr [esp + 36]
	mov	esi, dword ptr [esp + 32]
	call	.L13$pb
.L13$pb:
	pop	ebx
.Ltmp121:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp121-.L13$pb)
.Ltmp103:
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
.Ltmp104:
.Ltmp105:
	lea	ecx, [esp + 40]
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	mov	dword ptr [esp + 8], ecx
	call	eax
.Ltmp106:
	mov	esi, eax
	mov	eax, dword ptr [esp + 40]
.Ltmp113:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp114:
.Ltmp115:
	mov	dword ptr [esp], edi
	call	objc_release@PLT
.Ltmp116:
	mov	edx, dword ptr [esp + 40]
	mov	eax, esi
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB13_8:
.Ltmp117:
	mov	esi, eax
	jmp	.LBB13_9
.LBB13_5:
.Ltmp107:
	mov	esi, eax
	mov	eax, dword ptr [esp + 40]
.Ltmp108:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp109:
.Ltmp110:
	mov	dword ptr [esp], edi
	call	objc_release@PLT
.Ltmp111:
.LBB13_9:
	mov	eax, dword ptr [esp + 40]
.Ltmp118:
	mov	dword ptr [esp], eax
	call	objc_release@PLT
.Ltmp119:
	mov	dword ptr [esp], esi
	call	_Unwind_Resume@PLT
.LBB13_11:
.Ltmp120:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
.LBB13_7:
.Ltmp112:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
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
	.uleb128 .Ltmp103-.Lfunc_begin8
	.uleb128 .Ltmp106-.Ltmp103
	.uleb128 .Ltmp107-.Lfunc_begin8
	.byte	0
	.uleb128 .Ltmp113-.Lfunc_begin8
	.uleb128 .Ltmp116-.Ltmp113
	.uleb128 .Ltmp117-.Lfunc_begin8
	.byte	0
	.uleb128 .Ltmp108-.Lfunc_begin8
	.uleb128 .Ltmp111-.Ltmp108
	.uleb128 .Ltmp112-.Lfunc_begin8
	.byte	1
	.uleb128 .Ltmp118-.Lfunc_begin8
	.uleb128 .Ltmp119-.Ltmp118
	.uleb128 .Ltmp120-.Lfunc_begin8
	.byte	1
	.uleb128 .Ltmp119-.Lfunc_begin8
	.uleb128 .Lfunc_end13-.Ltmp119
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
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 28
	mov	edi, dword ptr [esp + 56]
	mov	ebp, dword ptr [esp + 52]
	mov	esi, dword ptr [esp + 48]
	call	.L14$pb
.L14$pb:
	pop	ebx
.Ltmp140:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp140-.L14$pb)
	mov	dword ptr [esp + 24], edi
.Ltmp122:
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
.Ltmp123:
.Ltmp124:
	lea	ecx, [esp + 24]
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	mov	dword ptr [esp + 8], ecx
	call	eax
.Ltmp125:
	mov	esi, eax
	mov	eax, dword ptr [esp + 24]
.Ltmp132:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp133:
.Ltmp134:
	mov	dword ptr [esp], edi
	call	objc_release@PLT
.Ltmp135:
	mov	edx, dword ptr [esp + 24]
	mov	eax, esi
	add	esp, 28
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB14_8:
.Ltmp136:
	mov	esi, eax
	jmp	.LBB14_9
.LBB14_5:
.Ltmp126:
	mov	esi, eax
	mov	eax, dword ptr [esp + 24]
.Ltmp127:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp128:
.Ltmp129:
	mov	dword ptr [esp], edi
	call	objc_release@PLT
.Ltmp130:
.LBB14_9:
	mov	eax, dword ptr [esp + 24]
	test	eax, eax
	je	.LBB14_11
.Ltmp137:
	mov	dword ptr [esp], eax
	call	objc_release@PLT
.Ltmp138:
.LBB14_11:
	mov	dword ptr [esp], esi
	call	_Unwind_Resume@PLT
.LBB14_12:
.Ltmp139:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
.LBB14_7:
.Ltmp131:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
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
	.uleb128 .Ltmp122-.Lfunc_begin9
	.uleb128 .Ltmp125-.Ltmp122
	.uleb128 .Ltmp126-.Lfunc_begin9
	.byte	0
	.uleb128 .Ltmp132-.Lfunc_begin9
	.uleb128 .Ltmp135-.Ltmp132
	.uleb128 .Ltmp136-.Lfunc_begin9
	.byte	0
	.uleb128 .Ltmp127-.Lfunc_begin9
	.uleb128 .Ltmp130-.Ltmp127
	.uleb128 .Ltmp131-.Lfunc_begin9
	.byte	1
	.uleb128 .Ltmp137-.Lfunc_begin9
	.uleb128 .Ltmp138-.Ltmp137
	.uleb128 .Ltmp139-.Lfunc_begin9
	.byte	1
	.uleb128 .Ltmp138-.Lfunc_begin9
	.uleb128 .Lfunc_end14-.Ltmp138
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
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 28
	mov	edi, dword ptr [esp + 56]
	mov	ebp, dword ptr [esp + 52]
	mov	esi, dword ptr [esp + 48]
	call	.L15$pb
.L15$pb:
	pop	ebx
.Ltmp159:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp159-.L15$pb)
	mov	dword ptr [esp + 24], edi
.Ltmp141:
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
.Ltmp142:
.Ltmp143:
	lea	ecx, [esp + 24]
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	mov	dword ptr [esp + 8], ecx
	call	eax
.Ltmp144:
	mov	esi, eax
	mov	eax, dword ptr [esp + 24]
.Ltmp151:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp152:
.Ltmp153:
	mov	dword ptr [esp], edi
	call	objc_release@PLT
.Ltmp154:
	mov	edx, dword ptr [esp + 24]
	mov	eax, esi
	add	esp, 28
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB15_8:
.Ltmp155:
	mov	esi, eax
	jmp	.LBB15_9
.LBB15_5:
.Ltmp145:
	mov	esi, eax
	mov	eax, dword ptr [esp + 24]
.Ltmp146:
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
.Ltmp147:
.Ltmp148:
	mov	dword ptr [esp], edi
	call	objc_release@PLT
.Ltmp149:
.LBB15_9:
	mov	eax, dword ptr [esp + 24]
	test	eax, eax
	je	.LBB15_11
.Ltmp156:
	mov	dword ptr [esp], eax
	call	objc_release@PLT
.Ltmp157:
.LBB15_11:
	mov	dword ptr [esp], esi
	call	_Unwind_Resume@PLT
.LBB15_12:
.Ltmp158:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
.LBB15_7:
.Ltmp150:
	call	SYM(core::panicking::panic_in_cleanup::GENERATED_ID, 0)@PLT
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
	.uleb128 .Ltmp141-.Lfunc_begin10
	.uleb128 .Ltmp144-.Ltmp141
	.uleb128 .Ltmp145-.Lfunc_begin10
	.byte	0
	.uleb128 .Ltmp151-.Lfunc_begin10
	.uleb128 .Ltmp154-.Ltmp151
	.uleb128 .Ltmp155-.Lfunc_begin10
	.byte	0
	.uleb128 .Ltmp146-.Lfunc_begin10
	.uleb128 .Ltmp149-.Ltmp146
	.uleb128 .Ltmp150-.Lfunc_begin10
	.byte	1
	.uleb128 .Ltmp156-.Lfunc_begin10
	.uleb128 .Ltmp157-.Ltmp156
	.uleb128 .Ltmp158-.Lfunc_begin10
	.byte	1
	.uleb128 .Ltmp157-.Lfunc_begin10
	.uleb128 .Lfunc_end15-.Ltmp157
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
	.p2align	2, 0x0
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 4
DW.ref.rust_eh_personality:
	.long	rust_eh_personality
	.section	".note.GNU-stack","",@progbits
