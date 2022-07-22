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
	add	esp, 4
	push	eax
	call	objc_retainAutoreleasedReturnValue@PLT
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

	.type	.L__unnamed_2,@object
	.section	.rodata.cst8,"aM",@progbits,8
.L__unnamed_2:
	.ascii	"Null IMP"
	.size	.L__unnamed_2, 8

	.type	.L__unnamed_3,@object
	.section	.rodata..L__unnamed_3,"a",@progbits
.L__unnamed_3:
	.ascii	"test-assembly/crates/test_retain_autoreleased/lib.rs"
	.size	.L__unnamed_3, 52

	.type	.L__unnamed_1,@object
	.section	.data.rel.ro..L__unnamed_1,"aw",@progbits
	.p2align	2
.L__unnamed_1:
	.long	.L__unnamed_3
	.asciz	"4\000\000\000\t\000\000\000\034\000\000"
	.size	.L__unnamed_1, 16

	.section	".note.GNU-stack","",@progbits
