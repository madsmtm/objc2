	.text
	.intel_syntax noprefix
	.section	.text.unlikely.SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_id::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0),"ax",@progbits
	.p2align	4, 0x90
	.type	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_id::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0),@function
SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_id::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0):
	push	ebx
	push	esi
	push	eax
	call	.L0$pb
.L0$pb:
	pop	ebx
	mov	esi, edx
.Ltmp0:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp0-.L0$pb)
	sub	esp, 12
	push	ecx
	call	objc_retain@PLT
	add	esp, 16
	test	eax, eax
	je	.LBB0_1
	add	esp, 4
	pop	esi
	pop	ebx
	ret
.LBB0_1:
	sub	esp, 4
	lea	eax, [ebx + .Lanon.[ID].0@GOTOFF]
	push	esi
	push	56
	push	eax
	call	SYM(core::option::expect_failed::GENERATED_ID, 0)@PLT
	add	esp, 16
	ud2
.Lfunc_end0:
	.size	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_id::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0), .Lfunc_end0-SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_id::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)

	.section	.text.unlikely.SYM(objc2[CRATE_ID]::__macro_helpers::msg_send::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0),"ax",@progbits
	.p2align	4, 0x90
	.type	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0),@function
SYM(objc2[CRATE_ID]::__macro_helpers::msg_send::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0):
	push	ebx
	sub	esp, 8
	call	.L1$pb
.L1$pb:
	pop	ebx
.Ltmp1:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp1-.L1$pb)
	sub	esp, 12
	push	ecx
	call	objc_retain@PLT
	add	esp, 16
	test	eax, eax
	je	.LBB1_1
	add	esp, 8
	pop	ebx
	ret
.LBB1_1:
	sub	esp, 4
	lea	eax, [ebx + .Lanon.[ID].3@GOTOFF]
	lea	ecx, [ebx + .Lanon.[ID].1@GOTOFF]
	push	eax
	push	54
	push	ecx
	call	SYM(core::option::expect_failed::GENERATED_ID, 0)@PLT
	add	esp, 16
	ud2
.Lfunc_end1:
	.size	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0), .Lfunc_end1-SYM(objc2[CRATE_ID]::__macro_helpers::msg_send::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)

	.section	.text.error_bool,"ax",@progbits
	.globl	error_bool
	.p2align	4, 0x90
	.type	error_bool,@function
error_bool:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	edi, dword ptr [esp + 40]
	mov	esi, dword ptr [esp + 32]
	mov	ebp, dword ptr [esp + 36]
	call	.L2$pb
.L2$pb:
	pop	ebx
	mov	dword ptr [esp + 8], 0
.Ltmp2:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp2-.L2$pb)
	sub	esp, 8
	push	ebp
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 16
	lea	ecx, [esp + 8]
	push	ecx
	push	edi
	push	ebp
	push	esi
	call	eax
	add	esp, 16
	mov	ecx, eax
	xor	eax, eax
	test	cl, cl
	je	.LBB2_1
.LBB2_2:
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.LBB2_1:
	mov	ecx, dword ptr [esp + 8]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	jmp	.LBB2_2
.Lfunc_end2:
	.size	error_bool, .Lfunc_end2-error_bool

	.section	.text.error_new,"ax",@progbits
	.globl	error_new
	.p2align	4, 0x90
	.type	error_new,@function
error_new:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L3$pb
.L3$pb:
	pop	ebx
	mov	dword ptr [esp + 12], 0
.Ltmp3:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp3-.L3$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 12
	lea	ecx, [esp + 16]
	push	ecx
	push	edi
	push	esi
	call	eax
	add	esp, 16
	test	eax, eax
	je	.LBB3_2
	mov	edx, eax
	xor	eax, eax
.LBB3_3:
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB3_2:
	mov	ecx, dword ptr [esp + 12]
	lea	edx, [ebx + .Lanon.[ID].4@GOTOFF]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_id::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	edx, eax
	mov	eax, 1
	jmp	.LBB3_3
.Lfunc_end3:
	.size	error_new, .Lfunc_end3-error_new

	.section	.text.error_alloc,"ax",@progbits
	.globl	error_alloc
	.p2align	4, 0x90
	.type	error_alloc,@function
error_alloc:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L4$pb
.L4$pb:
	pop	ebx
	mov	dword ptr [esp + 12], 0
.Ltmp4:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp4-.L4$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 12
	lea	ecx, [esp + 16]
	push	ecx
	push	edi
	push	esi
	call	eax
	add	esp, 16
	test	eax, eax
	je	.LBB4_2
	mov	edx, eax
	xor	eax, eax
.LBB4_3:
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB4_2:
	mov	ecx, dword ptr [esp + 12]
	lea	edx, [ebx + .Lanon.[ID].5@GOTOFF]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_id::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	edx, eax
	mov	eax, 1
	jmp	.LBB4_3
.Lfunc_end4:
	.size	error_alloc, .Lfunc_end4-error_alloc

	.section	.text.error_init,"ax",@progbits
	.globl	error_init
	.p2align	4, 0x90
	.type	error_init,@function
error_init:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	call	.L5$pb
.L5$pb:
	pop	ebx
	mov	dword ptr [esp + 12], 0
.Ltmp5:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp5-.L5$pb)
	test	esi, esi
	je	.LBB5_1
	mov	edi, dword ptr [esp + 36]
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 12
	lea	ecx, [esp + 16]
	push	ecx
	push	edi
	push	esi
	call	eax
	add	esp, 16
	test	eax, eax
	je	.LBB5_4
	mov	edx, eax
	xor	eax, eax
.LBB5_6:
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB5_1:
	xor	ecx, ecx
	jmp	.LBB5_5
.LBB5_4:
	mov	ecx, dword ptr [esp + 12]
.LBB5_5:
	lea	edx, [ebx + .Lanon.[ID].6@GOTOFF]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_id::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	edx, eax
	mov	eax, 1
	jmp	.LBB5_6
.Lfunc_end5:
	.size	error_init, .Lfunc_end5-error_init

	.section	.text.error_copy,"ax",@progbits
	.globl	error_copy
	.p2align	4, 0x90
	.type	error_copy,@function
error_copy:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L6$pb
.L6$pb:
	pop	ebx
	mov	dword ptr [esp + 12], 0
.Ltmp6:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp6-.L6$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 12
	lea	ecx, [esp + 16]
	push	ecx
	push	edi
	push	esi
	call	eax
	add	esp, 16
	test	eax, eax
	je	.LBB6_2
	mov	edx, eax
	xor	eax, eax
.LBB6_3:
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB6_2:
	mov	ecx, dword ptr [esp + 12]
	lea	edx, [ebx + .Lanon.[ID].7@GOTOFF]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_id::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	edx, eax
	mov	eax, 1
	jmp	.LBB6_3
.Lfunc_end6:
	.size	error_copy, .Lfunc_end6-error_copy

	.section	.text.error_autoreleased,"ax",@progbits
	.globl	error_autoreleased
	.p2align	4, 0x90
	.type	error_autoreleased,@function
error_autoreleased:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L7$pb
.L7$pb:
	pop	ebx
	mov	dword ptr [esp + 12], 0
.Ltmp7:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp7-.L7$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 12
	lea	ecx, [esp + 16]
	push	ecx
	push	edi
	push	esi
	call	eax
	add	esp, 4
	push	eax
	call	objc_retainAutoreleasedReturnValue@PLT
	add	esp, 16
	test	eax, eax
	je	.LBB7_2
	mov	edx, eax
	xor	eax, eax
.LBB7_3:
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB7_2:
	mov	ecx, dword ptr [esp + 12]
	lea	edx, [ebx + .Lanon.[ID].8@GOTOFF]
	call	SYM(objc2[CRATE_ID]::__macro_helpers::msg_send_id::encountered_error::<objc2[CRATE_ID]::runtime::AnyObject>, 0)
	mov	edx, eax
	mov	eax, 1
	jmp	.LBB7_3
.Lfunc_end7:
	.size	error_autoreleased, .Lfunc_end7-error_autoreleased

	.type	.Lanon.[ID].0,@object
	.section	.rodata..Lanon.[ID].0,"a",@progbits
.Lanon.[ID].0:
	.ascii	"error parameter should be set if the method returns NULL"
	.size	.Lanon.[ID].0, 56

	.type	.Lanon.[ID].1,@object
	.section	.rodata..Lanon.[ID].1,"a",@progbits
.Lanon.[ID].1:
	.ascii	"error parameter should be set if the method returns NO"
	.size	.Lanon.[ID].1, 54

	.type	.Lanon.[ID].2,@object
	.section	.rodata..Lanon.[ID].2,"a",@progbits
.Lanon.[ID].2:
	.ascii	"crates/$DIR/lib.rs"
	.size	.Lanon.[ID].2, 54

	.type	.Lanon.[ID].3,@object
	.section	.data.rel.ro..Lanon.[ID].3,"aw",@progbits
	.p2align	2, 0x0
.Lanon.[ID].3:
	.long	.Lanon.[ID].2
	.asciz	"6\000\000\000\n\000\000\000\005\000\000"
	.size	.Lanon.[ID].3, 16

	.type	.Lanon.[ID].4,@object
	.section	.data.rel.ro..Lanon.[ID].4,"aw",@progbits
	.p2align	2, 0x0
.Lanon.[ID].4:
	.long	.Lanon.[ID].2
	.asciz	"6\000\000\000\017\000\000\000\005\000\000"
	.size	.Lanon.[ID].4, 16

	.type	.Lanon.[ID].5,@object
	.section	.data.rel.ro..Lanon.[ID].5,"aw",@progbits
	.p2align	2, 0x0
.Lanon.[ID].5:
	.long	.Lanon.[ID].2
	.asciz	"6\000\000\000\024\000\000\000\005\000\000"
	.size	.Lanon.[ID].5, 16

	.type	.Lanon.[ID].6,@object
	.section	.data.rel.ro..Lanon.[ID].6,"aw",@progbits
	.p2align	2, 0x0
.Lanon.[ID].6:
	.long	.Lanon.[ID].2
	.asciz	"6\000\000\000\031\000\000\000\005\000\000"
	.size	.Lanon.[ID].6, 16

	.type	.Lanon.[ID].7,@object
	.section	.data.rel.ro..Lanon.[ID].7,"aw",@progbits
	.p2align	2, 0x0
.Lanon.[ID].7:
	.long	.Lanon.[ID].2
	.asciz	"6\000\000\000\036\000\000\000\005\000\000"
	.size	.Lanon.[ID].7, 16

	.type	.Lanon.[ID].8,@object
	.section	.data.rel.ro..Lanon.[ID].8,"aw",@progbits
	.p2align	2, 0x0
.Lanon.[ID].8:
	.long	.Lanon.[ID].2
	.asciz	"6\000\000\000#\000\000\000\005\000\000"
	.size	.Lanon.[ID].8, 16

	.section	".note.GNU-stack","",@progbits
