	.text
	.intel_syntax noprefix
	.section	.text.nonnull_nonnull,"ax",@progbits
	.globl	nonnull_nonnull
	.p2align	4, 0x90
	.type	nonnull_nonnull,@function
nonnull_nonnull:
	push	r15
	push	r14
	push	r12
	push	rbx
	push	rax
	mov	rbx, rdx
	mov	r14, rsi
	mov	r15, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	r12, qword ptr [rbx]
	mov	rdi, r15
	mov	rsi, r14
	mov	rdx, rbx
	call	rax
	mov	r14, rax
	mov	rdi, qword ptr [rbx]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	mov	rdi, r12
	call	qword ptr [rip + objc_release@GOTPCREL]
	mov	rax, r14
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	ret
.Lfunc_end0:
	.size	nonnull_nonnull, .Lfunc_end0-nonnull_nonnull

	.section	.text.null_nonnull,"ax",@progbits
	.globl	null_nonnull
	.p2align	4, 0x90
	.type	null_nonnull,@function
null_nonnull:
	push	r15
	push	r14
	push	r12
	push	rbx
	push	rax
	mov	rbx, rdx
	mov	r14, rsi
	mov	r15, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rcx, rax
	test	rbx, rbx
	je	.LBB1_2
	mov	r12, qword ptr [rbx]
	mov	rdi, r15
	mov	rsi, r14
	mov	rdx, rbx
	call	rcx
	mov	r14, rax
	mov	rdi, qword ptr [rbx]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	mov	rdi, r12
	call	qword ptr [rip + objc_release@GOTPCREL]
	mov	rax, r14
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	ret
.LBB1_2:
	mov	rdi, r15
	mov	rsi, r14
	xor	edx, edx
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	jmp	rcx
.Lfunc_end1:
	.size	null_nonnull, .Lfunc_end1-null_nonnull

	.section	.text.nonnull_null,"ax",@progbits
	.globl	nonnull_null
	.p2align	4, 0x90
	.type	nonnull_null,@function
nonnull_null:
	push	r15
	push	r14
	push	r12
	push	rbx
	push	rax
	mov	r15, rdx
	mov	r14, rsi
	mov	r12, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rbx, qword ptr [r15]
	mov	rdi, r12
	mov	rsi, r14
	mov	rdx, r15
	call	rax
	mov	r14, rax
	mov	rdi, qword ptr [r15]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	test	rbx, rbx
	je	.LBB2_2
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
.LBB2_2:
	mov	rax, r14
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	ret
.Lfunc_end2:
	.size	nonnull_null, .Lfunc_end2-nonnull_null

	.section	.text.null_null,"ax",@progbits
	.globl	null_null
	.p2align	4, 0x90
	.type	null_null,@function
null_null:
	push	r15
	push	r14
	push	r12
	push	rbx
	push	rax
	mov	rbx, rdx
	mov	r14, rsi
	mov	r12, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rcx, rax
	test	rbx, rbx
	je	.LBB3_4
	mov	r15, qword ptr [rbx]
	mov	rdi, r12
	mov	rsi, r14
	mov	rdx, rbx
	call	rcx
	mov	r14, rax
	mov	rdi, qword ptr [rbx]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	test	r15, r15
	je	.LBB3_3
	mov	rdi, r15
	call	qword ptr [rip + objc_release@GOTPCREL]
.LBB3_3:
	mov	rax, r14
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	ret
.LBB3_4:
	mov	rdi, r12
	mov	rsi, r14
	xor	edx, edx
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r14
	pop	r15
	jmp	rcx
.Lfunc_end3:
	.size	null_null, .Lfunc_end3-null_null

	.section	.text.two_nonnull_nonnull,"ax",@progbits
	.globl	two_nonnull_nonnull
	.p2align	4, 0x90
	.type	two_nonnull_nonnull,@function
two_nonnull_nonnull:
	push	rbp
	push	r15
	push	r14
	push	r13
	push	r12
	push	rbx
	push	rax
	mov	rbx, rcx
	mov	r14, rdx
	mov	r15, rsi
	mov	r12, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	r13, qword ptr [r14]
	mov	rbp, qword ptr [rbx]
	mov	rdi, r12
	mov	rsi, r15
	mov	rdx, r14
	mov	rcx, rbx
	call	rax
	mov	r15, rax
	mov	rdi, qword ptr [r14]
	mov	r14, qword ptr [rip + objc_retain@GOTPCREL]
	call	r14
	mov	r12, qword ptr [rip + objc_release@GOTPCREL]
	mov	rdi, r13
	call	r12
	mov	rdi, qword ptr [rbx]
	call	r14
	mov	rdi, rbp
	call	r12
	mov	rax, r15
	add	rsp, 8
	pop	rbx
	pop	r12
	pop	r13
	pop	r14
	pop	r15
	pop	rbp
	ret
.Lfunc_end4:
	.size	two_nonnull_nonnull, .Lfunc_end4-two_nonnull_nonnull

	.section	.text.call_with_none1,"ax",@progbits
	.globl	call_with_none1
	.p2align	4, 0x90
	.type	call_with_none1,@function
call_with_none1:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r14
	mov	rsi, rbx
	xor	edx, edx
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	rax
.Lfunc_end5:
	.size	call_with_none1, .Lfunc_end5-call_with_none1

	.section	.text.call_with_none2,"ax",@progbits
	.globl	call_with_none2
	.p2align	4, 0x90
	.type	call_with_none2,@function
call_with_none2:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdi, r14
	mov	rsi, rbx
	xor	edx, edx
	add	rsp, 8
	pop	rbx
	pop	r14
	jmp	rax
.Lfunc_end6:
	.size	call_with_none2, .Lfunc_end6-call_with_none2

	.section	.text.call_with_none3,"ax",@progbits
	.globl	call_with_none3
	.p2align	4, 0x90
	.type	call_with_none3,@function
call_with_none3:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	mov	qword ptr [rsp], 0
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdx, rsp
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
	mov	rbx, rax
	mov	rdi, qword ptr [rsp]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	mov	rdx, qword ptr [rsp]
	mov	rax, rbx
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.Lfunc_end7:
	.size	call_with_none3, .Lfunc_end7-call_with_none3

	.section	.text.call_with_none4,"ax",@progbits
	.globl	call_with_none4
	.p2align	4, 0x90
	.type	call_with_none4,@function
call_with_none4:
	push	r14
	push	rbx
	push	rax
	mov	rbx, rsi
	mov	r14, rdi
	mov	qword ptr [rsp], 0
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	mov	rdx, rsp
	mov	rdi, r14
	mov	rsi, rbx
	call	rax
	mov	rbx, rax
	mov	rdi, qword ptr [rsp]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	mov	rdx, qword ptr [rsp]
	mov	rax, rbx
	add	rsp, 8
	pop	rbx
	pop	r14
	ret
.Lfunc_end8:
	.size	call_with_none4, .Lfunc_end8-call_with_none4

	.section	.text.call_with_some1,"ax",@progbits
	.globl	call_with_some1
	.p2align	4, 0x90
	.type	call_with_some1,@function
call_with_some1:
	push	r15
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rdx
	mov	r14, rsi
	mov	r15, rdi
	mov	qword ptr [rsp + 8], rdx
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	lea	rdx, [rsp + 8]
	mov	rdi, r15
	mov	rsi, r14
	call	rax
	mov	r14, rax
	mov	rdi, qword ptr [rsp + 8]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
	mov	rdx, qword ptr [rsp + 8]
	mov	rax, r14
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	r15
	ret
.Lfunc_end9:
	.size	call_with_some1, .Lfunc_end9-call_with_some1

	.section	.text.call_with_some2,"ax",@progbits
	.globl	call_with_some2
	.p2align	4, 0x90
	.type	call_with_some2,@function
call_with_some2:
	push	r15
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rdx
	mov	r14, rsi
	mov	r15, rdi
	mov	qword ptr [rsp + 8], rdx
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	lea	rdx, [rsp + 8]
	mov	rdi, r15
	mov	rsi, r14
	call	rax
	mov	r14, rax
	mov	rdi, qword ptr [rsp + 8]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
	mov	rdx, qword ptr [rsp + 8]
	mov	rax, r14
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	r15
	ret
.Lfunc_end10:
	.size	call_with_some2, .Lfunc_end10-call_with_some2

	.section	.text.call_with_some3,"ax",@progbits
	.globl	call_with_some3
	.p2align	4, 0x90
	.type	call_with_some3,@function
call_with_some3:
	push	r15
	push	r14
	push	rbx
	sub	rsp, 16
	mov	rbx, rdx
	mov	r14, rsi
	mov	r15, rdi
	mov	qword ptr [rsp + 8], rdx
	call	qword ptr [rip + objc_msg_lookup@GOTPCREL]
	lea	rdx, [rsp + 8]
	mov	rdi, r15
	mov	rsi, r14
	call	rax
	mov	r14, rax
	mov	rdi, qword ptr [rsp + 8]
	call	qword ptr [rip + objc_retain@GOTPCREL]
	mov	rdi, rbx
	call	qword ptr [rip + objc_release@GOTPCREL]
	mov	rdx, qword ptr [rsp + 8]
	mov	rax, r14
	add	rsp, 16
	pop	rbx
	pop	r14
	pop	r15
	ret
.Lfunc_end11:
	.size	call_with_some3, .Lfunc_end11-call_with_some3

	.section	".note.GNU-stack","",@progbits
