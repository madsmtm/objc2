	.text
	.intel_syntax noprefix
	.section	.text.handle_new,"ax",@progbits
	.globl	handle_new
	.p2align	4, 0x90
	.type	handle_new,@function
handle_new:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r14
	mov	rsi, rbx
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	rax
.Lfunc_end0:
	.size	handle_new, .Lfunc_end0-handle_new

	.section	.text.handle_new_fallible,"ax",@progbits
	.globl	handle_new_fallible
	.p2align	4, 0x90
	.type	handle_new_fallible,@function
handle_new_fallible:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
	test	rax, rax
	je	.LBB1_1
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB1_1:
	lea	rdx, [rip + .Lanon.[ID].1]
	mov	rdi, r14
	mov	rsi, rbx
	call	qword ptr [rip + SYM(<objc2::__macro_helpers::method_family::RetainSemantics<1_u8> as objc2::__macro_helpers::msg_send_id::MsgSendIdFailed>::failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end1:
	.size	handle_new_fallible, .Lfunc_end1-handle_new_fallible

	.section	.text.handle_alloc,"ax",@progbits
	.globl	handle_alloc
	.p2align	4, 0x90
	.type	handle_alloc,@function
handle_alloc:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r14
	mov	rsi, rbx
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	rax
.Lfunc_end2:
	.size	handle_alloc, .Lfunc_end2-handle_alloc

	.section	.text.handle_init,"ax",@progbits
	.globl	handle_init
	.p2align	4, 0x90
	.type	handle_init,@function
handle_init:
	test	rdi, rdi
	je	.LBB3_1
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r14
	mov	rsi, rbx
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	rax
.LBB3_1:
	xor	eax, eax
	ret
.Lfunc_end3:
	.size	handle_init, .Lfunc_end3-handle_init

	.section	.text.handle_init_fallible,"ax",@progbits
	.globl	handle_init_fallible
	.p2align	4, 0x90
	.type	handle_init_fallible,@function
handle_init_fallible:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	test	rdi, rdi
	je	.LBB4_2
	mov	rdi, r14
	mov	rsi, rbx
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
	test	rax, rax
	je	.LBB4_2
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB4_2:
	lea	rdx, [rip + .Lanon.[ID].2]
	mov	rdi, r14
	mov	rsi, rbx
	call	qword ptr [rip + SYM(<objc2::__macro_helpers::method_family::RetainSemantics<3_u8> as objc2::__macro_helpers::msg_send_id::MsgSendIdFailed>::failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end4:
	.size	handle_init_fallible, .Lfunc_end4-handle_init_fallible

	.section	.text.handle_alloc_init,"ax",@progbits
	.globl	handle_alloc_init
	.p2align	4, 0x90
	.type	handle_alloc_init,@function
handle_alloc_init:
	push	r15
	push	r14
	push	rbx
	mov	rbx, rdx
	mov	r14, rsi
	mov	r15, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r15
	mov	rsi, r14
	call	rax
	test	rax, rax
	je	.LBB5_1
	mov	r14, rax
	mov	rdi, rax
	mov	rsi, rbx
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r14
	mov	rsi, rbx
	pop	rbx
	pop	r14
	pop	r15
	jmp	rax
.LBB5_1:
	xor	eax, eax
	pop	rbx
	pop	r14
	pop	r15
	ret
.Lfunc_end5:
	.size	handle_alloc_init, .Lfunc_end5-handle_alloc_init

	.section	.text.handle_alloc_release,"ax",@progbits
	.globl	handle_alloc_release
	.p2align	4, 0x90
	.type	handle_alloc_release,@function
handle_alloc_release:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
	mov	rdi, rax
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	qword ptr [rip + objc_release@GOTPCREL]
.Lfunc_end6:
	.size	handle_alloc_release, .Lfunc_end6-handle_alloc_release

	.section	.text.handle_alloc_init_release,"ax",@progbits
	.globl	handle_alloc_init_release
	.p2align	4, 0x90
	.type	handle_alloc_init_release,@function
handle_alloc_init_release:
	push	r15
	push	r14
	push	r12
	push	rbx
	push	rax
	mov	rbx, rdx
	mov	r14, rsi
	mov	r15, rdi
	mov	r12, qword ptr [rip + objc_msg_lookup@GOTPCREL]
	call	r12
	mov	rdi, r15
	mov	rsi, r14
	call	rax
	mov	r14, rax
	mov	rdi, rax
	mov	rsi, rbx
	call	r12
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
	mov	rdi, rax
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	jmp	qword ptr [rip + objc_release@GOTPCREL]
.Lfunc_end7:
	.size	handle_alloc_init_release, .Lfunc_end7-handle_alloc_init_release

	.section	.text.handle_copy,"ax",@progbits
	.globl	handle_copy
	.p2align	4, 0x90
	.type	handle_copy,@function
handle_copy:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r14
	mov	rsi, rbx
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	rax
.Lfunc_end8:
	.size	handle_copy, .Lfunc_end8-handle_copy

	.section	.text.handle_copy_fallible,"ax",@progbits
	.globl	handle_copy_fallible
	.p2align	4, 0x90
	.type	handle_copy_fallible,@function
handle_copy_fallible:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
	test	rax, rax
	je	.LBB9_1
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB9_1:
	lea	rdi, [rip + .Lanon.[ID].3]
	call	qword ptr [rip + SYM(<objc2::__macro_helpers::method_family::RetainSemantics<4_u8> as objc2::__macro_helpers::msg_send_id::MsgSendIdFailed>::failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end9:
	.size	handle_copy_fallible, .Lfunc_end9-handle_copy_fallible

	.section	.text.handle_autoreleased,"ax",@progbits
	.globl	handle_autoreleased
	.p2align	4, 0x90
	.type	handle_autoreleased,@function
handle_autoreleased:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
	mov	rdi, rax
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	qword ptr [rip + objc_retainAutoreleasedReturnValue@GOTPCREL]
.Lfunc_end10:
	.size	handle_autoreleased, .Lfunc_end10-handle_autoreleased

	.section	.text.handle_autoreleased_with_arg,"ax",@progbits
	.globl	handle_autoreleased_with_arg
	.p2align	4, 0x90
	.type	handle_autoreleased_with_arg,@function
handle_autoreleased_with_arg:
	push	r15
	push	r14
	push	rbx
	mov	ebx, edx
	mov	r14, rsi
	mov	r15, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	movzx	edx, bl
	mov	rdi, r15
	mov	rsi, r14
	call	rax
	mov	rdi, rax
	pop	rbx
	pop	r14
	pop	r15
	jmp	qword ptr [rip + objc_retainAutoreleasedReturnValue@GOTPCREL]
.Lfunc_end11:
	.size	handle_autoreleased_with_arg, .Lfunc_end11-handle_autoreleased_with_arg

	.section	.text.handle_autoreleased_fallible,"ax",@progbits
	.globl	handle_autoreleased_fallible
	.p2align	4, 0x90
	.type	handle_autoreleased_fallible,@function
handle_autoreleased_fallible:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
	mov	rdi, rax
	call	qword ptr [rip + objc_retainAutoreleasedReturnValue@GOTPCREL]
	test	rax, rax
	je	.LBB12_1
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.LBB12_1:
	lea	rdx, [rip + .Lanon.[ID].4]
	mov	rdi, r14
	mov	rsi, rbx
	call	qword ptr [rip + SYM(<objc2::__macro_helpers::method_family::RetainSemantics<5_u8> as objc2::__macro_helpers::msg_send_id::MsgSendIdFailed>::failed::GENERATED_ID, 0)@GOTPCREL]
	ud2
.Lfunc_end12:
	.size	handle_autoreleased_fallible, .Lfunc_end12-handle_autoreleased_fallible

	.section	.text.handle_with_out_param,"ax",@progbits
	.globl	handle_with_out_param
	.p2align	4, 0x90
	.type	handle_with_out_param,@function
handle_with_out_param:
	push	r15
	push	r14
	push	r12
	push	rbx
	push	rax
	mov	rbx, rdx
	mov	r14, rsi
	mov	r15, rdi
	mov	r12, qword ptr [rdx]
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r15
	mov	rsi, r14
	mov	rdx, rbx
	call	rax
	mov	r14, rax
	mov	rdi, qword ptr [rbx]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	mov	rdi, r12
	call	qword ptr [rip + objc_release@GOTPCREL]
	mov	rdi, r14
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	jmp	qword ptr [rip + objc_retainAutoreleasedReturnValue@GOTPCREL]
.Lfunc_end13:
	.size	handle_with_out_param, .Lfunc_end13-handle_with_out_param

	.type	.Lanon.[ID].0,@object
	.section	.rodata..Lanon.[ID].0,"a",@progbits
.Lanon.[ID].0:
	.ascii	"crates/$DIR/lib.rs"
	.size	.Lanon.[ID].0, 51

	.type	.Lanon.[ID].1,@object
	.section	.data.rel.ro..Lanon.[ID].1,"aw",@progbits
	.p2align	3, 0x0
.Lanon.[ID].1:
	.quad	.Lanon.[ID].0
	.asciz	"3\000\000\000\000\000\000\000\r\000\000\000\005\000\000"
	.size	.Lanon.[ID].1, 24

	.type	.Lanon.[ID].2,@object
	.section	.data.rel.ro..Lanon.[ID].2,"aw",@progbits
	.p2align	3, 0x0
.Lanon.[ID].2:
	.quad	.Lanon.[ID].0
	.asciz	"3\000\000\000\000\000\000\000\034\000\000\000\005\000\000"
	.size	.Lanon.[ID].2, 24

	.type	.Lanon.[ID].3,@object
	.section	.data.rel.ro..Lanon.[ID].3,"aw",@progbits
	.p2align	3, 0x0
.Lanon.[ID].3:
	.quad	.Lanon.[ID].0
	.asciz	"3\000\000\000\000\000\000\0008\000\000\000\005\000\000"
	.size	.Lanon.[ID].3, 24

	.type	.Lanon.[ID].4,@object
	.section	.data.rel.ro..Lanon.[ID].4,"aw",@progbits
	.p2align	3, 0x0
.Lanon.[ID].4:
	.quad	.Lanon.[ID].0
	.asciz	"3\000\000\000\000\000\000\000L\000\000\000\005\000\000"
	.size	.Lanon.[ID].4, 24

	.section	".note.GNU-stack","",@progbits
