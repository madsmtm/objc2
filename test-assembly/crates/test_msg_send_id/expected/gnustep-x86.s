	.text
	.intel_syntax noprefix
	.section	.text.handle_alloc,"ax",@progbits
	.globl	handle_alloc
	.p2align	4, 0x90
	.type	handle_alloc,@function
handle_alloc:
	push	ebx
	push	edi
	push	esi
	mov	edi, dword ptr [esp + 20]
	mov	esi, dword ptr [esp + 16]
	call	.L0$pb
.L0$pb:
	pop	ebx
.Ltmp0:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp0-.L0$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 16
	test	eax, eax
	je	.LBB0_1
	sub	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB0_1:
	sub	esp, 4
	lea	eax, [ebx + .L__unnamed_1@GOTOFF]
	lea	ecx, [ebx + .L__unnamed_2@GOTOFF]
	push	eax
	push	8
	push	ecx
	call	SYM(core::option::expect_failed::GENERATED_ID, 0)@PLT
	add	esp, 16
	ud2
.Lfunc_end0:
	.size	handle_alloc, .Lfunc_end0-handle_alloc

	.section	.text.handle_init,"ax",@progbits
	.globl	handle_init
	.p2align	4, 0x90
	.type	handle_init,@function
handle_init:
	push	ebx
	push	edi
	push	esi
	mov	esi, dword ptr [esp + 16]
	call	.L1$pb
.L1$pb:
	pop	ebx
.Ltmp1:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp1-.L1$pb)
	test	esi, esi
	je	.LBB1_3
	mov	edi, dword ptr [esp + 20]
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 16
	test	eax, eax
	je	.LBB1_5
	sub	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB1_3:
	xor	eax, eax
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB1_5:
	sub	esp, 4
	lea	eax, [ebx + .L__unnamed_3@GOTOFF]
	lea	ecx, [ebx + .L__unnamed_2@GOTOFF]
	push	eax
	push	8
	push	ecx
	call	SYM(core::option::expect_failed::GENERATED_ID, 0)@PLT
	add	esp, 16
	ud2
.Lfunc_end1:
	.size	handle_init, .Lfunc_end1-handle_init

	.section	.text.handle_alloc_init,"ax",@progbits
	.globl	handle_alloc_init
	.p2align	4, 0x90
	.type	handle_alloc_init,@function
handle_alloc_init:
	push	ebx
	push	edi
	push	esi
	mov	edi, dword ptr [esp + 20]
	mov	esi, dword ptr [esp + 16]
	call	.L2$pb
.L2$pb:
	pop	ebx
.Ltmp2:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp2-.L2$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 16
	test	eax, eax
	je	.LBB2_1
	sub	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	test	eax, eax
	je	.LBB2_7
	mov	edi, dword ptr [esp + 24]
	mov	esi, eax
	sub	esp, 8
	push	edi
	push	eax
	call	objc_msg_lookup@PLT
	add	esp, 16
	test	eax, eax
	je	.LBB2_5
	sub	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB2_7:
	xor	eax, eax
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB2_1:
	sub	esp, 4
	lea	eax, [ebx + .L__unnamed_4@GOTOFF]
	jmp	.LBB2_2
.LBB2_5:
	sub	esp, 4
	lea	eax, [ebx + .L__unnamed_5@GOTOFF]
.LBB2_2:
	lea	ecx, [ebx + .L__unnamed_2@GOTOFF]
	push	eax
	push	8
	push	ecx
	call	SYM(core::option::expect_failed::GENERATED_ID, 0)@PLT
	add	esp, 16
	ud2
.Lfunc_end2:
	.size	handle_alloc_init, .Lfunc_end2-handle_alloc_init

	.section	.text.handle_alloc_release,"ax",@progbits
	.globl	handle_alloc_release
	.p2align	4, 0x90
	.type	handle_alloc_release,@function
handle_alloc_release:
	push	ebx
	push	edi
	push	esi
	mov	edi, dword ptr [esp + 20]
	mov	esi, dword ptr [esp + 16]
	call	.L3$pb
.L3$pb:
	pop	ebx
.Ltmp3:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp3-.L3$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 16
	test	eax, eax
	je	.LBB3_1
	sub	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 4
	push	eax
	call	objc_release@PLT
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB3_1:
	sub	esp, 4
	lea	eax, [ebx + .L__unnamed_6@GOTOFF]
	lea	ecx, [ebx + .L__unnamed_2@GOTOFF]
	push	eax
	push	8
	push	ecx
	call	SYM(core::option::expect_failed::GENERATED_ID, 0)@PLT
	add	esp, 16
	ud2
.Lfunc_end3:
	.size	handle_alloc_release, .Lfunc_end3-handle_alloc_release

	.section	.text.handle_alloc_init_release,"ax",@progbits
	.globl	handle_alloc_init_release
	.p2align	4, 0x90
	.type	handle_alloc_init_release,@function
handle_alloc_init_release:
	push	ebx
	push	edi
	push	esi
	mov	edi, dword ptr [esp + 20]
	mov	esi, dword ptr [esp + 16]
	call	.L4$pb
.L4$pb:
	pop	ebx
.Ltmp4:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp4-.L4$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 16
	test	eax, eax
	je	.LBB4_1
	sub	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	test	eax, eax
	je	.LBB4_4
	mov	edi, dword ptr [esp + 24]
	mov	esi, eax
	sub	esp, 8
	push	edi
	push	eax
	call	objc_msg_lookup@PLT
	add	esp, 16
	test	eax, eax
	je	.LBB4_6
	sub	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	jmp	.LBB4_8
.LBB4_4:
	xor	eax, eax
.LBB4_8:
	sub	esp, 12
	push	eax
	call	objc_release@PLT
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB4_1:
	sub	esp, 4
	lea	eax, [ebx + .L__unnamed_7@GOTOFF]
	jmp	.LBB4_2
.LBB4_6:
	sub	esp, 4
	lea	eax, [ebx + .L__unnamed_8@GOTOFF]
.LBB4_2:
	lea	ecx, [ebx + .L__unnamed_2@GOTOFF]
	push	eax
	push	8
	push	ecx
	call	SYM(core::option::expect_failed::GENERATED_ID, 0)@PLT
	add	esp, 16
	ud2
.Lfunc_end4:
	.size	handle_alloc_init_release, .Lfunc_end4-handle_alloc_init_release

	.section	.text.handle_copy,"ax",@progbits
	.globl	handle_copy
	.p2align	4, 0x90
	.type	handle_copy,@function
handle_copy:
	push	ebx
	push	edi
	push	esi
	mov	edi, dword ptr [esp + 20]
	mov	esi, dword ptr [esp + 16]
	call	.L5$pb
.L5$pb:
	pop	ebx
.Ltmp5:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp5-.L5$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 16
	test	eax, eax
	je	.LBB5_1
	sub	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB5_1:
	sub	esp, 4
	lea	eax, [ebx + .L__unnamed_9@GOTOFF]
	lea	ecx, [ebx + .L__unnamed_2@GOTOFF]
	push	eax
	push	8
	push	ecx
	call	SYM(core::option::expect_failed::GENERATED_ID, 0)@PLT
	add	esp, 16
	ud2
.Lfunc_end5:
	.size	handle_copy, .Lfunc_end5-handle_copy

	.section	.text.handle_autoreleased,"ax",@progbits
	.globl	handle_autoreleased
	.p2align	4, 0x90
	.type	handle_autoreleased,@function
handle_autoreleased:
	push	ebx
	push	edi
	push	esi
	mov	edi, dword ptr [esp + 20]
	mov	esi, dword ptr [esp + 16]
	call	.L6$pb
.L6$pb:
	pop	ebx
.Ltmp6:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp6-.L6$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 16
	test	eax, eax
	je	.LBB6_1
	sub	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 4
	push	eax
	call	objc_retainAutoreleasedReturnValue@PLT
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB6_1:
	sub	esp, 4
	lea	eax, [ebx + .L__unnamed_10@GOTOFF]
	lea	ecx, [ebx + .L__unnamed_2@GOTOFF]
	push	eax
	push	8
	push	ecx
	call	SYM(core::option::expect_failed::GENERATED_ID, 0)@PLT
	add	esp, 16
	ud2
.Lfunc_end6:
	.size	handle_autoreleased, .Lfunc_end6-handle_autoreleased

	.type	.L__unnamed_2,@object
	.section	.rodata.cst8,"aM",@progbits,8
.L__unnamed_2:
	.ascii	"Null IMP"
	.size	.L__unnamed_2, 8

	.type	.L__unnamed_11,@object
	.section	.rodata..L__unnamed_11,"a",@progbits
.L__unnamed_11:
	.ascii	"test-assembly/crates/test_msg_send_id/lib.rs"
	.size	.L__unnamed_11, 44

	.type	.L__unnamed_1,@object
	.section	.data.rel.ro..L__unnamed_1,"aw",@progbits
	.p2align	2
.L__unnamed_1:
	.long	.L__unnamed_11
	.asciz	",\000\000\000\b\000\000\000\005\000\000"
	.size	.L__unnamed_1, 16

	.type	.L__unnamed_3,@object
	.section	.data.rel.ro..L__unnamed_3,"aw",@progbits
	.p2align	2
.L__unnamed_3:
	.long	.L__unnamed_11
	.asciz	",\000\000\000\020\000\000\000\005\000\000"
	.size	.L__unnamed_3, 16

	.type	.L__unnamed_4,@object
	.section	.data.rel.ro..L__unnamed_4,"aw",@progbits
	.p2align	2
.L__unnamed_4:
	.long	.L__unnamed_11
	.asciz	",\000\000\000\025\000\000\000\017\000\000"
	.size	.L__unnamed_4, 16

	.type	.L__unnamed_5,@object
	.section	.data.rel.ro..L__unnamed_5,"aw",@progbits
	.p2align	2
.L__unnamed_5:
	.long	.L__unnamed_11
	.asciz	",\000\000\000\026\000\000\000\005\000\000"
	.size	.L__unnamed_5, 16

	.type	.L__unnamed_6,@object
	.section	.data.rel.ro..L__unnamed_6,"aw",@progbits
	.p2align	2
.L__unnamed_6:
	.long	.L__unnamed_11
	.asciz	",\000\000\000\034\000\000\000\t\000\000"
	.size	.L__unnamed_6, 16

	.type	.L__unnamed_7,@object
	.section	.data.rel.ro..L__unnamed_7,"aw",@progbits
	.p2align	2
.L__unnamed_7:
	.long	.L__unnamed_11
	.asciz	",\000\000\000\"\000\000\000\017\000\000"
	.size	.L__unnamed_7, 16

	.type	.L__unnamed_8,@object
	.section	.data.rel.ro..L__unnamed_8,"aw",@progbits
	.p2align	2
.L__unnamed_8:
	.long	.L__unnamed_11
	.asciz	",\000\000\000$\000\000\000\t\000\000"
	.size	.L__unnamed_8, 16

	.type	.L__unnamed_9,@object
	.section	.data.rel.ro..L__unnamed_9,"aw",@progbits
	.p2align	2
.L__unnamed_9:
	.long	.L__unnamed_11
	.asciz	",\000\000\000*\000\000\000\005\000\000"
	.size	.L__unnamed_9, 16

	.type	.L__unnamed_10,@object
	.section	.data.rel.ro..L__unnamed_10,"aw",@progbits
	.p2align	2
.L__unnamed_10:
	.long	.L__unnamed_11
	.asciz	",\000\000\000/\000\000\000\005\000\000"
	.size	.L__unnamed_10, 16

	.section	".note.GNU-stack","",@progbits
