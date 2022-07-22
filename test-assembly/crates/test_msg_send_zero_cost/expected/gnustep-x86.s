	.text
	.intel_syntax noprefix
	.section	.text.handle,"ax",@progbits
	.globl	handle
	.p2align	4, 0x90
	.type	handle,@function
handle:
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
	.size	handle, .Lfunc_end0-handle

	.section	.text.handle_with_sel,"ax",@progbits
	.globl	handle_with_sel
	.p2align	4, 0x90
	.type	handle_with_sel,@function
handle_with_sel:
	push	ebx
	push	edi
	push	esi
	call	.L1$pb
.L1$pb:
	pop	ebx
	mov	esi, dword ptr [esp + 16]
.Ltmp1:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp1-.L1$pb)
	mov	eax, dword ptr [ebx + SEL_REF@GOT]
	mov	edi, dword ptr [eax]
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 16
	test	eax, eax
	je	.LBB1_1
	sub	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB1_1:
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
	.size	handle_with_sel, .Lfunc_end1-handle_with_sel

	.type	.L__unnamed_2,@object
	.section	.rodata.cst8,"aM",@progbits,8
.L__unnamed_2:
	.ascii	"Null IMP"
	.size	.L__unnamed_2, 8

	.type	.L__unnamed_4,@object
	.section	.rodata..L__unnamed_4,"a",@progbits
.L__unnamed_4:
	.ascii	"test-assembly/crates/test_msg_send_zero_cost/lib.rs"
	.size	.L__unnamed_4, 51

	.type	.L__unnamed_1,@object
	.section	.data.rel.ro..L__unnamed_1,"aw",@progbits
	.p2align	2
.L__unnamed_1:
	.long	.L__unnamed_4
	.asciz	"3\000\000\000\n\000\000\000\005\000\000"
	.size	.L__unnamed_1, 16

	.type	SEL,@object
	.section	.rodata.SEL,"a",@progbits
	.globl	SEL
SEL:
	.asciz	"someSelector"
	.size	SEL, 13

	.type	SEL_REF,@object
	.section	.data.rel.ro.SEL_REF,"aw",@progbits
	.globl	SEL_REF
	.p2align	2
SEL_REF:
	.long	SEL
	.size	SEL_REF, 4

	.type	.L__unnamed_3,@object
	.section	.data.rel.ro..L__unnamed_3,"aw",@progbits
	.p2align	2
.L__unnamed_3:
	.long	.L__unnamed_4
	.asciz	"3\000\000\000\032\000\000\000\005\000\000"
	.size	.L__unnamed_3, 16

	.section	".note.GNU-stack","",@progbits
