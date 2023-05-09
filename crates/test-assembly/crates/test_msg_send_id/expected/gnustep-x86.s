	.text
	.intel_syntax noprefix
	.section	.text.handle_new,"ax",@progbits
	.globl	handle_new
	.p2align	4, 0x90
	.type	handle_new,@function
handle_new:
	push	ebx
	push	edi
	push	esi
	mov	esi, dword ptr [esp + 16]
	mov	edi, dword ptr [esp + 20]
	call	.L0$pb
.L0$pb:
	pop	ebx
.Ltmp0:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp0-.L0$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end0:
	.size	handle_new, .Lfunc_end0-handle_new

	.section	.text.handle_new_fallible,"ax",@progbits
	.globl	handle_new_fallible
	.p2align	4, 0x90
	.type	handle_new_fallible,@function
handle_new_fallible:
	push	ebx
	push	edi
	push	esi
	mov	edi, dword ptr [esp + 20]
	mov	esi, dword ptr [esp + 16]
	call	.L1$pb
.L1$pb:
	pop	ebx
.Ltmp1:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp1-.L1$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	test	eax, eax
	je	.LBB1_1
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB1_1:
	sub	esp, 4
	lea	eax, [ebx + .Lanon.[ID].1@GOTOFF]
	push	eax
	push	edi
	push	esi
	call	SYM(<objc2::__macro_helpers::RetainSemantics<1_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)@PLT
	add	esp, 16
	ud2
.Lfunc_end1:
	.size	handle_new_fallible, .Lfunc_end1-handle_new_fallible

	.section	.text.handle_alloc,"ax",@progbits
	.globl	handle_alloc
	.p2align	4, 0x90
	.type	handle_alloc,@function
handle_alloc:
	push	ebx
	push	edi
	push	esi
	mov	esi, dword ptr [esp + 16]
	mov	edi, dword ptr [esp + 20]
	call	.L2$pb
.L2$pb:
	pop	ebx
.Ltmp2:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp2-.L2$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end2:
	.size	handle_alloc, .Lfunc_end2-handle_alloc

	.section	.text.handle_alloc_fallible,"ax",@progbits
	.globl	handle_alloc_fallible
	.p2align	4, 0x90
	.type	handle_alloc_fallible,@function
handle_alloc_fallible:
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
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	test	eax, eax
	je	.LBB3_1
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB3_1:
	sub	esp, 4
	lea	eax, [ebx + .Lanon.[ID].2@GOTOFF]
	push	eax
	push	edi
	push	esi
	call	SYM(<objc2::__macro_helpers::RetainSemantics<2_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)@PLT
	add	esp, 16
	ud2
.Lfunc_end3:
	.size	handle_alloc_fallible, .Lfunc_end3-handle_alloc_fallible

	.section	.text.handle_init,"ax",@progbits
	.globl	handle_init
	.p2align	4, 0x90
	.type	handle_init,@function
handle_init:
	push	ebx
	push	edi
	push	esi
	mov	esi, dword ptr [esp + 16]
	call	.L4$pb
.L4$pb:
	pop	ebx
.Ltmp4:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp4-.L4$pb)
	test	esi, esi
	je	.LBB4_2
	mov	edi, dword ptr [esp + 20]
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB4_2:
	xor	eax, eax
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end4:
	.size	handle_init, .Lfunc_end4-handle_init

	.section	.text.handle_init_fallible,"ax",@progbits
	.globl	handle_init_fallible
	.p2align	4, 0x90
	.type	handle_init_fallible,@function
handle_init_fallible:
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
	test	esi, esi
	je	.LBB5_2
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	test	eax, eax
	je	.LBB5_2
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB5_2:
	sub	esp, 4
	lea	eax, [ebx + .Lanon.[ID].3@GOTOFF]
	push	eax
	push	edi
	push	esi
	call	SYM(<objc2::__macro_helpers::RetainSemantics<3_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)@PLT
	add	esp, 16
	ud2
.Lfunc_end5:
	.size	handle_init_fallible, .Lfunc_end5-handle_init_fallible

	.section	.text.handle_alloc_init,"ax",@progbits
	.globl	handle_alloc_init
	.p2align	4, 0x90
	.type	handle_alloc_init,@function
handle_alloc_init:
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
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	test	eax, eax
	je	.LBB6_2
	mov	edi, dword ptr [esp + 24]
	mov	esi, eax
	sub	esp, 8
	push	edi
	push	eax
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB6_2:
	xor	eax, eax
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end6:
	.size	handle_alloc_init, .Lfunc_end6-handle_alloc_init

	.section	.text.handle_alloc_release,"ax",@progbits
	.globl	handle_alloc_release
	.p2align	4, 0x90
	.type	handle_alloc_release,@function
handle_alloc_release:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L7$pb
.L7$pb:
	pop	ebx
.Ltmp7:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp7-.L7$pb)
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	eax
	mov	dword ptr [esp], eax
	call	objc_release@PLT
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end7:
	.size	handle_alloc_release, .Lfunc_end7-handle_alloc_release

	.section	.text.handle_alloc_init_release,"ax",@progbits
	.globl	handle_alloc_init_release
	.p2align	4, 0x90
	.type	handle_alloc_init_release,@function
handle_alloc_init_release:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	esi, dword ptr [esp + 32]
	mov	ebp, dword ptr [esp + 36]
	mov	edi, dword ptr [esp + 40]
	call	.L8$pb
.L8$pb:
	pop	ebx
.Ltmp8:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp8-.L8$pb)
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
	mov	dword ptr [esp + 4], ebp
	mov	dword ptr [esp], esi
	call	eax
	mov	esi, eax
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], eax
	call	objc_msg_lookup@PLT
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	eax
	mov	dword ptr [esp], eax
	call	objc_release@PLT
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end8:
	.size	handle_alloc_init_release, .Lfunc_end8-handle_alloc_init_release

	.section	.text.handle_copy,"ax",@progbits
	.globl	handle_copy
	.p2align	4, 0x90
	.type	handle_copy,@function
handle_copy:
	push	ebx
	push	edi
	push	esi
	mov	esi, dword ptr [esp + 16]
	mov	edi, dword ptr [esp + 20]
	call	.L9$pb
.L9$pb:
	pop	ebx
.Ltmp9:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp9-.L9$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end9:
	.size	handle_copy, .Lfunc_end9-handle_copy

	.section	.text.handle_copy_fallible,"ax",@progbits
	.globl	handle_copy_fallible
	.p2align	4, 0x90
	.type	handle_copy_fallible,@function
handle_copy_fallible:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L10$pb
.L10$pb:
	pop	ebx
.Ltmp10:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp10-.L10$pb)
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	eax
	test	eax, eax
	je	.LBB10_1
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB10_1:
	lea	eax, [ebx + .Lanon.[ID].4@GOTOFF]
	mov	dword ptr [esp], eax
	call	SYM(<objc2::__macro_helpers::RetainSemantics<4_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)@PLT
	ud2
.Lfunc_end10:
	.size	handle_copy_fallible, .Lfunc_end10-handle_copy_fallible

	.section	.text.handle_autoreleased,"ax",@progbits
	.globl	handle_autoreleased
	.p2align	4, 0x90
	.type	handle_autoreleased,@function
handle_autoreleased:
	push	ebx
	push	edi
	push	esi
	sub	esp, 16
	mov	esi, dword ptr [esp + 32]
	mov	edi, dword ptr [esp + 36]
	call	.L11$pb
.L11$pb:
	pop	ebx
.Ltmp11:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp11-.L11$pb)
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	objc_msg_lookup@PLT
	mov	dword ptr [esp + 4], edi
	mov	dword ptr [esp], esi
	call	eax
	mov	dword ptr [esp], eax
	call	objc_retainAutoreleasedReturnValue@PLT
	add	esp, 16
	pop	esi
	pop	edi
	pop	ebx
	ret
.Lfunc_end11:
	.size	handle_autoreleased, .Lfunc_end11-handle_autoreleased

	.section	.text.handle_autoreleased_fallible,"ax",@progbits
	.globl	handle_autoreleased_fallible
	.p2align	4, 0x90
	.type	handle_autoreleased_fallible,@function
handle_autoreleased_fallible:
	push	ebx
	push	edi
	push	esi
	mov	edi, dword ptr [esp + 20]
	mov	esi, dword ptr [esp + 16]
	call	.L12$pb
.L12$pb:
	pop	ebx
.Ltmp12:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp12-.L12$pb)
	sub	esp, 8
	push	edi
	push	esi
	call	objc_msg_lookup@PLT
	add	esp, 8
	push	edi
	push	esi
	call	eax
	add	esp, 4
	push	eax
	call	objc_retainAutoreleasedReturnValue@PLT
	add	esp, 16
	test	eax, eax
	je	.LBB12_1
	pop	esi
	pop	edi
	pop	ebx
	ret
.LBB12_1:
	sub	esp, 4
	lea	eax, [ebx + .Lanon.[ID].5@GOTOFF]
	push	eax
	push	edi
	push	esi
	call	SYM(<objc2::__macro_helpers::RetainSemantics<5_u8> as objc2::__macro_helpers::MsgSendIdFailed>::failed::GENERATED_ID, 0)@PLT
	add	esp, 16
	ud2
.Lfunc_end12:
	.size	handle_autoreleased_fallible, .Lfunc_end12-handle_autoreleased_fallible

	.section	.text.handle_with_out_param,"ax",@progbits
	.globl	handle_with_out_param
	.p2align	4, 0x90
	.type	handle_with_out_param,@function
handle_with_out_param:
	push	ebp
	push	ebx
	push	edi
	push	esi
	sub	esp, 12
	mov	eax, dword ptr [esp + 32]
	mov	esi, dword ptr [esp + 36]
	mov	edi, dword ptr [esp + 40]
	call	.L13$pb
.L13$pb:
	pop	ebx
.Ltmp13:
	add	ebx, offset _GLOBAL_OFFSET_TABLE_+(.Ltmp13-.L13$pb)
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], eax
	call	objc_msg_lookup@PLT
	mov	ecx, dword ptr [esp + 32]
	mov	ebp, dword ptr [edi]
	mov	dword ptr [esp + 8], edi
	mov	dword ptr [esp + 4], esi
	mov	dword ptr [esp], ecx
	call	eax
	mov	esi, eax
	mov	eax, dword ptr [edi]
	mov	dword ptr [esp], eax
	call	objc_retain@PLT
	mov	dword ptr [esp], ebp
	call	objc_release@PLT
	mov	dword ptr [esp], esi
	call	objc_retainAutoreleasedReturnValue@PLT
	add	esp, 12
	pop	esi
	pop	edi
	pop	ebx
	pop	ebp
	ret
.Lfunc_end13:
	.size	handle_with_out_param, .Lfunc_end13-handle_with_out_param

	.type	.Lanon.[ID].0,@object
	.section	.rodata..Lanon.[ID].0,"a",@progbits
.Lanon.[ID].0:
	.ascii	"crates/$DIR/lib.rs"
	.size	.Lanon.[ID].0, 51

	.type	.Lanon.[ID].1,@object
	.section	.data.rel.ro..Lanon.[ID].1,"aw",@progbits
	.p2align	2, 0x0
.Lanon.[ID].1:
	.long	.Lanon.[ID].0
	.asciz	"3\000\000\000\r\000\000\000\005\000\000"
	.size	.Lanon.[ID].1, 16

	.type	.Lanon.[ID].2,@object
	.section	.data.rel.ro..Lanon.[ID].2,"aw",@progbits
	.p2align	2, 0x0
.Lanon.[ID].2:
	.long	.Lanon.[ID].0
	.asciz	"3\000\000\000\027\000\000\000\005\000\000"
	.size	.Lanon.[ID].2, 16

	.type	.Lanon.[ID].3,@object
	.section	.data.rel.ro..Lanon.[ID].3,"aw",@progbits
	.p2align	2, 0x0
.Lanon.[ID].3:
	.long	.Lanon.[ID].0
	.asciz	"3\000\000\000!\000\000\000\005\000\000"
	.size	.Lanon.[ID].3, 16

	.type	.Lanon.[ID].4,@object
	.section	.data.rel.ro..Lanon.[ID].4,"aw",@progbits
	.p2align	2, 0x0
.Lanon.[ID].4:
	.long	.Lanon.[ID].0
	.asciz	"3\000\000\000>\000\000\000\005\000\000"
	.size	.Lanon.[ID].4, 16

	.type	.Lanon.[ID].5,@object
	.section	.data.rel.ro..Lanon.[ID].5,"aw",@progbits
	.p2align	2, 0x0
.Lanon.[ID].5:
	.long	.Lanon.[ID].0
	.asciz	"3\000\000\000H\000\000\000\005\000\000"
	.size	.Lanon.[ID].5, 16

	.section	".note.GNU-stack","",@progbits
